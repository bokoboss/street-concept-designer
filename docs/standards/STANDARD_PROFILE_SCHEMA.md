# Standards Profile Schema Baseline

## Purpose

Define how engineering guidance/provenance enters the product without hard-coding jurisdiction-specific numbers into geometry code or silently changing old projects.

## Core separation

- **Geometry kernel** answers whether semantic geometry is internally coherent.
- **Standards/rules layer** evaluates coherent geometry against a selected guidance/profile.
- **Asset generator** may use verified profile parameters for standards-sensitive signs/markings/devices.
- **Project** pins profile versions explicitly.

## Profile identity

A standards profile should conceptually declare:

```text
profileId
profileVersion
authority
jurisdiction
name
documentSet[]
effectiveDate / publicationDate
status
applicability
rules[]
assetParameters[]
provenance
```

Examples of authority/context:
- Thailand / DOH;
- Thailand / DRR;
- Project-specific/custom;
- future external profiles.

Do not encode one ambiguous universal `Thailand` profile.

## Source document record

Each normative/supporting source should store:
- source id;
- authority/issuer;
- exact title;
- edition/revision;
- publication/effective date if known;
- source URL or controlled local-reference identifier;
- page/table/figure/drawing number for extracted parameters;
- applicability notes;
- supersession/status;
- verification date.

A URL alone is insufficient provenance for a rule.

## Rule record

Conceptual fields:

```text
ruleId
ruleVersion
subject
parameter/path
condition/applicability
comparison/logic
preferred/min/max/range/value where applicable
unit
severity
message
sourceRefs[]
overridePolicy
verificationStatus
```

The exact execution representation may be typed code, data-driven rules, or a hybrid. Do not implement a generic rules DSL before real rules prove that it is needed.

## Rule classifications

Distinguish at least:
- internal invariant — impossible/invalid semantic geometry, not a standards rule;
- mandatory requirement — only when source/applicability clearly supports that classification;
- advisory/preferred guidance;
- informational/context;
- project assumption/custom target.

UI severity must not exaggerate advisory guidance into statutory non-compliance.

## Verification status

Suggested statuses:
- `VERIFIED_PRIMARY` — exact primary authority source/page/applicability checked;
- `VERIFIED_SUPPORTING` — reliable supporting source but not final normative authority;
- `UNVERIFIED` — candidate/default not safe for authoritative guidance;
- `SUPERSEDED`;
- `PROJECT_CUSTOM`.

Only rules meeting the product's required verification threshold may present themselves as authority-backed engineering guidance.

## Profile pinning

A project stores `profileId + profileVersion` (and optional source-set hash/version). Opening the project with a newer application must not silently replace it with a newer standards profile.

Update flow:
1. detect newer profile;
2. show change/release notes where available;
3. user chooses whether to migrate;
4. re-run validation;
5. preserve prior result/evidence where project audit requires it.

## Override model

Concept work often requires exceptions. An advisory/overridable issue may store:
- target object;
- rule id/version;
- evaluated value;
- override status;
- optional engineer rationale/note;
- date/actor where audit features exist.

An override does not modify the underlying source rule and must be re-evaluated if relevant geometry/profile changes.

## Applicability

Rules must declare context rather than assuming all road types are equivalent. Potential dimensions include:
- authority/network owner;
- road class/function;
- urban/rural context;
- permanent vs temporary/work-zone;
- facility/component type;
- design/operating condition where explicitly supported;
- traffic side only when relevant.

If applicability cannot be resolved confidently, show the rule as not-evaluated/needs-context rather than guessing.

## Numeric units

Source values retain declared source units/provenance but rules evaluate against explicit physical units. Convert deterministically. UI may display alternate units without changing canonical rule semantics.

## Standards-sensitive assets

Marking/sign/signal/lighting asset parameters may reference the same profile/source system.

Example:
```text
asset definition
  semantic type: roadmark.crosswalk.zebra
  parameter set: DOH/<profile-version>/<parameter-id>
  source refs: document/page/figure
```

Generic presentation proxies remain clearly generic and must not masquerade as verified authority-specific assets.

## Source updates / web volatility

External URLs/terms may move. Store bibliographic identity and page/figure references, not only live URL. Where licensing permits and governance requires, controlled source metadata/checksums may be maintained without redistributing restricted documents.

## Testing

Rules/profile tests should cover:
- boundary values;
- condition/applicability branching;
- unit conversions;
- verified source metadata completeness;
- project profile pinning;
- migration does not happen silently;
- override invalidation/re-evaluation;
- deterministic messages/results for equal state/profile;
- no cross-profile leakage between DOH/DRR/custom.

## Initial Thailand policy

Use `THAILAND_SOURCE_REGISTER.md` to identify candidate authoritative material. No numeric rule should move from research into a verified profile until exact edition/page/table/figure applicability is recorded.

## Architecture invariant

**Standards are versioned, sourced interpretations applied to a design—not hidden constants inside geometry generation.**
