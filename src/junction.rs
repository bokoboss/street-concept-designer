//! R1B semantic junction candidates, topology, and concept-stage geometry.
//!
//! The types in this module deliberately keep three decisions separate:
//!
//! * an XY crossing is only a [`JunctionCandidate`];
//! * topology is created only by [`RoadNetwork::create_junction`]; and
//! * pavement and movement geometry are derived from the semantic junction.
//!
//! This is a bounded concept-stage model.  Its surface is a deterministic,
//! validated convex envelope of road mouths and corner radius samples.  It is
//! not a standards-grade curb, swept-path, channelization, or pavement-Boolean
//! engine.

use std::cmp::Ordering;
use std::f64::consts::PI;

use crate::alignment::{Alignment, SamplePoint, SamplingOptions};
use crate::cross_section::{ComponentId, ComponentKind, ComponentState, CrossSection};
use crate::error::KernelError;
use crate::math::{Point2, Vector2};
use crate::policy::TolerancePolicy;

macro_rules! stable_id {
    ($name:ident, $invalid:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(String);

        impl $name {
            /// Construct a non-empty, whitespace-free stable semantic id.
            pub fn new(value: impl Into<String>) -> Result<Self, KernelError> {
                let value = value.into();
                if value.is_empty() || value.chars().any(char::is_whitespace) {
                    return Err(KernelError::$invalid);
                }
                Ok(Self(value))
            }

            /// Borrow the stable id text.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }
    };
}

stable_id!(RoadId, InvalidRoadId);
stable_id!(JunctionId, InvalidJunctionId);
stable_id!(ApproachId, InvalidApproachId);
stable_id!(CornerId, InvalidCornerId);
stable_id!(LaneConnectionId, InvalidLaneConnectionId);

/// Direction of lane travel relative to increasing reference-alignment station.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaneDirection {
    /// Travel from lower station toward higher station.
    WithAlignment,
    /// Travel from higher station toward lower station.
    AgainstAlignment,
    /// The concept-stage lane may be used in either direction.
    Bidirectional,
}

/// Whether a detected XY crossing is at grade or explicitly grade-separated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossingRelation {
    /// A crossing that may be explicitly promoted to a junction.
    AtGrade,
    /// A geometric crossing that must remain disconnected in this model.
    GradeSeparated,
}

/// Geometric classification of a candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossingType {
    /// Both road centerlines pass through the candidate location.
    TrueCrossing,
    /// At least one road reaches the candidate at an alignment endpoint.
    EndpointMeeting,
}

/// Explicit disposition applied to a candidate without creating topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CandidateDisposition {
    /// No topology decision has been made yet.
    Pending,
    /// The candidate is intentionally not connected.
    Ignore,
}

/// Which side of the station cut an approach occupies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApproachSide {
    /// The arm toward decreasing alignment station.
    Start,
    /// The arm toward increasing alignment station.
    End,
}

/// Concept-stage movement categories. U-turns are intentionally deferred.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Movement {
    /// A turn to the left of the incoming travel direction.
    Left,
    /// A continuation in approximately the incoming travel direction.
    Through,
    /// A turn to the right of the incoming travel direction.
    Right,
}

impl Movement {
    /// Stable textual form used in generated semantic ids and evidence.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Through => "through",
            Self::Right => "right",
        }
    }
}

/// Current validity state of derived junction geometry and lane connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JunctionStatus {
    /// Derived geometry and semantic connections describe current source roads.
    Fresh,
    /// Source-road edits invalidated all derived geometry and connections.
    Stale,
}

/// Result of an explicit junction regeneration request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegenerationResult {
    /// The junction was rebuilt from the current source roads.
    Regenerated,
    /// The source roads no longer produce a usable candidate.
    Stale,
}

/// A semantic road wrapper around an accepted R1A alignment and cross-section.
#[derive(Debug, Clone, PartialEq)]
pub struct Road {
    id: RoadId,
    alignment: Alignment,
    cross_section: CrossSection,
    lane_directions: Vec<(ComponentId, LaneDirection)>,
}

impl Road {
    /// Construct a road with bidirectional concept-stage traffic lanes.
    pub fn new(
        id: impl Into<String>,
        alignment: Alignment,
        cross_section: CrossSection,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        policy.validate()?;
        if alignment.station_range() != cross_section.station_range() {
            return Err(KernelError::ComponentRangeMismatch);
        }
        let lane_directions = cross_section
            .components()
            .iter()
            .filter(|component| component.kind() == ComponentKind::TrafficLane)
            .map(|component| (component.id().clone(), LaneDirection::Bidirectional))
            .collect();
        Ok(Self {
            id: RoadId::new(id)?,
            alignment,
            cross_section,
            lane_directions,
        })
    }

    /// Construct a road and set explicit travel direction for selected lanes.
    /// Unspecified traffic lanes remain bidirectional for concept-stage use.
    pub fn with_lane_directions(
        id: impl Into<String>,
        alignment: Alignment,
        cross_section: CrossSection,
        directions: Vec<(ComponentId, LaneDirection)>,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        let mut road = Self::new(id, alignment, cross_section, policy)?;
        for (lane_id, direction) in directions {
            road.set_lane_direction(&lane_id, direction)?;
        }
        Ok(road)
    }

    /// Stable road identity.
    pub fn id(&self) -> &RoadId {
        &self.id
    }

    /// Reference alignment used as the geometric source of truth.
    pub fn alignment(&self) -> &Alignment {
        &self.alignment
    }

    /// Ordered semantic cross-section source.
    pub fn cross_section(&self) -> &CrossSection {
        &self.cross_section
    }

    /// Station domain shared by the alignment and cross-section.
    pub fn station_range(&self) -> crate::station::StationRange {
        self.alignment.station_range()
    }

    /// Traffic-lane ids in authored cross-section order.
    pub fn traffic_lane_ids(&self) -> Vec<ComponentId> {
        self.cross_section
            .components()
            .iter()
            .filter(|component| component.kind() == ComponentKind::TrafficLane)
            .map(|component| component.id().clone())
            .collect()
    }

    /// Resolve the configured travel direction for a lane.
    pub fn lane_direction(&self, lane_id: &ComponentId) -> Option<LaneDirection> {
        self.lane_directions
            .iter()
            .find(|(id, _)| id == lane_id)
            .map(|(_, direction)| *direction)
    }

    /// Set a lane's semantic travel direction.
    pub fn set_lane_direction(
        &mut self,
        lane_id: &ComponentId,
        direction: LaneDirection,
    ) -> Result<(), KernelError> {
        let Some(component) = self
            .cross_section
            .components()
            .iter()
            .find(|component| component.id() == lane_id)
        else {
            return Err(KernelError::MissingLane);
        };
        if component.kind() != ComponentKind::TrafficLane {
            return Err(KernelError::IncompatibleLaneConnection);
        }
        if let Some((_, current)) = self
            .lane_directions
            .iter_mut()
            .find(|(id, _)| id == lane_id)
        {
            *current = direction;
        } else {
            self.lane_directions.push((lane_id.clone(), direction));
        }
        Ok(())
    }
}

