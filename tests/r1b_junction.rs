use street_concept_designer_kernel::{
    Alignment, ApproachId, CandidateDisposition, ComponentId, ComponentKind, CornerId,
    CrossSection, CrossSectionComponent, CrossingRelation, CrossingType, JunctionOptions,
    JunctionStatus, LaneConnection, Movement, PavementSurface, PiecewiseLinearWidthProfile, Point2,
    RegenerationResult, Road, RoadId, RoadNetwork, StationRange, TolerancePolicy, WidthKnot,
};

fn policy() -> TolerancePolicy {
    TolerancePolicy::default()
}

fn line_road(id: &str, start: Point2, end: Point2, widths: &[f64]) -> Road {
    let policy = policy();
    let alignment = Alignment::line(start, end, &policy).expect("line alignment");
    road_for_alignment(id, alignment, widths)
}

fn road_for_alignment(id: &str, alignment: Alignment, widths: &[f64]) -> Road {
    let policy = policy();
    let range = alignment.station_range();
    let components = widths
        .iter()
        .enumerate()
        .map(|(index, width)| {
            let profile = PiecewiseLinearWidthProfile::constant(range, *width, &policy)
                .expect("constant width profile");
            CrossSectionComponent::new(
                format!("lane-{}", index + 1),
                ComponentKind::TrafficLane,
                profile,
            )
            .expect("traffic lane")
        })
        .collect();
    let cross_section = CrossSection::new(range, components, &policy).expect("cross-section");
    Road::new(id, alignment, cross_section, &policy).expect("road")
}

fn road_with_median(id: &str, start: Point2, end: Point2) -> Road {
    let policy = policy();
    let alignment = Alignment::line(start, end, &policy).expect("line alignment");
    let range = alignment.station_range();
    let lane = |id: &str, width: f64| {
        CrossSectionComponent::new(
            id,
            ComponentKind::TrafficLane,
            PiecewiseLinearWidthProfile::constant(range, width, &policy).expect("lane profile"),
        )
        .expect("lane")
    };
    let median = CrossSectionComponent::new(
        "median",
        ComponentKind::Median,
        PiecewiseLinearWidthProfile::constant(range, 1.0, &policy).expect("median profile"),
    )
    .expect("median");
    let cross_section = CrossSection::new(
        range,
        vec![lane("lane-left", 3.5), median, lane("lane-right", 3.5)],
        &policy,
    )
    .expect("divided cross-section");
    Road::new(id, alignment, cross_section, &policy).expect("divided road")
}

fn id(value: &str) -> RoadId {
    RoadId::new(value).expect("road id")
}

fn create(
    network: &mut RoadNetwork,
    road_a: &str,
    road_b: &str,
    junction_id: &str,
    options: JunctionOptions,
) -> street_concept_designer_kernel::JunctionId {
    let candidate = network
        .detect_candidate(
            &id(road_a),
            &id(road_b),
            CrossingRelation::AtGrade,
            &policy(),
        )
        .expect("candidate detection")
        .expect("candidate exists");
    network
        .create_junction(&candidate, junction_id, options, &policy())
        .expect("junction creation")
}

#[test]
fn candidate_detection_is_inert_and_classifies_required_geometry() {
    let mut t_network = RoadNetwork::new();
    t_network
        .add_road(line_road(
            "main",
            Point2::new(-100.0, 0.0),
            Point2::new(100.0, 0.0),
            &[3.5],
        ))
        .expect("main");
    t_network
        .add_road(line_road(
            "stem",
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 100.0),
            &[3.5],
        ))
        .expect("stem");
    let t_candidate = t_network
        .detect_candidate(
            &id("main"),
            &id("stem"),
            CrossingRelation::AtGrade,
            &policy(),
        )
        .expect("T candidate")
        .expect("T candidate exists");
    assert_eq!(t_candidate.crossing_type(), CrossingType::EndpointMeeting);
    assert!(t_network.junctions().is_empty());
    assert!(t_network.connected_road_ids().is_empty());

    let mut x_network = RoadNetwork::new();
    x_network
        .add_road(line_road(
            "east-west",
            Point2::new(-100.0, 0.0),
            Point2::new(100.0, 0.0),
            &[3.5, 3.5],
        ))
        .expect("east-west");
    x_network
        .add_road(line_road(
            "north-south",
            Point2::new(0.0, -100.0),
            Point2::new(0.0, 100.0),
            &[3.5, 3.5],
        ))
        .expect("north-south");
    let x_candidate = x_network
        .detect_candidate(
            &id("east-west"),
            &id("north-south"),
            CrossingRelation::AtGrade,
            &policy(),
        )
        .expect("X candidate")
        .expect("X candidate exists");
    assert_eq!(x_candidate.crossing_type(), CrossingType::TrueCrossing);
    assert_eq!(x_network.junctions().len(), 0);
}

