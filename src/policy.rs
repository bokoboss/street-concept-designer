use crate::error::KernelError;

/// Central numerical policy for the R1A kernel.
///
/// The defaults are deliberately explicit and small enough to preserve a
/// millimetre-scale authored feature. Display sampling settings are derived
/// from the same policy but can be overridden by a caller as a presentation
/// choice; they never change canonical semantic coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TolerancePolicy {
    /// Coordinate/vector coincidence classification in metres.
    pub coordinate_coincidence_m: f64,
    /// Permitted station overshoot used only to clamp endpoint queries.
    pub station_bound_m: f64,
    /// Angular classification tolerance in radians.
    pub angular_rad: f64,
    /// Smallest accepted primitive/station range in metres.
    pub minimum_alignment_length_m: f64,
    /// Projection refinement convergence distance in metres.
    pub projection_convergence_m: f64,
    /// Maximum geometric deviation used to build smooth-curve arc-length data.
    pub adaptive_curve_error_m: f64,
    /// Default maximum derived sampling segment length in metres.
    pub sampling_max_segment_length_m: f64,
    /// Maximum recursive subdivision depth for derived geometry.
    pub max_sampling_depth: u8,
    /// Maximum points in one derived sample or smooth-curve lookup table.
    pub max_sampling_points: usize,
    /// Number of deterministic local projection refinement iterations.
    pub projection_iterations: u8,
}

impl Default for TolerancePolicy {
    fn default() -> Self {
        Self {
            coordinate_coincidence_m: 1.0e-9,
            station_bound_m: 1.0e-9,
            angular_rad: 1.0e-12,
            minimum_alignment_length_m: 1.0e-6,
            projection_convergence_m: 1.0e-9,
            adaptive_curve_error_m: 1.0e-3,
            sampling_max_segment_length_m: 10.0,
            max_sampling_depth: 20,
            max_sampling_points: 4096,
            projection_iterations: 32,
        }
    }
}

impl TolerancePolicy {
    /// Validate that every policy value is finite and has a meaningful domain.
    pub fn validate(&self) -> Result<(), KernelError> {
        let positive_finite = [
            (self.coordinate_coincidence_m, "coordinate_coincidence_m"),
            (self.station_bound_m, "station_bound_m"),
            (self.angular_rad, "angular_rad"),
            (
                self.minimum_alignment_length_m,
                "minimum_alignment_length_m",
            ),
            (self.projection_convergence_m, "projection_convergence_m"),
            (self.adaptive_curve_error_m, "adaptive_curve_error_m"),
            (
                self.sampling_max_segment_length_m,
                "sampling_max_segment_length_m",
            ),
        ];
        if positive_finite
            .iter()
            .any(|(value, _)| !value.is_finite() || *value <= 0.0)
        {
            let (value, field) = positive_finite
                .iter()
                .find(|(value, _)| !value.is_finite() || *value <= 0.0)
                .expect("the preceding any check guarantees a matching policy value");
            if !value.is_finite() {
                return Err(KernelError::NonFiniteInput { field });
            }
            return Err(KernelError::InvalidParameter { field });
        }
        if self.max_sampling_depth == 0 {
            return Err(KernelError::InvalidParameter {
                field: "max_sampling_depth",
            });
        }
        if self.max_sampling_points < 2 {
            return Err(KernelError::InvalidParameter {
                field: "max_sampling_points",
            });
        }
        if self.projection_iterations == 0 {
            return Err(KernelError::InvalidParameter {
                field: "projection_iterations",
            });
        }
        Ok(())
    }
}
