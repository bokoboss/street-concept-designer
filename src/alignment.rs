use crate::error::KernelError;
use crate::math::{Point2, Vector2};
use crate::policy::TolerancePolicy;
use crate::station::{clamp_station, StationRange};

const FULL_TURN_RAD: f64 = std::f64::consts::TAU;

/// A nearest-point result in alignment station space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Projection {
    /// Bounded station of the nearest point in metres.
    pub station_m: f64,
    /// Point on the alignment at `station_m`.
    pub point: Point2,
    /// Euclidean query-to-alignment distance in metres.
    pub distance_m: f64,
    /// Signed query offset along the left-hand alignment normal in metres.
    pub lateral_offset_m: f64,
}

impl Projection {
    fn from_query(
        station_m: f64,
        point: Point2,
        tangent: Vector2,
        query: Point2,
    ) -> Result<Self, KernelError> {
        let normal = tangent.left();
        let offset = query - point;
        let distance_m = offset.length();
        let lateral_offset_m = offset.dot(normal);
        if !station_m.is_finite()
            || !point.is_finite()
            || !distance_m.is_finite()
            || !lateral_offset_m.is_finite()
        {
            return Err(KernelError::NonFiniteResult {
                operation: "projection",
            });
        }
        Ok(Self {
            station_m,
            point,
            distance_m,
            lateral_offset_m,
        })
    }

    fn prefer(candidate: Self, current: Self) -> Self {
        if candidate.distance_m < current.distance_m
            || (candidate.distance_m == current.distance_m
                && candidate.station_m < current.station_m)
        {
            candidate
        } else {
            current
        }
    }
}

/// One derived alignment sample with its local frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SamplePoint {
    /// Bounded station in metres.
    pub station_m: f64,
    /// Sampled engineering coordinate.
    pub point: Point2,
    /// Unit tangent in the increasing-station direction.
    pub tangent: Vector2,
    /// Unit left-hand normal.
    pub normal: Vector2,
}

/// Criteria for deterministic adaptive alignment sampling.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SamplingOptions {
    /// Maximum point-to-chord deviation in metres.
    pub max_chord_error_m: f64,
    /// Maximum station interval represented by one derived segment.
    pub max_segment_length_m: f64,
    /// Maximum number of returned points.
    pub max_points: usize,
    /// Maximum recursive subdivision depth.
    pub max_depth: u8,
}

impl SamplingOptions {
    /// Derive default display-independent sampling settings from the named policy.
    pub fn from_policy(policy: &TolerancePolicy) -> Self {
        Self {
            max_chord_error_m: policy.adaptive_curve_error_m,
            max_segment_length_m: policy.sampling_max_segment_length_m,
            max_points: policy.max_sampling_points,
            max_depth: policy.max_sampling_depth,
        }
    }

    fn validate(self) -> Result<(), KernelError> {
        if !self.max_chord_error_m.is_finite() || self.max_chord_error_m <= 0.0 {
            return Err(KernelError::InvalidParameter {
                field: "max_chord_error_m",
            });
        }
        if !self.max_segment_length_m.is_finite() || self.max_segment_length_m <= 0.0 {
            return Err(KernelError::InvalidParameter {
                field: "max_segment_length_m",
            });
        }
        if self.max_points < 2 {
            return Err(KernelError::InvalidParameter {
                field: "max_points",
            });
        }
        if self.max_depth == 0 {
            return Err(KernelError::InvalidParameter { field: "max_depth" });
        }
        Ok(())
    }
}

/// Alignment primitive kind. The enum is intentionally extensible for a future
/// spiral/clothoid variant without changing station-based consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignmentKind {
    /// Constant-tangent straight segment.
    Line,
    /// Constant-radius circular arc.
    CircularArc,
    /// Cubic Bezier conceptual curve with a deterministic arc-length lookup table.
    SmoothConceptualCurve,
}

/// A straight reference alignment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineAlignment {
    start: Point2,
    end: Point2,
    length_m: f64,
    tangent: Vector2,
    station_range: StationRange,
}

impl LineAlignment {
    /// Construct a line from its finite endpoints.
    pub fn new(start: Point2, end: Point2, policy: &TolerancePolicy) -> Result<Self, KernelError> {
        policy.validate()?;
        validate_point(start, "line.start")?;
        validate_point(end, "line.end")?;
        let delta = end - start;
        let length_m = delta.length();
        if !length_m.is_finite() {
            return Err(KernelError::NonFiniteResult {
                operation: "line length",
            });
        }
        if length_m <= policy.minimum_alignment_length_m {
            return Err(KernelError::InvalidLength { operation: "line" });
        }
        let tangent = delta / length_m;
        let station_range = StationRange::new(0.0, length_m, policy)?;
        tangent.checked("line tangent")?;
        Ok(Self {
            start,
            end,
            length_m,
            tangent,
            station_range,
        })
    }

