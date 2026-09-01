//! Renderer-neutral shared derivation and diagnostic output proof for R1C.
//!
//! The [`DerivedEngineeringSnapshot`] is the only layer in this module that
//! knows how to evaluate an alignment and cross-section together.  Its owned
//! f64 project-space records are consumed by both diagnostic adapters.  The
//! adapters only translate coordinates, create primitive/buffer records, and
//! triangulate already-derived polygons.

use crate::alignment::{SamplePoint, SamplingOptions};
use crate::cross_section::{ComponentId, ComponentKind};
use crate::error::KernelError;
use crate::junction::{CornerId, JunctionId, LaneConnectionId, RoadId, RoadNetwork};
use crate::math::Point2;
use crate::policy::TolerancePolicy;

/// Schema version for the owned R1C shared snapshot contract.
pub const SNAPSHOT_SCHEMA_VERSION: u32 = 1;

/// Named diagnostic tolerance for round-tripping small local coordinates
/// through a float32 display buffer.  It is not a canonical engineering
/// tolerance and is intentionally not used to alter or validate source data.
pub const FLOAT32_LOCAL_COORDINATE_TOLERANCE_M: f64 = 1.0e-3;

/// Stable, globally unambiguous identity carried by diagnostic primitives.
///
/// A component id is scoped by its owning road.  Renderer-generated handles
/// and coordinates are deliberately absent from this type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SemanticRef {
    /// A complete road semantic object.
    Road(RoadId),
    /// One cross-section component owned by one road.
    RoadComponent {
        /// Owning road identity.
        road_id: RoadId,
        /// Component identity within that road.
        component_id: ComponentId,
    },
    /// A first-class junction semantic object.
    Junction(JunctionId),
    /// A first-class junction corner semantic object.
    Corner(CornerId),
    /// A first-class lane-to-lane movement connection.
    LaneConnection(LaneConnectionId),
}

/// Role of a renderer-neutral diagnostic primitive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveRole {
    /// Reference line sampled from a road alignment.
    RoadAlignment,
    /// Surface strip owned by one road component.
    RoadComponentSurface,
    /// Accepted R1B junction pavement surface.
    JunctionSurface,
    /// Accepted R1B corner arc diagnostic curve.
    CornerCurve,
    /// Optional guide for an accepted lane connection.
    LaneConnectionGuide,
}

/// Deterministic project-space XY extents of a shared snapshot.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Extents2 {
    min: Point2,
    max: Point2,
}

impl Extents2 {
    /// Construct finite extents from two corners.
    pub fn new(min: Point2, max: Point2) -> Result<Self, KernelError> {
        if !min.is_finite() || !max.is_finite() || min.x > max.x || min.y > max.y {
            return Err(KernelError::InvalidParameter {
                field: "render extents",
            });
        }
        Ok(Self { min, max })
    }

    /// Minimum project-space point.
    pub fn min(&self) -> Point2 {
        self.min
    }

    /// Maximum project-space point.
    pub fn max(&self) -> Point2 {
        self.max
    }

    /// Whether the extent corners are finite and ordered.
    pub fn is_finite(&self) -> bool {
        self.min.is_finite()
            && self.max.is_finite()
            && self.min.x <= self.max.x
            && self.min.y <= self.max.y
    }
}

/// Non-geometric metadata needed by a diagnostic consumer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnapshotMetadata {
    schema_version: u32,
    render_origin: Point2,
}

impl SnapshotMetadata {
    /// Snapshot schema version.
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Explicit project-space origin used for renderer-local conversion.
    pub fn render_origin(&self) -> Point2 {
        self.render_origin
    }
}

/// One shared component boundary sample in f64 project coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DerivedComponentSample {
    station_m: f64,
    width_m: f64,
    left_boundary: Point2,
    right_boundary: Point2,
}

impl DerivedComponentSample {
    /// Station of this component sample.
    pub fn station_m(&self) -> f64 {
        self.station_m
    }

    /// Evaluated component width at this station.
    pub fn width_m(&self) -> f64 {
        self.width_m
    }

    /// Left-hand component boundary in project coordinates.
    pub fn left_boundary(&self) -> Point2 {
        self.left_boundary
    }

    /// Right-hand component boundary in project coordinates.
    pub fn right_boundary(&self) -> Point2 {
        self.right_boundary
    }
}

/// One finite non-degenerate shared engineering surface strip.
///
/// A strip contains three vertices for a lifecycle interval with a zero-width
/// endpoint, or four vertices for a full-width interval.  Zero-area intervals
/// are intentionally not emitted.
#[derive(Debug, Clone, PartialEq)]
pub struct DerivedComponentStrip {
    vertices: Vec<Point2>,
}

