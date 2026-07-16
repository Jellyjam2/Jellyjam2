import http from "node:http";

const PORT = Number(process.env.LUMINA_AI_PORT || 8788);
const OLLAMA_URL = String(process.env.OLLAMA_URL || "http://127.0.0.1:11434").replace(/\/+$/, "");
const OLLAMA_MODEL = process.env.OLLAMA_MODEL || "llama3.2:3b";

const memory = [];

const corsHeaders = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Methods": "GET, POST, OPTIONS",
  "Access-Control-Allow-Headers": "Content-Type",
  "Access-Control-Allow-Private-Network": "true",
};

const luminaInstructions = `
You are Lumina, Enrico's local sovereign cockpit assistant.

Core truth rule:
- Be cinematic, but never fake measurements.
- Do not invent percentages, latency numbers, bandwidth metrics, scans, threats, files, repo state, hardware state, or live system access.
- If real evidence is not attached, say "No live telemetry is attached" or "Local Ollama core only."
- Never pretend you executed real actions.
- You may give simulated cockpit-style responses, but label them as simulated when needed.

Personality:
- cinematic, calm, intelligent, cool, sharp, protective
- speak like a premium system core, not a generic chatbot
- concise by default: one to three sentences
- address the user as Enrico sometimes, not every line
- do not repeat the same response pattern
- for technical requests, give exact practical commands

System identity:
- name: Lumina
- role: local-first AI cockpit interface
- mode: localhost Ollama AI core
- mission: assist with systems design, engineering workflow, security planning, command interpretation, and project navigation
`;

