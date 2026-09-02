use street_concept_designer_kernel::{
    Alignment, ApproachId, ComponentId, ComponentKind, CrossSection, CrossSectionComponent,
    CrossingRelation, JunctionCandidate, JunctionOptions, LaneConnection, Movement,
    PiecewiseLinearWidthProfile, Point2, Road, RoadId, RoadNetwork, TolerancePolicy,
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

pub fn line_road(id: &str, start: Point2, end: Point2, width_m: f64) -> Road {
    line_road_with_lanes(id, start, end, width_m, &["lane-1"])
}

pub fn line_road_with_lanes(
    id: &str,
    start: Point2,
    end: Point2,
    width_m: f64,
    lane_ids: &[&str],
) -> Road {
    let policy = policy();
    let alignment = Alignment::line(start, end, &policy).expect("line alignment");
    let range = alignment.station_range();
    let components = lane_ids
        .iter()
        .map(|lane_id| {
            CrossSectionComponent::new(
                *lane_id,
                ComponentKind::TrafficLane,
                PiecewiseLinearWidthProfile::constant(range, width_m, &policy)
                    .expect("lane profile"),
            )
            .expect("lane component")
        })
        .collect();
    let cross_section = CrossSection::new(range, components, &policy).expect("cross-section");
    Road::new(id, alignment, cross_section, &policy).expect("road")
}

pub fn network_without_junction() -> RoadNetwork {
    let mut network = RoadNetwork::new();
    network
        .add_road(line_road_with_lanes(
            "R01",
            Point2::new(-150.0, 0.0),
            Point2::new(150.0, 0.0),
            3.5,
            &["lane-1", "lane-2"],
        ))
        .expect("main road");
    network
        .add_road(line_road_with_lanes(
            "R02",
            Point2::new(0.0, -150.0),
            Point2::new(0.0, 150.0),
            3.5,
            &["lane-1", "lane-2"],
        ))
        .expect("stem road");
    network.validate(&policy()).expect("valid road network");
    network
}

pub fn junction_candidate(network: &RoadNetwork) -> JunctionCandidate {
    network
        .detect_candidate(
            &road_id("R01"),
            &road_id("R02"),
            CrossingRelation::AtGrade,
            &policy(),
        )
        .expect("candidate detection")
        .expect("candidate")
}

pub fn valid_manual_connection(id: &str) -> LaneConnection {
    LaneConnection::new(
        id,
        ApproachId::new("R01::start").expect("from approach"),
        component_id("lane-1"),
        ApproachId::new("R02::end").expect("to approach"),
        component_id("lane-1"),
        Movement::Left,
    )
    .expect("manual connection")
}

pub fn network_with_junction() -> RoadNetwork {
    let mut network = network_without_junction();
    let candidate = junction_candidate(&network);
    let junction_id = network
        .create_junction(&candidate, "J01", JunctionOptions::new(), &policy())
        .expect("junction");
    let corner_id = network.junctions()[0].corners()[0].id().clone();
    network
        .junction_mut(&junction_id)
        .expect("junction mut")
        .set_corner_radius(&corner_id, 12.0, &policy())
        .expect("corner radius");
    network
        .junction_mut(&junction_id)
        .expect("junction mut")
        .replace_lane_connections(vec![valid_manual_connection("manual-1")])
        .expect("manual connectivity");
    network.validate(&policy()).expect("valid junction network");
    network
}

pub fn single_existing_project() -> Project {
    Project::with_scenario(
        project_id("project-r2c"),
        "R2C project",
        TrafficSide::LeftHand,
        CoordinateContext::empty(),
        Scenario::new(
            scenario_id("scenario-existing"),
            "Existing",
            ScenarioRole::Existing,
            true,
            network_with_junction(),
        )
        .expect("Existing scenario"),
    )
    .expect("single-scenario project")
}

pub fn representative_project() -> Project {
    let mut project = single_existing_project();
    project
        .duplicate_scenario(
            &scenario_id("scenario-existing"),
            scenario_id("scenario-alt-a"),
            "Alternative A",
            ScenarioRole::Alternative,
            false,
        )
        .expect("Alternative A");
    project
}

pub fn project_without_junction() -> Project {
    Project::with_scenario(
        project_id("project-r2c-create"),
        "R2C create junction project",
        TrafficSide::LeftHand,
        CoordinateContext::empty(),
        Scenario::new(
            scenario_id("scenario-editable"),
            "Editable",
            ScenarioRole::Alternative,
            false,
            network_without_junction(),
        )
        .expect("editable scenario"),
    )
    .expect("project without junction")
}

pub fn corner_id(
    project: &Project,
    scenario: &ScenarioId,
) -> street_concept_designer_kernel::CornerId {
    project
        .scenario(scenario)
        .expect("scenario")
        .network()
        .junction(&street_concept_designer_kernel::JunctionId::new("J01").expect("junction id"))
        .expect("junction")
        .corners()
        .first()
        .expect("corner")
        .id()
        .clone()
}
