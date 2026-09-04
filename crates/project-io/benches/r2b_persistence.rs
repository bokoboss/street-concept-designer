use std::hint::black_box;
use std::time::Instant;

use street_concept_designer_kernel::{
    derive_diagnostic_2d, derive_diagnostic_3d, Alignment, ComponentId, CrossingRelation,
    JunctionOptions, LaneConnection, LaneDirection, Movement, PiecewiseLinearWidthProfile, Point2,
    Road, RoadId, RoadNetwork, TolerancePolicy, WidthKnot,
};
use street_concept_designer_project_core::{
    CoordinateContext, Project, ProjectId, Scenario, ScenarioId, ScenarioRole, TrafficSide,
};
use street_concept_designer_project_io::{
    decode_project_from_bytes, decode_project_from_json, encode_project_to_bytes,
    encode_project_to_json,
};

fn policy() -> TolerancePolicy {
    TolerancePolicy::default()
}

fn benchmark_project() -> Project {
    let policy = policy();
    let make_road = |id: &str, start: Point2, end: Point2| {
        let alignment = Alignment::line(start, end, &policy).expect("alignment");
        let range = alignment.station_range();
        let profile = PiecewiseLinearWidthProfile::new(
            range,
            vec![
                WidthKnot {
                    station_m: range.start_m,
                    width_m: 0.0,
                },
                WidthKnot {
                    station_m: 35.0,
                    width_m: 3.25,
                },
                WidthKnot {
                    station_m: 80.0,
                    width_m: 3.25,
                },
                WidthKnot {
                    station_m: range.end_m,
                    width_m: 0.0,
                },
            ],
            &policy,
        )
        .expect("profile");
        let lane_profile =
            PiecewiseLinearWidthProfile::constant(range, 3.5, &policy).expect("lane profile");
        let cross_section = street_concept_designer_kernel::CrossSection::new(
            range,
            vec![
                street_concept_designer_kernel::CrossSectionComponent::traffic_lane(
                    "lane-1",
                    lane_profile,
                )
                .expect("lane"),
                street_concept_designer_kernel::CrossSectionComponent::traffic_lane(
                    "turn-pocket",
                    profile,
                )
                .expect("pocket"),
            ],
            &policy,
        )
        .expect("cross-section");
        Road::with_lane_directions(
            id,
            alignment,
            cross_section,
            vec![
                (
                    ComponentId::new("lane-1").expect("lane id"),
                    LaneDirection::Bidirectional,
                ),
                (
                    ComponentId::new("turn-pocket").expect("pocket id"),
                    LaneDirection::Bidirectional,
                ),
            ],
            &policy,
        )
        .expect("road")
    };
    let mut network = RoadNetwork::new();
    network
        .add_road(make_road(
            "R01",
            Point2::new(-150.0, 0.0),
            Point2::new(150.0, 0.0),
        ))
        .expect("road one");
    network
        .add_road(make_road(
            "R02",
            Point2::new(0.0, -150.0),
            Point2::new(0.0, 150.0),
        ))
        .expect("road two");
    let candidate = network
        .detect_candidate(
            &RoadId::new("R01").expect("road id"),
            &RoadId::new("R02").expect("road id"),
            CrossingRelation::AtGrade,
            &policy,
        )
        .expect("candidate")
        .expect("candidate exists");
    let junction_id = network
        .create_junction(&candidate, "J01", JunctionOptions::new(), &policy)
        .expect("junction");
    let corner_id = network.junctions()[0].corners()[0].id().clone();
    network
        .junction_mut(&junction_id)
        .expect("junction")
        .set_corner_radius(&corner_id, 12.5, &policy)
        .expect("corner");
    let manual = LaneConnection::new(
        "manual-connection",
        street_concept_designer_kernel::ApproachId::new("R01::start").expect("approach"),
        ComponentId::new("lane-1").expect("lane"),
        street_concept_designer_kernel::ApproachId::new("R02::end").expect("approach"),
        ComponentId::new("lane-1").expect("lane"),
        Movement::Left,
    )
    .expect("manual");
    network
        .junction_mut(&junction_id)
        .expect("junction")
        .replace_lane_connections(vec![manual])
        .expect("manual connectivity");
    let existing = Scenario::new(
        ScenarioId::new("scenario-z-existing").expect("scenario"),
        "Existing",
        ScenarioRole::Existing,
        true,
        network,
    )
    .expect("Existing");
    let mut project = Project::with_scenario(
        ProjectId::new("project-r2b-benchmark").expect("project"),
        "R2B benchmark",
        TrafficSide::LeftHand,
        CoordinateContext::empty(),
        existing,
    )
    .expect("project");
    project
        .duplicate_scenario(
            &ScenarioId::new("scenario-z-existing").expect("scenario"),
            ScenarioId::new("scenario-a-alt-a").expect("scenario"),
            "Alternative A",
            ScenarioRole::Alternative,
            false,
        )
        .expect("alternative");
    project
}

fn historical_v0() -> &'static str {
    include_str!("../tests/fixtures/r2b_schema_v0.json")
}

fn measure<T>(label: &str, iterations: usize, mut operation: impl FnMut() -> T) -> T {
    let start = Instant::now();
    let mut last = None;
    for _ in 0..iterations {
        last = Some(operation());
    }
    let elapsed = start.elapsed();
    let micros_per_operation = elapsed.as_secs_f64() * 1_000_000.0 / iterations as f64;
    println!(
        "{label}: iterations={iterations} total_ms={:.3} us_per_op={micros_per_operation:.3}",
        elapsed.as_secs_f64() * 1_000.0
    );
    last.expect("benchmark has positive iterations")
}

fn main() {
    let project = benchmark_project();
    let json = encode_project_to_json(&project).expect("encode");
    let bytes = json.as_bytes();
    let historical = historical_v0();
    let iterations = 2_000;
    println!(
        "R2B persistence benchmark: target_os={} target_arch={} document_bytes={}",
        std::env::consts::OS,
        std::env::consts::ARCH,
        bytes.len()
    );

    black_box(measure("encode Project -> JSON", iterations, || {
        black_box(encode_project_to_bytes(black_box(&project)).expect("encode"))
    }));
    black_box(measure("decode JSON -> Project", iterations, || {
        black_box(decode_project_from_bytes(black_box(bytes)).expect("decode"))
    }));
    black_box(measure("v0 -> v2 migration/load", iterations, || {
        black_box(decode_project_from_json(black_box(historical)).expect("migrate"))
    }));
    black_box(measure("complete in-memory save/load", iterations, || {
        let encoded = encode_project_to_bytes(black_box(&project)).expect("encode");
        black_box(decode_project_from_bytes(&encoded).expect("decode"))
    }));
    black_box(measure("clean R1C rebuild after load", iterations, || {
        let loaded = decode_project_from_bytes(black_box(bytes)).expect("decode");
        let scenario = &loaded.scenarios()[0];
        let snapshot = street_concept_designer_kernel::DerivedEngineeringSnapshot::derive(
            scenario.network(),
            Point2::new(0.0, 0.0),
            &policy(),
        )
        .expect("snapshot");
        let diagnostic_2d = derive_diagnostic_2d(&snapshot).expect("2D");
        let diagnostic_3d = derive_diagnostic_3d(&snapshot).expect("3D");
        black_box((loaded, snapshot, diagnostic_2d, diagnostic_3d))
    }));
}