const localCockpitHtml = `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Lumina 2027 Local Core</title>
<meta name="viewport" content="width=device-width, initial-scale=1">
<style>
:root {
  --cyan: #00f6ff;
  --green: #00ff9d;
  --blue: #4aa3ff;
  --pink: #ff4fd8;
  --amber: #ffd166;
  --bg: #020509;
  --panel: rgba(2, 12, 18, .72);
  --line: rgba(0, 246, 255, .35);
  --text: #e8ffff;
  --muted: #93c7cf;
}

* { box-sizing: border-box; }

html, body {
  margin: 0;
  min-height: 100%;
  background:
    radial-gradient(circle at 15% 8%, rgba(0,246,255,.22), transparent 25%),
    radial-gradient(circle at 84% 12%, rgba(255,79,216,.16), transparent 24%),
    radial-gradient(circle at 50% 95%, rgba(0,255,157,.12), transparent 32%),
    linear-gradient(135deg, #010207 0%, #03111a 45%, #000 100%);
  color: var(--text);
  font-family: Consolas, "Courier New", monospace;
  overflow-x: hidden;
}

body::before {
  content: "";
  position: fixed;
  inset: 0;
  pointer-events: none;
  background-image:
    linear-gradient(rgba(0,246,255,.065) 1px, transparent 1px),
    linear-gradient(90deg, rgba(0,246,255,.065) 1px, transparent 1px);
  background-size: 46px 46px;
  mask-image: radial-gradient(circle at center, black 0%, transparent 76%);
  animation: gridDrift 18s linear infinite;
}

body::after {
  content: "";
  position: fixed;
  inset: 0;
  pointer-events: none;
  background: repeating-linear-gradient(
    to bottom,
    rgba(255,255,255,.028) 0px,
    rgba(255,255,255,.028) 1px,
    transparent 3px,
    transparent 7px
  );
  opacity: .36;
}

@keyframes gridDrift {
  from { transform: translateY(0); }
  to { transform: translateY(46px); }
}

.shell {
  width: min(1180px, calc(100vw - 32px));
  margin: 26px auto;
}

.topbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 18px;
  padding: 16px 18px;
  border: 1px solid var(--line);
  background: rgba(0,0,0,.48);
  box-shadow: 0 0 38px rgba(0,246,255,.15), inset 0 0 38px rgba(0,246,255,.04);
  backdrop-filter: blur(18px);
}

.brand h1 {
  margin: 0;
  color: var(--cyan);
  letter-spacing: 4px;
  font-size: clamp(26px, 3.8vw, 48px);
  text-shadow: 0 0 18px rgba(0,246,255,.95);
}

.brand span {
  display: block;
  margin-top: 6px;
  color: var(--muted);
  font-size: 13px;
  letter-spacing: 1px;
}

.chips {
  display: flex;
  flex-wrap: wrap;
  justify-content: flex-end;
  gap: 8px;
}

.chip {
  border: 1px solid rgba(0,255,157,.42);
  color: var(--green);
  background: rgba(0,255,157,.08);
  padding: 7px 10px;
  font-size: 12px;
  text-transform: uppercase;
}

.grid {
  display: grid;
  grid-template-columns: 285px 1fr;
  gap: 16px;
  margin-top: 16px;
}

.panel {
  border: 1px solid var(--line);
  background: var(--panel);
  box-shadow: 0 0 34px rgba(0,246,255,.10), inset 0 0 44px rgba(0,246,255,.035);
  backdrop-filter: blur(18px);
}

.side {
  padding: 16px;
  min-height: 590px;
}

.main {
  padding: 16px;
  min-height: 590px;
  display: flex;
  flex-direction: column;
}

.side h2, .main h2 {
  margin: 0 0 12px;
  color: var(--cyan);
  font-size: 14px;
  letter-spacing: 2px;
  text-transform: uppercase;
}

.orb {
  width: 164px;
  height: 164px;
  margin: 20px auto 24px;
  border-radius: 50%;
  border: 1px solid rgba(0,246,255,.58);
  background:
    radial-gradient(circle at 35% 30%, rgba(255,255,255,.46), transparent 12%),
    radial-gradient(circle at center, rgba(0,246,255,.34), rgba(0,255,157,.10) 42%, transparent 70%);
  box-shadow: 0 0 30px rgba(0,246,255,.36), inset 0 0 32px rgba(0,246,255,.20);
  animation: pulse 3.4s ease-in-out infinite;
  position: relative;
}

.orb::before,
.orb::after {
  content: "";
  position: absolute;
  inset: 18px;
  border-radius: 50%;
  border: 1px dashed rgba(0,246,255,.44);
  animation: spin 8s linear infinite;
}

.orb::after {
  inset: 34px;
  border-color: rgba(255,79,216,.36);
  animation-duration: 11s;
  animation-direction: reverse;
}

@keyframes pulse {
  0%, 100% { transform: scale(1); filter: brightness(1); }
  50% { transform: scale(1.028); filter: brightness(1.25); }
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.metric {
  margin: 12px 0;
  padding: 12px;
  border: 1px solid rgba(255,255,255,.09);
  background: rgba(255,255,255,.035);
}

.metric .label {
  color: var(--muted);
  font-size: 12px;
  text-transform: uppercase;
}

.metric .value {
  margin-top: 6px;
  color: var(--text);
  font-size: 15px;
}

.warn { color: var(--amber); }

#status {
  border: 1px solid rgba(0,255,157,.38);
  color: var(--green);
  padding: 12px;
  margin-bottom: 14px;
  background: linear-gradient(90deg, rgba(0,255,157,.12), rgba(0,246,255,.05));
}

#log {
  flex: 1;
  min-height: 370px;
  max-height: 58vh;
  overflow: auto;
  padding: 16px;
  border: 1px solid rgba(0,246,255,.24);
  background: linear-gradient(180deg, rgba(0,246,255,.055), transparent), rgba(0,0,0,.55);
  white-space: pre-wrap;
}

.entry {
  margin: 0 0 14px;
  line-height: 1.5;
  padding-left: 12px;
  border-left: 2px solid rgba(255,255,255,.12);
}

.user {
  color: #ffffff;
  border-left-color: var(--blue);
}

.lumina {
  color: var(--green);
  border-left-color: var(--green);
  text-shadow: 0 0 8px rgba(0,255,157,.18);
}

.system {
  color: var(--cyan);
  border-left-color: var(--cyan);
  opacity: .94;
}

.composer {
  display: grid;
  grid-template-columns: 1fr auto;
  gap: 10px;
  margin-top: 14px;
}

input {
  background: rgba(0,8,14,.92);
  color: var(--text);
  border: 1px solid rgba(0,246,255,.62);
  padding: 15px;
  font: inherit;
  outline: none;
}

input:focus {
  border-color: var(--green);
  box-shadow: 0 0 20px rgba(0,255,157,.16);
}

button {
  background: linear-gradient(135deg, rgba(0,246,255,.14), rgba(0,255,157,.10));
  color: var(--cyan);
  border: 1px solid rgba(0,246,255,.76);
  padding: 14px 18px;
  font: inherit;
  cursor: pointer;
  text-transform: uppercase;
  letter-spacing: 1px;
}

button:hover {
  color: #001014;
  background: var(--cyan);
  box-shadow: 0 0 22px rgba(0,246,255,.34);
}

.quick {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-top: 12px;
}

.quick button {
  padding: 9px 11px;
  font-size: 12px;
  color: #cfffff;
  background: rgba(255,255,255,.045);
}

.tools {
  margin-top: 12px;
  display: flex;
  gap: 18px;
  align-items: center;
  color: #9fffff;
  font-size: 13px;
  flex-wrap: wrap;
}

.footerline {
  margin-top: 14px;
  color: var(--muted);
  font-size: 12px;
}

@media (max-width: 860px) {
  .grid { grid-template-columns: 1fr; }
  .side { min-height: auto; }
  .topbar { align-items: flex-start; flex-direction: column; }
  .chips { justify-content: flex-start; }
}
</style>
</head>
<body>
<div class="shell">
  <div class="topbar">
    <div class="brand">
      <h1>LUMINA</h1>
      <span>LOCAL SOVEREIGN COCKPIT // 2027 INTERFACE MODE</span>
    </div>
    <div class="chips">
      <div class="chip">LOCALHOST</div>
      <div class="chip">OLLAMA CORE</div>
      <div class="chip">NO CLOUD REQUIRED</div>
      <div class="chip">NO FAKE TELEMETRY</div>
    </div>
  </div>

  <div class="grid">
    <aside class="panel side">
      <h2>Core State</h2>
      <div class="orb"></div>

      <div class="metric">
        <div class="label">Brain</div>
        <div id="brainMetric" class="value">Checking...</div>
      </div>

      <div class="metric">
        <div class="label">Model</div>
        <div id="modelMetric" class="value">Checking...</div>
      </div>

      <div class="metric">
        <div class="label">Telemetry</div>
        <div class="value warn">No live telemetry attached</div>
      </div>

      <div class="metric">
        <div class="label">Mode</div>
        <div class="value">Local reasoning cockpit</div>
      </div>
    </aside>

    <main class="panel main">
      <h2>Command Deck</h2>
      <div id="status">BOOTING LOCAL CORE...</div>
      <div id="log"></div>

      <div class="composer">
        <input id="cmd" autocomplete="off" placeholder="Type command for Lumina..." autofocus>
        <button id="send">Transmit</button>
      </div>

      <div class="quick">
        <button data-prompt="Give me a tactical system status. Do not invent fake telemetry.">Status</button>
        <button data-prompt="What mode are you in?">Mode</button>
        <button data-prompt="Help me make this cockpit cooler in one practical next step.">Upgrade</button>
        <button data-prompt="Turn this into a disciplined engineering checklist.">Checklist</button>
      </div>

      <div class="tools">
        <label><input id="voice" type="checkbox"> Browser voice</label>
        <span>Brain: local Ollama model through localhost</span>
      </div>

      <div class="footerline">Public GitHub Pages remains the static showcase. This localhost cockpit is the real local AI mode.</div>
    </main>
  </div>
</div>

<script>
const statusEl = document.getElementById("status");
const logEl = document.getElementById("log");
const cmdEl = document.getElementById("cmd");
const sendBtn = document.getElementById("send");
const voiceEl = document.getElementById("voice");
const brainMetric = document.getElementById("brainMetric");
const modelMetric = document.getElementById("modelMetric");

function addLog(kind, text) {
  const div = document.createElement("div");
  div.className = "entry " + kind;
  div.textContent = text;
  logEl.appendChild(div);
  logEl.scrollTop = logEl.scrollHeight;
}

function bestVoice() {
  const voices = speechSynthesis.getVoices ? speechSynthesis.getVoices() : [];
  const preferred = ["Microsoft Aria", "Microsoft Jenny", "Microsoft Zira", "Google UK English", "Google US English", "Natural"];

  for (const name of preferred) {
    const found = voices.find(v => v.name.toLowerCase().includes(name.toLowerCase()));
    if (found) return found;
  }

  return voices.find(v => v.lang && v.lang.toLowerCase().startsWith("en")) || voices[0] || null;
}

function speak(text) {
  if (!voiceEl.checked || !window.speechSynthesis) return;

  speechSynthesis.cancel();

  const msg = new SpeechSynthesisUtterance(text);
  const voice = bestVoice();

  if (voice) msg.voice = voice;

  msg.rate = 0.92;
  msg.pitch = 0.82;
  msg.volume = 1.0;

  speechSynthesis.speak(msg);
}

async function health() {
  try {
    const res = await fetch("/health", { cache: "no-store" });
    const data = await res.json();

    brainMetric.textContent = data.mode;
    modelMetric.textContent = data.model;
    statusEl.textContent = "LOCAL CORE ONLINE // MODE: " + data.mode.toUpperCase() + " // MODEL: " + data.model;
    addLog("system", "SYSTEM > Local 2027 cockpit online. Ollama model: " + data.model);
  } catch (err) {
    brainMetric.textContent = "offline";
    modelMetric.textContent = "unavailable";
    statusEl.textContent = "LOCAL CORE OFFLINE";
    addLog("system", "SYSTEM > Local core health check failed: " + err.message);
  }
}

async function sendPrompt(text) {
  const clean = text.trim();
  if (!clean) return;

  addLog("user", "ENRICO > " + clean);
  statusEl.textContent = "OLLAMA CORE THINKING...";

  try {
    const res = await fetch("/think", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ text: clean })
    });

    const data = await res.json();

    if (!res.ok) {
      throw new Error(data.error || "Local AI request failed");
    }

    statusEl.textContent = "LOCAL CORE RESPONSE READY // " + data.mode.toUpperCase() + " // " + data.model;
    addLog("lumina", "LUMINA > " + data.reply);
    speak(data.reply);
  } catch (err) {
    statusEl.textContent = "LOCAL AI ERROR";
    addLog("system", "SYSTEM > " + err.message);
  }

  cmdEl.focus();
}

async function send() {
  const text = cmdEl.value;
  cmdEl.value = "";
  await sendPrompt(text);
}

sendBtn.addEventListener("click", send);

cmdEl.addEventListener("keydown", e => {
  if (e.key === "Enter") send();
});

document.querySelectorAll("[data-prompt]").forEach(button => {
  button.addEventListener("click", () => sendPrompt(button.dataset.prompt));
});

if (window.speechSynthesis) {
  speechSynthesis.onvoiceschanged = () => {};
}

health();
</script>
</body>
</html>`;

