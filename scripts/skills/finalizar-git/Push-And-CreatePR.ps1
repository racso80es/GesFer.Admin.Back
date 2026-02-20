<#
.SYNOPSIS
    Push de la rama actual y creación del PR hacia master/main (GitHub CLI si está disponible).
.DESCRIPTION
    Fase pre_pr de la skill finalizar-git: push origin <rama> y crear Pull Request.
    Si gh (GitHub CLI) está instalado y autenticado, ejecuta gh pr create. Si no, muestra la URL para crear el PR manualmente.
.PARAMETER BranchName
    Rama a pushear (por defecto: rama actual).
.PARAMETER Persist
    Ruta de documentación para el body del PR (ej. docs/features/skill.Token/).
.PARAMETER Title
    Título del PR (opcional; por defecto se deriva del nombre de rama).
.PARAMETER BaseBranch
    Rama base del PR: master o main (por defecto se detecta con git symbolic-ref origin/HEAD).
.EXAMPLE
    .\Push-And-CreatePR.ps1 -Persist "docs/features/skill.Token/"
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory = $false)]
    [string] $BranchName,

    [Parameter(Mandatory = $false)]
    [string] $Persist,

    [Parameter(Mandatory = $false)]
    [string] $Title,

    [Parameter(Mandatory = $false)]
    [ValidateSet("master", "main")]
    [string] $BaseBranch = ""
)

$ErrorActionPreference = "Stop"

if ([string]::IsNullOrWhiteSpace($BranchName)) {
    $BranchName = (git branch --show-current).Trim()
}
if ([string]::IsNullOrWhiteSpace($BranchName)) {
    Write-Error "No se pudo determinar la rama. Indique -BranchName."
    exit 1
}

# Detectar base (master/main)
if ([string]::IsNullOrWhiteSpace($BaseBranch)) {
    $ref = git symbolic-ref refs/remotes/origin/HEAD 2>$null
    if ($ref -match "origin/(master|main)") {
        $BaseBranch = $Matches[1]
    } else {
        $BaseBranch = "master"
    }
}

if ($BranchName -eq $BaseBranch) {
    Write-Warning "La rama actual es la troncal ($BaseBranch). No se hace push de PR."
    exit 0
}

Write-Host "[Push-And-CreatePR] Rama: $BranchName -> base: $BaseBranch" -ForegroundColor Cyan

# 1. Push
Write-Host "[1/2] Push origin $BranchName..." -ForegroundColor Yellow
git push origin $BranchName
if ($LASTEXITCODE -ne 0) {
    Write-Error "Falló git push origin $BranchName"
    exit 1
}
Write-Host "Push OK." -ForegroundColor Green

# 2. Crear PR (gh si está disponible)
$body = if ($Persist) { "Documentación: ``$Persist``" } else { "Rama: $BranchName" }
$prTitle = if ($Title) { $Title } else { $BranchName }

$ghPath = Get-Command gh -ErrorAction SilentlyContinue
if ($ghPath) {
    Write-Host "[2/2] Creando PR con GitHub CLI (gh)..." -ForegroundColor Yellow
    $ghArgs = @(
        "pr", "create",
        "--base", $BaseBranch,
        "--head", $BranchName,
        "--title", $prTitle,
        "--body", $body
    )
    & gh @ghArgs
    if ($LASTEXITCODE -eq 0) {
        Write-Host "PR creado correctamente." -ForegroundColor Green
        exit 0
    }
    Write-Warning "gh pr create falló (¿autenticado? gh auth login). Mostrando URL manual."
}

# Fallback: URL para crear PR manualmente (GitHub)
$remoteUrl = (git config --get remote.origin.url).Trim()
$repo = $null
if ($remoteUrl -match "github\.com[:/](.+?)(?:\.git)?$") {
    $repo = $Matches[1].TrimEnd("/")
}
if ($repo) {
    $createUrl = "https://github.com/$repo/compare/$BaseBranch...$BranchName?expand=1"
    Write-Host "[2/2] Crear PR manualmente:" -ForegroundColor Yellow
    Write-Host "  $createUrl" -ForegroundColor White
    Write-Host "  Título sugerido: $prTitle" -ForegroundColor Gray
    Write-Host "  Body: $body" -ForegroundColor Gray
} else {
    Write-Host "[2/2] Crea el PR desde tu proveedor (Azure DevOps, etc.). Rama ya pusheada: $BranchName" -ForegroundColor Yellow
}
exit 0