/// A geometry-only crossing observation. Constructing one never changes a
/// [`RoadNetwork`] and never creates a topology edge.
#[derive(Debug, Clone, PartialEq)]
pub struct JunctionCandidate {
    road_a: RoadId,
    road_b: RoadId,
    point: Point2,
    station_a_m: f64,
    station_b_m: f64,
    crossing_type: CrossingType,
    relation: CrossingRelation,
    disposition: CandidateDisposition,
}

impl JunctionCandidate {
    /// Stable pair of roads in lexical id order.
    pub fn road_ids(&self) -> [RoadId; 2] {
        [self.road_a.clone(), self.road_b.clone()]
    }

    /// First road in stable lexical id order.
    pub fn road_a(&self) -> &RoadId {
        &self.road_a
    }

    /// Second road in stable lexical id order.
    pub fn road_b(&self) -> &RoadId {
        &self.road_b
    }

    /// Candidate point in engineering coordinates.
    pub fn point(&self) -> Point2 {
        self.point
    }

    /// Station on the first road.
    pub fn station_a_m(&self) -> f64 {
        self.station_a_m
    }

    /// Station on the second road.
    pub fn station_b_m(&self) -> f64 {
        self.station_b_m
    }

    /// Crossing classification.
    pub fn crossing_type(&self) -> CrossingType {
        self.crossing_type
    }

    /// Grade relationship supplied by the caller.
    pub fn relation(&self) -> CrossingRelation {
        self.relation
    }

    /// Explicit candidate disposition.
    pub fn disposition(&self) -> CandidateDisposition {
        self.disposition
    }

    /// Whether this candidate is explicitly grade-separated.
    pub fn is_grade_separated(&self) -> bool {
        self.relation == CrossingRelation::GradeSeparated
    }

    /// Return a copy explicitly marked as ignored.
    pub fn ignored(mut self) -> Self {
        self.disposition = CandidateDisposition::Ignore;
        self
    }

    /// Return a copy with an explicit disposition.
    pub fn with_disposition(mut self, disposition: CandidateDisposition) -> Self {
        self.disposition = disposition;
        self
    }

    fn same_geometry(&self, other: &Self, policy: &TolerancePolicy) -> bool {
        self.road_a == other.road_a
            && self.road_b == other.road_b
            && self.crossing_type == other.crossing_type
            && self.point.distance_to(other.point) <= policy.coordinate_coincidence_m
            && (self.station_a_m - other.station_a_m).abs() <= policy.station_bound_m
            && (self.station_b_m - other.station_b_m).abs() <= policy.station_bound_m
    }
}

/// Concept-stage junction construction options.
#[derive(Debug, Clone, PartialEq)]
pub struct JunctionOptions {
    default_corner_radius_m: f64,
    corner_radii_m: Vec<f64>,
    auto_lane_connections: bool,
}

impl Default for JunctionOptions {
    fn default() -> Self {
        Self {
            // This is a concept-stage default, not a roadway standard.
            default_corner_radius_m: 8.0,
            corner_radii_m: Vec::new(),
            auto_lane_connections: true,
        }
    }
}

impl JunctionOptions {
    /// Construct default concept-stage options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Use one radius for every corner unless an explicit list is supplied.
    pub fn with_uniform_corner_radius(mut self, radius_m: f64) -> Self {
        self.default_corner_radius_m = radius_m;
        self.corner_radii_m.clear();
        self
    }

    /// Use independent radii in deterministic counter-clockwise corner order.
    pub fn with_corner_radii(mut self, radii_m: Vec<f64>) -> Self {
        self.corner_radii_m = radii_m;
        self
    }

    /// Enable or disable deterministic automatic lane-connection proposals.
    pub fn with_auto_lane_connections(mut self, enabled: bool) -> Self {
        self.auto_lane_connections = enabled;
        self
    }

    /// Configured fallback radius.
    pub fn default_corner_radius_m(&self) -> f64 {
        self.default_corner_radius_m
    }

    /// Explicit radii, if supplied.
    pub fn corner_radii_m(&self) -> &[f64] {
        &self.corner_radii_m
    }

    /// Whether automatic lane proposals are requested.
    pub fn auto_lane_connections(&self) -> bool {
        self.auto_lane_connections
    }
}

/// One explicit approach arm of a junction.
#[derive(Debug, Clone, PartialEq)]
pub struct Approach {
    id: ApproachId,
    road_id: RoadId,
    side: ApproachSide,
    cut_station_m: f64,
    center: Point2,
    tangent: Vector2,
    outward_direction: Vector2,
    normal: Vector2,
    cross_section_state: Vec<ComponentState>,
    incoming_lane_ids: Vec<ComponentId>,
    outgoing_lane_ids: Vec<ComponentId>,
}

impl Approach {
    /// Stable approach identity.
    pub fn id(&self) -> &ApproachId {
        &self.id
    }

    /// Source road identity.
    pub fn road_id(&self) -> &RoadId {
        &self.road_id
    }

    /// Station-domain side of the cut.
    pub fn side(&self) -> ApproachSide {
        self.side
    }

    /// Alignment station used to cut the road at the junction.
    pub fn cut_station_m(&self) -> f64 {
        self.cut_station_m
    }

    /// Semantic junction center used by the derived geometry.
    pub fn center(&self) -> Point2 {
        self.center
    }

    /// Unit tangent in increasing station direction.
    pub fn tangent(&self) -> Vector2 {
        self.tangent
    }

    /// Unit vector from the junction center out along this arm.
    pub fn outward_direction(&self) -> Vector2 {
        self.outward_direction
    }

    /// Unit left-hand normal of the source alignment.
    pub fn normal(&self) -> Vector2 {
        self.normal
    }

    /// Cross-section component state actually used at the cut station.
    pub fn cross_section_state(&self) -> &[ComponentState] {
        &self.cross_section_state
    }

    /// Stable incoming traffic-lane ids for this approach.
    pub fn incoming_lane_ids(&self) -> &[ComponentId] {
        &self.incoming_lane_ids
    }

    /// Stable outgoing traffic-lane ids for this approach.
    pub fn outgoing_lane_ids(&self) -> &[ComponentId] {
        &self.outgoing_lane_ids
    }

    /// Whether a lane is accepted as incoming at this approach.
    pub fn accepts_incoming_lane(&self, lane_id: &ComponentId) -> bool {
        self.incoming_lane_ids.iter().any(|id| id == lane_id)
    }

    /// Whether a lane is accepted as outgoing at this approach.
    pub fn accepts_outgoing_lane(&self, lane_id: &ComponentId) -> bool {
        self.outgoing_lane_ids.iter().any(|id| id == lane_id)
    }
}

/// One semantic corner between two adjacent approaches.
#[derive(Debug, Clone, PartialEq)]
pub struct Corner {
    id: CornerId,
    start_approach_id: ApproachId,
    end_approach_id: ApproachId,
    radius_m: f64,
    center: Point2,
    start_direction: Vector2,
    end_direction: Vector2,
    arc_points: Vec<Point2>,
}

