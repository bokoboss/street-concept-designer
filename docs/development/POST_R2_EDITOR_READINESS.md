# Post-R2 Editor Readiness

Date: 2026-09-03

## Status

**R2 ACCEPTED / COMPLETED. R3A COMPOSITE ALIGNMENT ACCEPTED / COMPLETED. R3B DESKTOP RUNTIME / BINDING / 2D RENDERER PROOF IS THE NEXT SEPARATELY GATED PACKET.**

Accepted R2 main SHA:
`fb933bb9c5da49225c3d6b7e52176e270c117cce`

R2 accepted:
- R2A Project / Scenario Domain Core;
- R2B Versioned Persistence & Migration;
- R2C Command / Transaction / Undo / Redo.

Final R2C-head hosted qualification:
- R2C `33595026496`: PASS
- R2B `33595026419`: PASS
- R2A `33595026470`: PASS
- R1A `33595026424`: PASS
- R1B `33595026436`: PASS
- R1C `33595026445`: PASS
- Engineering Workflow Integrity `33595026428`: PASS

## What R1 + R2 now provide

The project has evidence-backed foundations for:
- Rust semantic/geometry kernel;
- line, circular-arc, and smooth conceptual alignment primitives;
- station-based cross sections and exact lifecycle breakpoints;
- junction candidate/topology separation;
- deterministic shared 2D/3D derivation;
- semantic renderer identity and local render origin;
- Project/Scenario identity and isolation;
- strict versioned canonical JSON persistence;
- deterministic migration boundary;
- typed atomic preview/commit;
- monotonic stale-proposal-safe session revision;
- lock enforcement;
- exact snapshot undo/redo.

## Post-R2 scrutiny findings

### 1. Composite-alignment blocker — RESOLVED BY ACCEPTED R3A

The accepted runtime `Alignment` is currently one of:
- Line;
- CircularArc;
- SmoothConceptualCurve.

One Road therefore cannot yet own an ordered line/arc/curve sequence.

This is acceptable as an R1 architecture proof, but it is not sufficient for production Road Draw/Edit. Splitting one user road into multiple Road objects would corrupt the intended abstraction:
- one road identity;
- one station domain;
- one cross-section/lane lifecycle;
- one history-level edit target.

**Resolved:** R3A PR #32 was independently accepted and squash-merged as `127aa01556eeb73c1daca160b3e2fa68e211dca8`. One Road now owns an ordered tangent-continuous composite alignment with one cumulative station domain and stable segment ids.

### 2. R2 command inventory is deliberately minimal

R2C proves the mutation boundary but does not yet implement the complete editor-intent catalog such as:
- CreateRoad;
- EditAlignment;
- ApplyRoadConfiguration;
- SetComponentWidth;
- SetWidthProfile;
- AddLaneTransition;
- AddTurnPocket.

R3 must add these through the accepted ProjectSession transaction model rather than allowing React/PixiJS to manufacture canonical Road state independently.

### 3. Browser/JS binding is not yet qualified

The kernel/project crates compile to WASM, but no production TypeScript binding adapter exists.

The current research gate recommends:
- Tauri 2 + React/TypeScript + Vite + PixiJS for the R3 application direction;
- a bounded WASM-versus-native-IPC comparator before locking session placement.

See:
`docs/development/R3_APPLICATION_STACK_RESEARCH.md`.

### 4. Reference image/calibration is planned but not implemented in current Project v1

The architecture documents already reserve `referenceLayers[]`, quality metadata, and calibration, but the accepted R2 Project/domain and schema v1 do not contain them.

Reference-layer persistence/calibration is therefore a later R3 schema/domain packet, not a UI-only task.

## R3 packetization

R3 executes through separately accepted packets. No packet starts the next automatically.

### R3A — Composite Alignment Productionization

Status: **ACCEPTED / COMPLETED**.

Purpose:
make one Road capable of owning a stable ordered multi-segment reference alignment while preserving one continuous station domain.

Touches:
- kernel;
- project persistence/schema migration;
- R1C shared derivation;
- junction regeneration compatibility;
- WASM/native qualification.

This is the first required R3 packet.

### R3B — Desktop Runtime / Binding / 2D Renderer Proof

