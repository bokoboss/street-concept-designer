mod support;

use street_concept_designer_kernel::{
    Alignment, AlignmentKind, KernelError, Point2, SamplingOptions,
};

use support::{
    assert_finite_point, assert_sample_chord_error, circular_90deg, policy, smooth_s_curve,
    straight_500m,
};

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, got {actual}, tolerance {tolerance}"
    );
}

#[test]
fn canonical_line_provides_station_queries_and_frame() {
    let policy = policy();
    let alignment =
        Alignment::line(Point2::new(10.0, 20.0), Point2::new(110.0, 20.0), &policy).expect("line");

    assert_eq!(alignment.kind(), AlignmentKind::Line);
    assert_close(alignment.length(), 100.0, 1.0e-12);
    assert_eq!(
        alignment.point_at(0.0, &policy).expect("start"),
        Point2::new(10.0, 20.0)
    );
    assert_eq!(
        alignment.point_at(100.0, &policy).expect("end"),
        Point2::new(110.0, 20.0)
    );
    let point = alignment.point_at(25.0, &policy).expect("point");
    assert_close(point.x, 35.0, 1.0e-12);
    assert_close(point.y, 20.0, 1.0e-12);
    assert_eq!(
        alignment.tangent_at(25.0, &policy).expect("tangent"),
        street_concept_designer_kernel::Vector2::new(1.0, 0.0)
    );
    assert_eq!(
        alignment.normal_at(25.0, &policy).expect("normal"),
        street_concept_designer_kernel::Vector2::new(0.0, 1.0)
    );

    let projection = alignment
        .project(Point2::new(35.0, 23.0), &policy)
        .expect("projection");
    assert_close(projection.station_m, 25.0, 1.0e-12);
    assert_close(projection.distance_m, 3.0, 1.0e-12);
    assert_close(projection.lateral_offset_m, 3.0, 1.0e-12);
}

#[test]
fn canonical_circular_arc_has_analytic_length_and_orientation() {
    let policy = policy();
    let alignment = circular_90deg();
    let radius = 50.0;
    assert_eq!(alignment.kind(), AlignmentKind::CircularArc);
    assert_close(
        alignment.length(),
        radius * std::f64::consts::FRAC_PI_2,
        1.0e-12,
    );
    let start = alignment.point_at(0.0, &policy).expect("arc start");
    assert_close(start.x, radius, 1.0e-12);
    assert_close(start.y, 0.0, 1.0e-12);
    let end = alignment
        .point_at(alignment.length(), &policy)
        .expect("arc end");
    assert_close(end.x, 0.0, 1.0e-12);
    assert_close(end.y, radius, 1.0e-12);
    let tangent = alignment.tangent_at(0.0, &policy).expect("arc tangent");
    assert_close(tangent.x, 0.0, 1.0e-12);
    assert_close(tangent.y, 1.0, 1.0e-12);
    let projection = alignment
        .project(Point2::new(35.0, 35.0), &policy)
        .expect("arc projection");
    assert_close(
        projection.station_m,
        radius * std::f64::consts::FRAC_PI_4,
        1.0e-9,
    );
    assert_close(
        projection.distance_m,
        radius - 35.0 * 2.0_f64.sqrt(),
        1.0e-9,
    );
}

#[test]
fn clockwise_arc_and_center_query_have_deterministic_frames() {
    let policy = policy();
    let alignment = Alignment::circular_arc(
        Point2::new(0.0, 0.0),
        10.0,
        0.0,
        -std::f64::consts::FRAC_PI_2,
        &policy,
    )
    .expect("clockwise arc");
    let tangent = alignment
        .tangent_at(0.0, &policy)
        .expect("clockwise tangent");
    assert_close(tangent.x, 0.0, 1.0e-12);
    assert_close(tangent.y, -1.0, 1.0e-12);
    let center_projection = alignment
        .project(Point2::new(0.0, 0.0), &policy)
        .expect("center projection");
    assert_eq!(center_projection.station_m, 0.0);
    assert_close(center_projection.distance_m, 10.0, 1.0e-12);
}

#[test]
fn canonical_s_curve_is_stationed_by_deterministic_arc_length_table() {
    let policy = policy();
    let alignment = smooth_s_curve();
    assert_eq!(alignment.kind(), AlignmentKind::SmoothConceptualCurve);
    assert!(alignment.length() > 150.0);
    assert_eq!(
        alignment.point_at(0.0, &policy).expect("curve start"),
        Point2::new(0.0, 0.0)
    );
    assert_eq!(
        alignment
            .point_at(alignment.length(), &policy)
            .expect("curve end"),
        Point2::new(150.0, 0.0)
    );
    for fraction in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let station = alignment.length() * fraction;
        let point = alignment.point_at(station, &policy).expect("curve point");
        let tangent = alignment
            .tangent_at(station, &policy)
            .expect("curve tangent");
        let normal = alignment.normal_at(station, &policy).expect("curve normal");
        assert_finite_point(point);
        assert_close(tangent.length(), 1.0, 1.0e-12);
        assert_close(normal.length(), 1.0, 1.0e-12);
        assert_close(tangent.dot(normal), 0.0, 1.0e-12);
    }
    let projection = alignment
        .project(Point2::new(75.0, 0.0), &policy)
        .expect("curve projection");
    assert!(projection.station_m >= 0.0 && projection.station_m <= alignment.length());
    assert_finite_point(projection.point);
}

