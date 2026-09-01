use street_concept_designer_kernel::{
    Alignment, ComponentId, ComponentKind, CrossSection, CrossSectionComponent, CrossingRelation,
    JunctionOptions, LaneConnection, LaneDirection, Movement, PiecewiseLinearWidthProfile, Point2,
    Road, RoadId, RoadNetwork, TolerancePolicy, WidthKnot,
};
use street_concept_designer_project_core::{
    CoordinateContext, Project, ProjectId, Scenario, ScenarioId, ScenarioRole, TrafficSide,
};

pub fn policy() -> TolerancePolicy {
    TolerancePolicy::default()
}

pub fn project_id(value: &str) -> ProjectId {
    ProjectId::new(value).expect("valid project id")
}

pub fn scenario_id(value: &str) -> ScenarioId {
    ScenarioId::new(value).expect("valid scenario id")
}

pub fn road_id(value: &str) -> RoadId {
    RoadId::new(value).expect("valid road id")
}

pub fn component_id(value: &str) -> ComponentId {
    ComponentId::new(value).expect("valid component id")
}

fn width_profile(
    range: street_concept_designer_kernel::StationRange,
    knots: Vec<(f64, f64)>,
) -> PiecewiseLinearWidthProfile {
    PiecewiseLinearWidthProfile::new(
        range,
        knots
            .into_iter()
            .map(|(station_m, width_m)| WidthKnot { station_m, width_m })
            .collect(),
        &policy(),
    )
    .expect("valid width profile")
}

fn constant_component(
    id: &str,
    kind: ComponentKind,
    range: street_concept_designer_kernel::StationRange,
    width_m: f64,
) -> CrossSectionComponent {
    CrossSectionComponent::new(
        id,
        kind,
        PiecewiseLinearWidthProfile::constant(range, width_m, &policy())
            .expect("constant width profile"),
    )
    .expect("valid component")
}

fn road_with_components(
    id: &str,
    alignment: Alignment,
    components: Vec<CrossSectionComponent>,
    directions: Vec<(ComponentId, LaneDirection)>,
) -> Road {
    let range = alignment.station_range();
    let cross_section = CrossSection::new(range, components, &policy()).expect("cross-section");
    Road::with_lane_directions(id, alignment, cross_section, directions, &policy()).expect("road")
}

