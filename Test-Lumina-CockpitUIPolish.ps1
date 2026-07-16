$ErrorActionPreference = "Stop"

Write-Host "[1] Lumina cockpit UI polish source check"

$server = Join-Path (Get-Location) "tools\lumina_ai_core\server.mjs"
$content = Get-Content -LiteralPath $server -Raw

$required = @(
  "lumina-cockpit-polish-v1",
  "lumina-cockpit-polish-mark",
  "Lumina Cockpit · Local Mode",
  "radial-gradient",
  "lumina-voice-badge"
)

foreach ($item in $required) {
  if ($content -notmatch [regex]::Escape($item)) {
    throw "Missing cockpit polish piece: $item"
  }
}

Write-Host "`n[2] Runtime HTML check"
$page = Invoke-WebRequest -Uri "http://127.0.0.1:8788/" -TimeoutSec 30 -UseBasicParsing

if ($page.Content -notmatch "lumina-cockpit-polish-v1") {
  throw "Cockpit page does not include polish stylesheet."
}

if ($page.Content -notmatch "lumina-cockpit-polish-mark") {
  throw "Cockpit page does not include polish marker."
}

Write-Host "`n[PASS] Lumina cockpit UI polish is present in source and runtime HTML."
