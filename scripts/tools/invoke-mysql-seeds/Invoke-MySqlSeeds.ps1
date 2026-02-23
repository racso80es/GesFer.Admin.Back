<#
.SYNOPSIS
    Ejecuta migraciones EF y seeds de Admin sobre MySQL.
.DESCRIPTION
    Comprueba que MySQL este disponible, aplica dotnet ef database update y ejecuta
    los seeds (companies, admin-users) mediante RUN_SEEDS_ONLY=1 en la API.
    Cumple SddIA/tools/tools-contract.json: salida JSON y feedback adecuado.
    Capsula: scripts/tools/invoke-mysql-seeds/
.PARAMETER SkipMigrations
    No ejecutar dotnet ef database update; solo seeds.
.PARAMETER SkipSeeds
    Solo ejecutar migraciones; no ejecutar seeds.
.PARAMETER ConfigPath
    Ruta al JSON de configuracion.
.PARAMETER OutputPath
    Fichero donde escribir el resultado JSON.
.PARAMETER OutputJson
    Emitir el resultado JSON por stdout.
#>
[CmdletBinding()]
param(
    [switch] $SkipMigrations,
    [switch] $SkipSeeds,
    [string] $ConfigPath,
    [string] $OutputPath,
    [switch] $OutputJson
)

$ErrorActionPreference = "Stop"
$scriptDir = $PSScriptRoot
$repoRoot = (Resolve-Path (Join-Path $scriptDir "..\..\..")).Path
$startTime = Get-Date
$toolId = "mysql-seeds"
$feedbackList = [System.Collections.Generic.List[object]]::new()

