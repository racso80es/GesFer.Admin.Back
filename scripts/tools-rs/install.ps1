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
    @{ exe = "invoke_mysql_seeds"; capsule = "invoke-mysql-seeds" }
)
foreach ($cap in $capsules) {
    $src = Join-Path $releaseDir "$($cap.exe).exe"
    $binDir = Join-Path (Join-Path $toolsDir $cap.capsule) "bin"
    if (Test-Path $src) {
        if (-not (Test-Path $binDir)) { New-Item -ItemType Directory -Path $binDir -Force | Out-Null }
        Copy-Item -Path $src -Destination (Join-Path $binDir "$($cap.exe).exe") -Force
        Write-Host "  Copiado: scripts/tools/$($cap.capsule)/bin/$($cap.exe).exe" -ForegroundColor Cyan
    }
}
Write-Host "OK. Ejecutables en capsulas (scripts/tools/<tool>/bin/):" -ForegroundColor Green
Write-Host "  - prepare-full-env/bin/prepare_full_env.exe" -ForegroundColor White
Write-Host "  - invoke-mysql-seeds/bin/invoke_mysql_seeds.exe" -ForegroundColor White
