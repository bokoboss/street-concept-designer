use std::hint::black_box;
use std::time::Instant;

use street_concept_designer_kernel::{
    derive_diagnostic_2d, derive_diagnostic_3d, Alignment, ComponentKind, CrossSection,
    CrossSectionComponent, CrossingRelation, DerivedEngineeringSnapshot, JunctionOptions,
    PiecewiseLinearWidthProfile, Point2, Road, RoadId, RoadNetwork, TolerancePolicy,
};
use street_concept_designer_project_core::{
    CoordinateContext, Project, ProjectId, Scenario, ScenarioId, ScenarioRole, TrafficSide,
};
use street_concept_designer_project_io::encode_project_to_bytes;
use street_concept_designer_project_session::{
    Command, ProjectSession, Transaction, TransactionId,
};

fn policy() -> TolerancePolicy {
    TolerancePolicy::default()
}

fn road(id: &str, start: Point2, end: Point2) -> Road {
    let policy = policy();
    let alignment = Alignment::line(start, end, &policy).expect("alignment");
    let range = alignment.station_range();
    let lane = CrossSectionComponent::new(
        "lane-1",
        ComponentKind::TrafficLane,
        PiecewiseLinearWidthProfile::constant(range, 3.5, &policy).expect("lane profile"),
    )
    .expect("lane");
    let cross_section = CrossSection::new(range, vec![lane], &policy).expect("cross-section");
    Road::new(id, alignment, cross_section, &policy).expect("road")
}

fn network() -> RoadNetwork {
    let policy = policy();
    let mut network = RoadNetwork::new();
    network
        .add_road(road(
            "R01",
            Point2::new(-150.0, 0.0),
            Point2::new(150.0, 0.0),
        ))
        .expect("road one");
    network
        .add_road(road(
            "R02",
            Point2::new(0.0, -150.0),
            Point2::new(0.0, 150.0),
        ))
        .expect("road two");
    let candidate = network
        .detect_candidate(
            &RoadId::new("R01").expect("R01"),
            &RoadId::new("R02").expect("R02"),
            CrossingRelation::AtGrade,
            &policy,
        )
        .expect("candidate detection")
        .expect("candidate");
    network
        .create_junction(&candidate, "J01", JunctionOptions::new(), &policy)
        .expect("junction");
    network
}

fn benchmark_project() -> Project {
    let scenario = Scenario::new(
        ScenarioId::new("scenario-existing").expect("scenario"),
        "Existing",
        ScenarioRole::Existing,
        true,
        network(),
    )
    .expect("scenario");
    let mut project = Project::with_scenario(
        ProjectId::new("project-r2c-benchmark").expect("project"),
        "R2C benchmark",
        TrafficSide::LeftHand,
        CoordinateContext::empty(),
        scenario,
    )
    .expect("project");
    project
        .duplicate_scenario(
            &ScenarioId::new("scenario-existing").expect("source"),
            ScenarioId::new("scenario-alt-a").expect("alternative A"),
            "Alternative A",
            ScenarioRole::Alternative,
            false,
        )
        .expect("alternative A");
    project
        .duplicate_scenario(
            &ScenarioId::new("scenario-existing").expect("source"),
            ScenarioId::new("scenario-alt-b").expect("alternative B"),
            "Alternative B",
            ScenarioRole::Alternative,
            false,
        )
        .expect("alternative B");
    project
}

fn transaction() -> Transaction {
    Transaction::new(
        TransactionId::new("benchmark-add-road").expect("transaction id"),
        0,
        "Benchmark Add Road",
        vec![Command::AddRoad {
            scenario_id: ScenarioId::new("scenario-alt-a").expect("scenario"),
            road: road("R03", Point2::new(300.0, 0.0), Point2::new(420.0, 0.0)),
        }],
    )
    .expect("transaction")
}

fn measure<T>(label: &str, iterations: usize, mut operation: impl FnMut() -> T) -> T {
    let start = Instant::now();
    let mut last = None;
    for _ in 0..iterations {
        last = Some(operation());
    }
    let elapsed = start.elapsed();
    println!(
        "{label}: iterations={iterations} total_ms={:.3} us_per_op={:.3}",
        elapsed.as_secs_f64() * 1_000.0,
        elapsed.as_secs_f64() * 1_000_000.0 / iterations as f64
    );
    last.expect("positive benchmark iterations")
}

fn clean_rebuild(project: &Project) -> usize {
    project
        .scenarios()
        .iter()
        .map(|scenario| {
            let snapshot = DerivedEngineeringSnapshot::derive(
                scenario.network(),
                Point2::new(0.0, 0.0),
                &policy(),
            )
            .expect("R1C snapshot");
            let diagnostic_2d = derive_diagnostic_2d(&snapshot).expect("R1C 2D");
            let diagnostic_3d = derive_diagnostic_3d(&snapshot).expect("R1C 3D");
            snapshot.roads().len()
                + snapshot.junctions().len()
                + diagnostic_2d.primitives().len()
                + diagnostic_3d.meshes().len()
        })
        .sum()
}

fn main() {
    let project = benchmark_project();
    let transaction = transaction();
    let iterations = 500usize;
    let json_size = encode_project_to_bytes(&project)
        .expect("project JSON")
        .len();
    println!(
        "R2C command/history benchmark: fixture=3-scenario-project iterations={iterations} platform={} arch={} rust=rustc-1.98.0 json_bytes={json_size}",
        std::env::consts::OS,
        std::env::consts::ARCH,
    );
    println!(
        "snapshot storage proxy: canonical JSON bytes * 2 snapshots/history entry = {} bytes",
        json_size * 2
    );

    black_box(measure("Project clone", iterations, || {
        black_box(project.clone())
    }));
    black_box(measure("preview", iterations, || {
        let session = ProjectSession::new(project.clone()).expect("session");
        black_box(session.preview(&transaction).expect("preview"))
    }));
    black_box(measure("commit", iterations, || {
        let mut session = ProjectSession::new(project.clone()).expect("session");
        black_box(session.commit(&transaction).expect("commit"))
    }));
    black_box(measure("undo", iterations, || {
        let mut session = ProjectSession::new(project.clone()).expect("session");
        session.commit(&transaction).expect("commit");
        black_box(session.undo(1).expect("undo"))
    }));
    black_box(measure("redo", iterations, || {
        let mut session = ProjectSession::new(project.clone()).expect("session");
        session.commit(&transaction).expect("commit");
        session.undo(1).expect("undo");
        black_box(session.redo(2).expect("redo"))
    }));
    black_box(measure(
        "clean R1C rebuild after commit",
        iterations,
        || {
            let mut session = ProjectSession::new(project.clone()).expect("session");
            session.commit(&transaction).expect("commit");
            black_box(clean_rebuild(session.project()))
        },
    ));
}
