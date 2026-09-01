use std::hash::{Hash, Hasher};
use std::hint::black_box;
use std::time::Instant;

use street_concept_designer_kernel::{
    Alignment, ComponentKind, CrossSection, CrossSectionComponent, CrossingRelation,
    JunctionOptions, PiecewiseLinearWidthProfile, Point2, Road, RoadId, RoadNetwork,
    TolerancePolicy, WidthKnot,
};
use street_concept_designer_project_core::{
    CoordinateContext, Project, ProjectId, ProjectSemanticRef, Scenario, ScenarioId, ScenarioRole,
    TrafficSide,
};

fn policy() -> TolerancePolicy {
    TolerancePolicy::default()
}

fn non_trivial_network() -> RoadNetwork {
    let policy = policy();
    let main_alignment =
        Alignment::line(Point2::new(-150.0, 0.0), Point2::new(150.0, 0.0), &policy)
            .expect("main alignment");
    let main_range = main_alignment.station_range();
    let main_components = vec![
        CrossSectionComponent::new(
            "lane-1",
            ComponentKind::TrafficLane,
            PiecewiseLinearWidthProfile::constant(main_range, 3.5, &policy).expect("lane profile"),
        )
        .expect("lane"),
        CrossSectionComponent::new(
            "lane-2",
            ComponentKind::TrafficLane,
            PiecewiseLinearWidthProfile::constant(main_range, 3.5, &policy).expect("lane profile"),
        )
        .expect("lane"),
        CrossSectionComponent::traffic_lane(
            "turn-pocket",
            PiecewiseLinearWidthProfile::new(
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
            .expect("turn-pocket profile"),
        )
        .expect("turn-pocket"),
    ];
    let main_cross_section =
        CrossSection::new(main_range, main_components, &policy).expect("main cross-section");
    let main = Road::new("R01", main_alignment, main_cross_section, &policy).expect("main road");

    let stem_alignment =
        Alignment::line(Point2::new(0.0, -150.0), Point2::new(0.0, 150.0), &policy)
            .expect("stem alignment");
    let stem_range = stem_alignment.station_range();
    let stem_components = ["lane-1", "lane-2"]
        .into_iter()
        .map(|id| {
            CrossSectionComponent::traffic_lane(
                id,
                PiecewiseLinearWidthProfile::constant(stem_range, 3.5, &policy)
                    .expect("stem profile"),
            )
            .expect("stem lane")
        })
        .collect();
    let stem_cross_section =
        CrossSection::new(stem_range, stem_components, &policy).expect("stem cross-section");
    let stem = Road::new("R02", stem_alignment, stem_cross_section, &policy).expect("stem road");

    let mut network = RoadNetwork::new();
    network.add_road(main).expect("main insertion");
    network.add_road(stem).expect("stem insertion");
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
        .expect("junction creation");
    network
}

fn representative_project() -> Project {
    let scenario = Scenario::new(
        ScenarioId::new("scenario-existing").expect("scenario id"),
        "Existing",
        ScenarioRole::Existing,
        true,
        non_trivial_network(),
    )
    .expect("scenario");
    Project::with_scenario(
        ProjectId::new("project-benchmark").expect("project id"),
        "Benchmark project",
        TrafficSide::LeftHand,
        CoordinateContext::empty(),
        scenario,
    )
    .expect("project")
}

fn main() {
    let source_id = ScenarioId::new("scenario-existing").expect("source id");
    let representative = representative_project();
    let iterations = 2_000usize;

    let start = Instant::now();
    let mut duplicate_sink = 0usize;
    for index in 0..iterations {
        let mut project = representative.clone();
        let target_id =
            ScenarioId::new(format!("scenario-alt-{index}")).expect("generated benchmark id");
        project
            .duplicate_scenario(
                &source_id,
                target_id,
                "Alternative",
                ScenarioRole::Alternative,
                false,
            )
            .expect("duplicate scenario");
        duplicate_sink += project.scenarios().len();
        black_box(project);
    }
    let duplicate_elapsed = start.elapsed();

    let mut multi = representative.clone();
    for index in 0..4 {
        multi
            .duplicate_scenario(
                &source_id,
                ScenarioId::new(format!("scenario-multi-{index}")).expect("multi scenario id"),
                "Alternative",
                ScenarioRole::Alternative,
                false,
            )
            .expect("multi duplicate");
    }
    let start = Instant::now();
    let mut validation_sink = 0usize;
    for _ in 0..iterations {
        multi.validate(&policy()).expect("project validation");
        validation_sink += multi.scenarios().len();
    }
    let validation_elapsed = start.elapsed();

    let local_ref =
        street_concept_designer_kernel::SemanticRef::Road(RoadId::new("R01").expect("road ref"));
    let start = Instant::now();
    let mut reference_sink = 0usize;
    for _ in 0..iterations * 10 {
        let scoped = ProjectSemanticRef::new(source_id.clone(), local_ref.clone());
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        scoped.hash(&mut hasher);
        reference_sink ^= hasher.finish() as usize;
        black_box(scoped);
    }
    let reference_elapsed = start.elapsed();

    black_box(duplicate_sink);
    black_box(validation_sink);
    black_box(reference_sink);
    println!(
        "scenario duplicate: {iterations} ops in {:?} ({:.2} us/op)",
        duplicate_elapsed,
        duplicate_elapsed.as_secs_f64() * 1.0e6 / iterations as f64
    );
    println!(
        "five-scenario validation: {iterations} ops in {:?} ({:.2} us/op)",
        validation_elapsed,
        validation_elapsed.as_secs_f64() * 1.0e6 / iterations as f64
    );
    println!(
        "project semantic refs: {} ops in {:?} ({:.2} ns/op)",
        iterations * 10,
        reference_elapsed,
        reference_elapsed.as_secs_f64() * 1.0e9 / (iterations * 10) as f64
    );
}