impl DerivedComponentStrip {
    /// Counter-clockwise project-space strip vertices.
    pub fn vertices(&self) -> &[Point2] {
        &self.vertices
    }
}

/// Shared engineering geometry owned by one road component.
#[derive(Debug, Clone, PartialEq)]
pub struct DerivedComponent {
    semantic_ref: SemanticRef,
    component_id: ComponentId,
    kind: ComponentKind,
    samples: Vec<DerivedComponentSample>,
    strips: Vec<DerivedComponentStrip>,
}

impl DerivedComponent {
    /// Globally scoped semantic identity.
    pub fn semantic_ref(&self) -> &SemanticRef {
        &self.semantic_ref
    }

    /// Component id within its owning road.
    pub fn component_id(&self) -> &ComponentId {
        &self.component_id
    }

    /// Accepted semantic component kind.
    pub fn kind(&self) -> ComponentKind {
        self.kind
    }

    /// All station evaluations, including zero-width lifecycle states.
    pub fn samples(&self) -> &[DerivedComponentSample] {
        &self.samples
    }

    /// Renderable shared strips with zero-area intervals removed.
    pub fn strips(&self) -> &[DerivedComponentStrip] {
        &self.strips
    }
}

/// Shared sampled road geometry in f64 project coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct DerivedRoad {
    id: RoadId,
    alignment: Vec<SamplePoint>,
    components: Vec<DerivedComponent>,
}

impl DerivedRoad {
    /// Stable road identity.
    pub fn id(&self) -> &RoadId {
        &self.id
    }

    /// Shared alignment samples, including the accepted local frame.
    pub fn alignment(&self) -> &[SamplePoint] {
        &self.alignment
    }

    /// Components in accepted authored cross-section order.
    pub fn components(&self) -> &[DerivedComponent] {
        &self.components
    }
}

/// Shared corner curve geometry copied from an accepted R1B junction.
#[derive(Debug, Clone, PartialEq)]
pub struct DerivedCorner {
    id: CornerId,
    points: Vec<Point2>,
}

impl DerivedCorner {
    /// Stable corner identity.
    pub fn id(&self) -> &CornerId {
        &self.id
    }

    /// Accepted R1B sampled corner arc in project coordinates.
    pub fn points(&self) -> &[Point2] {
        &self.points
    }
}

/// Shared optional lane-connection guide geometry.
#[derive(Debug, Clone, PartialEq)]
pub struct DerivedLaneConnection {
    id: LaneConnectionId,
    points: Vec<Point2>,
}

impl DerivedLaneConnection {
    /// Stable lane-connection identity.
    pub fn id(&self) -> &LaneConnectionId {
        &self.id
    }

    /// Deterministic guide points in project coordinates.
    pub fn points(&self) -> &[Point2] {
        &self.points
    }
}

/// Shared current geometry of one accepted R1B junction.
#[derive(Debug, Clone, PartialEq)]
pub struct DerivedJunction {
    id: JunctionId,
    surface: Vec<Point2>,
    corners: Vec<DerivedCorner>,
    lane_connections: Vec<DerivedLaneConnection>,
}

impl DerivedJunction {
    /// Stable junction identity.
    pub fn id(&self) -> &JunctionId {
        &self.id
    }

    /// Accepted R1B pavement surface in project coordinates.
    pub fn surface(&self) -> &[Point2] {
        &self.surface
    }

    /// Accepted R1B corners in deterministic junction order.
    pub fn corners(&self) -> &[DerivedCorner] {
        &self.corners
    }

    /// Optional active lane-connection guides.
    pub fn lane_connections(&self) -> &[DerivedLaneConnection] {
        &self.lane_connections
    }
}

/// Owned renderer-facing shared engineering derivation.
///
/// Every geometry coordinate in this DTO is an f64 metre in project space.
/// Render-local coordinates are produced only by the diagnostic adapters.
#[derive(Debug, Clone, PartialEq)]
pub struct DerivedEngineeringSnapshot {
    metadata: SnapshotMetadata,
    project_extents: Extents2,
    roads: Vec<DerivedRoad>,
    junctions: Vec<DerivedJunction>,
}

