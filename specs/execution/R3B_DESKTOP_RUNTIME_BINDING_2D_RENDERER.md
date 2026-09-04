# R3B — Desktop Runtime / Binding / 2D Renderer Proof

Status: CONTROL-PLANE CONTRACT — IMPLEMENTATION NOT STARTED

Parent: Issue #30

Exact execution base: record in the R3B GitHub Issue after this control-plane contract is accepted and merged.

## Work mode

- **STRICT**
- Mode rationale: first production desktop/frontend architecture, new dependency families, a new Rust-to-UI interface, security/runtime authority, and a bridge decision that affects all later editor work.
- Mode confidence: high.
- Escalation triggers: dual-owner pressure, inability to preserve accepted semantic/session boundaries, unresolved dependency/license conflict, no defensible bridge winner, required root-kernel dependency pollution, or inability to run a real Windows desktop proof.

## Workspace safety

- Target project root: `D:\R&D\street-concept-designer`
- Writable boundary: target project root only.
- External writes approved: **No**.
- No global package installs.
- No system/user environment modification.
- No registry/service/firewall changes.
- Do not modify another repository.
- Project-local npm/Cargo dependency download/cache activity is allowed through normal build tooling.
- Target-repository GitHub push/PR handoff is allowed after qualification; do not merge.

Required workflow:
- project `AGENTS.md`;
- `PROJECT_PROFILE.md`;
- shared workflow root `SKILL.md`;
- `CONTEXT_MANAGEMENT.md`;
- `WORK_MODE_ROUTING.md`;
- `ACCEPTANCE_AND_EVIDENCE.md`;
- `MODEL_ROUTING_POLICY.md`;
- `skills/research-gate/SKILL.md`;
- `skills/scrutinize/SKILL.md`;
- `skills/independent-review/SKILL.md`.

## Objective

Adopt the minimum production desktop/2D application stack and select exactly one authoritative Rust `ProjectSession` placement by measured evidence, then prove that a real Windows Tauri application can render and select accepted R1C/R3A-derived 2D engineering geometry through PixiJS without creating renderer-owned engineering truth.

## Scope

### Application scaffold

Create the minimum production-oriented application structure using:

- Tauri 2;
- React;
- TypeScript;
- Vite;
- npm + checked-in `package-lock.json`;
- Node 24 LTS for CI/build qualification;
- PixiJS 8 forced to WebGL for this packet.

Use project-local Tauri CLI; no global Tauri install.

### Cargo boundary

Application-specific Cargo packages must be above/isolated from the accepted root engineering workspace.

Expected shape may be:

```text
apps/desktop/
  package.json
  package-lock.json
  src/
  src-tauri/
  bridge-wasm/
```

Names may differ if justified.

Mandatory behavior:

- root engineering workspace remains able to run all inherited native and wasm32 gates without trying to compile Tauri;
- Tauri dependencies do not enter kernel/project-core/project-io/project-session;
- comparator wasm-bindgen dependencies do not enter those accepted crates;
- use workspace exclusion or equivalent explicit boundary if Cargo parent-workspace discovery requires it.

### Bridge comparator

Implement both candidates only long enough to compare them:

A. native Tauri-owned `ProjectSession`;
B. WASM-owned `ProjectSession`.

Both operate on the same checked-in R3B current-schema fixture and the same semantic operation contract.

Measure:

- initialization/load;
- revision/read;
- real R2C preview;
- matching commit/reset;
- R1C 2D derivation;
- S/M/L scene transfer;
- stale/error path;
- repeated drag-preview cadence;
- requestAnimationFrame heartbeat/UI stalls.

Use at least 5 recorded benchmark runs and the method in:
`docs/development/R3B_RUNTIME_BRIDGE_RESEARCH.md`.

### Bridge decision / cleanup

Select exactly one production semantic-session owner using the ordered research criteria.

After selection:

- production app has one `ProjectSession` only;
- loser semantic-owner path is removed from production reachability;
- benchmark-only loser code may remain only if clearly isolated from the production dependency/build graph; otherwise rely on Git history + evidence record;
- record selected bridge, rejected bridge, evidence, and rationale in an R3B evidence/decision document;
- update `docs/product/DECISION_REGISTER.md` from evidence-gated to the accepted owner only when the PR is ready for acceptance.

### Minimal PixiJS 2D proof

Using the selected bridge:

- load/initialize representative accepted project state;
- derive accepted R1C 2D scene;
- render via PixiJS WebGL;
- use declared local render origin;
- preserve semantic/scoped renderer ids;
- pointer-select at least one meaningful road/lane representation;
- map renderer hit to semantic id;
- keep selected id in application state;
- visibly highlight selection;
- show minimal diagnostics: selected id, session revision, bridge owner;
- dispose and cleanly rebuild renderer objects/caches with equivalent semantic geometry/mapping.

