# Engineering Constitution

These rules override implementation convenience.

## 1. Semantic model is authoritative

Roads, lanes, junctions, markings, assets, scenarios, and commands are semantic engineering objects. SVG paths, Canvas pixels, WebGL meshes, GLB files, screenshots, and DOM nodes are derived representations only.

## 2. One model, multiple views

2D plan, cross section, and 3D must derive from the same canonical project state. Never create separate 2D and 3D engineering calculations.

## 3. Alignment and stationing are foundational

A road is not an arbitrary polygon. It has a reference alignment and station-based lateral composition. Longitudinal transitions must be represented explicitly.

## 4. Geometry and topology are distinct

Geometric crossing does not automatically mean network connection. Junction creation must be explicit and topology must be inspectable.

## 5. Automation first, manual override always

Standard road features should be generated from intent and parameters, not drafted line-by-line. Manual overrides must remain possible without destroying the semantic model.

## 6. Engineering truth is separate from presentation

Decorative assets, materials, lighting, labels, and presentation effects cannot change engineering geometry or validation results.

## 7. Standards are advisory and versioned

Never hard-code an unverified value and label it as an official standard. Each standard-sensitive rule must identify authority, document, edition/version, applicability, and provenance. Projects pin standard profiles explicitly.

## 8. AI cannot bypass commands

Natural-language AI may propose operations, but it must translate them into typed deterministic commands with validation, preview, apply/cancel, undo, and auditability. AI must never mutate mesh, renderer state, or project JSON directly.

## 9. Determinism is mandatory

Equal semantic input must produce equivalent semantic and geometric output. Randomness must be isolated to presentation features and seeded when persistence matters.

## 10. Technical debt is not protected

No prototype abstraction is retained merely because it exists. Prefer a clean replacement when migration would distort the correct architecture.

## 11. Qualification over completion claims

A stage is complete only when its acceptance evidence exists: tests, fixtures, invariants, benchmarks, review results, and human engineering acceptance where required.
