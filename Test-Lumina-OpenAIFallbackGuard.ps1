$ErrorActionPreference = "Stop"

$repo = Get-Location
$port = 8791
$outFile = Join-Path $repo "lumina_openai_fallback_guard.wav"

$oldPort = $env:LUMINA_AI_PORT
$oldProvider = $env:VOICE_PROVIDER
$oldFallback = $env:VOICE_PROVIDER_FALLBACK
$oldOpenAiKey = $env:OPENAI_API_KEY

$process = $null

try {
  Get-NetTCPConnection -LocalPort $port -ErrorAction SilentlyContinue |
    Select-Object -ExpandProperty OwningProcess -Unique |
    ForEach-Object { Stop-Process -Id $_ -Force -ErrorAction SilentlyContinue }

  $env:LUMINA_AI_PORT = "$port"
  $env:VOICE_PROVIDER = "openai"
  $env:VOICE_PROVIDER_FALLBACK = "piper"
  Remove-Item Env:OPENAI_API_KEY -ErrorAction SilentlyContinue

  Write-Host "[1] Starting isolated Lumina server with VOICE_PROVIDER=openai and no OPENAI_API_KEY"

  $process = Start-Process `
    -FilePath "node" `
    -ArgumentList @(".\tools\lumina_ai_core\server.mjs") `
    -WorkingDirectory $repo `
    -PassThru `
    -WindowStyle Hidden

  $health = $null
  $healthUri = "http://127.0.0.1:$port/health"

  for ($i = 0; $i -lt 30; $i++) {
    Start-Sleep -Seconds 1

    try {
      $health = Invoke-RestMethod -Uri $healthUri -TimeoutSec 5
      break
    } catch {
      if ($process.HasExited) {
        throw "Isolated Lumina server exited early."
      }
    }
  }

  if (-not $health) {
    throw "Isolated Lumina server did not become healthy."
  }

  Write-Host "`n[2] Health"
  $health.voice_provider | ConvertTo-Json -Depth 10

  if ($health.voice_provider.requested -ne "openai") {
    throw "Expected requested provider to be openai."
  }

  if ($health.voice_provider.active -ne "piper") {
    throw "Expected active provider to fall back to piper."
  }

  if ($health.voice_provider.premium_voice.openai.enabled -ne $false) {
    throw "OpenAI should not be enabled without OPENAI_API_KEY."
  }

  Write-Host "`n[3] /speak fallback test"

  $body = @{
    text = "OpenAI was requested, but Piper fallback is active and safe."
    voice = @{
      emotion = "focused"
      pace = "measured"
      humor = "none"
    }
  } | ConvertTo-Json -Depth 10

  $response = Invoke-WebRequest `
    -Uri "http://127.0.0.1:$port/speak" `
    -Method POST `
    -ContentType "application/json" `
    -Body $body `
    -OutFile $outFile `
    -TimeoutSec 120 `
    -PassThru

  $file = Get-Item $outFile

  if ($file.Length -lt 1000) {
    throw "Fallback audio file was unexpectedly small."
  }

  $provider = ($response.Headers["X-Lumina-Voice-Provider"] -join "")
  $requested = ($response.Headers["X-Lumina-Voice-Requested-Provider"] -join "")
  $fallback = ($response.Headers["X-Lumina-Voice-Fallback"] -join "")

  if ($provider -ne "piper") {
    throw "Expected /speak provider header to be piper."
  }

  if ($requested -ne "openai") {
    throw "Expected requested provider header to be openai."
  }

  if ($fallback -ne "true") {
    throw "Expected fallback header to be true."
  }

  Write-Host "`n[PASS] OpenAI request falls back safely to Piper when no key is configured."
} finally {
  if ($process -and -not $process.HasExited) {
    Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
  }

  Remove-Item $outFile -Force -ErrorAction SilentlyContinue

  if ($null -eq $oldPort) { Remove-Item Env:LUMINA_AI_PORT -ErrorAction SilentlyContinue } else { $env:LUMINA_AI_PORT = $oldPort }
  if ($null -eq $oldProvider) { Remove-Item Env:VOICE_PROVIDER -ErrorAction SilentlyContinue } else { $env:VOICE_PROVIDER = $oldProvider }
  if ($null -eq $oldFallback) { Remove-Item Env:VOICE_PROVIDER_FALLBACK -ErrorAction SilentlyContinue } else { $env:VOICE_PROVIDER_FALLBACK = $oldFallback }
  if ($null -eq $oldOpenAiKey) { Remove-Item Env:OPENAI_API_KEY -ErrorAction SilentlyContinue } else { $env:OPENAI_API_KEY = $oldOpenAiKey }
}
