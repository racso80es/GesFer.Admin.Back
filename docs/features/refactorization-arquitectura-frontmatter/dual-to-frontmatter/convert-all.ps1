# Script de migración Fase 2.2 + 2.3: dual_to_frontmatter sobre entidades y contratos.
# Ejecutar desde la raíz del repositorio.

$ErrorActionPreference = "Stop"
$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..\..\..\..")).Path
$exe = Join-Path $repoRoot "scripts\tools-rs\target\debug\dual_to_frontmatter.exe"

if (-not (Test-Path $exe)) {
    Write-Error "Compilar primero: cd scripts\tools-rs; cargo build --bin dual_to_frontmatter"
}

# Obtener entidades del inventario (84 con spec.md + spec.json)
$inventarioPath = Join-Path $repoRoot "docs\features\refactorization-arquitectura-frontmatter\inventario-duplicidad.json"
$inventario = Get-Content $inventarioPath | ConvertFrom-Json
$entityDirs = $inventario.inventario | Where-Object { $_.spec_md_exists } | ForEach-Object {
    $p = $_.entity_path -replace "/spec\.json$", ""
    Join-Path $repoRoot ($p -replace "/", "\")
} | Where-Object {
    # Excluir karma2-token
    $_ -notmatch "karma2-token"
} | Sort-Object -Unique

$converted = 0
foreach ($dir in $entityDirs) {
    if ((Test-Path (Join-Path $dir "spec.json")) -and (Test-Path (Join-Path $dir "spec.md"))) {
        & $exe --input-dir $dir 2>&1 | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "Error en $dir" }
        $converted++
    }
}

# Contratos duales (tienen *-contract.json y *-contract.md)
$contractBases = @("actions", "process", "patterns", "principles", "security", "skills", "tools", "templates")
foreach ($base in $contractBases) {
    $dir = switch ($base) {
        "actions" { Join-Path $repoRoot "SddIA\actions" }
        "process" { Join-Path $repoRoot "SddIA\process" }
        "patterns" { Join-Path $repoRoot "SddIA\patterns" }
        "principles" { Join-Path $repoRoot "SddIA\principles" }
        "security" { Join-Path $repoRoot "SddIA\security" }
        "skills" { Join-Path $repoRoot "SddIA\skills" }
        "tools" { Join-Path $repoRoot "SddIA\tools" }
        "templates" { Join-Path $repoRoot "SddIA\templates" }
    }
    $jsonPath = Join-Path $dir "$base-contract.json"
    $mdPath = Join-Path $dir "$base-contract.md"
    if ((Test-Path $jsonPath) -and (Test-Path $mdPath)) {
        $tempDir = Join-Path $env:TEMP ("dual2fm_" + [guid]::NewGuid().ToString("N").Substring(0,8))
        New-Item -ItemType Directory -Path $tempDir -Force | Out-Null
        Copy-Item $jsonPath (Join-Path $tempDir "spec.json")
        Copy-Item $mdPath (Join-Path $tempDir "spec.md")
        $outPath = Join-Path $dir "$base-contract.md"
        & $exe --input-dir $tempDir --output-path $outPath 2>&1 | Out-Null
        Remove-Item $tempDir -Recurse -Force
        if ($LASTEXITCODE -ne 0) { throw "Error en contrato $base" }
        $converted++
    }
}

Write-Host "Convertidas $converted entidades/contratos."
