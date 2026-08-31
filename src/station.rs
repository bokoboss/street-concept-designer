use crate::error::KernelError;
use crate::policy::TolerancePolicy;

/// A bounded monotonic station domain in metres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StationRange {
    /// Inclusive beginning of the domain.
    pub start_m: f64,
    /// Inclusive end of the domain.
    pub end_m: f64,
}

impl StationRange {
    /// Construct a finite, positive-length station range.
    pub fn new(start_m: f64, end_m: f64, policy: &TolerancePolicy) -> Result<Self, KernelError> {
        policy.validate()?;
        if !start_m.is_finite() {
            return Err(KernelError::NonFiniteInput { field: "start_m" });
        }
        if !end_m.is_finite() {
            return Err(KernelError::NonFiniteInput { field: "end_m" });
        }
        let span = end_m - start_m;
        if !span.is_finite() {
            return Err(KernelError::NonFiniteResult {
                operation: "station range span",
            });
        }
        if span <= policy.minimum_alignment_length_m {
            return Err(KernelError::InvalidLength {
                operation: "station range",
            });
        }
        Ok(Self { start_m, end_m })
    }

    /// Return the range span in metres.
    pub fn length_m(self) -> f64 {
        self.end_m - self.start_m
    }

    /// Check exact semantic domain equality.
    pub fn same_domain(self, other: Self) -> bool {
        self == other
    }
}

pub(crate) fn clamp_station(
    station: f64,
    range: StationRange,
    policy: &TolerancePolicy,
) -> Result<f64, KernelError> {
    policy.validate()?;
    if !station.is_finite() {
        return Err(KernelError::NonFiniteInput { field: "station" });
    }
    if station < range.start_m {
        if (range.start_m - station).is_finite()
            && range.start_m - station <= policy.station_bound_m
        {
            return Ok(range.start_m);
        }
        return Err(KernelError::StationOutOfBounds {
            station,
            start: range.start_m,
            end: range.end_m,
        });
    }
    if station > range.end_m {
        if (station - range.end_m).is_finite() && station - range.end_m <= policy.station_bound_m {
            return Ok(range.end_m);
        }
        return Err(KernelError::StationOutOfBounds {
            station,
            start: range.start_m,
            end: range.end_m,
        });
    }
    Ok(station)
}
