use crate::error::KernelError;
use crate::policy::TolerancePolicy;
use crate::station::{clamp_station, StationRange};

/// Stable identity for an ordered semantic cross-section component.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentId(String);

impl ComponentId {
    /// Construct an id that can remain stable across profile sections.
    pub fn new(value: impl Into<String>) -> Result<Self, KernelError> {
        let value = value.into();
        if value.is_empty() || value.chars().any(char::is_whitespace) {
            return Err(KernelError::InvalidComponentId);
        }
        Ok(Self(value))
    }

    /// Borrow the id text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ComponentId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Semantic cross-section component kind used by the R1A spike.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentKind {
    /// A traffic-bearing lane, including an added/drop or turn-storage lane.
    TrafficLane,
    /// A median component.
    Median,
    /// A shoulder component.
    Shoulder,
    /// An edge strip component.
    EdgeStrip,
}

/// One station/width knot in a component profile.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WidthKnot {
    /// Station in metres.
    pub station_m: f64,
    /// Non-negative component width in metres.
    pub width_m: f64,
}

/// A piecewise-linear non-negative width profile over one bounded station range.
#[derive(Debug, Clone, PartialEq)]
pub struct PiecewiseLinearWidthProfile {
    station_range: StationRange,
    knots: Vec<WidthKnot>,
}

impl PiecewiseLinearWidthProfile {
    /// Construct a profile with strictly increasing stations and non-negative widths.
    pub fn new(
        station_range: StationRange,
        knots: Vec<WidthKnot>,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        policy.validate()?;
        if knots.len() < 2 {
            return Err(KernelError::InvalidWidthProfile);
        }
        let mut normalized = knots;
        for knot in &mut normalized {
            if !knot.station_m.is_finite() {
                return Err(KernelError::NonFiniteInput {
                    field: "width knot station",
                });
            }
            if !knot.width_m.is_finite() {
                return Err(KernelError::NonFiniteInput {
                    field: "width knot width",
                });
            }
            if knot.width_m < 0.0 {
                return Err(KernelError::InvalidWidthProfile);
            }
        }
        if !within_station_bound(
            normalized[0].station_m,
            station_range.start_m,
            policy.station_bound_m,
        ) || !within_station_bound(
            normalized[normalized.len() - 1].station_m,
            station_range.end_m,
            policy.station_bound_m,
        ) {
            return Err(KernelError::InvalidWidthProfile);
        }
        normalized[0].station_m = station_range.start_m;
        let last = normalized.len() - 1;
        normalized[last].station_m = station_range.end_m;
        for pair in normalized.windows(2) {
            if pair[1].station_m <= pair[0].station_m {
                return Err(KernelError::InvalidWidthProfile);
            }
        }
        Ok(Self {
            station_range,
            knots: normalized,
        })
    }

    /// Construct a constant-width profile over a range.
    pub fn constant(
        station_range: StationRange,
        width_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        Self::new(
            station_range,
            vec![
                WidthKnot {
                    station_m: station_range.start_m,
                    width_m,
                },
                WidthKnot {
                    station_m: station_range.end_m,
                    width_m,
                },
            ],
            policy,
        )
    }

    /// Profile station domain.
    pub fn station_range(&self) -> StationRange {
        self.station_range
    }

    /// Authored/interpolated knots in station order.
    pub fn knots(&self) -> &[WidthKnot] {
        &self.knots
    }

    /// Evaluate the non-negative width at a bounded station.
    pub fn width_at(&self, station_m: f64, policy: &TolerancePolicy) -> Result<f64, KernelError> {
        let station_m = clamp_station(station_m, self.station_range, policy)?;
        if station_m <= self.knots[0].station_m {
            return Ok(self.knots[0].width_m);
        }
        let last = self.knots.len() - 1;
        if station_m >= self.knots[last].station_m {
            return Ok(self.knots[last].width_m);
        }
        let mut low = 0usize;
        let mut high = last;
        while high - low > 1 {
            let middle = (low + high) / 2;
            if self.knots[middle].station_m <= station_m {
                low = middle;
            } else {
                high = middle;
            }
        }
        let lower = self.knots[low];
        let upper = self.knots[high];
        let fraction = (station_m - lower.station_m) / (upper.station_m - lower.station_m);
        let width_m = lower.width_m + (upper.width_m - lower.width_m) * fraction;
        if width_m.is_finite() && width_m >= 0.0 {
            Ok(width_m)
        } else if width_m.is_finite() {
            Err(KernelError::InvalidWidthProfile)
        } else {
            Err(KernelError::NonFiniteResult {
                operation: "width interpolation",
            })
        }
    }

    /// Whether the component has positive width at a station.
    pub fn is_active_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<bool, KernelError> {
        Ok(self.width_at(station_m, policy)? > 0.0)
    }
}

/// A semantic cross-section component with stable identity and no renderer state.
#[derive(Debug, Clone, PartialEq)]
pub struct CrossSectionComponent {
    id: ComponentId,
    kind: ComponentKind,
    width_profile: PiecewiseLinearWidthProfile,
}

