# Project Profile

## Identity
- Project name: Street Concept Designer
- Repository URL: https://github.com/bokoboss/street-concept-designer
- Authoritative local path: `D:\\R&D\\street-concept-designer`
- Primary branch: `main`
- Package/application version: pre-implementation / unversioned

## Current accepted baseline
- Accepted branch: `main`
- Accepted HEAD SHA: resolve from current `main` when a bounded execution task creates its branch/worktree; record that exact base SHA in the task/PR/evidence package rather than self-referentially pinning it in this file
- Accepted date: 2026-09-04
- Current phase/milestone: R2 Production Project Core and R3A Composite Alignment Productionization are accepted/completed; R3B Desktop Runtime / Binding / 2D Renderer Proof control-plane planning is accepted, while R3B implementation remains separately gated and has not started
- Last accepted implementation milestone: R3A PR #32 independently re-reviewed `PASS` / scrutinized `GO` at accepted PR head `943d068e9351d44920f8048da21af22b2aabc459`, squash-merged as `127aa01556eeb73c1daca160b3e2fa68e211dca8`.
- Last accepted control-plane milestone: R3B planning PR #35 (`Plan R3B desktop runtime, bridge comparator, and 2D proof`) scrutinized `GO WITH CONDITIONS`, all final-head conditions passed at `f0f9aebd3e9bc0c26386ad801abdb397e04d0975`, and squash-merged as `87894add3d73f3debd59080922a1c492736764ed`; final-head Workflow Integrity `33832064165`, R1A `33832064151`, R1B `33832064179`, R1C `33832064181`, R2A `33832064171`, R2B `33832064170`, R2C `33832064162`, and R3A `33832064164` all PASS

## Technology stack
- Languages: Rust is accepted for the production engineering kernel; TypeScript remains the candidate future application/UI language
- Frameworks: accepted R3B control-plane direction is Tauri 2 + React/TypeScript + Vite + PixiJS 8 (WebGL for the R3B proof). Exact dependency pins and the authoritative Rust-to-UI `ProjectSession` owner remain evidence-gated until the R3B comparator is accepted. Three.js, MapLibre, WebGPU, and Web Worker architecture are outside R3B
- Package manager: Cargo established for the Rust engineering core; R3B control-plane selects project-local npm + checked-in `package-lock.json` for the frontend/app proof, subject to implementation qualification
- Supported OS/runtime: Windows-first desktop target; Windows x64 first. Product requires per-user installer plus no-install Portable distribution; final OS/runtime compatibility remains qualification-gated

## Standard commands
### Install/bootstrap
```text
Not established before R1 implementation.
```
### Fast validation
```text
cargo check --locked
cargo test --locked
```
### Full validation
```text
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
cargo build --locked --release
cargo build --locked --release --target wasm32-unknown-unknown
Engineering workflow integrity: python <workflow-checkout>/scripts/setup_project.py validate .
```
### Build/package
```text
Not established before R1 implementation.
```
### Local run
```text
Not established before R1 implementation.
```

## Architecture / invariants
- Canonical semantic engineering state is authoritative; renderer objects are derived only.
- 2D plan, cross section, and 3D derive from the same semantic model.
- Road geometry is reference-alignment/station based.
- Lane add/drop/taper/turn pockets use general station-based lane/component lifecycles.
- Geometry and topology are separate; crossing does not silently create a junction.
- Engineering state and presentation state are separate.
- Standards-sensitive behavior must have versioned provenance.
- AI actions must translate to typed previewable/undoable commands.
- Manual UI, AI, imports, and automation converge on one validated semantic transaction layer.
- Basemap/reference sources are separate from canonical engineering geometry and provider capabilities/terms govern tracing/cache/export.
- Equal semantic input must produce deterministic equivalent output.

## Protected behavior
Changes must not alter the following unless explicitly approved:
- clean-slate prohibition against inheriting legacy `road-concept-builder` source/schema/tests/architecture;
- semantic-model-first architecture;
- single canonical model for 2D/3D;
- explicit topology creation;
- standards provenance and non-fabrication policy;
- deterministic command/undo model requirement;
- renderer/provider separation from project engineering truth.