## Out of scope

Do not implement:

- R3C `CreateRoad`, `EditAlignment`, road configurations, lane-lifecycle editor commands, or turn-pocket commands;
- R3D production workspace, hierarchy, inspector, snapping, road draw/edit UX;
- R3F file-open/reference-image/calibration/persistence;
- Three.js or 3D runtime;
- MapLibre or map providers;
- WebGPU;
- Web Worker semantic-session architecture;
- Electron;
- Next.js/SSR/RSC;
- localhost runtime server/plugin;
- online services;
- shell/http/fs/dialog Tauri plugins unless a concrete R3B acceptance need appears and is escalated;
- final installer, Portable ZIP, WebView2 Fixed Version packaging, signing, updater, autosave/crash recovery;
- public plugin API;
- engineering/tolerance/schema changes unrelated to an unavoidable R3B blocker.

## Authoritative baseline

- Repository: `bokoboss/street-concept-designer`
- Parent R3 Issue: #30
- Accepted R3A merge: `127aa01556eeb73c1daca160b3e2fa68e211dca8`
- Current control-plane baseline before this contract: `fa7386c82e2766ca05e1bfb6f1d0196b65839db5`
- Exact R3B execution base and branch are pinned in the R3B Issue after this contract is merged.

## Research / decision basis

Research gate required: **Yes**

Research verdict: **GO WITH CONDITIONS**

Primary records:

- `docs/development/R3_APPLICATION_STACK_RESEARCH.md`
- `docs/development/R3B_RUNTIME_BRIDGE_RESEARCH.md`
- `docs/architecture/TECHNICAL_ARCHITECTURE.md`
- `docs/architecture/RENDERER_CONTRACT.md`
- `docs/architecture/WINDOWS_DISTRIBUTION_POLICY.md`
- `docs/development/DEPENDENCY_LICENSE_REGISTER.md`

Conditions carried into execution:

1. same semantic fixture/operations for both bridge candidates;
2. UI-thread/rAF health measured;
3. exactly one production `ProjectSession` owner after comparator;
4. WebGL-only Pixi proof;
5. app dependencies isolated from root engineering workspace;
6. static/no-localhost packaged runtime;
7. exact dependency/license evidence;
8. Windows build/runtime evidence;
9. inherited R1/R2/R3A evidence preserved;
10. independent review before merge.

## Execution routing

- Recommended model: **GPT-5.6 Luna**
- Reasoning effort: **Max**
- Chat: **new Codex chat**
- Execution strategy: single writer.

Routing rationale:
The architecture question, comparator design, decision rule, dependency boundary, security constraints, and acceptance matrix are now bounded by the control plane. Remaining work is difficult multi-tool implementation and Windows runtime evidence, but strongly specified and testable.

Why a higher tier is not required:
No unresolved product architecture choice should be invented by the executor. The bridge choice is an evidence result with explicit selection criteria rather than an open-ended design task.

Escalation:
- Terra High/Max if a concrete Tauri/WASM integration conflict requires materially broader cross-module judgment after a minimal reproducer exists.
- Sol only if protected architecture contracts conflict or independent evidence remains contradictory after bounded investigation.

## Context strategy

Use a **new** Codex context because R3B is a new application-architecture packet.

Load only:

- `PROJECT_PROFILE.md`
- `ENGINEERING_CONSTITUTION.md`
- `AGENTS.md`
- `docs/development/ENGINEERING_WORKFLOW.md`
- `docs/development/R3B_RUNTIME_BRIDGE_RESEARCH.md`
- this execution contract;
- `docs/development/R3_APPLICATION_STACK_RESEARCH.md`
- `docs/architecture/TECHNICAL_ARCHITECTURE.md`
- `docs/architecture/RENDERER_CONTRACT.md`
- `docs/architecture/WINDOWS_DISTRIBUTION_POLICY.md`
- `docs/development/DEPENDENCY_LICENSE_REGISTER.md`
- `docs/product/DECISION_REGISTER.md`
- accepted R1C/R2C/R3A evidence/specs only as needed for exact fixture/session interfaces.

Do not preload unrelated standards/assets/map/UX documents.

## Constraints / invariants

- Rust `ProjectSession` is the only semantic mutation authority.
- JavaScript owns ephemeral application/view/selection/request state only.
- Both comparator paths use the same accepted Project/ProjectSession semantics.
- Renderer adapters consume derived engineering output only.
- local render origin is applied before GPU float conversion.
- semantic ids survive bridge/render hit testing.
- selection does not mutate canonical engineering state.
- no renderer cache persists as project truth.
- no runtime localhost dependency.
- no end-user development toolchain requirement.
- equality of semantic input must still produce equivalent derived output regardless of bridge.

