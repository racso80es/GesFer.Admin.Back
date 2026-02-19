<#
.SYNOPSIS
    Prepara todo el entorno: Docker (DB, cache, Adminer) y opcionalmente API y clientes.
.DESCRIPTION
    Lee scripts/tools/prepare-env.json, levanta servicios Docker configurados,
    espera a MySQL y opcionalmente inicia la Admin API y clientes indicados.
    Cumple SddIA/tools/tools-contract.json: salida JSON y feedback adecuado.
    Documentacion: scripts/tools/prepare-env.md
.PARAMETER DockerOnly
    Solo levanta Docker (no API ni clientes).
.PARAMETER StartApi
    Ademas de Docker, levanta la Admin API en local.
.PARAMETER NoDocker
    No levanta Docker; solo API/clientes si estan habilitados.
.PARAMETER ConfigPath
    Ruta al JSON de configuracion (por defecto scripts/tools/prepare-env.json respecto a la raiz).
.PARAMETER OutputPath
    Ruta de fichero donde escribir el resultado JSON (contrato tools).
.PARAMETER OutputJson
    Emitir el resultado JSON por stdout al finalizar.
#>
[CmdletBinding()]
param(
    [switch] $DockerOnly,
    [switch] $StartApi,
    [switch] $NoDocker,
    [string] $ConfigPath,
    [string] $OutputPath,
    [switch] $OutputJson
)

$ErrorActionPreference = "Stop"
$scriptDir = $PSScriptRoot
$repoRoot = (Resolve-Path (Join-Path $scriptDir "..\..")).Path
$startTime = Get-Date
$toolId = "prepare-full-env"
$feedbackList = [System.Collections.Generic.List[object]]::new()

function Add-Feedback {
    param([string]$Phase, [string]$Level, [string]$Message, [string]$Detail = $null, [int]$DurationMs = $null)
    $entry = @{
        phase     = $Phase
        level     = $Level
        message   = $Message
        timestamp = (Get-Date -Format "o")
    }
    if ($Detail) { $entry.detail = $Detail }
    if ($null -ne $DurationMs) { $entry.duration_ms = $DurationMs }
    $feedbackList.Add($entry) | Out-Null
    $color = switch ($Level) { "error" { "Red" } "warning" { "Yellow" } default { "White" } }
    Write-Host $Message -ForegroundColor $color
}

function Write-Result {
    param([bool]$Success, [int]$ExitCode, [string]$Message, [object]$Data = @{})
    $endTime = Get-Date
    $durationMs = [int](($endTime - $startTime).TotalMilliseconds)
    $result = @{
        toolId     = $toolId
        exitCode   = $ExitCode
        success    = $Success
        timestamp  = $endTime.ToUniversalTime().ToString("o")
        message    = $Message
        feedback   = @($feedbackList)
        data       = $Data
        duration_ms = $durationMs
    }
    $json = $result | ConvertTo-Json -Depth 8 -Compress
    if ($OutputPath) {
        $dir = Split-Path -Parent $OutputPath
        if ($dir -and -not (Test-Path $dir)) { New-Item -ItemType Directory -Path $dir -Force | Out-Null }
        $json | Set-Content -Path $OutputPath -Encoding UTF8 -NoNewline
    }
    if ($OutputJson) { Write-Output $json }
    exit $ExitCode
}

if (-not $ConfigPath) {
    $ConfigPath = Join-Path $scriptDir "prepare-env.json"
}

Add-Feedback -Phase "init" -Level "info" -Message "Iniciando $toolId"