pub fn representative_project() -> Project {
    let policy = policy();
    let main_alignment = Alignment::line(
        Point2::new(1_000_000_000.125, -2_000_000_000.75),
        Point2::new(1_000_000_300.125, -2_000_000_000.75),
        &policy,
    )
    .expect("main alignment");
    let main_range = main_alignment.station_range();
    let turn_profile = width_profile(
        main_range,
        vec![
            (0.0, 0.0),
            (23.4, 0.0),
            (41.7, 3.25),
            (87.3, 3.25),
            (103.6, 0.0),
            (200.0, 0.0),
            (main_range.end_m, 0.0),
        ],
    );
    let main = road_with_components(
        "R01",
        main_alignment,
        vec![
            constant_component("lane-main-a", ComponentKind::TrafficLane, main_range, 3.5),
            constant_component("lane-main-b", ComponentKind::TrafficLane, main_range, 3.5),
            CrossSectionComponent::traffic_lane("turn-pocket", turn_profile).expect("turn pocket"),
        ],
        vec![
            (component_id("lane-main-a"), LaneDirection::WithAlignment),
            (component_id("lane-main-b"), LaneDirection::AgainstAlignment),
            (component_id("turn-pocket"), LaneDirection::Bidirectional),
        ],
    );

    let stem_alignment = Alignment::line(
        Point2::new(1_000_000_150.125, -2_000_000_150.75),
        Point2::new(1_000_000_150.125, -1_999_999_850.75),
        &policy,
    )
    .expect("stem alignment");
    let stem_range = stem_alignment.station_range();
    let stem = road_with_components(
        "R02",
        stem_alignment,
        vec![
            constant_component("lane-stem-a", ComponentKind::TrafficLane, stem_range, 3.5),
            constant_component("lane-stem-b", ComponentKind::TrafficLane, stem_range, 3.5),
        ],
        vec![
            (component_id("lane-stem-a"), LaneDirection::Bidirectional),
            (component_id("lane-stem-b"), LaneDirection::Bidirectional),
        ],
    );

    let mut network = RoadNetwork::new();
    network.add_road(main).expect("main road");
    network.add_road(stem).expect("stem road");
    let candidate = network
        .detect_candidate(
            &road_id("R01"),
            &road_id("R02"),
            CrossingRelation::AtGrade,
            &policy,
        )
        .expect("candidate detection")
        .expect("candidate");
    let junction_id = network
        .create_junction(
            &candidate,
            "J01",
            JunctionOptions::new().with_uniform_corner_radius(9.75),
            &policy,
        )
        .expect("junction");
    let corner_id = network.junctions()[0].corners()[1].id().clone();
    network
        .junction_mut(&junction_id)
        .expect("junction mut")
        .set_corner_radius(&corner_id, 17.25, &policy)
        .expect("corner override");
    let manual = LaneConnection::new(
        "manual-main-to-stem",
        street_concept_designer_kernel::ApproachId::new("R01::start").expect("from approach"),
        component_id("lane-main-a"),
        street_concept_designer_kernel::ApproachId::new("R02::end").expect("to approach"),
        component_id("lane-stem-a"),
        Movement::Left,
    )
    .expect("manual connection");
    network
        .junction_mut(&junction_id)
        .expect("junction mut")
        .replace_lane_connections(vec![manual])
        .expect("manual connectivity");
    network.validate(&policy).expect("representative network");

    let existing = Scenario::new(
        scenario_id("scenario-z-existing"),
        "Existing",
        ScenarioRole::Existing,
        true,
        network,
    )
    .expect("Existing scenario");
    let project_context = CoordinateContext::empty()
        .with_crs_identifier("EPSG:32647")
        .expect("CRS")
        .with_description("large-coordinate engineering fixture")
        .expect("description");
    let mut alternative_b_seed = Project::with_scenario(
        project_id("project-r2b-representative"),
        "R2B Representative",
        TrafficSide::LeftHand,
        project_context.clone(),
        existing.clone(),
    )
    .expect("representative project");
    alternative_b_seed
        .duplicate_scenario(
            &scenario_id("scenario-z-existing"),
            scenario_id("scenario-m-alt-b"),
            "Alternative B",
            ScenarioRole::Alternative,
            false,
        )
        .expect("Alternative B");
    let alternative_b = alternative_b_seed
        .scenario(&scenario_id("scenario-m-alt-b"))
        .expect("Alternative B")
        .clone();
    let mut alternative_a_seed = Project::with_scenario(
        project_id("project-r2b-representative"),
        "R2B Representative",
        TrafficSide::LeftHand,
        project_context.clone(),
        existing.clone(),
    )
    .expect("Alternative A seed");
    alternative_a_seed
        .duplicate_scenario(
            &scenario_id("scenario-z-existing"),
            scenario_id("scenario-a-alt-a"),
            "Alternative A",
            ScenarioRole::Alternative,
            false,
        )
        .expect("Alternative A");

    let alternative_id = scenario_id("scenario-a-alt-a");
    let mut alternative_network = alternative_a_seed
        .scenario(&alternative_id)
        .expect("Alternative A")
        .network()
        .clone();
    let mut changed_road = alternative_network
        .road(&road_id("R01"))
        .expect("Alternative road")
        .clone();
    changed_road
        .set_lane_direction(&component_id("lane-main-b"), LaneDirection::WithAlignment)
        .expect("lane direction difference");
    alternative_network
        .replace_road(changed_road)
        .expect("replace alternative road");
    assert_eq!(
        alternative_network
            .regenerate_junction(
                &street_concept_designer_kernel::JunctionId::new("J01").unwrap(),
                &policy
            )
            .expect("regenerate Alternative A"),
        street_concept_designer_kernel::RegenerationResult::Regenerated
    );
    let alternative_a = Scenario::new(
        alternative_id,
        "Alternative A",
        ScenarioRole::Alternative,
        false,
        alternative_network,
    )
    .expect("Alternative A scenario");
    let project = Project::from_scenarios(
        project_id("project-r2b-representative"),
        "R2B Representative",
        TrafficSide::LeftHand,
        project_context,
        vec![existing, alternative_b, alternative_a],
    )
    .expect("representative project");
    project.validate(&policy).expect("representative project");
    project
}

