# AGENTS.md

Repository-wide instructions for Codex and other coding agents.

## Mission

Build Street Concept Designer as a clean-slate map-first 2D/3D engineering concept design environment for street, access, and intersection work.

## Development workflow

This project adopts `bokoboss/engineering-development-workflow` v1.4.1. Read `docs/development/ENGINEERING_WORKFLOW.md` and follow the upstream normative workflow/skills for bounded execution, model routing, scrutiny, debugging, evidence, and acceptance.

Before coding-agent work, use `PROJECT_PROFILE.md` to establish the verified project baseline. Material implementation tasks should have an explicit execution contract and success gates.

## Mandatory context

Before implementation, read:

- `PROJECT_PROFILE.md`
- `ENGINEERING_CONSTITUTION.md`
- `docs/development/ENGINEERING_WORKFLOW.md`
- `docs/product/PRODUCT_VISION.md`
- `docs/product/GOLDEN_WORKFLOWS.md`
- `docs/ux/UX_ARCHITECTURE.md`
- `docs/architecture/SEMANTIC_MODEL.md`
- `docs/architecture/TECHNICAL_ARCHITECTURE.md`
- the current file under `specs/`

Read the relevant specialized policy before touching these surfaces:
- editor/undo/AI/import mutations: `docs/architecture/COMMAND_TRANSACTION_MODEL.md`;
- maps/geospatial/reference layers: `docs/architecture/MAP_BASEMAP_POLICY.md`;
- assets/markings/3D props: `docs/assets/ASSET_SYSTEM.md`, `ASSET_PRODUCTION_PIPELINE.md`, and `ASSET_VISUAL_STYLE.md`;
- standards-sensitive behavior: `docs/standards/STANDARDS_POLICY.md` and the applicable source register.

## Clean-slate prohibition

Do not copy source code, schemas, tests, or architecture from `road-concept-builder`. The legacy repository is not a dependency and not an implementation template. If legacy behavior is referenced, re-derive the requirement from product/engineering intent.

## Work discipline

For non-trivial changes:

1. inspect the current repository and Git/GitHub state;
2. confirm the authoritative baseline from `PROJECT_PROFILE.md`;
3. restate the bounded goal and non-goals;
4. produce a repository-grounded implementation plan;
5. scrutinize the plan for unnecessary complexity and architectural drift;
6. implement one coherent change;
7. run every required success gate;
8. review the actual diff and evidence against intent/acceptance criteria;
9. fix findings;
10. report evidence, limitations, remaining risks, and exact commit/PR/CI identifiers.

## Model routing

Use the shared workflow model-routing policy. Prefer the cheapest model that can reliably finish the already-bounded task. Diagnose failures and increase reasoning effort before escalating model tier when appropriate.

## Guardrails

- Do not make renderer objects source-of-truth.
- Do not encode turn pockets as arbitrary paint polygons.
- Do not silently connect intersecting roads.
- Do not create separate 2D and 3D semantic calculations.
- Do not hard-code unverified Thai standards.
- Do not start polished UI or a large asset library before the semantic/geometry kernel is qualified.
- Do not add cloud, auth, collaboration, simulation, BIM, grading, or detailed CAD features unless an explicit later specification authorizes them.

## Tests and evidence

Geometry work must include appropriate examples/golden fixtures, invariants, deterministic tests, and property/fuzz/benchmark coverage when the stage calls for it. A completion claim requires objective evidence; visually plausible output alone is insufficient.

## Human authority

The user is the domain authority for traffic-engineering judgment and final acceptance. Agents own implementation evidence, not final engineering approval.
