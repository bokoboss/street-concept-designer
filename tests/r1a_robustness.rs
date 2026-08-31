mod support;

use street_concept_designer_kernel::{
    Alignment, KernelError, Point2, SamplingOptions, TolerancePolicy,
};

use support::{assert_sample_chord_error, circular_90deg, policy, smooth_s_curve, straight_500m};

#[derive(Clone, Copy)]
struct DeterministicRng(u64);

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0
    }

    fn unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    fn between(&mut self, low: f64, high: f64) -> f64 {
        low + (high - low) * self.unit()
    }
}

fn assert_finite_projection(alignment: &Alignment, query: Point2) {
    let policy = policy();
    match alignment.project(query, &policy) {
        Ok(projection) => {
            assert!(projection.station_m.is_finite());
            assert!(projection.distance_m.is_finite());
            assert!(projection.lateral_offset_m.is_finite());
            assert!(projection.point.is_finite());
            assert!(projection.station_m >= 0.0);
            assert!(projection.station_m <= alignment.length());
        }
        Err(KernelError::NonFiniteResult { operation }) => {
            panic!("finite fixture produced non-finite result: {operation}");
        }
        Err(error) => panic!("finite fixture unexpectedly rejected: {error}"),
    }
}

fn adversarial_smooth_curves(policy: &TolerancePolicy) -> Vec<Alignment> {
    let named = [
        (
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 1.0),
            Point2::new(1.0, -1.0),
            Point2::new(1.0, 0.0),
        ),
        (
            Point2::new(0.0, 0.0),
            Point2::new(10.0, 60.0),
            Point2::new(80.0, -20.0),
            Point2::new(140.0, 10.0),
        ),
        (
            Point2::new(0.0, 0.0),
            Point2::new(40.0, 0.5),
            Point2::new(80.0, -0.5),
            Point2::new(120.0, 0.0),
        ),
        (
            Point2::new(0.0, 0.0),
            Point2::new(2.0, 50.0),
            Point2::new(30.0, -50.0),
            Point2::new(60.0, 0.0),
        ),
        (
            Point2::new(0.0, 0.0),
            Point2::new(1.0e-4, 1.0e-4),
            Point2::new(99.9999, -1.0e-4),
            Point2::new(100.0, 0.0),
        ),
    ];
    named
        .into_iter()
        .map(|(p0, p1, p2, p3)| Alignment::smooth_curve(p0, p1, p2, p3, policy))
        .collect::<Result<Vec<_>, _>>()
        .expect("named smooth adversarial curves")
}

#[test]
fn generated_smooth_curves_preserve_finite_bounded_and_error_bounded_behavior() {
    let policy = policy();
    let options = SamplingOptions {
        max_chord_error_m: 0.01,
        max_segment_length_m: 10.0,
        max_points: 4096,
        max_depth: 20,
    };
    let mut rng = DeterministicRng::new(0x534d_4f4f_5448_5253);
    let mut cases = adversarial_smooth_curves(&policy);
    for _ in 0..64 {
        let length = rng.between(20.0, 220.0);
        let p0 = Point2::new(rng.between(-1.0e5, 1.0e5), rng.between(-1.0e5, 1.0e5));
        let p3 = Point2::new(p0.x + length, p0.y + rng.between(-20.0, 20.0));
        let p1 = Point2::new(
            p0.x + length * rng.between(0.1, 0.4),
            p0.y + rng.between(-80.0, 80.0),
        );
        let p2 = Point2::new(
            p0.x + length * rng.between(0.6, 0.9),
            p3.y + rng.between(-80.0, 80.0),
        );
        cases.push(
            Alignment::smooth_curve(p0, p1, p2, p3, &policy).expect("generated smooth curve"),
        );
    }

    for (index, alignment) in cases.iter().enumerate() {
        assert!(alignment.length().is_finite());
        assert!(alignment.length() > 0.0);
        let samples = alignment.sample(options, &policy).expect("smooth samples");
        assert!(samples.len() >= 2, "case {index} needs endpoints");
        assert_eq!(samples.first().expect("first sample").station_m, 0.0);
        assert_eq!(
            samples.last().expect("last sample").station_m,
            alignment.length()
        );
        for pair in samples.windows(2) {
            assert!(pair[0].station_m < pair[1].station_m);
            assert!(
                pair[1].station_m - pair[0].station_m
                    <= options.max_segment_length_m + policy.station_bound_m * 10.0
            );
            for sample in pair {
                assert!(sample.point.is_finite());
                assert!(sample.tangent.is_finite());
                assert!(sample.normal.is_finite());
            }
        }
        assert_sample_chord_error(alignment, &samples, options, &policy, 8);
        assert_eq!(
            samples,
            alignment
                .sample(options, &policy)
                .expect("repeat smooth samples")
        );
        for fraction in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let station = alignment.length() * fraction;
            let point = alignment.point_at(station, &policy).expect("smooth point");
            let tangent = alignment
                .tangent_at(station, &policy)
                .expect("smooth tangent");
            let normal = alignment
                .normal_at(station, &policy)
                .expect("smooth normal");
            assert!(point.is_finite());
            assert!(tangent.is_finite());
            assert!(normal.is_finite());
            assert_finite_projection(alignment, point);
        }
    }
}

