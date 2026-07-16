$ErrorActionPreference = "Stop"

Write-Host "[1] Lumina voice provider health"
$health = Invoke-RestMethod -Uri "http://127.0.0.1:8788/health" -TimeoutSec 30
$health | Format-List

if (-not $health.voice) {
  throw "Health response did not include voice status."
}

Write-Host "`n[2] Lumina /speak router test"
$outFile = Join-Path (Get-Location) "lumina_voice_router_test.wav"

$body = @{
  text = "Lumina voice provider router is active. Piper remains the local voice provider."
  voice = @{
    emotion = "pleased"
    pace = "natural"
    humor = "light"
  }
} | ConvertTo-Json -Depth 10

$response = Invoke-WebRequest `
  -Uri "http://127.0.0.1:8788/speak" `
  -Method POST `
  -ContentType "application/json" `
  -Body $body `
  -OutFile $outFile `
  -TimeoutSec 120 `
  -PassThru

$file = Get-Item $outFile

if ($file.Length -lt 1000) {
  throw "Voice router produced an unexpectedly small audio file."
}

Write-Host "`n[VOICE ROUTER HEADERS]"
$response.Headers

Remove-Item $outFile -Force -ErrorAction SilentlyContinue

Write-Host "`n[PASS] Lumina voice provider router generated audio without opening media player."