## Protected areas / do not change

Do not change unless a stop/escalation condition is met:

- kernel geometry/tolerance policy;
- R3A composite alignment semantics;
- R2B schema-v2 migration contract;
- R2C revision/preview/commit/undo semantics;
- junction/topology ownership;
- shared R1C derivation algorithms;
- standards behavior;
- map/reference policy;
- R3C+ product semantics.

## Comparator requirements

### Fixed canonical fixture

Add one reviewable current-schema R3B fixture that exercises:

- at least one multi-segment R3A Road;
- non-empty CrossSection/traffic-lane content;
- scoped semantic ids;
- a deterministic derived 2D scene.

Do not replace existing historical R2B migration fixtures.

### Common benchmark API

Both candidates must expose benchmark-equivalent operations for:

- load/init;
- revision;
- preview;
- commit/reset;
- derive scene;
- scene transfer;
- stale/error case.

Bridge adapters may differ internally, but the benchmark driver must compare equivalent observable semantics.

### Scene packet benchmark

Record for S/M/L:

- primitive count;
- semantic-id count;
- numeric coordinate/value count where applicable;
- transferred bytes;
- encode time;
- bridge time;
- JS decode/materialization time.

Use optimized bulk bytes/TypedArray paths when appropriate rather than proving only a knowingly inefficient transport.

### Drag benchmark

At least:

- 120 requested preview samples;
- 60 Hz requested cadence;
- 5 repeated runs per candidate;
- p50/p95/p99/max;
- completed/superseded/out-of-order counts;
- rAF intervals >33.3 ms and >100 ms.

Benchmark-only request sequence ids are allowed; canonical session revision still governs semantic staleness.

### Selection rule

Apply the ordered criteria from the research record.

Do not retain both because results are close.

If neither meets the representative responsiveness floor or results are materially contradictory, stop with `ARCHITECTURE_ESCALATION`.

## PixiJS proof requirements

- `Application.init({ preference: 'webgl', ... })` or equivalent explicit WebGL-only configuration.
- Renderer input is a derived R1C-facing DTO/adapter, never direct Project internals.
- Deterministic initial view sufficient to see the fixture.
- Meaningful Pixi objects have semantic renderer ids in hit mapping.
- Pointer click selects semantic id.
- Selected representation highlights without changing Project state.
- Cache/object destruction + rebuild returns equivalent primitive/id mapping.
- Release proof must not silently fall back to WebGPU.

## Security / application-runtime requirements

- static bundled frontend;
- no remote URL loaded as app content;
- no localhost plugin/server in packaged runtime;
- no shell/http/fs/dialog plugin by default;
- minimal Tauri capability/permission configuration;
- explicit CSP; no wildcard remote source;
- release build does not require dev server;
- malformed bridge/project input returns a controlled error and does not blank/corrupt the semantic session.

## Dependency discipline

### Frontend

Use exact resolved stable versions and commit:

- `package.json`;
- `package-lock.json`.

Expected direct families during the comparator (final selected graph must remove any direct dependency that becomes unused):

- `react`;
- `react-dom`;
- `pixi.js`;
- `@tauri-apps/api`;
- `@tauri-apps/cli` as a development dependency;
- `vite`;
- React Vite plugin if used;
- `typescript`;
- only minimal test tooling justified by actual R3B tests.

No canary/dev/prerelease dependency.

### Rust application adapters

Use exact pins/lockfiles for selected application crates where project policy requires exact adoption evidence.

The accepted root engineering crates must not gain Tauri/wasm-bindgen dependencies merely for convenience.

### License evidence

Before acceptance:

- update `docs/development/DEPENDENCY_LICENSE_REGISTER.md` with selected direct application dependencies;
- create/update an R3B dependency evidence record containing the exact transitive npm and application-Cargo graphs/licenses from lockfiles/metadata;
- identify notices or license-choice decisions;
- record Node/npm/Rust toolchain versions.

## Independent review

Required: **Yes**

Reason:
R3B selects a long-lived application/session architecture and adds the first production desktop dependency graph.

Independence basis:
fresh control-plane context or equivalent materially independent reviewer, plus deterministic comparator/CI evidence.

Review scope:

- bridge selection actually follows recorded evidence;
- no production dual `ProjectSession`;
- comparator is semantically fair;
- UI-thread metrics were not omitted;
- root engineering dependency/workspace boundary preserved;
- Pixi renderer does not own engineering truth;
- local-origin and semantic selection mappings are correct;
- Tauri security/runtime configuration is minimal;
- dependency/license evidence is complete;
- Windows artifact/runtime evidence is real;
- no R3C+ scope leakage.

Same-tier fresh-context review is sufficient unless the evidence itself is contradictory.

## Success gates