impl Corner {
    fn from_directions(
        id: CornerId,
        start_approach_id: ApproachId,
        end_approach_id: ApproachId,
        radius_m: f64,
        frame: CornerFrame,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        policy.validate()?;
        let CornerFrame {
            center,
            start_direction,
            end_direction,
        } = frame;
        if !center.is_finite() {
            return Err(KernelError::NonFiniteInput {
                field: "corner center",
            });
        }
        if !radius_m.is_finite() {
            return Err(KernelError::NonFiniteInput {
                field: "corner radius",
            });
        }
        if radius_m <= policy.coordinate_coincidence_m {
            return Err(KernelError::InvalidParameter {
                field: "corner radius",
            });
        }
        let start_direction = start_direction.try_normalized(policy)?;
        let end_direction = end_direction.try_normalized(policy)?;
        let delta = angle_delta(start_direction, end_direction)?;
        let full_turn = 2.0 * PI;
        if delta <= policy.angular_rad || full_turn - delta <= policy.angular_rad {
            return Err(KernelError::InvalidJunctionGeometry);
        }
        let arc_points =
            sample_corner_arc(center, radius_m, start_direction, end_direction, policy)?;
        Ok(Self {
            id,
            start_approach_id,
            end_approach_id,
            radius_m,
            center,
            start_direction,
            end_direction,
            arc_points,
        })
    }

    /// Stable corner identity.
    pub fn id(&self) -> &CornerId {
        &self.id
    }

    /// First approach in counter-clockwise derived order.
    pub fn start_approach_id(&self) -> &ApproachId {
        &self.start_approach_id
    }

    /// Second approach in counter-clockwise derived order.
    pub fn end_approach_id(&self) -> &ApproachId {
        &self.end_approach_id
    }

    /// Independent authored concept-stage radius.
    pub fn radius_m(&self) -> f64 {
        self.radius_m
    }

    /// Circle center used by the radius-style derived geometry.
    pub fn center(&self) -> Point2 {
        self.center
    }

    /// Deterministic sampled circular arc points.
    pub fn arc_points(&self) -> &[Point2] {
        &self.arc_points
    }
}

#[derive(Debug, Clone, Copy)]
struct CornerFrame {
    center: Point2,
    start_direction: Vector2,
    end_direction: Vector2,
}

/// A validated, renderer-independent derived pavement surface.
#[derive(Debug, Clone, PartialEq)]
pub struct PavementSurface {
    vertices: Vec<Point2>,
}

impl PavementSurface {
    /// Validate and normalize a simple counter-clockwise polygon.
    pub fn new(mut vertices: Vec<Point2>, policy: &TolerancePolicy) -> Result<Self, KernelError> {
        policy.validate()?;
        if vertices.len() < 3 || vertices.iter().any(|point| !point.is_finite()) {
            return Err(KernelError::InvalidSurface);
        }
        for pair in vertices.windows(2) {
            if pair[0].distance_to(pair[1]) <= policy.coordinate_coincidence_m {
                return Err(KernelError::InvalidSurface);
            }
        }
        if vertices[0].distance_to(*vertices.last().expect("three vertices"))
            <= policy.coordinate_coincidence_m
        {
            return Err(KernelError::InvalidSurface);
        }
        if !is_simple_polygon(&vertices) {
            return Err(KernelError::InvalidSurface);
        }
        let signed_area = polygon_signed_area(&vertices)?;
        let minimum_area = policy.minimum_alignment_length_m * policy.minimum_alignment_length_m;
        if !minimum_area.is_finite() || signed_area.abs() <= minimum_area {
            return Err(KernelError::InvalidSurface);
        }
        if signed_area < 0.0 {
            vertices.reverse();
        }
        Ok(Self { vertices })
    }

    /// Counter-clockwise polygon vertices in deterministic order.
    pub fn vertices(&self) -> &[Point2] {
        &self.vertices
    }

    /// Signed area; valid surfaces are positive/counter-clockwise.
    pub fn signed_area_m2(&self) -> Result<f64, KernelError> {
        polygon_signed_area(&self.vertices)
    }

    /// Positive surface area.
    pub fn area_m2(&self) -> Result<f64, KernelError> {
        Ok(self.signed_area_m2()?.abs())
    }

    /// Re-run the centralized validity checks.
    pub fn validate(&self, policy: &TolerancePolicy) -> Result<(), KernelError> {
        Self::new(self.vertices.clone(), policy).map(|_| ())
    }
}

/// One explicit semantic lane-to-lane movement connection.
#[derive(Debug, Clone, PartialEq)]
pub struct LaneConnection {
    id: LaneConnectionId,
    from_approach_id: ApproachId,
    from_lane_id: ComponentId,
    to_approach_id: ApproachId,
    to_lane_id: ComponentId,
    movement: Movement,
}

impl LaneConnection {
    /// Construct a semantic connection; junction insertion performs endpoint validation.
    pub fn new(
        id: impl Into<String>,
        from_approach_id: ApproachId,
        from_lane_id: ComponentId,
        to_approach_id: ApproachId,
        to_lane_id: ComponentId,
        movement: Movement,
    ) -> Result<Self, KernelError> {
        Ok(Self {
            id: LaneConnectionId::new(id)?,
            from_approach_id,
            from_lane_id,
            to_approach_id,
            to_lane_id,
            movement,
        })
    }

    /// Stable connection identity.
    pub fn id(&self) -> &LaneConnectionId {
        &self.id
    }

    /// Incoming approach identity.
    pub fn from_approach_id(&self) -> &ApproachId {
        &self.from_approach_id
    }

    /// Incoming lane identity.
    pub fn from_lane_id(&self) -> &ComponentId {
        &self.from_lane_id
    }

    /// Outgoing approach identity.
    pub fn to_approach_id(&self) -> &ApproachId {
        &self.to_approach_id
    }

    /// Outgoing lane identity.
    pub fn to_lane_id(&self) -> &ComponentId {
        &self.to_lane_id
    }

    /// Semantic movement category.
    pub fn movement(&self) -> Movement {
        self.movement
    }
}

/// First-class semantic junction. Surface and lane connections are derived
/// members and are absent while the junction is stale.
#[derive(Debug, Clone, PartialEq)]
pub struct Junction {
    id: JunctionId,
    road_ids: Vec<RoadId>,
    candidate: JunctionCandidate,
    approaches: Vec<Approach>,
    corners: Vec<Corner>,
    surface: Option<PavementSurface>,
    lane_connections: Vec<LaneConnection>,
    options: JunctionOptions,
    policy: TolerancePolicy,
    status: JunctionStatus,
}

impl Junction {
    /// Stable junction identity.
    pub fn id(&self) -> &JunctionId {
        &self.id
    }

    /// Connected road identities in stable lexical order.
    pub fn road_ids(&self) -> &[RoadId] {
        &self.road_ids
    }

    /// Candidate that was explicitly promoted to this junction.
    pub fn candidate(&self) -> &JunctionCandidate {
        &self.candidate
    }

    /// Explicit approaches.
    pub fn approaches(&self) -> &[Approach] {
        &self.approaches
    }

    /// Find an approach by stable id.
    pub fn approach(&self, id: &ApproachId) -> Option<&Approach> {
        self.approaches.iter().find(|approach| approach.id() == id)
    }

    /// Independent semantic corners.
    pub fn corners(&self) -> &[Corner] {
        &self.corners
    }

