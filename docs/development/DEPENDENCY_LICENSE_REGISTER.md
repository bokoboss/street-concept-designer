# Dependency / License Register

Verified research date: 2026-08-27

## Purpose

Track candidate runtime, geometry, testing, UI, and asset-source dependencies before adoption. A candidate appearing here is **not approved or installed** merely because its license is permissive.

Before adoption also review:
- maintenance/activity;
- platform/native/WASM support;
- bundle/binary size;
- performance;
- security history;
- transitive dependencies;
- API stability;
- compatibility with the project architecture.

## Status

- CANDIDATE — may be evaluated.
- PREFERRED_CANDIDATE — currently best fit, still evidence-gated.
- ADOPTED — intentionally added to project with version/license recorded.
- HOLD — do not adopt until a named risk is resolved.
- REJECTED — incompatible with current product/governance.

---

# A. Desktop / UI / rendering candidates

| Dependency | Role | Current license observed | Status | Notes / source |
|---|---|---|---|---|
| Tauri 2 / tauri-apps/tauri | desktop shell/native bridge | MIT or MIT/Apache-2.0 where applicable | PREFERRED_CANDIDATE | https://github.com/tauri-apps/tauri — code license permissive; Tauri logo has separate CC-BY-NC-ND terms, so do not redistribute/rebrand logo casually |
| React | UI component/state layer | MIT | PREFERRED_CANDIDATE | https://github.com/facebook/react |
| PixiJS | high-performance 2D engineering renderer | MIT | PREFERRED_CANDIDATE | https://github.com/pixijs/pixijs |
| Three.js | 3D renderer | MIT | PREFERRED_CANDIDATE | https://github.com/mrdoob/three.js |
| MapLibre GL JS | map/reference renderer | BSD-3-Clause | PREFERRED_CANDIDATE | https://github.com/maplibre/maplibre-gl-js; renderer license does not grant rights to basemap data/providers |
| Lucide | general UI icon set | ISC, with inherited Feather icons under MIT notices where applicable | CANDIDATE | https://github.com/lucide-icons/lucide; do not use it as source of traffic sign artwork |

Current observed upstream examples:
- PixiJS repository showed v8.18.1 as latest release in April 2026 at research time.
- Three.js repository showed r184 as latest release in April 2026 at research time.
- MapLibre package metadata showed 6.0.0 at research time.

These observations are **not version pins**. Exact versions must be selected and locked only in an implementation PR with test evidence.

---

# B. Frontend build / test candidates

| Dependency | Role | License | Status | Notes |
|---|---|---|---|---|
| Vite | frontend build/dev tooling | MIT | PREFERRED_CANDIDATE | https://github.com/vitejs/vite |
| Vitest | TS/JS unit/component tests | MIT | PREFERRED_CANDIDATE | https://github.com/vitest-dev/vitest; published artifact contains bundled dependencies under permissive MIT/BSD/ISC licenses per upstream notices |
| Playwright | browser/E2E/visual regression | Apache-2.0 | PREFERRED_CANDIDATE | https://github.com/microsoft/playwright |
| Vite build license output | license notice generation | feature of Vite | CANDIDATE | current Vite docs support generating a bundled-dependency license file via build license settings |

Adoption expectation:
- lock versions;
- commit lockfile;
- include license-notice generation in release qualification where feasible;
- do not rely on package homepage summaries instead of actual package/license metadata.

## B1. R3B qualification graph (exact lockfile evidence)

These are the exact direct application dependencies used by the R3B proof. They
are adopted for this qualification packet; the production desktop decision
remains subject to the R3B evidence record and Windows acceptance gates.

| Dependency | Exact version | License in lock metadata | Role | Evidence |
|---|---:|---|---|---|
| `@tauri-apps/api` | 2.11.1 | Apache-2.0 OR MIT | native IPC client | `apps/desktop/package-lock.json` |
| `@tauri-apps/cli` | 2.11.4 | Apache-2.0 OR MIT | project-local build CLI | `apps/desktop/package-lock.json` |
| `pixi.js` | 8.20.1 | MIT | forced-WebGL 2D renderer | `apps/desktop/package-lock.json` |
| `react` / `react-dom` | 19.2.8 | MIT | UI shell | `apps/desktop/package-lock.json` |
| `vite` | 8.2.2 | MIT | static frontend build | `apps/desktop/package-lock.json` |
| `@vitejs/plugin-react` | 6.1.1 | MIT | React Vite transform | `apps/desktop/package-lock.json` |
| `typescript` | 7.0.2 | Apache-2.0 | frontend type checking | `apps/desktop/package-lock.json` |
| `tauri` | 2.11.5 | Apache-2.0 OR MIT | Windows desktop shell/native owner candidate | `apps/desktop/Cargo.lock` / `cargo metadata` |
| `tauri-build` | 2.6.3 | Apache-2.0 OR MIT | Tauri build integration | `apps/desktop/Cargo.lock` / `cargo metadata` |
| `wasm-bindgen` | 0.2.127 | MIT OR Apache-2.0 | selected WASM production owner | `apps/desktop/Cargo.lock` / `cargo metadata` |