#[test]
fn t_and_four_leg_junctions_have_valid_deterministic_derived_geometry() {
    let mut t_network = RoadNetwork::new();
    t_network
        .add_road(line_road(
            "main",
            Point2::new(-100.0, 0.0),
            Point2::new(100.0, 0.0),
            &[3.5],
        ))
        .expect("main");
    t_network
        .add_road(line_road(
            "stem",
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 100.0),
            &[5.0],
        ))
        .expect("stem");
    let t_id = create(
        &mut t_network,
        "main",
        "stem",
        "j-t",
        JunctionOptions::new(),
    );
    let t_junction = t_network.junction(&t_id).expect("T junction");
    assert_eq!(t_junction.approaches().len(), 3);
    assert_eq!(t_junction.corners().len(), 3);
    assert_eq!(t_junction.status(), JunctionStatus::Fresh);
    let t_surface = t_junction.surface().expect("T surface");
    t_surface.validate(&policy()).expect("valid T surface");
    assert!(t_surface.area_m2().expect("T area") > 0.0);
    assert!(!t_junction.lane_connections().is_empty());

    let mut x_network = RoadNetwork::new();
    x_network
        .add_road(line_road(
            "east-west",
            Point2::new(-100.0, 0.0),
            Point2::new(100.0, 0.0),
            &[3.5, 3.5],
        ))
        .expect("east-west");
    x_network
        .add_road(line_road(
            "north-south",
            Point2::new(0.0, -100.0),
            Point2::new(0.0, 100.0),
            &[3.5, 3.5],
        ))
        .expect("north-south");
    let x_id = create(
        &mut x_network,
        "east-west",
        "north-south",
        "j-x",
        JunctionOptions::new(),
    );
    let x_junction = x_network.junction(&x_id).expect("X junction");
    assert_eq!(x_junction.approaches().len(), 4);
    assert_eq!(x_junction.corners().len(), 4);
    let x_surface = x_junction.surface().expect("X surface");
    x_surface.validate(&policy()).expect("valid X surface");
    assert!(x_surface.signed_area_m2().expect("X signed area") > 0.0);
    assert!(x_junction
        .movement_categories()
        .iter()
        .all(|movement| matches!(
            movement,
            Movement::Left | Movement::Through | Movement::Right
        )));
}

#[test]
fn skewed_t_fixture_keeps_station_side_semantics_and_valid_surface() {
    let mut network = RoadNetwork::new();
    network
        .add_road(line_road(
            "main",
            Point2::new(-150.0, 0.0),
            Point2::new(150.0, 0.0),
            &[3.5, 3.5],
        ))
        .expect("main");
    network
        .add_road(line_road(
            "skew-stem",
            Point2::new(0.0, 0.0),
            Point2::new(100.0, 70.0),
            &[4.0],
        ))
        .expect("skew stem");
    let candidate = network
        .detect_candidate(
            &id("main"),
            &id("skew-stem"),
            CrossingRelation::AtGrade,
            &policy(),
        )
        .unwrap()
        .unwrap();
    assert_eq!(candidate.crossing_type(), CrossingType::EndpointMeeting);
    let junction_id = network
        .create_junction(&candidate, "j-skew-t", JunctionOptions::new(), &policy())
        .unwrap();
    let junction = network.junction(&junction_id).unwrap();
    assert_eq!(junction.approaches().len(), 3);
    assert!(junction
        .approaches()
        .iter()
        .any(|approach| approach.road_id() == &id("skew-stem")
            && approach.side() == street_concept_designer_kernel::ApproachSide::End));
    junction.surface().unwrap().validate(&policy()).unwrap();
}

