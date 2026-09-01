use std::hint::black_box;
use std::time::Instant;

use street_concept_designer_kernel::{
    Alignment, ComponentKind, CrossSection, CrossSectionComponent, CrossingRelation,
    JunctionOptions, PiecewiseLinearWidthProfile, Point2, Road, RoadNetwork, TolerancePolicy,
};

fn line_road(id: &str, start: Point2, end: Point2, widths: &[f64]) -> Road {
    let policy = TolerancePolicy::default();
    let alignment = Alignment::line(start, end, &policy).expect("benchmark alignment");
    let range = alignment.station_range();
    let components = widths
        .iter()
        .enumerate()
        .map(|(index, width)| {
            CrossSectionComponent::new(
                format!("lane-{}", index + 1),
                ComponentKind::TrafficLane,
                PiecewiseLinearWidthProfile::constant(range, *width, &policy)
                    .expect("benchmark profile"),
            )
            .expect("benchmark lane")
        })
        .collect();
    let cross_section =
        CrossSection::new(range, components, &policy).expect("benchmark cross-section");
    Road::new(id, alignment, cross_section, &policy).expect("benchmark road")
}

fn t_network() -> RoadNetwork {
    let mut network = RoadNetwork::new();
    network
        .add_road(line_road(
            "main",
            Point2::new(-500.0, 0.0),
            Point2::new(500.0, 0.0),
            &[3.5, 3.5],
        ))
        .expect("benchmark main");
    network
        .add_road(line_road(
            "stem",
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 500.0),
            &[3.5],
        ))
        .expect("benchmark stem");
    network
}

fn four_leg_network() -> RoadNetwork {
    let mut network = RoadNetwork::new();
    network
        .add_road(line_road(
            "east-west",
            Point2::new(-500.0, 0.0),
            Point2::new(500.0, 0.0),
            &[3.5, 3.5],
        ))
        .expect("benchmark east-west");
    network
        .add_road(line_road(
            "north-south",
            Point2::new(0.0, -500.0),
            Point2::new(0.0, 500.0),
            &[3.5, 3.5],
        ))
        .expect("benchmark north-south");
    network
}

fn main() {
    let policy = TolerancePolicy::default();
    let iterations = 2_000usize;

    let candidate_network = four_leg_network();
    let start = Instant::now();
    let mut candidate_sink = 0usize;
    for _ in 0..iterations {
        let candidates = candidate_network
            .detect_candidates(CrossingRelation::AtGrade, &policy)
            .expect("candidate detection");
        candidate_sink += candidates.len();
    }
    let candidate_elapsed = start.elapsed();

    let start = Instant::now();
    let mut t_sink = 0usize;
    for _ in 0..iterations {
        let mut network = t_network();
        let candidates = network
            .detect_candidates(CrossingRelation::AtGrade, &policy)
            .expect("T candidate");
        let id = network
            .create_junction(&candidates[0], "bench-t", JunctionOptions::new(), &policy)
            .expect("T generation");
        t_sink += network
            .junction(&id)
            .expect("T junction")
            .approaches()
            .len();
    }
    let t_elapsed = start.elapsed();

    let start = Instant::now();
    let mut four_sink = 0usize;
    for _ in 0..iterations {
        let mut network = four_leg_network();
        let candidates = network
            .detect_candidates(CrossingRelation::AtGrade, &policy)
            .expect("four-leg candidate");
        let id = network
            .create_junction(
                &candidates[0],
                "bench-four",
                JunctionOptions::new(),
                &policy,
            )
            .expect("four-leg generation");
        four_sink += network
            .junction(&id)
            .expect("four-leg junction")
            .corners()
            .len();
    }
    let four_elapsed = start.elapsed();

    let mut connectivity_network = four_leg_network();
    let candidate = connectivity_network
        .detect_candidates(CrossingRelation::AtGrade, &policy)
        .expect("connectivity candidate")[0]
        .clone();
    let connectivity_id = connectivity_network
        .create_junction(
            &candidate,
            "bench-connectivity",
            JunctionOptions::new(),
            &policy,
        )
        .expect("connectivity junction");
    let start = Instant::now();
    let mut connectivity_sink = 0usize;
    for _ in 0..iterations * 10 {
        connectivity_sink += connectivity_network.connected_road_ids().len();
        connectivity_sink += connectivity_network
            .junction(&connectivity_id)
            .expect("connectivity lookup")
            .lane_connections()
            .len();
    }
    let connectivity_elapsed = start.elapsed();

    let start = Instant::now();
    let mut regeneration_sink = 0usize;
    for _ in 0..iterations {
        let mut network = four_leg_network();
        let candidate = network
            .detect_candidates(CrossingRelation::AtGrade, &policy)
            .expect("regeneration candidate")[0]
            .clone();
        let junction_id = network
            .create_junction(
                &candidate,
                "bench-regeneration",
                JunctionOptions::new(),
                &policy,
            )
            .expect("regeneration junction");
        network
            .replace_road(line_road(
                "east-west",
                Point2::new(-500.0, 0.0),
                Point2::new(500.0, 0.0),
                &[8.0, 8.0],
            ))
            .expect("width change");
        network
            .regenerate_junction(&junction_id, &policy)
            .expect("regeneration");
        regeneration_sink += network
            .junction(&junction_id)
            .expect("regenerated junction")
            .surface()
            .expect("regenerated surface")
            .vertices()
            .len();
    }
    let regeneration_elapsed = start.elapsed();

    black_box(candidate_sink);
    black_box(t_sink);
    black_box(four_sink);
    black_box(connectivity_sink);
    black_box(regeneration_sink);
    println!(
        "candidate detection: {iterations} ops in {:?} ({:.2} us/op)",
        candidate_elapsed,
        candidate_elapsed.as_secs_f64() * 1.0e6 / iterations as f64
    );
    println!(
        "T generation: {iterations} ops in {:?} ({:.2} us/op)",
        t_elapsed,
        t_elapsed.as_secs_f64() * 1.0e6 / iterations as f64
    );
    println!(
        "four-leg generation: {iterations} ops in {:?} ({:.2} us/op)",
        four_elapsed,
        four_elapsed.as_secs_f64() * 1.0e6 / iterations as f64
    );
    println!(
        "connectivity lookup: {} ops in {:?}",
        iterations * 10,
        connectivity_elapsed
    );
    println!(
        "regeneration after width change: {iterations} ops in {:?} ({:.2} us/op)",
        regeneration_elapsed,
        regeneration_elapsed.as_secs_f64() * 1.0e6 / iterations as f64
    );
}