Status: **NEXT / CONTROL-PLANE GATED**.

Research/contract:
- `docs/development/R3B_RUNTIME_BRIDGE_RESEARCH.md`
- `specs/execution/R3B_DESKTOP_RUNTIME_BINDING_2D_RENDERER.md`

Purpose:
adopt the minimum production application stack and select the Rust-to-UI session bridge by evidence.

Candidate stack:
- Tauri 2;
- React + TypeScript;
- Vite;
- PixiJS 8.

Must compare:
- WASM-owned ProjectSession;
- native ProjectSession via Tauri IPC.

Adopt one; do not keep dual semantic owners.

### R3C — Road Authoring Commands & Generic Configurations

Purpose:
extend the R2C typed transaction boundary with editor-level road intents and deterministic generic road-configuration generators.

Initial scope should cover the commands needed by R3 only.

### R3D — Workspace / Selection / Road Draw & Alignment Edit

Purpose:
deliver the production 2D authoring workspace using the accepted R3A-R3C contracts.

Includes:
- dominant PixiJS viewport;
- Select vs Hand separation;
- scenario visibility/context;
- semantic hierarchy/properties;
- road draw/edit;
- direct manipulation + exact input;
- basic snapping that never creates topology silently.

### R3E — Cross Section / Lane Lifecycle / Turn Pocket

Purpose:
deliver editable cross-section components, width/lifecycle editing, generic configurations, and parameterized turn-pocket workflow through one semantic command path.

### R3F — Local Reference Image / Calibration / Persistence

Purpose:
add user JPG/PNG/site-plan references, known-distance calibration, reference quality metadata, lock/display controls, and save/reopen behavior without transforming existing engineering geometry silently.

This packet is schema/domain-sensitive and must not be treated as presentation-only UI.

### R3G — R3 Integrated UAT / Qualification

Purpose:
run the R3 exit gate across:
- simplified UAT-01 through road/turn-pocket authoring;
- UAT-03 widening/lifecycle;
- UAT-06 calibration;
- UAT-08 relevant undo/redo;
- UAT-11 save/reopen;
- first-time-user UX benchmark;
- Windows desktop smoke.

## Scope controls

R3 does not authorize:
- production junction/access authoring UI beyond read-only/diagnostic compatibility;
- rich 3D editor;
- MapLibre/provider integration;
- markings/assets/standards rules;
- AI runtime;
- final .scd physical package;
- autosave/crash recovery;
- installer/Portable release qualification;
- R4 work.

## Work modes

- R3A: **STRICT** — geometry/public API/schema migration.
- R3B: **STRICT** — new application architecture/dependencies/bridge.
- R3C: **STRICT** — public editor mutation contract.
- R3D: **STANDARD**, escalating to STRICT if semantic/architecture changes emerge; mandatory UX review gate.
- R3E: **STANDARD** after R3C semantics are accepted; mandatory UX + engineering regression gates.
- R3F: **STRICT** — project schema/reference calibration/file handling.
- R3G: acceptance/qualification; independent review required for R3 exit.

## Historical R3A start checklist

Before coding:
- [ ] post-R2/R3 control-plane PR merged;
- [ ] exact current `main` SHA resolved and recorded;
- [ ] R3 umbrella Issue created;
- [ ] R3A Issue created with exact base;
- [ ] one writer owns kernel/project-io/project-session scope;
- [ ] R1/R2 accepted evidence identified for reuse;
- [ ] R3A scrutiny/contract recorded;
- [ ] no UI/Tauri/frontend work included in R3A.

## Stop principle

Stop with `ARCHITECTURE_ESCALATION` rather than hiding a conflict if composite alignment requires:
- separate Road objects to fake one road;
- renderer-owned geometry;
- silently averaged/invented engineering geometry at segment joins;
- breaking accepted R2 migration without a deterministic migration path;
- weakening topology/transaction/persistence invariants.

## Readiness decision

**R3 PLANNING: GO WITH CONDITIONS.**

**R3A: ACCEPTED / COMPLETED.**

**R3B: GO WITH CONDITIONS after its control-plane contract is merged and the exact execution base is recorded in the R3B Issue.**
