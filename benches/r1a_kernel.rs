use std::hint::black_box;
use std::time::Instant;

use street_concept_designer_kernel::{
    Alignment, ComponentKind, CrossSection, CrossSectionComponent, PiecewiseLinearWidthProfile,
    Point2, SamplingOptions, StationRange, TolerancePolicy, WidthKnot,
};

fn main() {
    let policy = TolerancePolicy::default();
    let alignment = Alignment::line(Point2::new(0.0, 0.0), Point2::new(500.0, 0.0), &policy)
        .expect("canonical benchmark line");
    let samples = alignment
        .sample(SamplingOptions::from_policy(&policy), &policy)
        .expect("canonical benchmark samples");
    let range = StationRange::new(0.0, alignment.length(), &policy).expect("benchmark range");
    let pocket_profile = PiecewiseLinearWidthProfile::new(
        range,
        vec![
            WidthKnot {
                station_m: 0.0,
                width_m: 0.0,
            },
            WidthKnot {
                station_m: 40.0,
                width_m: 3.25,
            },
            WidthKnot {
                station_m: 100.0,
                width_m: 3.25,
            },
            WidthKnot {
                station_m: 140.0,
                width_m: 0.0,
            },
            WidthKnot {
                station_m: 500.0,
                width_m: 0.0,
            },
        ],
        &policy,
    )
    .expect("canonical benchmark pocket profile");
    let pocket = CrossSectionComponent::new(
        "benchmark-pocket",
        ComponentKind::TrafficLane,
        pocket_profile,
    )
    .expect("canonical benchmark pocket");
    let cross_section =
        CrossSection::new(range, vec![pocket], &policy).expect("canonical benchmark cross-section");

    let iterations = 100_000usize;
    let query_start = Instant::now();
    let mut query_sink = 0.0;
    for index in 0..iterations {
        let station_m = (index % 500) as f64;
        let point = alignment.point_at(station_m, &policy).expect("point");
        let tangent = alignment.tangent_at(station_m, &policy).expect("tangent");
        let normal = alignment.normal_at(station_m, &policy).expect("normal");
        query_sink += point.x + point.y + tangent.x + normal.y;
    }
    let query_elapsed = query_start.elapsed();

    let regeneration_start = Instant::now();
    let mut regeneration_sink = 0.0;
    for sample in &samples {
        let states = cross_section
            .states_at(sample.station_m, &policy)
            .expect("profile state");
        regeneration_sink += sample.point.x + states[0].width_m;
    }
    let regeneration_elapsed = regeneration_start.elapsed();

    let profile_start = Instant::now();
    let mut profile_sink = 0.0;
    for index in 0..iterations {
        let station_m = (index % 500) as f64;
        profile_sink += cross_section
            .total_width_at(station_m, &policy)
            .expect("profile width");
    }
    let profile_elapsed = profile_start.elapsed();

    black_box(query_sink);
    black_box(regeneration_sink);
    black_box(profile_sink);
    println!(
        "point/tangent/normal: {iterations} ops in {:?} ({:.2} ns/op)",
        query_elapsed,
        query_elapsed.as_secs_f64() * 1.0e9 / iterations as f64
    );
    println!(
        "500m sampled regeneration: {} points in {:?}",
        samples.len(),
        regeneration_elapsed
    );
    println!(
        "variable-width pocket queries: {iterations} ops in {:?} ({:.2} ns/op)",
        profile_elapsed,
        profile_elapsed.as_secs_f64() * 1.0e9 / iterations as f64
    );
}
