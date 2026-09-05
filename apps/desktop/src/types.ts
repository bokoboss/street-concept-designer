export type PrimitiveKind = "polygon" | "polyline";

export interface ScenePrimitive {
  kind: PrimitiveKind;
  role: number;
  semanticId: string;
  vertices: Array<[number, number]>;
}

export interface ScenePacket {
  revision: number;
  renderOrigin: [number, number];
  primitiveCount: number;
  semanticIdCount: number;
  coordinateCount: number;
  payloadBytes: number;
  primitives: ScenePrimitive[];
}

export function toBytes(value: unknown): Uint8Array {
  if (value instanceof Uint8Array) return value;
  if (value instanceof ArrayBuffer) return new Uint8Array(value);
  if (ArrayBuffer.isView(value)) {
    return new Uint8Array(value.buffer, value.byteOffset, value.byteLength);
  }
  if (Array.isArray(value)) return Uint8Array.from(value as number[]);
  throw new Error("bridge returned a non-byte scene packet");
}

export function decodeScene(value: unknown): ScenePacket {
  const bytes = toBytes(value);
  if (bytes.byteLength < 48) throw new Error("scene packet is truncated");
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const magic = new TextDecoder().decode(bytes.subarray(0, 4));
  if (magic !== "R3B2") throw new Error("scene packet magic mismatch");
  if (view.getUint32(4, true) !== 1) throw new Error("unsupported scene packet version");

  const revision = Number(view.getBigUint64(8, true));
  const renderOrigin: [number, number] = [view.getFloat64(16, true), view.getFloat64(24, true)];
  const primitiveCount = view.getUint32(32, true);
  const semanticIdCount = view.getUint32(36, true);
  const coordinateCount = view.getUint32(40, true);
  const payloadBytes = view.getUint32(44, true);
  if (payloadBytes !== bytes.byteLength - 48) throw new Error("scene packet payload length mismatch");

  const decoder = new TextDecoder("utf-8", { fatal: true });
  let offset = 48;
  const primitives: ScenePrimitive[] = [];
  const semanticIds = new Set<string>();
  for (let index = 0; index < primitiveCount; index += 1) {
    if (offset + 8 > bytes.byteLength) throw new Error("scene primitive header is truncated");
    const kindCode = bytes[offset];
    const role = bytes[offset + 1];
    const idLength = view.getUint16(offset + 2, true);
    const vertexCount = view.getUint32(offset + 4, true);
    offset += 8;
    const vertexBytes = vertexCount * 8;
    if (offset + idLength + vertexBytes > bytes.byteLength) {
      throw new Error("scene primitive payload is truncated");
    }
    const semanticId = decoder.decode(bytes.subarray(offset, offset + idLength));
    semanticIds.add(semanticId);
    offset += idLength;
    const vertices: Array<[number, number]> = [];
    for (let vertex = 0; vertex < vertexCount; vertex += 1) {
      const x = view.getFloat32(offset, true);
      const y = view.getFloat32(offset + 4, true);
      if (!Number.isFinite(x) || !Number.isFinite(y)) throw new Error("scene packet has non-finite coordinates");
      vertices.push([x, y]);
      offset += 8;
    }
    if (kindCode !== 0 && kindCode !== 1) throw new Error("unknown scene primitive kind");
    primitives.push({
      kind: kindCode === 1 ? "polygon" : "polyline",
      role,
      semanticId,
      vertices,
    });
  }
  if (offset !== bytes.byteLength || semanticIds.size !== semanticIdCount || coordinateCount !== primitives.reduce((sum, primitive) => sum + primitive.vertices.length * 2, 0)) {
    throw new Error("scene packet coordinate count mismatch");
  }
  return { revision, renderOrigin, primitiveCount, semanticIdCount, coordinateCount, payloadBytes, primitives };
}

export function sceneBounds(scene: ScenePacket): [number, number, number, number] {
  const points = scene.primitives.flatMap((primitive) => primitive.vertices);
  if (points.length === 0) return [0, 0, 1, 1];
  const xs = points.map(([x]) => x);
  const ys = points.map(([, y]) => y);
  return [Math.min(...xs), Math.min(...ys), Math.max(...xs), Math.max(...ys)];
}