impl DerivedEngineeringSnapshot {
    /// Derive one deterministic snapshot from accepted semantic kernel state.
    pub fn derive(
        network: &RoadNetwork,
        render_origin: Point2,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        policy.validate()?;
        if !render_origin.is_finite() {
            return Err(KernelError::NonFiniteInput {
                field: "render origin",
            });
        }

        let options = SamplingOptions::from_policy(policy);
        let mut roads = Vec::with_capacity(network.roads().len());
        let mut extents = ExtentAccumulator::default();
        for road in network.roads() {
            let derived = derive_road(road, options, policy, &mut extents)?;
            roads.push(derived);
        }

        let mut junctions = Vec::new();
        for junction in network.junctions() {
            // R1B deliberately clears stale geometry.  Omitting a stale
            // junction from the disposable geometry snapshot prevents old
            // surfaces from leaking into either diagnostic representation.
            let Some(surface) = junction.surface() else {
                continue;
            };
            surface.validate(policy)?;
            let surface_points = surface.vertices().to_vec();
            add_points(&mut extents, &surface_points)?;

            let corners = junction
                .corners()
                .iter()
                .map(|corner| {
                    let points = corner.arc_points().to_vec();
                    if points.len() < 2 || points.iter().any(|point| !point.is_finite()) {
                        return Err(KernelError::InvalidJunctionGeometry);
                    }
                    add_points(&mut extents, &points)?;
                    Ok(DerivedCorner {
                        id: corner.id().clone(),
                        points,
                    })
                })
                .collect::<Result<Vec<_>, KernelError>>()?;

            let lane_connections = junction
                .lane_connections()
                .iter()
                .filter_map(|connection| {
                    let from = junction.approach(connection.from_approach_id())?;
                    let to = junction.approach(connection.to_approach_id())?;
                    let points = vec![from.center(), junction.candidate().point(), to.center()];
                    if points.iter().all(|point| point.is_finite()) {
                        Some(Ok(DerivedLaneConnection {
                            id: connection.id().clone(),
                            points,
                        }))
                    } else {
                        Some(Err(KernelError::NonFiniteResult {
                            operation: "lane connection guide",
                        }))
                    }
                })
                .collect::<Result<Vec<_>, KernelError>>()?;
            for connection in &lane_connections {
                add_points(&mut extents, &connection.points)?;
            }

            junctions.push(DerivedJunction {
                id: junction.id().clone(),
                surface: surface_points,
                corners,
                lane_connections,
            });
        }

        Ok(Self {
            metadata: SnapshotMetadata {
                schema_version: SNAPSHOT_SCHEMA_VERSION,
                render_origin,
            },
            project_extents: extents.finish(),
            roads,
            junctions,
        })
    }

    /// Snapshot metadata including schema version and explicit render origin.
    pub fn metadata(&self) -> &SnapshotMetadata {
        &self.metadata
    }

    /// Explicit project-space render origin.
    pub fn render_origin(&self) -> Point2 {
        self.metadata.render_origin
    }

    /// Project-space extents; these remain independent of render origin.
    pub fn project_extents(&self) -> Extents2 {
        self.project_extents
    }

    /// Shared roads in deterministic semantic order.
    pub fn roads(&self) -> &[DerivedRoad] {
        &self.roads
    }

    /// Current non-stale accepted junction geometry.
    pub fn junctions(&self) -> &[DerivedJunction] {
        &self.junctions
    }

    /// Reuse canonical geometry with a different local-origin metadata value.
    ///
    /// This copies only disposable DTO data; it never changes source semantic
    /// state or converts canonical project coordinates to float32.
    pub fn with_render_origin(&self, render_origin: Point2) -> Result<Self, KernelError> {
        if !render_origin.is_finite() {
            return Err(KernelError::NonFiniteInput {
                field: "render origin",
            });
        }
        let mut replacement = self.clone();
        replacement.metadata.render_origin = render_origin;
        Ok(replacement)
    }

    /// Validate all shared project-space geometry and semantic ordering.
    pub fn validate(&self, policy: &TolerancePolicy) -> Result<(), KernelError> {
        policy.validate()?;
        if self.metadata.schema_version != SNAPSHOT_SCHEMA_VERSION
            || !self.metadata.render_origin.is_finite()
            || !self.project_extents.is_finite()
        {
            return Err(KernelError::InvalidParameter {
                field: "derived snapshot metadata",
            });
        }
        for road in &self.roads {
            if road.alignment.len() < 2
                || road
                    .alignment
                    .iter()
                    .any(|sample| !sample.point.is_finite())
            {
                return Err(KernelError::InvalidSurface);
            }
            for component in &road.components {
                if component.samples.len() != road.alignment.len()
                    || component.samples.iter().any(|sample| {
                        !sample.station_m.is_finite()
                            || !sample.width_m.is_finite()
                            || sample.width_m < 0.0
                            || !sample.left_boundary.is_finite()
                            || !sample.right_boundary.is_finite()
                    })
                {
                    return Err(KernelError::InvalidSurface);
                }
                for strip in &component.strips {
                    validate_polygon(&strip.vertices, policy)?;
                }
            }
        }
        for junction in &self.junctions {
            validate_polygon(&junction.surface, policy)?;
            for corner in &junction.corners {
                validate_polyline(&corner.points)?;
            }
            for connection in &junction.lane_connections {
                validate_polyline(&connection.points)?;
            }
        }
        Ok(())
    }
}

