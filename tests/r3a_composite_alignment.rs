use std::f64::consts::FRAC_PI_2;

use street_concept_designer_kernel::{
    derive_diagnostic_2d, derive_diagnostic_3d, Alignment, AlignmentKind, AlignmentPrimitive,
    AlignmentSegment, ComponentKind, CrossSection, CrossSectionComponent, CrossingRelation,
    CrossingType, DerivedEngineeringSnapshot, JunctionOptions, JunctionStatus, KernelError,
    PiecewiseLinearWidthProfile, Point2, RegenerationResult, Road, RoadNetwork, SamplingOptions,
    TolerancePolicy, WidthKnot,
};

fn policy() -> TolerancePolicy {
    TolerancePolicy::default()
}

fn sampling_options() -> SamplingOptions {
    SamplingOptions {
        max_chord_error_m: 1.0e-3,
        max_segment_length_m: 20.0,
        max_points: 4096,
        max_depth: 20,
    }
}

fn assert_close(actual: f64, expected: f64, tolerance: f64, field: &str) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{field}: {actual:?} != {expected:?} within {tolerance}"
    );
}

fn assert_point_close(actual: Point2, expected: Point2, tolerance: f64, field: &str) {
    assert_close(actual.x, expected.x, tolerance, &format!("{field}.x"));
    assert_close(actual.y, expected.y, tolerance, &format!("{field}.y"));
}

fn line_arc_line() -> Alignment {
    let policy = policy();
    let line_a = AlignmentSegment::line(
        "line-a",
        Point2::new(0.0, 0.0),
        Point2::new(100.0, 0.0),
        &policy,
    )
    .expect("line-a");
    let arc = AlignmentSegment::circular_arc(
        "arc-b",
        Point2::new(100.0, 50.0),
        50.0,
        -FRAC_PI_2,
        FRAC_PI_2,
        &policy,
    )
    .expect("arc-b");
    let line_c = AlignmentSegment::line(
        "line-c",
        Point2::new(150.0, 50.0),
        Point2::new(150.0, 150.0),
        &policy,
    )
    .expect("line-c");
    Alignment::from_segments(vec![line_a, arc, line_c], &policy).expect("line-arc-line")
}

fn line_smooth_line() -> Alignment {
    let policy = policy();
    let line_a = AlignmentSegment::line(
        "line-a",
        Point2::new(0.0, 0.0),
        Point2::new(50.0, 0.0),
        &policy,
    )
    .expect("line-a");
    let curve = AlignmentSegment::smooth_curve(
        "curve-b",
        Point2::new(50.0, 0.0),
        Point2::new(75.0, 0.0),
        Point2::new(100.0, 25.0),
        Point2::new(125.0, 50.0),
        &policy,
    )
    .expect("curve-b");
    let line_c = AlignmentSegment::line(
        "line-c",
        Point2::new(125.0, 50.0),
        Point2::new(175.0, 100.0),
        &policy,
    )
    .expect("line-c");
    Alignment::from_segments(vec![line_a, curve, line_c], &policy).expect("line-smooth-line")
}

fn composite_main() -> Alignment {
    let policy = policy();
    Alignment::from_segments(
        vec![
            AlignmentSegment::line(
                "main-a",
                Point2::new(-100.0, 0.0),
                Point2::new(0.0, 0.0),
                &policy,
            )
            .expect("main-a"),
            AlignmentSegment::line(
                "main-b",
                Point2::new(0.0, 0.0),
                Point2::new(100.0, 0.0),
                &policy,
            )
            .expect("main-b"),
        ],
        &policy,
    )
    .expect("composite main")
}

fn road_for_alignment(id: &str, alignment: Alignment, widths: &[f64]) -> Road {
    let policy = policy();
    let range = alignment.station_range();
    let components = widths
        .iter()
        .enumerate()
        .map(|(index, width)| {
            CrossSectionComponent::new(
                format!("lane-{}", index + 1),
                ComponentKind::TrafficLane,
                PiecewiseLinearWidthProfile::constant(range, *width, &policy)
                    .expect("constant width"),
            )
            .expect("lane")
        })
        .collect();
    let cross_section = CrossSection::new(range, components, &policy).expect("cross-section");
    Road::new(id, alignment, cross_section, &policy).expect("road")
}