#[test]
fn property_style_alignment_sweep_preserves_finite_monotonic_results() {
    let policy = policy();
    let options = SamplingOptions {
        max_chord_error_m: 0.01,
        max_segment_length_m: 50.0,
        max_points: 4096,
        max_depth: 20,
    };
    let mut rng = DeterministicRng::new(0x5241_7072_6f70_6572);

    for _ in 0..256 {
        let start = Point2::new(rng.between(-1.0e6, 1.0e6), rng.between(-1.0e6, 1.0e6));
        let angle = rng.between(-std::f64::consts::PI, std::f64::consts::PI);
        let length = rng.between(0.01, 2_000.0);
        let end = Point2::new(
            start.x + length * angle.cos(),
            start.y + length * angle.sin(),
        );
        let alignment = Alignment::line(start, end, &policy).expect("generated line");
        let samples = alignment.sample(options, &policy).expect("line samples");
        for pair in samples.windows(2) {
            assert!(pair[0].station_m < pair[1].station_m);
            assert!(pair[0].point.is_finite());
            assert!(pair[1].point.is_finite());
        }
        let station = rng.between(0.0, length);
        let point = alignment.point_at(station, &policy).expect("line point");
        assert_finite_projection(&alignment, point);
    }

    for _ in 0..128 {
        let radius = rng.between(0.1, 1_000.0);
        let start_angle = rng.between(-std::f64::consts::PI, std::f64::consts::PI);
        let sweep = rng.between(0.01, std::f64::consts::PI * 1.9);
        let alignment = Alignment::circular_arc(
            Point2::new(rng.between(-1.0e6, 1.0e6), rng.between(-1.0e6, 1.0e6)),
            radius,
            start_angle,
            if rng.unit() > 0.5 { sweep } else { -sweep },
            &policy,
        )
        .expect("generated arc");
        let samples = alignment.sample(options, &policy).expect("arc samples");
        assert!(samples.iter().all(|sample| sample.point.is_finite()));
        let point = alignment
            .point_at(rng.between(0.0, alignment.length()), &policy)
            .expect("arc point");
        assert_finite_projection(&alignment, point);
    }
}

#[test]
fn fuzz_style_finite_query_corpus_does_not_panic_or_emit_non_finite_results() {
    let policy = policy();
    let alignments = [straight_500m(), circular_90deg(), smooth_s_curve()];
    let mut rng = DeterministicRng::new(0x0046_555a_5a5f_7231);
    for alignment in &alignments {
        for _ in 0..10_000 {
            let query = Point2::new(rng.between(-1.0e6, 1.0e6), rng.between(-1.0e6, 1.0e6));
            let result = std::panic::catch_unwind(|| alignment.project(query, &policy));
            let projection_result = result.expect("projection must not panic");
            assert_finite_projection_result(projection_result);
        }
    }
}

fn assert_finite_projection_result(
    result: Result<street_concept_designer_kernel::Projection, KernelError>,
) {
    match result {
        Ok(projection) => {
            assert!(projection.point.is_finite());
            assert!(projection.station_m.is_finite());
            assert!(projection.distance_m.is_finite());
            assert!(projection.lateral_offset_m.is_finite());
        }
        Err(KernelError::NonFiniteResult { operation }) => {
            panic!("finite query emitted non-finite result: {operation}");
        }
        Err(error) => panic!("finite query unexpectedly failed: {error}"),
    }
}

#[test]
fn adversarial_millimetre_short_large_coordinate_and_near_parallel_cases_are_controlled() {
    let policy = policy();
    let millimetre = Alignment::line(Point2::new(0.0, 0.0), Point2::new(0.001, 0.0), &policy)
        .expect("one millimetre alignment remains representable");
    assert!(millimetre.length() > policy.minimum_alignment_length_m);
    let near_parallel = Alignment::line(
        Point2::new(1.0e9, -1.0e9),
        Point2::new(1.0e9 + 1_000.0, -1.0e9 + 1.0e-6),
        &policy,
    )
    .expect("near-parallel alignment");
    let near_coincident_endpoint = near_parallel
        .point_at(near_parallel.length(), &policy)
        .expect("large endpoint");
    assert!(near_coincident_endpoint.is_finite());
    let projection = near_parallel
        .project(Point2::new(1.0e9 + 500.0, -1.0e9 + 0.001), &policy)
        .expect("large-coordinate projection");
    assert!(projection.station_m >= 0.0 && projection.station_m <= near_parallel.length());
    assert!(projection.distance_m.is_finite());

    let short_rejected = Alignment::line(
        Point2::new(0.0, 0.0),
        Point2::new(policy.minimum_alignment_length_m / 2.0, 0.0),
        &policy,
    );
    assert!(matches!(
        short_rejected,
        Err(KernelError::InvalidLength { .. })
    ));
    assert!(matches!(
        millimetre.project(Point2::new(f64::NAN, 0.0), &policy),
        Err(KernelError::NonFiniteInput { .. })
    ));
}
