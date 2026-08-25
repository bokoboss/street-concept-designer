# AGENTS.md

Repository-wide instructions for Codex and other coding agents.

## Mission

Build Street Concept Designer as a clean-slate map-first 2D/3D engineering concept design environment for street, access, and intersection work.

## Mandatory context

Before implementation, read:

- `ENGINEERING_CONSTITUTION.md`
- `docs/product/PRODUCT_VISION.md`
- `docs/product/GOLDEN_WORKFLOWS.md`
- `docs/ux/UX_ARCHITECTURE.md`
- `docs/architecture/SEMANTIC_MODEL.md`
- `docs/architecture/TECHNICAL_ARCHITECTURE.md`
- the current file under `specs/`

Read asset/standards documents when those surfaces are involved.

## Clean-slate prohibition

Do not copy source code, schemas, tests, or architecture from `road-concept-builder`. The legacy repository is not a dependency and not an implementation template. If legacy behavior is referenced, re-derive the requirement from product/engineering intent.

## Work discipline

For non-trivial changes:

1. inspect the current repository;
2. restate the bounded goal and non-goals;
3. produce a repository-grounded implementation plan;
4. scrutinize the plan for unnecessary complexity and architectural drift;
5. implement one coherent change;
6. run deterministic validation;
7. review the diff against intent and acceptance criteria;
8. fix findings;
9. report evidence, limitations, and remaining risks.

## Guardrails

- Do not make renderer objects source-of-truth.
- Do not encode turn pockets as arbitrary paint polygons.
- Do not silently connect intersecting roads.
- Do not create separate 2D and 3D semantic calculations.
- Do not hard-code unverified Thai standards.
- Do not start polished UI or a large asset library before the semantic/geometry kernel is qualified.
- Do not add cloud, auth, collaboration, simulation, BIM, grading, or detailed CAD features unless an explicit later specification authorizes them.

## Tests

Geometry work must include appropriate examples/golden fixtures, invariants, deterministic tests, and property/fuzz/benchmark coverage when the stage calls for it.

## Human authority

The user is the domain authority for traffic-engineering judgment and final acceptance. Agents own implementation evidence, not final engineering approval.
