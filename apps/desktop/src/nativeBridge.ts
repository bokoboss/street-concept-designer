import { invoke } from "@tauri-apps/api/core";
import { decodeScene, type ScenePacket } from "./types";

export interface BridgeInfo {
  bridge_owner: "native-tauri";
  revision: number;
}

export class NativeBridge {
  async info(): Promise<BridgeInfo> {
    return invoke<BridgeInfo>("bridge_info");
  }

  async scene(size: "S" | "M" | "L"): Promise<ScenePacket> {
    return decodeScene(await invoke<unknown>("bridge_scene", { size }));
  }

  async previewScene(sample: number): Promise<ScenePacket> {
    return decodeScene(await invoke<unknown>("bridge_preview_scene", { sample }));
  }

  async commit(sample: number): Promise<number> {
    return invoke<number>("bridge_commit", { sample });
  }

  async reset(): Promise<number> {
    return invoke<number>("bridge_reset");
  }

  async staleProbe(): Promise<void> {
    await invoke("bridge_stale_probe");
  }
}
