<#
.SYNOPSIS
    Ejecuta la acción finalize con suite Git S+: git-sync-remote + git-create-pr.
.DESCRIPTION
    Orquestador de la acción finalize (SddIA/actions/finalize).
    No ejecuta git/gh directamente: invoca skills ejecutables (capsule-json-io).
.PARAMETER Persist
    Ruta de la carpeta de la feature (Cúmulo), ej. docs/features/create-tool-postman-mcp-validation/
.PARAMETER BranchName
    Rama a publicar (obligatoria; el script no resuelve rama por git directo).
.PARAMETER NoVerify
    No ejecutar verify-pr-protocol antes de push/PR.
.PARAMETER Title
    Título del PR (opcional; se pasa a la skill).
.EXAMPLE
    .\Invoke-Finalize.ps1 -Persist "docs/features/create-tool-postman-mcp-validation/"
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string] $Persist,

    [Parameter(Mandatory = $false)]
    [string] $BranchName,

    [Parameter(Mandatory = $false)]
    [switch] $NoVerify,

    [Parameter(Mandatory = $false)]
    [string] $Title
)

$ErrorActionPreference = "Stop"
$scriptDir = $PSScriptRoot
$repoRoot = (Resolve-Path (Join-Path $scriptDir "..\..\..")).Path
Push-Location $repoRoot
try {
    if ([string]::IsNullOrWhiteSpace($BranchName)) {
        Write-Error "BranchName es obligatorio. Este script no ejecuta git para resolver la rama actual."
        exit 1
    }
    if ($BranchName -eq "master" -or $BranchName -eq "main") {
        Write-Error "La acción finalize no debe ejecutarse en la rama troncal (master/main)."
        exit 1
    }

    $persistFull = Join-Path $repoRoot $Persist
    if (-not (Test-Path $persistFull)) {
        Write-Error "No existe la carpeta de la feature: $persistFull"
        exit 1
    }
    $objectivesPath = Join-Path $persistFull "objectives.md"
    if (-not (Test-Path $objectivesPath)) {
        Write-Warning "No se encontró objectives.md en $Persist (recomendado para el proceso)."
    }
    $validacionPath = Join-Path $persistFull "validacion.json"
    if (Test-Path $validacionPath) {
        try {
            $validacion = Get-Content $validacionPath -Raw -Encoding UTF8 | ConvertFrom-Json
            if ($validacion.result -and $validacion.result -ne "pass") {
                Write-Warning "validacion.json no tiene result=pass. Finalize debería ejecutarse tras validación correcta."
            }
        } catch {
            Write-Warning "No se pudo leer validacion.json."
        }
    } else {
        Write-Warning "No se encontró validacion.json en $Persist (recomendado: validar antes de finalize)."
    }

    if (-not $NoVerify) {
        Write-Warning "verify-pr-protocol no se ejecuta aquí (prohibido cargo run directo). Ejecutar vía skill/herramienta autorizada si aplica."
    }

    $syncExe = Join-Path $repoRoot "scripts\\skills\\git-sync-remote\\git_sync_remote.exe"
    $prExe = Join-Path $repoRoot "scripts\\skills\\git-create-pr\\git_create_pr.exe"
    if (-not (Test-Path $syncExe)) {
        Write-Error "No se encontró git_sync_remote.exe en scripts/skills/git-sync-remote/. Compile/copie con skills-rs/install.ps1."
        exit 1
    }
    if (-not (Test-Path $prExe)) {
        Write-Error "No se encontró git_create_pr.exe en scripts/skills/git-create-pr/. Compile/copie con skills-rs/install.ps1."
        exit 1
    }
    if ([string]::IsNullOrWhiteSpace($Title)) {
        Write-Error "Title es obligatorio para crear PR con git-create-pr."
        exit 1
    }

    $objText = ""
    if (Test-Path $objectivesPath) { $objText = Get-Content $objectivesPath -Raw -Encoding UTF8 }
    $valText = ""
    if (Test-Path $validacionPath) { $valText = Get-Content $validacionPath -Raw -Encoding UTF8 }
    $body = "## Artefactos`n`n- Persist: $Persist`n- objectives.md: $(if ($objText) { 'OK' } else { 'NO' })`n- validacion.json: $(if ($valText) { 'OK' } else { 'NO' })`n`n## Objectives (extracto)`n`n$objText`n`n## Validación (extracto)`n`n$valText"

    $syncReq = @{ meta = @{ schema_version = "2.0"; entity_kind = "skill"; entity_id = "git-sync-remote" }; request = @{ force = $false } } | ConvertTo-Json -Compress
    $env:GESFER_CAPSULE_REQUEST = $syncReq
    Write-Host "[Finalize] git-sync-remote..." -ForegroundColor Cyan
    & $syncExe | Write-Host
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

    $prReq = @{ meta = @{ schema_version = "2.0"; entity_kind = "skill"; entity_id = "git-create-pr" }; request = @{ title = $Title; body = $body; base_branch = "main" } } | ConvertTo-Json -Compress
    $env:GESFER_CAPSULE_REQUEST = $prReq
    Write-Host "[Finalize] git-create-pr..." -ForegroundColor Cyan
    & $prExe | Write-Host
    $exitCode = $LASTEXITCODE
} finally {
    Remove-Item Env:GESFER_CAPSULE_REQUEST -ErrorAction SilentlyContinue
    Pop-Location
}
exit $exitCode
