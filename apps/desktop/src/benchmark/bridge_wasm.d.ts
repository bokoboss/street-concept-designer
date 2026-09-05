export {};

declare module "./wasm/bridge_wasm.js" {
  export default function init(input?: unknown): Promise<unknown>;
  export class WasmBridge {
    constructor();
    revision(): bigint;
    scene(size: string): Uint8Array;
    preview_scene(sample: number): Uint8Array;
    commit(sample: number): bigint;
    reset(): bigint;
    stale_probe(): void;
  }
}

declare module "./benchmark/wasm/bridge_wasm.js" {
  export default function init(input?: unknown): Promise<unknown>;
  export class WasmBridge {
    constructor();
    revision(): bigint;
    scene(size: string): Uint8Array;
    preview_scene(sample: number): Uint8Array;
    commit(sample: number): bigint;
    reset(): bigint;
    stale_probe(): void;
  }
}