/// A diagnostic 2D polygon in render-local f64 coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct Polygon2D {
    semantic_ref: SemanticRef,
    role: PrimitiveRole,
    vertices: Vec<Point2>,
}

impl Polygon2D {
    /// Semantic owner of the polygon.
    pub fn semantic_ref(&self) -> &SemanticRef {
        &self.semantic_ref
    }

    /// Diagnostic role.
    pub fn role(&self) -> PrimitiveRole {
        self.role
    }

    /// Render-local f64 polygon vertices.
    pub fn vertices(&self) -> &[Point2] {
        &self.vertices
    }
}

/// A diagnostic 2D polyline in render-local f64 coordinates.
#[derive(Debug, Clone, PartialEq)]
pub struct Polyline2D {
    semantic_ref: SemanticRef,
    role: PrimitiveRole,
    points: Vec<Point2>,
}

impl Polyline2D {
    /// Semantic owner of the polyline.
    pub fn semantic_ref(&self) -> &SemanticRef {
        &self.semantic_ref
    }

    /// Diagnostic role.
    pub fn role(&self) -> PrimitiveRole {
        self.role
    }

    /// Render-local f64 points.
    pub fn points(&self) -> &[Point2] {
        &self.points
    }
}

/// Renderer-neutral diagnostic 2D primitive.
#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticPrimitive2D {
    /// A filled polygon or surface strip.
    Polygon(Polygon2D),
    /// A reference or guide line.
    Polyline(Polyline2D),
}

impl DiagnosticPrimitive2D {
    /// Semantic owner used for selection/highlight lookup.
    pub fn semantic_ref(&self) -> &SemanticRef {
        match self {
            Self::Polygon(polygon) => polygon.semantic_ref(),
            Self::Polyline(polyline) => polyline.semantic_ref(),
        }
    }

    /// Diagnostic role.
    pub fn role(&self) -> PrimitiveRole {
        match self {
            Self::Polygon(polygon) => polygon.role(),
            Self::Polyline(polyline) => polyline.role(),
        }
    }
}

/// Renderer-neutral diagnostic 2D scene data.
#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic2D {
    render_origin: Point2,
    primitives: Vec<DiagnosticPrimitive2D>,
}

impl Diagnostic2D {
    /// Render origin used for all local coordinates.
    pub fn render_origin(&self) -> Point2 {
        self.render_origin
    }

    /// Owned diagnostic primitives in deterministic order.
    pub fn primitives(&self) -> &[DiagnosticPrimitive2D] {
        &self.primitives
    }

    /// Whether at least one primitive carries the requested semantic id.
    pub fn contains_semantic_ref(&self, semantic_ref: &SemanticRef) -> bool {
        self.primitives
            .iter()
            .any(|primitive| primitive.semantic_ref() == semantic_ref)
    }

    /// Primitive matches for renderer-neutral selection/highlight lookup.
    pub fn matching_semantic_ref(&self, semantic_ref: &SemanticRef) -> Vec<&DiagnosticPrimitive2D> {
        self.primitives
            .iter()
            .filter(|primitive| primitive.semantic_ref() == semantic_ref)
            .collect()
    }

    /// Validate local 2D primitive data.
    pub fn validate(&self, policy: &TolerancePolicy) -> Result<(), KernelError> {
        policy.validate()?;
        if !self.render_origin.is_finite() {
            return Err(KernelError::InvalidParameter {
                field: "2D render origin",
            });
        }
        for primitive in &self.primitives {
            match primitive {
                DiagnosticPrimitive2D::Polygon(polygon) => {
                    validate_polygon(&polygon.vertices, policy)?;
                }
                DiagnosticPrimitive2D::Polyline(polyline) => {
                    validate_polyline(&polyline.points)?;
                }
            }
        }
        Ok(())
    }
}

/// Topology represented by a diagnostic 3D buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshTopology {
    /// Indices are interpreted as triangles.
    Triangles,
    /// Indices are interpreted as one connected line strip.
    LineStrip,
}

