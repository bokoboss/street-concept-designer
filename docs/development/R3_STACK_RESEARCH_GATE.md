# R3 Editor Stack Research Gate

Date: 2026-09-02

Status: **GO WITH CONDITIONS**

## Decision being researched

Choose the minimum current frontend/desktop/2D-rendering stack suitable for the R3 2D Road Authoring Alpha without:
- weakening the accepted Rust engineering/project core;
- making renderer state engineering truth;
- requiring development tools on end-user machines;
- prematurely adopting 3D/map/provider dependencies;
- hiding the current local Windows toolchain limitation.

This record is a dependency/feasibility gate, not permission to implement all of R3.

## Evidence sources

Primary/current upstream evidence reviewed on 2026-09-02:

- Tauri releases: https://tauri.app/release/tauri/all-versions/
- Tauri Rust/frontend command IPC: https://v2.tauri.app/develop/calling-rust/
- Tauri frontend-from-Rust communication: https://v2.tauri.app/develop/calling-frontend/
- Tauri state management: https://v2.tauri.app/develop/state-management/
- Tauri permissions/capabilities: https://v2.tauri.app/security/permissions/
- Tauri Windows prerequisites: https://v2.tauri.app/start/prerequisites/
- Vite releases: https://vite.dev/releases
- Vite 8 announcement / Node support: https://vite.dev/blog/announcing-vite8
- React versions: https://react.dev/versions
- TypeScript 7 announcement: https://devblogs.microsoft.com/typescript/announcing-typescript-7-0/
- PixiJS renderer guidance: https://pixijs.com/8.x/guides/components/renderers
- current npm package metadata for exact candidate versions/licenses
- Playwright npm package metadata for browser E2E qualification

## Current candidate versions

These are **R3A adoption candidates**, not locked repository dependencies until R3A resolves and records the actual lockfiles.

| Component | Current candidate | License / evidence | R3 position |
|---|---:|---|---|
| Rust Tauri crate | 2.11.5 | Tauri project MIT/Apache-2.0 family | Preferred desktop shell |
| @tauri-apps/api | 2.11.1 | Apache-2.0 OR MIT | Preferred frontend IPC API |
| @tauri-apps/cli | 2.11.4 | Apache-2.0 OR MIT | Dev/build only |
| React | 19.2.8 | MIT | Preferred workspace UI |
| React DOM | 19.2.8 | MIT | Preferred DOM renderer |
| Vite | 8.2.2 | MIT | Preferred dev/build tool |
| @vitejs/plugin-react | 6.1.1 | MIT | Preferred React/Vite integration |
| TypeScript | 7.0.2 | Apache-2.0 | Preferred type-checker, compatibility-gated |
| PixiJS | 8.20.1 | MIT | Preferred R3 2D renderer |
| @types/react | 19.2.18 | MIT | Type definitions |
| @types/react-dom | 19.2.5 | MIT | Type definitions |
| @playwright/test | 1.62.1 | Apache-2.0 | Preferred browser E2E/visual-flow test runner |
| Node.js | 24.20.0 LTS | development toolchain only | Preferred R3 frontend build runtime |
| npm | version bundled/qualified with chosen Node | development toolchain only | Preferred package manager; commit package-lock |

Exact transitive dependency versions/licenses must be recorded after the R3A npm/Cargo lockfiles resolve.

## Tauri verdict

**GO WITH CONDITIONS.**

Why it fits:
- Rust backend can directly own/use the accepted ProjectSession and project-io architecture.
- Tauri command IPC is appropriate for bounded request/response application commands.
- Tauri managed state supports a normal Rust `Mutex` when no guard is held across await points.
- Windows uses WebView2, consistent with the accepted Windows distribution policy.
- Bundled frontend code can be kept local/offline.