fn straight_road(id: &str, start: Point2, end: Point2) -> Road {
    road_for_alignment(
        id,
        Alignment::line(start, end, &policy()).expect("line"),
        &[3.5],
    )
}

#[test]
fn one_segment_convenience_constructors_match_primitive_semantics() {
    let policy = policy();
    let cases = [
        Alignment::line(Point2::new(10.0, 20.0), Point2::new(110.0, 20.0), &policy).expect("line"),
        Alignment::circular_arc(
            Point2::new(100.0, 50.0),
            50.0,
            -FRAC_PI_2,
            FRAC_PI_2,
            &policy,
        )
        .expect("arc"),
        Alignment::smooth_curve(
            Point2::new(0.0, 0.0),
            Point2::new(40.0, 0.0),
            Point2::new(80.0, 30.0),
            Point2::new(120.0, 30.0),
            &policy,
        )
        .expect("curve"),
    ];

    for alignment in cases {
        assert_eq!(alignment.segment_count(), 1);
        assert_eq!(alignment.segment_ids()[0].as_str(), "segment-0");
        assert_eq!(alignment.kind(), alignment.segments()[0].kind());
        assert!(!alignment.is_composite());
        let segment = alignment.segment(0).expect("one segment");
        let primitive = segment.primitive();
        for fraction in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let station = alignment.length() * fraction;
            assert_eq!(
                alignment.point_at(station, &policy).expect("point"),
                primitive
                    .point_at(station, &policy)
                    .expect("primitive point")
            );
            assert_eq!(
                alignment.tangent_at(station, &policy).expect("tangent"),
                primitive
                    .tangent_at(station, &policy)
                    .expect("primitive tangent")
            );
            assert_eq!(
                alignment.normal_at(station, &policy).expect("normal"),
                primitive
                    .normal_at(station, &policy)
                    .expect("primitive normal")
            );
        }
        let query = alignment
            .point_at(alignment.length() * 0.37, &policy)
            .expect("query point");
        assert_eq!(
            alignment.project(query, &policy).expect("projection"),
            primitive
                .project(query, &policy)
                .expect("primitive projection")
        );
        assert_eq!(
            alignment
                .sample(sampling_options(), &policy)
                .expect("sample"),
            {
                let repeated = Alignment::from_segments(vec![segment.clone()], &policy)
                    .expect("repeated one-segment alignment");
                repeated
                    .sample(sampling_options(), &policy)
                    .expect("repeat sample")
            }
        );
    }
}

#[test]
fn canonical_composite_fixtures_preserve_boundaries_and_frames() {
    for alignment in [line_arc_line(), line_smooth_line()] {
        assert_eq!(alignment.kind(), AlignmentKind::Composite);
        assert_eq!(alignment.segment_count(), 3);
        assert!(alignment.is_composite());
        let segments = alignment.segments();
        let ranges = alignment.segment_ranges();
        assert_eq!(segments.len(), 3);
        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0].start_m, 0.0);
        assert_eq!(ranges[2].end_m, alignment.length());
        for (index, pair) in ranges.windows(2).enumerate() {
            assert_eq!(pair[0].end_m, pair[1].start_m);
            let previous_end = segments[index]
                .end_point(&policy())
                .expect("previous endpoint");
            let next_start = segments[index + 1]
                .start_point(&policy())
                .expect("next endpoint");
            assert_point_close(previous_end, next_start, 1.0e-12, "boundary point");
            let previous_tangent = segments[index]
                .end_tangent(&policy())
                .expect("previous tangent");
            let next_tangent = segments[index + 1]
                .start_tangent(&policy())
                .expect("next tangent");
            assert_close(
                previous_tangent.cross(next_tangent).abs(),
                0.0,
                1.0e-12,
                "boundary tangent cross",
            );
            assert!(previous_tangent.dot(next_tangent) > 0.0);
        }

        let samples = alignment
            .sample(sampling_options(), &policy())
            .expect("composite sample");
        assert_eq!(samples.first().expect("first sample").station_m, 0.0);
        assert_eq!(
            samples.last().expect("last sample").station_m,
            alignment.length()
        );
        for pair in samples.windows(2) {
            assert!(pair[1].station_m > pair[0].station_m);
            assert!(pair[0].point.is_finite());
            assert!(pair[0].tangent.is_finite());
            assert!(pair[0].normal.is_finite());
        }
        for range in &ranges[..ranges.len() - 1] {
            assert_eq!(
                samples
                    .iter()
                    .filter(|sample| sample.station_m == range.end_m)
                    .count(),
                1
            );
            assert_point_close(
                alignment
                    .point_at(range.end_m, &policy())
                    .expect("boundary point"),
                samples
                    .iter()
                    .find(|sample| sample.station_m == range.end_m)
                    .expect("boundary sample")
                    .point,
                0.0,
                "exact sampled boundary",
            );
        }
    }
}

