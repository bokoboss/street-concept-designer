mod support;

use std::collections::HashSet;

use street_concept_designer_kernel::{
    derive_diagnostic_2d, derive_diagnostic_3d, ComponentId, DerivedEngineeringSnapshot, Point2,
    RoadId, SemanticRef,
};
use street_concept_designer_project_core::{
    CoordinateContext, Project, ProjectError, ProjectSemanticRef, Scenario, ScenarioRole,
    TrafficSide,
};

use support::{
    existing_with_alternative, line_road, minimal_existing, network_with_one_road,
    non_trivial_existing, non_trivial_network, policy, project_id, scenario_id,
};

#[test]
fn ids_are_caller_supplied_stable_and_whitespace_invalid() {
    let project_id = project_id("project-stable");
    let scenario_id = scenario_id("scenario-stable");
    assert_eq!(project_id.as_str(), "project-stable");
    assert_eq!(scenario_id.as_str(), "scenario-stable");
    assert!(matches!(
        street_concept_designer_project_core::ProjectId::new(""),
        Err(ProjectError::InvalidProjectId)
    ));
    assert!(matches!(
        street_concept_designer_project_core::ProjectId::new("project id"),
        Err(ProjectError::InvalidProjectId)
    ));
    assert!(matches!(
        street_concept_designer_project_core::ScenarioId::new(""),
        Err(ProjectError::InvalidScenarioId)
    ));
    assert!(matches!(
        street_concept_designer_project_core::ScenarioId::new("scenario\tid"),
        Err(ProjectError::InvalidScenarioId)
    ));

    let mut project = minimal_existing();
    let original_project_id = project.id().clone();
    let original_scenario_id = project.scenarios()[0].id().clone();
    project.rename("Renamed project");
    project
        .set_scenario_locked(&original_scenario_id, false)
        .expect("unlock for rename");
    project
        .rename_scenario(&original_scenario_id, "Renamed scenario")
        .expect("rename scenario");
    assert_eq!(project.id(), &original_project_id);
    assert_eq!(project.scenarios()[0].id(), &original_scenario_id);
}

#[test]
fn project_root_roles_traffic_side_and_coordinate_context_are_explicit() {
    let mut project = minimal_existing();
    assert_eq!(project.traffic_side(), TrafficSide::LeftHand);
    assert!(project.traffic_side().is_left_hand());
    assert!(!project.traffic_side().is_right_hand());
    assert_eq!(project.coordinate_context(), &CoordinateContext::empty());
    assert_eq!(project.scenarios().len(), 1);
    assert_eq!(project.scenarios()[0].role(), ScenarioRole::Existing);
    assert!(project.scenarios()[0].is_locked());
    project.set_traffic_side(TrafficSide::RightHand);
    let context = CoordinateContext::empty()
        .with_crs_identifier("EPSG:32647")
        .expect("CRS identifier")
        .with_description("Project survey coordinates")
        .expect("coordinate description");
    project
        .set_coordinate_context(context.clone())
        .expect("coordinate context");
    assert_eq!(project.traffic_side(), TrafficSide::RightHand);
    assert_eq!(project.coordinate_context(), &context);
    project.validate(&policy()).expect("valid explicit context");

    assert!(matches!(
        CoordinateContext::from_parts(Some(String::new()), None),
        Err(ProjectError::InvalidCoordinateContext)
    ));
    assert!(matches!(
        CoordinateContext::from_parts(Some("EPSG 32647".to_owned()), None),
        Err(ProjectError::InvalidCoordinateContext)
    ));
    assert!(matches!(
        CoordinateContext::from_parts(None, Some("   ".to_owned())),
        Err(ProjectError::InvalidCoordinateContext)
    ));
}

#[test]
fn project_validation_requires_a_scenario_and_is_deterministic() {
    let empty = Project::new(
        project_id("project-empty"),
        "Empty project shell",
        TrafficSide::LeftHand,
        CoordinateContext::empty(),
    );
    assert!(matches!(
        empty.validate(&policy()),
        Err(ProjectError::NoScenarios)
    ));

    let first = non_trivial_existing();
    let second = non_trivial_existing();
    assert_eq!(first, second);
    assert_eq!(
        first.validate(&policy()),
        second.validate(&policy()),
        "equal projects have equal validation results"
    );

    let invalid_context = CoordinateContext::empty();
    let mut project = Project::new(
        project_id("project-shell"),
        "Shell",
        TrafficSide::LeftHand,
        invalid_context,
    );
    project
        .add_scenario(
            Scenario::new(
                scenario_id("scenario-existing"),
                "Existing",
                ScenarioRole::Existing,
                false,
                network_with_one_road(),
            )
            .expect("scenario"),
        )
        .expect("scenario insertion");
    project.validate(&policy()).expect("valid project");
}

