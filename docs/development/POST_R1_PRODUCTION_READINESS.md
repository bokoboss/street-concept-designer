# Post-R1 Production Readiness

## Status

READY TO START R2A AFTER THIS CONTROL-PLANE PLAN IS ACCEPTED.

R1 is completed and merged. This document replaces the R1-specific readiness posture for the next production stage; it does not rewrite historical R1 evidence.

## Accepted R1 conclusions

The following are now evidence-backed rather than pre-R1 hypotheses:

- Rust remains the engineering kernel language.
- The accepted kernel is viable on Windows/MSVC and Linux/native, with a WASM compile path.
- No third-party geometry dependency was required through R1.
- Named R1 numerical policy/tolerances passed canonical/adversarial/property-style tests.
- Road/lane geometry is reference-alignment + station based.
- Width/lifecycle breakpoints are semantic and must appear exactly in shared derivation.
- Junction topology is explicit and separate from geometric crossing.
- Authored junction overrides survive derived invalidation/regeneration.
- One owned f64 derived snapshot can feed 2D and 3D.
- Project→render-local subtraction belongs before f32/GPU conversion.
- Renderer identities are structured semantic references, not coordinates/handles.
- Derived render geometry is disposable.

## R2 control-plane conclusions

R2 is packetized:

1. R2A Project / Scenario Domain Core
2. R2B Versioned Persistence & Migration
3. R2C Command / Transaction / Undo / Redo

No packet rolls automatically to the next.

## Package boundary

Preserve the accepted engineering kernel as a low-level package.

Production dependency direction:

```text
kernel
  ↑
project-core
  ↑
project-io
  ↑
future application shell
```

R2A establishes project-core.

R2B adds persistence above project-core and may adopt serde/serde_json after exact dependency review.

R2C adds the typed mutation/history engine to project-core and integrates with project-io.

## Scenario lineage decision

Scenario duplication creates a new ScenarioId while preserving internal semantic object ids.

Reason:
- semantic comparison between Existing and Alternatives;
- stable lineage;
- no need to invent id remapping on every duplicate.

Therefore project-level object identity includes ScenarioId + scenario-local semantic identity.

## Persistence decision

R2B should prove a canonical versioned JSON semantic document.

This is the logical document encoding, not the final physical project package/container.

Do not persist renderer DTOs/caches or private derived junction geometry as truth.

## Undo decision

R2C may start with snapshot-based semantic history.

This is explicitly an implementation mechanism subject to benchmark evidence, not a file-format/product requirement.

Correctness/atomicity outrank premature history optimization.

## Deliberately deferred

Still not authorized in R2:
- production Tauri/React shell;
- PixiJS/Three.js adoption;
- map/reference UI;
- assets;
- markings;
- Thai standards values;
- export;
- AI runtime;
- autosave/crash recovery;
- installer/portable packaging.

## R2A start checklist

Before Codex:
- [ ] R2 control-plane PR merged;
- [ ] current accepted main exact SHA resolved;
- [ ] R2 umbrella Issue created;
- [ ] R2A Issue created;
- [ ] isolated R2A branch created from exact base;
- [ ] Engineering Workflow Integrity green;
- [ ] all R1 regression workflows green;
- [ ] active R2A spec recorded in Issue;
- [ ] no concurrent writer owns kernel/project-core files.

## R2A expected outcome

R2A should return:
- minimal production workspace/package boundary;
- Project/Scenario semantic root;
- stable project/scenario ids;
- explicit LHT/RHT context;
- scenario duplication/isolation;
- project-scoped semantic identity;
- deterministic validation;
- R1 compatibility;
- evidence/CI/benchmark;
- recommendation to proceed/remediate/escalate.

## Stop principle

If R2A requires reworking accepted R1 identity/topology or makes scenario duplication incompatible with stable internal object lineage, stop with a smallest reproducible architecture case instead of hiding the conflict.

## Readiness decision

**R2A: READY AFTER CONTROL-PLANE PLAN MERGE.**

R2B/R2C remain blocked by accepted preceding evidence.