#[test]
fn symmetric_inflection_sampling_respects_the_chord_error_budget() {
    let policy = policy();
    let alignment = Alignment::smooth_curve(
        Point2::new(0.0, 0.0),
        Point2::new(0.0, 1.0),
        Point2::new(1.0, -1.0),
        Point2::new(1.0, 0.0),
        &policy,
    )
    .expect("symmetric inflection");
    let options = SamplingOptions {
        max_chord_error_m: 0.001,
        max_segment_length_m: 10.0,
        max_points: 4096,
        max_depth: 20,
    };
    let samples = alignment.sample(options, &policy).expect("samples");

    assert!(samples.len() > 2, "the inflection must be subdivided");
    assert_close(alignment.length(), 1.659, 0.01);
    // Sixteen evenly spaced probes per returned interval cover the known
    // quarter-region extrema; the margin covers double-rounding only.
    assert_sample_chord_error(&alignment, &samples, options, &policy, 16);
}

#[test]
fn full_turn_arcs_construct_and_sample_in_both_directions() {
    let policy = policy();
    let options = SamplingOptions {
        max_chord_error_m: 0.01,
        max_segment_length_m: 20.0,
        max_points: 4096,
        max_depth: 20,
    };
    for sweep in [std::f64::consts::TAU, -std::f64::consts::TAU] {
        let alignment =
            Alignment::circular_arc(Point2::new(10.0, -20.0), 20.0, 0.25, sweep, &policy)
                .expect("full-turn arc");
        assert_eq!(alignment.kind(), AlignmentKind::CircularArc);
        assert_close(alignment.length(), 20.0 * std::f64::consts::TAU, 1.0e-12);

        for fraction in [0.0, 0.37, 0.5, 1.0] {
            let station = alignment.length() * fraction;
            let point = alignment
                .point_at(station, &policy)
                .expect("full-turn point");
            let tangent = alignment
                .tangent_at(station, &policy)
                .expect("full-turn tangent");
            let normal = alignment
                .normal_at(station, &policy)
                .expect("full-turn normal");
            assert_finite_point(point);
            assert_close(tangent.length(), 1.0, 1.0e-12);
            assert_close(normal.length(), 1.0, 1.0e-12);
            let projection = alignment
                .project(point, &policy)
                .expect("full-turn projection");
            assert!(projection.station_m >= 0.0 && projection.station_m <= alignment.length());
            assert_finite_point(projection.point);
        }

        let samples = alignment
            .sample(options, &policy)
            .expect("full-turn samples");
        assert!(samples.len() > 2);
        assert_sample_chord_error(&alignment, &samples, options, &policy, 4);
        assert_eq!(
            samples,
            alignment.sample(options, &policy).expect("repeat samples")
        );
    }
}

#[test]
fn sampling_is_adaptive_bounded_and_station_monotonic() {
    let policy = policy();
    let options = SamplingOptions {
        max_chord_error_m: 0.001,
        max_segment_length_m: 25.0,
        max_points: 4096,
        max_depth: 20,
    };
    for alignment in [straight_500m(), circular_90deg(), smooth_s_curve()] {
        let samples = alignment.sample(options, &policy).expect("samples");
        assert!(samples.len() >= 2);
        assert_eq!(samples.first().expect("first").station_m, 0.0);
        assert_eq!(samples.last().expect("last").station_m, alignment.length());
        for pair in samples.windows(2) {
            assert!(pair[1].station_m > pair[0].station_m);
            assert!(pair[1].station_m - pair[0].station_m <= 25.0 + 1.0e-12);
            assert_finite_point(pair[0].point);
            assert_close(pair[0].tangent.length(), 1.0, 1.0e-10);
            assert_close(pair[0].normal.length(), 1.0, 1.0e-10);
        }
        assert_sample_chord_error(&alignment, &samples, options, &policy, 8);
    }
}

#[test]
fn invalid_and_out_of_domain_alignment_inputs_are_explicitly_rejected() {
    let policy = policy();
    assert!(matches!(
        Alignment::line(Point2::new(0.0, 0.0), Point2::new(0.0, 0.0), &policy),
        Err(KernelError::InvalidLength { .. })
    ));
    assert!(matches!(
        Alignment::line(Point2::new(f64::NAN, 0.0), Point2::new(1.0, 0.0), &policy),
        Err(KernelError::NonFiniteInput { .. })
    ));
    assert!(matches!(
        Alignment::circular_arc(
            Point2::new(0.0, 0.0),
            10.0,
            0.0,
            std::f64::consts::TAU * 1.5,
            &policy
        ),
        Err(KernelError::InvalidParameter { .. })
    ));
    let line = straight_500m();
    assert!(matches!(
        line.point_at(-1.0, &policy),
        Err(KernelError::StationOutOfBounds { .. })
    ));
    assert!(matches!(
        line.point_at(f64::INFINITY, &policy),
        Err(KernelError::NonFiniteInput { .. })
    ));
}
