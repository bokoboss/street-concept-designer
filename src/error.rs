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
        }
    }
}

impl std::error::Error for KernelError {}