#[test]
fn duplicate_scenario_preserves_lineage_and_explicit_duplicate_policy() {
    let mut project = non_trivial_existing();
    let source_id = scenario_id("scenario-existing");
    let source_before = project
        .scenario(&source_id)
        .expect("Existing before duplicate")
        .clone();
    project
        .duplicate_scenario(
            &source_id,
            scenario_id("scenario-alt-a"),
            "Alternative A",
            ScenarioRole::Alternative,
            false,
        )
        .expect("duplicate Existing");
    let existing = project.scenario(&source_id).expect("Existing");
    let alternative = project
        .scenario(&scenario_id("scenario-alt-a"))
        .expect("Alternative A");

    assert_eq!(existing, &source_before);
    assert_ne!(existing.id(), alternative.id());
    assert_eq!(alternative.name(), "Alternative A");
    assert_eq!(alternative.role(), ScenarioRole::Alternative);
    assert!(!alternative.is_locked());
    assert_eq!(existing.network().roads().len(), 2);
    assert_eq!(existing.network().junctions().len(), 1);
    assert_eq!(
        existing
            .network()
            .roads()
            .iter()
            .map(|road| road.id().clone())
            .collect::<Vec<_>>(),
        alternative
            .network()
            .roads()
            .iter()
            .map(|road| road.id().clone())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        existing.network().junctions()[0].id(),
        alternative.network().junctions()[0].id()
    );
    assert_eq!(
        existing.network().roads()[0]
            .cross_section()
            .ordered_component_ids(),
        alternative.network().roads()[0]
            .cross_section()
            .ordered_component_ids()
    );
    assert_eq!(
        existing.network().junctions()[0].lane_connections(),
        alternative.network().junctions()[0].lane_connections()
    );
    project
        .validate(&policy())
        .expect("valid duplicated project");
}

#[test]
fn duplicate_snapshots_are_initially_equivalent_and_renderable() {
    let project = existing_with_alternative();
    let existing = project
        .scenario(&scenario_id("scenario-existing"))
        .expect("Existing");
    let alternative = project
        .scenario(&scenario_id("scenario-alt-a"))
        .expect("Alternative");
    let origin = Point2::new(0.0, 0.0);
    let existing_snapshot =
        DerivedEngineeringSnapshot::derive(existing.network(), origin, &policy())
            .expect("Existing snapshot");
    let alternative_snapshot =
        DerivedEngineeringSnapshot::derive(alternative.network(), origin, &policy())
            .expect("Alternative snapshot");
    assert_eq!(existing_snapshot, alternative_snapshot);
    derive_diagnostic_2d(&existing_snapshot)
        .expect("Existing 2D")
        .validate(&policy())
        .expect("valid Existing 2D");
    derive_diagnostic_3d(&alternative_snapshot)
        .expect("Alternative 3D")
        .validate()
        .expect("valid Alternative 3D");
}

#[test]
fn duplicate_network_is_deep_copy_and_lock_aware_mutation_is_controlled() {
    let mut project = existing_with_alternative();
    let source_id = scenario_id("scenario-existing");
    let alternative_id = scenario_id("scenario-alt-a");
    let source_before = project
        .scenario(&source_id)
        .expect("Existing")
        .network()
        .clone();
    let mut changed_network = project
        .scenario(&alternative_id)
        .expect("Alternative")
        .network()
        .clone();
    changed_network
        .replace_road(line_road(
            "R01",
            Point2::new(-200.0, 0.0),
            Point2::new(200.0, 0.0),
            4.0,
        ))
        .expect("replace duplicate road");
    project
        .replace_scenario_network(&alternative_id, changed_network.clone(), &policy())
        .expect("controlled alternative replacement");
    assert_eq!(
        project.scenario(&source_id).expect("Existing").network(),
        &source_before
    );
    assert_ne!(
        project
            .scenario(&alternative_id)
            .expect("Alternative")
            .network(),
        &source_before
    );
    project
        .validate(&policy())
        .expect("stale derived state is valid");

    let locked_network = project
        .scenario(&source_id)
        .expect("Existing")
        .network()
        .clone();
    assert!(matches!(
        project.replace_scenario_network(&source_id, locked_network, &policy()),
        Err(ProjectError::ScenarioLocked)
    ));
    assert!(matches!(
        project.rename_scenario(&source_id, "must remain protected"),
        Err(ProjectError::ScenarioLocked)
    ));
}

#[test]
fn project_scoped_refs_distinguish_equal_local_identity_and_hash_deterministically() {
    let project = existing_with_alternative();
    let local_ref = SemanticRef::RoadComponent {
        road_id: RoadId::new("R01").expect("road id"),
        component_id: ComponentId::new("lane-1").expect("component id"),
    };
    let existing_ref = project
        .semantic_ref(&scenario_id("scenario-existing"), local_ref.clone())
        .expect("Existing project ref");
    let alternative_ref = project
        .semantic_ref(&scenario_id("scenario-alt-a"), local_ref.clone())
        .expect("Alternative project ref");
    let existing_again =
        ProjectSemanticRef::new(scenario_id("scenario-existing"), local_ref.clone());
    assert_eq!(existing_ref, existing_again);
    assert_ne!(existing_ref, alternative_ref);
    assert_eq!(existing_ref.semantic_ref(), alternative_ref.semantic_ref());
    assert_ne!(existing_ref.scenario_id(), alternative_ref.scenario_id());

    let mut refs = HashSet::new();
    refs.insert(existing_ref);
    refs.insert(alternative_ref);
    refs.insert(existing_again);
    assert_eq!(refs.len(), 2);
}

