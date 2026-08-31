//! Deterministic reference-alignment and station-based cross-section kernel.
//!
//! This crate is intentionally renderer-free. Engineering coordinates are metres,
//! and all derived values are computed from the semantic alignment and component
//! profiles exposed here.

pub mod alignment;
pub mod cross_section;
pub mod error;
pub mod math;
pub mod policy;
pub mod station;

pub use alignment::{
    Alignment, AlignmentKind, CircularArcAlignment, LineAlignment, Projection, SamplePoint,
    SamplingOptions, SmoothConceptualCurve,
};
pub use cross_section::{
    ComponentId, ComponentKind, ComponentState, CrossSection, CrossSectionComponent,
    PiecewiseLinearWidthProfile, WidthKnot,
};
pub use error::KernelError;
pub use math::{Point2, Vector2};
pub use policy::TolerancePolicy;
pub use station::StationRange;

/// Minimal scalar ABI probe used only to prove that the safe core can be emitted
/// as a WASM cdylib. Rich semantic calls remain on the Rust side until a binding
/// adapter is deliberately evaluated in a later stage.
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn r1a_wasm_feasibility_probe(value: f64) -> f64 {
    value
}
