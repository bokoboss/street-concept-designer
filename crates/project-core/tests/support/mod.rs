use street_concept_designer_kernel::{
    Alignment, ComponentKind, CrossSection, CrossSectionComponent, CrossingRelation,
    JunctionOptions, PiecewiseLinearWidthProfile, Point2, Road, RoadNetwork, TolerancePolicy,
    WidthKnot,
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

pub fn line_road(id: &str, start: Point2, end: Point2, width_m: f64) -> Road {
    let policy = policy();
    let alignment = Alignment::line(start, end, &policy).expect("line alignment");
    let range = alignment.station_range();
    let profile = PiecewiseLinearWidthProfile::constant(range, width_m, &policy)
        .expect("constant width profile");
    let lane = CrossSectionComponent::traffic_lane("lane-1", profile).expect("lane");
    let cross_section = CrossSection::new(range, vec![lane], &policy).expect("cross-section");
    Road::new(id, alignment, cross_section, &policy).expect("road")
}

pub fn network_with_one_road() -> RoadNetwork {
    let mut network = RoadNetwork::new();
    network
        .add_road(line_road(
            "R01",
            Point2::new(0.0, 0.0),
            Point2::new(120.0, 0.0),
            3.5,
        ))
        .expect("minimal road");
    network
}

fn road_with_components(
    id: &str,
    start: Point2,
    end: Point2,
    components: Vec<CrossSectionComponent>,
) -> Road {
    let policy = policy();
    let alignment = Alignment::line(start, end, &policy).expect("line alignment");
    let range = alignment.station_range();
    assert!(components
        .iter()
        .all(|component| component.width_profile().station_range() == range));
    let cross_section = CrossSection::new(range, components, &policy).expect("cross-section");
    Road::new(id, alignment, cross_section, &policy).expect("road")
}

fn constant_component(
    id: &str,
    kind: ComponentKind,
    range: street_concept_designer_kernel::StationRange,
    width_m: f64,
) -> CrossSectionComponent {
    let profile = PiecewiseLinearWidthProfile::constant(range, width_m, &policy())
        .expect("constant component profile");
    CrossSectionComponent::new(id, kind, profile).expect("component")
}

pub fn non_trivial_network() -> RoadNetwork {
    let policy = policy();
    let main_alignment =
        Alignment::line(Point2::new(-150.0, 0.0), Point2::new(150.0, 0.0), &policy)
            .expect("main alignment");
    let main_range = main_alignment.station_range();
    let turn_profile = PiecewiseLinearWidthProfile::new(
        main_range,
        vec![
            WidthKnot {
                station_m: main_range.start_m,
                width_m: 0.0,
            },
            WidthKnot {
                station_m: 90.0,
                width_m: 3.25,
            },
            WidthKnot {
                station_m: 210.0,
                width_m: 3.25,
            },
            WidthKnot {
                station_m: main_range.end_m,
                width_m: 0.0,
            },
        ],
        &policy,
    )
    .expect("turn-pocket lifecycle");
    let main = road_with_components(
        "R01",
        Point2::new(-150.0, 0.0),
        Point2::new(150.0, 0.0),
        vec![
            constant_component("lane-1", ComponentKind::TrafficLane, main_range, 3.5),
            constant_component("lane-2", ComponentKind::TrafficLane, main_range, 3.5),
            CrossSectionComponent::traffic_lane("turn-pocket", turn_profile)
                .expect("turn-pocket lane"),
        ],
    );

    let stem_alignment =
        Alignment::line(Point2::new(0.0, -150.0), Point2::new(0.0, 150.0), &policy)
            .expect("stem alignment");
    let stem_range = stem_alignment.station_range();
    let stem = road_with_components(
        "R02",
        Point2::new(0.0, -150.0),
        Point2::new(0.0, 150.0),
        vec![
            constant_component("lane-1", ComponentKind::TrafficLane, stem_range, 3.5),
            constant_component("lane-2", ComponentKind::TrafficLane, stem_range, 3.5),
        ],
    );

    let mut network = RoadNetwork::new();
    network.add_road(main).expect("main road");
    network.add_road(stem).expect("stem road");
    let candidate = network
        .detect_candidate(
            &street_concept_designer_kernel::RoadId::new("R01").expect("R01"),
            &street_concept_designer_kernel::RoadId::new("R02").expect("R02"),
            CrossingRelation::AtGrade,
            &policy,
        )
        .expect("candidate detection")
        .expect("candidate");
    network
        .create_junction(&candidate, "J01", JunctionOptions::new(), &policy)
        .expect("junction");
    network
        .validate(&policy)
        .expect("valid non-trivial network");
    network
}

pub fn minimal_existing() -> Project {
    let scenario = Scenario::new(
        scenario_id("scenario-existing"),
        "Existing",
        ScenarioRole::Existing,
        true,
        network_with_one_road(),
    )
    .expect("minimal Existing scenario");
    Project::with_scenario(
        project_id("project-p01"),
        "P-01 Minimal Existing",
        TrafficSide::LeftHand,
        CoordinateContext::empty(),
        scenario,
    )
    .expect("minimal project")
}

pub fn non_trivial_existing() -> Project {
    let scenario = Scenario::new(
        scenario_id("scenario-existing"),
        "Existing",
        ScenarioRole::Existing,
        true,
        non_trivial_network(),
    )
    .expect("non-trivial Existing scenario");
    Project::with_scenario(
        project_id("project-p02"),
        "P-02 Non-trivial Existing",
        TrafficSide::LeftHand,
        CoordinateContext::empty(),
        scenario,
    )
    .expect("non-trivial project")
}

pub fn existing_with_alternative() -> Project {
    let mut project = non_trivial_existing();
    project
        .duplicate_scenario(
            &scenario_id("scenario-existing"),
            scenario_id("scenario-alt-a"),
            "Alternative A",
            ScenarioRole::Alternative,
            false,
        )
        .expect("duplicate Existing");
    project
}