#[test]
fn curved_alignment_candidates_retain_station_and_tangent_semantics() {
    let policy = policy();
    let mut network = RoadNetwork::new();
    network
        .add_road(line_road(
            "line",
            Point2::new(-100.0, 0.0),
            Point2::new(100.0, 0.0),
            &[3.5],
        ))
        .unwrap();
    let arc = Alignment::circular_arc(
        Point2::new(0.0, 0.0),
        50.0,
        -std::f64::consts::FRAC_PI_2,
        std::f64::consts::PI,
        &policy,
    )
    .unwrap();
    network
        .add_road(road_for_alignment("arc", arc, &[3.5]))
        .unwrap();
    let candidate = network
        .detect_candidate(&id("line"), &id("arc"), CrossingRelation::AtGrade, &policy)
        .unwrap()
        .unwrap();
    assert_eq!(candidate.crossing_type(), CrossingType::TrueCrossing);
    assert!((candidate.point().x - 50.0).abs() < 0.01);
    let junction_id = network
        .create_junction(&candidate, "j-arc", JunctionOptions::new(), &policy)
        .unwrap();
    let junction = network.junction(&junction_id).unwrap();
    assert_eq!(junction.approaches().len(), 4);
    junction.surface().unwrap().validate(&policy).unwrap();
}

#[test]
fn skewed_and_unequal_width_junctions_remain_finite_and_valid() {
    let mut network = RoadNetwork::new();
    network
        .add_road(line_road(
            "arterial",
            Point2::new(-150.0, 0.0),
            Point2::new(150.0, 0.0),
            &[4.0, 4.0],
        ))
        .expect("arterial");
    network
        .add_road(line_road(
            "skew",
            Point2::new(-75.0, -100.0),
            Point2::new(100.0, 100.0),
            &[2.5],
        ))
        .expect("skew");
    let junction_id = create(
        &mut network,
        "arterial",
        "skew",
        "j-skew",
        JunctionOptions::new().with_corner_radii(vec![8.0, 12.0, 10.0, 14.0]),
    );
    let junction = network.junction(&junction_id).expect("skew junction");
    assert_eq!(junction.approaches().len(), 4);
    assert_eq!(junction.corner_radii_m(), vec![8.0, 12.0, 10.0, 14.0]);
    let surface = junction.surface().expect("skew surface");
    assert!(surface.vertices().iter().all(|point| point.is_finite()));
    surface.validate(&policy()).expect("valid skew surface");
}

#[test]
fn corner_radii_are_independent_and_have_stable_ids() {
    let mut network = RoadNetwork::new();
    network
        .add_road(line_road(
            "a",
            Point2::new(-100.0, 0.0),
            Point2::new(100.0, 0.0),
            &[3.5],
        ))
        .expect("a");
    network
        .add_road(line_road(
            "b",
            Point2::new(0.0, -100.0),
            Point2::new(0.0, 100.0),
            &[3.5],
        ))
        .expect("b");
    let junction_id = create(
        &mut network,
        "a",
        "b",
        "j-corners",
        JunctionOptions::new().with_corner_radii(vec![8.0, 12.0, 10.0, 14.0]),
    );
    let before = network
        .junction(&junction_id)
        .expect("junction")
        .corners()
        .iter()
        .map(|corner| (corner.id().clone(), corner.radius_m()))
        .collect::<Vec<_>>();
    assert_eq!(before.len(), 4);
    assert!(before
        .iter()
        .enumerate()
        .all(|(index, (_, radius))| *radius == [8.0, 12.0, 10.0, 14.0][index]));
    let target_id = before[1].0.clone();
    let old_area = network
        .junction(&junction_id)
        .and_then(|junction| junction.surface())
        .expect("surface")
        .area_m2()
        .expect("area");
    network
        .junction_mut(&junction_id)
        .expect("mutable junction")
        .set_corner_radius(&target_id, 20.0, &policy())
        .expect("corner edit");
    let after = network.junction(&junction_id).expect("junction after edit");
    assert_eq!(after.corners()[1].id(), &target_id);
    assert_eq!(after.corners()[1].radius_m(), 20.0);
    assert_eq!(after.corners()[0].radius_m(), before[0].1);
    assert_ne!(
        after
            .surface()
            .expect("surface after edit")
            .area_m2()
            .unwrap(),
        old_area
    );
    assert!(after
        .corners()
        .iter()
        .all(|corner| !corner.arc_points().is_empty()));
}

