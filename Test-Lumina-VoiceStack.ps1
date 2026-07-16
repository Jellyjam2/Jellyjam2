$ErrorActionPreference = "Stop"

Write-Host "[LUMINA VOICE STACK] Starting full verification..."

$tests = @(
  ".\Test-Lumina.ps1",
  ".\Test-Lumina-VoiceRouter.ps1",
  ".\Test-Lumina-PremiumVoiceStub.ps1",
  ".\Test-Lumina-OpenAIVoiceAdapter.ps1",
  ".\Test-Lumina-OpenAITTSFunction.ps1",
  ".\Test-Lumina-OpenAITTSRouting.ps1",
  ".\Test-Lumina-OpenAIFallbackGuard.ps1",
  ".\Test-Lumina-OpenAILiveVoiceGuard.ps1"
)

foreach ($test in $tests) {
  if (!(Test-Path -LiteralPath $test)) {
    throw "Missing required voice stack test: $test"
  }

  Write-Host "`n[RUN] $test"
  & $test

  if ($LASTEXITCODE -ne 0) {
    throw "Voice stack test failed: $test"
  }
}

Write-Host "`n[PASS] Lumina voice stack verification complete."
Write-Host "[OK] Piper default voice."
Write-Host "[OK] Voice metadata."
Write-Host "[OK] Provider router."
Write-Host "[OK] Premium status."
Write-Host "[OK] OpenAI function and routing."
Write-Host "[OK] Safe fallback behavior."
Write-Host "[OK] Live OpenAI guard skips safely unless explicitly configured."

