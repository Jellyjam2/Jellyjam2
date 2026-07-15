import http from "node:http";

const PORT = 8787;
const API_KEY = process.env.OPENAI_API_KEY;

const corsHeaders = {
  "Access-Control-Allow-Origin": "*",
  "Access-Control-Allow-Methods": "POST, OPTIONS",
  "Access-Control-Allow-Headers": "Content-Type",
};

function sendJson(res, status, payload) {
  res.writeHead(status, {
    ...corsHeaders,
    "Content-Type": "application/json",
  });
  res.end(JSON.stringify(payload));
}

function readBody(req) {
  return new Promise((resolve, reject) => {
    let body = "";
    req.on("data", chunk => {
      body += chunk;
      if (body.length > 12000) {
        reject(new Error("Request too large"));
        req.destroy();
      }
    });
    req.on("end", () => resolve(body));
    req.on("error", reject);
  });
}

const server = http.createServer(async (req, res) => {
  if (req.method === "OPTIONS") {
    res.writeHead(204, corsHeaders);
    res.end();
    return;
  }

  if (req.method !== "POST" || req.url !== "/speak") {
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

    const response = await fetch("https://api.openai.com/v1/audio/speech", {
      method: "POST",
      headers: {
        "Authorization": `Bearer ${API_KEY}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        model: "gpt-4o-mini-tts",
        voice: "cedar",
        input: text,
        instructions: "Speak as Lumina: calm, premium, intelligent, warm, controlled, confident, cinematic, and non-robotic. Use natural pauses and clear articulation.",
        response_format: "mp3",
      }),
    });

    if (!response.ok) {
      const errorText = await response.text();
      sendJson(res, response.status, { error: errorText });
      return;
    }

    const audio = Buffer.from(await response.arrayBuffer());

    res.writeHead(200, {
      ...corsHeaders,
      "Content-Type": "audio/mpeg",
      "Cache-Control": "no-store",
    });
    res.end(audio);
  } catch (error) {
    sendJson(res, 500, { error: error.message });
  }
});

server.listen(PORT, "127.0.0.1", () => {
  console.log(`[LUMINA VOICE] Local proxy online: http://127.0.0.1:${PORT}`);
  console.log("[LUMINA VOICE] Do not paste or commit your API key.");
});