The complete transitive graphs are intentionally kept in the checked-in npm
and Cargo lockfiles. The native core crates remain outside both application
graphs; `cargo metadata --no-deps` confirms the root workspace contains only
the accepted kernel/project packages.

---

# C. Geometry kernel candidates — Rust path

These are candidates for R1 evaluation only. R1 decides actual adoption.

| Dependency | Potential role | License | Status | Main risk/evidence needed |
|---|---|---|---|---|
| georust/geo | geometry primitives/general algorithms | MIT OR Apache-2.0 | PREFERRED_CANDIDATE | verify exact algorithms/features used, WASM behavior, determinism/performance |
| iShape-Rust/iOverlay | robust polygon overlay/boolean/buffering | MIT OR Apache-2.0 | PREFERRED_CANDIDATE | verify float/integer strategy, WASM, precision/tolerance policy, complex junction behavior |
| cavalier_contours | line/arc polyline + parallel offsets | MIT OR Apache-2.0 | PREFERRED_CANDIDATE | verify exact offset semantics, arc support, WASM, integration with canonical alignment model |
| georust/rstar | spatial R-tree queries/snapping candidates | MIT OR Apache-2.0 | CANDIDATE | validate query performance and deterministic tie-ranking policy |
| Stoeoef/spade | Delaunay/constrained triangulation candidate | MIT OR Apache-2.0 | CANDIDATE/LATER | only if junction meshing needs it beyond simpler triangulation |
| Earcut algorithm/library | polygon triangulation | upstream Mapbox earcut is ISC | CANDIDATE | exact Rust/JS package chosen must have its own license verified; do not infer wrapper license from upstream algorithm |

Current upstream research notes:
- iOverlay advertises integer and floating-point APIs, boolean operations, buffering, simplification, and WASM relevance.
- iOverlay is also used by current georust/geo for polygon overlay.
- cavalier_contours explicitly mentions Rust/WASM tooling advantages and line/arc polyline processing.
- Spade uses a precise calculation kernel by default and is dual MIT/Apache-2.0.

These are functional claims to evaluate in R1, not reasons to bypass our own fixtures/fuzz/benchmark gates.

---

# D. Rust test/fuzz candidates

Potential tools:
- Rust built-in unit/integration tests;
- proptest for property-based testing;
- cargo-fuzz/libFuzzer for fuzzing;
- Criterion or another benchmark harness if appropriate.

Status:
CANDIDATE until R1 selects exact packages/versions/licenses.

Requirement:
The implementation PR must record exact package/license metadata rather than relying on this conceptual list.

---

# E. Asset / texture source candidates

## Poly Haven

Source:
https://polyhaven.com/license

Observed license:
CC0 for downloadable HDRIs, textures, and 3D models.

Status:
PREFERRED_CANDIDATE for generic presentation materials/context assets.

Policy:
- still retain source URL, asset id, author/source metadata where available;
- do not copy protected site branding, logos, example renders, or web content merely because downloadable assets are CC0;
- normalize/optimize before bundling.

## ambientCG

License documentation:
https://docs.ambientcg.com/license/

Observed policy:
downloadable assets are published under CC0 1.0.

Status:
PREFERRED_CANDIDATE for generic PBR surfaces/HDRIs and some presentation content.

Policy:
- retain provenance even when attribution is not legally required;
- do not treat the website/brand itself as CC0;
- do not scrape/bulk-download in ways that violate site/API terms.

## AI-generated assets

Status:
CANDIDATE only.

Requirements before bundling:
- model/service terms permit intended commercial redistribution;
- generated content has no obvious trademark/copyright cloning issue;
- source/prompt/model metadata policy defined;
- output reviewed and normalized;
- no false claim that AI-generated traffic signs/markings are official standards artwork.