#[test]
fn ignore_and_grade_separation_never_create_topology() {
    let mut network = RoadNetwork::new();
    network
        .add_road(line_road(
            "a",
            Point2::new(-50.0, 0.0),
            Point2::new(50.0, 0.0),
            &[3.5],
        ))
        .expect("a");
    network
        .add_road(line_road(
            "b",
            Point2::new(0.0, -50.0),
            Point2::new(0.0, 50.0),
            &[3.5],
        ))
        .expect("b");
    let candidate = network
        .detect_candidate(&id("a"), &id("b"), CrossingRelation::AtGrade, &policy())
        .unwrap()
        .unwrap();
    let ignored = candidate
        .clone()
        .with_disposition(CandidateDisposition::Ignore);
    assert!(network
        .create_junction(&ignored, "ignored", JunctionOptions::new(), &policy())
        .is_err());
    assert!(network.junctions().is_empty());
    let grade_separated = network
        .detect_candidate(
            &id("a"),
            &id("b"),
            CrossingRelation::GradeSeparated,
            &policy(),
        )
        .unwrap()
        .unwrap();
    assert!(grade_separated.is_grade_separated());
    assert!(network
        .create_junction(
            &grade_separated,
            "grade-separated",
            JunctionOptions::new(),
            &policy(),
        )
        .is_err());
    assert!(network.junctions().is_empty());
}

#[test]
fn endpoint_meeting_and_near_miss_are_distinct() {
    let mut endpoint = RoadNetwork::new();
    endpoint
        .add_road(line_road(
            "a",
            Point2::new(-10.0, 0.0),
            Point2::new(0.0, 0.0),
            &[3.5],
        ))
        .unwrap();
    endpoint
        .add_road(line_road(
            "b",
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 10.0),
            &[3.5],
        ))
        .unwrap();
    assert_eq!(
        endpoint
            .detect_candidate(&id("a"), &id("b"), CrossingRelation::AtGrade, &policy())
            .unwrap()
            .unwrap()
            .crossing_type(),
        CrossingType::EndpointMeeting
    );

    let mut near_miss = RoadNetwork::new();
    near_miss
        .add_road(line_road(
            "a",
            Point2::new(-10.0, 0.0),
            Point2::new(10.0, 0.0),
            &[3.5],
        ))
        .unwrap();
    near_miss
        .add_road(line_road(
            "b",
            Point2::new(0.0, 0.001),
            Point2::new(0.0, 10.0),
            &[3.5],
        ))
        .unwrap();
    assert!(near_miss
        .detect_candidate(&id("a"), &id("b"), CrossingRelation::AtGrade, &policy())
        .unwrap()
        .is_none());
}

#[test]
fn lane_connections_are_stable_and_manual_replacement_is_validated() {
    let mut network = RoadNetwork::new();
    network
        .add_road(line_road(
            "a",
            Point2::new(-100.0, 0.0),
            Point2::new(100.0, 0.0),
            &[3.5],
        ))
        .unwrap();
    network
        .add_road(line_road(
            "b",
            Point2::new(0.0, -100.0),
            Point2::new(0.0, 100.0),
            &[3.5],
        ))
        .unwrap();
    let junction_id = create(&mut network, "a", "b", "j-lanes", JunctionOptions::new());
    let junction = network.junction(&junction_id).unwrap();
    let ids = junction
        .lane_connections()
        .iter()
        .map(|connection| connection.id().clone())
        .collect::<Vec<_>>();
    let mut sorted = ids.clone();
    sorted.sort();
    assert_eq!(ids, sorted);
    assert!(junction.lane_connections().iter().all(|connection| {
        matches!(
            connection.movement(),
            Movement::Left | Movement::Through | Movement::Right
        )
    }));

    let manual = LaneConnection::new(
        "manual-connection",
        ApproachId::new("a::start").unwrap(),
        ComponentId::new("lane-1").unwrap(),
        ApproachId::new("b::end").unwrap(),
        ComponentId::new("lane-1").unwrap(),
        Movement::Left,
    )
    .unwrap();
    network
        .junction_mut(&junction_id)
        .unwrap()
        .replace_lane_connections(vec![manual])
        .expect("manual replacement");
    assert_eq!(
        network
            .junction(&junction_id)
            .unwrap()
            .lane_connections()
            .len(),
        1
    );

    let missing_lane = LaneConnection::new(
        "bad-connection",
        ApproachId::new("a::start").unwrap(),
        ComponentId::new("missing-lane").unwrap(),
        ApproachId::new("b::end").unwrap(),
        ComponentId::new("lane-1").unwrap(),
        Movement::Left,
    )
    .unwrap();
    assert!(network
        .junction_mut(&junction_id)
        .unwrap()
        .replace_lane_connections(vec![missing_lane])
        .is_err());

    let wrong_movement = LaneConnection::new(
        "wrong-movement",
        ApproachId::new("a::start").unwrap(),
        ComponentId::new("lane-1").unwrap(),
        ApproachId::new("b::end").unwrap(),
        ComponentId::new("lane-1").unwrap(),
        Movement::Right,
    )
    .unwrap();
    assert!(network
        .junction_mut(&junction_id)
        .unwrap()
        .replace_lane_connections(vec![wrong_movement])
        .is_err());
}

