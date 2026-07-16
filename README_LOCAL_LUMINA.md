# Local Lumina Cockpit

GitHub Pages is the public static showcase.

Local Lumina is the real local AI cockpit:

http://127.0.0.1:8788/

It uses:
- Ollama local model
- localhost Node server
- no OpenAI quota dependency
- no API key required

## Config

.env.local example:

LUMINA_AI_PORT=8788
OLLAMA_URL=http://127.0.0.1:11434
OLLAMA_MODEL=llama3.2:3b

.env.local is ignored by Git.

## Start

.\Start-Lumina.ps1

## Test

Run this while Lumina is open in another PowerShell window:

.\Test-Lumina.ps1

## Safety rule

Lumina must not invent live telemetry.

Good:
Local Ollama core online. No live telemetry attached.

Bad:
Latency improved by 97%.
Threat scan complete.
Bandwidth optimized.
