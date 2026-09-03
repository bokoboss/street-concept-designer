use std::hint::black_box;
use std::time::Instant;

use street_concept_designer_kernel::{
    Alignment, AlignmentSegment, ComponentKind, CrossSection, CrossSectionComponent,
    PiecewiseLinearWidthProfile, Point2, Road, RoadNetwork, TolerancePolicy,
};
use street_concept_designer_project_core::{
    CoordinateContext, Project, ProjectId, Scenario, ScenarioId, ScenarioRole, TrafficSide,
};
use street_concept_designer_project_session::{
    Command, ProjectSession, Transaction, TransactionId,
};

fn policy() -> TolerancePolicy {
    TolerancePolicy::default()
}

fn alignment_with_count(count: usize, y: f64) -> Alignment {
    let policy = policy();
    let segments = (0..count)
        .map(|index| {
            let start_x = index as f64 * 10.0;
            AlignmentSegment::line(
                format!("segment-{index}"),
                Point2::new(start_x, y),
                Point2::new(start_x + 10.0, y),
                &policy,
            )
            .expect("benchmark segment")
        })
        .collect();
    Alignment::from_segments(segments, &policy).expect("benchmark alignment")
}

fn road_with_count(count: usize, y: f64) -> Road {
    let policy = policy();
    let alignment = alignment_with_count(count, y);
    let range = alignment.station_range();
    let lane = CrossSectionComponent::new(
        "lane-1",
        ComponentKind::TrafficLane,
        PiecewiseLinearWidthProfile::constant(range, 3.5, &policy).expect("lane profile"),
    )
    .expect("lane");
    let cross_section = CrossSection::new(range, vec![lane], &policy).expect("cross-section");
    Road::new("benchmark-road", alignment, cross_section, &policy).expect("road")
}

fn project_with_count(count: usize) -> Project {
    let mut network = RoadNetwork::new();
    network
        .add_road(road_with_count(count, 0.0))
        .expect("benchmark road");
    let scenario = Scenario::new(
        ScenarioId::new("scenario-benchmark").expect("scenario id"),
        "Benchmark",
        ScenarioRole::Alternative,
        false,
        network,
    )
    .expect("scenario");
    Project::with_scenario(
        ProjectId::new("project-benchmark").expect("project id"),
        "R3A benchmark",
        TrafficSide::LeftHand,
        CoordinateContext::empty(),
        scenario,
    )
    .expect("project")
}

fn replace_transaction(revision: u64, id: &str, road: Road) -> Transaction {
    Transaction::new(
        TransactionId::new(id).expect("transaction id"),
        revision,
        "Replace composite alignment",
        vec![Command::ReplaceRoad {
            scenario_id: ScenarioId::new("scenario-benchmark").expect("scenario id"),
            road,
        }],
    )
    .expect("transaction")
}

fn elapsed_ms(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}

fn main() {
    println!(
        "R3A ProjectSession benchmark: target_os={} target_arch={} rust={}",
        std::env::consts::OS,
        std::env::consts::ARCH,
        rustc_version(),
    );
    for count in [3usize, 10, 50] {
        let project = project_with_count(count);
        let iterations = 1000usize;
        let start = Instant::now();
        for _ in 0..iterations {
            black_box(project.clone());
        }
        println!(
            "segments={count} Project clone: {iterations} ops in {:.3} ms ({:.3} us/op)",
            elapsed_ms(start),
            elapsed_ms(start) * 1000.0 / iterations as f64
        );

        let replacement = road_with_count(count, 1.0);
        let session = ProjectSession::new(project.clone()).expect("session");
        let preview_transaction = replace_transaction(0, "preview-composite", replacement.clone());
        let start = Instant::now();
        for _ in 0..iterations {
            black_box(session.preview(&preview_transaction).expect("preview"));
        }
        println!(
            "segments={count} ProjectSession preview: {iterations} ops in {:.3} ms ({:.3} us/op)",
            elapsed_ms(start),
            elapsed_ms(start) * 1000.0 / iterations as f64
        );

        let commit_iterations = 500usize;
        let road_a = road_with_count(count, 0.0);
        let road_b = road_with_count(count, 1.0);
        let mut committing_session = ProjectSession::new(project).expect("commit session");
        let start = Instant::now();
        for index in 0..commit_iterations {
            let road = if index % 2 == 0 {
                road_b.clone()
            } else {
                road_a.clone()
            };
            let transaction = replace_transaction(
                committing_session.revision(),
                &format!("commit-composite-{index}"),
                road,
            );
            black_box(committing_session.commit(&transaction).expect("commit"));
        }
        println!(
            "segments={count} ProjectSession commit: {commit_iterations} ops in {:.3} ms ({:.3} us/op)",
            elapsed_ms(start),
            elapsed_ms(start) * 1000.0 / commit_iterations as f64
        );
    }
}

fn rustc_version() -> String {
    std::process::Command::new("rustc")
        .arg("-V")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|version| version.trim().to_owned())
        .unwrap_or_else(|| "unknown".to_owned())
}
