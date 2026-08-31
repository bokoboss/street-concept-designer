mod support;

use street_concept_designer_kernel::{
    ComponentId, ComponentKind, CrossSection, KernelError, PiecewiseLinearWidthProfile, WidthKnot,
};

use support::{
    assert_finite_component_kind, divided_four_lane_cross_section, lane, policy, range,
    right_turn_pocket_cross_section, straight_two_lane_cross_section, width_profile,
};

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1.0e-12,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn canonical_divided_four_lane_fixture_preserves_order_and_identity() {
    let policy = policy();
    let cross_section = divided_four_lane_cross_section();
    let expected = ["lane-1", "lane-2", "median", "lane-3", "lane-4"];
    let ids = cross_section.ordered_component_ids();
    let actual: Vec<&str> = ids.iter().map(ComponentId::as_str).collect();
    assert_eq!(actual, expected);
    let states = cross_section.states_at(250.0, &policy).expect("states");
    assert_eq!(states.len(), expected.len());
    for (state, expected_id) in states.iter().zip(expected) {
        assert_eq!(state.id.as_str(), expected_id);
        assert!(state.active);
        assert_close(
            state.width_m,
            if expected_id == "median" { 4.0 } else { 3.5 },
        );
        assert_finite_component_kind(state.kind);
    }
    assert_close(
        cross_section.total_width_at(250.0, &policy).expect("total"),
        18.0,
    );
}

#[test]
fn canonical_straight_two_lane_fixture_includes_ordered_edge_components() {
    let policy = policy();
    let cross_section = straight_two_lane_cross_section();
    let ids = cross_section.ordered_component_ids();
    let actual: Vec<&str> = ids.iter().map(ComponentId::as_str).collect();
    assert_eq!(
        actual,
        [
            "shoulder-left",
            "lane-1",
            "lane-2",
            "edge-right",
            "shoulder-right"
        ]
    );
    let states = cross_section.states_at(250.0, &policy).expect("states");
    assert_eq!(states[0].kind, ComponentKind::Shoulder);
    assert_eq!(states[1].kind, ComponentKind::TrafficLane);
    assert_eq!(states[3].kind, ComponentKind::EdgeStrip);
    assert_close(
        cross_section.total_width_at(250.0, &policy).expect("total"),
        10.25,
    );
}

#[test]
fn widening_and_narrowing_are_piecewise_linear_and_non_negative() {
    let policy = policy();
    let range = range(100.0);
    let widening = width_profile(range, &[(0.0, 3.0), (50.0, 4.0), (100.0, 5.0)]);
    let narrowing = width_profile(range, &[(0.0, 5.0), (50.0, 4.0), (100.0, 3.0)]);
    assert_close(widening.width_at(0.0, &policy).expect("widen start"), 3.0);
    assert_close(widening.width_at(25.0, &policy).expect("widen middle"), 3.5);
    assert_close(widening.width_at(100.0, &policy).expect("widen end"), 5.0);
    assert_close(
        narrowing.width_at(25.0, &policy).expect("narrow middle"),
        4.5,
    );
    for station in (0..=100).map(f64::from) {
        assert!(widening.width_at(station, &policy).expect("widen sample") >= 0.0);
        assert!(narrowing.width_at(station, &policy).expect("narrow sample") >= 0.0);
    }
}

#[test]
fn abrupt_millimetre_scale_profile_knots_remain_ordered_and_non_negative() {
    let policy = policy();
    let range = range(100.0);
    let profile = width_profile(
        range,
        &[(0.0, 1.0), (50.0, 5.0), (50.001, 0.0), (100.0, 0.0)],
    );
    assert_eq!(profile.knots().len(), 4);
    let width = profile
        .width_at(50.0005, &policy)
        .expect("abrupt interpolation");
    assert!((width - 2.5).abs() <= policy.projection_convergence_m);
    assert!(width >= 0.0);
}

#[test]
fn zero_to_full_and_full_to_zero_keep_lane_identity() {
    let policy = policy();
    let range = range(100.0);
    let add = lane(
        "lane-add",
        width_profile(range, &[(0.0, 0.0), (20.0, 3.25), (100.0, 3.25)]),
    );
    let drop = lane(
        "lane-drop",
        width_profile(range, &[(0.0, 3.25), (80.0, 3.25), (100.0, 0.0)]),
    );
    let add_id = add.id().clone();
    let drop_id = drop.id().clone();
    let cross_section =
        CrossSection::new(range, vec![add, drop], &policy).expect("lifecycle section");

    let at_start = cross_section.states_at(0.0, &policy).expect("start states");
    assert_eq!(at_start[0].id, add_id);
    assert!(!at_start[0].active);
    assert_eq!(at_start[1].id, drop_id);
    assert!(at_start[1].active);
    let at_end = cross_section.states_at(100.0, &policy).expect("end states");
    assert_eq!(at_end[0].id, add_id);
    assert!(at_end[0].active);
    assert_eq!(at_end[1].id, drop_id);
    assert!(!at_end[1].active);
}

#[test]
fn right_turn_pocket_uses_the_same_traffic_lane_lifecycle() {
    let policy = policy();
    let (cross_section, pocket_id) = right_turn_pocket_cross_section();
    let expected_order = ["lane-through", "lane-right-turn-storage"];
    let ids = cross_section.ordered_component_ids();
    let actual: Vec<&str> = ids.iter().map(ComponentId::as_str).collect();
    assert_eq!(actual, expected_order);
    for (station, expected_width, expected_active) in [
        (0.0, 0.0, false),
        (35.0, 1.625, true),
        (50.0, 3.25, true),
        (100.0, 3.25, true),
        (145.0, 1.625, true),
        (180.0, 0.0, false),
    ] {
        let state = cross_section
            .state_at(&pocket_id, station, &policy)
            .expect("pocket state")
            .expect("pocket identity");
        assert_eq!(state.id, pocket_id);
        assert_eq!(state.kind, ComponentKind::TrafficLane);
        assert_close(state.width_m, expected_width);
        assert_eq!(state.active, expected_active);
    }
}

#[test]
fn invalid_profiles_and_component_order_are_rejected() {
    let policy = policy();
    let range = range(10.0);
    assert!(matches!(
        PiecewiseLinearWidthProfile::new(
            range,
            vec![
                WidthKnot {
                    station_m: 0.0,
                    width_m: 3.0
                },
                WidthKnot {
                    station_m: 5.0,
                    width_m: -0.1
                },
                WidthKnot {
                    station_m: 10.0,
                    width_m: 3.0
                },
            ],
            &policy
        ),
        Err(KernelError::InvalidWidthProfile)
    ));
    assert!(matches!(
        PiecewiseLinearWidthProfile::new(
            range,
            vec![
                WidthKnot {
                    station_m: 0.0,
                    width_m: 3.0
                },
                WidthKnot {
                    station_m: 6.0,
                    width_m: 3.0
                },
                WidthKnot {
                    station_m: 5.0,
                    width_m: 3.0
                },
                WidthKnot {
                    station_m: 10.0,
                    width_m: 3.0
                },
            ],
            &policy
        ),
        Err(KernelError::InvalidWidthProfile)
    ));
    let profile = PiecewiseLinearWidthProfile::constant(range, 3.0, &policy).expect("profile");
    let first = lane("duplicate", profile.clone());
    let second = lane("duplicate", profile);
    assert!(matches!(
        CrossSection::new(range, vec![first, second], &policy),
        Err(KernelError::DuplicateComponentId)
    ));
}
