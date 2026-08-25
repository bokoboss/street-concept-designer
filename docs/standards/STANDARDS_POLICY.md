# Standards Policy

## Goal

Support engineering guidance without falsely claiming full compliance.

## Source hierarchy

Prefer authoritative primary sources from the relevant road authority. For Thailand, investigate and register DOH/DRR and other competent authorities by topic before encoding values.

## Standard profile record

Each rule/asset-sensitive value should be traceable to:
- jurisdiction;
- authority;
- document title;
- document identifier;
- edition/revision;
- effective/publication date;
- page/table/section when available;
- applicability/context;
- parameter/rule id;
- severity/advisory behavior;
- verification status.

## Project behavior

Projects pin a standards-profile version. Updating the application must not silently change a saved project's engineering interpretation.

## Validation behavior

Prefer advisory status with explanation and source. Only internally impossible geometry should be blocked by default. Allow explicit engineer override where a design is feasible but outside preferred guidance.

## Prohibition

Do not label project assumptions, numeric safeguards, UI defaults, or international references as official Thai requirements.
