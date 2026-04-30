$ErrorActionPreference = "Stop"
Get-ChildItem -Path ".\\src\\GesFer.Admin.Back.Infrastructure\\Data\\Migrations" | ForEach-Object { $_.Name }