    /// Find a corner by stable id.
    pub fn corner(&self, id: &CornerId) -> Option<&Corner> {
        self.corners.iter().find(|corner| corner.id() == id)
    }

    /// Derived pavement surface, absent when stale.
    pub fn surface(&self) -> Option<&PavementSurface> {
        self.surface.as_ref()
    }

    /// Explicit lane connections, absent semantically as an empty list when stale.
    pub fn lane_connections(&self) -> &[LaneConnection] {
        &self.lane_connections
    }

    /// Unique movement categories in deterministic connection order.
    pub fn movement_categories(&self) -> Vec<Movement> {
        let mut movements = Vec::new();
        for connection in &self.lane_connections {
            if !movements.contains(&connection.movement) {
                movements.push(connection.movement);
            }
        }
        movements
    }

    /// Current freshness state.
    pub fn status(&self) -> JunctionStatus {
        self.status
    }

    /// Current independent corner radius values in corner order.
    pub fn corner_radii_m(&self) -> Vec<f64> {
        self.corners
            .iter()
            .map(|corner| corner.radius_m())
            .collect()
    }

    /// Replace semantic lane connections after validating every endpoint.
    pub fn replace_lane_connections(
        &mut self,
        connections: Vec<LaneConnection>,
    ) -> Result<(), KernelError> {
        if self.status != JunctionStatus::Fresh {
            return Err(KernelError::StaleJunction);
        }
        validate_lane_connections(&self.approaches, &connections, &self.policy)?;
        self.lane_connections = connections;
        Ok(())
    }

    /// Change one independent corner radius and atomically rederive the surface.
    pub fn set_corner_radius(
        &mut self,
        corner_id: &CornerId,
        radius_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<(), KernelError> {
        if self.status != JunctionStatus::Fresh {
            return Err(KernelError::StaleJunction);
        }
        let index = self
            .corners
            .iter()
            .position(|corner| corner.id() == corner_id)
            .ok_or(KernelError::InvalidCornerId)?;
        let old = &self.corners[index];
        let replacement = Corner::from_directions(
            old.id.clone(),
            old.start_approach_id.clone(),
            old.end_approach_id.clone(),
            radius_m,
            CornerFrame {
                center: old.center,
                start_direction: old.start_direction,
                end_direction: old.end_direction,
            },
            policy,
        )?;
        let mut corners = self.corners.clone();
        corners[index] = replacement;
        let surface = derive_surface(&self.approaches, &corners, policy)?;
        self.corners = corners;
        self.surface = Some(surface);
        self.policy = *policy;
        Ok(())
    }

    fn invalidate(&mut self) {
        self.status = JunctionStatus::Stale;
        self.approaches.clear();
        self.corners.clear();
        self.surface = None;
        self.lane_connections.clear();
    }
}

/// A semantic road network containing roads and explicitly created junctions.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RoadNetwork {
    roads: Vec<Road>,
    junctions: Vec<Junction>,
}

impl RoadNetwork {
    /// Construct an empty semantic network.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a road without detecting or creating any topology.
    pub fn add_road(&mut self, road: Road) -> Result<(), KernelError> {
        if self.roads.iter().any(|existing| existing.id() == road.id()) {
            return Err(KernelError::DuplicateRoadId);
        }
        let index = self
            .roads
            .binary_search_by(|existing| existing.id().cmp(road.id()))
            .unwrap_or_else(|index| index);
        self.roads.insert(index, road);
        Ok(())
    }

    /// Replace a road and explicitly invalidate all junctions that source it.
    pub fn replace_road(&mut self, road: Road) -> Result<(), KernelError> {
        let index = self
            .roads
            .iter()
            .position(|existing| existing.id() == road.id())
            .ok_or(KernelError::MissingRoad)?;
        self.roads[index] = road;
        let changed_id = self.roads[index].id().clone();
        for junction in &mut self.junctions {
            if junction
                .road_ids
                .iter()
                .any(|road_id| road_id == &changed_id)
            {
                junction.invalidate();
            }
        }
        Ok(())
    }

    /// All roads in stable lexical id order.
    pub fn roads(&self) -> &[Road] {
        &self.roads
    }

    /// Find a road by stable identity.
    pub fn road(&self, id: &RoadId) -> Option<&Road> {
        self.roads.iter().find(|road| road.id() == id)
    }

    /// All junctions in stable lexical id order.
    pub fn junctions(&self) -> &[Junction] {
        &self.junctions
    }

    /// Find a junction by stable identity.
    pub fn junction(&self, id: &JunctionId) -> Option<&Junction> {
        self.junctions.iter().find(|junction| junction.id() == id)
    }

    /// Find a junction for an explicit semantic edit such as a corner radius
    /// or manual lane-connection replacement.
    pub fn junction_mut(&mut self, id: &JunctionId) -> Option<&mut Junction> {
        self.junctions
            .iter_mut()
            .find(|junction| junction.id() == id)
    }

    /// Detect one candidate without mutating network topology.
    pub fn detect_candidate(
        &self,
        road_a: &RoadId,
        road_b: &RoadId,
        relation: CrossingRelation,
        policy: &TolerancePolicy,
    ) -> Result<Option<JunctionCandidate>, KernelError> {
        let first = self.road(road_a).ok_or(KernelError::MissingRoad)?;
        let second = self.road(road_b).ok_or(KernelError::MissingRoad)?;
        detect_candidate(first, second, relation, policy)
    }

    /// Detect every pairwise candidate without creating topology.
    pub fn detect_candidates(
        &self,
        relation: CrossingRelation,
        policy: &TolerancePolicy,
    ) -> Result<Vec<JunctionCandidate>, KernelError> {
        policy.validate()?;
        let mut candidates = Vec::new();
        for (index, first) in self.roads.iter().enumerate() {
            for second in self.roads.iter().skip(index + 1) {
                if let Some(candidate) = detect_candidate(first, second, relation, policy)? {
                    candidates.push(candidate);
                }
            }
        }
        candidates.sort_by(candidate_ordering);
        Ok(candidates)
    }

    /// Explicitly create first-class junction topology from a current candidate.
    pub fn create_junction(
        &mut self,
        candidate: &JunctionCandidate,
        id: impl Into<String>,
        options: JunctionOptions,
        policy: &TolerancePolicy,
    ) -> Result<JunctionId, KernelError> {
        policy.validate()?;
        if candidate.relation != CrossingRelation::AtGrade {
            return Err(KernelError::CandidateNotAtGrade);
        }
        if candidate.disposition == CandidateDisposition::Ignore {
            return Err(KernelError::CandidateIgnored);
        }
        let current = self
            .detect_candidate(
                &candidate.road_a,
                &candidate.road_b,
                CrossingRelation::AtGrade,
                policy,
            )?
            .ok_or(KernelError::CandidateStale)?;
        if !candidate.same_geometry(&current, policy) {
            return Err(KernelError::CandidateStale);
        }
        if self
            .junctions
            .iter()
            .any(|junction| junction.candidate.same_geometry(&current, policy))
        {
            return Err(KernelError::CandidateAlreadyUsed);
        }
        let junction_id = JunctionId::new(id)?;
        if self
            .junctions
            .iter()
            .any(|junction| junction.id() == &junction_id)
        {
            return Err(KernelError::DuplicateJunctionId);
        }
        let (road_a, road_b) = self.road_pair(&current)?;
        let junction = build_junction(
            junction_id.clone(),
            current,
            road_a,
            road_b,
            options,
            policy,
            None,
        )?;
        let index = self
            .junctions
            .binary_search_by(|existing| existing.id().cmp(&junction_id))
            .unwrap_or_else(|index| index);
        self.junctions.insert(index, junction);
        Ok(junction_id)
    }

