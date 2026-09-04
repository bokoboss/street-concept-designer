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
    /// An authored ordered sequence of alignment primitives.
    Composite,
}

/// Stable identity for one authored segment within a road alignment.
///
/// Segment ids are scoped by their owning road/scenario.  They are semantic
/// ids, not renderer handles or coordinate-derived values.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AlignmentSegmentId(String);

impl AlignmentSegmentId {
    /// Construct a non-empty, whitespace-free segment identity.
    pub fn new(value: impl Into<String>) -> Result<Self, KernelError> {
        let value = value.into();
        if value.is_empty() || value.chars().any(char::is_whitespace) {
            return Err(KernelError::InvalidAlignmentSegmentId);
        }
        Ok(Self(value))
    }

    /// Borrow the stable id text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for AlignmentSegmentId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// One accepted primitive family used by an authored alignment segment.
#[derive(Debug, Clone, PartialEq)]
pub enum AlignmentPrimitive {
    /// Straight reference segment.
    Line(LineAlignment),
    /// Directed circular-arc reference segment.
    CircularArc(CircularArcAlignment),
    /// Smooth conceptual cubic reference segment.
    SmoothConceptualCurve(SmoothConceptualCurve),
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

impl AlignmentPrimitive {
    /// Construct a line primitive.
    pub fn line(start: Point2, end: Point2, policy: &TolerancePolicy) -> Result<Self, KernelError> {
        Ok(Self::Line(LineAlignment::new(start, end, policy)?))
    }

    /// Construct a circular-arc primitive.
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

    /// Construct a smooth conceptual cubic primitive.
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

    /// Primitive family.
    pub fn kind(&self) -> AlignmentKind {
        match self {
            Self::Line(_) => AlignmentKind::Line,
            Self::CircularArc(_) => AlignmentKind::CircularArc,
            Self::SmoothConceptualCurve(_) => AlignmentKind::SmoothConceptualCurve,
        }
    }

    /// Primitive length in metres.
    pub fn length(&self) -> f64 {
        match self {
            Self::Line(alignment) => alignment.length(),
            Self::CircularArc(alignment) => alignment.length(),
            Self::SmoothConceptualCurve(alignment) => alignment.length(),
        }
    }

    /// Primitive-local station range beginning at zero.
    pub fn station_range(&self) -> StationRange {
        match self {
            Self::Line(alignment) => alignment.station_range(),
            Self::CircularArc(alignment) => alignment.station_range(),
            Self::SmoothConceptualCurve(alignment) => alignment.station_range(),
        }
    }

    /// Point at primitive-local station `s` in metres.
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

    /// Unit tangent at primitive-local station `s`.
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

    /// Unit left-hand normal at primitive-local station `s`.
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

    /// Project a finite query to primitive-local station space.
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

    fn validate(&self, policy: &TolerancePolicy) -> Result<(), KernelError> {
        policy.validate()?;
        let range = self.station_range();
        if range.start_m != 0.0
            || !self.length().is_finite()
            || self.length() <= policy.minimum_alignment_length_m
        {
            return Err(KernelError::InvalidLength {
                operation: "alignment segment",
            });
        }
        self.point_at(range.start_m, policy)?;
        self.point_at(range.end_m, policy)?;
        self.tangent_at(range.start_m, policy)?;
        self.tangent_at(range.end_m, policy)?;
        Ok(())
    }
}

/// One stable-id-bearing segment in a production alignment.
#[derive(Debug, Clone, PartialEq)]
pub struct AlignmentSegment {
    id: AlignmentSegmentId,
    primitive: AlignmentPrimitive,
}

impl AlignmentSegment {
    /// Construct a segment from a validated primitive.
    pub fn new(
        id: impl Into<String>,
        primitive: AlignmentPrimitive,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        policy.validate()?;
        let id = AlignmentSegmentId::new(id)?;
        primitive.validate(policy)?;
        Ok(Self { id, primitive })
    }

    /// Construct a line segment with a stable id.
    pub fn line(
        id: impl Into<String>,
        start: Point2,
        end: Point2,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        Self::new(id, AlignmentPrimitive::line(start, end, policy)?, policy)
    }

    /// Construct a circular-arc segment with a stable id.
    pub fn circular_arc(
        id: impl Into<String>,
        center: Point2,
        radius_m: f64,
        start_angle_rad: f64,
        sweep_angle_rad: f64,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        Self::new(
            id,
            AlignmentPrimitive::circular_arc(
                center,
                radius_m,
                start_angle_rad,
                sweep_angle_rad,
                policy,
            )?,
            policy,
        )
    }