impl CrossSectionComponent {
    /// Construct a component from a validated width profile.
    pub fn new(
        id: impl Into<String>,
        kind: ComponentKind,
        width_profile: PiecewiseLinearWidthProfile,
    ) -> Result<Self, KernelError> {
        Ok(Self {
            id: ComponentId::new(id)?,
            kind,
            width_profile,
        })
    }

    /// Construct a traffic lane. Turn-storage lanes use this same constructor.
    pub fn traffic_lane(
        id: impl Into<String>,
        width_profile: PiecewiseLinearWidthProfile,
    ) -> Result<Self, KernelError> {
        Self::new(id, ComponentKind::TrafficLane, width_profile)
    }

    /// Construct a median component.
    pub fn median(
        id: impl Into<String>,
        width_profile: PiecewiseLinearWidthProfile,
    ) -> Result<Self, KernelError> {
        Self::new(id, ComponentKind::Median, width_profile)
    }

    /// Construct a shoulder component.
    pub fn shoulder(
        id: impl Into<String>,
        width_profile: PiecewiseLinearWidthProfile,
    ) -> Result<Self, KernelError> {
        Self::new(id, ComponentKind::Shoulder, width_profile)
    }

    /// Construct an edge-strip component.
    pub fn edge_strip(
        id: impl Into<String>,
        width_profile: PiecewiseLinearWidthProfile,
    ) -> Result<Self, KernelError> {
        Self::new(id, ComponentKind::EdgeStrip, width_profile)
    }

    /// Stable component identity.
    pub fn id(&self) -> &ComponentId {
        &self.id
    }

    /// Semantic component kind.
    pub fn kind(&self) -> ComponentKind {
        self.kind
    }

    /// Component width profile.
    pub fn width_profile(&self) -> &PiecewiseLinearWidthProfile {
        &self.width_profile
    }
}

/// Resolved component state at a station, preserving semantic identity/order.
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentState {
    /// Stable component identity.
    pub id: ComponentId,
    /// Semantic kind.
    pub kind: ComponentKind,
    /// Interpolated width in metres.
    pub width_m: f64,
    /// True only when `width_m` is positive.
    pub active: bool,
}

/// Ordered semantic cross-section independent of rendering.
#[derive(Debug, Clone, PartialEq)]
pub struct CrossSection {
    station_range: StationRange,
    components: Vec<CrossSectionComponent>,
}

impl CrossSection {
    /// Construct an ordered cross-section with unique component identities.
    pub fn new(
        station_range: StationRange,
        components: Vec<CrossSectionComponent>,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        policy.validate()?;
        if components.is_empty() {
            return Err(KernelError::InvalidWidthProfile);
        }
        for (index, component) in components.iter().enumerate() {
            if component.width_profile.station_range() != station_range {
                return Err(KernelError::ComponentRangeMismatch);
            }
            if components[..index]
                .iter()
                .any(|previous| previous.id == component.id)
            {
                return Err(KernelError::DuplicateComponentId);
            }
        }
        Ok(Self {
            station_range,
            components,
        })
    }

    /// Cross-section station domain.
    pub fn station_range(&self) -> StationRange {
        self.station_range
    }

    /// Components in authored semantic order.
    pub fn components(&self) -> &[CrossSectionComponent] {
        &self.components
    }

    /// Stable ids in authored semantic order.
    pub fn ordered_component_ids(&self) -> Vec<ComponentId> {
        self.components
            .iter()
            .map(|component| component.id.clone())
            .collect()
    }

    /// Resolve every component at a station without changing identity or order.
    pub fn states_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Vec<ComponentState>, KernelError> {
        let station_m = clamp_station(station_m, self.station_range, policy)?;
        self.components
            .iter()
            .map(|component| {
                let width_m = component.width_profile.width_at(station_m, policy)?;
                Ok(ComponentState {
                    id: component.id.clone(),
                    kind: component.kind,
                    width_m,
                    active: width_m > 0.0,
                })
            })
            .collect()
    }

    /// Resolve one component by stable id at a station.
    pub fn state_at(
        &self,
        id: &ComponentId,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Option<ComponentState>, KernelError> {
        let states = self.states_at(station_m, policy)?;
        Ok(states.into_iter().find(|state| &state.id == id))
    }

    /// Sum the ordered component widths at a station.
    pub fn total_width_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<f64, KernelError> {
        let total =
            self.states_at(station_m, policy)?
                .into_iter()
                .try_fold(0.0, |total, state| {
                    let next = total + state.width_m;
                    if next.is_finite() {
                        Ok(next)
                    } else {
                        Err(KernelError::NonFiniteResult {
                            operation: "cross-section total width",
                        })
                    }
                })?;
        Ok(total)
    }
}

fn within_station_bound(value: f64, target: f64, bound: f64) -> bool {
    let difference = (value - target).abs();
    difference.is_finite() && difference <= bound
}
