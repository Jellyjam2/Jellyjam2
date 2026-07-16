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

## Premium voice activation

Lumina uses local Piper voice by default. Premium voice is optional and must be enabled only from local environment settings.

Do not commit real API keys. Do not paste API keys into chat.

Safe default local voice settings:

```text
VOICE_PROVIDER=piper
VOICE_PROVIDER_FALLBACK=piper
VOICE_STYLE=natural_companion
VOICE_EMOTION_AUTO=true
```

OpenAI premium voice activation, local only:

```text
VOICE_PROVIDER=openai
VOICE_PROVIDER_FALLBACK=piper
OPENAI_API_KEY=your_local_key_here
OPENAI_TTS_MODEL=gpt-4o-mini-tts
OPENAI_TTS_VOICE=cedar
OPENAI_TTS_FORMAT=wav
OPENAI_TTS_SPEED=1
```

Safety behavior:

- Without OPENAI_API_KEY, Lumina falls back to Piper.
- With VOICE_PROVIDER=piper, Lumina stays fully local.
- The live OpenAI voice test skips safely unless both OPENAI_API_KEY and VOICE_PROVIDER=openai are set.
- Piper remains the safe fallback provider.