#[test]
fn mixed_large_coordinate_and_near_minimum_segments_are_deterministic() {
    let policy = policy();
    let large = Alignment::from_segments(
        vec![
            AlignmentSegment::line(
                "large-a",
                Point2::new(1.0e9 + 0.375, 1.0e9 + 0.625),
                Point2::new(1.0e9 + 80.375, 1.0e9 + 0.625),
                &policy,
            )
            .expect("large-a"),
            AlignmentSegment::line(
                "large-b",
                Point2::new(1.0e9 + 80.375, 1.0e9 + 0.625),
                Point2::new(1.0e9 + 160.375, 1.0e9 + 0.625),
                &policy,
            )
            .expect("large-b"),
        ],
        &policy,
    )
    .expect("large composite");
    assert_eq!(large.segment_ranges()[0].end_m, 80.0);
    assert_point_close(
        large.point_at(80.0, &policy).expect("large boundary point"),
        Point2::new(1.0e9 + 80.375, 1.0e9 + 0.625),
        0.0,
        "large boundary",
    );
    let projection = large
        .project(Point2::new(1.0e9 + 120.375, 1.0e9 + 4.625), &policy)
        .expect("large projection");
    assert_close(
        projection.station_m,
        120.0,
        1.0e-9,
        "large projection station",
    );
    assert!(projection.point.is_finite());

    let near_minimum = Alignment::from_segments(
        vec![
            AlignmentSegment::line(
                "near-min",
                Point2::new(0.0, 0.0),
                Point2::new(1.1e-6, 0.0),
                &policy,
            )
            .expect("near-minimum valid segment"),
            AlignmentSegment::line(
                "remainder",
                Point2::new(1.1e-6, 0.0),
                Point2::new(10.0, 0.0),
                &policy,
            )
            .expect("remainder"),
        ],
        &policy,
    )
    .expect("near-minimum composite");
    assert!(near_minimum.segment_ranges()[0].end_m > policy.minimum_alignment_length_m);
    assert!(near_minimum
        .sample(sampling_options(), &policy)
        .expect("near-minimum sample")
        .iter()
        .any(|sample| sample.station_m == near_minimum.segment_ranges()[0].end_m));
}

#[test]
fn invalid_continuity_identity_and_finite_inputs_are_rejected_atomically() {
    let policy = policy();
    let line = |id: &str, start: Point2, end: Point2| {
        AlignmentSegment::line(id, start, end, &policy).expect("line segment")
    };

    assert!(Alignment::from_segments(
        vec![
            line("near-a", Point2::new(0.0, 0.0), Point2::new(10.0, 0.0)),
            line(
                "near-b",
                Point2::new(10.0 + 0.5e-9, 0.0),
                Point2::new(20.0, 0.0),
            ),
        ],
        &policy,
    )
    .is_ok());

    assert!(matches!(
        Alignment::from_segments(
            vec![
                line("a", Point2::new(0.0, 0.0), Point2::new(10.0, 0.0)),
                line("b", Point2::new(10.0 + 2.0e-9, 0.0), Point2::new(20.0, 0.0)),
            ],
            &policy,
        ),
        Err(KernelError::AlignmentEndpointGap {
            segment_index: 1,
            ..
        })
    ));
    assert!(matches!(
        Alignment::from_segments(
            vec![
                line("a", Point2::new(0.0, 0.0), Point2::new(10.0, 0.0)),
                line("b", Point2::new(10.0, 0.0), Point2::new(10.0, 10.0)),
            ],
            &policy,
        ),
        Err(KernelError::AlignmentTangentDiscontinuity {
            segment_index: 1,
            ..
        })
    ));
    assert!(matches!(
        Alignment::from_segments(
            vec![
                line("a", Point2::new(0.0, 0.0), Point2::new(10.0, 0.0)),
                line("a", Point2::new(10.0, 0.0), Point2::new(20.0, 0.0)),
            ],
            &policy,
        ),
        Err(KernelError::DuplicateAlignmentSegmentId)
    ));
    assert!(matches!(
        AlignmentSegment::line(
            "short",
            Point2::new(0.0, 0.0),
            Point2::new(1.0e-7, 0.0),
            &policy,
        ),
        Err(KernelError::InvalidLength { .. })
    ));
    assert!(matches!(
        AlignmentPrimitive::line(Point2::new(f64::NAN, 0.0), Point2::new(10.0, 0.0), &policy,),
        Err(KernelError::NonFiniteInput { .. })
    ));
    assert!(matches!(
        AlignmentSegment::line("", Point2::new(0.0, 0.0), Point2::new(10.0, 0.0), &policy),
        Err(KernelError::InvalidAlignmentSegmentId)
    ));
    assert!(matches!(
        AlignmentSegment::line(
            "contains whitespace",
            Point2::new(0.0, 0.0),
            Point2::new(10.0, 0.0),
            &policy,
        ),
        Err(KernelError::InvalidAlignmentSegmentId)
    ));
}

