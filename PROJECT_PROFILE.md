# Project Profile

## Identity
- Project name: Street Concept Designer
- Repository URL: https://github.com/bokoboss/street-concept-designer
- Authoritative local path: not established yet
- Primary branch: `main`
- Package/application version: pre-implementation / unversioned

## Current accepted baseline
- Accepted branch: `main`
- Accepted HEAD SHA: `62321562fa0dddc0981d8776f65324a6356a1ab5`
- Accepted date: 2026-08-25
- Current phase/milestone: R0 clean foundation
- Last accepted PR / CI run: none; repository initialized directly

## Technology stack
- Languages: not locked; TypeScript and Rust are candidate implementation languages
- Frameworks: candidate desktop/UI stack is Tauri 2 + React; 2D PixiJS, 3D Three.js, MapLibre under evaluation
- Package manager: not established
- Supported OS/runtime: Windows-first desktop target; exact runtime/toolchain not established

## Standard commands
### Install/bootstrap
```text
Not established in R0.
```
### Fast validation
```text
Not established in R0.
```
### Full validation
```text
Not established in R0.
```
### Build/package
```text
Not established in R0.
```
### Local run
```text
Not established in R0.
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
- Equal semantic input must produce deterministic equivalent output.

## Protected behavior
Changes must not alter the following unless explicitly approved:
- clean-slate prohibition against inheriting legacy `road-concept-builder` source/schema/tests/architecture;
- semantic-model-first architecture;
- single canonical model for 2D/3D;
- explicit topology creation;
- standards provenance and non-fabrication policy;
- deterministic command/undo model requirement.

## Important paths
- Source: not created yet
- Tests: not created yet
- Documentation: `docs/`
- Specifications: `specs/`
- Development workflow templates: `docs/development/templates/`
- Generated output: not established
- Local-only / sensitive / licensed data: not established; must never be committed without explicit policy

## Validation matrix
| Gate | Command / Method | Required |
|---|---|---|
| Unit / targeted | to be established by R1 | Yes for implementation |
| Integration / regression | to be established | Yes when applicable |
| Browser/UI | later product-shell stages | When applicable |
| Build/package/runtime | after toolchain scaffold | Yes |
| Real-data/reference | golden engineering workflows/fixtures | Yes where applicable |
| CI | GitHub Actions to be established | Yes after scaffold |

## Execution characteristics
- Typical task ambiguity: high during product/kernel foundation; should be reduced in ChatGPT before Codex execution
- High-risk areas: geometry robustness, topology, coordinate precision, project schema, standards claims, undo/redo semantics, 2D/3D divergence
- Modules safe to parallelize: independent research/review; later isolated asset/reference work
- Modules tightly coupled / single-owner: canonical semantic model, geometry kernel, command transaction model
- Preferred local execution constraints: isolated branch/worktree; one writer per tightly coupled task; fresh-context review for critical changes

## Git / release policy
- Branch naming: task-scoped branch such as `codex/r1-*` or `chatgpt/r0-*`
- Commit policy: small coherent commits; no unrelated cleanup
- PR policy: material changes through PR with explicit success gates/evidence
- Merge policy: merge only after required gates and review pass; prefer squash when history is exploratory
- Release policy: not established; no release until qualified product milestone

## Current known limitations / risks
- No production code or executable toolchain exists yet.
- Kernel language/geometry libraries are not locked.
- Current UI/rendering stack is candidate architecture only.
- Thai standards source register is not yet complete enough to encode engineering values.
- R1 must prove alignment, stationing, variable-width components, topology, determinism, and shared 2D/3D derivation before production editor work.

## Current next objective
- Complete and review R0 foundation, then execute a bounded R1 geometry/semantic kernel spike with objective qualification evidence.
