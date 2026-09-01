use std::fmt::{Display, Formatter};

/// Errors returned when semantic or geometric input cannot be accepted safely.
#[derive(Debug, Clone, PartialEq)]
pub enum KernelError {
    /// An input value was not finite.
    NonFiniteInput { field: &'static str },
    /// A derived value became non-finite.
    NonFiniteResult { operation: &'static str },
    /// A parameter or tolerance value is outside the supported domain.
    InvalidParameter { field: &'static str },
    /// A primitive or station range is too short to be meaningful.
    InvalidLength { operation: &'static str },
    /// A station is outside the bounded alignment/profile range.
    StationOutOfBounds { station: f64, start: f64, end: f64 },
    /// The requested sampling budget cannot satisfy the requested criteria.
    SamplingLimitExceeded,
    /// Projection is not defined for the supplied geometry/query pair.
    ProjectionUndefined,
    /// A width profile has no usable ordered knots.
    InvalidWidthProfile,
    /// A component id is empty or otherwise not usable as a stable identity.
    InvalidComponentId,
    /// Two components have the same stable semantic id.
    DuplicateComponentId,
    /// A component does not share the cross-section station range.
    ComponentRangeMismatch,
    /// A road id is empty or otherwise not usable as a stable identity.
    InvalidRoadId,
    /// Two roads have the same stable semantic id.
    DuplicateRoadId,
    /// A referenced road does not exist in the network.
    MissingRoad,
    /// A junction id is empty or otherwise not usable as a stable identity.
    InvalidJunctionId,
    /// Two junctions have the same stable semantic id.
    DuplicateJunctionId,
    /// A referenced junction does not exist in the network.
    MissingJunction,
    /// An approach id is empty or otherwise not usable as a stable identity.
    InvalidApproachId,
    /// A corner id is empty or otherwise not usable as a stable identity.
    InvalidCornerId,
    /// A lane connection id is empty or otherwise not usable as a stable identity.
    InvalidLaneConnectionId,
    /// Two lane connections have the same stable semantic id.
    DuplicateLaneConnectionId,
    /// A referenced traffic lane does not exist or is inactive at the junction cut.
    MissingLane,
    /// A lane connection references a non-traffic or directionally incompatible lane.
    IncompatibleLaneConnection,
    /// A candidate cannot be used to create an at-grade junction.
    CandidateNotAtGrade,
    /// A candidate was explicitly ignored and cannot create topology.
    CandidateIgnored,
    /// A candidate no longer describes the current road geometry.
    CandidateStale,
    /// A detected candidate is already represented by a junction.
    CandidateAlreadyUsed,
    /// A junction's derived surface or approach geometry is not valid concept geometry.
    InvalidJunctionGeometry,
    /// A derived polygon is not finite, non-degenerate, or simple.
    InvalidSurface,
    /// A stale junction cannot accept derived semantic edits until regenerated.
    StaleJunction,
}

impl Display for KernelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFiniteInput { field } => write!(f, "non-finite input: {field}"),
            Self::NonFiniteResult { operation } => write!(f, "non-finite result: {operation}"),
            Self::InvalidParameter { field } => write!(f, "invalid parameter: {field}"),
            Self::InvalidLength { operation } => write!(f, "invalid length: {operation}"),
            Self::StationOutOfBounds {
                station,
                start,
                end,
            } => write!(f, "station {station} outside [{start}, {end}]"),
            Self::SamplingLimitExceeded => write!(f, "sampling limit exceeded"),
            Self::ProjectionUndefined => write!(f, "projection is undefined"),
            Self::InvalidWidthProfile => write!(f, "invalid width profile"),
            Self::InvalidComponentId => write!(f, "invalid component id"),
            Self::DuplicateComponentId => write!(f, "duplicate component id"),
            Self::ComponentRangeMismatch => write!(f, "component station range mismatch"),
            Self::InvalidRoadId => write!(f, "invalid road id"),
            Self::DuplicateRoadId => write!(f, "duplicate road id"),
            Self::MissingRoad => write!(f, "missing road"),
            Self::InvalidJunctionId => write!(f, "invalid junction id"),
            Self::DuplicateJunctionId => write!(f, "duplicate junction id"),
            Self::MissingJunction => write!(f, "missing junction"),
            Self::InvalidApproachId => write!(f, "invalid approach id"),
            Self::InvalidCornerId => write!(f, "invalid corner id"),
            Self::InvalidLaneConnectionId => write!(f, "invalid lane connection id"),
            Self::DuplicateLaneConnectionId => write!(f, "duplicate lane connection id"),
            Self::MissingLane => write!(f, "missing lane"),
            Self::IncompatibleLaneConnection => write!(f, "incompatible lane connection"),
            Self::CandidateNotAtGrade => write!(f, "candidate is not at grade"),
            Self::CandidateIgnored => write!(f, "candidate was explicitly ignored"),
            Self::CandidateStale => write!(f, "candidate is stale"),
            Self::CandidateAlreadyUsed => write!(f, "candidate is already used by a junction"),
            Self::InvalidJunctionGeometry => write!(f, "invalid junction geometry"),
            Self::InvalidSurface => write!(f, "invalid surface"),
            Self::StaleJunction => write!(f, "junction is stale"),
        }
    }
}

impl std::error::Error for KernelError {}