#[test]
fn projection_boundary_ties_and_endpoint_bounds_are_repeatable() {
    let policy = policy();
    let alignment = Alignment::from_segments(
        vec![
            AlignmentSegment::line(
                "first",
                Point2::new(0.0, 0.0),
                Point2::new(10.0, 0.0),
                &policy,
            )
            .expect("first"),
            AlignmentSegment::line(
                "second",
                Point2::new(10.0, 0.0),
                Point2::new(20.0, 0.0),
                &policy,
            )
            .expect("second"),
        ],
        &policy,
    )
    .expect("two-line alignment");
    let boundary = alignment.segment_ranges()[0].end_m;
    let exact = alignment
        .project(Point2::new(10.0, 0.0), &policy)
        .expect("exact boundary projection");
    assert_eq!(exact.station_m, boundary);
    for _ in 0..32 {
        assert_eq!(
            alignment
                .project(Point2::new(10.0, 1.0), &policy)
                .expect("equidistant boundary projection"),
            alignment
                .project(Point2::new(10.0, 1.0), &policy)
                .expect("repeat equidistant boundary projection")
        );
    }
    let before = alignment
        .project(Point2::new(-1.0, 1.0), &policy)
        .expect("before endpoint projection");
    assert_eq!(before.station_m, 0.0);
    let after = alignment
        .project(Point2::new(21.0, 1.0), &policy)
        .expect("after endpoint projection");
    assert_eq!(after.station_m, alignment.length());
    assert!((0.0..=alignment.length()).contains(&exact.station_m));
}