#[test]
fn source_road_changes_clear_derived_state_and_regenerate_deterministically() {
    let mut network = RoadNetwork::new();
    let original_a = line_road(
        "a",
        Point2::new(-100.0, 0.0),
        Point2::new(100.0, 0.0),
        &[3.5],
    );
    let original_b = line_road(
        "b",
        Point2::new(0.0, -100.0),
        Point2::new(0.0, 100.0),
        &[3.5],
    );
    network.add_road(original_a.clone()).unwrap();
    network.add_road(original_b).unwrap();
    let junction_id = create(
        &mut network,
        "a",
        "b",
        "j-regenerate",
        JunctionOptions::new(),
    );
    let old_area = network
        .junction(&junction_id)
        .unwrap()
        .surface()
        .unwrap()
        .area_m2()
        .unwrap();
    let old_corner_ids = network
        .junction(&junction_id)
        .unwrap()
        .corners()
        .iter()
        .map(|corner| corner.id().clone())
        .collect::<Vec<_>>();
    let old_connection_ids = network
        .junction(&junction_id)
        .unwrap()
        .lane_connections()
        .iter()
        .map(|connection| connection.id().clone())
        .collect::<Vec<_>>();

    let wider_a = line_road(
        "a",
        Point2::new(-100.0, 0.0),
        Point2::new(100.0, 0.0),
        &[8.0],
    );
    network.replace_road(wider_a).unwrap();
    let stale = network.junction(&junction_id).unwrap();
    assert_eq!(stale.status(), JunctionStatus::Stale);
    assert!(stale.surface().is_none());
    assert!(stale.lane_connections().is_empty());
    assert_eq!(
        network
            .regenerate_junction(&junction_id, &policy())
            .unwrap(),
        RegenerationResult::Regenerated
    );
    let regenerated = network.junction(&junction_id).unwrap();
    assert_eq!(regenerated.status(), JunctionStatus::Fresh);
    assert_ne!(regenerated.surface().unwrap().area_m2().unwrap(), old_area);
    assert_eq!(
        regenerated
            .corners()
            .iter()
            .map(|corner| corner.id().clone())
            .collect::<Vec<_>>(),
        old_corner_ids
    );
    assert_eq!(
        regenerated
            .lane_connections()
            .iter()
            .map(|connection| connection.id().clone())
            .collect::<Vec<_>>(),
        old_connection_ids
    );

    let moved_b = line_road(
        "b",
        Point2::new(0.0, 200.0),
        Point2::new(0.0, 300.0),
        &[3.5],
    );
    network.replace_road(moved_b).unwrap();
    assert_eq!(
        network
            .regenerate_junction(&junction_id, &policy())
            .unwrap(),
        RegenerationResult::Stale
    );
    let no_candidate = network.junction(&junction_id).unwrap();
    assert!(no_candidate.surface().is_none());
    assert!(no_candidate.lane_connections().is_empty());
}

