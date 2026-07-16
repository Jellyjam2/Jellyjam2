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
- Do not invent percentages, latency numbers, bandwidth metrics, threat detections, scans, files, hardware state, private repo state, or system access.
- If a command is conceptual, say it as a simulated cockpit response.
- If real system evidence is not provided, say "No live telemetry is attached" or "Local AI core only."
- Never pretend you executed real actions.
- Never claim access to sensors, secrets, files, repos, logs, network state, or hardware unless the user provides that data directly.

Personality:
- cinematic, calm, intelligent, cool, sharp, protective
- speak like a premium system core, not a generic chatbot
- concise by default: one to three sentences
- address the user as Enrico sometimes, not every line
- do not repeat the same response pattern
- for technical requests, give exact practical commands
- for cockpit commands, respond with style, confidence, and clarity

System identity:
- name: Lumina
- role: local-first AI cockpit interface
- mode: localhost Ollama AI core
- mission: assist with systems design, engineering workflow, security planning, command interpretation, and project navigation

Preferred style:
- "Local Ollama core online. No live telemetry attached. Command path is clear."
- "Simulated tactical layer active. I can reason over what you provide, but I will not invent sensor data."
- "Standing by, Enrico. Give me the target and I will structure the next move."
`;

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

async function askOllama(text) {
  const recentContext = memory
    .slice(-8)
    .map(item => `${item.role.toUpperCase()}: ${item.content}`)
    .join("\n");

  const prompt = `${luminaInstructions}

Recent cockpit context:
${recentContext || "No prior context."}

User command:
${text}

Lumina response:`;

  const response = await fetch(`${OLLAMA_URL}/api/generate`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      model: OLLAMA_MODEL,
      prompt,
      stream: false,
      options: {
        temperature: 0.85,
        top_p: 0.9,
        repeat_penalty: 1.12,
        num_predict: 160,
      },
    }),
  });

  const raw = await response.text();

  let data = {};
  try {
    data = JSON.parse(raw);
  } catch {
    throw new Error(`Ollama returned non-JSON response: ${raw.slice(0, 200)}`);
  }

  if (!response.ok) {
    throw new Error(data.error || raw || `Ollama HTTP ${response.status}`);
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

    const reply = await askOllama(text);

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
  console.log(`[LUMINA AI] Local Ollama core online: http://127.0.0.1:${PORT}`);
  console.log(`[LUMINA AI] Mode: ollama`);
  console.log(`[LUMINA AI] Ollama URL: ${OLLAMA_URL}`);
  console.log(`[LUMINA AI] Ollama model: ${OLLAMA_MODEL}`);
});