function Add-Feedback {
    param([string]$Phase, [string]$Level, [string]$Message, [string]$Detail = $null, [int]$DurationMs = $null)
    $entry = @{ phase = $Phase; level = $Level; message = $Message; timestamp = (Get-Date -Format "o") }
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
        toolId      = $toolId
        exitCode    = $ExitCode
        success     = $Success
        timestamp   = $endTime.ToUniversalTime().ToString("o")
        message     = $Message
        feedback    = @($feedbackList)
        data        = $Data
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

if (-not $ConfigPath) { $ConfigPath = Join-Path $scriptDir "mysql-seeds-config.json" }

$config = @{
    efProject           = "src/Infrastructure/GesFer.Admin.Infra.csproj"
    startupProject      = "src/Api/GesFer.Admin.Api.csproj"
    runMigrations       = $true
    runSeeds            = $true
    mysqlContainerName  = "gesfer_db"
    healthCheck         = @{ mysqlPingMaxAttempts = 15; mysqlPingRetrySeconds = 2 }
}
if (Test-Path $ConfigPath) {
    try {
        $fromFile = Get-Content $ConfigPath -Raw -Encoding UTF8 | ConvertFrom-Json
        if ($fromFile.efProject)          { $config.efProject = $fromFile.efProject }
        if ($fromFile.startupProject)      { $config.startupProject = $fromFile.startupProject }
        if ($null -ne $fromFile.runMigrations) { $config.runMigrations = $fromFile.runMigrations }
        if ($null -ne $fromFile.runSeeds)  { $config.runSeeds = $fromFile.runSeeds }
        if ($fromFile.mysqlContainerName)  { $config.mysqlContainerName = $fromFile.mysqlContainerName }
        if ($fromFile.healthCheck) {
            if ($fromFile.healthCheck.mysqlPingMaxAttempts)   { $config.healthCheck.mysqlPingMaxAttempts = $fromFile.healthCheck.mysqlPingMaxAttempts }
            if ($fromFile.healthCheck.mysqlPingRetrySeconds) { $config.healthCheck.mysqlPingRetrySeconds = $fromFile.healthCheck.mysqlPingRetrySeconds }
        }
        Add-Feedback -Phase "init" -Level "info" -Message "Config cargada: $ConfigPath"
    } catch {
        Add-Feedback -Phase "init" -Level "warning" -Message "No se pudo cargar config; usando valores por defecto." -Detail $_.Exception.Message
    }
}

Add-Feedback -Phase "init" -Level "info" -Message "Iniciando $toolId"

$data = @{ migrations = $null; seeds = $null; mysql = $null }

try {
# Comprobar MySQL (contenedor Docker)
$containerName = $config.mysqlContainerName
Add-Feedback -Phase "mysql" -Level "info" -Message "Comprobando MySQL ($containerName)..."
$attempt = 0
$maxAttempts = $config.healthCheck.mysqlPingMaxAttempts
$retrySec = $config.healthCheck.mysqlPingRetrySeconds
do {
    $p = Start-Process -FilePath "docker" -ArgumentList "exec", $containerName, "mysqladmin", "ping", "-h", "localhost", "-u", "root", "-prootpassword" -PassThru -Wait -NoNewWindow -RedirectStandardError "NUL"
    if ($p.ExitCode -eq 0) {
        Add-Feedback -Phase "mysql" -Level "info" -Message "MySQL disponible."
        $data.mysql = @{ container = $containerName; ready = $true }
        break
    }
    $attempt++
    Add-Feedback -Phase "mysql" -Level "info" -Message "Intento $attempt/$maxAttempts..."
    Start-Sleep -Seconds $retrySec
} while ($attempt -lt $maxAttempts)
if ($attempt -ge $maxAttempts) {
    Add-Feedback -Phase "mysql" -Level "error" -Message "MySQL no responde. Levanta Docker (Prepare-FullEnv) o comprueba el contenedor."
    $data.mysql = @{ container = $containerName; ready = $false }
    Write-Result -Success $false -ExitCode 1 -Message "MySQL no disponible" -Data $data
}

# Migraciones
$runMigrations = -not $SkipMigrations -and $config.runMigrations
if ($runMigrations) {
    $efProj = Join-Path $repoRoot ($config.efProject -replace "/", "\")
    $startupProj = Join-Path $repoRoot ($config.startupProject -replace "/", "\")
    if (-not (Test-Path $efProj) -or -not (Test-Path $startupProj)) {
        Add-Feedback -Phase "migrations" -Level "error" -Message "No se encuentran proyectos EF o Api."
        Write-Result -Success $false -ExitCode 1 -Message "Proyectos no encontrados" -Data $data
    }
    Add-Feedback -Phase "migrations" -Level "info" -Message "Aplicando migraciones EF..."
    $migStart = Get-Date
    Push-Location $repoRoot
    try {
        $proc = Start-Process -FilePath "dotnet" -ArgumentList "ef", "database", "update", "--project", $config.efProject, "--startup-project", $config.startupProject -PassThru -Wait -NoNewWindow
        $data.migrations = @{ exitCode = $proc.ExitCode; duration_ms = [int]((Get-Date) - $migStart).TotalMilliseconds }
        if ($proc.ExitCode -ne 0) {
            Add-Feedback -Phase "migrations" -Level "error" -Message "dotnet ef database update fallo (exit $($proc.ExitCode))."
            Write-Result -Success $false -ExitCode $proc.ExitCode -Message "Error en migraciones" -Data $data
        }
        Add-Feedback -Phase "migrations" -Level "info" -Message "Migraciones aplicadas." -DurationMs $data.migrations.duration_ms
    } finally {
        Pop-Location
    }
} else {
    Add-Feedback -Phase "migrations" -Level "info" -Message "Migraciones omitidas (SkipMigrations o config)."
}

# Seeds
$runSeeds = -not $SkipSeeds -and $config.runSeeds
if ($runSeeds) {
    Add-Feedback -Phase "seeds" -Level "info" -Message "Ejecutando seeds (RUN_SEEDS_ONLY)..."
    $seedStart = Get-Date
    $env:RUN_SEEDS_ONLY = "1"
    try {
        $startupProjPath = (Join-Path $repoRoot $config.startupProject) -replace "/", "\"
        $proc = Start-Process -FilePath "dotnet" -ArgumentList "run", "--project", $startupProjPath -PassThru -Wait -NoNewWindow -WorkingDirectory $repoRoot
        $data.seeds = @{ exitCode = $proc.ExitCode; duration_ms = [int]((Get-Date) - $seedStart).TotalMilliseconds }
        if ($proc.ExitCode -ne 0) {
            Add-Feedback -Phase "seeds" -Level "error" -Message "Seeds fallaron (exit $($proc.ExitCode))."
            Write-Result -Success $false -ExitCode $proc.ExitCode -Message "Error en seeds" -Data $data
        }
        Add-Feedback -Phase "seeds" -Level "info" -Message "Seeds ejecutados correctamente." -DurationMs $data.seeds.duration_ms
    } finally {
        Remove-Item Env:RUN_SEEDS_ONLY -ErrorAction SilentlyContinue
    }
} else {
    Add-Feedback -Phase "seeds" -Level "info" -Message "Seeds omitidos (SkipSeeds o config)."
}

Add-Feedback -Phase "done" -Level "info" -Message "MySQL y seeds listos."
Write-Result -Success $true -ExitCode 0 -Message "Migraciones y seeds completados" -Data $data
} catch {
    Add-Feedback -Phase "error" -Level "error" -Message "Excepcion no controlada." -Detail $_.Exception.Message
    Write-Result -Success $false -ExitCode 1 -Message "Error: $($_.Exception.Message)" -Data $data
}
