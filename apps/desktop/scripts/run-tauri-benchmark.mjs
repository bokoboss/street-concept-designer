import { writeFile } from "node:fs/promises";
import { spawn } from "node:child_process";
import { resolve } from "node:path";

const root = resolve(import.meta.dirname, "..");
const executable = resolve(root, "target", "release", "street-concept-designer-desktop.exe");
const port = Number(process.env.R3B_CDP_PORT ?? 9247);
const outputPath = resolve(root, process.env.R3B_BENCHMARK_OUTPUT ?? "benchmark-result.json");
const child = spawn(executable, [], {
  cwd: root,
  env: {
    ...process.env,
    WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS: `--remote-debugging-port=${port}`,
  },
  windowsHide: true,
});

const sleep = (milliseconds) => new Promise((resolvePromise) => setTimeout(resolvePromise, milliseconds));

async function waitForTarget() {
  for (let attempt = 0; attempt < 30; attempt += 1) {
    await sleep(1000);
    try {
      const response = await fetch(`http://127.0.0.1:${port}/json`);
      const targets = await response.json();
      if (targets[0]) return targets[0];
    } catch {}
  }
  throw new Error("WebView2 CDP target not found");
}

let socket;
try {
  const target = await waitForTarget();
  socket = new WebSocket(target.webSocketDebuggerUrl);
  let nextId = 0;
  const pending = new Map();
  socket.addEventListener("message", (event) => {
    const message = JSON.parse(event.data);
    const resolvePending = pending.get(message.id);
    if (resolvePending) {
      pending.delete(message.id);
      resolvePending(message);
    }
  });

  await new Promise((resolvePromise, reject) => {
    socket.addEventListener("open", resolvePromise);
    socket.addEventListener("error", reject);
  });

  function call(method, params = {}) {
    return new Promise((resolvePromise, reject) => {
      const id = ++nextId;
      pending.set(id, resolvePromise);
      socket.send(JSON.stringify({ id, method, params }));
      setTimeout(() => {
        if (pending.delete(id)) reject(new Error(`${method} timed out`));
      }, 30_000);
    });
  }

  async function evaluate(expression) {
    const response = await call("Runtime.evaluate", { expression, returnByValue: true });
    if (response.error) throw new Error(response.error.message);
    if (response.result?.exceptionDetails) {
      throw new Error(response.result.exceptionDetails.exception?.description ?? "Runtime evaluation failed");
    }
    return response.result?.result?.value;
  }

  await sleep(5000);
  await call("Page.navigate", { url: "http://tauri.localhost/benchmark.html" });
  let ready = false;
  for (let attempt = 0; attempt < 45; attempt += 1) {
    await sleep(1000);
    const state = JSON.parse(await evaluate("JSON.stringify({button:!!document.querySelector('button'), error:document.querySelector('.error')?.textContent||null})"));
    if (state.error) throw new Error(state.error);
    if (state.button) {
      ready = true;
      break;
    }
  }
  if (!ready) throw new Error("benchmark page did not become ready");
  await evaluate("document.querySelector('button').click()");

  let result;
  for (let attempt = 0; attempt < 240; attempt += 1) {
    await sleep(1000);
    const state = JSON.parse(await evaluate("JSON.stringify({result:document.getElementById('benchmark-result')?.textContent||null, error:document.querySelector('.error')?.textContent||null})"));
    if (state.error) throw new Error(state.error);
    if (state.result) {
      result = JSON.parse(state.result);
      break;
    }
  }
  if (!result) throw new Error("benchmark did not complete within 240 seconds");
  await writeFile(outputPath, `${JSON.stringify(result, null, 2)}\n`);
  console.log(JSON.stringify(result));
} finally {
  socket?.close();
  child.kill();
}
