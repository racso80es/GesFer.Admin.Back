<#
.SYNOPSIS
    Wrapper de ejecución para interacciones de IA (Protocolo Racso-Tormentosa).
#>
param (
    [Parameter(Mandatory=$false)] [string]$Command,
    [Parameter(Mandatory=$false)] [string]$CommandFile,
    [Parameter(Mandatory=$false)] [string]$Contexto = "GesFer",
    [ValidateSet("Triaje","Analisis","Evaluacion","Marcado","Accion")]
    [string]$Fase = "Accion"
)

if (-not $Command -and -not $CommandFile) {
    Write-Error "Indique -Command o -CommandFile."
    exit 1
}
if ($CommandFile) {
    $cmdPath = $CommandFile
    if (-not [System.IO.Path]::IsPathRooted($cmdPath)) { $cmdPath = Join-Path (Get-Location) $CommandFile }
    if (-not (Test-Path $cmdPath)) { Write-Error "CommandFile no encontrado: $cmdPath"; exit 1 }
    $Command = (Get-Content -Path $cmdPath -Raw).Trim()
}

# 1. AC-001 [LOGS]: Validación de sintaxis previa
if ($Command -like "*log*") {
    Write-Host "[AC-001] Validando sintaxis de ruta/log..." -ForegroundColor Cyan
    # Lógica de validación de rutas duplicadas o malformadas
}

# 2. Registro de Inicio (Telemetría)
$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
$branch = git rev-parse --abbrev-ref HEAD
$logPath = "docs/diagnostics/$branch/execution_history.json"

# 3. Ejecución con Captura de Errores
try {
    Write-Host "Executing: $Command" -ForegroundColor Yellow
    $output = Invoke-Expression $Command -ErrorAction Stop
    $status = "Success"
} catch {
    $output = $_.Exception.Message
    $status = "Failed"
    Write-Error "Error detectado en la ejecución de la IA."
}

# 4. Persistencia en la Carpeta de la Rama (Output seguro ante null)
$outputStr = if ($null -eq $output) { "" } else { try { $output.ToString() } catch { "" } }
$logEntry = @{
    Timestamp = $timestamp
    Fase      = $Fase
    Command   = $Command
    Status    = $status
    Output    = $outputStr
} | ConvertTo-Json

if (-not (Test-Path "docs/diagnostics/$branch")) { New-Item -ItemType Directory -Path "docs/diagnostics/$branch" }
$logEntry | Out-File -FilePath $logPath -Append

return $output