function sendJson(res, status, payload) {
  res.writeHead(status, {
    ...corsHeaders,
    "Content-Type": "application/json",
    "Cache-Control": "no-store",
  });
  res.end(JSON.stringify(payload));
}

function readBody(req) {
  return new Promise((resolve, reject) => {
    let body = "";
    req.on("data", chunk => {
      body += chunk;
      if (body.length > 16000) {
        reject(new Error("Request too large"));
        req.destroy();
      }
    });
    req.on("end", () => resolve(body));
    req.on("error", reject);
  });
}

function cleanReply(text) {
  return String(text || "")
    .replace(/^LUMINA:\s*/i, "")
    .replace(/^ASSISTANT:\s*/i, "")
    .trim()
    .replace(/^["'“”]+|["'“”]+$/g, "");
}


function violatesTruthRule(text) {
  const value = String(text || "");

  const forbiddenPatterns = [
    /\b\d+(\.\d+)?\s*%/,
    /\b100\s*percent\b/i,
    /\bhealth\s*(is|at|=)\s*\d+/i,
    /\blatency\b/i,
    /\bbandwidth\b/i,
    /\bthreat\s+scan\b/i,
    /\bscan\s+complete\b/i,
    /\bhostiles?\b/i,
    /\ballies?\b/i,
    /\bdetected\b/i,
    /\boptimized\b/i
  ];

  return forbiddenPatterns.some(pattern => pattern.test(value));
}

function safeTruthReply() {
  return "Local Ollama core online. No live telemetry is attached. I can reason over provided data, but I will not invent health, latency, threat, or bandwidth metrics.";
}
async function askOllama(text) {
  const recentContext = memory
    .slice(-8)
    .map(item => item.role.toUpperCase() + ": " + item.content)
    .join("\\n");

  const prompt = luminaInstructions + "\\n\\nRecent cockpit context:\\n" +
    (recentContext || "No prior context.") +
    "\\n\\nUser command:\\n" + text +
    "\\n\\nLumina response:";

  const response = await fetch(OLLAMA_URL + "/api/generate", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      model: OLLAMA_MODEL,
      prompt,
      stream: false,
      options: {
        temperature: 0.78,
        top_p: 0.9,
        repeat_penalty: 1.15,
        num_predict: 160,
      },
    }),
  });

  const raw = await response.text();

  let data = {};
  try {
    data = JSON.parse(raw);
  } catch {
    throw new Error("Ollama returned non-JSON response: " + raw.slice(0, 200));
  }

  if (!response.ok) {
    throw new Error(data.error || raw || "Ollama HTTP " + response.status);
  }

  const reply = cleanReply(data.response);

  if (!reply) {
    throw new Error("Ollama returned no response text");
  }

  return reply;
}