    /// Start coordinate.
    pub fn start(&self) -> Point2 {
        self.start
    }

    /// End coordinate.
    pub fn end(&self) -> Point2 {
        self.end
    }

    /// Total alignment length in metres.
    pub fn length(&self) -> f64 {
        self.length_m
    }

    /// Inclusive station domain.
    pub fn station_range(&self) -> StationRange {
        self.station_range
    }

    /// Point at bounded station `s` in metres.
    pub fn point_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Point2, KernelError> {
        let station_m = clamp_station(station_m, self.station_range, policy)?;
        (self.start + self.tangent * station_m).checked("line point")
    }

    /// Unit tangent at bounded station `s`.
    pub fn tangent_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Vector2, KernelError> {
        clamp_station(station_m, self.station_range, policy)?;
        Ok(self.tangent)
    }

    /// Unit left-hand normal at bounded station `s`.
    pub fn normal_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Vector2, KernelError> {
        clamp_station(station_m, self.station_range, policy)?;
        Ok(self.tangent.left())
    }

    /// Project a finite XY query to the bounded segment.
    pub fn project(
        &self,
        query: Point2,
        policy: &TolerancePolicy,
    ) -> Result<Projection, KernelError> {
        policy.validate()?;
        validate_point(query, "line.query")?;
        let along = (query - self.start).dot(self.tangent);
        if !along.is_finite() {
            return Err(KernelError::NonFiniteResult {
                operation: "line projection station",
            });
        }
        let station_m = if along < 0.0 {
            0.0
        } else if along > self.length_m {
            self.length_m
        } else {
            along
        };
        let point = self.point_at(station_m, policy)?;
        Projection::from_query(station_m, point, self.tangent, query)
    }
}

/// A directed constant-radius circular arc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircularArcAlignment {
    center: Point2,
    radius_m: f64,
    start_angle_rad: f64,
    sweep_angle_rad: f64,
    length_m: f64,
    station_range: StationRange,
}

impl CircularArcAlignment {
    /// Construct an arc. Positive sweep is counter-clockwise; negative sweep is clockwise.
    pub fn new(
        center: Point2,
        radius_m: f64,
        start_angle_rad: f64,
        sweep_angle_rad: f64,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        policy.validate()?;
        validate_point(center, "arc.center")?;
        if !radius_m.is_finite() || radius_m <= policy.minimum_alignment_length_m {
            return Err(if radius_m.is_finite() {
                KernelError::InvalidLength {
                    operation: "arc radius",
                }
            } else {
                KernelError::NonFiniteInput {
                    field: "arc.radius_m",
                }
            });
        }
        if !start_angle_rad.is_finite() {
            return Err(KernelError::NonFiniteInput {
                field: "arc.start_angle_rad",
            });
        }
        if !sweep_angle_rad.is_finite() {
            return Err(KernelError::NonFiniteInput {
                field: "arc.sweep_angle_rad",
            });
        }
        if sweep_angle_rad.abs() <= policy.angular_rad {
            return Err(KernelError::InvalidParameter {
                field: "arc.sweep_angle_rad",
            });
        }
        if sweep_angle_rad.abs() > FULL_TURN_RAD + policy.angular_rad {
            return Err(KernelError::InvalidParameter {
                field: "arc.sweep_angle_rad",
            });
        }
        let sweep_angle_rad = if (sweep_angle_rad.abs() - FULL_TURN_RAD).abs() <= policy.angular_rad
        {
            sweep_angle_rad.signum() * FULL_TURN_RAD
        } else {
            sweep_angle_rad
        };
        let length_m = radius_m * sweep_angle_rad.abs();
        if !length_m.is_finite() {
            return Err(KernelError::NonFiniteResult {
                operation: "arc length",
            });
        }
        let station_range = StationRange::new(0.0, length_m, policy)?;
        Ok(Self {
            center,
            radius_m,
            start_angle_rad,
            sweep_angle_rad,
            length_m,
            station_range,
        })
    }

    /// Circle center.
    pub fn center(&self) -> Point2 {
        self.center
    }

    /// Radius in metres.
    pub fn radius(&self) -> f64 {
        self.radius_m
    }

    /// Directed start angle in radians.
    pub fn start_angle(&self) -> f64 {
        self.start_angle_rad
    }

    /// Directed sweep in radians.
    pub fn sweep_angle(&self) -> f64 {
        self.sweep_angle_rad
    }

    /// Total arc length in metres.
    pub fn length(&self) -> f64 {
        self.length_m
    }

    /// Inclusive station domain.
    pub fn station_range(&self) -> StationRange {
        self.station_range
    }