/// Renderer-neutral 3D positions and indices.
#[derive(Debug, Clone, PartialEq)]
pub struct MeshPrimitive3D {
    semantic_ref: SemanticRef,
    role: PrimitiveRole,
    topology: MeshTopology,
    positions: Vec<[f32; 3]>,
    indices: Vec<u32>,
}

impl MeshPrimitive3D {
    /// Semantic owner used for selection/highlight lookup.
    pub fn semantic_ref(&self) -> &SemanticRef {
        &self.semantic_ref
    }

    /// Diagnostic role.
    pub fn role(&self) -> PrimitiveRole {
        self.role
    }

    /// Buffer topology.
    pub fn topology(&self) -> MeshTopology {
        self.topology
    }

    /// Local float32 XYZ positions; z is flat zero for this R1C proof.
    pub fn positions(&self) -> &[[f32; 3]] {
        &self.positions
    }

    /// Buffer indices.
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    /// Validate finite positions, index ranges, winding, and triangle area.
    pub fn validate(&self) -> Result<(), KernelError> {
        if self
            .positions
            .iter()
            .any(|position| position.iter().any(|coordinate| !coordinate.is_finite()))
        {
            return Err(KernelError::NonFiniteResult {
                operation: "3D render buffer",
            });
        }
        if self
            .indices
            .iter()
            .any(|index| (*index as usize) >= self.positions.len())
        {
            return Err(KernelError::InvalidSurface);
        }
        match self.topology {
            MeshTopology::Triangles => {
                if self.positions.len() < 3
                    || self.indices.len() < 3
                    || !self.indices.len().is_multiple_of(3)
                {
                    return Err(KernelError::InvalidSurface);
                }
                for triangle in self.indices.as_chunks::<3>().0 {
                    let a = self.positions[triangle[0] as usize];
                    let b = self.positions[triangle[1] as usize];
                    let c = self.positions[triangle[2] as usize];
                    let cross = (f64::from(b[0]) - f64::from(a[0]))
                        * (f64::from(c[1]) - f64::from(a[1]))
                        - (f64::from(b[1]) - f64::from(a[1])) * (f64::from(c[0]) - f64::from(a[0]));
                    if !cross.is_finite() || cross <= 0.0 {
                        return Err(KernelError::InvalidSurface);
                    }
                }
            }
            MeshTopology::LineStrip => {
                if self.positions.len() < 2 || self.indices.len() < 2 {
                    return Err(KernelError::InvalidSurface);
                }
            }
        }
        Ok(())
    }
}

/// Renderer-neutral diagnostic 3D mesh/buffer data.
#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostic3D {
    render_origin: Point2,
    meshes: Vec<MeshPrimitive3D>,
}

impl Diagnostic3D {
    /// Render origin used before float32 conversion.
    pub fn render_origin(&self) -> Point2 {
        self.render_origin
    }

    /// Owned diagnostic mesh/buffer primitives in deterministic order.
    pub fn meshes(&self) -> &[MeshPrimitive3D] {
        &self.meshes
    }

    /// Whether at least one mesh carries the requested semantic id.
    pub fn contains_semantic_ref(&self, semantic_ref: &SemanticRef) -> bool {
        self.meshes
            .iter()
            .any(|mesh| mesh.semantic_ref() == semantic_ref)
    }

    /// Mesh matches for renderer-neutral selection/highlight lookup.
    pub fn matching_semantic_ref(&self, semantic_ref: &SemanticRef) -> Vec<&MeshPrimitive3D> {
        self.meshes
            .iter()
            .filter(|mesh| mesh.semantic_ref() == semantic_ref)
            .collect()
    }

    /// Validate all diagnostic buffers.
    pub fn validate(&self) -> Result<(), KernelError> {
        if !self.render_origin.is_finite() {
            return Err(KernelError::InvalidParameter {
                field: "3D render origin",
            });
        }
        for mesh in &self.meshes {
            mesh.validate()?;
        }
        Ok(())
    }
}