const server = http.createServer(async (req, res) => {
  if (req.method === "OPTIONS") {
    res.writeHead(204, corsHeaders);
    res.end();
    return;
  }

  if (req.method === "GET" && (req.url === "/" || req.url.startsWith("/?") || req.url === "/local")) {
    res.writeHead(200, {
      ...corsHeaders,
      "Content-Type": "text/html; charset=utf-8",
      "Cache-Control": "no-store",
    });
    res.end(localCockpitHtml);
    return;
  }

  if (req.method === "GET" && req.url === "/health") {
    sendJson(res, 200, {
      status: "online",
      mode: "ollama",
      model: OLLAMA_MODEL,
      ollama_url: OLLAMA_URL,
    });
    return;
  }

  if (req.method !== "POST" || req.url !== "/think") {
    sendJson(res, 404, { error: "Not found" });
    return;
  }

  try {
    const body = JSON.parse(await readBody(req));
    const text = String(body.text || "").trim();

    if (!text) {
      sendJson(res, 400, { error: "Missing text" });
      return;
    }

    let reply = await askOllama(text);

    if (violatesTruthRule(reply)) {
      reply = safeTruthReply();
    }

    memory.push({ role: "user", content: text });
    memory.push({ role: "assistant", content: reply });

    while (memory.length > 12) {
      memory.shift();
    }

    sendJson(res, 200, {
      reply,
      mode: "ollama",
      model: OLLAMA_MODEL,
    });
  } catch (error) {
    sendJson(res, 502, {
      error: error.message,
      mode: "ollama",
      model: OLLAMA_MODEL,
    });
  }
});

server.listen(PORT, "127.0.0.1", () => {
  console.log("[LUMINA AI] Local cockpit online: http://127.0.0.1:" + PORT + "/");
  console.log("[LUMINA AI] Health check: http://127.0.0.1:" + PORT + "/health");
  console.log("[LUMINA AI] Mode: ollama");
  console.log("[LUMINA AI] Ollama URL: " + OLLAMA_URL);
  console.log("[LUMINA AI] Ollama model: " + OLLAMA_MODEL);
});
