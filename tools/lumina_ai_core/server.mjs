import http from "node:http";

const PORT = 8788;
const API_KEY = process.env.OPENAI_API_KEY;
const MODEL = process.env.OPENAI_MODEL || "gpt-5.6";

const memory = [];

const corsHeaders = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Methods": "POST, OPTIONS",
  "Access-Control-Allow-Headers": "Content-Type",
};

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

function extractText(data) {
  if (typeof data.output_text === "string" && data.output_text.trim()) {
    return data.output_text.trim();
  }

  const parts = [];

  for (const item of data.output || []) {
    for (const content of item.content || []) {
      if (typeof content.text === "string") {
        parts.push(content.text);
      }
    }
  }

  return parts.join(" ").trim();
}

const luminaInstructions = `
You are Lumina, Enrico's local sovereign cockpit assistant.

Personality:
- cinematic, calm, intelligent, cool, sharp, protective
- sounds like a premium system core, not a chatbot
- concise by default: 1 to 3 sentences
- address the user as Enrico sometimes, not every line
- never pretend you executed real system actions unless the user only asked for a concept
- do not claim access to files, hardware, private repos, sensors, or secrets unless explicitly provided in the current message
- for coding or terminal requests, give exact practical commands
- for casual commands, respond with style and confidence

System identity:
- name: Lumina
- role: local-first AI cockpit interface
- public mode: GitHub Pages interface
- private brain: optional localhost AI core
- mission: assist with systems design, engineering workflow, security-focused planning, and command interpretation

Output style:
- no markdown unless useful
- no long essays
- make it feel alive, controlled, and premium
`;

const server = http.createServer(async (req, res) => {
  if (req.method === "OPTIONS") {
    res.writeHead(204, corsHeaders);
    res.end();
    return;
  }

  if (req.method !== "POST" || req.url !== "/think") {
    sendJson(res, 404, { error: "Not found" });
    return;
  }

  if (!API_KEY) {
    sendJson(res, 500, { error: "OPENAI_API_KEY is not set" });
    return;
  }

  try {
    const body = JSON.parse(await readBody(req));
    const text = String(body.text || "").trim();

    if (!text) {
      sendJson(res, 400, { error: "Missing text" });
      return;
    }

    memory.push({ role: "user", content: text });
    while (memory.length > 10) memory.shift();

    const response = await fetch("https://api.openai.com/v1/responses", {
      method: "POST",
      headers: {
        "Authorization": `Bearer ${API_KEY}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        model: MODEL,
        instructions: luminaInstructions,
        input: memory,
      }),
    });

    const data = await response.json();

    if (!response.ok) {
      sendJson(res, response.status, {
        error: data.error?.message || JSON.stringify(data),
      });
      return;
    }

    const reply = extractText(data) || "Neural core returned silence. Local logic remains online.";

    memory.push({ role: "assistant", content: reply });
    while (memory.length > 10) memory.shift();

    sendJson(res, 200, { reply });
  } catch (error) {
    sendJson(res, 500, { error: error.message });
  }
});

server.listen(PORT, "127.0.0.1", () => {
  console.log(`[LUMINA AI] Neural core online: http://127.0.0.1:${PORT}`);
  console.log(`[LUMINA AI] Model: ${MODEL}`);
  console.log("[LUMINA AI] API key is read from environment only. Do not commit secrets.");
});
