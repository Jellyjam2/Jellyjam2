$ErrorActionPreference = "Stop"

Write-Host "[1] Lumina voice badge source check"

$server = Join-Path (Get-Location) "tools\lumina_ai_core\server.mjs"
$content = Get-Content -LiteralPath $server -Raw

$required = @(
  "lumina-voice-badge",
  "lvb-active",
  "lvb-requested",
  "lvb-fallback",
  "lvb-premium",
  "lvb-emotion",
  "updateHealthBadge",
  "updateVoiceBadge"
)

foreach ($item in $required) {
  if ($content -notmatch [regex]::Escape($item)) {
    throw "Missing voice badge piece: $item"
  }
}

Write-Host "`n[2] Lumina cockpit HTML check"
$page = Invoke-WebRequest -Uri "http://127.0.0.1:8788/" -TimeoutSec 30 -UseBasicParsing

if ($page.Content -notmatch "lumina-voice-badge") {
  throw "Cockpit page does not include the voice badge."
}

Write-Host "`n[3] Voice metadata check"
$body = @{
  text = "Reply naturally in one sentence so the voice badge can update emotion."
} | ConvertTo-Json

$result = Invoke-RestMethod `
  -Uri "http://127.0.0.1:8788/think" `
  -Method POST `
  -ContentType "application/json" `
  -Body $body `
  -TimeoutSec 180

if (-not $result.voice) {
  throw "Think response did not return voice metadata."
}

$result.voice | Format-List

Write-Host "`n[PASS] Lumina voice mode badge is present and voice metadata is available."
