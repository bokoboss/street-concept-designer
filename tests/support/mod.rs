#![allow(dead_code)]

use street_concept_designer_kernel::{
    Alignment, ComponentId, ComponentKind, CrossSection, CrossSectionComponent,
    PiecewiseLinearWidthProfile, Point2, SamplePoint, SamplingOptions, StationRange,
    TolerancePolicy, WidthKnot,
};

pub fn policy() -> TolerancePolicy {
    TolerancePolicy::default()
}

pub fn range(end_m: f64) -> StationRange {
    let policy = policy();
    StationRange::new(0.0, end_m, &policy).expect("fixture station range")
}

pub fn straight_500m() -> Alignment {
    let policy = policy();
    Alignment::line(Point2::new(0.0, 0.0), Point2::new(500.0, 0.0), &policy)
        .expect("straight canonical alignment")
}

pub fn circular_90deg() -> Alignment {
    let policy = policy();
    Alignment::circular_arc(
        Point2::new(0.0, 0.0),
        50.0,
        0.0,
        std::f64::consts::FRAC_PI_2,
        &policy,
    )
    .expect("circular canonical alignment")
}

pub fn smooth_s_curve() -> Alignment {
    let policy = policy();
    Alignment::smooth_curve(
        Point2::new(0.0, 0.0),
        Point2::new(50.0, 100.0),
        Point2::new(100.0, -100.0),
        Point2::new(150.0, 0.0),
        &policy,
    )
    .expect("smooth canonical alignment")
}

pub fn width_profile(range: StationRange, knots: &[(f64, f64)]) -> PiecewiseLinearWidthProfile {
    let policy = policy();
    PiecewiseLinearWidthProfile::new(
        range,
        knots
            .iter()
            .map(|&(station_m, width_m)| WidthKnot { station_m, width_m })
            .collect(),
        &policy,
    )
    .expect("fixture width profile")
}

pub fn lane(id: &str, profile: PiecewiseLinearWidthProfile) -> CrossSectionComponent {
    CrossSectionComponent::traffic_lane(id, profile).expect("fixture traffic lane")
}

pub fn median(id: &str, profile: PiecewiseLinearWidthProfile) -> CrossSectionComponent {
    CrossSectionComponent::median(id, profile).expect("fixture median")
}

pub fn shoulder(id: &str, profile: PiecewiseLinearWidthProfile) -> CrossSectionComponent {
    CrossSectionComponent::shoulder(id, profile).expect("fixture shoulder")
}

pub fn edge_strip(id: &str, profile: PiecewiseLinearWidthProfile) -> CrossSectionComponent {
    CrossSectionComponent::edge_strip(id, profile).expect("fixture edge strip")
}

pub fn straight_two_lane_cross_section() -> CrossSection {
    let policy = policy();
    let range = range(500.0);
    let lane_width =
        PiecewiseLinearWidthProfile::constant(range, 3.5, &policy).expect("two-lane width");
    let shoulder_width =
        PiecewiseLinearWidthProfile::constant(range, 1.5, &policy).expect("shoulder width");
    let edge_width =
        PiecewiseLinearWidthProfile::constant(range, 0.25, &policy).expect("edge-strip width");
    CrossSection::new(
        range,
        vec![
            shoulder("shoulder-left", shoulder_width.clone()),
            lane("lane-1", lane_width.clone()),
            lane("lane-2", lane_width),
            edge_strip("edge-right", edge_width),
            shoulder("shoulder-right", shoulder_width),
        ],
        &policy,
    )
    .expect("straight two-lane cross-section")
}

pub fn divided_four_lane_cross_section() -> CrossSection {
    let policy = policy();
    let range = range(500.0);
    let lane_width =
        PiecewiseLinearWidthProfile::constant(range, 3.5, &policy).expect("four-lane width");
    let median_width =
        PiecewiseLinearWidthProfile::constant(range, 4.0, &policy).expect("median width");
    CrossSection::new(
        range,
        vec![
            lane("lane-1", lane_width.clone()),
            lane("lane-2", lane_width.clone()),
            median("median", median_width),
            lane("lane-3", lane_width.clone()),
            lane("lane-4", lane_width),
        ],
        &policy,
    )
    .expect("divided four-lane cross-section")
}

pub fn right_turn_pocket_cross_section() -> (CrossSection, ComponentId) {
    let policy = policy();
    let range = range(200.0);
    let through =
        PiecewiseLinearWidthProfile::constant(range, 3.5, &policy).expect("through lane width");
    let pocket_profile = width_profile(
        range,
        &[
            (0.0, 0.0),
            (20.0, 0.0),
            (50.0, 3.25),
            (130.0, 3.25),
            (160.0, 0.0),
            (200.0, 0.0),
        ],
    );
    let pocket = lane("lane-right-turn-storage", pocket_profile);
    let pocket_id = pocket.id().clone();
    let cross_section =
        CrossSection::new(range, vec![lane("lane-through", through), pocket], &policy)
            .expect("right-turn pocket cross-section");
    (cross_section, pocket_id)
}

pub fn assert_finite_point(point: Point2) {
    assert!(point.is_finite(), "non-finite point: {point:?}");
}

pub fn assert_sample_chord_error(
    alignment: &Alignment,
    samples: &[SamplePoint],
    options: SamplingOptions,
    policy: &TolerancePolicy,
    probe_count: usize,
) {
    assert!(probe_count > 0);
    let allowed_error = options.max_chord_error_m + policy.station_bound_m * 10.0;
    for (segment_index, pair) in samples.windows(2).enumerate() {
        let start = pair[0];
        let end = pair[1];
        for probe_index in 1..=probe_count {
            let fraction = probe_index as f64 / (probe_count + 1) as f64;
            let station = start.station_m + (end.station_m - start.station_m) * fraction;
            let point = alignment.point_at(station, policy).expect("probe point");
            let deviation = point_segment_distance(point, start.point, end.point);
            assert!(
                deviation <= allowed_error,
                "segment {segment_index} probe {probe_index} exceeded chord error: {deviation} m > {allowed_error} m"
            );
        }
    }
}

fn point_segment_distance(point: Point2, start: Point2, end: Point2) -> f64 {
    let segment = end - start;
    let length = segment.length();
    if length == 0.0 {
        return point.distance_to(start);
    }
    let fraction = ((point - start).dot(segment) / (length * length)).clamp(0.0, 1.0);
    point.distance_to(start + segment * fraction)
}

pub fn assert_finite_component_kind(kind: ComponentKind) {
    assert!(matches!(
        kind,
        ComponentKind::TrafficLane
            | ComponentKind::Median
            | ComponentKind::Shoulder
            | ComponentKind::EdgeStrip
    ));
}