## Traffic-control engineering assets

Default:
BUILD OUR OWN PROCEDURALLY or from verified public-authority geometry/source material as legally permitted.

Reason:
Correctness, dimensions, semantic metadata, and source traceability matter more than art convenience.

---

# F. High-risk / default-reject license classes

## Strong copyleft runtime/code dependencies

AGPL/GPL-family dependencies are not automatically banned in every possible use, but for a potentially proprietary/commercial standalone product they require explicit architecture/legal review before adoption.

Default:
HOLD.

Do not allow a coding agent to introduce them casually.

Examples of concern:
- AGPL editor/runtime libraries;
- GPL native libraries linked/bundled into proprietary application;
- code copied from an AGPL project.

3DStreet is useful for product/UX research, but its open-source licensing must be reviewed before any code reuse. The project policy is currently to **learn from patterns, not copy implementation**.

## Public repository with no license

Default:
REJECT CODE/TEXT COPYING unless explicit permission is obtained.

Example:
if a useful skills/research repository has no LICENSE file, we may independently recreate the workflow concept, but do not vendor/copy its text/code.

## “Source available” or custom noncommercial license

Default:
HOLD/REJECT for core runtime unless commercial/product rights are explicit.

---

# G. Map/data/provider licensing is separate from software licensing

MapLibre being BSD-3-Clause does **not** make Google, OSM-hosted tiles, satellite imagery, or any third-party tiles unrestricted.

Always use:
- `docs/architecture/MAP_BASEMAP_POLICY.md`;
- provider capability/terms records;
- attribution/cache/offline/export/digitization classification.

Do not put basemap license logic in the dependency register alone.

---

# H. Bundled notices / SBOM direction

Before beta/release, establish automated output covering:
- direct runtime dependency name/version/license;
- required copyright/license notice;
- bundled/transitive license notices where applicable;
- asset source/license/provenance;
- standards documents are references, not software dependencies;
- map/provider attribution separately.

Recommended standards/approaches:
- SPDX identifiers wherever possible;
- SPDX-compatible SBOM later;
- REUSE-style file-level licensing for project-authored/third-party source files where valuable.

---

# I. Dependency adoption gate

A candidate becomes ADOPTED only when a PR records:

1. exact dependency/package/crate name;
2. exact selected version;
3. upstream repository/source;
4. license/SPDX id;
5. required notices;
6. reason for dependency;
7. alternatives considered when material;
8. target support:
   - Windows;
   - native;
   - WASM where required;
9. performance/robustness evidence relevant to its role;
10. security/maintenance assessment appropriate to dependency criticality;
11. transitive-license review;
12. lockfile/update policy.

Geometry-critical dependencies additionally require canonical/adversarial fixture evidence.

---

# J. Current product stance

Preferred candidate stack remains:

```text
Desktop:    Tauri 2
UI:         React + TypeScript
2D:         PixiJS
3D:         Three.js
Map:        MapLibre GL JS + provider abstraction

Kernel:     Rust native/WASM is the first architecture hypothesis
            but remains evidence-gated by R1.
```

This document deliberately does not turn that hypothesis into an implementation lock.


# K. Windows runtime / distribution dependencies

## Microsoft Edge WebView2 Runtime

Role:
Windows webview runtime used by the Tauri shell.

Source:
https://learn.microsoft.com/en-us/microsoft-edge/webview2/

Status:
PLATFORM RUNTIME / DISTRIBUTION DEPENDENCY.

Distribution strategies under project policy:
- Evergreen for standard installed use;
- Evergreen offline standalone installer for offline setup;
- existing Evergreen for Portable Light;
- Fixed Version candidate for Portable Offline.

Important:
- Fixed Version must be serviced by our release process and cannot be frozen indefinitely.
- exact runtime version is a release artifact decision, not hard-coded in this preimplementation register.
- license/redistribution terms for the selected Microsoft runtime package must be checked at release adoption.

References:
- https://learn.microsoft.com/en-us/microsoft-edge/webview2/concepts/evergreen-vs-fixed-version
- https://learn.microsoft.com/en-us/microsoft-edge/webview2/concepts/distribution

## Windows code signing

Role:
publisher identity / trusted direct distribution.

Status:
REQUIRED RELEASE GOVERNANCE, provider not selected.

Candidate mechanisms may include:
- Microsoft Artifact Signing where eligible;
- trusted OV/other CA-issued code-signing certificate compatible with current Microsoft requirements.

