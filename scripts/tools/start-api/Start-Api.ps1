<#
.SYNOPSIS
    Levanta la API del proyecto (Admin Back): build opcional, dotnet run, opcional healthcheck.
.DESCRIPTION
    Cumple SddIA/tools/tools-contract.json: salida JSON y feedback. Capsula: scripts/tools/start-api/
    Recomendado ejecutar antes prepare-full-env e invoke-mysql-seeds.
.PARAMETER NoBuild
    No compilar; solo ejecutar si ya hay build.
.PARAMETER Profile
    Perfil de ejecucion (ej. Development).
.PARAMETER Port
    Puerto del host (override; puede requerir variable de entorno o launchSettings).
.PARAMETER ConfigPath
    Ruta al JSON de configuracion (por defecto start-api-config.json en la capsula).
.PARAMETER OutputPath
    Ruta de fichero donde escribir el resultado JSON (contrato tools).
.PARAMETER OutputJson
    Emitir el resultado JSON por stdout al finalizar.
#>
[CmdletBinding()]
param(
    [switch] $NoBuild,
    [string] $Profile,
    [int] $Port,
    [string] $ConfigPath,
    [string] $OutputPath,
    [switch] $OutputJson
)

$ErrorActionPreference = "Stop"
$scriptDir = $PSScriptRoot
$repoRoot = (Resolve-Path (Join-Path $scriptDir "..\..\..")).Path
$startTime = Get-Date
$toolId = "start-api"
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

if (-not $ConfigPath) { $ConfigPath = Join-Path $scriptDir "start-api-config.json" }
if (-not (Test-Path $ConfigPath)) {
    Add-Feedback -Phase "init" -Level "error" -Message "Config no encontrado: $ConfigPath"
    Write-Result -Success $false -ExitCode 1 -Message "Config no encontrado"
}
$config = Get-Content $ConfigPath -Raw | ConvertFrom-Json
$apiDir = Join-Path $repoRoot $config.apiWorkingDir
if (-not (Test-Path $apiDir)) {
    Add-Feedback -Phase "init" -Level "error" -Message "Directorio API no encontrado: $apiDir"
    Write-Result -Success $false -ExitCode 2 -Message "Directorio API no encontrado"
}

Add-Feedback -Phase "init" -Level "info" -Message "Iniciando start-api; repo: $repoRoot"

# Build (opcional)
if (-not $NoBuild) {
    $buildStart = Get-Date
    Add-Feedback -Phase "build" -Level "info" -Message "Compilando proyecto..."
    try {
        Push-Location $repoRoot
        $buildOut = dotnet build $config.apiWorkingDir -c Release 2>&1
        if ($LASTEXITCODE -ne 0) {
            Add-Feedback -Phase "build" -Level "error" -Message "Build fallido" -Detail ($buildOut | Out-String)
            Write-Result -Success $false -ExitCode 3 -Message "Build fallido" -Data @{ build_output = $buildOut }
        }
        $buildMs = [int]((Get-Date - $buildStart).TotalMilliseconds)
        Add-Feedback -Phase "build" -Level "info" -Message "Build OK" -DurationMs $buildMs
    } finally { Pop-Location }
} else {
    Add-Feedback -Phase "build" -Level "info" -Message "NoBuild: omitiendo compilacion"
}

# Launch: dotnet run en segundo plano para que la herramienta devuelva JSON y terceros puedan gestionar la solucion
$port = if ($Port -gt 0) { $Port } else { $config.defaultPort }
$prof = if ($Profile) { $Profile } else { $config.defaultProfile }
$healthUrl = $config.healthUrl -replace "5010", $port
Add-Feedback -Phase "launch" -Level "info" -Message "Levantando API en $apiDir (Profile: $prof, Port: $port)"

Push-Location $apiDir
try {
    $psi = New-Object System.Diagnostics.ProcessStartInfo
    $psi.FileName = "dotnet"
    $psi.Arguments = "run --no-build -c Release"
    $psi.UseShellExecute = $false
    $psi.CreateNoWindow = $false
    $psi.WorkingDirectory = $apiDir
    $psi.Environment["ASPNETCORE_URLS"] = "http://localhost:$port"
    $psi.Environment["ASPNETCORE_ENVIRONMENT"] = $prof
    $p = [System.Diagnostics.Process]::Start($psi)
    $pid = $p.Id
    Add-Feedback -Phase "launch" -Level "info" -Message "API iniciada con PID $pid"

    # Healthcheck opcional (esperar y comprobar /health)
    $timeout = if ($config.healthCheckTimeoutSeconds) { $config.healthCheckTimeoutSeconds } else { 30 }
    $step = 2
    $elapsed = 0
    $healthy = $false
    while ($elapsed -lt $timeout) {
        Start-Sleep -Seconds $step
        $elapsed += $step
        try {
            $r = Invoke-WebRequest -Uri $healthUrl -UseBasicParsing -TimeoutSec 3 -ErrorAction Stop
            if ($r.StatusCode -eq 200) { $healthy = $true; break }
        } catch { }
        Add-Feedback -Phase "healthcheck" -Level "info" -Message "Esperando salud ($elapsed/$timeout s)..."
    }
    if ($healthy) {
        Add-Feedback -Phase "healthcheck" -Level "info" -Message "Health OK: $healthUrl"
    } else {
        Add-Feedback -Phase "healthcheck" -Level "warning" -Message "Timeout salud; API arrancada (PID $pid)"
    }

    $data = @{
        url_base = $healthUrl
        port    = $port
        profile = $prof
        pid     = $pid
        healthy = $healthy
    }
    Add-Feedback -Phase "done" -Level "info" -Message "API levantada. PID: $pid URL: $healthUrl"
    Write-Result -Success $true -ExitCode 0 -Message "API levantada" -Data $data
} catch {
    Add-Feedback -Phase "error" -Level "error" -Message $_.Exception.Message
    Write-Result -Success $false -ExitCode 4 -Message "Error al levantar API" -Data @{ port = $port; profile = $prof }
} finally {
    Pop-Location
}