try {
# Cargar config o valores por defecto
$config = @{
    dockerComposePath   = "docker-compose.yml"
    mysqlContainerName  = "gesfer_db"
    dockerServices     = @("gesfer-db", "cache", "adminer")
    startApi           = @{ enabled = $false; workingDir = "src/Api"; command = "dotnet run"; serviceName = "AdminApi"; healthUrl = "http://localhost:5010/health" }
    startClients       = @()
    healthCheck        = @{ mysqlMaxAttempts = 30; mysqlRetrySeconds = 2; apiWaitSeconds = 15 }
}
if (Test-Path $ConfigPath) {
    try {
        $fromFile = Get-Content $ConfigPath -Raw -Encoding UTF8 | ConvertFrom-Json
        if ($fromFile.dockerComposePath)     { $config.dockerComposePath    = $fromFile.dockerComposePath }
        if ($fromFile.mysqlContainerName)   { $config.mysqlContainerName   = $fromFile.mysqlContainerName }
        if ($fromFile.dockerServices)       { $config.dockerServices       = @($fromFile.dockerServices) }
        if ($fromFile.healthCheck) {
            if ($fromFile.healthCheck.mysqlMaxAttempts)   { $config.healthCheck.mysqlMaxAttempts   = $fromFile.healthCheck.mysqlMaxAttempts }
            if ($fromFile.healthCheck.mysqlRetrySeconds) { $config.healthCheck.mysqlRetrySeconds = $fromFile.healthCheck.mysqlRetrySeconds }
            if ($fromFile.healthCheck.apiWaitSeconds)    { $config.healthCheck.apiWaitSeconds    = $fromFile.healthCheck.apiWaitSeconds }
        }
        if ($fromFile.startApi) {
            $config.startApi.enabled     = if ($null -ne $fromFile.startApi.enabled) { $fromFile.startApi.enabled } else { $config.startApi.enabled }
            $config.startApi.workingDir  = if ($fromFile.startApi.workingDir) { $fromFile.startApi.workingDir } else { $config.startApi.workingDir }
            $config.startApi.command     = if ($fromFile.startApi.command) { $fromFile.startApi.command } else { $config.startApi.command }
            $config.startApi.serviceName = if ($fromFile.startApi.serviceName) { $fromFile.startApi.serviceName } else { $config.startApi.serviceName }
            $config.startApi.healthUrl   = if ($fromFile.startApi.healthUrl) { $fromFile.startApi.healthUrl } else { $config.startApi.healthUrl }
        }
        if ($fromFile.startClients)       { $config.startClients = @($fromFile.startClients) }
        Add-Feedback -Phase "init" -Level "info" -Message "Configuracion cargada desde $ConfigPath"
    } catch {
        Add-Feedback -Phase "init" -Level "warning" -Message "No se pudo cargar prepare-env.json; usando valores por defecto." -Detail $_.Exception.Message
    }
}

$composePath = Join-Path $repoRoot $config.dockerComposePath
$runServiceScript = Join-Path $repoRoot "scripts\run-service-with-log.ps1"
$data = @{ docker = @{ services = @(); urls = @{} }; api = $null; clients = @() }

# --- Docker ---
if (-not $NoDocker) {
    Add-Feedback -Phase "docker" -Level "info" -Message "Comprobando Docker..."
    docker info 2>&1 | Out-Null
    if ($LASTEXITCODE -ne 0) {
        Add-Feedback -Phase "docker" -Level "error" -Message "Docker no esta corriendo. Inicia Docker Desktop."
        Write-Result -Success $false -ExitCode 1 -Message "Docker no disponible" -Data $data
    }

    if (-not (Test-Path $composePath)) {
        Add-Feedback -Phase "docker" -Level "error" -Message "No se encuentra docker-compose en $composePath"
        Write-Result -Success $false -ExitCode 1 -Message "docker-compose no encontrado" -Data $data
    }

    Add-Feedback -Phase "docker" -Level "info" -Message "Iniciando servicios: $($config.dockerServices -join ', ')"
    $dockerStart = Get-Date
    Push-Location $repoRoot
    try {
        $svcList = $config.dockerServices -join " "
        Invoke-Expression "docker-compose up -d $svcList"
        if ($LASTEXITCODE -ne 0) {
            Add-Feedback -Phase "docker" -Level "error" -Message "Error al levantar Docker. Revisa: docker-compose logs"
            Pop-Location
            $data.docker.error = "docker-compose up -d failed"
            Write-Result -Success $false -ExitCode 1 -Message "Fallo al levantar contenedores" -Data $data
        }
    } finally {
        Pop-Location
    }
    $data.docker.services = @($config.dockerServices)
    $data.docker.urls = @{ mysql = "localhost:3306"; memcached = "localhost:11211"; adminer = "http://localhost:8080" }
    Add-Feedback -Phase "docker" -Level "info" -Message "Servicios Docker iniciados" -DurationMs ([int]((Get-Date) - $dockerStart).TotalMilliseconds)

    $mysqlContainer = $config.mysqlContainerName
    Add-Feedback -Phase "mysql" -Level "info" -Message "Esperando a que MySQL ($mysqlContainer) este listo..."
    $maxAttempts = $config.healthCheck.mysqlMaxAttempts
    $retrySec = $config.healthCheck.mysqlRetrySeconds
    $attempt = 0
    do {
        Start-Sleep -Seconds $retrySec
        $attempt++
        $p = Start-Process -FilePath "docker" -ArgumentList "exec", $mysqlContainer, "mysqladmin", "ping", "-h", "localhost", "-u", "root", "-prootpassword" -PassThru -Wait -NoNewWindow -RedirectStandardError "NUL"
        if ($p.ExitCode -eq 0) {
            Add-Feedback -Phase "mysql" -Level "info" -Message "MySQL listo."
            break
        }
        Add-Feedback -Phase "mysql" -Level "info" -Message "Intento $attempt/$maxAttempts..."
    } while ($attempt -lt $maxAttempts)
    if ($attempt -ge $maxAttempts) {
        Add-Feedback -Phase "mysql" -Level "warning" -Message "MySQL puede tardar mas. Comprueba: docker-compose logs $($config.dockerServices[0])"
    }
}

# --- API en local ---
$startApiLocal = $StartApi -or ((-not $DockerOnly) -and $config.startApi.enabled)
if ($startApiLocal -and (Test-Path $runServiceScript)) {
    $apiDir = Join-Path $repoRoot ($config.startApi.workingDir -replace "/", "\")
    if (-not (Test-Path $apiDir)) {
        Add-Feedback -Phase "api" -Level "warning" -Message "No existe directorio API: $apiDir"
    } else {
        Add-Feedback -Phase "api" -Level "info" -Message "Iniciando Admin API en background..."
        Start-Job -ScriptBlock {
            param($script, $name, $dir, $cmd)
            & $script -ServiceName $name -WorkingDir $dir -Command $cmd
        } -ArgumentList $runServiceScript, $config.startApi.serviceName, $apiDir, $config.startApi.command | Out-Null
        Start-Sleep -Seconds $config.healthCheck.apiWaitSeconds
        Add-Feedback -Phase "api" -Level "info" -Message "API en background; health: $($config.startApi.healthUrl)"
        $data.api = @{ serviceName = $config.startApi.serviceName; healthUrl = $config.startApi.healthUrl; logPath = "logs\services\$($config.startApi.serviceName).log" }
    }
}

# --- Clientes ---
if ((-not $DockerOnly) -and $config.startClients.Count -gt 0 -and (Test-Path $runServiceScript)) {
    foreach ($client in $config.startClients) {
        $name = $client.name
        $cDir = Join-Path $repoRoot ($client.workingDir -replace "/", "\")
        $cmd = $client.command
        if (Test-Path $cDir) {
            Add-Feedback -Phase "clients" -Level "info" -Message "Iniciando cliente '$name' en background..."
            Start-Job -ScriptBlock {
                param($script, $n, $dir, $c)
                & $script -ServiceName $n -WorkingDir $dir -Command $c
            } -ArgumentList $runServiceScript, $name, $cDir, $cmd | Out-Null
            $data.clients += @{ name = $name; workingDir = $client.workingDir }
        } else {
            Add-Feedback -Phase "clients" -Level "warning" -Message "No existe directorio para cliente '$name': $cDir"
        }
    }
}

Add-Feedback -Phase "done" -Level "info" -Message "Entorno preparado. Para detener Docker: docker-compose down"
Write-Result -Success $true -ExitCode 0 -Message "Entorno preparado correctamente" -Data $data
} catch {
    Add-Feedback -Phase "error" -Level "error" -Message "Excepcion no controlada." -Detail $_.Exception.Message
    if (-not $data.docker) { $data.docker = @{ services = @(); urls = @{} } }
    Write-Result -Success $false -ExitCode 1 -Message "Error: $($_.Exception.Message)" -Data $data
}