    /// Construct a smooth conceptual curve segment with a stable id.
    pub fn smooth_curve(
        id: impl Into<String>,
        p0: Point2,
        p1: Point2,
        p2: Point2,
        p3: Point2,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        Self::new(
            id,
            AlignmentPrimitive::smooth_curve(p0, p1, p2, p3, policy)?,
            policy,
        )
    }

    /// Alias emphasizing that the cubic is a concept-design curve.
    pub fn smooth_conceptual_curve(
        id: impl Into<String>,
        p0: Point2,
        p1: Point2,
        p2: Point2,
        p3: Point2,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        Self::smooth_curve(id, p0, p1, p2, p3, policy)
    }

    /// Stable semantic segment id.
    pub fn id(&self) -> &AlignmentSegmentId {
        &self.id
    }

    /// Segment primitive.
    pub fn primitive(&self) -> &AlignmentPrimitive {
        &self.primitive
    }

    /// Segment primitive family.
    pub fn kind(&self) -> AlignmentKind {
        self.primitive.kind()
    }

    /// Segment length in metres.
    pub fn length(&self) -> f64 {
        self.primitive.length()
    }

    /// Segment-local station range beginning at zero.
    pub fn station_range(&self) -> StationRange {
        self.primitive.station_range()
    }

    /// Segment start point.
    pub fn start_point(&self, policy: &TolerancePolicy) -> Result<Point2, KernelError> {
        self.primitive.point_at(0.0, policy)
    }

    /// Segment end point.
    pub fn end_point(&self, policy: &TolerancePolicy) -> Result<Point2, KernelError> {
        self.primitive.point_at(self.length(), policy)
    }

    /// Segment start tangent.
    pub fn start_tangent(&self, policy: &TolerancePolicy) -> Result<Vector2, KernelError> {
        self.primitive.tangent_at(0.0, policy)
    }

    /// Segment end tangent.
    pub fn end_tangent(&self, policy: &TolerancePolicy) -> Result<Vector2, KernelError> {
        self.primitive.tangent_at(self.length(), policy)
    }
}

const LEGACY_SEGMENT_ID: &str = "segment-0";

/// Validated cumulative station data for one authored composite alignment.
#[derive(Debug, Clone, PartialEq)]
pub struct CompositeAlignment {
    segments: Vec<AlignmentSegment>,
    segment_ranges: Vec<StationRange>,
    station_range: StationRange,
}

impl CompositeAlignment {
    /// Construct and validate an ordered, tangent-continuous alignment.
    pub fn new(
        segments: Vec<AlignmentSegment>,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        policy.validate()?;
        if segments.is_empty() {
            return Err(KernelError::InvalidParameter {
                field: "alignment.segments",
            });
        }

        for (index, segment) in segments.iter().enumerate() {
            if segments[..index]
                .iter()
                .any(|previous| previous.id() == segment.id())
            {
                return Err(KernelError::DuplicateAlignmentSegmentId);
            }
        }

        for index in 1..segments.len() {
            let previous = &segments[index - 1];
            let current = &segments[index];
            let previous_end = previous.end_point(policy)?;
            let current_start = current.start_point(policy)?;
            let gap_m = previous_end.distance_to(current_start);
            if !gap_m.is_finite() {
                return Err(KernelError::NonFiniteResult {
                    operation: "alignment segment endpoint gap",
                });
            }
            if gap_m > policy.coordinate_coincidence_m {
                return Err(KernelError::AlignmentEndpointGap {
                    segment_index: index,
                    gap_m,
                });
            }

            let previous_tangent = previous.end_tangent(policy)?;
            let current_tangent = current.start_tangent(policy)?;
            let angle_rad = previous_tangent
                .cross(current_tangent)
                .abs()
                .atan2(previous_tangent.dot(current_tangent));
            if !angle_rad.is_finite() {
                return Err(KernelError::NonFiniteResult {
                    operation: "alignment segment tangent continuity",
                });
            }
            if angle_rad > policy.angular_rad {
                return Err(KernelError::AlignmentTangentDiscontinuity {
                    segment_index: index,
                    angle_rad,
                });
            }
        }

        let mut segment_ranges = Vec::with_capacity(segments.len());
        let mut cumulative_station_m = 0.0;
        for segment in &segments {
            let end_station_m = cumulative_station_m + segment.length();
            if !end_station_m.is_finite() {
                return Err(KernelError::NonFiniteResult {
                    operation: "composite alignment station range",
                });
            }
            if end_station_m <= cumulative_station_m {
                return Err(KernelError::InvalidLength {
                    operation: "composite alignment station order",
                });
            }
            segment_ranges.push(StationRange::new(
                cumulative_station_m,
                end_station_m,
                policy,
            )?);
            cumulative_station_m = end_station_m;
        }
        let station_range = StationRange::new(0.0, cumulative_station_m, policy)?;
        Ok(Self {
            segments,
            segment_ranges,
            station_range,
        })
    }