/// Derive the renderer-neutral diagnostic 2D representation.
pub fn derive_diagnostic_2d(
    snapshot: &DerivedEngineeringSnapshot,
) -> Result<Diagnostic2D, KernelError> {
    let origin = snapshot.render_origin();
    let mut primitives = Vec::new();
    for road in snapshot.roads() {
        let alignment: Vec<Point2> = road
            .alignment()
            .iter()
            .map(|sample| localize(sample.point, origin))
            .collect::<Result<_, _>>()?;
        if alignment.len() >= 2 {
            primitives.push(DiagnosticPrimitive2D::Polyline(Polyline2D {
                semantic_ref: SemanticRef::Road(road.id().clone()),
                role: PrimitiveRole::RoadAlignment,
                points: alignment,
            }));
        }
        for component in road.components() {
            for strip in component.strips() {
                let vertices = strip
                    .vertices()
                    .iter()
                    .map(|point| localize(*point, origin))
                    .collect::<Result<Vec<_>, _>>()?;
                primitives.push(DiagnosticPrimitive2D::Polygon(Polygon2D {
                    semantic_ref: component.semantic_ref().clone(),
                    role: PrimitiveRole::RoadComponentSurface,
                    vertices,
                }));
            }
        }
    }
    for junction in snapshot.junctions() {
        primitives.push(DiagnosticPrimitive2D::Polygon(Polygon2D {
            semantic_ref: SemanticRef::Junction(junction.id().clone()),
            role: PrimitiveRole::JunctionSurface,
            vertices: localize_points(junction.surface(), origin)?,
        }));
        for corner in junction.corners() {
            primitives.push(DiagnosticPrimitive2D::Polyline(Polyline2D {
                semantic_ref: SemanticRef::Corner(corner.id().clone()),
                role: PrimitiveRole::CornerCurve,
                points: localize_points(corner.points(), origin)?,
            }));
        }
        for connection in junction.lane_connections() {
            primitives.push(DiagnosticPrimitive2D::Polyline(Polyline2D {
                semantic_ref: SemanticRef::LaneConnection(connection.id().clone()),
                role: PrimitiveRole::LaneConnectionGuide,
                points: localize_points(connection.points(), origin)?,
            }));
        }
    }
    Ok(Diagnostic2D {
        render_origin: origin,
        primitives,
    })
}

/// Derive the renderer-neutral diagnostic 3D representation.
pub fn derive_diagnostic_3d(
    snapshot: &DerivedEngineeringSnapshot,
) -> Result<Diagnostic3D, KernelError> {
    let origin = snapshot.render_origin();
    let mut meshes = Vec::new();
    for road in snapshot.roads() {
        let alignment: Vec<Point2> = road
            .alignment()
            .iter()
            .map(|sample| localize(sample.point, origin))
            .collect::<Result<_, _>>()?;
        if alignment.len() >= 2 {
            meshes.push(line_mesh(
                SemanticRef::Road(road.id().clone()),
                PrimitiveRole::RoadAlignment,
                &alignment,
            )?);
        }
        for component in road.components() {
            for strip in component.strips() {
                meshes.push(polygon_mesh(
                    component.semantic_ref().clone(),
                    PrimitiveRole::RoadComponentSurface,
                    strip.vertices(),
                    origin,
                )?);
            }
        }
    }
    for junction in snapshot.junctions() {
        meshes.push(polygon_mesh(
            SemanticRef::Junction(junction.id().clone()),
            PrimitiveRole::JunctionSurface,
            junction.surface(),
            origin,
        )?);
        for corner in junction.corners() {
            meshes.push(line_mesh(
                SemanticRef::Corner(corner.id().clone()),
                PrimitiveRole::CornerCurve,
                &localize_points(corner.points(), origin)?,
            )?);
        }
        for connection in junction.lane_connections() {
            meshes.push(line_mesh(
                SemanticRef::LaneConnection(connection.id().clone()),
                PrimitiveRole::LaneConnectionGuide,
                &localize_points(connection.points(), origin)?,
            )?);
        }
    }
    let diagnostic = Diagnostic3D {
        render_origin: origin,
        meshes,
    };
    diagnostic.validate()?;
    Ok(diagnostic)
}

