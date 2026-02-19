<#
.SYNOPSIS
    Post-merge: posiciona en master/main, sincroniza y elimina la rama ya mergeada (local y opcionalmente remota).
.DESCRIPTION
    Ejecutar tras haber aceptado/mergeado el PR en el remoto. Cumple la fase post_pr de la skill
    finalizar-git (SddIA/skills/finalizar-git.md). No hace merge local: asume que el merge ya ocurrió
    en origin (vía PR). Ley GIT: no commit en master; el merge a master es vía PR.
.PARAMETER BranchName
    Rama que ya fue mergeada (ej. feat/nombre-feature). Si no se indica, se usa la rama actual.
.PARAMETER DeleteRemote
    Si se especifica, elimina también la rama en origin (git push origin --delete <rama>).
.PARAMETER MainBranch
    Nombre de la rama troncal: "master" o "main". Por defecto se detecta con git symbolic-ref.
.EXAMPLE
    .\scripts\skills\Merge-To-Master-Cleanup.ps1 -BranchName "feat/mi-feature" -DeleteRemote
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [string] $BranchName,

    [Parameter(Mandatory = $false)]
    [switch] $DeleteRemote,

    [Parameter(Mandatory = $false)]
    [ValidateSet("master", "main")]
    [string] $MainBranch = ""
)

$ErrorActionPreference = "Stop"

# Resolver rama troncal si no se indicó
if ([string]::IsNullOrWhiteSpace($MainBranch)) {
    $defaultRef = git symbolic-ref refs/remotes/origin/HEAD 2>$null
    if ($defaultRef -match "origin/(master|main)") {
        $MainBranch = $Matches[1]
    } else {
        $MainBranch = "master"
    }
}

# Rama a limpiar: la actual si no se pasó parámetro
if ([string]::IsNullOrWhiteSpace($BranchName)) {
    $BranchName = (git branch --show-current).Trim()
}
if ([string]::IsNullOrWhiteSpace($BranchName)) {
    Write-Error "No se pudo determinar la rama. Indique -BranchName o ejecute desde una rama de trabajo."
    exit 1
}

if ($BranchName -eq $MainBranch) {
    Write-Warning "Ya estás en la rama troncal ($MainBranch). No hay rama de feature/fix que limpiar."
    exit 0
}

Write-Host "[Merge-To-Master-Cleanup] Rama a limpiar: $BranchName | Troncal: $MainBranch" -ForegroundColor Cyan

# 1. Checkout a la rama troncal
Write-Host "[1/4] Checkout a $MainBranch..." -ForegroundColor Yellow
git checkout $MainBranch
if ($LASTEXITCODE -ne 0) {
    Write-Error "Falló git checkout $MainBranch"
    exit 1
}

# 2. Sincronizar con remoto
Write-Host "[2/4] Sincronizando con origin/$MainBranch..." -ForegroundColor Yellow
git pull origin $MainBranch
if ($LASTEXITCODE -ne 0) {
    Write-Error "Falló git pull origin $MainBranch"
    exit 1
}

# 3. Eliminar rama local ya fusionada
Write-Host "[3/4] Eliminando rama local $BranchName..." -ForegroundColor Yellow
git branch -d $BranchName
if ($LASTEXITCODE -ne 0) {
    Write-Host "Intentando eliminación forzada (-D)..." -ForegroundColor Yellow
    git branch -D $BranchName
    if ($LASTEXITCODE -ne 0) {
        Write-Warning "No se pudo eliminar la rama local $BranchName (puede no estar mergeada o tener cambios no integrados)."
    }
}

# 4. Opcional: eliminar rama remota
if ($DeleteRemote) {
    Write-Host "[4/4] Eliminando rama remota origin/$BranchName..." -ForegroundColor Yellow
    git push origin --delete $BranchName
    if ($LASTEXITCODE -ne 0) {
        Write-Warning "No se pudo eliminar la rama remota origin/$BranchName."
    }
} else {
    Write-Host "[4/4] Rama remota no eliminada (use -DeleteRemote para eliminarla)." -ForegroundColor Gray
}

Write-Host "Estado final:" -ForegroundColor Green
git status
git branch -vv
Write-Host "Merge-To-Master-Cleanup finalizado. Repositorio en $MainBranch y sincronizado." -ForegroundColor Green