    /// Authored segments in exact stable order.
    pub fn segments(&self) -> &[AlignmentSegment] {
        &self.segments
    }

    /// Exact cumulative station range for each segment.
    pub fn segment_ranges(&self) -> &[StationRange] {
        &self.segment_ranges
    }

    /// Whole-alignment station range.
    pub fn station_range(&self) -> StationRange {
        self.station_range
    }

    /// Total cumulative alignment length in metres.
    pub fn length(&self) -> f64 {
        self.station_range.end_m
    }

    fn locate_segment(&self, station_m: f64) -> (&AlignmentSegment, StationRange, f64) {
        let index = self
            .segment_ranges
            .iter()
            .position(|range| station_m < range.end_m)
            .unwrap_or(self.segment_ranges.len() - 1);
        let range = self.segment_ranges[index];
        let local_station_m = if station_m <= range.start_m {
            0.0
        } else if station_m >= range.end_m {
            self.segments[index].length()
        } else {
            station_m - range.start_m
        };
        (&self.segments[index], range, local_station_m)
    }

    fn point_at(&self, station_m: f64, policy: &TolerancePolicy) -> Result<Point2, KernelError> {
        let station_m = clamp_station(station_m, self.station_range, policy)?;
        let (segment, _, local_station_m) = self.locate_segment(station_m);
        segment.primitive().point_at(local_station_m, policy)
    }

    fn tangent_at(&self, station_m: f64, policy: &TolerancePolicy) -> Result<Vector2, KernelError> {
        let station_m = clamp_station(station_m, self.station_range, policy)?;
        let (segment, _, local_station_m) = self.locate_segment(station_m);
        segment.primitive().tangent_at(local_station_m, policy)
    }

    fn normal_at(&self, station_m: f64, policy: &TolerancePolicy) -> Result<Vector2, KernelError> {
        let station_m = clamp_station(station_m, self.station_range, policy)?;
        let (segment, _, local_station_m) = self.locate_segment(station_m);
        segment.primitive().normal_at(local_station_m, policy)
    }

    fn project(&self, query: Point2, policy: &TolerancePolicy) -> Result<Projection, KernelError> {
        policy.validate()?;
        if !query.is_finite() {
            return Err(KernelError::NonFiniteInput {
                field: "alignment.query",
            });
        }
        let mut best = None;
        for (segment, range) in self.segments.iter().zip(&self.segment_ranges) {
            let local = segment.primitive().project(query, policy)?;
            let station_m = if local.station_m <= 0.0 {
                range.start_m
            } else if local.station_m >= segment.length() {
                range.end_m
            } else {
                range.start_m + local.station_m
            };
            if !station_m.is_finite() {
                return Err(KernelError::NonFiniteResult {
                    operation: "composite alignment projection station",
                });
            }
            let candidate = Projection { station_m, ..local };
            best = Some(match best {
                Some(current) => Projection::prefer(candidate, current),
                None => candidate,
            });
        }
        best.ok_or(KernelError::ProjectionUndefined)
    }

    fn sample_stations(
        &self,
        options: SamplingOptions,
        policy: &TolerancePolicy,
    ) -> Result<Vec<f64>, KernelError> {
        if self
            .segments
            .len()
            .checked_add(1)
            .is_none_or(|count| count > options.max_points)
        {
            return Err(KernelError::SamplingLimitExceeded);
        }
        let mut stations = Vec::with_capacity(self.segments.len() + 1);
        for (index, (segment, range)) in self.segments.iter().zip(&self.segment_ranges).enumerate()
        {
            if index == 0 {
                stations.push(range.start_m);
            } else if stations.last().copied() != Some(range.start_m) {
                return Err(KernelError::NonFiniteResult {
                    operation: "composite alignment boundary station",
                });
            }
            let start = segment.primitive().point_at(0.0, policy)?;
            let end = segment.primitive().point_at(segment.length(), policy)?;
            let mut context = SampleContext {
                alignment: segment.primitive(),
                options,
                policy,
                stations: &mut stations,
                station_offset: range.start_m,
                local_end_station: segment.length(),
                global_end_station: range.end_m,
            };
            context.subdivide(0.0, start, segment.length(), end, 0)?;
            if let Some(last) = stations.last_mut() {
                *last = range.end_m;
            }
        }
        validate_sample_stations(&stations, self.station_range)?;
        Ok(stations)
    }
}

/// A production alignment with one cumulative station domain and stable,
/// ordered segment identities.
#[derive(Debug, Clone, PartialEq)]
pub struct Alignment {
    composite: CompositeAlignment,
}

impl Alignment {
    /// Construct a one-segment line alignment with the deterministic id
    /// `segment-0`.
    pub fn line(start: Point2, end: Point2, policy: &TolerancePolicy) -> Result<Self, KernelError> {
        let primitive = AlignmentPrimitive::line(start, end, policy)?;
        Self::from_segments(
            vec![AlignmentSegment::new(LEGACY_SEGMENT_ID, primitive, policy)?],
            policy,
        )
    }

