$repoRoot = "c:\Proyectos\GesFer.Admin.Back"
Push-Location $repoRoot
$knownTextFields = @("title", "description", "purpose", "name")
$inventario = @()

Get-ChildItem -Path "SddIA" -Recurse -Filter "spec.json" | Where-Object {
    $_.FullName -notlike "*\Tokens\karma2-token\spec.json"
} | ForEach-Object {
    $fullPath = $_.FullName
    $dirPath = $_.DirectoryName
    $rel = (Resolve-Path -Relative $fullPath).Replace("\", "/").TrimStart("./")
    
    $jsonFieldsWithText = @()
    try {
        $json = Get-Content -Path $fullPath -Raw -Encoding UTF8 | ConvertFrom-Json
        $json.PSObject.Properties | ForEach-Object {
            $val = $_.Value
            if ($null -ne $val -and $val -is [string] -and $val.Trim().Length -gt 0) {
                $jsonFieldsWithText += $_.Name
            }
        }
    } catch {
        $jsonFieldsWithText = @("error_parsing")
    }
    
    $specMdPath = Join-Path $dirPath "spec.md"
    $specMdExists = Test-Path $specMdPath
    
    $inventario += [PSCustomObject]@{
        entity_path = $rel
        json_fields_with_text = $jsonFieldsWithText
        spec_md_exists = $specMdExists
    }
}

Pop-Location

$result = @{
    inventario = ($inventario | ForEach-Object { @{
        entity_path = $_.entity_path
        json_fields_with_text = $_.json_fields_with_text
        spec_md_exists = $_.spec_md_exists
    } })
    total_entities = $inventario.Count
    fields_to_remove = $knownTextFields
}

$outPath = Join-Path $repoRoot "docs\features\refactorization-arquitectura-frontmatter\inventario-duplicidad.json"
$result | ConvertTo-Json -Depth 5 | Set-Content -Path $outPath -Encoding UTF8
Write-Output "Generado: $outPath"
