use std::hint::black_box;
use std::time::Instant;

use street_concept_designer_kernel::{
    derive_diagnostic_2d, derive_diagnostic_3d, Alignment, ComponentKind, CrossSection,
    CrossSectionComponent, CrossingRelation, DerivedEngineeringSnapshot, JunctionOptions,
    PiecewiseLinearWidthProfile, Point2, Road, RoadNetwork, TolerancePolicy,
};

fn policy() -> TolerancePolicy {
    TolerancePolicy::default()
}

fn line_road(id: &str, start: Point2, end: Point2, widths: &[f64]) -> Road {
    let policy = policy();
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

fn network() -> RoadNetwork {
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
        .add_road(line_road(
            "bypass",
            Point2::new(-400.0, 100.0),
            Point2::new(400.0, 160.0),
            &[3.25, 3.25],
        ))
        .expect("benchmark bypass");
    let candidate = network
        .detect_candidate(
            &street_concept_designer_kernel::RoadId::new("main").expect("main id"),
            &street_concept_designer_kernel::RoadId::new("stem").expect("stem id"),
            CrossingRelation::AtGrade,
            &policy(),
        )
        .expect("benchmark candidate")
        .expect("benchmark crossing");
    network
        .create_junction(
            &candidate,
            "benchmark-junction",
            JunctionOptions::new(),
            &policy(),
        )
        .expect("benchmark junction");
    network
}

fn changed_network(source: &RoadNetwork) -> RoadNetwork {
    let mut changed = source.clone();
    changed
        .replace_road(line_road(
            "main",
            Point2::new(-500.0, 0.0),
            Point2::new(500.0, 0.0),
            &[4.5, 4.5],
        ))
        .expect("benchmark width update");
    changed
        .regenerate_junction(
            &street_concept_designer_kernel::JunctionId::new("benchmark-junction")
                .expect("junction id"),
            &policy(),
        )
        .expect("benchmark junction regeneration");
    changed
}

fn main() {
    let policy = policy();
    let iterations = 1_000usize;
    let source = network();
    let origin = Point2::new(0.0, 0.0);

    let start = Instant::now();
    let mut snapshot_sink = 0usize;
    for _ in 0..iterations {
        let snapshot = DerivedEngineeringSnapshot::derive(&source, origin, &policy)
            .expect("snapshot derivation");
        snapshot_sink += snapshot.roads().len() + snapshot.junctions().len();
        black_box(snapshot);
    }
    let snapshot_elapsed = start.elapsed();

    let snapshot = DerivedEngineeringSnapshot::derive(&source, origin, &policy)
        .expect("shared benchmark snapshot");
    let start = Instant::now();
    let mut two_d_sink = 0usize;
    for _ in 0..iterations {
        let scene = derive_diagnostic_2d(&snapshot).expect("2D derivation");
        two_d_sink += scene.primitives().len();
        black_box(scene);
    }
    let two_d_elapsed = start.elapsed();

    let start = Instant::now();
    let mut three_d_sink = 0usize;
    for _ in 0..iterations {
        let scene = derive_diagnostic_3d(&snapshot).expect("3D derivation");
        three_d_sink += scene.meshes().len();
        black_box(scene);
    }
    let three_d_elapsed = start.elapsed();

    let start = Instant::now();
    let mut rebuild_sink = 0usize;
    for _ in 0..iterations {
        let snapshot =
            DerivedEngineeringSnapshot::derive(&source, origin, &policy).expect("clean snapshot");
        let two_d = derive_diagnostic_2d(&snapshot).expect("clean 2D");
        let three_d = derive_diagnostic_3d(&snapshot).expect("clean 3D");
        rebuild_sink += two_d.primitives().len() + three_d.meshes().len();
        black_box((snapshot, two_d, three_d));
    }
    let rebuild_elapsed = start.elapsed();

    let start = Instant::now();
    let mut update_sink = 0usize;
    for _ in 0..iterations {
        let changed = changed_network(&source);
        let snapshot = DerivedEngineeringSnapshot::derive(&changed, origin, &policy)
            .expect("updated snapshot");
        let two_d = derive_diagnostic_2d(&snapshot).expect("updated 2D");
        let three_d = derive_diagnostic_3d(&snapshot).expect("updated 3D");
        update_sink += snapshot.roads().len() + two_d.primitives().len() + three_d.meshes().len();
        black_box((changed, snapshot, two_d, three_d));
    }
    let update_elapsed = start.elapsed();

    black_box(snapshot_sink);
    black_box(two_d_sink);
    black_box(three_d_sink);
    black_box(rebuild_sink);
    black_box(update_sink);
    println!(
        "fixture=3-road-plus-junction iterations={iterations} platform={} arch={} toolchain=rustc-1.98.0",
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    print_rate("shared snapshot", iterations, snapshot_elapsed);
    print_rate("2D DTO from shared snapshot", iterations, two_d_elapsed);
    print_rate(
        "3D buffers from shared snapshot",
        iterations,
        three_d_elapsed,
    );
    print_rate("complete clean rebuild", iterations, rebuild_elapsed);
    print_rate(
        "semantic width update and clean rebuild",
        iterations,
        update_elapsed,
    );
}

fn print_rate(label: &str, iterations: usize, elapsed: std::time::Duration) {
    println!(
        "{label}: {iterations} ops in {elapsed:?} ({:.2} us/op)",
        elapsed.as_secs_f64() * 1.0e6 / iterations as f64
    );
}