pub fn automatic_project() -> Project {
    let policy = policy();
    let horizontal = Alignment::line(Point2::new(-100.0, 0.0), Point2::new(100.0, 0.0), &policy)
        .expect("horizontal alignment");
    let horizontal_range = horizontal.station_range();
    let vertical = Alignment::line(Point2::new(0.0, -100.0), Point2::new(0.0, 100.0), &policy)
        .expect("vertical alignment");
    let vertical_range = vertical.station_range();
    let road_a = road_with_components(
        "auto-a",
        horizontal,
        vec![constant_component(
            "lane-a",
            ComponentKind::TrafficLane,
            horizontal_range,
            3.5,
        )],
        vec![(component_id("lane-a"), LaneDirection::Bidirectional)],
    );
    let road_b = road_with_components(
        "auto-b",
        vertical,
        vec![constant_component(
            "lane-b",
            ComponentKind::TrafficLane,
            vertical_range,
            3.5,
        )],
        vec![(component_id("lane-b"), LaneDirection::Bidirectional)],
    );
    let mut network = RoadNetwork::new();
    network.add_road(road_a).expect("auto road a");
    network.add_road(road_b).expect("auto road b");
    let candidate = network
        .detect_candidate(
            &road_id("auto-a"),
            &road_id("auto-b"),
            CrossingRelation::AtGrade,
            &policy,
        )
        .expect("auto candidate")
        .expect("auto candidate exists");
    network
        .create_junction(&candidate, "auto-junction", JunctionOptions::new(), &policy)
        .expect("auto junction");
    Project::with_scenario(
        project_id("project-r2b-automatic"),
        "Automatic connectivity",
        TrafficSide::RightHand,
        CoordinateContext::empty(),
        Scenario::new(
            scenario_id("scenario-auto"),
            "Existing",
            ScenarioRole::Existing,
            false,
            network,
        )
        .expect("automatic scenario"),
    )
    .expect("automatic project")
}

pub fn all_alignment_project() -> Project {
    let policy = policy();
    let line = Alignment::line(
        Point2::new(-100.0, -100.0),
        Point2::new(100.0, -100.0),
        &policy,
    )
    .expect("line");
    let arc = Alignment::circular_arc(
        Point2::new(500.0, 500.0),
        50.0,
        0.125,
        1.2345678901234567,
        &policy,
    )
    .expect("arc");
    let curve = Alignment::smooth_conceptual_curve(
        Point2::new(-100.0, 200.0),
        Point2::new(-50.0, 260.0),
        Point2::new(50.0, 260.0),
        Point2::new(100.0, 200.0),
        &policy,
    )
    .expect("smooth curve");
    let road = |id: &str, alignment: Alignment| {
        let range = alignment.station_range();
        road_with_components(
            id,
            alignment,
            vec![constant_component(
                "lane-1",
                ComponentKind::TrafficLane,
                range,
                3.25,
            )],
            vec![(component_id("lane-1"), LaneDirection::Bidirectional)],
        )
    };
    let mut network = RoadNetwork::new();
    network.add_road(road("line", line)).expect("line road");
    network.add_road(road("arc", arc)).expect("arc road");
    network
        .add_road(road("smooth", curve))
        .expect("smooth road");
    Project::with_scenario(
        project_id("project-r2b-alignments"),
        "All alignments",
        TrafficSide::LeftHand,
        CoordinateContext::empty(),
        Scenario::new(
            scenario_id("scenario-alignments"),
            "Existing",
            ScenarioRole::Existing,
            false,
            network,
        )
        .expect("alignment scenario"),
    )
    .expect("alignment project")
}

pub fn finite_corpus_project() -> Project {
    let policy = policy();
    let alignment = Alignment::line(
        Point2::new(1_000_000_000.125, 0.125),
        Point2::new(1_000_001_000.125, 0.125),
        &policy,
    )
    .expect("finite corpus alignment");
    let range = alignment.station_range();
    let profile = width_profile(
        range,
        vec![
            (0.0, -0.0),
            (0.125, f64::from_bits(0x3fb9_9999_9999_999b)),
            (23.4, 3.25),
            (41.7, 7.000000000000001),
            (87.3, 12.345678901234567),
            (103.6, -0.0),
            (200.0, 9_007_199_254_740_991.0),
            (range.end_m, 0.0),
        ],
    );
    let road = road_with_components(
        "corpus-road",
        alignment,
        vec![CrossSectionComponent::traffic_lane("corpus-lane", profile).expect("corpus lane")],
        vec![(component_id("corpus-lane"), LaneDirection::Bidirectional)],
    );
    let mut network = RoadNetwork::new();
    network.add_road(road).expect("corpus road");
    Project::with_scenario(
        project_id("project-r2b-f64"),
        "Finite f64 corpus",
        TrafficSide::LeftHand,
        CoordinateContext::empty(),
        Scenario::new(
            scenario_id("scenario-f64"),
            "Existing",
            ScenarioRole::Existing,
            false,
            network,
        )
        .expect("corpus scenario"),
    )
    .expect("corpus project")
}
