$ErrorActionPreference = "Stop"

Write-Host "[1] Lumina OpenAI TTS adapter status"
$health = Invoke-RestMethod -Uri "http://127.0.0.1:8788/health" -TimeoutSec 30

$openai = $health.voice_provider.premium_voice.openai
$health.voice_provider | ConvertTo-Json -Depth 10

if (-not $openai) {
  throw "Missing OpenAI adapter status."
}

if ($openai.adapter -ne "openai_tts") {
  throw "OpenAI adapter did not report openai_tts."
}

if ($openai.model -ne "gpt-4o-mini-tts") {
  throw "Unexpected OpenAI model default."
}

if ($openai.voice -ne "cedar") {
  throw "Unexpected OpenAI voice default."
}

if ($openai.response_format -ne "wav") {
  throw "Unexpected OpenAI response format."
}

if ($health.voice_provider.active -ne "piper") {
  throw "Piper must remain active by default."
}

Write-Host "`n[PASS] OpenAI TTS adapter status is installed and Piper remains active."
