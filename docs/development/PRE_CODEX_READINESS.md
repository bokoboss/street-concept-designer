# Pre-Codex Readiness Gate

Status target: R1A execution readiness after this architecture pack is reviewed/merged.

## Why this gate exists

Codex should receive a bounded implementation problem, not responsibility for inventing product direction, UX philosophy, domain semantics, numerical policy, or project governance during coding.

## Decisions already established by ChatGPT/control-plane work

### Product
- clean-slate Generation 2 successor;
- map/reference-first rapid street/intersection concept designer;
- Traffic Engineer / Transport Planner primary workflow;
- 2D plan primary authoring + synchronized live 3D;
- concept design, not detailed Civil CAD/simulation/BIM.

### Semantic/domain
- reference alignment + stationing;
- ordered semantic cross-section components;
- station-varying width/lifecycle;
- turn pockets through general lane lifecycle;
- junction as first-class semantic object;
- geometry crossing separate from explicit topology;
- explicit lane-to-lane connectivity;
- stable semantic ids;
- engineering and presentation state separated.

### Architecture boundaries
- semantic/kernel state is authoritative;
- renderer outputs/caches are derived/disposable;
- one shared derivation path for 2D/3D;
- typed semantic command/transaction layer for manual UI/AI/import/automation;
- f64/metre canonical geometry with explicit tolerance policy;
- local render origin for large coordinates;
- project schema/versioning policy separated from renderer memory layout;
- map renderer separated from basemap provider/licensing capabilities;
- standards are versioned sourced profiles, not hard-coded geometry constants;
- assets are semantic/sourced definitions with procedural/2D/3D representations.

### UX
- plan view is primary;
- direct manipulation + exact numeric entry;
- contextual actions over CAD-sized modal toolset;
- explicit Select vs Hand/navigation behavior;
- synchronized semantic selection across views;
- snapping is not topology;
- candidate junction requires explicit connection decision;
- presets/configurations generate editable semantics;
- user is not required to create 2D/3D assets manually.

### Development governance
- Engineering Development Workflow v1.4.1 installed and CI-validated;
- one writer per tightly coupled kernel surface;
- bounded execution contracts;
- machine-verifiable gates/evidence;
- scrutiny before architecture acceptance;
- no legacy `road-concept-builder` source/schema/tests/architecture reuse.

## R1 packetization

### R1A — Alignment & lane lifecycle
May execute first.

### R1B — Junction geometry/topology
Blocked until R1A accepted.

### R1C — Shared 2D/3D derivation proof
Blocked until R1A + R1B accepted.

No worker may automatically roll from one packet into the next.

## Intentionally unresolved — Codex/R1 evidence must answer

These are not missing requirements; they are experimental architecture questions:
- Rust native+WASM kernel feasibility;
- exact geometry dependencies/libraries;
- whether Rust remains preferable to TypeScript after evidence;
- named numerical tolerance defaults;
- performance benchmark baselines;
- exact module/crate/package layout;
- low-level undo implementation mechanism;
- exact physical project container format;
- production renderer DTO structure beyond policy constraints.

Codex may investigate these only within the active execution contract and must report evidence rather than silently locking a choice.

## Explicitly not delegated to R1A

R1A must not decide/implement:
- product UI shell;
- map provider;
- persistence/export;
- assets/sign/marking library;
- Thai standards numeric values;
- AI/LLM integration;
- scenarios;
- junction/topology;
- production 2D/3D renderers;
- terrain/vertical alignment;
- roundabout/U-turn special module;
- simulation/swept path.

## R1A start checklist

Before invoking Codex:
- [ ] architecture-pack PR merged to `main`;
- [ ] `PROJECT_PROFILE.md` accepted SHA updated to merged baseline;
- [ ] Engineering Workflow Integrity CI passing;
- [ ] Issue #2 remains open and correctly scoped;
- [ ] `R1A_ALIGNMENT_LANE_LIFECYCLE.md` references current accepted main SHA;
- [ ] dedicated branch/worktree will be used;
- [ ] writer model/effort chosen through shared routing policy;
- [ ] completion evidence template available;
- [ ] no parallel writer owns the same kernel files.

## R1A expected deliverable

R1A is not "an app". It should return:
- minimal kernel/tooling scaffold;
- native+WASM feasibility evidence;
- alignment/stationing APIs;
- ordered cross-section/width profiles;
- lane add/drop/taper/right-turn-pocket lifecycle proof;
- canonical/adversarial fixtures;
- deterministic/invariant/property/fuzz-style tests as applicable;
- benchmark observations;
- documented tolerance/dependency assumptions;
- PR/CI/Evidence Package;
- recommendation to proceed/remediate/escalate.

## Stop/escalation principles

Codex stops rather than broadens scope when:
- accepted product/semantic architecture appears contradictory;
- Rust/WASM feasibility materially fails;
- lane lifecycle cannot represent required cases without special-case geometry;
- deterministic behavior requires architecture change;
- numerical failures are only being hidden by larger tolerances;
- a required solution pulls in junction/UI/map/persistence work;
- dependency licensing creates material product risk.

## Readiness decision

When this architecture pack is merged and the checklist above is current, the control-plane assessment is:

**R1A: READY FOR BOUNDED CODEX EXECUTION**

R1B/R1C remain blocked by preceding evidence gates.
