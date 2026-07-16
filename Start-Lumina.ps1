$ErrorActionPreference = "Stop"

$Root = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location -LiteralPath $Root

$EnvFile = Join-Path $Root ".env.local"
$ServerFile = Join-Path $Root "tools\lumina_ai_core\server.mjs"

if (!(Test-Path -LiteralPath $EnvFile)) {
@"
LUMINA_AI_PORT=8788
OLLAMA_URL=http://127.0.0.1:11434
OLLAMA_MODEL=llama3.2:3b
"@ | Set-Content -LiteralPath $EnvFile -Encoding UTF8
}

Get-Content -LiteralPath $EnvFile | ForEach-Object {
  if ($_ -match '^\s*([^#][^=]+)=(.*)$') {
    [Environment]::SetEnvironmentVariable($matches[1].Trim(), $matches[2].Trim(), "Process")
  }
}

if (-not $env:LUMINA_AI_PORT) { $env:LUMINA_AI_PORT = "8788" }
if (-not $env:OLLAMA_URL) { $env:OLLAMA_URL = "http://127.0.0.1:11434" }
if (-not $env:OLLAMA_MODEL) { $env:OLLAMA_MODEL = "llama3.2:3b" }

Write-Host "[LUMINA] Checking Ollama..."
$tags = Invoke-RestMethod -Uri "$($env:OLLAMA_URL.TrimEnd('/'))/api/tags" -Method GET -TimeoutSec 15
$modelNames = @($tags.models | ForEach-Object { $_.name })

if ($modelNames -notcontains $env:OLLAMA_MODEL) {
  Write-Host "[AVAILABLE MODELS]"
  $modelNames | ForEach-Object { Write-Host " - $_" }
  throw "Configured model '$env:OLLAMA_MODEL' was not found."
}

Get-NetTCPConnection -LocalPort ([int]$env:LUMINA_AI_PORT) -ErrorAction SilentlyContinue |
  Select-Object -ExpandProperty OwningProcess -Unique |
  ForEach-Object { Stop-Process -Id $_ -Force -ErrorAction SilentlyContinue }

$url = "http://127.0.0.1:$env:LUMINA_AI_PORT/?v=local-start-$(Get-Date -Format yyyyMMddHHmmss)"

Write-Host "[LUMINA] Starting local cockpit:"
Write-Host "         $url"
Write-Host "[LUMINA] Press Ctrl+C to stop."

Start-Job -ScriptBlock {
  param($OpenUrl)
  Start-Sleep -Seconds 2
  Start-Process $OpenUrl
} -ArgumentList $url | Out-Null

node $ServerFile