    /// Regenerate one junction from its current source roads.
    pub fn regenerate_junction(
        &mut self,
        id: &JunctionId,
        policy: &TolerancePolicy,
    ) -> Result<RegenerationResult, KernelError> {
        policy.validate()?;
        let index = self
            .junctions
            .iter()
            .position(|junction| junction.id() == id)
            .ok_or(KernelError::MissingJunction)?;
        let (road_a_id, road_b_id, options, previous_radii) = {
            let junction = &self.junctions[index];
            (
                junction.road_ids[0].clone(),
                junction.road_ids[1].clone(),
                junction.options.clone(),
                junction
                    .corners
                    .iter()
                    .map(|corner| (corner.id.clone(), corner.radius_m))
                    .collect::<Vec<_>>(),
            )
        };
        let candidate =
            self.detect_candidate(&road_a_id, &road_b_id, CrossingRelation::AtGrade, policy)?;
        let Some(candidate) = candidate else {
            self.junctions[index].invalidate();
            return Ok(RegenerationResult::Stale);
        };
        let (road_a, road_b) = self.road_pair(&candidate)?;
        match build_junction(
            id.clone(),
            candidate,
            road_a,
            road_b,
            options,
            policy,
            Some(&previous_radii),
        ) {
            Ok(junction) => {
                self.junctions[index] = junction;
                Ok(RegenerationResult::Regenerated)
            }
            Err(error) => {
                self.junctions[index].invalidate();
                Err(error)
            }
        }
    }

    /// Stable list of roads participating in at least one explicit junction.
    pub fn connected_road_ids(&self) -> Vec<RoadId> {
        let mut ids = Vec::new();
        for junction in &self.junctions {
            for road_id in &junction.road_ids {
                if !ids.contains(road_id) {
                    ids.push(road_id.clone());
                }
            }
        }
        ids.sort();
        ids
    }

    fn road_pair<'a>(
        &'a self,
        candidate: &JunctionCandidate,
    ) -> Result<(&'a Road, &'a Road), KernelError> {
        let first = self
            .road(&candidate.road_a)
            .ok_or(KernelError::MissingRoad)?;
        let second = self
            .road(&candidate.road_b)
            .ok_or(KernelError::MissingRoad)?;
        Ok((first, second))
    }
}

/// Detect one pairwise candidate from the sampled R1A alignment geometry.
///
/// The returned observation is intentionally inert: callers must explicitly
/// pass it to [`RoadNetwork::create_junction`] to add topology.
pub fn detect_candidate(
    first: &Road,
    second: &Road,
    relation: CrossingRelation,
    policy: &TolerancePolicy,
) -> Result<Option<JunctionCandidate>, KernelError> {
    policy.validate()?;
    if first.id() == second.id() {
        return Err(KernelError::InvalidParameter {
            field: "candidate road pair",
        });
    }
    let (road_a, road_b) = if first.id() < second.id() {
        (first, second)
    } else {
        (second, first)
    };
    let samples_a = road_a
        .alignment()
        .sample(SamplingOptions::from_policy(policy), policy)?;
    let samples_b = road_b
        .alignment()
        .sample(SamplingOptions::from_policy(policy), policy)?;
    let mut candidates = Vec::new();
    for segment_a in sample_segments(&samples_a) {
        for segment_b in sample_segments(&samples_b) {
            let Some(hit) = segment_intersection(segment_a, segment_b, policy)? else {
                continue;
            };
            let station_a_m = interpolate_station(segment_a, hit.t)?;
            let station_b_m = interpolate_station(segment_b, hit.u)?;
            let at_a_endpoint = (station_a_m - road_a.station_range().start_m).abs()
                <= policy.station_bound_m
                || (station_a_m - road_a.station_range().end_m).abs() <= policy.station_bound_m;
            let at_b_endpoint = (station_b_m - road_b.station_range().start_m).abs()
                <= policy.station_bound_m
                || (station_b_m - road_b.station_range().end_m).abs() <= policy.station_bound_m;
            let crossing_type = if at_a_endpoint || at_b_endpoint {
                CrossingType::EndpointMeeting
            } else {
                CrossingType::TrueCrossing
            };
            let point = hit.point.checked("junction candidate point")?;
            let candidate = JunctionCandidate {
                road_a: road_a.id().clone(),
                road_b: road_b.id().clone(),
                point,
                station_a_m,
                station_b_m,
                crossing_type,
                relation,
                disposition: CandidateDisposition::Pending,
            };
            if !candidates
                .iter()
                .any(|existing: &JunctionCandidate| existing.same_geometry(&candidate, policy))
            {
                candidates.push(candidate);
            }
        }
    }
    candidates.sort_by(candidate_ordering);
    Ok(candidates.into_iter().next())
}

#[derive(Debug, Clone, Copy)]
struct SampleSegment {
    start: Point2,
    end: Point2,
    start_station_m: f64,
    end_station_m: f64,
}

#[derive(Debug, Clone, Copy)]
struct SegmentHit {
    t: f64,
    u: f64,
    point: Point2,
}

fn sample_segments(samples: &[SamplePoint]) -> impl Iterator<Item = SampleSegment> + '_ {
    samples.windows(2).map(|pair| SampleSegment {
        start: pair[0].point,
        end: pair[1].point,
        start_station_m: pair[0].station_m,
        end_station_m: pair[1].station_m,
    })
}

fn segment_intersection(
    first: SampleSegment,
    second: SampleSegment,
    policy: &TolerancePolicy,
) -> Result<Option<SegmentHit>, KernelError> {
    let r = first.end - first.start;
    let s = second.end - second.start;
    if !r.is_finite() || !s.is_finite() {
        return Err(KernelError::NonFiniteResult {
            operation: "junction segment vector",
        });
    }
    let r_length = r.length();
    let s_length = s.length();
    if !r_length.is_finite() || !s_length.is_finite() {
        return Err(KernelError::NonFiniteResult {
            operation: "junction segment length",
        });
    }
    if r_length <= policy.coordinate_coincidence_m || s_length <= policy.coordinate_coincidence_m {
        return endpoint_hit(first, second, policy);
    }
    let q_minus_p = second.start - first.start;
    if !q_minus_p.is_finite() {
        return Err(KernelError::NonFiniteResult {
            operation: "junction segment offset",
        });
    }
    let denominator = r.cross(s);
    if !denominator.is_finite() {
        return Err(KernelError::NonFiniteResult {
            operation: "junction segment determinant",
        });
    }
    if denominator == 0.0 {
        return endpoint_hit(first, second, policy);
    }
    let t = q_minus_p.cross(s) / denominator;
    let u = q_minus_p.cross(r) / denominator;
    if !t.is_finite() || !u.is_finite() {
        return Err(KernelError::NonFiniteResult {
            operation: "junction segment intersection parameters",
        });
    }
    let t_tolerance = policy.coordinate_coincidence_m / r_length;
    let u_tolerance = policy.coordinate_coincidence_m / s_length;
    if !t_tolerance.is_finite() || !u_tolerance.is_finite() {
        return Err(KernelError::NonFiniteResult {
            operation: "junction segment parameter tolerance",
        });
    }
    if t < -t_tolerance || t > 1.0 + t_tolerance || u < -u_tolerance || u > 1.0 + u_tolerance {
        return Ok(None);
    }
    let t = t.clamp(0.0, 1.0);
    let u = u.clamp(0.0, 1.0);
    let point = first.start + r * t;
    let point = point.checked("junction segment intersection")?;
    Ok(Some(SegmentHit { t, u, point }))
}