Important conditions:
1. Do not expose remote web origins to engineering commands.
2. Use the Tauri capability/permission model explicitly; grant only the commands required by the current R3 packet.
3. Tauri 2.11.1 contained important custom-command/remote-origin ACL security fixes; do not adopt an older 2.11.0 or earlier baseline.
4. Keep the Tauri crate thin. Application/domain behavior belongs in a testable Rust bridge/backend package above project-session, not inside command macros.
5. Do not add shell/fs/http plugins before a packet actually needs them.

## Rust <-> frontend bridge verdict

**Use Tauri command IPC for R3A, with an explicit transport DTO boundary.**

Required architecture:

```text
React/Pixi frontend
       |
  typed TS client
       |
   Tauri invoke
       |
thin Tauri command adapter
       |
editor/application bridge DTO + backend
       |
ProjectSession / kernel
```

Do not:
- send canonical project JSON and let the frontend edit it;
- expose mutable Rust model state;
- make Pixi objects command parameters;
- duplicate engineering calculations in TypeScript.

R3A should use coarse request/response commands such as:
- workspace/project summary;
- scenario list;
- derived read-only 2D scene snapshot.

For later high-frequency direct manipulation:
- Tauri events are explicitly not intended for low-latency/high-throughput streams;
- do not build drag preview on events;
- first benchmark ordinary invoke/request-response preview in R3C;
- if representative preview cannot support an acceptable interactive cadence, research Channels or raw/binary response at that point.

No binary protocol is justified in R3A.

## Transport DTO policy

Create an application/transport DTO distinct from:
- R2B canonical project persistence DTO;
- private Rust runtime layouts;
- Pixi display objects.

The transport DTO may use serde above the kernel.

For read-only 2D scene data, transport:
- project/scenario identity;
- ProjectSemanticRef-compatible semantic identities;
- renderer-neutral polygon/polyline geometry from the accepted R1C derivation;
- role/category/style hint;
- optional world extents.

Do not transport:
- private Junction derived state not already represented by the accepted renderer contract;
- canonical project-file bytes as an editor mutation mechanism;
- GPU handles;
- DOM/Pixi ids as semantic ids.

## PixiJS verdict

**GO for R3A 2D prototype using WebGL.**

Evidence:
- current PixiJS supports WebGL/WebGL2 and WebGPU;
- PixiJS documents WebGL as the stable/recommended production renderer;
- WebGPU is still described as maturing/experimental for production-browser consistency.

R3 policy:
- explicitly choose WebGL for R3A;
- do not enable a WebGPU production path yet;
- do not add a Canvas fallback architecture unless actual WebView2 evidence requires it;
- do not adopt `@pixi/react` initially. Mount/manage Pixi directly inside a React lifecycle wrapper to avoid another binding dependency before evidence.

Pixi may:
- draw shared 2D polygons/polylines;
- apply world-to-screen transform;
- own pan/zoom;
- do hit testing;
- attach semantic-reference metadata to display objects;
- draw selection/hover styling.

Pixi must not compute lane widths, offsets, lifecycle, corner geometry, or topology.

## React/Vite/TypeScript verdict

**GO WITH CONDITIONS.**

- React 19.2.x is the current stable major/minor family.
- Vite 8.2.x is the current supported minor and requires Node >=20.19 or >=22.12; Node 24 LTS satisfies this.
- TypeScript 7.0.2 is a stable production release and is designed to preserve TypeScript 6 checking semantics while using the new native implementation.

R3A conditions:
1. Pin direct versions in package.json/package-lock for qualification.
2. Use strict TypeScript settings.
3. Do not enable experimental React Compiler or Vite experimental features.
4. TypeScript 7 is accepted only if the actual R3A React/Vite/Pixi/Playwright build and type-check are clean without compatibility shims. If ecosystem compatibility is materially problematic, stop and compare the still-supported TypeScript 6.0.x line rather than patching around the compiler.
5. Use npm + committed package-lock to avoid requiring an additional global package manager.

## Playwright verdict

**GO for frontend/browser qualification.**

