<#
.SYNOPSIS
    Instala Rust (si no esta) y compila gesfer-tools (scripts/tools-rs).
.DESCRIPTION
    Recuperacion e instalacion: asegura PATH con .cargo\bin, compila con cargo build --release.
    En Windows requiere Visual Studio Build Tools (C++) para el target msvc; si falla link.exe, ver RECUPERACION.
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
    Write-Host "Rust no detectado. Anade al PATH: $cargoBin" -ForegroundColor Yellow
    Write-Host "O instala desde: https://www.rust-lang.org/tools/install" -ForegroundColor Cyan
    exit 1
}

Write-Host "Compilando gesfer-tools (release)..." -ForegroundColor Green
Set-Location $scriptDir
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "" -ForegroundColor Red
    Write-Host "RECUPERACION: Si el error es 'link.exe not found', instala Visual Studio Build Tools con C++:" -ForegroundColor Yellow
    Write-Host "  https://visualstudio.microsoft.com/visual-cpp-build-tools/ -> C++ build tools." -ForegroundColor White
    Write-Host "  Luego abre una terminal nueva y ejecuta de nuevo: .\install.ps1" -ForegroundColor White
    exit 1
}
$toolsDir = Join-Path $scriptDir "..\tools"
$releaseDir = Join-Path $scriptDir "target\release"
$capsules = @(
    @{ exe = "prepare_full_env"; capsule = "prepare-full-env" },
    @{ exe = "invoke_mysql_seeds"; capsule = "invoke-mysql-seeds" },
    @{ exe = "start_api"; capsule = "start-api" }
)
foreach ($cap in $capsules) {
    $src = Join-Path $releaseDir "$($cap.exe).exe"
    $capDir = Join-Path $toolsDir $cap.capsule
    if (-not (Test-Path $src)) { continue }
    # start-api: exe en la carpeta de la tool (junto al .bat); el resto en bin/
    $destDir = if ($cap.capsule -eq "start-api") { $capDir } else { Join-Path $capDir "bin" }
    if (-not (Test-Path $destDir)) { New-Item -ItemType Directory -Path $destDir -Force | Out-Null }
    Copy-Item -Path $src -Destination (Join-Path $destDir "$($cap.exe).exe") -Force
    $relPath = if ($cap.capsule -eq "start-api") { "$($cap.capsule)/$($cap.exe).exe" } else { "$($cap.capsule)/bin/$($cap.exe).exe" }
    Write-Host "  Copiado: scripts/tools/$relPath" -ForegroundColor Cyan
}
Write-Host "OK. Ejecutables en capsulas:" -ForegroundColor Green
Write-Host "  - prepare-full-env/bin/prepare_full_env.exe" -ForegroundColor White
Write-Host "  - invoke-mysql-seeds/bin/invoke_mysql_seeds.exe" -ForegroundColor White
Write-Host "  - start-api/start_api.exe (misma carpeta que el .bat)" -ForegroundColor White