fn endpoint_hit(
    first: SampleSegment,
    second: SampleSegment,
    policy: &TolerancePolicy,
) -> Result<Option<SegmentHit>, KernelError> {
    let endpoints = [
        (first.start, 0.0, second.start, 0.0),
        (first.start, 0.0, second.end, 1.0),
        (first.end, 1.0, second.start, 0.0),
        (first.end, 1.0, second.end, 1.0),
    ];
    for (first_point, t, second_point, u) in endpoints {
        let distance = first_point.distance_to(second_point);
        if !distance.is_finite() {
            return Err(KernelError::NonFiniteResult {
                operation: "junction endpoint distance",
            });
        }
        if distance <= policy.coordinate_coincidence_m {
            let midpoint = Point2::new(
                (first_point.x + second_point.x) / 2.0,
                (first_point.y + second_point.y) / 2.0,
            )
            .checked("junction endpoint midpoint")?;
            return Ok(Some(SegmentHit {
                t,
                u,
                point: midpoint,
            }));
        }
    }
    Ok(None)
}

fn interpolate_station(segment: SampleSegment, fraction: f64) -> Result<f64, KernelError> {
    let station = segment
        .start_station_m
        .mul_add(1.0 - fraction, segment.end_station_m * fraction);
    if station.is_finite() {
        Ok(station)
    } else {
        Err(KernelError::NonFiniteResult {
            operation: "junction candidate station",
        })
    }
}

fn candidate_ordering(left: &JunctionCandidate, right: &JunctionCandidate) -> Ordering {
    left.station_a_m
        .total_cmp(&right.station_a_m)
        .then_with(|| left.station_b_m.total_cmp(&right.station_b_m))
        .then_with(|| left.point.x.total_cmp(&right.point.x))
        .then_with(|| left.point.y.total_cmp(&right.point.y))
}

fn build_junction(
    id: JunctionId,
    candidate: JunctionCandidate,
    road_a: &Road,
    road_b: &Road,
    options: JunctionOptions,
    policy: &TolerancePolicy,
    previous_radii: Option<&[(CornerId, f64)]>,
) -> Result<Junction, KernelError> {
    if candidate.crossing_type == CrossingType::EndpointMeeting
        && candidate_approach_count(&candidate, road_a, road_b, policy)? < 3
    {
        return Err(KernelError::InvalidJunctionGeometry);
    }
    let mut approaches = Vec::new();
    append_approaches(
        &mut approaches,
        road_a,
        candidate.station_a_m,
        candidate.point,
        policy,
    )?;
    append_approaches(
        &mut approaches,
        road_b,
        candidate.station_b_m,
        candidate.point,
        policy,
    )?;
    approaches.sort_by(|left, right| left.id.cmp(&right.id));
    if approaches.len() < 3 {
        return Err(KernelError::InvalidJunctionGeometry);
    }
    let ordered_indices = angular_approach_order(&approaches)?;
    let corner_count = ordered_indices.len();
    if previous_radii.is_none()
        && !options.corner_radii_m.is_empty()
        && options.corner_radii_m.len() != corner_count
    {
        return Err(KernelError::InvalidParameter {
            field: "corner_radii_m",
        });
    }
    let mut corners = Vec::with_capacity(corner_count);
    for (corner_index, window) in ordered_indices
        .iter()
        .copied()
        .zip(ordered_indices.iter().copied().cycle().skip(1))
        .take(corner_count)
        .enumerate()
    {
        let start = &approaches[window.0];
        let end = &approaches[window.1];
        let corner_id = make_corner_id(&id, start.id(), end.id())?;
        let radius_m = previous_radii
            .and_then(|radii| {
                radii
                    .iter()
                    .find(|(existing_id, _)| existing_id == &corner_id)
                    .map(|(_, radius)| *radius)
            })
            .or_else(|| options.corner_radii_m.get(corner_index).copied())
            .unwrap_or(options.default_corner_radius_m);
        corners.push(Corner::from_directions(
            corner_id,
            start.id.clone(),
            end.id.clone(),
            radius_m,
            CornerFrame {
                center: candidate.point,
                start_direction: start.outward_direction,
                end_direction: end.outward_direction,
            },
            policy,
        )?);
    }
    let surface = derive_surface(&approaches, &corners, policy)?;
    let lane_connections = if options.auto_lane_connections {
        propose_lane_connections(&id, &approaches, policy)?
    } else {
        Vec::new()
    };
    validate_lane_connections(&approaches, &lane_connections, policy)?;
    Ok(Junction {
        id,
        road_ids: vec![road_a.id.clone(), road_b.id.clone()],
        candidate,
        approaches,
        corners,
        surface: Some(surface),
        lane_connections,
        options,
        policy: *policy,
        status: JunctionStatus::Fresh,
    })
}

fn candidate_approach_count(
    candidate: &JunctionCandidate,
    road_a: &Road,
    road_b: &Road,
    policy: &TolerancePolicy,
) -> Result<usize, KernelError> {
    let mut count = 0;
    count += approach_count_for_station(road_a, candidate.station_a_m, policy)?;
    count += approach_count_for_station(road_b, candidate.station_b_m, policy)?;
    Ok(count)
}

fn approach_count_for_station(
    road: &Road,
    station_m: f64,
    policy: &TolerancePolicy,
) -> Result<usize, KernelError> {
    let range = road.station_range();
    let at_start = (station_m - range.start_m).abs() <= policy.station_bound_m;
    let at_end = (station_m - range.end_m).abs() <= policy.station_bound_m;
    if at_start && at_end {
        return Err(KernelError::InvalidJunctionGeometry);
    }
    Ok(if at_start || at_end { 1 } else { 2 })
}

fn append_approaches(
    approaches: &mut Vec<Approach>,
    road: &Road,
    cut_station_m: f64,
    center: Point2,
    policy: &TolerancePolicy,
) -> Result<(), KernelError> {
    let range = road.station_range();
    let at_start = (cut_station_m - range.start_m).abs() <= policy.station_bound_m;
    let at_end = (cut_station_m - range.end_m).abs() <= policy.station_bound_m;
    if at_start && at_end {
        return Err(KernelError::InvalidJunctionGeometry);
    }
    if !at_start {
        append_approach(
            approaches,
            road,
            cut_station_m,
            ApproachSide::Start,
            center,
            policy,
        )?;
    }
    if !at_end {
        append_approach(
            approaches,
            road,
            cut_station_m,
            ApproachSide::End,
            center,
            policy,
        )?;
    }
    Ok(())
}