Use Playwright against a web-mode/mock bridge build to test:
- workspace layout;
- Select vs Hand behavior;
- pan/zoom;
- semantic selection;
- inspector synchronization;
- resize behavior;
- keyboard/Escape safety where implemented.

This does not substitute for native Tauri qualification.

Workspace-safety condition:
- local Playwright browser installation must use hermetic project-local mode, e.g. `PLAYWRIGHT_BROWSERS_PATH=0`, which places browser binaries under the local Playwright package;
- do not run local `playwright install --with-deps` / `install-deps`, because that may mutate system packages;
- hosted CI may install ephemeral runner dependencies as part of the CI job.

## Windows local-development constraint

Tauri's official Windows prerequisites require:
- Microsoft C++ Build Tools;
- Microsoft Edge WebView2.

The current project workstation has repeatedly qualified Rust only through local GNU fallback because `cl.exe/link.exe` are absent. Tauri does not officially support Windows GNU as the normal Windows development path.

Therefore R3A must not silently install Visual Studio Build Tools or change the machine.

Qualification strategy:
1. Local/web visual development and Playwright through the Vite frontend/mock bridge if Node is already available.
2. Rust bridge/application logic through normal Cargo tests that do not require linking the Tauri Windows shell locally.
3. Keep the Tauri shell outside the default root Cargo workspace regression surface (for example via `workspace.exclude`) so Linux `cargo test --workspace` does not acquire GTK/WebKit system-dependency obligations solely because the Windows shell exists.
4. Hosted Windows/MSVC CI builds the actual Tauri application.
5. Hosted Linux may build frontend/Rust layers but does not prove Windows WebView2 runtime.
6. Do not claim actual desktop runtime UAT until a built Windows artifact is exercised on a qualified Windows machine.

If local Node is missing, do not globally install it automatically. Report the environment blocker and use hosted frontend qualification unless an explicitly approved project-local strategy exists.

## Security boundary

R3A shell should:
- load only bundled/local frontend content;
- define the narrowest Tauri capability for current custom commands;
- avoid remote-origin capabilities;
- avoid shell/process/network/filesystem plugins;
- return structured errors rather than raw Rust panic/debug internals;
- keep renderer/front-end failure incapable of corrupting ProjectSession state.

## End-user runtime rule

Node/npm/Vite/TypeScript/Rust/Cargo/MSVC are build-time only.

The final user machine must not require any of them.

R3A adoption of this stack does not change the existing Windows distribution policy:
- per-user/no-admin installer later;
- Portable Light later;
- Portable Offline evidence later;
- WebView2 runtime strategy qualified in release engineering.

## 3D / map decisions

Do not adopt Three.js in R3A.

R1 already proved a diagnostic 3D derivation; R3 is 2D-primary and the next unknown is production 2D/editor integration.

Do not adopt MapLibre in R3.

R3 reference scope is user image/site-plan calibration. Online basemap/provider integration remains later (roadmap R7) and carries provider-rights complexity.

## Newly discovered R3 semantic blocker

Current accepted `Road` holds one `Alignment` primitive:
- Line;
- CircularArc;
- SmoothConceptualCurve.

The current UX/product language expects a road alignment that can continue through multiple line/arc/smooth segments.

This is not a renderer problem and must not be faked by:
- creating several independent Roads;
- stitching Pixi paths;
- storing a frontend-only polyline.

R3 therefore needs a separately gated **compound/reference alignment** semantic packet before multi-segment road authoring can be accepted.

The application shell/2D renderer can be proven first against current canonical fixtures.

## Research-gate decision

**GO WITH CONDITIONS.**

R3A may adopt the Tauri/React/Vite/Pixi stack under the conditions above.

R3 road-authoring implementation is blocked on a separately reviewed compound-alignment semantic packet unless the product explicitly chooses a one-primitive Road limitation. Current product intent favors solving the semantic gap rather than presenting a misleading authoring UI.
