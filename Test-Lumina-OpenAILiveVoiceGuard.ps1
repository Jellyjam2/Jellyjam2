$ErrorActionPreference = "Stop"

Write-Host "[1] Checking OpenAI live voice configuration"

$hasKey = -not [string]::IsNullOrWhiteSpace($env:OPENAI_API_KEY)
$provider = $env:VOICE_PROVIDER

if (-not $hasKey) {
  Write-Host "[SKIP] OPENAI_API_KEY is not set in this shell. Live OpenAI voice test skipped safely."
  Write-Host "[OK] Piper/local stack remains the safe default."
  exit 0
}

if ($provider -ne "openai") {
  Write-Host "[SKIP] VOICE_PROVIDER is not set to openai. Live OpenAI voice test skipped safely."
  Write-Host "[OK] Set VOICE_PROVIDER=openai only when you intentionally want premium voice."
  exit 0
}

Write-Host "[2] Lumina health"
$health = Invoke-RestMethod -Uri "http://127.0.0.1:8788/health" -TimeoutSec 30
$health.voice_provider | ConvertTo-Json -Depth 10

if ($health.voice_provider.requested -ne "openai") {
  throw "Expected requested provider to be openai."
}

if ($health.voice_provider.active -ne "openai") {
  throw "Expected active provider to be openai when key and provider are configured."
}

Write-Host "`n[3] OpenAI /speak live test"

$outFile = Join-Path (Get-Location) "lumina_openai_live_voice_test.wav"

$body = @{
  text = "Hi Enrico. Lumina premium voice is active. I will stay natural, useful, and emotionally aware."
  voice = @{
    emotion = "warm"
    pace = "natural"
    energy = "medium"
    humor = "light"
    seriousness = "normal"
  }
} | ConvertTo-Json -Depth 10

$response = Invoke-WebRequest `
  -Uri "http://127.0.0.1:8788/speak" `
  -Method POST `
  -ContentType "application/json" `
  -Body $body `
  -OutFile $outFile `
  -TimeoutSec 180 `
  -PassThru

$file = Get-Item $outFile

if ($file.Length -lt 1000) {
  throw "OpenAI voice output was unexpectedly small."
}

$providerHeader = ($response.Headers["X-Lumina-Voice-Provider"] -join "")

if ($providerHeader -ne "openai") {
  throw "Expected X-Lumina-Voice-Provider to be openai."
}

Remove-Item $outFile -Force -ErrorAction SilentlyContinue

Write-Host "`n[PASS] Live OpenAI premium voice generated audio successfully."
