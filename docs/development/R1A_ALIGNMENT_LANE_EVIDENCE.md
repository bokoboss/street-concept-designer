# R1A Alignment & Lane Lifecycle Evidence

This document records the bounded R1A execution evidence. It is intentionally
separate from renderer, UI, junction, topology, persistence, map, and asset
implementation.

## Execution identity

- Repository: `https://github.com/bokoboss/street-concept-designer`
- Execution branch: `codex/r1a-alignment-lanes`
- Exact execution base SHA: `9aeefd63305bf35c340c3959a011b020301c3697`
- Previous reviewed HEAD: `cf2628a08961120ceec4fbeaa854135557af9620` (PR #11)
- Remediation scope: independent-review findings F-01 (smooth sampling chord
  error) and F-02 (Windows/MSVC native qualification), plus low-cost full-turn
  arc regression coverage.
- Base verification: `origin/main` and `origin/codex/r1a-alignment-lanes` both
  resolved to the exact base SHA before implementation; the worktree was clean.
- The pasted task brief calls this GitHub issue `#9`, while the checked-in R1A
  contract names issue `#2`. Both public issue endpoints returned 404 during A0,
  so this execution follows the pasted R1A objective/contract as the authoritative
  task input and does not infer issue state.

## A0 — Workflow and toolchain

The installed Engineering Development Workflow validation passed before any
implementation file was added:

```text
python <workflow-v1.5.0>/scripts/setup_project.py validate D:\R&D\street-concept-designer
VALIDATION PASS (8 managed files, 2 project-owned files)
```

Selected toolchain and feasibility environment:

| Tool | Version / target | Role | Status |
|---|---|---|---|
| `rustc` | 1.98.0 (`88d9e12ae`, 2026-08-18) | kernel compiler | selected |
| `cargo` | 1.98.0 (`797e8a9bc`) | build/test/bench | selected |
| Rust host | `x86_64-pc-windows-msvc` | Windows-first project toolchain | `cargo check` available; MSVC linker absent on this image |
| Rust native test fallback | `stable-x86_64-pc-windows-gnu` | local executable test run | passed with validation-only WinLibs GCC |
| Rust WASM target | `wasm32-unknown-unknown` | cdylib feasibility/build target | selected and passed |
| Node.js | v24.19.0, workspace bundle `26.826.12353` | inspected; not used by R1A | no project dependency added |
| Python | 3.12.13, workspace bundle `26.826.12353` | workflow validation | passed |
| `wasm-pack` / `wasm-bindgen` / `wasm-tools` | not installed | JS adapter/tooling | intentionally not adopted in R1A |

The throwaway A1 proof used no third-party crate and passed native test
execution plus a 1,629,251-byte WASM cdylib compile. The project crate repeats
the proof through its own native test and WASM build commands.

## A1 — Dependency decision

R1A adopts no geometry, serialization, test, fuzzing, or benchmark crates.
`Cargo.lock` contains only the project package. The kernel uses Rust `std`
(`f64`, collections, and timing only); no third-party runtime license or
transitive dependency was introduced. Rust itself is distributed under the
Rust project MIT/Apache-2.0 licensing model.

The locally installed WinLibs GCC bundle and LLVM-MinGW archive were validation
toolchains only. They are not Cargo dependencies, are not linked into the
project artifact, and are not committed. A future JavaScript-facing adapter
may evaluate `wasm-bindgen`; R1A deliberately proves the pure Rust/WASM core
boundary without locking that adapter or its license/transitive bundle.

## Independent review remediation

The independent review recorded on PR #11 found two R1A acceptance gaps. F-01
was a real correctness defect: `SampleContext::subdivide` inspected only the
single station midpoint. On the symmetric inflection fixture below, that
midpoint lies exactly on the endpoint chord, so the complete 1.659 m curve
could be returned as one segment despite approximately 0.281 m interior
deviation against a `0.001 m` budget. F-02 was an evidence gap: the prior
native run used Windows GNU locally and Linux in CI, while the local MSVC
environment could only be checked, not linked and executed.

The F-01 correction keeps the sampler local and primitive-aware:

- Lines report zero geometric chord error and are subdivided only by station
  span.
- Circular arcs use the analytic maximum sagitta for the interval,
  `2 r sin²(|Δθ| / 4)`.
- Smooth cubics map the station interval to its parameter interval, derive the
  exact cubic subcurve controls with de Casteljau subdivision, and use the
  maximum distance of all four controls to the returned endpoint chord as a
  conservative convex-hull upper bound. Because the cubic lies in that hull,
  an accepted interval cannot hide an inflection behind a zero midpoint
  deviation.

The mandatory regression fixture is `p0=(0,0)`, `p1=(0,1)`,
`p2=(1,-1)`, `p3=(1,0)`, sampled with `max_chord_error_m=0.001 m` and
`max_segment_length_m=10 m`. The test requires more than endpoint output and
probes 16 evenly spaced interior stations in every returned interval. All
probes stayed within `0.001 m + 1e-8 m`; the extra margin is ten times the
named `station_bound_m` and covers floating-point station/interpolation
rounding only. The sampler now subdivides this short curve because of its
geometric bound, not because the 10 m station limit was reduced.

F-01 coverage also includes named symmetric-inflection, asymmetric S-curve,
shallow, strong-curvature, and near-degenerate-valid-tangent cubics, plus 64
bounded deterministic generated cubics. These cases check finite length,
monotonic bounded samples, finite point/frame values, bounded finite
projection, requested chord-error probes, and repeatability.

F-02 adds a `windows-latest` GitHub Actions job using the pinned standard
`1.98.0-x86_64-pc-windows-msvc` toolchain. It runs formatting, strict Clippy,
native check, native tests, and a native release build while the Ubuntu native
and WASM job remains in place. Hosted run/job identifiers are recorded in the
qualification table after the remediation commit is executed.

## Numerical policy

All kernel tolerance values are centralized in `TolerancePolicy`; algorithms do
not introduce local epsilon literals. The evidence-backed R1A defaults are:

| Policy field | Default | Meaning |
|---|---:|---|
| `coordinate_coincidence_m` | `1e-9 m` | vector/point coincidence classification |
| `station_bound_m` | `1e-9 m` | endpoint-only station clamping allowance |
| `angular_rad` | `1e-12 rad` | arc-angle classification |
| `minimum_alignment_length_m` | `1e-6 m` | minimum accepted primitive/domain length |
| `projection_convergence_m` | `1e-9 m` | smooth projection refinement stop distance |
| `adaptive_curve_error_m` | `1e-3 m` | smooth-curve flattening deviation |
| `sampling_max_segment_length_m` | `10 m` | default derived station interval |

The policy validates finiteness and positive domains. Negative widths, invalid
station order, zero-length primitives, non-finite values, and out-of-range
stations are rejected explicitly. A one-millimetre line remains above the
minimum accepted length, so the policy does not collapse the adversarial
millimetre-scale fixture.

## Kernel surface

- `Alignment::Line` — analytic length, point, tangent, left normal, bounded
  projection, and adaptive sampling.
- `Alignment::CircularArc` — directed positive/negative sweep, analytic length,
  analytic frame, bounded radial projection, and adaptive sampling.
- `Alignment::SmoothConceptualCurve` — cubic Bezier conceptual curve with a
  deterministic adaptive flattening/arc-length lookup table, bounded station
  queries, frame, per-lookup-interval projection refinement, and sampling.
- `Alignment` remains a tagged abstraction so a future spiral/clothoid variant
  can be added without changing station-based consumers.
- `CrossSection` stores an ordered vector of semantic components. Each
  `CrossSectionComponent` has a stable `ComponentId`, `ComponentKind`, and a
  piecewise-linear width profile. `states_at` preserves identity and order.
- `TrafficLane` is the only lane kind needed for through, add/drop, taper, and
  right-turn-storage fixtures. There is no turn-pocket polygon or renderer
  object in the model.

## Fixture inventory

### Canonical fixtures

1. Straight 500 m reference alignment.
2. Divided four-lane ordered cross-section: lane, lane, median, lane, lane.
3. 90-degree circular arc.
4. Smooth S-shaped conceptual cubic curve.
5. Constant-width lane.
6. Widening and narrowing profiles.
7. Zero-to-full lane add and full-to-zero lane drop.
8. Right-turn storage lane: zero width, taper, full storage, taper to zero.

### Added smooth-curve remediation fixtures

1. Symmetric inflection cubic from the independent-review counterexample.
2. Asymmetric S-curve.
3. Shallow curve.
4. Strong-curvature curve.
5. Near-degenerate but valid endpoint tangents.
6. Sixty-four bounded deterministic generated smooth cubics.

### Adversarial fixtures

1. One-millimetre accepted alignment and below-minimum rejection.
2. Near-parallel large-coordinate line with a small local offset.
3. Large engineering coordinates (`1e9` scale) with finite point/projection/frame.
4. Non-finite query and station rejection.
5. Abrupt/closely spaced width-profile knots and negative-width rejection.
6. Deterministic finite query corpus of 10,000 queries per primitive.
7. Generated property-style lines/arcs over bounded random parameter ranges.
8. Full positive and negative circular turns.

Junction, topology, surfaces, UI, maps, renderers, persistence, assets,
standards, and R1B/R1C fixtures are intentionally absent.

## Qualification gate record

The post-remediation local qualification run produced the following results.
The hosted PR/CI identifiers are recorded separately below after execution.

| Gate | Evidence method | Result |
|---|---|---|
| A-G0 | baseline verification + workflow validation | PASS — baseline/branch identity preserved; workflow validation rerun after remediation |
| A-G1 | Windows/MSVC hosted native test/build plus Ubuntu native and `wasm32-unknown-unknown` release build | PASS — hosted Windows/MSVC run recorded below; local GNU fallback and WASM build also exit 0 |
| A-G2 | `tests/r1a_alignment.rs` canonical line/arc/curve/frame/sampling tests | PASS — 8 tests, including inflection and positive/negative full-turn coverage |
| A-G3 | station bounds, projection, frame, primitive-aware chord-error sampling, near-degenerate tests | PASS — inflection regression probes every returned interval and remains within the configured budget |
| A-G4 | `tests/r1a_cross_section.rs` order/width/profile tests | PASS — 7 tests |
| A-G5 | add/drop/right-turn-storage identity and active-state tests | PASS — same `TrafficLane` component/profile mechanism |
| A-G6 | repeated semantic construction/sample/state equality tests | PASS — 3 tests |
| A-G7 | large-coordinate finite projection/frame/property tests | PASS — `1e9`-scale fixture and generated cases green |
| A-G8 | deterministic property-style sweep + panic-guarded fuzz-style corpus + smooth adversarial/generated corpus | PASS — 256 lines, 128 arcs, 5 named + 64 generated smooth cubics, 30,000 finite queries |
| A-G9 | custom `cargo bench --bench r1a_kernel` preliminary timings | PASS — 3 post-remediation release runs: query 36.02–39.41 ns/op; 500 m regeneration 12.7–15.5 µs; profile queries 85.62–88.37 ns/op |
| A-G10 | policy unit tests + diff audit for centralized tolerance use | PASS — 2 policy tests; source audit found tolerance literals only in policy/tests/benchmark unit conversion |
| A-G11 | scoped diff review; no later-stage modules introduced | PASS — remediation remains limited to R1A sampling/tests/CI/evidence; no R1B work |

### Hosted CI evidence

The remediation commit’s hosted GitHub Actions run and its job identifiers are
recorded here after push/review execution:

| Workflow/job | Runner/toolchain | Result |
|---|---|---|
| `R1A Kernel Qualification / qualify-r1a-windows-msvc` | `windows-latest`, `1.98.0-x86_64-pc-windows-msvc` | pending hosted run |
| `R1A Kernel Qualification / qualify-r1a-kernel` | `ubuntu-latest`, native + `wasm32-unknown-unknown` | pending hosted run |

Additional exact local commands:

```text
cargo fmt --all -- --check                                      PASS
cargo clippy --locked --all-targets --all-features -- -D warnings PASS
cargo check --locked --all-targets                              PASS (MSVC host target)
cargo +stable-x86_64-pc-windows-gnu test --locked --all-targets --all-features -- --nocapture
                                                                  PASS (24 tests; benchmark target also ran)
cargo +stable-x86_64-pc-windows-gnu build --locked --release --target x86_64-pc-windows-gnu
                                                                  PASS; native DLL 1,409,697 bytes
cargo build --locked --target wasm32-unknown-unknown --release   PASS
cargo +stable-x86_64-pc-windows-gnu bench --locked --bench r1a_kernel
                                                                  PASS (three post-remediation runs recorded above)
C:\Users\kittipat_t\.cache\codex-runtimes\codex-primary-runtime\dependencies\python.exe <workflow-v1.5.0>/scripts/setup_project.py validate D:\R&D\street-concept-designer
                                                                  PASS (8 managed files, 2 project-owned files)
```

## Known limitations and architecture decisions

- Smooth conceptual curves use an adaptive polyline arc-length approximation
  governed by `adaptive_curve_error_m`; this is appropriate for the concept
  spike and is not a survey-grade spiral/clothoid implementation.
- The R1A core exposes pure Rust values and a WASM-compatible cdylib build, but
  does not add a JS binding or browser harness. The adapter/interface choice is
  intentionally deferred until renderer integration is in scope.
- The local image still uses the GNU fallback for executable tests because it
  lacks the MSVC linker/Visual C++ build tools. The pinned MSVC toolchain passes
  local checking, while the hosted `windows-latest` job supplies the required
  MSVC link/test/build evidence; Ubuntu continues to qualify native/WASM paths.
- No external geometry dependency was necessary for R1A. That is a reversible
  decision: polygon overlay/triangulation or spatial indexing can be evaluated
  later against the same fixtures if R1B/R1C requires it.
- No junction/topology or product UI work was started.
