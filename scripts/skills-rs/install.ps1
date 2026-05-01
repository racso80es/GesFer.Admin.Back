<#
.SYNOPSIS
    Compila gesfer-skills (scripts/skills-rs) y copia los .exe a cada cápsula de skill en scripts/skills/.
.DESCRIPTION
    Requiere Rust en PATH (.cargo\bin). Tras cargo build --release, copia los ejecutables a la raíz
    de cada cápsula (convención Admin: index.json paths.skillCapsules) salvo sddia-evolution, que usa bin/.
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

# Raíz de cápsula (skills-contract v2 / manifest components.executable en raíz)
$capsulesRoot = @(
    @{ exe = "invoke_command"; capsule = "invoke-command" },
    @{ exe = "invoke_commit"; capsule = "invoke-commit" },
    @{ exe = "git_workspace_recon"; capsule = "git-workspace-recon" },
    @{ exe = "git_branch_manager"; capsule = "git-branch-manager" },
    @{ exe = "git_save_snapshot"; capsule = "git-save-snapshot" },
    @{ exe = "git_sync_remote"; capsule = "git-sync-remote" },
    @{ exe = "git_tactical_retreat"; capsule = "git-tactical-retreat" },
    @{ exe = "git_create_pr"; capsule = "git-create-pr" },
    @{ exe = "git_close_cycle"; capsule = "git-close-cycle" },
    @{ exe = "verify_pr_protocol"; capsule = "verify-pr-protocol" }
)
foreach ($cap in $capsulesRoot) {
    $src = Join-Path $releaseDir "$($cap.exe).exe"
    $destDir = Join-Path $skillsDir $cap.capsule
    if (Test-Path $src) {
        if (-not (Test-Path $destDir)) { New-Item -ItemType Directory -Path $destDir -Force | Out-Null }
        Copy-Item -Path $src -Destination (Join-Path $destDir "$($cap.exe).exe") -Force
        Write-Host "  Copiado: scripts/skills/$($cap.capsule)/$($cap.exe).exe" -ForegroundColor Cyan
    }
}

# Cápsula sddia-evolution: binarios en bin/ (manifest components.*_exe)
$capsulesBin = @(
    @{ exe = "sddia_evolution_register"; capsule = "sddia-evolution" },
    @{ exe = "sddia_evolution_validate"; capsule = "sddia-evolution" },
    @{ exe = "sddia_evolution_watch"; capsule = "sddia-evolution" }
)
foreach ($cap in $capsulesBin) {
    $src = Join-Path $releaseDir "$($cap.exe).exe"
    $binDir = Join-Path (Join-Path $skillsDir $cap.capsule) "bin"
    if (Test-Path $src) {
        if (-not (Test-Path $binDir)) { New-Item -ItemType Directory -Path $binDir -Force | Out-Null }
        Copy-Item -Path $src -Destination (Join-Path $binDir "$($cap.exe).exe") -Force
        Write-Host "  Copiado: scripts/skills/$($cap.capsule)/bin/$($cap.exe).exe" -ForegroundColor Cyan
    }
}
Write-Host "OK. Ejecutables en cápsulas (scripts/skills/<skill-id>/)." -ForegroundColor Green
