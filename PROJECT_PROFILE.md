# Project Profile

## Identity
- Project name: Street Concept Designer
- Repository URL: https://github.com/bokoboss/street-concept-designer
- Authoritative local path: not established yet
- Primary branch: `main`
- Package/application version: pre-implementation / unversioned

## Current accepted baseline
- Accepted branch: `main`
- Accepted HEAD SHA: `949da9145fa4da714ff9fc1357dfedb23b174284`
- Accepted date: 2026-08-25
- Current phase/milestone: pre-Codex R1 architecture/execution preparation
- Last accepted PR / CI run: PR #5 (`Add product, UX, map, standards, and asset research baseline`); Engineering Workflow Integrity run `32825313913` PASS

## Technology stack
- Languages: not locked; TypeScript and Rust are candidate implementation languages
- Frameworks: candidate desktop/UI stack is Tauri 2 + React; 2D PixiJS, 3D Three.js, MapLibre under evaluation
- Package manager: not established
- Supported OS/runtime: Windows-first desktop target; exact runtime/toolchain not established

## Standard commands
### Install/bootstrap
```text
Not established before R1 implementation.
```
### Fast validation
```text
Not established before R1 implementation.
```
### Full validation
```text
Engineering workflow integrity: python <workflow-checkout>/scripts/setup_project.py validate .
R1-specific commands will be established by the chosen kernel implementation.
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
- Source: not created yet
- Tests: not created yet
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
- Release policy: not established; no release until qualified product milestone

## Current known limitations / risks
- No production code or executable application toolchain exists yet.
- Kernel language/geometry libraries are not locked.
- Current UI/rendering stack is candidate architecture only.
- Thai standards sources are catalogued but not yet extracted/verified to page-level numeric rule profiles.
- R1 must prove alignment, stationing, variable-width components, topology, determinism, precision policy, and shared 2D/3D derivation before production editor work.
- Product-file physical container, persistence mechanism, and final renderer DTOs remain intentionally unimplemented; policy boundaries are documented first.

## Current next objective
- Finish pre-Codex architecture/execution preparation, then execute R1A only: kernel feasibility + alignment/stationing + cross-section/lane lifecycle. R1B junction/topology and R1C shared-render proof remain separately gated and must not start automatically.
