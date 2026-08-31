use street_concept_designer_kernel::{KernelError, TolerancePolicy};

#[test]
fn default_tolerance_policy_is_named_centralized_and_valid() {
    let policy = TolerancePolicy::default();
    policy.validate().expect("default policy");
    assert_eq!(policy.coordinate_coincidence_m, 1.0e-9);
    assert_eq!(policy.station_bound_m, 1.0e-9);
    assert_eq!(policy.angular_rad, 1.0e-12);
    assert_eq!(policy.minimum_alignment_length_m, 1.0e-6);
    assert_eq!(policy.projection_convergence_m, 1.0e-9);
    assert_eq!(policy.adaptive_curve_error_m, 1.0e-3);
    assert_eq!(policy.sampling_max_segment_length_m, 10.0);
}

#[test]
fn invalid_policy_values_are_rejected_instead_of_becoming_local_epsilons() {
    let policy = TolerancePolicy {
        coordinate_coincidence_m: 0.0,
        ..TolerancePolicy::default()
    };
    assert!(matches!(
        policy.validate(),
        Err(KernelError::InvalidParameter {
            field: "coordinate_coincidence_m"
        })
    ));

    let policy = TolerancePolicy {
        adaptive_curve_error_m: f64::NAN,
        ..TolerancePolicy::default()
    };
    assert!(matches!(
        policy.validate(),
        Err(KernelError::NonFiniteInput {
            field: "adaptive_curve_error_m"
        })
    ));

    let policy = TolerancePolicy {
        max_sampling_points: 1,
        ..TolerancePolicy::default()
    };
    assert!(matches!(
        policy.validate(),
        Err(KernelError::InvalidParameter {
            field: "max_sampling_points"
        })
    ));
}