#[test]
fn divided_to_undivided_cross_sections_are_supported_without_topology_inference() {
    let mut network = RoadNetwork::new();
    network
        .add_road(road_with_median(
            "divided",
            Point2::new(-100.0, 0.0),
            Point2::new(100.0, 0.0),
        ))
        .unwrap();
    network
        .add_road(line_road(
            "undivided",
            Point2::new(0.0, -100.0),
            Point2::new(0.0, 100.0),
            &[3.5, 3.5],
        ))
        .unwrap();
    let junction_id = create(
        &mut network,
        "divided",
        "undivided",
        "j-divided",
        JunctionOptions::new(),
    );
    let junction = network.junction(&junction_id).unwrap();
    assert_eq!(junction.approaches().len(), 4);
    assert!(junction.approaches().iter().all(|approach| approach
        .cross_section_state()
        .iter()
        .any(
            |state| state.kind == ComponentKind::Median || state.kind == ComponentKind::TrafficLane
        )));
    assert!(junction.surface().is_some());
}

#[test]
fn repeated_detection_and_creation_are_deterministic_independent_of_insert_order() {
    let roads = [
        line_road(
            "a",
            Point2::new(-100.0, 0.0),
            Point2::new(100.0, 0.0),
            &[3.5, 3.5],
        ),
        line_road(
            "b",
            Point2::new(-30.0, -100.0),
            Point2::new(70.0, 100.0),
            &[4.0],
        ),
    ];
    let mut first = RoadNetwork::new();
    first.add_road(roads[0].clone()).unwrap();
    first.add_road(roads[1].clone()).unwrap();
    let mut second = RoadNetwork::new();
    second.add_road(roads[1].clone()).unwrap();
    second.add_road(roads[0].clone()).unwrap();

    let first_candidate = first
        .detect_candidate(&id("a"), &id("b"), CrossingRelation::AtGrade, &policy())
        .unwrap()
        .unwrap();
    for _ in 0..20 {
        assert_eq!(
            first
                .detect_candidate(&id("a"), &id("b"), CrossingRelation::AtGrade, &policy())
                .unwrap()
                .unwrap(),
            first_candidate
        );
    }
    create(
        &mut first,
        "a",
        "b",
        "j-deterministic",
        JunctionOptions::new(),
    );
    create(
        &mut second,
        "a",
        "b",
        "j-deterministic",
        JunctionOptions::new(),
    );
    assert_eq!(first.junctions(), second.junctions());
}

#[test]
fn adversarial_geometry_rejects_invalid_inputs_without_non_finite_surface() {
    let mut acute = RoadNetwork::new();
    acute
        .add_road(line_road(
            "a",
            Point2::new(-1000.0, 0.0),
            Point2::new(1000.0, 0.0),
            &[3.5],
        ))
        .unwrap();
    acute
        .add_road(line_road(
            "b",
            Point2::new(-1000.0, -0.001),
            Point2::new(1000.0, 0.001),
            &[3.5],
        ))
        .unwrap();
    let acute_candidate = acute
        .detect_candidate(&id("a"), &id("b"), CrossingRelation::AtGrade, &policy())
        .unwrap()
        .unwrap();
    let acute_result = acute.create_junction(
        &acute_candidate,
        "j-acute",
        JunctionOptions::new(),
        &policy(),
    );
    if let Ok(junction_id) = acute_result {
        let surface = acute.junction(&junction_id).unwrap().surface().unwrap();
        surface.validate(&policy()).unwrap();
        assert!(surface.vertices().iter().all(|point| point.is_finite()));
    }

    let mut invalid_radius = RoadNetwork::new();
    invalid_radius
        .add_road(line_road(
            "a",
            Point2::new(-100.0, 0.0),
            Point2::new(100.0, 0.0),
            &[3.5],
        ))
        .unwrap();
    invalid_radius
        .add_road(line_road(
            "b",
            Point2::new(0.0, -100.0),
            Point2::new(0.0, 100.0),
            &[3.5],
        ))
        .unwrap();
    let candidate = invalid_radius
        .detect_candidate(&id("a"), &id("b"), CrossingRelation::AtGrade, &policy())
        .unwrap()
        .unwrap();
    assert!(invalid_radius
        .create_junction(
            &candidate,
            "j-invalid-radius",
            JunctionOptions::new().with_uniform_corner_radius(0.0),
            &policy(),
        )
        .is_err());

    let mut large = RoadNetwork::new();
    large
        .add_road(line_road(
            "a",
            Point2::new(1.0e9 - 100.0, 1.0e9),
            Point2::new(1.0e9 + 100.0, 1.0e9),
            &[3.5],
        ))
        .unwrap();
    large
        .add_road(line_road(
            "b",
            Point2::new(1.0e9, 1.0e9 - 100.0),
            Point2::new(1.0e9, 1.0e9 + 100.0),
            &[3.5],
        ))
        .unwrap();
    let candidate = large
        .detect_candidate(&id("a"), &id("b"), CrossingRelation::AtGrade, &policy())
        .unwrap()
        .unwrap();
    let junction_id = create(&mut large, "a", "b", "j-large", JunctionOptions::new());
    assert!(large
        .junction(&junction_id)
        .unwrap()
        .surface()
        .unwrap()
        .vertices()
        .iter()
        .all(|point| point.is_finite()));
    assert!(large
        .create_junction(
            &candidate,
            "j-large-duplicate",
            JunctionOptions::new(),
            &policy()
        )
        .is_err());
}

