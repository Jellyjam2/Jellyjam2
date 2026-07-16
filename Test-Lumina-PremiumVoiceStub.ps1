$ErrorActionPreference = "Stop"

Write-Host "[1] Lumina premium voice stub health"
$health = Invoke-RestMethod -Uri "http://127.0.0.1:8788/health" -TimeoutSec 30

$health.voice_provider | ConvertTo-Json -Depth 10

if (-not $health.voice_provider) {
  throw "Missing voice_provider health block."
}

if (-not $health.voice_provider.premium_voice) {
  throw "Missing premium_voice status block."
}

if ($health.voice_provider.premium_voice.openai.adapter -ne "openai_tts") {
  throw "OpenAI premium adapter status missing."
}

if ($health.voice_provider.premium_voice.elevenlabs.adapter -ne "stub") {
  throw "ElevenLabs premium adapter stub missing."
}

if ($health.voice_provider.active -ne "piper") {
  throw "Expected Piper to remain active by default."
}

Write-Host "`n[PASS] Premium voice status is visible and Piper remains active."

