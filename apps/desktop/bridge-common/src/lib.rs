use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};

use street_concept_designer_kernel::{
    derive_diagnostic_2d, Alignment, AlignmentSegment, CrossSection, CrossSectionComponent,
    DerivedEngineeringSnapshot, DiagnosticPrimitive2D, PiecewiseLinearWidthProfile, Point2,
    PrimitiveRole, Road, SemanticRef, TolerancePolicy,
};
use street_concept_designer_project_core::{Project, ScenarioId};
use street_concept_designer_project_io::decode_project_from_bytes;
use street_concept_designer_project_session::{
    Command, ProjectSession, SessionError, Transaction, TransactionId,
};

const FIXTURE: &[u8] = include_bytes!("../../fixtures/r3b_project_v2.json");
pub const RENDER_ORIGIN: Point2 = Point2::new(1_000_000_000.0, -2_000_000_000.0);
const SCENE_MAGIC: [u8; 4] = *b"R3B2";
const SCENE_VERSION: u32 = 1;
const HEADER_BYTES: usize = 48;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeError(String);

impl BridgeError {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl Display for BridgeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for BridgeError {}

impl From<street_concept_designer_project_io::PersistenceError> for BridgeError {
    fn from(error: street_concept_designer_project_io::PersistenceError) -> Self {
        Self::new(error.to_string())
    }
}

impl From<SessionError> for BridgeError {
    fn from(error: SessionError) -> Self {
        Self::new(error.to_string())
    }
}

impl From<street_concept_designer_kernel::KernelError> for BridgeError {
    fn from(error: street_concept_designer_kernel::KernelError) -> Self {
        Self::new(error.to_string())
    }
}

impl From<street_concept_designer_project_core::ProjectError> for BridgeError {
    fn from(error: street_concept_designer_project_core::ProjectError) -> Self {
        Self::new(error.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceneSize {
    Small,
    Medium,
    Large,
}

impl SceneSize {
    pub fn parse(value: &str) -> Result<Self, BridgeError> {
        match value {
            "S" | "small" => Ok(Self::Small),
            "M" | "medium" => Ok(Self::Medium),
            "L" | "large" => Ok(Self::Large),
            _ => Err(BridgeError::new(format!("unknown scene size: {value}"))),
        }
    }

    fn copies(self) -> usize {
        match self {
            Self::Small => 1,
            Self::Medium => 8,
            Self::Large => 64,
        }
    }
}

pub struct BridgeSession {
    session: ProjectSession,
}

impl BridgeSession {
    pub fn new() -> Result<Self, BridgeError> {
        let project = decode_project_from_bytes(FIXTURE)?;
        Ok(Self {
            session: ProjectSession::new(project)?,
        })
    }

    pub fn revision(&self) -> u64 {
        self.session.revision()
    }

    pub fn scene(&self, size: SceneSize) -> Result<Vec<u8>, BridgeError> {
        encode_scene(self.session.project(), self.session.revision(), size)
    }

    pub fn preview_scene(&self, sample: u32) -> Result<Vec<u8>, BridgeError> {
        let transaction = self.preview_transaction(sample)?;
        let preview = self.session.preview(&transaction)?;
        encode_scene(
            preview.candidate_project(),
            self.session.revision(),
            SceneSize::Small,
        )
    }

    pub fn commit(&mut self, sample: u32) -> Result<u64, BridgeError> {
        let transaction = self.preview_transaction(sample)?;
        self.session.commit(&transaction)?;
        Ok(self.session.revision())
    }

    pub fn reset(&mut self) -> Result<u64, BridgeError> {
        *self = Self::new()?;
        Ok(self.session.revision())
    }

    pub fn stale_probe(&self) -> Result<(), BridgeError> {
        let transaction = self.preview_transaction(self.session.revision() as u32 + 1)?;
        match self.session.preview(&transaction) {
            Err(SessionError::StaleRevision {
                expected_revision,
                current_revision,
            }) => Err(BridgeError::new(format!(
                "controlled stale revision: expected {expected_revision}, current {current_revision}"
            ))),
            Err(error) => Err(error.into()),
            Ok(_) => Err(BridgeError::new("stale probe unexpectedly succeeded")),
        }
    }

    fn preview_transaction(&self, sample: u32) -> Result<Transaction, BridgeError> {
        let scenario_id = ScenarioId::new("scenario-preview")?;
        let road = preview_road(sample)?;
        let transaction_id = TransactionId::new(format!("r3b-preview-{sample}"))?;
        Ok(Transaction::new(
            transaction_id,
            self.session.revision(),
            "R3B preview road offset",
            vec![Command::ReplaceRoad { scenario_id, road }],
        )?)
    }
}

fn preview_road(sample: u32) -> Result<Road, BridgeError> {
    let policy = TolerancePolicy::default();
    let offset_y = f64::from(sample % 120) * 0.01;
    let p0 = Point2::new(RENDER_ORIGIN.x, RENDER_ORIGIN.y + offset_y);
    let p1 = Point2::new(RENDER_ORIGIN.x + 60.0, RENDER_ORIGIN.y + offset_y);
    let p2 = Point2::new(RENDER_ORIGIN.x + 120.0, RENDER_ORIGIN.y + offset_y);
    let alignment = Alignment::from_segments(
        vec![
            AlignmentSegment::line("segment-a", p0, p1, &policy)?,
            AlignmentSegment::line("segment-b", p1, p2, &policy)?,
        ],
        &policy,
    )?;
    let range = alignment.station_range();
    let lane_left = CrossSectionComponent::traffic_lane(
        "lane-left",
        PiecewiseLinearWidthProfile::constant(range, 3.5, &policy)?,
    )?;
    let lane_right = CrossSectionComponent::traffic_lane(
        "lane-right",
        PiecewiseLinearWidthProfile::constant(range, 3.5, &policy)?,
    )?;
    let shoulder = CrossSectionComponent::shoulder(
        "shoulder-east",
        PiecewiseLinearWidthProfile::constant(range, 1.5, &policy)?,
    )?;
    let cross_section = CrossSection::new(range, vec![lane_left, lane_right, shoulder], &policy)?;
    Ok(Road::new("R3B-road", alignment, cross_section, &policy)?)
}

struct WirePrimitive {
    kind: u8,
    role: u8,
    semantic_id: String,
    vertices: Vec<[f32; 2]>,
}

fn encode_scene(project: &Project, revision: u64, size: SceneSize) -> Result<Vec<u8>, BridgeError> {
    let scenario_id = ScenarioId::new("scenario-preview")?;
    let scenario = project
        .scenario(&scenario_id)
        .ok_or_else(|| BridgeError::new("R3B fixture is missing scenario-preview"))?;
    let snapshot = DerivedEngineeringSnapshot::derive(
        scenario.network(),
        RENDER_ORIGIN,
        &TolerancePolicy::default(),
    )?;
    snapshot.validate(&TolerancePolicy::default())?;
    let diagnostic = derive_diagnostic_2d(&snapshot)?;
    diagnostic.validate(&TolerancePolicy::default())?;

    let mut primitives = Vec::new();
    let copies = size.copies();
    for copy in 0..copies {
        let dx = copy as f64 * 250.0;
        let dy = (copy % 8) as f64 * 250.0;
        for primitive in diagnostic.primitives() {
            let semantic_base = semantic_id(&scenario_id, primitive.semantic_ref());
            let semantic_id = if copy == 0 {
                semantic_base
            } else {
                format!("{semantic_base}#copy-{copy}")
            };
            let (kind, points): (u8, Vec<Point2>) = match primitive {
                DiagnosticPrimitive2D::Polygon(polygon) => (1, polygon.vertices().to_vec()),
                DiagnosticPrimitive2D::Polyline(polyline) => (0, polyline.points().to_vec()),
            };
            let vertices = points
                .into_iter()
                .map(|point| {
                    // R1C has already localized in f64. Only this wire step
                    // converts the local coordinates to GPU-friendly f32.
                    let x = (point.x + dx) as f32;
                    let y = (point.y + dy) as f32;
                    if !x.is_finite() || !y.is_finite() {
                        return Err(BridgeError::new("non-finite local scene coordinate"));
                    }
                    Ok([x, y])
                })
                .collect::<Result<Vec<_>, BridgeError>>()?;
            primitives.push(WirePrimitive {
                kind,
                role: role_code(primitive.role()),
                semantic_id,
                vertices,
            });
        }
    }

    let semantic_count = primitives
        .iter()
        .map(|primitive| primitive.semantic_id.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let coordinate_count = primitives
        .iter()
        .map(|primitive| primitive.vertices.len() * 2)
        .sum::<usize>();
    let mut bytes = Vec::with_capacity(HEADER_BYTES + coordinate_count * 4);
    bytes.extend_from_slice(&SCENE_MAGIC);
    bytes.extend_from_slice(&SCENE_VERSION.to_le_bytes());
    bytes.extend_from_slice(&revision.to_le_bytes());
    bytes.extend_from_slice(&diagnostic.render_origin().x.to_le_bytes());
    bytes.extend_from_slice(&diagnostic.render_origin().y.to_le_bytes());
    bytes.extend_from_slice(&(primitives.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&(semantic_count as u32).to_le_bytes());
    bytes.extend_from_slice(&(coordinate_count as u32).to_le_bytes());
    bytes.extend_from_slice(&0u32.to_le_bytes());

    for primitive in primitives {
        bytes.push(primitive.kind);
        bytes.push(primitive.role);
        bytes.extend_from_slice(&(primitive.semantic_id.len() as u16).to_le_bytes());
        bytes.extend_from_slice(&(primitive.vertices.len() as u32).to_le_bytes());
        bytes.extend_from_slice(primitive.semantic_id.as_bytes());
        for [x, y] in primitive.vertices {
            bytes.extend_from_slice(&x.to_le_bytes());
            bytes.extend_from_slice(&y.to_le_bytes());
        }
    }
    let payload_bytes = bytes.len() - HEADER_BYTES;
    bytes[44..48].copy_from_slice(&(payload_bytes as u32).to_le_bytes());
    Ok(bytes)
}

fn role_code(role: PrimitiveRole) -> u8 {
    match role {
        PrimitiveRole::RoadAlignment => 0,
        PrimitiveRole::RoadComponentSurface => 1,
        PrimitiveRole::JunctionSurface => 2,
        PrimitiveRole::CornerCurve => 3,
    }
}

fn semantic_id(scenario_id: &ScenarioId, semantic_ref: &SemanticRef) -> String {
    let scope = scenario_id.as_str();
    match semantic_ref {
        SemanticRef::Road(road_id) => format!("{scope}/road/{}", road_id.as_str()),
        SemanticRef::RoadComponent {
            road_id,
            component_id,
        } => format!(
            "{scope}/road/{}/component/{}",
            road_id.as_str(),
            component_id.as_str()
        ),
        SemanticRef::Junction(junction_id) => {
            format!("{scope}/junction/{}", junction_id.as_str())
        }
        SemanticRef::Corner(corner_id) => format!("{scope}/corner/{}", corner_id.as_str()),
        SemanticRef::LaneConnection {
            junction_id,
            connection_id,
        } => format!(
            "{scope}/junction/{}/connection/{}",
            junction_id.as_str(),
            connection_id.as_str()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_uses_current_composite_schema_and_local_scene_coordinates() {
        let session = BridgeSession::new().expect("fixture session");
        let scene = session.scene(SceneSize::Small).expect("scene");
        assert_eq!(&scene[..4], b"R3B2");
        assert_eq!(u32::from_le_bytes(scene[4..8].try_into().unwrap()), 1);
        assert_eq!(
            f64::from_le_bytes(scene[16..24].try_into().unwrap()),
            RENDER_ORIGIN.x
        );
        let lane_id = b"scenario-preview/road/R3B-road/component/lane-left";
        assert!(scene.windows(lane_id.len()).any(|window| window == lane_id));
        assert!(scene.len() > HEADER_BYTES);
        assert_eq!(session.revision(), 0);
    }

    #[test]
    fn preview_is_real_session_work_and_stale_errors_are_controlled() {
        let mut session = BridgeSession::new().expect("fixture session");
        let before = session.revision();
        let preview = session.preview_scene(60).expect("preview");
        assert!(preview.len() > HEADER_BYTES);
        assert_eq!(session.revision(), before);
        assert!(session.stale_probe().is_err());
        assert_eq!(session.commit(60).expect("commit"), 1);
        assert_eq!(session.reset().expect("reset"), 0);
    }
}