## Important paths
- Source: `src/` (accepted R1 engineering kernel/shared derivation) + `crates/project-core/` (accepted R2A Project/Scenario domain package) + `crates/project-io/` (accepted R2B persistence/migration) + `crates/project-session/` (accepted R2C command/history boundary)
- Tests: `tests/` (accepted R1A/R1B/R1C coverage) + `crates/project-core/tests/` (R2A) + `crates/project-io/tests/` (R2B) + `crates/project-session/tests/` (R2C)
- Documentation: `docs/` including `docs/development/R3B_RUNTIME_BRIDGE_RESEARCH.md`
- Specifications: `specs/`
- Execution contracts: `specs/execution/` including `R3B_DESKTOP_RUNTIME_BINDING_2D_RENDERER.md`
- Development workflow templates: `docs/development/templates/`
- Generated output: not established
- Local-only / sensitive / licensed data: not established; must never be committed without explicit policy

## Validation matrix
| Gate | Command / Method | Required |
|---|---|---|
| Engineering workflow integrity | upstream `setup_project.py validate .` + GitHub Actions | Yes |
| Unit / targeted | to be established by R1 | Yes for implementation |
| Integration / regression | to be established | Yes when applicable |
| Browser/UI | later product-shell stages | When applicable |
| Build/package/runtime | after toolchain scaffold | Yes |
| Real-data/reference | golden engineering workflows/fixtures | Yes where applicable |
| CI | workflow-integrity gate established; implementation CI to be added by R1 | Yes |

## Execution characteristics
- Typical task ambiguity: high during product/kernel foundation; reduce in ChatGPT before Codex execution
- High-risk areas: geometry robustness, topology, coordinate precision/tolerance, project schema, standards claims, command/undo semantics, 2D/3D divergence, basemap/provider rights, asset provenance
- Modules safe to parallelize: independent research/read-only review; later isolated asset/reference work
- Modules tightly coupled / single-owner: canonical semantic model, geometry kernel, command transaction model
- Preferred local execution constraints: isolated branch/worktree; one writer per tightly coupled task; fresh-context review for critical changes

## Git / release policy
- Branch naming: task-scoped branch such as `codex/r1-*` or `chatgpt/*`
- Commit policy: small coherent commits; no unrelated cleanup
- PR policy: material changes through PR with explicit success gates/evidence
- Merge policy: merge only after required gates and review pass; prefer squash when history is exploratory
- Release policy: no release until qualified product milestone; Windows release must qualify per-user/no-admin installer, Portable mode, offline deployment strategy, trusted code signing, and office-PC UAT
- Baseline pinning policy: execution tasks must record the exact `main` base SHA at branch/worktree creation in the task/PR/evidence package; do not attempt to make an in-repository profile file self-reference its own HEAD SHA

## Current known limitations / risks
- No production desktop/editor shell exists yet. R3B has an accepted stack/bridge research direction and execution contract, but Tauri/React/Vite/PixiJS dependency pins and the native-vs-WASM `ProjectSession` owner remain unadopted until R3B evidence is accepted.
- Rust is accepted for the current R1 kernel path; no third-party geometry library has been required through R1B.
- Current UI/rendering stack is candidate architecture only.
- R2B persistence reconstructs canonical projects with the accepted default `TolerancePolicy`; non-default numerical policy is not persisted project state and must not become user-facing without a separate versioning decision.
- Thai standards sources are catalogued but not yet extracted/verified to page-level numeric rule profiles.
- R1A/R1B/R1C established the geometry/topology/shared-render proof, and accepted R3A now productionizes one Road-owned ordered composite alignment with stable segment ids, cumulative stationing, tangent-continuity validation, road-global CrossSection/lane lifecycle, deterministic whole-alignment queries, and preserved shared 2D/3D derivation.
- R2B established the strict persistence boundary; accepted R3A advances the canonical writer to schema v2 for composite alignment. Historical v1 single-primitive documents migrate deterministically to one `segment-0`, historical v0 remains the single v1-shaped/`"metres"` contract, and v2-shaped documents mislabeled as v0 are rejected. The final physical `.scd` package/container remains intentionally unselected.
- Windows portable/offline packaging is a product requirement but remains release-engineering evidence-gated; do not assume a bare Tauri executable is a qualified portable release.

## Current next objective
- Execute only the separately pinned R3B packet under `specs/execution/R3B_DESKTOP_RUNTIME_BINDING_2D_RENDERER.md`: compare WASM-owned versus native-Tauri-IPC-owned `ProjectSession` using the same semantic fixture/operations, measure bridge latency plus UI-thread/rAF health, select exactly one owner, qualify the minimal Tauri/React/Vite/PixiJS WebGL desktop proof and dependency/license graph, and leave R3C unstarted until R3B independent acceptance.
