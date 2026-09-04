use std::hint::black_box;
use std::time::Instant;

use street_concept_designer_kernel::{
    Alignment, AlignmentSegment, CrossSection, CrossSectionComponent, DerivedEngineeringSnapshot,
    PiecewiseLinearWidthProfile, Point2, Road, RoadNetwork, SamplingOptions, TolerancePolicy,
};

fn policy() -> TolerancePolicy {
    TolerancePolicy::default()
}

fn alignment_with_count(count: usize) -> Alignment {
    let policy = policy();
    let segments = (0..count)
        .map(|index| {
            let start_x = index as f64 * 10.0;
            AlignmentSegment::line(
                format!("segment-{index}"),
                Point2::new(start_x, 0.0),
                Point2::new(start_x + 10.0, 0.0),
                &policy,
            )
            .expect("benchmark segment")
        })
        .collect();
    Alignment::from_segments(segments, &policy).expect("benchmark alignment")
}

fn network_with_alignment(alignment: Alignment) -> RoadNetwork {
    let policy = policy();
    let range = alignment.station_range();
    let lane = CrossSectionComponent::traffic_lane(
        "lane-1",
        PiecewiseLinearWidthProfile::constant(range, 3.5, &policy).expect("lane profile"),
    )
    .expect("lane");
    let cross_section = CrossSection::new(range, vec![lane], &policy).expect("cross-section");
    let road = Road::new("benchmark-road", alignment, cross_section, &policy).expect("road");
    let mut network = RoadNetwork::new();
    network.add_road(road).expect("road network");
    network
}

fn elapsed_ms(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}

fn main() {
    let policy = policy();
    let options = SamplingOptions {
        max_chord_error_m: policy.adaptive_curve_error_m,
        max_segment_length_m: policy.sampling_max_segment_length_m,
        max_points: policy.max_sampling_points,
        max_depth: policy.max_sampling_depth,
    };
    println!(
        "R3A composite alignment benchmark: target_os={} target_arch={} rust={}",
        std::env::consts::OS,
        std::env::consts::ARCH,
        rustc_version(),
    );
    for count in [3usize, 10, 50] {
        let iterations = 1000usize;
        let start = Instant::now();
        for _ in 0..iterations {
            black_box(alignment_with_count(count));
        }
        println!(
            "segments={count} construction+validation: {iterations} ops in {:.3} ms ({:.3} us/op)",
            elapsed_ms(start),
            elapsed_ms(start) * 1000.0 / iterations as f64
        );

        let alignment = alignment_with_count(count);
        let start = Instant::now();
        for _ in 0..iterations {
            black_box(alignment.sample(options, &policy).expect("sample"));
        }
        println!(
            "segments={count} sampling: {iterations} ops in {:.3} ms ({:.3} us/op)",
            elapsed_ms(start),
            elapsed_ms(start) * 1000.0 / iterations as f64
        );

        let query = Point2::new(alignment.length() * 0.37, 2.0);
        let projection_iterations = 10_000usize;
        let start = Instant::now();
        for _ in 0..projection_iterations {
            black_box(alignment.project(query, &policy).expect("projection"));
        }
        println!(
            "segments={count} projection: {projection_iterations} ops in {:.3} ms ({:.3} ns/op)",
            elapsed_ms(start),
            elapsed_ms(start) * 1_000_000.0 / projection_iterations as f64
        );

        let network = network_with_alignment(alignment);
        let start = Instant::now();
        for _ in 0..iterations {
            black_box(
                DerivedEngineeringSnapshot::derive(&network, Point2::new(0.0, 0.0), &policy)
                    .expect("R1C rebuild"),
            );
        }
        println!(
            "segments={count} R1C rebuild: {iterations} ops in {:.3} ms ({:.3} us/op)",
            elapsed_ms(start),
            elapsed_ms(start) * 1000.0 / iterations as f64
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