fn append_approach(
    approaches: &mut Vec<Approach>,
    road: &Road,
    cut_station_m: f64,
    side: ApproachSide,
    center: Point2,
    policy: &TolerancePolicy,
) -> Result<(), KernelError> {
    let tangent = road.alignment().tangent_at(cut_station_m, policy)?;
    let normal = road.alignment().normal_at(cut_station_m, policy)?;
    let outward_direction = match side {
        ApproachSide::Start => tangent * -1.0,
        ApproachSide::End => tangent,
    }
    .try_normalized(policy)?;
    let cross_section_state = road.cross_section().states_at(cut_station_m, policy)?;
    let total_width = cross_section_state.iter().try_fold(0.0, |total, state| {
        let next = total + state.width_m;
        if next.is_finite() {
            Ok(next)
        } else {
            Err(KernelError::NonFiniteResult {
                operation: "junction approach width",
            })
        }
    })?;
    if total_width <= policy.coordinate_coincidence_m {
        return Err(KernelError::InvalidJunctionGeometry);
    }
    let incoming = lane_ids_for_side(road, side, &cross_section_state, true);
    let outgoing = lane_ids_for_side(road, side, &cross_section_state, false);
    let id = ApproachId::new(format!(
        "{}::{}",
        road.id().as_str(),
        match side {
            ApproachSide::Start => "start",
            ApproachSide::End => "end",
        }
    ))?;
    approaches.push(Approach {
        id,
        road_id: road.id.clone(),
        side,
        cut_station_m,
        center,
        tangent,
        outward_direction,
        normal,
        cross_section_state,
        incoming_lane_ids: incoming,
        outgoing_lane_ids: outgoing,
    });
    Ok(())
}

fn lane_ids_for_side(
    road: &Road,
    side: ApproachSide,
    states: &[ComponentState],
    incoming: bool,
) -> Vec<ComponentId> {
    states
        .iter()
        .filter(|state| state.kind == ComponentKind::TrafficLane && state.active)
        .filter_map(|state| {
            let direction = road
                .lane_direction(&state.id)
                .unwrap_or(LaneDirection::Bidirectional);
            let flows_incoming = match direction {
                LaneDirection::Bidirectional => true,
                LaneDirection::WithAlignment => matches!(side, ApproachSide::Start),
                LaneDirection::AgainstAlignment => matches!(side, ApproachSide::End),
            };
            let flows_outgoing = match direction {
                LaneDirection::Bidirectional => true,
                LaneDirection::WithAlignment => matches!(side, ApproachSide::End),
                LaneDirection::AgainstAlignment => matches!(side, ApproachSide::Start),
            };
            if (incoming && flows_incoming) || (!incoming && flows_outgoing) {
                Some(state.id.clone())
            } else {
                None
            }
        })
        .collect()
}

fn angular_approach_order(approaches: &[Approach]) -> Result<Vec<usize>, KernelError> {
    let mut indices: Vec<usize> = (0..approaches.len()).collect();
    for approach in approaches {
        if !approach.outward_direction.is_finite() {
            return Err(KernelError::NonFiniteResult {
                operation: "junction approach direction",
            });
        }
    }
    indices.sort_by(|left, right| {
        let left_angle = approaches[*left]
            .outward_direction
            .y
            .atan2(approaches[*left].outward_direction.x);
        let right_angle = approaches[*right]
            .outward_direction
            .y
            .atan2(approaches[*right].outward_direction.x);
        left_angle
            .total_cmp(&right_angle)
            .then_with(|| approaches[*left].id.cmp(&approaches[*right].id))
    });
    Ok(indices)
}

fn make_corner_id(
    junction_id: &JunctionId,
    first: &ApproachId,
    second: &ApproachId,
) -> Result<CornerId, KernelError> {
    let (first, second) = if first <= second {
        (first.as_str(), second.as_str())
    } else {
        (second.as_str(), first.as_str())
    };
    CornerId::new(format!(
        "{}::corner::{}::{}",
        junction_id.as_str(),
        first,
        second
    ))
}

fn angle_delta(first: Vector2, second: Vector2) -> Result<f64, KernelError> {
    let delta = first.cross(second).atan2(first.dot(second));
    let delta = delta.rem_euclid(2.0 * PI);
    if delta.is_finite() {
        Ok(delta)
    } else {
        Err(KernelError::NonFiniteResult {
            operation: "corner angle",
        })
    }
}

fn sample_corner_arc(
    center: Point2,
    radius_m: f64,
    start_direction: Vector2,
    end_direction: Vector2,
    _policy: &TolerancePolicy,
) -> Result<Vec<Point2>, KernelError> {
    let start_angle = start_direction.y.atan2(start_direction.x);
    let end_angle = end_direction.y.atan2(end_direction.x);
    let delta = (end_angle - start_angle).rem_euclid(2.0 * PI);
    let mut points = Vec::with_capacity(5);
    for index in 0..=4 {
        let angle = start_angle + delta * (index as f64 / 4.0);
        let offset = Vector2::new(angle.cos() * radius_m, angle.sin() * radius_m);
        let point = center + offset;
        points.push(point.checked("corner arc point")?);
    }
    Ok(points)
}

fn derive_surface(
    approaches: &[Approach],
    corners: &[Corner],
    policy: &TolerancePolicy,
) -> Result<PavementSurface, KernelError> {
    let mut points = Vec::new();
    for approach in approaches {
        let width = approach
            .cross_section_state
            .iter()
            .try_fold(0.0, |total, state| {
                let next = total + state.width_m;
                if next.is_finite() {
                    Ok(next)
                } else {
                    Err(KernelError::NonFiniteResult {
                        operation: "junction surface width",
                    })
                }
            })?;
        if width <= policy.coordinate_coincidence_m {
            return Err(KernelError::InvalidJunctionGeometry);
        }
        let half_width = width / 2.0;
        let adjacent_radius = corners
            .iter()
            .filter(|corner| {
                corner.start_approach_id == approach.id || corner.end_approach_id == approach.id
            })
            .map(|corner| corner.radius_m)
            .fold(0.0, f64::max);
        if !adjacent_radius.is_finite() {
            return Err(KernelError::NonFiniteResult {
                operation: "junction corner radius",
            });
        }
        let extension = adjacent_radius + half_width;
        if !extension.is_finite() {
            return Err(KernelError::NonFiniteResult {
                operation: "junction surface extension",
            });
        }
        let mouth_center = approach.center + approach.outward_direction * extension;
        let mouth_center = mouth_center.checked("junction surface mouth")?;
        points.push(
            (mouth_center + approach.normal * half_width).checked("junction surface left mouth")?,
        );
        points.push(
            (mouth_center - approach.normal * half_width)
                .checked("junction surface right mouth")?,
        );
    }
    for corner in corners {
        points.extend(corner.arc_points.iter().copied());
    }
    let hull = convex_hull(points, policy)?;
    PavementSurface::new(hull, policy)
}

