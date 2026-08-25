# R0 — Clean Foundation

## Objective

Establish the authoritative product, engineering, UX, asset, standards, and AI-development operating baseline before production code begins.

## Scope

- clean-slate repository policy;
- engineering constitution and agent instructions;
- verified project profile;
- Engineering Development Workflow v1.4.1 adoption;
- product vision and golden workflows;
- UX architecture and tool taxonomy;
- semantic-model baseline;
- technical architecture boundaries/candidates;
- asset-system architecture and starter catalog;
- standards governance policy;
- AI development operating model;
- bounded R1 geometry/semantic spike specification.

## Out of scope

- production application scaffold;
- geometry implementation;
- map integration;
- 2D/3D renderer implementation;
- asset artwork/models;
- Thai-standard numeric rule encoding;
- AI/LLM runtime integration.

## Protected decisions

- Do not inherit legacy `road-concept-builder` source/schema/tests/architecture.
- Canonical semantic state is source of truth.
- 2D/3D derive from the same model.
- Geometry and topology are separate.
- Standards claims require provenance.
- AI authoring must use typed deterministic commands.

## Success gates

| Gate | Criterion | Evidence |
|---|---|---|
| R0-G1 | Authoritative context is internally consistent and has no legacy architecture dependency | docs + scrutiny review |
| R0-G2 | Shared Engineering Development Workflow installation is complete and versioned | `.engineering-workflow.json` + managed files |
| R0-G3 | PROJECT_PROFILE contains only verified facts/candidate labels | review |
| R0-G4 | Golden workflows cover the product's core engineering value | `docs/product/GOLDEN_WORKFLOWS.md` |
| R0-G5 | UX/tool/asset architecture supports users who do not manually create 2D/3D assets | UX + asset docs |
| R0-G6 | R1 is executable as a bounded spike with explicit non-goals and qualification criteria | `specs/R1_GEOMETRY_SPIKE.md` |
| R0-G7 | No production-code completion claim is made | PR scope/diff |

## Required review

Use upstream Engineering Development Workflow scrutiny principles. Specifically challenge:
- whether the new product should exist in this form;
- whether any subsystem is being prematurely locked;
- whether R1 tests the highest-risk assumptions first;
- whether any document accidentally recreates legacy technical debt;
- whether the scope can be reduced without losing architectural evidence.

## Definition of done

R0 is done when the foundation PR passes scrutiny, any contradictions are resolved, the workflow installation is valid, and R1 can be handed to Codex without relying on unstated architectural assumptions.
