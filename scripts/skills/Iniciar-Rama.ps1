<#
.SYNOPSIS
    Crea una rama nueva (feat/ o fix/) actualizada con master/main y se posiciona en ella.
.DESCRIPTION
    Para el inicio de una acción o tarea: actualiza la rama troncal con origin, crea la rama de trabajo
    y hace checkout. Cumple la skill iniciar-rama (SddIA/skills/iniciar-rama.md). Ley GIT: no trabajar
    en master; todo el trabajo en feat/ o fix/.
.PARAMETER BranchType
    Tipo de rama: "feat" (funcionalidad) o "fix" (corrección).
.PARAMETER BranchName
    Nombre/slug de la rama (ej. mi-feature, admin-back-login). Se formará feat/<BranchName> o fix/<BranchName>.
.PARAMETER MainBranch
    Rama troncal: "master" o "main". Por defecto se detecta con git symbolic-ref.
.PARAMETER SkipPull
    Si se especifica, no ejecuta git pull (útil si master ya está actualizado).
.EXAMPLE
    .\scripts\skills\Iniciar-Rama.ps1 -BranchType feat -BranchName auditoria-scripts
.EXAMPLE
    .\scripts\skills\Iniciar-Rama.ps1 -BranchType fix -BranchName correccion-timeout
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [ValidateSet("feat", "fix")]
    [string] $BranchType,

    [Parameter(Mandatory = $true)]
    [string] $BranchName,

    [Parameter(Mandatory = $false)]
    [ValidateSet("master", "main")]
    [string] $MainBranch = "",

    [Parameter(Mandatory = $false)]
    [switch] $SkipPull
)

$ErrorActionPreference = "Stop"

# Normalizar slug: sin barras, espacios ni caracteres problemáticos
$slug = ($BranchName -replace "[\s/\\]", "-").Trim()
if ([string]::IsNullOrWhiteSpace($slug)) {
    Write-Error "BranchName no puede quedar vacío tras normalizar."
    exit 1
}

$newBranch = "${BranchType}/${slug}"

# Resolver rama troncal si no se indicó
if ([string]::IsNullOrWhiteSpace($MainBranch)) {
    $defaultRef = git symbolic-ref refs/remotes/origin/HEAD 2>$null
    if ($defaultRef -match "origin/(master|main)") {
        $MainBranch = $Matches[1]
    } else {
        $MainBranch = "master"
    }
}

Write-Host "[Iniciar-Rama] Rama a crear: $newBranch | Troncal: $MainBranch" -ForegroundColor Cyan

# Comprobar si la rama ya existe
$exists = git rev-parse --verify $newBranch 2>$null
if ($LASTEXITCODE -eq 0) {
    Write-Host "La rama '$newBranch' ya existe. Haciendo checkout y actualizando desde $MainBranch..." -ForegroundColor Yellow
    git fetch origin $MainBranch 2>$null
    git checkout $newBranch
    if ($LASTEXITCODE -ne 0) { exit 1 }
    git merge origin/$MainBranch --no-edit
    if ($LASTEXITCODE -ne 0) {
        Write-Warning "Merge de origin/$MainBranch en $newBranch tuvo conflictos. Resolver manualmente."
        exit 1
    }
    Write-Host "Rama '$newBranch' actualizada con $MainBranch." -ForegroundColor Green
    git status
    exit 0
}

# 1. Fetch
Write-Host "[1/4] Fetch origin..." -ForegroundColor Yellow
git fetch origin
if ($LASTEXITCODE -ne 0) {
    Write-Error "Falló git fetch origin"
    exit 1
}

# 2. Checkout a troncal
Write-Host "[2/4] Checkout a $MainBranch..." -ForegroundColor Yellow
git checkout $MainBranch
if ($LASTEXITCODE -ne 0) {
    Write-Error "Falló git checkout $MainBranch"
    exit 1
}

# 3. Actualizar troncal con remoto
if (-not $SkipPull) {
    Write-Host "[3/4] Actualizando $MainBranch con origin..." -ForegroundColor Yellow
    git pull origin $MainBranch
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Falló git pull origin $MainBranch"
        exit 1
    }
} else {
    Write-Host "[3/4] SkipPull: no se ejecuta pull." -ForegroundColor Gray
}

# 4. Crear y cambiar a la nueva rama
Write-Host "[4/4] Creando rama $newBranch..." -ForegroundColor Yellow
git checkout -b $newBranch
if ($LASTEXITCODE -ne 0) {
    Write-Error "Falló git checkout -b $newBranch"
    exit 1
}

Write-Host "Rama '$newBranch' creada y actualizada con $MainBranch. Listo para trabajar." -ForegroundColor Green
git status
git branch -vv