fn convex_hull(
    mut points: Vec<Point2>,
    policy: &TolerancePolicy,
) -> Result<Vec<Point2>, KernelError> {
    if points.iter().any(|point| !point.is_finite()) {
        return Err(KernelError::InvalidSurface);
    }
    points.sort_by(|left, right| {
        left.x
            .total_cmp(&right.x)
            .then_with(|| left.y.total_cmp(&right.y))
    });
    let mut unique = Vec::new();
    for point in points {
        if unique
            .last()
            .map(|last: &Point2| last.distance_to(point) > policy.coordinate_coincidence_m)
            .unwrap_or(true)
        {
            unique.push(point);
        }
    }
    if unique.len() < 3 {
        return Err(KernelError::InvalidSurface);
    }
    let mut lower = Vec::new();
    for point in &unique {
        while lower.len() >= 2
            && orientation(lower[lower.len() - 2], lower[lower.len() - 1], *point) <= 0.0
        {
            lower.pop();
        }
        lower.push(*point);
    }
    let mut upper = Vec::new();
    for point in unique.iter().rev() {
        while upper.len() >= 2
            && orientation(upper[upper.len() - 2], upper[upper.len() - 1], *point) <= 0.0
        {
            upper.pop();
        }
        upper.push(*point);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    if lower.len() < 3 {
        return Err(KernelError::InvalidSurface);
    }
    Ok(lower)
}

fn orientation(first: Point2, second: Point2, third: Point2) -> f64 {
    (second - first).cross(third - first)
}

fn polygon_signed_area(vertices: &[Point2]) -> Result<f64, KernelError> {
    if vertices.len() < 3 {
        return Err(KernelError::InvalidSurface);
    }
    let origin = vertices[0];
    let mut double_area = 0.0;
    for index in 1..vertices.len() - 1 {
        let first = vertices[index] - origin;
        let second = vertices[index + 1] - origin;
        double_area += first.cross(second);
        if !double_area.is_finite() {
            return Err(KernelError::NonFiniteResult {
                operation: "junction polygon area",
            });
        }
    }
    Ok(double_area / 2.0)
}

fn is_simple_polygon(vertices: &[Point2]) -> bool {
    let count = vertices.len();
    for first in 0..count {
        let first_next = (first + 1) % count;
        for second in (first + 1)..count {
            let second_next = (second + 1) % count;
            if first == second
                || first_next == second
                || second_next == first
                || (first == 0 && second_next == 0)
            {
                continue;
            }
            if closed_segments_intersect(
                vertices[first],
                vertices[first_next],
                vertices[second],
                vertices[second_next],
            ) {
                return false;
            }
        }
    }
    true
}

fn closed_segments_intersect(first: Point2, second: Point2, third: Point2, fourth: Point2) -> bool {
    let first_orientation = orientation(first, second, third);
    let second_orientation = orientation(first, second, fourth);
    let third_orientation = orientation(third, fourth, first);
    let fourth_orientation = orientation(third, fourth, second);
    if !first_orientation.is_finite()
        || !second_orientation.is_finite()
        || !third_orientation.is_finite()
        || !fourth_orientation.is_finite()
    {
        return true;
    }
    let proper = ((first_orientation > 0.0 && second_orientation < 0.0)
        || (first_orientation < 0.0 && second_orientation > 0.0))
        && ((third_orientation > 0.0 && fourth_orientation < 0.0)
            || (third_orientation < 0.0 && fourth_orientation > 0.0));
    proper
        || (first_orientation == 0.0 && on_segment(first, second, third))
        || (second_orientation == 0.0 && on_segment(first, second, fourth))
        || (third_orientation == 0.0 && on_segment(third, fourth, first))
        || (fourth_orientation == 0.0 && on_segment(third, fourth, second))
}

fn on_segment(start: Point2, end: Point2, point: Point2) -> bool {
    point.x >= start.x.min(end.x)
        && point.x <= start.x.max(end.x)
        && point.y >= start.y.min(end.y)
        && point.y <= start.y.max(end.y)
}

fn propose_lane_connections(
    junction_id: &JunctionId,
    approaches: &[Approach],
    policy: &TolerancePolicy,
) -> Result<Vec<LaneConnection>, KernelError> {
    let mut connections = Vec::new();
    for from in approaches {
        for to in approaches {
            if from.id == to.id {
                continue;
            }
            let Some(movement) = classify_movement(from, to, policy)? else {
                continue;
            };
            for from_lane_id in &from.incoming_lane_ids {
                for to_lane_id in &to.outgoing_lane_ids {
                    let connection_id = LaneConnectionId::new(format!(
                        "{}::lane::{}::{}::{}::{}::{}",
                        junction_id.as_str(),
                        from.id.as_str(),
                        from_lane_id.as_str(),
                        to.id.as_str(),
                        to_lane_id.as_str(),
                        movement.as_str()
                    ))?;
                    connections.push(LaneConnection {
                        id: connection_id,
                        from_approach_id: from.id.clone(),
                        from_lane_id: from_lane_id.clone(),
                        to_approach_id: to.id.clone(),
                        to_lane_id: to_lane_id.clone(),
                        movement,
                    });
                }
            }
        }
    }
    connections.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(connections)
}

fn classify_movement(
    from: &Approach,
    to: &Approach,
    policy: &TolerancePolicy,
) -> Result<Option<Movement>, KernelError> {
    let incoming_heading = from.outward_direction * -1.0;
    let outgoing_heading = to.outward_direction;
    let angle = incoming_heading
        .cross(outgoing_heading)
        .atan2(incoming_heading.dot(outgoing_heading));
    if !angle.is_finite() {
        return Err(KernelError::NonFiniteResult {
            operation: "junction movement angle",
        });
    }
    let absolute = angle.abs();
    if absolute <= PI / 4.0 + policy.angular_rad {
        return Ok(Some(Movement::Through));
    }
    if absolute >= 3.0 * PI / 4.0 - policy.angular_rad {
        return Ok(None);
    }
    Ok(Some(if angle > 0.0 {
        Movement::Left
    } else {
        Movement::Right
    }))
}

fn validate_lane_connections(
    approaches: &[Approach],
    connections: &[LaneConnection],
    policy: &TolerancePolicy,
) -> Result<(), KernelError> {
    for (index, connection) in connections.iter().enumerate() {
        if connections[..index]
            .iter()
            .any(|previous| previous.id == connection.id)
        {
            return Err(KernelError::DuplicateLaneConnectionId);
        }
        let from = approaches
            .iter()
            .find(|approach| approach.id == connection.from_approach_id)
            .ok_or(KernelError::IncompatibleLaneConnection)?;
        let to = approaches
            .iter()
            .find(|approach| approach.id == connection.to_approach_id)
            .ok_or(KernelError::IncompatibleLaneConnection)?;
        if !from.accepts_incoming_lane(&connection.from_lane_id)
            || !to.accepts_outgoing_lane(&connection.to_lane_id)
        {
            return Err(KernelError::MissingLane);
        }
        if from.id == to.id {
            return Err(KernelError::IncompatibleLaneConnection);
        }
        let expected_movement =
            classify_movement(from, to, policy)?.ok_or(KernelError::IncompatibleLaneConnection)?;
        if expected_movement != connection.movement {
            return Err(KernelError::IncompatibleLaneConnection);
        }
    }
    Ok(())
}