#[test]
fn cross_section_lane_lifecycle_and_shared_derivation_include_composite_boundaries() {
    let policy = policy();
    let alignment = line_arc_line();
    let range = alignment.station_range();
    let boundary = alignment.segment_ranges()[0].end_m;
    let taper_end = boundary + 20.0;
    let lane_main = CrossSectionComponent::traffic_lane(
        "lane-main",
        PiecewiseLinearWidthProfile::constant(range, 3.5, &policy).expect("main profile"),
    )
    .expect("main lane");
    let turn_pocket = CrossSectionComponent::traffic_lane(
        "right-turn-pocket",
        PiecewiseLinearWidthProfile::new(
            range,
            vec![
                WidthKnot {
                    station_m: range.start_m,
                    width_m: 0.0,
                },
                WidthKnot {
                    station_m: boundary,
                    width_m: 0.0,
                },
                WidthKnot {
                    station_m: taper_end,
                    width_m: 3.25,
                },
                WidthKnot {
                    station_m: range.end_m,
                    width_m: 3.25,
                },
            ],
            &policy,
        )
        .expect("turn-pocket profile"),
    )
    .expect("turn pocket");
    let cross_section = CrossSection::new(range, vec![lane_main, turn_pocket], &policy)
        .expect("composite cross-section");
    let road = Road::new("composite-road", alignment.clone(), cross_section, &policy)
        .expect("composite road");
    let states_at_boundary = road
        .cross_section()
        .states_at(boundary, &policy)
        .expect("boundary states");
    assert_eq!(states_at_boundary[1].width_m, 0.0);
    assert!(!states_at_boundary[1].active);
    let states_after_boundary = road
        .cross_section()
        .states_at(boundary + 10.0, &policy)
        .expect("post-boundary states");
    assert!(states_after_boundary[1].active);
    assert_close(
        states_after_boundary[1].width_m,
        1.625,
        1.0e-12,
        "turn-pocket taper",
    );

    let mut network = RoadNetwork::new();
    network.add_road(road).expect("add composite road");
    network.validate(&policy).expect("network validates");
    let snapshot = DerivedEngineeringSnapshot::derive(&network, Point2::new(0.0, 0.0), &policy)
        .expect("shared composite derivation");
    snapshot.validate(&policy).expect("snapshot validates");
    let derived_road = &snapshot.roads()[0];
    assert!(derived_road
        .alignment()
        .iter()
        .any(|sample| sample.station_m == boundary));
    let derived_pocket = derived_road
        .components()
        .iter()
        .find(|component| component.component_id().as_str() == "right-turn-pocket")
        .expect("derived turn pocket");
    assert!(derived_pocket
        .samples()
        .iter()
        .any(|sample| sample.station_m() == boundary && sample.width_m() == 0.0));
    assert!(derived_pocket
        .samples()
        .iter()
        .any(|sample| sample.station_m() == taper_end && sample.width_m() == 3.25));
    let diagnostic_2d = derive_diagnostic_2d(&snapshot).expect("2D adapter");
    let diagnostic_3d = derive_diagnostic_3d(&snapshot).expect("3D adapter");
    diagnostic_2d.validate(&policy).expect("2D validates");
    diagnostic_3d.validate().expect("3D validates");
}

