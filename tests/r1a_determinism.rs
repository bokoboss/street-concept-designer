mod support;

use street_concept_designer_kernel::{
    Alignment, ComponentId, CrossSection, CrossSectionComponent, PiecewiseLinearWidthProfile,
    Point2, SamplingOptions, StationRange, TolerancePolicy, WidthKnot,
};

use support::{policy, smooth_s_curve};

fn build_semantic_fixture(policy: &TolerancePolicy) -> (Alignment, CrossSection) {
    let alignment = Alignment::smooth_curve(
        Point2::new(1_000_000.0, -2_000_000.0),
        Point2::new(1_000_050.0, -1_999_900.0),
        Point2::new(1_000_100.0, -2_000_100.0),
        Point2::new(1_000_150.0, -2_000_000.0),
        policy,
    )
    .expect("deterministic curve");
    let range = StationRange::new(0.0, alignment.length(), policy).expect("deterministic range");
    let profile = PiecewiseLinearWidthProfile::new(
        range,
        vec![
            WidthKnot {
                station_m: 0.0,
                width_m: 0.0,
            },
            WidthKnot {
                station_m: 50.0,
                width_m: 3.25,
            },
            WidthKnot {
                station_m: alignment.length() / 2.0,
                width_m: 3.25,
            },
            WidthKnot {
                station_m: alignment.length(),
                width_m: 0.0,
            },
        ],
        policy,
    )
    .expect("deterministic profile");
    let component = CrossSectionComponent::traffic_lane("lane-deterministic", profile)
        .expect("deterministic component");
    let cross_section =
        CrossSection::new(range, vec![component], policy).expect("deterministic cross-section");
    (alignment, cross_section)
}

#[test]
fn equal_semantic_input_produces_byte_stable_value_sequences() {
    let policy = policy();
    let (first_alignment, first_section) = build_semantic_fixture(&policy);
    let (second_alignment, second_section) = build_semantic_fixture(&policy);
    assert_eq!(first_alignment, second_alignment);
    assert_eq!(first_section, second_section);

    let options = SamplingOptions::from_policy(&policy);
    let first_samples = first_alignment
        .sample(options, &policy)
        .expect("first samples");
    let second_samples = second_alignment
        .sample(options, &policy)
        .expect("second samples");
    assert_eq!(first_samples, second_samples);
    for station in [
        0.0,
        25.0,
        50.0,
        first_alignment.length() / 2.0,
        first_alignment.length(),
    ] {
        assert_eq!(
            first_section
                .states_at(station, &policy)
                .expect("first state"),
            second_section
                .states_at(station, &policy)
                .expect("second state")
        );
    }
}

#[test]
fn smooth_curve_construction_and_sampling_are_repeatable() {
    let policy = policy();
    let first = smooth_s_curve();
    let second = smooth_s_curve();
    assert_eq!(first, second);
    assert_eq!(
        first
            .sample(SamplingOptions::from_policy(&policy), &policy)
            .expect("first"),
        second
            .sample(SamplingOptions::from_policy(&policy), &policy)
            .expect("second")
    );
}

#[test]
fn stable_component_lookup_does_not_reassign_identity() {
    let policy = policy();
    let (alignment, section) = build_semantic_fixture(&policy);
    let id = ComponentId::new("lane-deterministic").expect("id");
    for station in [
        0.0,
        50.0,
        25.0,
        alignment.length() / 2.0,
        alignment.length(),
    ] {
        let state = section
            .state_at(&id, station, &policy)
            .expect("lookup")
            .expect("existing lane");
        assert_eq!(state.id, id);
    }
}
