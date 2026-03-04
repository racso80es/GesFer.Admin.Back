<#
.SYNOPSIS
    Punto de entrada "finalizar proceso": post-merge completo (checkout main, pull, eliminar rama local y remota).
.DESCRIPTION
    Ejecuta la fase post_pr de la skill FinalizarProceso. Tras haber mergeado el PR en el remoto, unifica
    el repositorio local en main, elimina la rama de trabajo (local) y, por defecto, la rama remota.
    Cumple la acción "finalizar proceso" (paths.actionsPath/finalize/, paths.skillsDefinitionPath/finalizar-proceso/).
.PARAMETER BranchName
    Rama ya mergeada (ej. feat/kaizen-iteracion-20260304). Si no se indica, se usa la rama actual.
.PARAMETER NoDeleteRemote
    Si se especifica, no elimina la rama remota (solo local). Por defecto se elimina la rama en origin (limpieza completa).
.EXAMPLE
    .\Finalizar-Proceso.ps1 -BranchName "feat/kaizen-iteracion-20260304"
.EXAMPLE
    .\Finalizar-Proceso.ps1 -NoDeleteRemote
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [string] $BranchName,

    [Parameter(Mandatory = $false)]
    [switch] $NoDeleteRemote
)

$ErrorActionPreference = "Stop"
$scriptDir = $PSScriptRoot
$cleanupScript = Join-Path $scriptDir "Merge-To-Master-Cleanup.ps1"
if (-not (Test-Path $cleanupScript)) {
    Write-Error "No se encontró Merge-To-Master-Cleanup.ps1 en la cápsula FinalizarProceso."
    exit 1
}

# Por defecto eliminar rama remota (limpieza completa); -NoDeleteRemote para no borrarla
$deleteRemote = -not $NoDeleteRemote
Write-Host "[Finalizar-Proceso] Fase post_pr: unificar en main y limpiar rama (remota: $deleteRemote)" -ForegroundColor Cyan
$params = @{}
if (-not [string]::IsNullOrWhiteSpace($BranchName)) { $params.BranchName = $BranchName }
if ($deleteRemote) { $params.DeleteRemote = $true }

& $cleanupScript @params
exit $LASTEXITCODE