    fn point_for_angle(&self, angle_rad: f64) -> Result<Point2, KernelError> {
        let point = self.center
            + Vector2::new(
                angle_rad.cos() * self.radius_m,
                angle_rad.sin() * self.radius_m,
            );
        point.checked("arc point")
    }

    fn angle_at(&self, station_m: f64) -> f64 {
        self.start_angle_rad + self.sweep_angle_rad.signum() * station_m / self.radius_m
    }

    /// Point at bounded station `s` in metres.
    pub fn point_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Point2, KernelError> {
        let station_m = clamp_station(station_m, self.station_range, policy)?;
        self.point_for_angle(self.angle_at(station_m))
    }

    /// Unit tangent at bounded station `s`.
    pub fn tangent_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Vector2, KernelError> {
        let station_m = clamp_station(station_m, self.station_range, policy)?;
        let angle_rad = self.angle_at(station_m);
        let direction = self.sweep_angle_rad.signum();
        Vector2::new(-angle_rad.sin() * direction, angle_rad.cos() * direction)
            .checked("arc tangent")
    }

    /// Unit left-hand normal at bounded station `s`.
    pub fn normal_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Vector2, KernelError> {
        Ok(self.tangent_at(station_m, policy)?.left())
    }

    fn directed_angle_delta(&self, angle_rad: f64) -> f64 {
        if self.sweep_angle_rad.is_sign_positive() {
            (angle_rad - self.start_angle_rad).rem_euclid(FULL_TURN_RAD)
        } else {
            -(self.start_angle_rad - angle_rad).rem_euclid(FULL_TURN_RAD)
        }
    }

    fn chord_error_bound(&self, start_station: f64, end_station: f64) -> Result<f64, KernelError> {
        let station_span = end_station - start_station;
        let angle_span = station_span / self.radius_m;
        let half_chord_angle = angle_span.abs() / 4.0;
        let sine = half_chord_angle.sin();
        let error = 2.0 * self.radius_m * sine * sine;
        if error.is_finite() {
            Ok(error)
        } else {
            Err(KernelError::NonFiniteResult {
                operation: "arc sampling chord error",
            })
        }
    }

