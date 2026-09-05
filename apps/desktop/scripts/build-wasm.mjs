import { existsSync, mkdirSync, rmSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";
import { spawnSync } from "node:child_process";

const desktop = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const wasmManifest = resolve(desktop, "bridge-wasm", "Cargo.toml");
const output = resolve(desktop, "src", "benchmark", "wasm");
const root = resolve(desktop, "..", "..");
const wasmArtifact = resolve(
  desktop,
  "target",
  "wasm32-unknown-unknown",
  "release",
  "street_concept_designer_bridge_wasm.wasm",
);
const localBin = resolve(desktop, ".tools", "wasm-bindgen", "bin");
const wasmBindgen = process.env.R3B_WASM_BINDGEN
  ? resolve(process.env.R3B_WASM_BINDGEN)
  : resolve(localBin, process.platform === "win32" ? "wasm-bindgen.exe" : "wasm-bindgen");

function run(command, args) {
  const result = spawnSync(command, args, { cwd: root, stdio: "inherit", shell: false });
  if (result.status !== 0) process.exit(result.status ?? 1);
}

run("cargo", [
  "build",
  "--manifest-path",
  wasmManifest,
  "--target",
  "wasm32-unknown-unknown",
  "--release",
]);

if (!existsSync(wasmBindgen) && !process.env.R3B_WASM_BINDGEN) {
  mkdirSync(resolve(desktop, ".tools"), { recursive: true });
  run("cargo", [
    "install",
    "--locked",
    "--version",
    "0.2.127",
    "wasm-bindgen-cli",
    "--root",
    resolve(desktop, ".tools", "wasm-bindgen"),
  ]);
}

rmSync(output, { recursive: true, force: true });
mkdirSync(output, { recursive: true });
run(wasmBindgen, ["--target", "web", "--out-dir", output, "--out-name", "bridge_wasm", wasmArtifact]);