#[test]
fn composite_roads_preserve_candidate_junction_and_stale_regeneration_semantics() {
    let policy = policy();
    let mut true_crossing = RoadNetwork::new();
    true_crossing
        .add_road(road_for_alignment("main", composite_main(), &[3.5]))
        .expect("main");
    true_crossing
        .add_road(straight_road(
            "cross",
            Point2::new(0.0, -100.0),
            Point2::new(0.0, 100.0),
        ))
        .expect("cross");
    let candidate = true_crossing
        .detect_candidate(
            &street_concept_designer_kernel::RoadId::new("main").expect("main id"),
            &street_concept_designer_kernel::RoadId::new("cross").expect("cross id"),
            CrossingRelation::AtGrade,
            &policy,
        )
        .expect("candidate detection")
        .expect("true crossing");
    assert_eq!(candidate.crossing_type(), CrossingType::TrueCrossing);
    assert_close(
        candidate.station_a_m(),
        100.0,
        1.0e-9,
        "true crossing station",
    );
    let junction_id = true_crossing
        .create_junction(&candidate, "j-composite", JunctionOptions::new(), &policy)
        .expect("junction creation");
    let junction = true_crossing.junction(&junction_id).expect("junction");
    assert_eq!(junction.status(), JunctionStatus::Fresh);
    assert_eq!(junction.approaches().len(), 4);
    assert!(junction.surface().is_some());

    let mut endpoint_meeting = RoadNetwork::new();
    endpoint_meeting
        .add_road(road_for_alignment("main", composite_main(), &[3.5]))
        .expect("endpoint main");
    endpoint_meeting
        .add_road(straight_road(
            "stem",
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 100.0),
        ))
        .expect("endpoint stem");
    let endpoint_candidate = endpoint_meeting
        .detect_candidate(
            &street_concept_designer_kernel::RoadId::new("main").expect("main id"),
            &street_concept_designer_kernel::RoadId::new("stem").expect("stem id"),
            CrossingRelation::AtGrade,
            &policy,
        )
        .expect("endpoint detection")
        .expect("endpoint meeting");
    assert_eq!(
        endpoint_candidate.crossing_type(),
        CrossingType::EndpointMeeting
    );

    let main = road_for_alignment("main", composite_main(), &[3.5]);
    let replacement_alignment = Alignment::from_segments(
        vec![
            AlignmentSegment::line(
                "main-a",
                Point2::new(-120.0, 0.0),
                Point2::new(0.0, 0.0),
                &policy,
            )
            .expect("replacement main-a"),
            AlignmentSegment::line(
                "main-b",
                Point2::new(0.0, 0.0),
                Point2::new(120.0, 0.0),
                &policy,
            )
            .expect("replacement main-b"),
        ],
        &policy,
    )
    .expect("replacement alignment");
    let replacement = road_for_alignment("main", replacement_alignment, &[3.5]);
    let mut network = RoadNetwork::new();
    network.add_road(main).expect("main road");
    network
        .add_road(straight_road(
            "cross",
            Point2::new(0.0, -100.0),
            Point2::new(0.0, 100.0),
        ))
        .expect("cross road");
    let candidate = network
        .detect_candidate(
            &street_concept_designer_kernel::RoadId::new("main").expect("main id"),
            &street_concept_designer_kernel::RoadId::new("cross").expect("cross id"),
            CrossingRelation::AtGrade,
            &policy,
        )
        .expect("candidate")
        .expect("candidate exists");
    let junction_id = network
        .create_junction(&candidate, "j-regenerate", JunctionOptions::new(), &policy)
        .expect("create");
    let authored_connection = network
        .junction(&junction_id)
        .expect("created junction")
        .lane_connections()
        .first()
        .cloned()
        .expect("automatic connection");
    network
        .junction_mut(&junction_id)
        .expect("junction mutable")
        .replace_lane_connections(vec![authored_connection])
        .expect("manual connection");
    network
        .replace_road(replacement)
        .expect("replace composite road");
    assert_eq!(
        network
            .junction(&junction_id)
            .expect("stale junction")
            .status(),
        JunctionStatus::Stale
    );
    assert!(!network
        .junction(&junction_id)
        .expect("stale junction")
        .authored_lane_connections()
        .is_empty());
    let missing_corner =
        street_concept_designer_kernel::CornerId::new("missing").expect("valid missing corner id");
    assert!(matches!(
        network
            .junction_mut(&junction_id)
            .expect("stale junction mutable")
            .set_corner_radius(&missing_corner, 9.0, &policy),
        Err(KernelError::StaleJunction)
    ));
    assert_eq!(
        network
            .regenerate_junction(&junction_id, &policy)
            .expect("regenerate composite junction"),
        RegenerationResult::Regenerated
    );
    let regenerated = network
        .junction(&junction_id)
        .expect("regenerated junction");
    assert_eq!(regenerated.status(), JunctionStatus::Fresh);
    assert_eq!(
        regenerated.connectivity_mode(),
        street_concept_designer_kernel::LaneConnectivityMode::Manual
    );
    assert_eq!(regenerated.authored_lane_connections().len(), 1);
    assert_eq!(
        network
            .road(&street_concept_designer_kernel::RoadId::new("main").expect("main id"))
            .expect("main")
            .id()
            .as_str(),
        "main"
    );
}

#[test]
fn cumulative_station_and_frame_property_corpus_is_deterministic() {
    let policy = policy();
    let alignment = line_smooth_line();
    let repeated = line_smooth_line();
    assert_eq!(alignment, repeated);
    let range = alignment.station_range();
    for index in 0..=128usize {
        let station = range.end_m * index as f64 / 128.0;
        let point = alignment.point_at(station, &policy).expect("point");
        let tangent = alignment.tangent_at(station, &policy).expect("tangent");
        let normal = alignment.normal_at(station, &policy).expect("normal");
        assert!(point.is_finite());
        assert!(tangent.is_finite());
        assert!(normal.is_finite());
        assert_close(tangent.length(), 1.0, 1.0e-12, "unit tangent");
        assert_close(normal.length(), 1.0, 1.0e-12, "unit normal");
        let projection = alignment
            .project(point + normal * 0.25, &policy)
            .expect("projection");
        assert!((0.0..=range.end_m).contains(&projection.station_m));
        assert!(projection.point.is_finite());
        assert!(projection.distance_m.is_finite());
    }
    let samples = alignment
        .sample(sampling_options(), &policy)
        .expect("property sample");
    assert!(samples
        .windows(2)
        .all(|pair| pair[1].station_m > pair[0].station_m));
    for segment_range in alignment.segment_ranges() {
        assert!(samples
            .iter()
            .any(|sample| sample.station_m == segment_range.start_m));
        assert!(samples
            .iter()
            .any(|sample| sample.station_m == segment_range.end_m));
    }
}
