//! Versioned semantic project persistence for Street Concept Designer.
//!
//! Runtime/domain types are converted through explicit schema DTOs. The DTOs
//! contain authored engineering intent only; renderer caches and derived
//! geometry are deliberately not part of this crate's canonical document.

use std::fmt::{Display, Formatter};

use serde::Deserialize;
use serde_json::Value;
use street_concept_designer_kernel::{
    Alignment, AuthoredJunctionSnapshot, ComponentId, ComponentKind, CrossSection,
    CrossSectionComponent, CrossingRelation, CrossingType, JunctionCandidate, JunctionOptions,
    KernelError, LaneConnection, LaneConnectivityMode, LaneDirection, Movement,
    PiecewiseLinearWidthProfile, Point2, Road, RoadId, RoadNetwork, StationRange, TolerancePolicy,
    WidthKnot,
};
use street_concept_designer_project_core::{
    CoordinateContext, Project, ProjectError, ProjectId, Scenario, ScenarioId, ScenarioRole,
    TrafficSide,
};

pub mod schema {
    //! Schema-v1 persistence DTOs.
    //!
    //! Field names use one explicit external camelCase policy. These types are
    //! intentionally separate from the runtime/domain model.

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum CanonicalUnitsDocumentV1 {
        #[serde(rename = "m")]
        Metres,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum TrafficSideDocumentV1 {
        #[serde(rename = "LHT")]
        LeftHand,
        #[serde(rename = "RHT")]
        RightHand,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum ScenarioRoleDocumentV1 {
        Existing,
        Alternative,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum ComponentKindDocumentV1 {
        TrafficLane,
        Median,
        Shoulder,
        EdgeStrip,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum LaneDirectionValueDocumentV1 {
        WithAlignment,
        AgainstAlignment,
        Bidirectional,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum CrossingTypeDocumentV1 {
        TrueCrossing,
        EndpointMeeting,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum CrossingRelationDocumentV1 {
        AtGrade,
        GradeSeparated,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum LaneConnectivityModeDocumentV1 {
        Automatic,
        Manual,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum MovementDocumentV1 {
        Left,
        Through,
        Right,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct PointDocumentV1 {
        pub x: f64,
        pub y: f64,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct StationRangeDocumentV1 {
        pub start_m: f64,
        pub end_m: f64,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct WidthKnotDocumentV1 {
        pub station_m: f64,
        pub width_m: f64,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct WidthProfileDocumentV1 {
        pub station_range: StationRangeDocumentV1,
        pub knots: Vec<WidthKnotDocumentV1>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct CrossSectionComponentDocumentV1 {
        pub component_id: String,
        pub kind: ComponentKindDocumentV1,
        pub width_profile: WidthProfileDocumentV1,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct CrossSectionDocumentV1 {
        pub station_range: StationRangeDocumentV1,
        pub components: Vec<CrossSectionComponentDocumentV1>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "kind", deny_unknown_fields, rename_all = "camelCase")]
    pub enum AlignmentDocumentV1 {
        Line {
            start: PointDocumentV1,
            end: PointDocumentV1,
        },
        CircularArc {
            center: PointDocumentV1,
            radius_m: f64,
            start_angle_rad: f64,
            sweep_angle_rad: f64,
        },
        SmoothConceptualCurve {
            p0: PointDocumentV1,
            p1: PointDocumentV1,
            p2: PointDocumentV1,
            p3: PointDocumentV1,
        },
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct LaneDirectionDocumentV1 {
        pub lane_id: String,
        pub direction: LaneDirectionValueDocumentV1,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct RoadDocumentV1 {
        pub road_id: String,
        pub alignment: AlignmentDocumentV1,
        pub cross_section: CrossSectionDocumentV1,
        pub lane_directions: Vec<LaneDirectionDocumentV1>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct JunctionCandidateDocumentV1 {
        pub point: PointDocumentV1,
        pub station_a_m: f64,
        pub station_b_m: f64,
        pub crossing_type: CrossingTypeDocumentV1,
        pub relation: CrossingRelationDocumentV1,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct JunctionOptionsDocumentV1 {
        pub default_corner_radius_m: f64,
        pub corner_radii_m: Vec<f64>,
        pub auto_lane_connections: bool,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct CornerRadiusDocumentV1 {
        pub corner_id: String,
        pub radius_m: f64,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct LaneConnectionDocumentV1 {
        pub lane_connection_id: String,
        pub from_approach_id: String,
        pub from_component_id: String,
        pub to_approach_id: String,
        pub to_component_id: String,
        pub movement: MovementDocumentV1,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct JunctionDocumentV1 {
        pub junction_id: String,
        pub road_ids: Vec<String>,
        pub candidate: JunctionCandidateDocumentV1,
        pub options: JunctionOptionsDocumentV1,
        pub authored_corner_radii: Vec<CornerRadiusDocumentV1>,
        pub connectivity_mode: LaneConnectivityModeDocumentV1,
        pub manual_lane_connections: Vec<LaneConnectionDocumentV1>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct NetworkDocumentV1 {
        pub roads: Vec<RoadDocumentV1>,
        pub junction_definitions: Vec<JunctionDocumentV1>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct ScenarioDocumentV1 {
        pub scenario_id: String,
        pub name: String,
        pub role: ScenarioRoleDocumentV1,
        pub locked: bool,
        pub network: NetworkDocumentV1,
    }

    #[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct CoordinateContextDocumentV1 {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub reference_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "camelCase")]
    pub struct ProjectDocumentV1 {
        pub schema_version: u32,
        pub project_id: String,
        pub name: String,
        pub canonical_units: CanonicalUnitsDocumentV1,
        pub traffic_side: TrafficSideDocumentV1,
        pub coordinate_context: CoordinateContextDocumentV1,
        pub scenarios: Vec<ScenarioDocumentV1>,
    }
}

pub use schema::ProjectDocumentV1;

/// The only canonical document version emitted by this crate.
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

/// Persistence failures are categorized so callers can translate them without
/// depending on serde_json's private error wording.
#[derive(Debug, Clone, PartialEq)]
pub enum PersistenceError {
    JsonSyntax {
        message: String,
    },
    JsonDecode {
        message: String,
    },
    JsonEncode {
        message: String,
    },
    MissingSchemaVersion,
    InvalidSchemaVersion {
        value: String,
    },
    UnsupportedSchemaVersion {
        version: u32,
    },
    Migration {
        message: String,
    },
    MalformedPersistenceDto {
        message: String,
    },
    NonFiniteEngineeringValue {
        field: String,
    },
    Project(ProjectError),
    Kernel(KernelError),
    #[cfg(not(target_arch = "wasm32"))]
    Io {
        operation: &'static str,
        path: String,
        message: String,
    },
}

impl Display for PersistenceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JsonSyntax { message } => write!(f, "JSON syntax error: {message}"),
            Self::JsonDecode { message } => write!(f, "JSON decode error: {message}"),
            Self::JsonEncode { message } => write!(f, "JSON encode error: {message}"),
            Self::MissingSchemaVersion => write!(f, "missing schemaVersion"),
            Self::InvalidSchemaVersion { value } => {
                write!(f, "invalid schemaVersion value: {value}")
            }
            Self::UnsupportedSchemaVersion { version } => {
                write!(f, "unsupported future schemaVersion: {version}")
            }
            Self::Migration { message } => write!(f, "schema migration failed: {message}"),
            Self::MalformedPersistenceDto { message } => {
                write!(f, "malformed persistence DTO: {message}")
            }
            Self::NonFiniteEngineeringValue { field } => {
                write!(f, "non-finite engineering value: {field}")
            }
            Self::Project(error) => write!(f, "project error: {error}"),
            Self::Kernel(error) => write!(f, "kernel error: {error}"),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Io {
                operation,
                path,
                message,
            } => write!(f, "I/O {operation} failed for {path}: {message}"),
        }
    }
}

impl std::error::Error for PersistenceError {}

impl From<ProjectError> for PersistenceError {
    fn from(error: ProjectError) -> Self {
        Self::Project(error)
    }
}

impl From<KernelError> for PersistenceError {
    fn from(error: KernelError) -> Self {
        Self::Kernel(error)
    }
}

/// Encode a validated Project as compact canonical UTF-8 JSON.
pub fn encode_project_to_json(project: &Project) -> Result<String, PersistenceError> {
    let bytes = encode_project_to_bytes(project)?;
    String::from_utf8(bytes).map_err(|error| PersistenceError::JsonEncode {
        message: error.to_string(),
    })
}

/// Encode a validated Project as compact canonical UTF-8 JSON bytes.
pub fn encode_project_to_bytes(project: &Project) -> Result<Vec<u8>, PersistenceError> {
    let document = project_to_document(project)?;
    serde_json::to_vec(&document).map_err(|error| PersistenceError::JsonEncode {
        message: error.to_string(),
    })
}

/// Decode schema v1 or the explicitly supported synthetic pre-release v0
/// fixture into a validated canonical Project.
pub fn decode_project_from_json(json: &str) -> Result<Project, PersistenceError> {
    decode_project_from_bytes(json.as_bytes())
}

/// Decode schema v1 or the explicitly supported synthetic pre-release v0
/// fixture from UTF-8 JSON bytes.
pub fn decode_project_from_bytes(bytes: &[u8]) -> Result<Project, PersistenceError> {
    let document = decode_document(bytes)?;
    project_from_document(document)
}

/// Save a canonical JSON document through ordinary file I/O.
#[cfg(not(target_arch = "wasm32"))]
pub fn save_project_to_path(
    path: impl AsRef<std::path::Path>,
    project: &Project,
) -> Result<(), PersistenceError> {
    let path = path.as_ref();
    let bytes = encode_project_to_bytes(project)?;
    std::fs::write(path, bytes).map_err(|error| PersistenceError::Io {
        operation: "write",
        path: path.display().to_string(),
        message: error.to_string(),
    })
}

/// Load a canonical JSON document through ordinary file I/O.
#[cfg(not(target_arch = "wasm32"))]
pub fn load_project_from_path(
    path: impl AsRef<std::path::Path>,
) -> Result<Project, PersistenceError> {
    let path = path.as_ref();
    let bytes = std::fs::read(path).map_err(|error| PersistenceError::Io {
        operation: "read",
        path: path.display().to_string(),
        message: error.to_string(),
    })?;
    decode_project_from_bytes(&bytes)
}

fn decode_document(bytes: &[u8]) -> Result<ProjectDocumentV1, PersistenceError> {
    let value: Value =
        serde_json::from_slice(bytes).map_err(|error| PersistenceError::JsonSyntax {
            message: error.to_string(),
        })?;
    let object = value
        .as_object()
        .ok_or_else(|| PersistenceError::MalformedPersistenceDto {
            message: "top-level JSON value must be an object".to_owned(),
        })?;
    let version_value = object
        .get("schemaVersion")
        .ok_or(PersistenceError::MissingSchemaVersion)?;
    let version = version_value
        .as_u64()
        .and_then(|value| u32::try_from(value).ok())
        .ok_or_else(|| PersistenceError::InvalidSchemaVersion {
            value: version_value.to_string(),
        })?;

    match version {
        CURRENT_SCHEMA_VERSION => {
            serde_json::from_value(value).map_err(|error| PersistenceError::JsonDecode {
                message: error.to_string(),
            })
        }
        0 => {
            let historical: ProjectDocumentV0 =
                serde_json::from_value(value).map_err(|error| PersistenceError::JsonDecode {
                    message: error.to_string(),
                })?;
            migrate_v0(historical)
        }
        version if version > CURRENT_SCHEMA_VERSION => {
            Err(PersistenceError::UnsupportedSchemaVersion { version })
        }
        version => Err(PersistenceError::InvalidSchemaVersion {
            value: version.to_string(),
        }),
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct ProjectDocumentV0 {
    schema_version: u32,
    project_id: String,
    name: String,
    canonical_units: String,
    traffic_side: schema::TrafficSideDocumentV1,
    coordinate_context: schema::CoordinateContextDocumentV1,
    scenarios: Vec<schema::ScenarioDocumentV1>,
}

fn migrate_v0(historical: ProjectDocumentV0) -> Result<ProjectDocumentV1, PersistenceError> {
    // Synthetic pre-release R2 v0 fixture: the only supported difference is
    // the equivalent spelling "metres" for the v1 canonical "m" value.
    if historical.schema_version != 0 {
        return Err(PersistenceError::Migration {
            message: format!(
                "pre-release fixture has schemaVersion {}, expected 0",
                historical.schema_version
            ),
        });
    }
    if historical.canonical_units != "metres" {
        return Err(PersistenceError::Migration {
            message: "synthetic v0 canonicalUnits must be \"metres\"".to_owned(),
        });
    }
    Ok(ProjectDocumentV1 {
        schema_version: CURRENT_SCHEMA_VERSION,
        project_id: historical.project_id,
        name: historical.name,
        canonical_units: schema::CanonicalUnitsDocumentV1::Metres,
        traffic_side: historical.traffic_side,
        coordinate_context: historical.coordinate_context,
        scenarios: historical.scenarios,
    })
}

fn project_to_document(project: &Project) -> Result<ProjectDocumentV1, PersistenceError> {
    validate_project_finite(project)?;
    project
        .validate_default()
        .map_err(PersistenceError::Project)?;
    Ok(ProjectDocumentV1 {
        schema_version: CURRENT_SCHEMA_VERSION,
        project_id: project.id().as_str().to_owned(),
        name: project.name().to_owned(),
        canonical_units: schema::CanonicalUnitsDocumentV1::Metres,
        traffic_side: traffic_side_to_document(project.traffic_side()),
        coordinate_context: coordinate_context_to_document(project.coordinate_context()),
        scenarios: project
            .scenarios()
            .iter()
            .map(scenario_to_document)
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn scenario_to_document(
    scenario: &Scenario,
) -> Result<schema::ScenarioDocumentV1, PersistenceError> {
    Ok(schema::ScenarioDocumentV1 {
        scenario_id: scenario.id().as_str().to_owned(),
        name: scenario.name().to_owned(),
        role: scenario_role_to_document(scenario.role()),
        locked: scenario.is_locked(),
        network: network_to_document(scenario.network())?,
    })
}

fn network_to_document(
    network: &RoadNetwork,
) -> Result<schema::NetworkDocumentV1, PersistenceError> {
    Ok(schema::NetworkDocumentV1 {
        roads: network
            .roads()
            .iter()
            .map(road_to_document)
            .collect::<Result<Vec<_>, _>>()?,
        junction_definitions: network
            .junctions()
            .iter()
            .map(junction_to_document)
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn road_to_document(road: &Road) -> Result<schema::RoadDocumentV1, PersistenceError> {
    let lane_directions = road
        .traffic_lane_ids()
        .into_iter()
        .map(|lane_id| {
            let direction = road.lane_direction(&lane_id).ok_or_else(|| {
                PersistenceError::MalformedPersistenceDto {
                    message: format!(
                        "road {} has no direction for lane {}",
                        road.id().as_str(),
                        lane_id.as_str()
                    ),
                }
            })?;
            Ok(schema::LaneDirectionDocumentV1 {
                lane_id: lane_id.as_str().to_owned(),
                direction: lane_direction_to_document(direction),
            })
        })
        .collect::<Result<Vec<_>, PersistenceError>>()?;
    Ok(schema::RoadDocumentV1 {
        road_id: road.id().as_str().to_owned(),
        alignment: alignment_to_document(road.alignment()),
        cross_section: cross_section_to_document(road.cross_section()),
        lane_directions,
    })
}

fn alignment_to_document(alignment: &Alignment) -> schema::AlignmentDocumentV1 {
    match alignment {
        Alignment::Line(line) => schema::AlignmentDocumentV1::Line {
            start: point_to_document(line.start()),
            end: point_to_document(line.end()),
        },
        Alignment::CircularArc(arc) => schema::AlignmentDocumentV1::CircularArc {
            center: point_to_document(arc.center()),
            radius_m: arc.radius(),
            start_angle_rad: arc.start_angle(),
            sweep_angle_rad: arc.sweep_angle(),
        },
        Alignment::SmoothConceptualCurve(curve) => {
            let [p0, p1, p2, p3] = curve.control_points();
            schema::AlignmentDocumentV1::SmoothConceptualCurve {
                p0: point_to_document(p0),
                p1: point_to_document(p1),
                p2: point_to_document(p2),
                p3: point_to_document(p3),
            }
        }
    }
}

fn cross_section_to_document(cross_section: &CrossSection) -> schema::CrossSectionDocumentV1 {
    schema::CrossSectionDocumentV1 {
        station_range: station_range_to_document(cross_section.station_range()),
        components: cross_section
            .components()
            .iter()
            .map(|component| schema::CrossSectionComponentDocumentV1 {
                component_id: component.id().as_str().to_owned(),
                kind: component_kind_to_document(component.kind()),
                width_profile: width_profile_to_document(component.width_profile()),
            })
            .collect(),
    }
}

fn width_profile_to_document(
    profile: &PiecewiseLinearWidthProfile,
) -> schema::WidthProfileDocumentV1 {
    schema::WidthProfileDocumentV1 {
        station_range: station_range_to_document(profile.station_range()),
        knots: profile
            .knots()
            .iter()
            .map(|knot| schema::WidthKnotDocumentV1 {
                station_m: knot.station_m,
                width_m: knot.width_m,
            })
            .collect(),
    }
}

fn junction_to_document(
    junction: &street_concept_designer_kernel::Junction,
) -> Result<schema::JunctionDocumentV1, PersistenceError> {
    let authored = junction.authored_snapshot();
    let candidate = authored.candidate();
    let road_ids = junction
        .road_ids()
        .iter()
        .map(|road_id| road_id.as_str().to_owned())
        .collect();
    Ok(schema::JunctionDocumentV1 {
        junction_id: authored.id().as_str().to_owned(),
        road_ids,
        candidate: schema::JunctionCandidateDocumentV1 {
            point: point_to_document(candidate.point()),
            station_a_m: candidate.station_a_m(),
            station_b_m: candidate.station_b_m(),
            crossing_type: crossing_type_to_document(candidate.crossing_type()),
            relation: crossing_relation_to_document(candidate.relation()),
        },
        options: schema::JunctionOptionsDocumentV1 {
            default_corner_radius_m: authored.options().default_corner_radius_m(),
            corner_radii_m: authored.options().corner_radii_m().to_vec(),
            auto_lane_connections: authored.options().auto_lane_connections(),
        },
        authored_corner_radii: authored
            .corner_radii_m()
            .iter()
            .map(|(corner_id, radius_m)| schema::CornerRadiusDocumentV1 {
                corner_id: corner_id.as_str().to_owned(),
                radius_m: *radius_m,
            })
            .collect(),
        connectivity_mode: connectivity_mode_to_document(authored.connectivity_mode()),
        manual_lane_connections: authored
            .manual_lane_connections()
            .iter()
            .map(lane_connection_to_document)
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn lane_connection_to_document(
    connection: &LaneConnection,
) -> Result<schema::LaneConnectionDocumentV1, PersistenceError> {
    Ok(schema::LaneConnectionDocumentV1 {
        lane_connection_id: connection.id().as_str().to_owned(),
        from_approach_id: connection.from_approach_id().as_str().to_owned(),
        from_component_id: connection.from_lane_id().as_str().to_owned(),
        to_approach_id: connection.to_approach_id().as_str().to_owned(),
        to_component_id: connection.to_lane_id().as_str().to_owned(),
        movement: movement_to_document(connection.movement()),
    })
}

fn project_from_document(document: ProjectDocumentV1) -> Result<Project, PersistenceError> {
    if document.schema_version != CURRENT_SCHEMA_VERSION {
        return Err(PersistenceError::MalformedPersistenceDto {
            message: format!(
                "current DTO has schemaVersion {}, expected {}",
                document.schema_version, CURRENT_SCHEMA_VERSION
            ),
        });
    }
    validate_document_finite(&document)?;
    let project_id = ProjectId::new(document.project_id).map_err(PersistenceError::Project)?;
    let coordinate_context = coordinate_context_from_document(document.coordinate_context)?;
    let scenarios = document
        .scenarios
        .into_iter()
        .map(scenario_from_document)
        .collect::<Result<Vec<_>, _>>()?;
    Project::from_scenarios(
        project_id,
        document.name,
        traffic_side_from_document(document.traffic_side),
        coordinate_context,
        scenarios,
    )
    .map_err(PersistenceError::Project)
}

fn scenario_from_document(
    document: schema::ScenarioDocumentV1,
) -> Result<Scenario, PersistenceError> {
    let scenario_id = ScenarioId::new(document.scenario_id).map_err(PersistenceError::Project)?;
    let network = network_from_document(document.network)?;
    Scenario::new(
        scenario_id,
        document.name,
        scenario_role_from_document(document.role),
        document.locked,
        network,
    )
    .map_err(PersistenceError::Project)
}

fn network_from_document(
    document: schema::NetworkDocumentV1,
) -> Result<RoadNetwork, PersistenceError> {
    let policy = TolerancePolicy::default();
    let mut network = RoadNetwork::new();
    for road in document.roads {
        let road = road_from_document(road, &policy)?;
        network.add_road(road).map_err(PersistenceError::Kernel)?;
    }
    for junction in document.junction_definitions {
        let authored = authored_junction_from_document(junction)?;
        network
            .create_junction_from_authored(authored, &policy)
            .map_err(PersistenceError::Kernel)?;
    }
    network
        .validate(&policy)
        .map_err(PersistenceError::Kernel)?;
    Ok(network)
}

fn road_from_document(
    document: schema::RoadDocumentV1,
    policy: &TolerancePolicy,
) -> Result<Road, PersistenceError> {
    let alignment = alignment_from_document(document.alignment, policy)?;
    let cross_section = cross_section_from_document(document.cross_section, policy)?;
    if alignment.station_range() != cross_section.station_range() {
        return Err(PersistenceError::MalformedPersistenceDto {
            message: "road alignment and cross-section station ranges differ".to_owned(),
        });
    }
    let traffic_lane_ids: Vec<ComponentId> = cross_section
        .components()
        .iter()
        .filter(|component| component.kind() == ComponentKind::TrafficLane)
        .map(|component| component.id().clone())
        .collect();
    let mut directions = Vec::with_capacity(document.lane_directions.len());
    for direction in document.lane_directions {
        let lane_id = ComponentId::new(direction.lane_id).map_err(PersistenceError::Kernel)?;
        if directions.iter().any(|(id, _)| id == &lane_id) {
            return Err(PersistenceError::MalformedPersistenceDto {
                message: "duplicate lane direction id".to_owned(),
            });
        }
        if !traffic_lane_ids.iter().any(|id| id == &lane_id) {
            return Err(PersistenceError::Kernel(KernelError::MissingLane));
        }
        directions.push((lane_id, lane_direction_from_document(direction.direction)));
    }
    if directions.len() != traffic_lane_ids.len()
        || traffic_lane_ids
            .iter()
            .any(|lane_id| !directions.iter().any(|(id, _)| id == lane_id))
    {
        return Err(PersistenceError::MalformedPersistenceDto {
            message: "laneDirections must contain exactly one entry for every traffic lane"
                .to_owned(),
        });
    }
    Road::with_lane_directions(
        document.road_id,
        alignment,
        cross_section,
        directions,
        policy,
    )
    .map_err(PersistenceError::Kernel)
}

fn alignment_from_document(
    document: schema::AlignmentDocumentV1,
    policy: &TolerancePolicy,
) -> Result<Alignment, PersistenceError> {
    match document {
        schema::AlignmentDocumentV1::Line { start, end } => Alignment::line(
            point_from_document(start, "alignment line start")?,
            point_from_document(end, "alignment line end")?,
            policy,
        )
        .map_err(PersistenceError::Kernel),
        schema::AlignmentDocumentV1::CircularArc {
            center,
            radius_m,
            start_angle_rad,
            sweep_angle_rad,
        } => Alignment::circular_arc(
            point_from_document(center, "alignment arc center")?,
            finite_value(radius_m, "alignment arc radius")?,
            finite_value(start_angle_rad, "alignment arc start angle")?,
            finite_value(sweep_angle_rad, "alignment arc sweep angle")?,
            policy,
        )
        .map_err(PersistenceError::Kernel),
        schema::AlignmentDocumentV1::SmoothConceptualCurve { p0, p1, p2, p3 } => {
            Alignment::smooth_conceptual_curve(
                point_from_document(p0, "smooth curve p0")?,
                point_from_document(p1, "smooth curve p1")?,
                point_from_document(p2, "smooth curve p2")?,
                point_from_document(p3, "smooth curve p3")?,
                policy,
            )
            .map_err(PersistenceError::Kernel)
        }
    }
}

fn cross_section_from_document(
    document: schema::CrossSectionDocumentV1,
    policy: &TolerancePolicy,
) -> Result<CrossSection, PersistenceError> {
    let range = station_range_from_document(document.station_range, policy)?;
    let components = document
        .components
        .into_iter()
        .map(|component| {
            let profile = width_profile_from_document(component.width_profile, policy)?;
            if profile.station_range() != range {
                return Err(PersistenceError::Kernel(
                    KernelError::ComponentRangeMismatch,
                ));
            }
            CrossSectionComponent::new(
                component.component_id,
                component_kind_from_document(component.kind),
                profile,
            )
            .map_err(PersistenceError::Kernel)
        })
        .collect::<Result<Vec<_>, _>>()?;
    CrossSection::new(range, components, policy).map_err(PersistenceError::Kernel)
}

fn width_profile_from_document(
    document: schema::WidthProfileDocumentV1,
    policy: &TolerancePolicy,
) -> Result<PiecewiseLinearWidthProfile, PersistenceError> {
    let range = station_range_from_document(document.station_range, policy)?;
    for knot in &document.knots {
        finite_value(knot.station_m, "width knot station")?;
        finite_value(knot.width_m, "width knot width")?;
    }
    if document.knots.first().map(|knot| knot.station_m) != Some(range.start_m)
        || document.knots.last().map(|knot| knot.station_m) != Some(range.end_m)
    {
        return Err(PersistenceError::MalformedPersistenceDto {
            message: "width profile knots must start/end at the declared station range".to_owned(),
        });
    }
    PiecewiseLinearWidthProfile::new(
        range,
        document
            .knots
            .into_iter()
            .map(|knot| WidthKnot {
                station_m: knot.station_m,
                width_m: knot.width_m,
            })
            .collect(),
        policy,
    )
    .map_err(PersistenceError::Kernel)
}

fn station_range_from_document(
    document: schema::StationRangeDocumentV1,
    policy: &TolerancePolicy,
) -> Result<StationRange, PersistenceError> {
    StationRange::new(
        finite_value(document.start_m, "station range start")?,
        finite_value(document.end_m, "station range end")?,
        policy,
    )
    .map_err(PersistenceError::Kernel)
}

fn authored_junction_from_document(
    document: schema::JunctionDocumentV1,
) -> Result<AuthoredJunctionSnapshot, PersistenceError> {
    if document.road_ids.len() != 2 || document.road_ids[0] == document.road_ids[1] {
        return Err(PersistenceError::MalformedPersistenceDto {
            message: "junction roadIds must contain two distinct roads".to_owned(),
        });
    }
    if document.road_ids[0] > document.road_ids[1] {
        return Err(PersistenceError::MalformedPersistenceDto {
            message: "junction roadIds must use stable lexical order".to_owned(),
        });
    }
    let road_a = RoadId::new(document.road_ids[0].clone()).map_err(PersistenceError::Kernel)?;
    let road_b = RoadId::new(document.road_ids[1].clone()).map_err(PersistenceError::Kernel)?;
    let candidate = JunctionCandidate::from_parts(
        road_a,
        road_b,
        point_from_document(document.candidate.point, "junction candidate point")?,
        finite_value(
            document.candidate.station_a_m,
            "junction candidate station a",
        )?,
        finite_value(
            document.candidate.station_b_m,
            "junction candidate station b",
        )?,
        crossing_type_from_document(document.candidate.crossing_type),
        crossing_relation_from_document(document.candidate.relation),
    )
    .map_err(PersistenceError::Kernel)?;
    let options = JunctionOptions::new()
        .with_uniform_corner_radius(finite_value(
            document.options.default_corner_radius_m,
            "junction default corner radius",
        )?)
        .with_corner_radii(
            document
                .options
                .corner_radii_m
                .iter()
                .map(|radius_m| finite_value(*radius_m, "junction option corner radius"))
                .collect::<Result<Vec<_>, _>>()?,
        )
        .with_auto_lane_connections(document.options.auto_lane_connections);
    let corner_radii = document
        .authored_corner_radii
        .into_iter()
        .map(|corner| {
            Ok((
                street_concept_designer_kernel::CornerId::new(corner.corner_id)
                    .map_err(PersistenceError::Kernel)?,
                finite_value(corner.radius_m, "authored corner radius")?,
            ))
        })
        .collect::<Result<Vec<_>, PersistenceError>>()?;
    let manual_lane_connections = document
        .manual_lane_connections
        .into_iter()
        .map(|connection| {
            LaneConnection::new(
                connection.lane_connection_id,
                street_concept_designer_kernel::ApproachId::new(connection.from_approach_id)
                    .map_err(PersistenceError::Kernel)?,
                ComponentId::new(connection.from_component_id).map_err(PersistenceError::Kernel)?,
                street_concept_designer_kernel::ApproachId::new(connection.to_approach_id)
                    .map_err(PersistenceError::Kernel)?,
                ComponentId::new(connection.to_component_id).map_err(PersistenceError::Kernel)?,
                movement_from_document(connection.movement),
            )
            .map_err(PersistenceError::Kernel)
        })
        .collect::<Result<Vec<_>, _>>()?;
    AuthoredJunctionSnapshot::from_parts(
        street_concept_designer_kernel::JunctionId::new(document.junction_id)
            .map_err(PersistenceError::Kernel)?,
        candidate,
        options,
        corner_radii,
        connectivity_mode_from_document(document.connectivity_mode),
        manual_lane_connections,
    )
    .map_err(PersistenceError::Kernel)
}

fn coordinate_context_to_document(
    context: &CoordinateContext,
) -> schema::CoordinateContextDocumentV1 {
    schema::CoordinateContextDocumentV1 {
        reference_id: context.reference_id().map(str::to_owned),
        description: context.description().map(str::to_owned),
    }
}

fn coordinate_context_from_document(
    document: schema::CoordinateContextDocumentV1,
) -> Result<CoordinateContext, PersistenceError> {
    CoordinateContext::from_parts(document.reference_id, document.description)
        .map_err(PersistenceError::Project)
}

fn point_to_document(point: Point2) -> schema::PointDocumentV1 {
    schema::PointDocumentV1 {
        x: point.x,
        y: point.y,
    }
}

fn station_range_to_document(range: StationRange) -> schema::StationRangeDocumentV1 {
    schema::StationRangeDocumentV1 {
        start_m: range.start_m,
        end_m: range.end_m,
    }
}

fn point_from_document(
    point: schema::PointDocumentV1,
    field: &str,
) -> Result<Point2, PersistenceError> {
    Ok(Point2::new(
        finite_value(point.x, &format!("{field} x"))?,
        finite_value(point.y, &format!("{field} y"))?,
    ))
}

fn finite_value(value: f64, field: &str) -> Result<f64, PersistenceError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(PersistenceError::NonFiniteEngineeringValue {
            field: field.to_owned(),
        })
    }
}

fn validate_project_finite(project: &Project) -> Result<(), PersistenceError> {
    for scenario in project.scenarios() {
        for road in scenario.network().roads() {
            validate_alignment_finite(road.alignment())?;
            finite_value(road.station_range().start_m, "road station range start")?;
            finite_value(road.station_range().end_m, "road station range end")?;
            for component in road.cross_section().components() {
                finite_value(
                    component.width_profile().station_range().start_m,
                    "width profile station range start",
                )?;
                finite_value(
                    component.width_profile().station_range().end_m,
                    "width profile station range end",
                )?;
                for knot in component.width_profile().knots() {
                    finite_value(knot.station_m, "width knot station")?;
                    finite_value(knot.width_m, "width knot width")?;
                }
            }
        }
        for junction in scenario.network().junctions() {
            let authored = junction.authored_snapshot();
            finite_value(authored.candidate().point().x, "junction candidate point x")?;
            finite_value(authored.candidate().point().y, "junction candidate point y")?;
            finite_value(
                authored.candidate().station_a_m(),
                "junction candidate station a",
            )?;
            finite_value(
                authored.candidate().station_b_m(),
                "junction candidate station b",
            )?;
            finite_value(
                authored.options().default_corner_radius_m(),
                "junction default corner radius",
            )?;
            for radius_m in authored.options().corner_radii_m() {
                finite_value(*radius_m, "junction option corner radius")?;
            }
            for (_, radius_m) in authored.corner_radii_m() {
                finite_value(*radius_m, "authored corner radius")?;
            }
        }
    }
    Ok(())
}

fn validate_document_finite(document: &ProjectDocumentV1) -> Result<(), PersistenceError> {
    for scenario in &document.scenarios {
        for road in &scenario.network.roads {
            validate_alignment_document_finite(&road.alignment)?;
            validate_station_range_document_finite(&road.cross_section.station_range)?;
            for component in &road.cross_section.components {
                validate_width_profile_document_finite(&component.width_profile)?;
            }
        }
        for junction in &scenario.network.junction_definitions {
            finite_value(junction.candidate.point.x, "junction candidate point x")?;
            finite_value(junction.candidate.point.y, "junction candidate point y")?;
            finite_value(
                junction.candidate.station_a_m,
                "junction candidate station a",
            )?;
            finite_value(
                junction.candidate.station_b_m,
                "junction candidate station b",
            )?;
            finite_value(
                junction.options.default_corner_radius_m,
                "junction default corner radius",
            )?;
            for radius_m in &junction.options.corner_radii_m {
                finite_value(*radius_m, "junction option corner radius")?;
            }
            for corner in &junction.authored_corner_radii {
                finite_value(corner.radius_m, "authored corner radius")?;
            }
        }
    }
    Ok(())
}

fn validate_alignment_finite(alignment: &Alignment) -> Result<(), PersistenceError> {
    match alignment {
        Alignment::Line(line) => {
            validate_point_finite(line.start(), "line start")?;
            validate_point_finite(line.end(), "line end")?;
        }
        Alignment::CircularArc(arc) => {
            validate_point_finite(arc.center(), "arc center")?;
            finite_value(arc.radius(), "arc radius")?;
            finite_value(arc.start_angle(), "arc start angle")?;
            finite_value(arc.sweep_angle(), "arc sweep angle")?;
        }
        Alignment::SmoothConceptualCurve(curve) => {
            for (index, point) in curve.control_points().into_iter().enumerate() {
                validate_point_finite(point, &format!("smooth curve p{index}"))?;
            }
        }
    }
    Ok(())
}

fn validate_point_finite(point: Point2, field: &str) -> Result<(), PersistenceError> {
    finite_value(point.x, &format!("{field} x"))?;
    finite_value(point.y, &format!("{field} y"))?;
    Ok(())
}

fn validate_alignment_document_finite(
    alignment: &schema::AlignmentDocumentV1,
) -> Result<(), PersistenceError> {
    match alignment {
        schema::AlignmentDocumentV1::Line { start, end } => {
            validate_point_document_finite(start, "line start")?;
            validate_point_document_finite(end, "line end")?;
        }
        schema::AlignmentDocumentV1::CircularArc {
            center,
            radius_m,
            start_angle_rad,
            sweep_angle_rad,
        } => {
            validate_point_document_finite(center, "arc center")?;
            finite_value(*radius_m, "arc radius")?;
            finite_value(*start_angle_rad, "arc start angle")?;
            finite_value(*sweep_angle_rad, "arc sweep angle")?;
        }
        schema::AlignmentDocumentV1::SmoothConceptualCurve { p0, p1, p2, p3 } => {
            validate_point_document_finite(p0, "smooth curve p0")?;
            validate_point_document_finite(p1, "smooth curve p1")?;
            validate_point_document_finite(p2, "smooth curve p2")?;
            validate_point_document_finite(p3, "smooth curve p3")?;
        }
    }
    Ok(())
}

fn validate_point_document_finite(
    point: &schema::PointDocumentV1,
    field: &str,
) -> Result<(), PersistenceError> {
    finite_value(point.x, &format!("{field} x"))?;
    finite_value(point.y, &format!("{field} y"))?;
    Ok(())
}

fn validate_station_range_document_finite(
    range: &schema::StationRangeDocumentV1,
) -> Result<(), PersistenceError> {
    finite_value(range.start_m, "station range start")?;
    finite_value(range.end_m, "station range end")?;
    Ok(())
}

fn validate_width_profile_document_finite(
    profile: &schema::WidthProfileDocumentV1,
) -> Result<(), PersistenceError> {
    validate_station_range_document_finite(&profile.station_range)?;
    for knot in &profile.knots {
        finite_value(knot.station_m, "width knot station")?;
        finite_value(knot.width_m, "width knot width")?;
    }
    Ok(())
}

fn traffic_side_to_document(side: TrafficSide) -> schema::TrafficSideDocumentV1 {
    match side {
        TrafficSide::LeftHand => schema::TrafficSideDocumentV1::LeftHand,
        TrafficSide::RightHand => schema::TrafficSideDocumentV1::RightHand,
    }
}

fn traffic_side_from_document(side: schema::TrafficSideDocumentV1) -> TrafficSide {
    match side {
        schema::TrafficSideDocumentV1::LeftHand => TrafficSide::LeftHand,
        schema::TrafficSideDocumentV1::RightHand => TrafficSide::RightHand,
    }
}

fn scenario_role_to_document(role: ScenarioRole) -> schema::ScenarioRoleDocumentV1 {
    match role {
        ScenarioRole::Existing => schema::ScenarioRoleDocumentV1::Existing,
        ScenarioRole::Alternative => schema::ScenarioRoleDocumentV1::Alternative,
    }
}

fn scenario_role_from_document(role: schema::ScenarioRoleDocumentV1) -> ScenarioRole {
    match role {
        schema::ScenarioRoleDocumentV1::Existing => ScenarioRole::Existing,
        schema::ScenarioRoleDocumentV1::Alternative => ScenarioRole::Alternative,
    }
}

fn component_kind_to_document(kind: ComponentKind) -> schema::ComponentKindDocumentV1 {
    match kind {
        ComponentKind::TrafficLane => schema::ComponentKindDocumentV1::TrafficLane,
        ComponentKind::Median => schema::ComponentKindDocumentV1::Median,
        ComponentKind::Shoulder => schema::ComponentKindDocumentV1::Shoulder,
        ComponentKind::EdgeStrip => schema::ComponentKindDocumentV1::EdgeStrip,
    }
}

fn component_kind_from_document(kind: schema::ComponentKindDocumentV1) -> ComponentKind {
    match kind {
        schema::ComponentKindDocumentV1::TrafficLane => ComponentKind::TrafficLane,
        schema::ComponentKindDocumentV1::Median => ComponentKind::Median,
        schema::ComponentKindDocumentV1::Shoulder => ComponentKind::Shoulder,
        schema::ComponentKindDocumentV1::EdgeStrip => ComponentKind::EdgeStrip,
    }
}

fn lane_direction_to_document(direction: LaneDirection) -> schema::LaneDirectionValueDocumentV1 {
    match direction {
        LaneDirection::WithAlignment => schema::LaneDirectionValueDocumentV1::WithAlignment,
        LaneDirection::AgainstAlignment => schema::LaneDirectionValueDocumentV1::AgainstAlignment,
        LaneDirection::Bidirectional => schema::LaneDirectionValueDocumentV1::Bidirectional,
    }
}

fn lane_direction_from_document(direction: schema::LaneDirectionValueDocumentV1) -> LaneDirection {
    match direction {
        schema::LaneDirectionValueDocumentV1::WithAlignment => LaneDirection::WithAlignment,
        schema::LaneDirectionValueDocumentV1::AgainstAlignment => LaneDirection::AgainstAlignment,
        schema::LaneDirectionValueDocumentV1::Bidirectional => LaneDirection::Bidirectional,
    }
}

fn crossing_type_to_document(crossing_type: CrossingType) -> schema::CrossingTypeDocumentV1 {
    match crossing_type {
        CrossingType::TrueCrossing => schema::CrossingTypeDocumentV1::TrueCrossing,
        CrossingType::EndpointMeeting => schema::CrossingTypeDocumentV1::EndpointMeeting,
    }
}

fn crossing_type_from_document(crossing_type: schema::CrossingTypeDocumentV1) -> CrossingType {
    match crossing_type {
        schema::CrossingTypeDocumentV1::TrueCrossing => CrossingType::TrueCrossing,
        schema::CrossingTypeDocumentV1::EndpointMeeting => CrossingType::EndpointMeeting,
    }
}

fn crossing_relation_to_document(relation: CrossingRelation) -> schema::CrossingRelationDocumentV1 {
    match relation {
        CrossingRelation::AtGrade => schema::CrossingRelationDocumentV1::AtGrade,
        CrossingRelation::GradeSeparated => schema::CrossingRelationDocumentV1::GradeSeparated,
    }
}

fn crossing_relation_from_document(
    relation: schema::CrossingRelationDocumentV1,
) -> CrossingRelation {
    match relation {
        schema::CrossingRelationDocumentV1::AtGrade => CrossingRelation::AtGrade,
        schema::CrossingRelationDocumentV1::GradeSeparated => CrossingRelation::GradeSeparated,
    }
}

fn connectivity_mode_to_document(
    mode: LaneConnectivityMode,
) -> schema::LaneConnectivityModeDocumentV1 {
    match mode {
        LaneConnectivityMode::Automatic => schema::LaneConnectivityModeDocumentV1::Automatic,
        LaneConnectivityMode::Manual => schema::LaneConnectivityModeDocumentV1::Manual,
    }
}

fn connectivity_mode_from_document(
    mode: schema::LaneConnectivityModeDocumentV1,
) -> LaneConnectivityMode {
    match mode {
        schema::LaneConnectivityModeDocumentV1::Automatic => LaneConnectivityMode::Automatic,
        schema::LaneConnectivityModeDocumentV1::Manual => LaneConnectivityMode::Manual,
    }
}

fn movement_to_document(movement: Movement) -> schema::MovementDocumentV1 {
    match movement {
        Movement::Left => schema::MovementDocumentV1::Left,
        Movement::Through => schema::MovementDocumentV1::Through,
        Movement::Right => schema::MovementDocumentV1::Right,
    }
}

fn movement_from_document(movement: schema::MovementDocumentV1) -> Movement {
    match movement {
        schema::MovementDocumentV1::Left => Movement::Left,
        schema::MovementDocumentV1::Through => Movement::Through,
        schema::MovementDocumentV1::Right => Movement::Right,
    }
}