Do not select a signing provider merely from price/convenience; verify eligibility, organization identity, CI secret/security handling, timestamping, renewal, and enterprise procurement needs.

Reference:
https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/smartscreen-reputation

## Portable packaging

Portable ZIP is a project release artifact, not a third-party runtime dependency.

Tauri supports `build --no-bundle`, but the final portable artifact must explicitly package/test:
- application executable;
- resources/assets;
- required sidecars/native files;
- selected WebView2 strategy;
- license/notices;
- diagnostics/version metadata.

See:
`docs/architecture/WINDOWS_DISTRIBUTION_POLICY.md`.

# L. R2B persistence dependencies

Qualification date: 2026-09-01. These are the exact packages resolved by the
R2B lockfile for `street-concept-designer-project-io`. The direct dependency
declarations are exact pins; all transitive packages below are recorded from
`cargo metadata --locked` and `cargo tree --locked`.

| Package | Version | Direct/transitive | Role | SPDX/license expression | Repository/source | Native/MSVC and WASM qualification |
|---|---:|---|---|---|---|---|
| `serde` | 1.0.229 | direct | Explicit DTO serialization/deserialization traits | MIT OR Apache-2.0 | https://github.com/serde-rs/serde | Pure Rust; compiled for native MSVC/Linux and WASM target |
| `serde_json` | 1.0.151 | direct | Compact JSON encoding/decoding with `float_roundtrip` | MIT OR Apache-2.0 | https://github.com/serde-rs/json | Pure Rust; compiled for native MSVC/Linux and WASM target |
| `serde_core` | 1.0.229 | transitive | Core Serde traits used by Serde and Serde JSON | MIT OR Apache-2.0 | https://github.com/serde-rs/serde | Pure Rust; native/MSVC and WASM dependency path qualified |
| `serde_derive` | 1.0.229 | transitive | Proc-macro implementation of DTO derives | MIT OR Apache-2.0 | https://github.com/serde-rs/serde | Host proc macro for native/MSVC/Linux builds; not linked into WASM artifact |
| `itoa` | 1.0.18 | transitive | Integer formatting used by Serde JSON | MIT OR Apache-2.0 | https://github.com/dtolnay/itoa | Pure Rust; native/MSVC and WASM dependency path qualified |
| `memchr` | 2.8.3 | transitive | Byte scanning used by Serde JSON | Unlicense OR MIT | https://github.com/BurntSushi/memchr | Pure Rust; native/MSVC and WASM dependency path qualified |
| `zmij` | 1.0.23 | transitive | Floating-point formatting used by Serde JSON | MIT | https://github.com/dtolnay/zmij | Pure Rust; native/MSVC and WASM dependency path qualified |
| `proc-macro2` | 1.0.107 | transitive | Proc-macro token support | MIT OR Apache-2.0 | https://github.com/dtolnay/proc-macro2 | Host proc macro for native/MSVC/Linux builds; not linked into WASM artifact |
| `quote` | 1.0.47 | transitive | Proc-macro token generation | MIT OR Apache-2.0 | https://github.com/dtolnay/quote | Host proc macro for native/MSVC/Linux builds; not linked into WASM artifact |
| `syn` | 3.0.4 | transitive | Proc-macro Rust syntax parsing | MIT OR Apache-2.0 | https://github.com/dtolnay/syn | Host proc macro for native/MSVC/Linux builds; not linked into WASM artifact |
| `unicode-ident` | 1.0.24 | transitive | Unicode identifier classification for proc macros | (MIT OR Apache-2.0) AND Unicode-3.0 | https://github.com/dtolnay/unicode-ident | Host proc macro dependency; Unicode notice retained by package metadata |

The selected feature graph is deliberately minimal: `serde` enables
`derive`; `serde_json` enables `float_roundtrip` (alongside its normal `std`
default). `preserve_order`, `arbitrary_precision`, `raw_value`, and
`unbounded_depth` are not enabled. No dependency has a native C library or
runtime service requirement. The complete resolved graph is permissively
licensed under project policy; the `memchr` Unlicense option and the
`unicode-ident` Unicode-3.0 notice are explicitly retained for downstream
notice generation.

The R2B acceptance fixtures cover native compilation and the project-io WASM
artifact. Hosted Linux and Windows/MSVC CI remain authoritative for the
workflow matrix because the development host lacks the MSVC linker and
Windows SDK libraries.
