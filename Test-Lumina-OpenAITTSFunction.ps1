$ErrorActionPreference = "Stop"

Write-Host "[1] OpenAI TTS function source check"

$server = Join-Path (Get-Location) "tools\lumina_ai_core\server.mjs"
$content = Get-Content -LiteralPath $server -Raw

$required = @(
  "function voiceDirectionToOpenAiInstructions",
  "function audioContentType",
  "async function speakWithOpenAi",
  "OPENAI_API_KEY",
  "OPENAI_TTS_MODEL",
  "OPENAI_TTS_VOICE",
  "OPENAI_TTS_FORMAT",
  "OPENAI_TTS_SPEED",
  "OPENAI_TTS_URL"
)

foreach ($item in $required) {
  if ($content -notmatch [regex]::Escape($item)) {
    throw "Missing OpenAI TTS function piece: $item"
  }
}

Write-Host "`n[2] Runtime health must still prefer Piper"
$health = Invoke-RestMethod -Uri "http://127.0.0.1:8788/health" -TimeoutSec 30

if ($health.voice_provider.active -ne "piper") {
  throw "Piper should remain active after adding function-only OpenAI TTS helper."
}

if ($health.voice_provider.premium_voice.openai.adapter -ne "openai_tts") {
  throw "OpenAI adapter status missing."
}

Write-Host "`n[PASS] OpenAI TTS function exists and Piper remains active."
