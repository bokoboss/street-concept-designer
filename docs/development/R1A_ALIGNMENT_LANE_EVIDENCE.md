# R1A Alignment & Lane Lifecycle Evidence

This document records the bounded R1A execution evidence. It is intentionally
separate from renderer, UI, junction, topology, persistence, map, and asset
implementation.

## Execution identity

- Repository: `https://github.com/bokoboss/street-concept-designer`
- Execution branch: `codex/r1a-alignment-lanes`
- Exact execution base SHA: `9aeefd63305bf35c340c3959a011b020301c3697`
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

### Adversarial fixtures

1. One-millimetre accepted alignment and below-minimum rejection.
2. Near-parallel large-coordinate line with a small local offset.
3. Large engineering coordinates (`1e9` scale) with finite point/projection/frame.
4. Non-finite query and station rejection.
5. Abrupt/closely spaced width-profile knots and negative-width rejection.
6. Deterministic finite query corpus of 10,000 queries per primitive.
7. Generated property-style lines/arcs over bounded random parameter ranges.

Junction, topology, surfaces, UI, maps, renderers, persistence, assets,
standards, and R1B/R1C fixtures are intentionally absent.

## Qualification gate record

The final local qualification run produced the following results. The PR/CI
identifier is intentionally external to this repository evidence file.

| Gate | Evidence method | Result |
|---|---|---|
| A-G0 | baseline verification + workflow validation | PASS before implementation |
| A-G1 | Native `x86_64-pc-windows-gnu` test/build plus `wasm32-unknown-unknown` release build | PASS — tests/build exit 0; native DLL 1,407,649 bytes; WASM 418 bytes, valid `00 61 73 6D 01 00 00 00` header |
| A-G2 | `tests/r1a_alignment.rs` canonical line/arc/curve/frame/sampling tests | PASS — 6 tests |
| A-G3 | station bounds, projection, frame, adaptive sampling, near-degenerate tests | PASS — alignment/adversarial coverage green |
| A-G4 | `tests/r1a_cross_section.rs` order/width/profile tests | PASS — 7 tests |
| A-G5 | add/drop/right-turn-storage identity and active-state tests | PASS — same `TrafficLane` component/profile mechanism |
| A-G6 | repeated semantic construction/sample/state equality tests | PASS — 3 tests |
| A-G7 | large-coordinate finite projection/frame/property tests | PASS — `1e9`-scale fixture and generated cases green |
| A-G8 | deterministic property-style sweep + panic-guarded fuzz-style corpus | PASS — 256 lines, 128 arcs, 30,000 finite queries |
| A-G9 | custom `cargo bench --bench r1a_kernel` preliminary timings | PASS — 3 release runs: query 37.45–40.01 ns/op; 500 m regeneration 13.5–20.7 µs; profile queries 89.35–108.32 ns/op |
| A-G10 | policy unit tests + diff audit for centralized tolerance use | PASS — 2 policy tests; source audit found tolerance literals only in policy/tests/benchmark unit conversion |
| A-G11 | scoped diff review; no later-stage modules introduced | PASS — no junction/topology/UI/map/asset/persistence modules in R1A diff |

Additional exact local commands:

```text
cargo fmt --all -- --check                                      PASS
cargo clippy --locked --all-targets --all-features -- -D warnings PASS
cargo check --locked --all-targets                              PASS (MSVC host target)
cargo +stable-x86_64-pc-windows-gnu test --locked --all-targets --all-features -- --nocapture
                                                                  PASS (21 tests; benchmark target also ran)
cargo +stable-x86_64-pc-windows-gnu build --locked --release --target x86_64-pc-windows-gnu
                                                                  PASS
cargo build --locked --target wasm32-unknown-unknown --release   PASS
cargo +stable-x86_64-pc-windows-gnu bench --locked --bench r1a_kernel
                                                                  PASS (three runs recorded above)
```

## Known limitations and architecture decisions

- Smooth conceptual curves use an adaptive polyline arc-length approximation
  governed by `adaptive_curve_error_m`; this is appropriate for the concept
  spike and is not a survey-grade spiral/clothoid implementation.
- The R1A core exposes pure Rust values and a WASM-compatible cdylib build, but
  does not add a JS binding or browser harness. The adapter/interface choice is
  intentionally deferred until renderer integration is in scope.
- The native Windows test executable used the GNU fallback because this image
  lacks the MSVC linker/Visual C++ build tools. The pinned MSVC toolchain still
  passed host-target checking, and CI runs the same pure Rust crate on Linux.
- No external geometry dependency was necessary for R1A. That is a reversible
  decision: polygon overlay/triangulation or spatial indexing can be evaluated
  later against the same fixtures if R1B/R1C requires it.
- No junction/topology or product UI work was started.
