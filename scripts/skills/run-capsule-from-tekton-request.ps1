# Invoca una cápsula (skill/tool) con envelope capsule-json-io v2 leyendo la raíz del repo.
# Uso: desde la raíz del repositorio, escribir .tekton_request.json y ejecutar:
#   powershell -NoProfile -ExecutionPolicy Bypass -File scripts/skills/run-capsule-from-tekton-request.ps1 -ExePath "scripts/skills/git-save-snapshot/git_save_snapshot.exe"
# Al terminar, elimina .tekton_request.json y limpia GESFER_CAPSULE_REQUEST.
param(
    [Parameter(Mandatory = $true)][string]$ExePath
)
$ErrorActionPreference = "Stop"
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$reqPath = Join-Path $repoRoot ".tekton_request.json"
if (-not (Test-Path -LiteralPath $reqPath)) {
    Write-Error "Falta $reqPath"
    exit 2
}
$j = (Get-Content -Raw -LiteralPath $reqPath).Trim()
if ([string]::IsNullOrWhiteSpace($j)) {
    Write-Error "Contenido vacío en .tekton_request.json"
    exit 2
}
Set-Location -LiteralPath $repoRoot
$env:GESFER_CAPSULE_REQUEST = $j
try {
    $exeResolved = if ([System.IO.Path]::IsPathRooted($ExePath)) { $ExePath } else { Join-Path $repoRoot $ExePath }
    & $exeResolved
    $code = $LASTEXITCODE
} finally {
    Remove-Item Env:GESFER_CAPSULE_REQUEST -ErrorAction SilentlyContinue
    Remove-Item -LiteralPath $reqPath -Force -ErrorAction SilentlyContinue
}
exit $code