#[test]
fn property_and_fuzz_style_candidate_corpus_is_repeatable_and_controlled() {
    let policy = policy();
    assert!(Alignment::line(Point2::new(0.0, 0.0), Point2::new(1.0e-8, 0.0), &policy,).is_err());

    for index in 0..64usize {
        let offset = -80.0 + index as f64 * 2.5;
        let mut network = RoadNetwork::new();
        network
            .add_road(line_road(
                "horizontal",
                Point2::new(-100.0, 0.0),
                Point2::new(100.0, 0.0),
                &[3.5],
            ))
            .unwrap();
        network
            .add_road(line_road(
                "vertical",
                Point2::new(offset, -100.0),
                Point2::new(offset, 100.0),
                &[3.5],
            ))
            .unwrap();
        let first = network
            .detect_candidate(
                &id("horizontal"),
                &id("vertical"),
                CrossingRelation::AtGrade,
                &policy,
            )
            .unwrap()
            .unwrap();
        let second = network
            .detect_candidate(
                &id("horizontal"),
                &id("vertical"),
                CrossingRelation::AtGrade,
                &policy,
            )
            .unwrap()
            .unwrap();
        assert_eq!(first, second);
        let junction_id = network
            .create_junction(
                &first,
                format!("property-{index}"),
                JunctionOptions::new(),
                &policy,
            )
            .unwrap();
        network
            .junction(&junction_id)
            .unwrap()
            .surface()
            .unwrap()
            .validate(&policy)
            .unwrap();
    }

    let mut near_coincident = RoadNetwork::new();
    near_coincident
        .add_road(line_road(
            "a",
            Point2::new(-10.0, 0.0),
            Point2::new(0.0, 0.0),
            &[3.5],
        ))
        .unwrap();
    near_coincident
        .add_road(line_road(
            "b",
            Point2::new(5.0e-10, 0.0),
            Point2::new(5.0e-10, 10.0),
            &[3.5],
        ))
        .unwrap();
    assert_eq!(
        near_coincident
            .detect_candidate(&id("a"), &id("b"), CrossingRelation::AtGrade, &policy)
            .unwrap()
            .unwrap()
            .crossing_type(),
        CrossingType::EndpointMeeting
    );
}

#[test]
fn public_surface_validation_rejects_self_intersection_and_degeneracy() {
    assert!(PavementSurface::new(
        vec![
            Point2::new(0.0, 0.0),
            Point2::new(2.0, 2.0),
            Point2::new(0.0, 2.0),
            Point2::new(2.0, 0.0),
        ],
        &policy(),
    )
    .is_err());
    assert!(PavementSurface::new(
        vec![
            Point2::new(0.0, 0.0),
            Point2::new(1.0e-10, 0.0),
            Point2::new(0.0, 1.0e-10),
        ],
        &policy(),
    )
    .is_err());
}

#[test]
fn station_range_helper_remains_available_for_r1b_fixture_authors() {
    let range = StationRange::new(0.0, 10.0, &policy()).unwrap();
    let profile = PiecewiseLinearWidthProfile::new(
        range,
        vec![
            WidthKnot {
                station_m: 0.0,
                width_m: 3.5,
            },
            WidthKnot {
                station_m: 10.0,
                width_m: 3.5,
            },
        ],
        &policy(),
    )
    .unwrap();
    assert_eq!(profile.width_at(5.0, &policy()).unwrap(), 3.5);
    let _ = CornerId::new("fixture-corner").unwrap();
}
