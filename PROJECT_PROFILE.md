# Project Profile

## Identity
- Project name: Street Concept Designer
- Repository URL: https://github.com/bokoboss/street-concept-designer
- Authoritative local path: not established yet
- Primary branch: `main`
- Package/application version: pre-implementation / unversioned

## Current accepted baseline
- Accepted branch: `main`
- Accepted HEAD SHA: resolve from current `main` when a bounded execution task creates its branch/worktree; record that exact base SHA in the task/PR/evidence package rather than self-referentially pinning it in this file
- Accepted date: 2026-09-01
- Current phase/milestone: R1B accepted; R1C ready for bounded execution
- Last accepted milestone PR / CI: PR #13 (`R1B: qualify junction geometry and topology kernel`) merged as `329281f464456b7bc94896cf5e08f0c58643e7d5`; final-head R1B qualification run `33456860583` PASS and workflow integrity run `33456860580` PASS

## Technology stack
- Languages: Rust is accepted for the R1 engineering kernel; TypeScript remains the candidate application/UI language
- Frameworks: candidate desktop/UI stack is Tauri 2 + React; 2D PixiJS, 3D Three.js, MapLibre under evaluation
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
- Source: `src/` (accepted R1A alignment/lifecycle kernel + R1B junction/topology kernel)
- Tests: `tests/` (R1A/R1B regression/integration coverage)
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
- No production desktop/editor shell exists yet; accepted R1 kernel source and qualification CI now exist.
- Rust is accepted for the current R1 kernel path; no third-party geometry library has been required through R1B.
- Current UI/rendering stack is candidate architecture only.
- Thai standards sources are catalogued but not yet extracted/verified to page-level numeric rule profiles.
- R1A/R1B have proven alignment, stationing, variable-width components, topology, determinism, and precision policy; R1C must now prove shared 2D/3D derivation before production editor work.
- Product-file physical container, persistence mechanism, and final renderer DTOs remain intentionally unimplemented; policy boundaries are documented first.
- Windows portable/offline packaging is a product requirement but remains release-engineering evidence-gated; do not assume a bare Tauri executable is a qualified portable release.

## Current next objective
- Execute R1C only: shared renderer-facing derivation proof under `specs/execution/R1C_SHARED_RENDER_PROOF.md`. At task start, branch from current accepted `main` and record the exact base SHA in execution evidence. Production UI/editor work remains separately gated and must not start automatically.
