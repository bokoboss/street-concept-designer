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
- Accepted date: 2026-09-03
- Current phase/milestone: R2 Production Project Core is accepted/completed; R3 2D Road Authoring Alpha is control-plane planned, with R3A Composite Alignment as the first separately gated implementation packet
- Last accepted milestone PR / CI: PR #27 (`R2C: qualify typed transactions and undo/redo`) squash-merged as `fb933bb9c5da49225c3d6b7e52176e270c117cce`; final-head R2C `33595026496` PASS, R2B `33595026419` PASS, R2A `33595026470` PASS, R1A `33595026424` PASS, R1B `33595026436` PASS, R1C `33595026445` PASS, and workflow integrity `33595026428` PASS

## Technology stack
- Languages: Rust is accepted for the production engineering kernel; TypeScript remains the candidate future application/UI language
- Frameworks: R3 research supports Tauri 2 + React/TypeScript + Vite + PixiJS as the bounded desktop/2D adoption path; exact versions and the Rust-to-UI bridge remain evidence-gated for R3B. Three.js and MapLibre are intentionally not part of the first R3 runtime packet
- Package manager: Cargo established for the Rust kernel; frontend package manager not yet established
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
- Documentation: `docs/`
- Specifications: `specs/`
- Execution contracts: `specs/execution/`
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
- No production desktop/editor shell exists yet; R3 application stack remains unadopted until the R3B bridge/runtime proof.
- Rust is accepted for the current R1 kernel path; no third-party geometry library has been required through R1B.
- Current UI/rendering stack is candidate architecture only.
- R2B persistence reconstructs canonical projects with the accepted default `TolerancePolicy`; non-default numerical policy is not persisted project state and must not become user-facing without a separate versioning decision.
- Thai standards sources are catalogued but not yet extracted/verified to page-level numeric rule profiles.
- R1A/R1B/R1C have proven primitive alignment/stationing, exact variable-width lifecycle breakpoints, topology, deterministic shared 2D/3D derivation, scoped semantic selection identity, and local-render-origin precision. A Road still owns only one line/arc/smooth alignment primitive; R3A must productionize an ordered composite alignment before Road Draw UI.
- R2B has proven a strict versioned canonical JSON semantic document and migration boundary in `project-io`; R3A is expected to introduce the next schema version for composite alignment, while the final physical `.scd` package/container remains intentionally unselected.
- Windows portable/offline packaging is a product requirement but remains release-engineering evidence-gated; do not assume a bare Tauri executable is a qualified portable release.

## Current next objective
- Merge the post-R2/R3 control-plane baseline, create the R3 umbrella + R3A Issues from the resulting exact `main` SHA, then execute only `specs/execution/R3A_COMPOSITE_ALIGNMENT.md`. R3B+ must not start automatically.
