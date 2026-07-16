$ErrorActionPreference = "Stop"

$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location -LiteralPath $Root

$EnvFile = Join-Path $Root ".env.local"

if (Test-Path -LiteralPath $EnvFile) {
  Get-Content -LiteralPath $EnvFile | ForEach-Object {
    if ($_ -match '^\s*([^#][^=]+)=(.*)$') {
      [Environment]::SetEnvironmentVariable($matches[1].Trim(), $matches[2].Trim(), "Process")
    }
  }
}

if (-not $env:LUMINA_AI_PORT) { $env:LUMINA_AI_PORT = "8788" }

Write-Host "`n[1] Lumina health"
Invoke-RestMethod `
  -Uri "http://127.0.0.1:$env:LUMINA_AI_PORT/health" `
  -Method GET `
  -TimeoutSec 15 | Format-List

Write-Host "`n[2] Lumina reasoning test"
$body = @{
  text = "Lumina, confirm local mode in one honest tactical sentence. Do not invent fake telemetry."
} | ConvertTo-Json

Invoke-RestMethod `
  -Uri "http://127.0.0.1:$env:LUMINA_AI_PORT/think" `
  -Method POST `
  -ContentType "application/json" `
  -Body $body `
  -TimeoutSec 180 | Format-List

Write-Host "`n[PASS] Lumina local AI core responded."
