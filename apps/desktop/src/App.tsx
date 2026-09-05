import { useEffect, useRef, useState } from "react";
import "pixi.js/unsafe-eval";
import { Application, Container, Graphics, RendererType } from "pixi.js";
import { bridgeInfo, bridgeScene } from "./wasmBridge";
import { sceneBounds, sceneSignature, type ScenePacket, type ScenePrimitive } from "./types";

function roleLabel(role: number): string {
  return ["road alignment", "road component surface", "junction surface", "corner curve"][role] ?? "derived primitive";
}

function primitiveColor(primitive: ScenePrimitive): number {
  if (primitive.role === 0) return 0x94a3b8;
  if (primitive.semanticId.includes("lane-left")) return 0x38bdf8;
  if (primitive.semanticId.includes("lane-right")) return 0x22c55e;
  return 0xf59e0b;
}

function buildRenderer(
  world: Container,
  scene: ScenePacket,
  selectedId: string | null,
  select: (id: string) => void,
  width: number,
  height: number,
) {
  world.removeChildren().forEach((child) => child.destroy({ children: true }));
  const [minX, minY, maxX, maxY] = sceneBounds(scene);
  const scale = Math.min((width - 80) / Math.max(maxX - minX, 1), (height - 80) / Math.max(maxY - minY, 1));
  world.scale.set(Math.max(scale, 0.1));
  world.position.set((width - (maxX - minX) * scale) / 2 - minX * scale, (height - (maxY - minY) * scale) / 2 - minY * scale);

  for (const primitive of scene.primitives) {
    const graphics = new Graphics();
    const [first, ...rest] = primitive.vertices;
    if (!first) continue;
    graphics.moveTo(first[0], first[1]);
    for (const [x, y] of rest) graphics.lineTo(x, y);
    if (primitive.kind === "polygon") graphics.closePath();
    const selected = primitive.semanticId === selectedId;
    const color = primitiveColor(primitive);
    if (primitive.kind === "polygon") graphics.fill({ color, alpha: selected ? 0.85 : 0.45 });
    graphics.stroke({ color: selected ? 0xfef08a : color, width: selected ? 2.5 / Math.max(scale, 0.1) : 1 / Math.max(scale, 0.1), alpha: 0.95 });
    graphics.label = primitive.semanticId;
    graphics.eventMode = "static";
    graphics.cursor = "pointer";
    graphics.on("pointertap", () => select(primitive.semanticId));
    world.addChild(graphics);
  }
}

export function App() {
  const hostRef = useRef<HTMLDivElement>(null);
  const pixiRef = useRef<Application | null>(null);
  const worldRef = useRef<Container | null>(null);
  const [scene, setScene] = useState<ScenePacket | null>(null);
  const [owner, setOwner] = useState("loading");
  const [revision, setRevision] = useState<number | null>(null);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [webgl, setWebgl] = useState("pending");
  const [rebuild, setRebuild] = useState("not run");
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let disposed = false;
    const host = hostRef.current;
    if (!host) return;
    const application = new Application();
    pixiRef.current = application;
    void (async () => {
      try {
        await application.init({ preference: "webgl", resizeTo: host, antialias: true, background: 0x0b1220 });
        if (disposed) return;
        if (application.renderer.type !== RendererType.WEBGL) throw new Error(`Pixi renderer was not WebGL: ${application.renderer.type}`);
        setWebgl("WebGL forced");
        host.appendChild(application.canvas);
        const world = new Container();
        worldRef.current = world;
        application.stage.addChild(world);
        const info = await bridgeInfo();
        const initialScene = await bridgeScene("S");
        if (disposed) return;
        setOwner(info.bridge_owner);
        setRevision(initialScene.revision);
        setScene(initialScene);
      } catch (caught) {
        setError(caught instanceof Error ? caught.message : String(caught));
      }
    })();
    return () => {
      disposed = true;
      worldRef.current?.removeChildren().forEach((child) => child.destroy({ children: true }));
      worldRef.current?.destroy({ children: true });
      application.destroy(true);
      pixiRef.current = null;
    };
  }, []);

  useEffect(() => {
    if (!scene || !worldRef.current) return;
    buildRenderer(
      worldRef.current,
      scene,
      selectedId,
      setSelectedId,
      hostRef.current?.clientWidth ?? 900,
      hostRef.current?.clientHeight ?? 560,
    );
  }, [scene, selectedId]);

  const rebuildRenderer = () => {
    if (!scene || !worldRef.current) return;
    const before = sceneSignature(scene);
    buildRenderer(
      worldRef.current,
      scene,
      selectedId,
      setSelectedId,
      hostRef.current?.clientWidth ?? 900,
      hostRef.current?.clientHeight ?? 560,
    );
    setRebuild(before === sceneSignature(scene) ? "equivalent" : "mismatch");
  };

  return (
    <main className="app-shell">
      <header className="topbar">
        <div>
          <p className="eyebrow">R3B desktop runtime / renderer proof</p>
          <h1>Street Concept Designer</h1>
        </div>
        <div className="status-pill">{error ? "Controlled error" : "Local fixture"}</div>
      </header>
      <section className="workspace">
        <div ref={hostRef} className="canvas-host" aria-label="Derived 2D engineering scene" />
        <aside className="diagnostics">
          <h2>Proof diagnostics</h2>
          {error && <p className="error" role="alert">{error}</p>}
          <dl>
            <dt>Renderer</dt><dd>{webgl}</dd>
            <dt>Bridge owner</dt><dd>{owner}</dd>
            <dt>Session revision</dt><dd>{revision ?? "—"}</dd>
            <dt>Selected semantic id</dt><dd className="mono">{selectedId ?? "click a lane/surface"}</dd>
            <dt>Primitive count</dt><dd>{scene?.primitiveCount ?? "—"}</dd>
            <dt>Local render origin</dt><dd className="mono">{scene ? `${scene.renderOrigin[0]}, ${scene.renderOrigin[1]}` : "—"}</dd>
            <dt>Cache rebuild</dt><dd>{rebuild}</dd>
          </dl>
          <button type="button" onClick={rebuildRenderer} disabled={!scene}>Destroy + rebuild renderer cache</button>
          {import.meta.env.DEV && <a className="benchmark-link" href="/benchmark.html">Open bridge comparator</a>}
          <p className="note">Renderer objects are disposable. Engineering geometry arrives only as the derived R1C scene packet.</p>
        </aside>
      </section>
      {scene && selectedId && (
        <footer className="selection-footer">
          Selected: <span className="mono">{selectedId}</span> · derived role: {roleLabel(scene.primitives.find((item) => item.semanticId === selectedId)?.role ?? -1)}
        </footer>
      )}
    </main>
  );
}
