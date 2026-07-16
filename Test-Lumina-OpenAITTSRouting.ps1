$ErrorActionPreference = "Stop"

Write-Host "[1] OpenAI TTS routing source check"

$server = Join-Path (Get-Location) "tools\lumina_ai_core\server.mjs"
$content = Get-Content -LiteralPath $server -Raw

$required = @(
  'if (status.active === "openai")',
  "await speakWithOpenAi(text, voice)",
  'provider: "openai"',
  "contentType: audioContentType(OPENAI_TTS_FORMAT)"
)

foreach ($item in $required) {
  if ($content -notmatch [regex]::Escape($item)) {
    throw "Missing OpenAI routing piece: $item"
  }
}

Write-Host "`n[2] Runtime must still use Piper by default"
$health = Invoke-RestMethod -Uri "http://127.0.0.1:8788/health" -TimeoutSec 30

if ($health.voice_provider.active -ne "piper") {
  throw "Piper should remain active by default."
}

if ($health.voice_provider.premium_voice.openai.adapter -ne "openai_tts") {
  throw "OpenAI adapter status missing."
}

Write-Host "`n[PASS] OpenAI TTS routing is installed and Piper remains active by default."