    /// Project a finite XY query to the nearest point on the bounded arc.
    pub fn project(
        &self,
        query: Point2,
        policy: &TolerancePolicy,
    ) -> Result<Projection, KernelError> {
        policy.validate()?;
        validate_point(query, "arc.query")?;
        let start_point = self.point_at(0.0, policy)?;
        let start_tangent = self.tangent_at(0.0, policy)?;
        let mut best = Projection::from_query(0.0, start_point, start_tangent, query)?;
        let end_point = self.point_at(self.length_m, policy)?;
        let end_tangent = self.tangent_at(self.length_m, policy)?;
        best = Projection::prefer(
            Projection::from_query(self.length_m, end_point, end_tangent, query)?,
            best,
        );

        let radial = query - self.center;
        let radial_length = radial.length();
        if !radial_length.is_finite() {
            return Err(KernelError::NonFiniteResult {
                operation: "arc projection radius",
            });
        }
        if radial_length > policy.coordinate_coincidence_m {
            let angle_rad = radial.y.atan2(radial.x);
            let delta = self.directed_angle_delta(angle_rad);
            let inside = if self.sweep_angle_rad.is_sign_positive() {
                delta >= -policy.angular_rad && delta <= self.sweep_angle_rad + policy.angular_rad
            } else {
                delta <= policy.angular_rad && delta >= self.sweep_angle_rad - policy.angular_rad
            };
            if inside {
                let station_m = if delta.abs() * self.radius_m <= policy.station_bound_m {
                    0.0
                } else {
                    (delta.abs() * self.radius_m).min(self.length_m)
                };
                let point = self.point_at(station_m, policy)?;
                let tangent = self.tangent_at(station_m, policy)?;
                best = Projection::prefer(
                    Projection::from_query(station_m, point, tangent, query)?,
                    best,
                );
            }
        }
        Ok(best)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct CubicControl {
    p0: Point2,
    p1: Point2,
    p2: Point2,
    p3: Point2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ArcLengthEntry {
    t: f64,
    station_m: f64,
    point: Point2,
}

/// A smooth conceptual cubic curve.
///
/// This is a concept-stage primitive, not a survey-grade clothoid. Its canonical
/// station map is a deterministic, adaptively flattened arc-length table; the
/// `Alignment` abstraction remains open for a future spiral/clothoid variant.
#[derive(Debug, Clone, PartialEq)]
pub struct SmoothConceptualCurve {
    control: CubicControl,
    lookup: Vec<ArcLengthEntry>,
    length_m: f64,
    station_range: StationRange,
}

impl SmoothConceptualCurve {
    /// Construct a cubic Bezier conceptual curve from four control points.
    pub fn new(
        p0: Point2,
        p1: Point2,
        p2: Point2,
        p3: Point2,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        policy.validate()?;
        for (point, field) in [
            (p0, "curve.p0"),
            (p1, "curve.p1"),
            (p2, "curve.p2"),
            (p3, "curve.p3"),
        ] {
            validate_point(point, field)?;
        }
        if (p1 - p0).length() <= policy.coordinate_coincidence_m {
            return Err(KernelError::InvalidParameter {
                field: "curve.start_tangent",
            });
        }
        if (p3 - p2).length() <= policy.coordinate_coincidence_m {
            return Err(KernelError::InvalidParameter {
                field: "curve.end_tangent",
            });
        }
        let control = CubicControl { p0, p1, p2, p3 };
        let lookup = build_lookup(control, policy)?;
        let length_m = lookup
            .last()
            .map(|entry| entry.station_m)
            .ok_or(KernelError::ProjectionUndefined)?;
        if !length_m.is_finite() {
            return Err(KernelError::NonFiniteResult {
                operation: "smooth curve length",
            });
        }
        let station_range = StationRange::new(0.0, length_m, policy)?;
        Ok(Self {
            control,
            lookup,
            length_m,
            station_range,
        })
    }

    /// Control points in Bezier order.
    pub fn control_points(&self) -> [Point2; 4] {
        [
            self.control.p0,
            self.control.p1,
            self.control.p2,
            self.control.p3,
        ]
    }

    /// Total conceptual-curve length in metres.
    pub fn length(&self) -> f64 {
        self.length_m
    }

    /// Inclusive station domain.
    pub fn station_range(&self) -> StationRange {
        self.station_range
    }

    fn point_for_parameter(&self, t: f64) -> Point2 {
        cubic_point(self.control, t)
    }

    fn parameter_at_station(&self, station_m: f64) -> f64 {
        if station_m <= 0.0 {
            return 0.0;
        }
        if station_m >= self.length_m {
            return 1.0;
        }
        let mut low = 0usize;
        let mut high = self.lookup.len() - 1;
        while high - low > 1 {
            let middle = (low + high) / 2;
            if self.lookup[middle].station_m < station_m {
                low = middle;
            } else {
                high = middle;
            }
        }
        let lower = self.lookup[low];
        let upper = self.lookup[high];
        let station_span = upper.station_m - lower.station_m;
        if station_span <= 0.0 {
            upper.t
        } else {
            let fraction = (station_m - lower.station_m) / station_span;
            lower.t + (upper.t - lower.t) * fraction
        }
    }

    fn chord_error_bound(
        &self,
        start_station: f64,
        start_point: Point2,
        end_station: f64,
        end_point: Point2,
        policy: &TolerancePolicy,
    ) -> Result<f64, KernelError> {
        let start_t = self.parameter_at_station(start_station);
        let end_t = self.parameter_at_station(end_station);
        let control = cubic_subcurve(self.control, start_t, end_t)?;
        cubic_chord_error_bound(control, start_point, end_point, policy)
    }

    /// Point at bounded station `s` in metres.
    pub fn point_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Point2, KernelError> {
        let station_m = clamp_station(station_m, self.station_range, policy)?;
        self.point_for_parameter(self.parameter_at_station(station_m))
            .checked("smooth curve point")
    }

    fn derivative_at_parameter(&self, t: f64) -> Vector2 {
        let one_minus_t = 1.0 - t;
        (self.control.p1 - self.control.p0) * (3.0 * one_minus_t * one_minus_t)
            + (self.control.p2 - self.control.p1) * (6.0 * one_minus_t * t)
            + (self.control.p3 - self.control.p2) * (3.0 * t * t)
    }

    fn fallback_tangent(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Vector2, KernelError> {
        for pair in self.lookup.windows(2) {
            if station_m >= pair[0].station_m && station_m <= pair[1].station_m {
                let direction = pair[1].point - pair[0].point;
                if direction.length() > policy.coordinate_coincidence_m {
                    return direction.try_normalized(policy);
                }
            }
        }
        for pair in self.lookup.windows(2) {
            let direction = pair[1].point - pair[0].point;
            if direction.length() > policy.coordinate_coincidence_m {
                return direction.try_normalized(policy);
            }
        }
        Err(KernelError::ProjectionUndefined)
    }

    /// Unit tangent at bounded station `s`.
    pub fn tangent_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Vector2, KernelError> {
        let station_m = clamp_station(station_m, self.station_range, policy)?;
        let parameter = self.parameter_at_station(station_m);
        let derivative = self.derivative_at_parameter(parameter);
        if derivative.length() > policy.coordinate_coincidence_m {
            derivative.try_normalized(policy)
        } else {
            self.fallback_tangent(station_m, policy)
        }
    }

    /// Unit left-hand normal at bounded station `s`.
    pub fn normal_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Vector2, KernelError> {
        Ok(self.tangent_at(station_m, policy)?.left())
    }

    fn station_for_parameter(
        &self,
        lower: ArcLengthEntry,
        upper: ArcLengthEntry,
        parameter: f64,
    ) -> f64 {
        let fraction = ((parameter - lower.t) / (upper.t - lower.t)).clamp(0.0, 1.0);
        lower.station_m + (upper.station_m - lower.station_m) * fraction
    }

    fn refine_projection_parameter(
        &self,
        query: Point2,
        lower: ArcLengthEntry,
        upper: ArcLengthEntry,
        policy: &TolerancePolicy,
    ) -> Result<f64, KernelError> {
        let mut left = lower.t;
        let mut right = upper.t;
        for _ in 0..policy.projection_iterations {
            let one_third = (right - left) / 3.0;
            let first = left + one_third;
            let second = right - one_third;
            let first_distance = self.point_for_parameter(first).distance_to(query);
            let second_distance = self.point_for_parameter(second).distance_to(query);
            if !first_distance.is_finite() || !second_distance.is_finite() {
                return Err(KernelError::NonFiniteResult {
                    operation: "smooth curve projection refinement",
                });
            }
            if first_distance <= second_distance {
                right = second;
            } else {
                left = first;
            }
            let bracket_length = self
                .point_for_parameter(left)
                .distance_to(self.point_for_parameter(right));
            if bracket_length.is_finite() && bracket_length <= policy.projection_convergence_m {
                break;
            }
        }
        Ok((left + right) / 2.0)
    }

    /// Project a finite XY query to the nearest point on the conceptual curve.
    pub fn project(
        &self,
        query: Point2,
        policy: &TolerancePolicy,
    ) -> Result<Projection, KernelError> {
        policy.validate()?;
        validate_point(query, "curve.query")?;
        let mut best = Projection::from_query(
            0.0,
            self.point_at(0.0, policy)?,
            self.tangent_at(0.0, policy)?,
            query,
        )?;
        best = Projection::prefer(
            Projection::from_query(
                self.length_m,
                self.point_at(self.length_m, policy)?,
                self.tangent_at(self.length_m, policy)?,
                query,
            )?,
            best,
        );

        for pair in self.lookup.windows(2) {
            let lower = pair[0];
            let upper = pair[1];
            if upper.t <= lower.t || upper.station_m <= lower.station_m {
                continue;
            }
            let chord = upper.point - lower.point;
            let chord_length = chord.length();
            let query_offset = query - lower.point;
            let fraction = if chord_length <= policy.coordinate_coincidence_m {
                0.0
            } else {
                (query_offset.dot(chord / chord_length) / chord_length).clamp(0.0, 1.0)
            };
            if !fraction.is_finite() {
                return Err(KernelError::NonFiniteResult {
                    operation: "smooth curve projection candidate",
                });
            }
            let candidate_parameter = lower.t + (upper.t - lower.t) * fraction;
            let candidate_station = self.station_for_parameter(lower, upper, candidate_parameter);
            let candidate = Projection::from_query(
                candidate_station,
                self.point_at(candidate_station, policy)?,
                self.tangent_at(candidate_station, policy)?,
                query,
            )?;
            best = Projection::prefer(candidate, best);

            let refined_parameter =
                self.refine_projection_parameter(query, lower, upper, policy)?;
            let refined_station = self.station_for_parameter(lower, upper, refined_parameter);
            let refined = Projection::from_query(
                refined_station,
                self.point_at(refined_station, policy)?,
                self.tangent_at(refined_station, policy)?,
                query,
            )?;
            best = Projection::prefer(refined, best);
        }
        Ok(best)
    }
}

/// A tagged alignment primitive used by semantic roads.
#[derive(Debug, Clone, PartialEq)]
pub enum Alignment {
    /// Straight segment.
    Line(LineAlignment),
    /// Directed circular arc.
    CircularArc(CircularArcAlignment),
    /// Smooth cubic conceptual curve.
    SmoothConceptualCurve(SmoothConceptualCurve),
}

impl Alignment {
    /// Construct a line alignment.
    pub fn line(start: Point2, end: Point2, policy: &TolerancePolicy) -> Result<Self, KernelError> {
        Ok(Self::Line(LineAlignment::new(start, end, policy)?))
    }

    /// Construct a circular arc alignment.
    pub fn circular_arc(
        center: Point2,
        radius_m: f64,
        start_angle_rad: f64,
        sweep_angle_rad: f64,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        Ok(Self::CircularArc(CircularArcAlignment::new(
            center,
            radius_m,
            start_angle_rad,
            sweep_angle_rad,
            policy,
        )?))
    }

    /// Construct a smooth cubic conceptual curve alignment.
    pub fn smooth_curve(
        p0: Point2,
        p1: Point2,
        p2: Point2,
        p3: Point2,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        Ok(Self::SmoothConceptualCurve(SmoothConceptualCurve::new(
            p0, p1, p2, p3, policy,
        )?))
    }

    /// Alias emphasizing that the cubic is a concept-design curve.
    pub fn smooth_conceptual_curve(
        p0: Point2,
        p1: Point2,
        p2: Point2,
        p3: Point2,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        Self::smooth_curve(p0, p1, p2, p3, policy)
    }

    /// Primitive kind.
    pub fn kind(&self) -> AlignmentKind {
        match self {
            Self::Line(_) => AlignmentKind::Line,
            Self::CircularArc(_) => AlignmentKind::CircularArc,
            Self::SmoothConceptualCurve(_) => AlignmentKind::SmoothConceptualCurve,
        }
    }

    /// Total alignment length in metres.
    pub fn length(&self) -> f64 {
        match self {
            Self::Line(alignment) => alignment.length(),
            Self::CircularArc(alignment) => alignment.length(),
            Self::SmoothConceptualCurve(alignment) => alignment.length(),
        }
    }

    /// Inclusive station domain beginning at zero.
    pub fn station_range(&self) -> StationRange {
        match self {
            Self::Line(alignment) => alignment.station_range(),
            Self::CircularArc(alignment) => alignment.station_range(),
            Self::SmoothConceptualCurve(alignment) => alignment.station_range(),
        }
    }

    /// Point at bounded station `s` in metres.
    pub fn point_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Point2, KernelError> {
        match self {
            Self::Line(alignment) => alignment.point_at(station_m, policy),
            Self::CircularArc(alignment) => alignment.point_at(station_m, policy),
            Self::SmoothConceptualCurve(alignment) => alignment.point_at(station_m, policy),
        }
    }

    /// Unit tangent at bounded station `s`.
    pub fn tangent_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Vector2, KernelError> {
        match self {
            Self::Line(alignment) => alignment.tangent_at(station_m, policy),
            Self::CircularArc(alignment) => alignment.tangent_at(station_m, policy),
            Self::SmoothConceptualCurve(alignment) => alignment.tangent_at(station_m, policy),
        }
    }

    /// Unit left-hand normal at bounded station `s`.
    pub fn normal_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Vector2, KernelError> {
        match self {
            Self::Line(alignment) => alignment.normal_at(station_m, policy),
            Self::CircularArc(alignment) => alignment.normal_at(station_m, policy),
            Self::SmoothConceptualCurve(alignment) => alignment.normal_at(station_m, policy),
        }
    }

    /// Project a finite XY query to the bounded alignment.
    pub fn project(
        &self,
        query: Point2,
        policy: &TolerancePolicy,
    ) -> Result<Projection, KernelError> {
        match self {
            Self::Line(alignment) => alignment.project(query, policy),
            Self::CircularArc(alignment) => alignment.project(query, policy),
            Self::SmoothConceptualCurve(alignment) => alignment.project(query, policy),
        }
    }

    fn chord_error_bound(
        &self,
        start_station: f64,
        start_point: Point2,
        end_station: f64,
        end_point: Point2,
        policy: &TolerancePolicy,
    ) -> Result<f64, KernelError> {
        match self {
            Self::Line(_) => Ok(0.0),
            Self::CircularArc(alignment) => alignment.chord_error_bound(start_station, end_station),
            Self::SmoothConceptualCurve(alignment) => alignment.chord_error_bound(
                start_station,
                start_point,
                end_station,
                end_point,
                policy,
            ),
        }
    }

    /// Adaptively sample the alignment with a deterministic point budget.
    ///
    /// `max_chord_error_m` is enforced with a primitive-specific bound: zero for
    /// lines, circular-arc sagitta for arcs, and a cubic convex-hull bound for
    /// smooth conceptual curves.
    pub fn sample(
        &self,
        options: SamplingOptions,
        policy: &TolerancePolicy,
    ) -> Result<Vec<SamplePoint>, KernelError> {
        policy.validate()?;
        options.validate()?;
        let length_m = self.length();
        let start = self.point_at(0.0, policy)?;
        let end = self.point_at(length_m, policy)?;
        let mut stations = vec![0.0];
        let mut context = SampleContext {
            alignment: self,
            options,
            policy,
            stations: &mut stations,
        };
        context.subdivide(0.0, start, length_m, end, 0)?;
        stations
            .into_iter()
            .map(|station_m| {
                Ok(SamplePoint {
                    station_m,
                    point: self.point_at(station_m, policy)?,
                    tangent: self.tangent_at(station_m, policy)?,
                    normal: self.normal_at(station_m, policy)?,
                })
            })
            .collect()
    }
}

fn validate_point(point: Point2, field: &'static str) -> Result<(), KernelError> {
    if point.is_finite() {
        Ok(())
    } else {
        Err(KernelError::NonFiniteInput { field })
    }
}

fn point_segment_distance(
    point: Point2,
    segment_start: Point2,
    segment_end: Point2,
    policy: &TolerancePolicy,
) -> Result<f64, KernelError> {
    let segment = segment_end - segment_start;
    let segment_length = segment.length();
    if !segment_length.is_finite() {
        return Err(KernelError::NonFiniteResult {
            operation: "point-segment distance",
        });
    }
    if segment_length <= policy.coordinate_coincidence_m {
        let distance = point.distance_to(segment_start);
        return if distance.is_finite() {
            Ok(distance)
        } else {
            Err(KernelError::NonFiniteResult {
                operation: "point-segment distance",
            })
        };
    }
    let along =
        ((point - segment_start).dot(segment / segment_length) / segment_length).clamp(0.0, 1.0);
    if !along.is_finite() {
        return Err(KernelError::NonFiniteResult {
            operation: "point-segment projection",
        });
    }
    let closest = segment_start + segment * along;
    let distance = point.distance_to(closest);
    if distance.is_finite() {
        Ok(distance)
    } else {
        Err(KernelError::NonFiniteResult {
            operation: "point-segment distance",
        })
    }
}

struct SampleContext<'a> {
    alignment: &'a Alignment,
    options: SamplingOptions,
    policy: &'a TolerancePolicy,
    stations: &'a mut Vec<f64>,
}

impl<'a> SampleContext<'a> {
    fn subdivide(
        &mut self,
        start_station: f64,
        start_point: Point2,
        end_station: f64,
        end_point: Point2,
        depth: u8,
    ) -> Result<(), KernelError> {
        let midpoint_station = start_station + (end_station - start_station) / 2.0;
        let midpoint = self.alignment.point_at(midpoint_station, self.policy)?;
        let station_span = end_station - start_station;
        if !station_span.is_finite() {
            return Err(KernelError::NonFiniteResult {
                operation: "sampling station span",
            });
        }
        let chord_error = self.alignment.chord_error_bound(
            start_station,
            start_point,
            end_station,
            end_point,
            self.policy,
        )?;
        let needs_split = station_span > self.options.max_segment_length_m
            || chord_error > self.options.max_chord_error_m;
        if needs_split {
            if depth >= self.options.max_depth || self.stations.len() + 2 > self.options.max_points
            {
                return Err(KernelError::SamplingLimitExceeded);
            }
            if midpoint_station <= start_station || midpoint_station >= end_station {
                return Err(KernelError::SamplingLimitExceeded);
            }
            self.subdivide(
                start_station,
                start_point,
                midpoint_station,
                midpoint,
                depth + 1,
            )?;
            self.subdivide(
                midpoint_station,
                midpoint,
                end_station,
                end_point,
                depth + 1,
            )?;
        } else {
            self.stations.push(end_station);
        }
        Ok(())
    }
}

fn cubic_point(control: CubicControl, t: f64) -> Point2 {
    let one_minus_t = 1.0 - t;
    let first = one_minus_t * one_minus_t * one_minus_t;
    let second = 3.0 * one_minus_t * one_minus_t * t;
    let third = 3.0 * one_minus_t * t * t;
    let fourth = t * t * t;
    Point2::new(
        control.p0.x * first + control.p1.x * second + control.p2.x * third + control.p3.x * fourth,
        control.p0.y * first + control.p1.y * second + control.p2.y * third + control.p3.y * fourth,
    )
}

fn split_cubic(control: CubicControl) -> (CubicControl, CubicControl) {
    let p01 = midpoint(control.p0, control.p1);
    let p12 = midpoint(control.p1, control.p2);
    let p23 = midpoint(control.p2, control.p3);
    let p012 = midpoint(p01, p12);
    let p123 = midpoint(p12, p23);
    let p0123 = midpoint(p012, p123);
    (
        CubicControl {
            p0: control.p0,
            p1: p01,
            p2: p012,
            p3: p0123,
        },
        CubicControl {
            p0: p0123,
            p1: p123,
            p2: p23,
            p3: control.p3,
        },
    )
}

fn split_cubic_at(control: CubicControl, parameter: f64) -> (CubicControl, CubicControl) {
    let p01 = lerp(control.p0, control.p1, parameter);
    let p12 = lerp(control.p1, control.p2, parameter);
    let p23 = lerp(control.p2, control.p3, parameter);
    let p012 = lerp(p01, p12, parameter);
    let p123 = lerp(p12, p23, parameter);
    let p0123 = lerp(p012, p123, parameter);
    (
        CubicControl {
            p0: control.p0,
            p1: p01,
            p2: p012,
            p3: p0123,
        },
        CubicControl {
            p0: p0123,
            p1: p123,
            p2: p23,
            p3: control.p3,
        },
    )
}

fn cubic_subcurve(
    control: CubicControl,
    start_parameter: f64,
    end_parameter: f64,
) -> Result<CubicControl, KernelError> {
    if !start_parameter.is_finite()
        || !end_parameter.is_finite()
        || !(0.0..=1.0).contains(&start_parameter)
        || !(0.0..=1.0).contains(&end_parameter)
        || end_parameter <= start_parameter
    {
        return Err(KernelError::SamplingLimitExceeded);
    }
    let (_, remaining) = split_cubic_at(control, start_parameter);
    let local_end = (end_parameter - start_parameter) / (1.0 - start_parameter);
    let (subcurve, _) = split_cubic_at(remaining, local_end);
    if [subcurve.p0, subcurve.p1, subcurve.p2, subcurve.p3]
        .iter()
        .all(|point| point.is_finite())
    {
        Ok(subcurve)
    } else {
        Err(KernelError::NonFiniteResult {
            operation: "smooth curve sampling subdivision",
        })
    }
}

fn midpoint(first: Point2, second: Point2) -> Point2 {
    Point2::new((first.x + second.x) / 2.0, (first.y + second.y) / 2.0)
}

fn lerp(first: Point2, second: Point2, fraction: f64) -> Point2 {
    first + (second - first) * fraction
}

fn cubic_flatness(control: CubicControl, policy: &TolerancePolicy) -> Result<f64, KernelError> {
    let first = point_segment_distance(control.p1, control.p0, control.p3, policy)?;
    let second = point_segment_distance(control.p2, control.p0, control.p3, policy)?;
    Ok(first.max(second))
}

fn cubic_chord_error_bound(
    control: CubicControl,
    chord_start: Point2,
    chord_end: Point2,
    policy: &TolerancePolicy,
) -> Result<f64, KernelError> {
    let mut bound = 0.0_f64;
    for point in [control.p0, control.p1, control.p2, control.p3] {
        bound = bound.max(point_segment_distance(
            point,
            chord_start,
            chord_end,
            policy,
        )?);
    }
    Ok(bound)
}

fn flatten_cubic(
    control: CubicControl,
    start_t: f64,
    end_t: f64,
    depth: u8,
    policy: &TolerancePolicy,
    points: &mut Vec<(f64, Point2)>,
) -> Result<(), KernelError> {
    let flatness = cubic_flatness(control, policy)?;
    if !flatness.is_finite() {
        return Err(KernelError::NonFiniteResult {
            operation: "smooth curve flatness",
        });
    }
    if flatness > policy.adaptive_curve_error_m {
        if depth >= policy.max_sampling_depth || points.len() + 2 > policy.max_sampling_points {
            return Err(KernelError::SamplingLimitExceeded);
        }
        let (left, right) = split_cubic(control);
        let middle_t = start_t + (end_t - start_t) / 2.0;
        flatten_cubic(left, start_t, middle_t, depth + 1, policy, points)?;
        flatten_cubic(right, middle_t, end_t, depth + 1, policy, points)?;
    } else {
        points.push((end_t, control.p3));
    }
    Ok(())
}

fn build_lookup(
    control: CubicControl,
    policy: &TolerancePolicy,
) -> Result<Vec<ArcLengthEntry>, KernelError> {
    let mut flattened = vec![(0.0, control.p0)];
    flatten_cubic(control, 0.0, 1.0, 0, policy, &mut flattened)?;
    let mut lookup = Vec::with_capacity(flattened.len());
    lookup.push(ArcLengthEntry {
        t: 0.0,
        station_m: 0.0,
        point: control.p0,
    });
    let mut station_m = 0.0;
    for pair in flattened.windows(2) {
        let segment_length = pair[1].1.distance_to(pair[0].1);
        if !segment_length.is_finite() {
            return Err(KernelError::NonFiniteResult {
                operation: "smooth curve chord length",
            });
        }
        if segment_length <= policy.coordinate_coincidence_m {
            continue;
        }
        station_m += segment_length;
        if !station_m.is_finite() {
            return Err(KernelError::NonFiniteResult {
                operation: "smooth curve station length",
            });
        }
        lookup.push(ArcLengthEntry {
            t: pair[1].0,
            station_m,
            point: pair[1].1,
        });
    }
    if station_m <= policy.minimum_alignment_length_m || lookup.len() < 2 {
        return Err(KernelError::InvalidLength {
            operation: "smooth curve",
        });
    }
    Ok(lookup)
}