fn derive_road(
    road: &crate::junction::Road,
    options: SamplingOptions,
    policy: &TolerancePolicy,
    extents: &mut ExtentAccumulator,
) -> Result<DerivedRoad, KernelError> {
    let alignment = road.alignment().sample(options, policy)?;
    if alignment.len() < 2 {
        return Err(KernelError::InvalidLength {
            operation: "shared road derivation",
        });
    }
    for sample in &alignment {
        add_point(extents, sample.point)?;
    }

    let components = road.cross_section().components();
    let mut component_samples: Vec<Vec<DerivedComponentSample>> = components
        .iter()
        .map(|_| Vec::with_capacity(alignment.len()))
        .collect();
    for sample in &alignment {
        let states = road.cross_section().states_at(sample.station_m, policy)?;
        let total_width = states.iter().try_fold(0.0, |total, state| {
            let next = total + state.width_m;
            if next.is_finite() {
                Ok(next)
            } else {
                Err(KernelError::NonFiniteResult {
                    operation: "shared cross-section width",
                })
            }
        })?;
        let mut lateral = -total_width / 2.0;
        for (index, state) in states.into_iter().enumerate() {
            let left_offset = lateral;
            let right_offset = lateral + state.width_m;
            let left_boundary = sample.point + sample.normal * left_offset;
            let right_boundary = sample.point + sample.normal * right_offset;
            if !left_boundary.is_finite() || !right_boundary.is_finite() {
                return Err(KernelError::NonFiniteResult {
                    operation: "shared component boundary",
                });
            }
            component_samples[index].push(DerivedComponentSample {
                station_m: sample.station_m,
                width_m: state.width_m,
                left_boundary,
                right_boundary,
            });
            lateral = right_offset;
        }
    }

    let derived_components = components
        .iter()
        .zip(component_samples)
        .map(|(component, samples)| {
            let strips = derive_strips(&samples, policy, extents)?;
            Ok(DerivedComponent {
                semantic_ref: SemanticRef::RoadComponent {
                    road_id: road.id().clone(),
                    component_id: component.id().clone(),
                },
                component_id: component.id().clone(),
                kind: component.kind(),
                samples,
                strips,
            })
        })
        .collect::<Result<Vec<_>, KernelError>>()?;

    Ok(DerivedRoad {
        id: road.id().clone(),
        alignment,
        components: derived_components,
    })
}

fn derive_strips(
    samples: &[DerivedComponentSample],
    policy: &TolerancePolicy,
    extents: &mut ExtentAccumulator,
) -> Result<Vec<DerivedComponentStrip>, KernelError> {
    let mut strips = Vec::new();
    for pair in samples.windows(2) {
        if pair[0].width_m <= 0.0 && pair[1].width_m <= 0.0 {
            continue;
        }
        let raw = [
            pair[0].left_boundary,
            pair[1].left_boundary,
            pair[1].right_boundary,
            pair[0].right_boundary,
        ];
        let vertices = remove_consecutive_duplicates(&raw, policy.coordinate_coincidence_m);
        if vertices.len() < 3 {
            continue;
        }
        validate_polygon(&vertices, policy)?;
        add_points(extents, &vertices)?;
        strips.push(DerivedComponentStrip { vertices });
    }
    Ok(strips)
}

fn localize(project: Point2, origin: Point2) -> Result<Point2, KernelError> {
    let local = Point2::new(project.x - origin.x, project.y - origin.y);
    if project.is_finite() && origin.is_finite() && local.is_finite() {
        Ok(local)
    } else {
        Err(KernelError::NonFiniteResult {
            operation: "project-to-render-local conversion",
        })
    }
}

fn localize_points(points: &[Point2], origin: Point2) -> Result<Vec<Point2>, KernelError> {
    points
        .iter()
        .map(|point| localize(*point, origin))
        .collect()
}

fn polygon_mesh(
    semantic_ref: SemanticRef,
    role: PrimitiveRole,
    project_vertices: &[Point2],
    origin: Point2,
) -> Result<MeshPrimitive3D, KernelError> {
    let local = localize_points(project_vertices, origin)?;
    validate_local_polygon(&local)?;
    let positions = local
        .iter()
        .map(|point| [point.x as f32, point.y as f32, 0.0])
        .collect::<Vec<_>>();
    let indices = fan_indices(positions.len())?;
    let mesh = MeshPrimitive3D {
        semantic_ref,
        role,
        topology: MeshTopology::Triangles,
        positions,
        indices,
    };
    mesh.validate()?;
    Ok(mesh)
}