#[test]
fn duplicate_and_scenario_operations_report_identity_adversaries() {
    let mut project = non_trivial_existing();
    let existing_id = scenario_id("scenario-existing");
    let duplicate = Scenario::new(
        existing_id.clone(),
        "Another Existing",
        ScenarioRole::Existing,
        false,
        network_with_one_road(),
    )
    .expect("duplicate candidate scenario");
    assert!(matches!(
        project.add_scenario(duplicate),
        Err(ProjectError::DuplicateScenarioId)
    ));
    assert!(matches!(
        project.duplicate_scenario(
            &scenario_id("missing"),
            scenario_id("scenario-alt-a"),
            "Alternative A",
            ScenarioRole::Alternative,
            false,
        ),
        Err(ProjectError::MissingScenario)
    ));
    project
        .duplicate_scenario(
            &existing_id,
            scenario_id("scenario-alt-a"),
            "Alternative A",
            ScenarioRole::Alternative,
            false,
        )
        .expect("first duplicate");
    assert!(matches!(
        project.duplicate_scenario(
            &existing_id,
            scenario_id("scenario-alt-a"),
            "Alternative A again",
            ScenarioRole::Alternative,
            false,
        ),
        Err(ProjectError::DuplicateScenarioId)
    ));
}

#[test]
fn last_scenario_and_locked_scenario_removal_are_rejected() {
    let mut project = minimal_existing();
    let existing_id = scenario_id("scenario-existing");
    assert!(matches!(
        project.remove_scenario(&existing_id),
        Err(ProjectError::LastScenarioRemoval)
    ));

    let mut project = existing_with_alternative();
    assert!(matches!(
        project.remove_scenario(&existing_id),
        Err(ProjectError::ScenarioLocked)
    ));
    let alternative_id = scenario_id("scenario-alt-a");
    let removed = project
        .remove_scenario(&alternative_id)
        .expect("editable alternative removal");
    assert_eq!(removed.id(), &alternative_id);
    project.validate(&policy()).expect("Existing remains");
}

#[test]
fn large_coordinate_project_validates_and_derives_without_render_origin_leakage() {
    let policy = policy();
    let mut network = street_concept_designer_kernel::RoadNetwork::new();
    network
        .add_road(line_road(
            "R-large",
            Point2::new(1.0e9, -1.0e9),
            Point2::new(1.0e9 + 120.0, -1.0e9),
            3.5,
        ))
        .expect("large-coordinate road");
    let scenario = Scenario::new(
        scenario_id("scenario-large"),
        "Large coordinates",
        ScenarioRole::Existing,
        false,
        network,
    )
    .expect("large scenario");
    let project = Project::with_scenario(
        project_id("project-large"),
        "Large-coordinate project",
        TrafficSide::RightHand,
        CoordinateContext::empty(),
        scenario,
    )
    .expect("large project");
    project.validate(&policy).expect("large project validation");
    let snapshot = DerivedEngineeringSnapshot::derive(
        project.scenarios()[0].network(),
        Point2::new(1.0e9, -1.0e9),
        &policy,
    )
    .expect("large-coordinate snapshot");
    assert_eq!(snapshot.render_origin(), Point2::new(1.0e9, -1.0e9));
    assert!(snapshot
        .roads()
        .iter()
        .flat_map(|road| road.alignment())
        .all(|sample| sample.point.is_finite()));
}

#[test]
fn stale_r1_derived_state_remains_valid_and_is_not_rendered_as_current_truth() {
    let policy = policy();
    let mut network = non_trivial_network();
    network
        .replace_road(line_road(
            "R01",
            Point2::new(-180.0, 0.0),
            Point2::new(180.0, 0.0),
            3.5,
        ))
        .expect("source road edit");
    network.validate(&policy).expect("stale network validation");
    assert!(network.junctions()[0].surface().is_none());
    let snapshot = DerivedEngineeringSnapshot::derive(&network, Point2::new(0.0, 0.0), &policy)
        .expect("stale-safe snapshot");
    assert!(snapshot.junctions().is_empty());
}

#[test]
fn repeated_duplicate_construction_is_deterministic() {
    let mut first = non_trivial_existing();
    first
        .duplicate_scenario(
            &scenario_id("scenario-existing"),
            scenario_id("scenario-alt-a"),
            "Alternative A",
            ScenarioRole::Alternative,
            false,
        )
        .expect("first duplicate");
    let mut second = non_trivial_existing();
    second
        .duplicate_scenario(
            &scenario_id("scenario-existing"),
            scenario_id("scenario-alt-a"),
            "Alternative A",
            ScenarioRole::Alternative,
            false,
        )
        .expect("second duplicate");
    assert_eq!(first, second);
    assert_eq!(first.validate(&policy()), second.validate(&policy()));
}
