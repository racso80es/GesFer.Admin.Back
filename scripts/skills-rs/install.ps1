<#
.SYNOPSIS
    Compila gesfer-skills (scripts/skills-rs) y copia los .exe a la raíz de cada cápsula (contrato v2).
.DESCRIPTION
    Requiere Rust en PATH (.cargo\bin). Tras cargo build --release, copia los binarios a scripts/skills/<skill-id>/ (sin carpeta bin/).
#>
$ErrorActionPreference = "Stop"
$scriptDir = $PSScriptRoot
$cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
if (Test-Path $cargoBin) { $env:Path = $cargoBin + ";" + $env:Path }

function Test-RustInstalled {
    try {
        $null = Get-Command cargo -ErrorAction Stop
        return $true
    } catch {
        return $false
    }
}

if (-not (Test-RustInstalled)) {
    Write-Host "Rust no detectado. Añade al PATH: $cargoBin" -ForegroundColor Yellow
    Write-Host "O instala desde: https://www.rust-lang.org/tools/install" -ForegroundColor Cyan
    exit 1
}

Write-Host "Compilando gesfer-skills (release)..." -ForegroundColor Green
Set-Location $scriptDir
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "Si el error es 'link.exe not found', instala Visual Studio Build Tools con C++." -ForegroundColor Yellow
    exit 1
}

$skillsDir = Join-Path $scriptDir "..\skills"
$releaseDir = Join-Path $scriptDir "target\release"
$capsules = @(
    @{ exe = "iniciar_rama"; capsule = "iniciar-rama" },
    @{ exe = "merge_to_master_cleanup"; capsule = "finalizar-git" },
    @{ exe = "invoke_command"; capsule = "invoke-command" },
    @{ exe = "push_and_create_pr"; capsule = "finalizar-git" },
    @{ exe = "invoke_commit"; capsule = "invoke-commit" }
)
foreach ($cap in $capsules) {
    $src = Join-Path $releaseDir "$($cap.exe).exe"
    $capDir = Join-Path $skillsDir $cap.capsule
    if (Test-Path $src) {
        if (-not (Test-Path $capDir)) { New-Item -ItemType Directory -Path $capDir -Force | Out-Null }
        $dest = Join-Path $capDir "$($cap.exe).exe"
        Copy-Item -Path $src -Destination $dest -Force
        Write-Host "  Copiado: scripts/skills/$($cap.capsule)/$($cap.exe).exe" -ForegroundColor Cyan
    }
}
Write-Host "OK. Ejecutables en raíz de cápsula (scripts/skills/<skill-id>/)." -ForegroundColor Green
