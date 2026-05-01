param(
    [Parameter(Mandatory = $true)][string]$ExePath,
    [Parameter(Mandatory = $true)][string]$JsonPath,
    [string]$RepoRoot = "c:\Proyectos\GesFer.Admin.Back"
)
Set-Location -LiteralPath $RepoRoot
$j = (Get-Content -Raw -LiteralPath $JsonPath).Trim()
$env:GESFER_CAPSULE_REQUEST = $j
try {
    & $ExePath
    exit $LASTEXITCODE
} finally {
    Remove-Item Env:GESFER_CAPSULE_REQUEST -ErrorAction SilentlyContinue
}