    /// Construct a one-segment circular-arc alignment with the deterministic
    /// id `segment-0`.
    pub fn circular_arc(
        center: Point2,
        radius_m: f64,
        start_angle_rad: f64,
        sweep_angle_rad: f64,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        let primitive = AlignmentPrimitive::circular_arc(
            center,
            radius_m,
            start_angle_rad,
            sweep_angle_rad,
            policy,
        )?;
        Self::from_segments(
            vec![AlignmentSegment::new(LEGACY_SEGMENT_ID, primitive, policy)?],
            policy,
        )
    }

    /// Construct a one-segment smooth conceptual curve with the deterministic
    /// id `segment-0`.
    pub fn smooth_curve(
        p0: Point2,
        p1: Point2,
        p2: Point2,
        p3: Point2,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        let primitive = AlignmentPrimitive::smooth_curve(p0, p1, p2, p3, policy)?;
        Self::from_segments(
            vec![AlignmentSegment::new(LEGACY_SEGMENT_ID, primitive, policy)?],
            policy,
        )
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

    /// Construct an authored ordered alignment from stable-id-bearing segments.
    pub fn from_segments(
        segments: Vec<AlignmentSegment>,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        Ok(Self {
            composite: CompositeAlignment::new(segments, policy)?,
        })
    }

    /// Alias for [`Self::from_segments`].
    pub fn composite(
        segments: Vec<AlignmentSegment>,
        policy: &TolerancePolicy,
    ) -> Result<Self, KernelError> {
        Self::from_segments(segments, policy)
    }

    /// Whether this alignment contains multiple authored segments.
    pub fn is_composite(&self) -> bool {
        self.segment_count() > 1
    }

    /// Number of logical authored segments.
    pub fn segment_count(&self) -> usize {
        self.composite.segments.len()
    }

    /// Logical authored segments in stable order.
    ///
    /// The returned values are clones so callers cannot mutate the validated
    /// alignment without rebuilding its cumulative index.
    pub fn segments(&self) -> Vec<AlignmentSegment> {
        self.composite.segments.clone()
    }

    /// Stable logical segment ids in authored order.
    pub fn segment_ids(&self) -> Vec<AlignmentSegmentId> {
        self.segments()
            .into_iter()
            .map(|segment| segment.id)
            .collect()
    }

    /// Exact cumulative station ranges in authored segment order.
    pub fn segment_ranges(&self) -> Vec<StationRange> {
        self.composite.segment_ranges.clone()
    }

    /// Get one logical authored segment by zero-based order.
    pub fn segment(&self, index: usize) -> Option<AlignmentSegment> {
        self.segments().into_iter().nth(index)
    }

    /// Get one exact cumulative station range by zero-based segment order.
    pub fn segment_range(&self, index: usize) -> Option<StationRange> {
        self.segment_ranges().into_iter().nth(index)
    }

    /// Alignment start point.
    pub fn start_point(&self, policy: &TolerancePolicy) -> Result<Point2, KernelError> {
        self.point_at(self.station_range().start_m, policy)
    }

    /// Alignment end point.
    pub fn end_point(&self, policy: &TolerancePolicy) -> Result<Point2, KernelError> {
        self.point_at(self.station_range().end_m, policy)
    }

    /// Alignment kind, returning the primitive family for one segment and
    /// `Composite` for an alignment with multiple segments.
    pub fn kind(&self) -> AlignmentKind {
        if self.segment_count() == 1 {
            self.composite.segments[0].kind()
        } else {
            AlignmentKind::Composite
        }
    }

    /// Total alignment length in metres.
    pub fn length(&self) -> f64 {
        self.composite.length()
    }

    /// Inclusive station domain beginning at zero.
    pub fn station_range(&self) -> StationRange {
        self.composite.station_range()
    }

    /// Point at bounded whole-alignment station `s` in metres.
    pub fn point_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Point2, KernelError> {
        self.composite.point_at(station_m, policy)
    }

    /// Unit tangent at bounded whole-alignment station `s`.
    pub fn tangent_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Vector2, KernelError> {
        self.composite.tangent_at(station_m, policy)
    }

    /// Unit left-hand normal at bounded whole-alignment station `s`.
    pub fn normal_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Vector2, KernelError> {
        self.composite.normal_at(station_m, policy)
    }

    /// Project a finite XY query to the bounded whole-alignment station.
    pub fn project(
        &self,
        query: Point2,
        policy: &TolerancePolicy,
    ) -> Result<Projection, KernelError> {
        self.composite.project(query, policy)
    }

    /// Adaptively sample the whole alignment with a deterministic point budget.
    ///
    /// Every composite segment boundary is emitted exactly once.  The
    /// following segment owns an interior boundary for point/tangent/normal
    /// dispatch; no endpoint is averaged or moved.
    pub fn sample(
        &self,
        options: SamplingOptions,
        policy: &TolerancePolicy,
    ) -> Result<Vec<SamplePoint>, KernelError> {
        policy.validate()?;
        options.validate()?;
        let stations = self.composite.sample_stations(options, policy)?;
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

trait SampleAlignment {
    fn sample_point_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Point2, KernelError>;

    fn sample_chord_error_bound(
        &self,
        start_station: f64,
        start_point: Point2,
        end_station: f64,
        end_point: Point2,
        policy: &TolerancePolicy,
    ) -> Result<f64, KernelError>;
}

impl SampleAlignment for AlignmentPrimitive {
    fn sample_point_at(
        &self,
        station_m: f64,
        policy: &TolerancePolicy,
    ) -> Result<Point2, KernelError> {
        self.point_at(station_m, policy)
    }

    fn sample_chord_error_bound(
        &self,
        start_station: f64,
        start_point: Point2,
        end_station: f64,
        end_point: Point2,
        policy: &TolerancePolicy,
    ) -> Result<f64, KernelError> {
        self.chord_error_bound(start_station, start_point, end_station, end_point, policy)
    }
}

struct SampleContext<'a, A: SampleAlignment + ?Sized> {
    alignment: &'a A,
    options: SamplingOptions,
    policy: &'a TolerancePolicy,
    stations: &'a mut Vec<f64>,
    station_offset: f64,
    local_end_station: f64,
    global_end_station: f64,
}

impl<'a, A: SampleAlignment + ?Sized> SampleContext<'a, A> {
    fn subdivide(
        &mut self,
        start_station: f64,
        start_point: Point2,
        end_station: f64,
        end_point: Point2,
        depth: u8,
    ) -> Result<(), KernelError> {
        let midpoint_station = start_station + (end_station - start_station) / 2.0;
        let midpoint = self
            .alignment
            .sample_point_at(midpoint_station, self.policy)?;
        let station_span = end_station - start_station;
        if !station_span.is_finite() {
            return Err(KernelError::NonFiniteResult {
                operation: "sampling station span",
            });
        }
        let chord_error = self.alignment.sample_chord_error_bound(
            start_station,
            start_point,
            end_station,
            end_point,
            self.policy,
        )?;
        let needs_split = station_span > self.options.max_segment_length_m
            || chord_error > self.options.max_chord_error_m;
        if needs_split {
            if depth >= self.options.max_depth
                || self
                    .stations
                    .len()
                    .checked_add(2)
                    .is_none_or(|count| count > self.options.max_points)
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
            let global_station = if end_station == self.local_end_station {
                self.global_end_station
            } else {
                self.station_offset + end_station
            };
            if !global_station.is_finite() {
                return Err(KernelError::NonFiniteResult {
                    operation: "sampling global station",
                });
            }
            self.stations.push(global_station);
        }
        Ok(())
    }
}

fn validate_sample_stations(
    stations: &[f64],
    station_range: StationRange,
) -> Result<(), KernelError> {
    if stations.len() < 2
        || stations.first().copied() != Some(station_range.start_m)
        || stations.last().copied() != Some(station_range.end_m)
    {
        return Err(KernelError::SamplingLimitExceeded);
    }
    if stations.iter().any(|station_m| !station_m.is_finite())
        || stations.windows(2).any(|pair| pair[1] <= pair[0])
    {
        return Err(KernelError::NonFiniteResult {
            operation: "sampling station order",
        });
    }
    Ok(())
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
