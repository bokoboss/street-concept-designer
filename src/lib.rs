//! Deterministic reference-alignment and station-based cross-section kernel.
//!
//! This crate is intentionally renderer-free. Engineering coordinates are metres,
//! and all derived values are computed from the semantic alignment and component
//! profiles exposed here.

pub mod alignment;
pub mod cross_section;
pub mod error;
pub mod junction;
pub mod math;
pub mod policy;
pub mod render;
pub mod station;

pub use alignment::{
    Alignment, AlignmentKind, AlignmentPrimitive, AlignmentSegment, AlignmentSegmentId,
    CircularArcAlignment, CompositeAlignment, LineAlignment, Projection, SamplePoint,
    SamplingOptions, SmoothConceptualCurve,
};
pub use cross_section::{
    ComponentId, ComponentKind, ComponentState, CrossSection, CrossSectionComponent,
    PiecewiseLinearWidthProfile, WidthKnot,
};
pub use error::KernelError;
pub use junction::{
    detect_candidate, Approach, ApproachId, ApproachSide, AuthoredJunctionSnapshot,
    CandidateDisposition, Corner, CornerId, CrossingRelation, CrossingType, Junction,
    JunctionCandidate, JunctionId, JunctionOptions, JunctionStatus, LaneConnection,
    LaneConnectionId, LaneConnectivityMode, LaneDirection, Movement, PavementSurface,
    RegenerationResult, Road, RoadId, RoadNetwork,
};
pub use math::{Point2, Vector2};
pub use policy::TolerancePolicy;
pub use render::{
    derive_diagnostic_2d, derive_diagnostic_3d, DerivedComponent, DerivedComponentSample,
    DerivedComponentStrip, DerivedCorner, DerivedEngineeringSnapshot, DerivedJunction,
    DerivedLaneConnection, DerivedRoad, Diagnostic2D, Diagnostic3D, DiagnosticPrimitive2D,
    Extents2, MeshPrimitive3D, MeshTopology, Polygon2D, Polyline2D, PrimitiveRole, SemanticRef,
    SnapshotMetadata, FLOAT32_LOCAL_COORDINATE_TOLERANCE_M, SNAPSHOT_SCHEMA_VERSION,
};
pub use station::StationRange;

/// Minimal scalar ABI probe used only to prove that the safe core can be emitted
/// as a WASM cdylib. Rich semantic calls remain on the Rust side until a binding
/// adapter is deliberately evaluated in a later stage.
#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn r1a_wasm_feasibility_probe(value: f64) -> f64 {
    value
}