| Gate | Criterion | Method / evidence | Required |
|---|---|---|---|
| R3B-G0 | Exact accepted base, clean worktree, scope/workspace safety | Git status/base SHA + changed-file audit | Yes |
| R3B-G1 | Stable exact dependency graph and licenses | npm lock + Cargo locks/metadata + license evidence | Yes |
| R3B-G2 | Static Tauri/React/Vite shell builds without runtime localhost | frontend production build + Tauri config/build inspection | Yes |
| R3B-G3 | Both bridge candidates preserve identical semantic fixture/results | comparator correctness assertions | Yes |
| R3B-G4 | Comparator records required latency/payload/rAF evidence | 5-run benchmark evidence on Windows | Yes |
| R3B-G5 | Exactly one production ProjectSession owner remains | source/dependency/build graph audit + regression | Yes |
| R3B-G6 | PixiJS WebGL renders accepted derived scene | real app runtime proof + deterministic adapter checks | Yes |
| R3B-G7 | local origin + semantic selection mapping | targeted tests + runtime selection proof | Yes |
| R3B-G8 | renderer caches are disposable/rebuild-equivalent | destroy/rebuild regression | Yes |
| R3B-G9 | Minimal security/runtime authority | Tauri capabilities/CSP/plugin/network audit + controlled error probe | Yes |
| R3B-G10 | Windows/MSVC desktop build and launch smoke | Windows release/dev build + launch/runtime evidence | Yes |
| R3B-G11 | Root inherited R1/R2/R3A behavior unaffected | required existing root workflows/tests + wasm32 gates | Yes |
| R3B-G12 | App/frontend CI reproducible from clean install | `npm ci`, typecheck/test/build + selected bridge builds | Yes |
| R3B-G13 | Evidence record and decision register complete | docs/diff audit | Yes |
| R3B-G14 | Independent review | fresh-context review decision | Yes |

## Evidence reuse

May reuse:

- accepted R1C shared-derived-scene correctness;
- accepted R2C `ProjectSession` semantics;
- accepted R3A composite-alignment semantics/schema migration;
- current Windows distribution policy;
- research-family license findings.

Do not claim reused evidence proves:

- Tauri Windows runtime;
- bridge transport performance;
- app dependency graph/license completeness;
- Pixi/WebView2 behavior.

Those require new R3B evidence.

## Required evidence artifact

Create:
`docs/development/R3B_DESKTOP_RUNTIME_BINDING_EVIDENCE.md`

It must record:

- exact execution base/head;
- package/toolchain versions;
- candidate architecture snippets;
- benchmark fixtures/payload sizes;
- full benchmark tables;
- bridge selection;
- loser cleanup;
- 2D renderer proof;
- selection/local-origin/cache evidence;
- security/capability/CSP evidence;
- dependency/license evidence location;
- Windows build/launch evidence;
- inherited CI;
- limitations;
- independent-review status.

## Stop conditions

Stop and report rather than expanding scope if:

- comparator would require two production semantic sessions;
- a candidate needs JavaScript-owned canonical Road/Project state;
- Tauri or wasm-bindgen must enter kernel/project-core/project-io/project-session;
- root accepted wasm32 gates would need to be weakened to accommodate the app;
- bridge selection cannot be justified by the specified evidence;
- selected architecture cannot meet the representative responsiveness floor;
- Pixi requires renderer-side engineering recomputation;
- a new public semantic command/API is required before R3C;
- file handling/reference persistence becomes necessary to prove R3B;
- system/global installs or writes outside the project root are required.

## Escalation conditions

Report `ARCHITECTURE_ESCALATION` with the smallest reproducible evidence if:

- both bridge candidates fail required responsiveness/correctness;
- one-owner architecture conflicts with required Tauri/WebView behavior;
- WASM main-thread constraints make it nonviable and native IPC simultaneously cannot meet interaction needs;
- dependency/security constraints block both viable paths;
- accepted R1/R2/R3A contracts must change.

## Definition of done

R3B is done only when:

- minimum Tauri/React/Vite/PixiJS desktop app exists;
- both bridge candidates were fairly compared;
- exactly one production `ProjectSession` owner is selected;
- losing owner is removed/isolated from production;
- PixiJS WebGL renders accepted derived 2D geometry;
- semantic selection/local-origin/cache rebuild are proven;
- exact dependency/license evidence exists;
- Windows desktop build/launch is proven;
- inherited root gates remain green;
- final-head hosted CI is green;
- independent review is PASS or PASS WITH CONDITIONS whose conditions are completed;
- PR remains unmerged for control-plane acceptance.

## Final recommendation

Exactly one:

- `PROCEED_TO_R3C`
- `REMEDIATE_R3B`
- `ARCHITECTURE_ESCALATION`

Do not start R3C automatically.