fn line_mesh(
    semantic_ref: SemanticRef,
    role: PrimitiveRole,
    local_points: &[Point2],
) -> Result<MeshPrimitive3D, KernelError> {
    if local_points.len() < 2 || local_points.iter().any(|point| !point.is_finite()) {
        return Err(KernelError::InvalidSurface);
    }
    let positions = local_points
        .iter()
        .map(|point| [point.x as f32, point.y as f32, 0.0])
        .collect::<Vec<_>>();
    let indices = (0..positions.len())
        .map(|index| {
            u32::try_from(index).map_err(|_| KernelError::InvalidParameter {
                field: "3D line index",
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mesh = MeshPrimitive3D {
        semantic_ref,
        role,
        topology: MeshTopology::LineStrip,
        positions,
        indices,
    };
    mesh.validate()?;
    Ok(mesh)
}

fn fan_indices(vertex_count: usize) -> Result<Vec<u32>, KernelError> {
    if vertex_count < 3 {
        return Err(KernelError::InvalidSurface);
    }
    let mut indices = Vec::with_capacity((vertex_count - 2) * 3);
    for index in 1..(vertex_count - 1) {
        indices.push(0);
        indices.push(
            u32::try_from(index).map_err(|_| KernelError::InvalidParameter {
                field: "3D triangle index",
            })?,
        );
        indices.push(
            u32::try_from(index + 1).map_err(|_| KernelError::InvalidParameter {
                field: "3D triangle index",
            })?,
        );
    }
    Ok(indices)
}

fn validate_polygon(vertices: &[Point2], policy: &TolerancePolicy) -> Result<(), KernelError> {
    if vertices.len() < 3 || vertices.iter().any(|point| !point.is_finite()) {
        return Err(KernelError::InvalidSurface);
    }
    for pair in vertices.windows(2) {
        if pair[0].distance_to(pair[1]) <= policy.coordinate_coincidence_m {
            return Err(KernelError::InvalidSurface);
        }
    }
    if vertices[0].distance_to(*vertices.last().expect("validated polygon"))
        <= policy.coordinate_coincidence_m
    {
        return Err(KernelError::InvalidSurface);
    }
    let area = signed_area(vertices)?;
    let minimum_area = policy.minimum_alignment_length_m * policy.minimum_alignment_length_m;
    if area <= minimum_area {
        return Err(KernelError::InvalidSurface);
    }
    Ok(())
}

fn validate_local_polygon(vertices: &[Point2]) -> Result<(), KernelError> {
    if vertices.len() < 3 || vertices.iter().any(|point| !point.is_finite()) {
        return Err(KernelError::InvalidSurface);
    }
    let area = signed_area(vertices)?;
    if area <= 0.0 {
        return Err(KernelError::InvalidSurface);
    }
    Ok(())
}

fn validate_polyline(points: &[Point2]) -> Result<(), KernelError> {
    if points.len() < 2 || points.iter().any(|point| !point.is_finite()) {
        return Err(KernelError::InvalidSurface);
    }
    Ok(())
}

fn signed_area(vertices: &[Point2]) -> Result<f64, KernelError> {
    let origin = vertices.first().ok_or(KernelError::InvalidSurface)?;
    let mut area = 0.0;
    for pair in vertices[1..].windows(2) {
        let first = Point2::new(pair[0].x - origin.x, pair[0].y - origin.y);
        let second = Point2::new(pair[1].x - origin.x, pair[1].y - origin.y);
        area += first.x * second.y - second.x * first.y;
    }
    let area = area / 2.0;
    if area.is_finite() {
        Ok(area)
    } else {
        Err(KernelError::NonFiniteResult {
            operation: "render polygon area",
        })
    }
}

fn remove_consecutive_duplicates(points: &[Point2], coincidence_m: f64) -> Vec<Point2> {
    let mut unique = Vec::with_capacity(points.len());
    for point in points {
        if unique
            .last()
            .is_none_or(|previous: &Point2| previous.distance_to(*point) > coincidence_m)
        {
            unique.push(*point);
        }
    }
    if unique.len() > 1
        && unique[0].distance_to(*unique.last().expect("more than one unique point"))
            <= coincidence_m
    {
        unique.pop();
    }
    unique
}

fn add_points(extents: &mut ExtentAccumulator, points: &[Point2]) -> Result<(), KernelError> {
    for point in points {
        add_point(extents, *point)?;
    }
    Ok(())
}

fn add_point(extents: &mut ExtentAccumulator, point: Point2) -> Result<(), KernelError> {
    if !point.is_finite() {
        return Err(KernelError::NonFiniteResult {
            operation: "render extents",
        });
    }
    extents.include(point);
    Ok(())
}

#[derive(Debug, Default)]
struct ExtentAccumulator {
    min: Option<Point2>,
    max: Option<Point2>,
}

impl ExtentAccumulator {
    fn include(&mut self, point: Point2) {
        self.min = Some(match self.min {
            Some(min) => Point2::new(min.x.min(point.x), min.y.min(point.y)),
            None => point,
        });
        self.max = Some(match self.max {
            Some(max) => Point2::new(max.x.max(point.x), max.y.max(point.y)),
            None => point,
        });
    }

    fn finish(self) -> Extents2 {
        Extents2 {
            min: self.min.unwrap_or(Point2::new(0.0, 0.0)),
            max: self.max.unwrap_or(Point2::new(0.0, 0.0)),
        }
    }
}
