mod support;

use street_concept_designer_kernel::{
    derive_diagnostic_2d, derive_diagnostic_3d, ApproachId, CandidateDisposition, CornerId,
    DerivedEngineeringSnapshot, Diagnostic2D, Diagnostic3D, JunctionId, JunctionOptions,
    JunctionStatus, KernelError, LaneConnection, Movement, Point2,
};
use street_concept_designer_project_core::{
    CoordinateContext, Project, ProjectError, Scenario, ScenarioRole, TrafficSide,
};
use street_concept_designer_project_session::{
    Command, HistoryItem, PreviewResult, ProjectSession, SessionError, Transaction, TransactionId,
};

use support::{
    corner_id, junction_candidate, line_road, line_road_with_lanes, project_id,
    project_without_junction, representative_project, scenario_id, single_existing_project,
    valid_manual_connection,
};

fn transaction(
    id: &str,
    expected_revision: u64,
    summary: &str,
    commands: Vec<Command>,
) -> Transaction {
    Transaction::new(
        TransactionId::new(id).expect("valid transaction id"),
        expected_revision,
        summary,
        commands,
    )
    .expect("valid transaction")
}

fn representative_session() -> ProjectSession {
    ProjectSession::new(representative_project()).expect("representative session")
}

fn junction_id(value: &str) -> JunctionId {
    JunctionId::new(value).expect("valid junction id")
}

fn corner_id_value(value: &str) -> CornerId {
    CornerId::new(value).expect("valid corner id")
}

fn unchanged(
    session: &ProjectSession,
    project: &Project,
    revision: u64,
    undo: &[HistoryItem],
    redo: &[HistoryItem],
) {
    assert_eq!(session.project(), project);
    assert_eq!(session.revision(), revision);
    assert_eq!(session.undo_history(), undo);
    assert_eq!(session.redo_history(), redo);
}

fn clean_rebuild(
    project: &Project,
) -> Vec<(DerivedEngineeringSnapshot, Diagnostic2D, Diagnostic3D)> {
    project
        .scenarios()
        .iter()
        .map(|scenario| {
            let snapshot = DerivedEngineeringSnapshot::derive(
                scenario.network(),
                Point2::new(0.0, 0.0),
                &street_concept_designer_kernel::TolerancePolicy::default(),
            )
            .expect("clean R1C snapshot");
            let diagnostic_2d = derive_diagnostic_2d(&snapshot).expect("clean R1C 2D");
            let diagnostic_3d = derive_diagnostic_3d(&snapshot).expect("clean R1C 3D");
            (snapshot, diagnostic_2d, diagnostic_3d)
        })
        .collect()
}

fn assert_persisted_state(project: &Project) {
    let bytes = street_concept_designer_project_io::encode_project_to_bytes(project)
        .expect("encode canonical project");
    let loaded = street_concept_designer_project_io::decode_project_from_bytes(&bytes)
        .expect("decode canonical project");
    assert_eq!(&loaded, project);
    assert_eq!(clean_rebuild(&loaded), clean_rebuild(project));
}

#[test]
fn transaction_ids_and_transactions_are_caller_supplied_and_validated() {
    assert!(matches!(
        TransactionId::new(""),
        Err(SessionError::InvalidTransactionId)
    ));
    assert!(matches!(
        TransactionId::new("contains whitespace"),
        Err(SessionError::InvalidTransactionId)
    ));

    let id = TransactionId::new("tx-stable-001").expect("valid id");
    assert_eq!(id.as_str(), "tx-stable-001");
    let valid = transaction(
        "tx-stable-001",
        7,
        "  Add a road  ",
        vec![Command::AddRoad {
            scenario_id: scenario_id("scenario-alt-a"),
            road: line_road("R03", Point2::new(300.0, 0.0), Point2::new(420.0, 0.0), 3.5),
        }],
    );
    assert_eq!(valid.id().as_str(), "tx-stable-001");
    assert_eq!(valid.expected_revision(), 7);
    assert_eq!(valid.summary(), "Add a road");
    assert_eq!(valid.commands().len(), 1);

    assert!(matches!(
        Transaction::new(id.clone(), 0, "valid", Vec::new()),
        Err(SessionError::EmptyTransaction)
    ));
    assert!(matches!(
        Transaction::new(
            id,
            0,
            "   ",
            vec![Command::SetScenarioLock {
                scenario_id: scenario_id("scenario-alt-a"),
                locked: true,
            }]
        ),
        Err(SessionError::InvalidTransactionSummary)
    ));
}

#[test]
fn session_validates_initial_project_and_starts_without_history() {
    let empty = Project::new(
        project_id("project-empty"),
        "Empty",
        TrafficSide::LeftHand,
        CoordinateContext::empty(),
    );
    assert!(matches!(
        ProjectSession::new(empty),
        Err(SessionError::Project(ProjectError::NoScenarios))
    ));

    let session = ProjectSession::new(single_existing_project()).expect("valid session");
    assert_eq!(session.revision(), 0);
    assert!(session.undo_history().is_empty());
    assert!(session.redo_history().is_empty());
}

#[test]
fn preview_is_immutable_and_commit_uses_the_same_candidate_result() {
    let mut session = representative_session();
    let before = session.project().clone();
    let before_revision = session.revision();
    let before_undo = session.undo_history();
    let before_redo = session.redo_history();
    let tx = transaction(
        "preview-add-road",
        0,
        "Add Road R03",
        vec![Command::AddRoad {
            scenario_id: scenario_id("scenario-alt-a"),
            road: line_road("R03", Point2::new(300.0, 0.0), Point2::new(420.0, 0.0), 3.5),
        }],
    );

    let preview: PreviewResult = session.preview(&tx).expect("preview");
    let candidate = preview.candidate_project().clone();
    assert_ne!(&candidate, &before);
    assert_eq!(preview.project(), &candidate);
    assert_eq!(preview.transaction_id(), tx.id());
    assert_eq!(preview.summary(), tx.summary());
    unchanged(
        &session,
        &before,
        before_revision,
        &before_undo,
        &before_redo,
    );

    let item = session.commit(&tx).expect("commit same transaction");
    assert_eq!(&candidate, session.project());
    assert_eq!(item.transaction_id(), tx.id());
    assert_eq!(item.summary(), "Add Road R03");
    assert_eq!(session.revision(), 1);
    assert_eq!(session.undo_history(), vec![item]);
    assert!(session.redo_history().is_empty());
}

#[test]
fn add_scenario_and_multi_command_commit_use_typed_commands() {
    let mut session = ProjectSession::new(single_existing_project()).expect("session");
    let added = Scenario::new(
        scenario_id("scenario-added"),
        "Added scenario",
        ScenarioRole::Alternative,
        false,
        street_concept_designer_kernel::RoadNetwork::new(),
    )
    .expect("scenario");
    session
        .commit(&transaction(
            "add-scenario",
            0,
            "Add scenario",
            vec![Command::AddScenario { scenario: added }],
        ))
        .expect("add scenario");
    assert!(session
        .project()
        .scenario(&scenario_id("scenario-added"))
        .is_some());

    let mut session = representative_session();
    let before_multi = session.project().clone();
    session
        .commit(&transaction(
            "multi-command",
            0,
            "Add road and rename alternative",
            vec![
                Command::AddRoad {
                    scenario_id: scenario_id("scenario-alt-a"),
                    road: line_road("R03", Point2::new(300.0, 0.0), Point2::new(420.0, 0.0), 3.5),
                },
                Command::RenameScenario {
                    scenario_id: scenario_id("scenario-alt-a"),
                    new_name: "Alternative A revised".to_owned(),
                },
            ],
        ))
        .expect("multi-command commit");
    let after_multi = session.project().clone();
    assert_eq!(session.revision(), 1);
    assert_eq!(session.undo_history().len(), 1);
    assert_eq!(
        session
            .project()
            .scenario(&scenario_id("scenario-alt-a"))
            .expect("alternative")
            .name(),
        "Alternative A revised"
    );
    assert!(session
        .project()
        .scenario(&scenario_id("scenario-alt-a"))
        .expect("alternative")
        .network()
        .road(&street_concept_designer_kernel::RoadId::new("R03").expect("R03"))
        .is_some());
    session.undo(1).expect("undo multi-command transaction");
    assert_eq!(session.project(), &before_multi);
    session.redo(2).expect("redo multi-command transaction");
    assert_eq!(session.project(), &after_multi);
}

#[test]
fn failed_multi_command_transaction_is_fully_atomic() {
    let mut session = representative_session();
    let before = session.project().clone();
    let before_revision = session.revision();
    let before_undo = session.undo_history();
    let before_redo = session.redo_history();
    let add_road = || Command::AddRoad {
        scenario_id: scenario_id("scenario-alt-a"),
        road: line_road("R03", Point2::new(300.0, 0.0), Point2::new(420.0, 0.0), 3.5),
    };
    let error = session
        .commit(&transaction(
            "atomic-failure",
            0,
            "Add duplicate road transaction",
            vec![add_road(), add_road()],
        ))
        .expect_err("second command must fail");
    assert_eq!(error, SessionError::Kernel(KernelError::DuplicateRoadId));
    unchanged(
        &session,
        &before,
        before_revision,
        &before_undo,
        &before_redo,
    );
}

#[test]
fn stale_proposals_remain_stale_after_commit_undo_and_redo() {
    let mut session = representative_session();
    let old = transaction(
        "old-proposal",
        0,
        "Add Road R03",
        vec![Command::AddRoad {
            scenario_id: scenario_id("scenario-alt-a"),
            road: line_road("R03", Point2::new(300.0, 0.0), Point2::new(420.0, 0.0), 3.5),
        }],
    );
    session.commit(&old).expect("first commit");
    assert!(matches!(
        session.commit(&old),
        Err(SessionError::StaleRevision {
            expected_revision: 0,
            current_revision: 1
        })
    ));
    session.undo(1).expect("undo");
    assert_eq!(session.revision(), 2);
    assert!(matches!(
        session.commit(&old),
        Err(SessionError::StaleRevision {
            expected_revision: 0,
            current_revision: 2
        })
    ));
    session.redo(2).expect("redo");
    assert_eq!(session.revision(), 3);
    assert!(matches!(
        session.commit(&old),
        Err(SessionError::StaleRevision {
            expected_revision: 0,
            current_revision: 3
        })
    ));
}

#[test]
fn undo_and_redo_restore_exact_semantic_snapshots() {
    let mut session = representative_session();
    let before = session.project().clone();
    let target = scenario_id("scenario-alt-a");
    let corner = corner_id(&before, &target);
    session
        .commit(&transaction(
            "set-corner",
            0,
            "Set corner radius to 14 m",
            vec![Command::SetCornerRadius {
                scenario_id: target,
                junction_id: junction_id("J01"),
                corner_id: corner,
                radius_m: 14.0,
            }],
        ))
        .expect("corner edit");
    let after = session.project().clone();
    assert_ne!(after, before);

    let item = session.undo(1).expect("undo");
    assert_eq!(session.project(), &before);
    assert_eq!(session.revision(), 2);
    assert_eq!(item.summary(), "Set corner radius to 14 m");
    assert!(session.undo_history().is_empty());
    assert_eq!(session.redo_history(), vec![item.clone()]);

    let redo_item = session.redo(2).expect("redo");
    assert_eq!(session.project(), &after);
    assert_eq!(session.revision(), 3);
    assert_eq!(redo_item, item);
    assert_eq!(session.undo_history(), vec![item]);
    assert!(session.redo_history().is_empty());
}

#[test]
fn divergent_commit_clears_redo_history() {
    let mut session = representative_session();
    session
        .commit(&transaction(
            "commit-a",
            0,
            "Rename A once",
            vec![Command::RenameScenario {
                scenario_id: scenario_id("scenario-alt-a"),
                new_name: "Alternative A once".to_owned(),
            }],
        ))
        .expect("commit A");
    session.undo(1).expect("undo A");
    assert_eq!(session.revision(), 2);
    session
        .commit(&transaction(
            "commit-b",
            2,
            "Rename A differently",
            vec![Command::RenameScenario {
                scenario_id: scenario_id("scenario-alt-a"),
                new_name: "Alternative A differently".to_owned(),
            }],
        ))
        .expect("divergent commit B");
    assert_eq!(session.revision(), 3);
    assert!(session.redo_history().is_empty());
    assert!(matches!(
        session.redo(3),
        Err(SessionError::NoRedoAvailable)
    ));
    assert_eq!(
        session
            .project()
            .scenario(&scenario_id("scenario-alt-a"))
            .expect("alternative")
            .name(),
        "Alternative A differently"
    );
}

#[test]
fn history_metadata_keeps_redo_order_without_exposing_snapshots() {
    let mut session = representative_session();
    session
        .commit(&transaction(
            "metadata-a",
            0,
            "Rename A once",
            vec![Command::RenameScenario {
                scenario_id: scenario_id("scenario-alt-a"),
                new_name: "Alternative A once".to_owned(),
            }],
        ))
        .expect("commit A");
    session
        .commit(&transaction(
            "metadata-b",
            1,
            "Rename A twice",
            vec![Command::RenameScenario {
                scenario_id: scenario_id("scenario-alt-a"),
                new_name: "Alternative A twice".to_owned(),
            }],
        ))
        .expect("commit B");

    session.undo(2).expect("undo B");
    session.undo(3).expect("undo A");
    assert_eq!(
        session
            .redo_history()
            .iter()
            .map(|item| item.transaction_id().as_str())
            .collect::<Vec<_>>(),
        vec!["metadata-a", "metadata-b"]
    );
    session.redo(4).expect("redo A");
    assert_eq!(
        session
            .project()
            .scenario(&scenario_id("scenario-alt-a"))
            .expect("alternative")
            .name(),
        "Alternative A once"
    );
}

#[test]
fn repeated_commit_undo_redo_cycles_are_deterministic_and_monotonic() {
    let mut session = representative_session();
    let before = session.project().clone();
    let tx = transaction(
        "cycle",
        0,
        "Set cycle corner radius",
        vec![Command::SetCornerRadius {
            scenario_id: scenario_id("scenario-alt-a"),
            junction_id: junction_id("J01"),
            corner_id: corner_id(&before, &scenario_id("scenario-alt-a")),
            radius_m: 15.0,
        }],
    );
    session.commit(&tx).expect("commit");
    let after = session.project().clone();
    session.undo(1).expect("undo 1");
    assert_eq!(session.project(), &before);
    session.redo(2).expect("redo 1");
    assert_eq!(session.project(), &after);
    session.undo(3).expect("undo 2");
    assert_eq!(session.project(), &before);
    session.redo(4).expect("redo 2");
    assert_eq!(session.project(), &after);
    assert_eq!(session.revision(), 5);
}

#[test]
fn no_history_operations_fail_without_mutation() {
    let mut session = representative_session();
    let before = session.project().clone();
    assert!(matches!(
        session.undo(0),
        Err(SessionError::NoUndoAvailable)
    ));
    assert!(matches!(
        session.redo(0),
        Err(SessionError::NoRedoAvailable)
    ));
    assert_eq!(session.project(), &before);
    assert_eq!(session.revision(), 0);
}

#[test]
fn locked_scenario_rejects_all_ordinary_engineering_mutations_in_preview_and_commit() {
    let project = single_existing_project();
    let existing = scenario_id("scenario-existing");
    let candidate = junction_candidate(project.scenario(&existing).expect("Existing").network());
    let corner = corner_id(&project, &existing);
    let commands = vec![
        Command::RenameScenario {
            scenario_id: existing.clone(),
            new_name: "Not allowed".to_owned(),
        },
        Command::AddRoad {
            scenario_id: existing.clone(),
            road: line_road("R03", Point2::new(300.0, 0.0), Point2::new(420.0, 0.0), 3.5),
        },
        Command::ReplaceRoad {
            scenario_id: existing.clone(),
            road: line_road_with_lanes(
                "R01",
                Point2::new(-150.0, 0.0),
                Point2::new(150.0, 0.0),
                4.0,
                &["lane-1", "lane-2"],
            ),
        },
        Command::CreateJunctionFromCandidate {
            scenario_id: existing.clone(),
            candidate,
            junction_id: junction_id("J02"),
            options: JunctionOptions::new(),
        },
        Command::SetCornerRadius {
            scenario_id: existing.clone(),
            junction_id: junction_id("J01"),
            corner_id: corner,
            radius_m: 14.0,
        },
        Command::ReplaceLaneConnections {
            scenario_id: existing.clone(),
            junction_id: junction_id("J01"),
            connections: vec![valid_manual_connection("manual-locked")],
        },
    ];

    for (index, command) in commands.into_iter().enumerate() {
        let mut session = ProjectSession::new(project.clone()).expect("locked session");
        let before = session.project().clone();
        let tx = transaction(
            &format!("locked-{index}"),
            0,
            "Locked mutation",
            vec![command],
        );
        assert!(matches!(
            session.preview(&tx),
            Err(SessionError::Project(ProjectError::ScenarioLocked))
        ));
        assert!(matches!(
            session.commit(&tx),
            Err(SessionError::Project(ProjectError::ScenarioLocked))
        ));
        assert_eq!(session.project(), &before);
        assert_eq!(session.revision(), 0);
        assert!(session.undo_history().is_empty());
        assert!(session.redo_history().is_empty());
    }
}

#[test]
fn locked_source_can_be_duplicated_and_explicit_unlock_allows_editing() {
    let mut session = ProjectSession::new(single_existing_project()).expect("session");
    let source = session
        .project()
        .scenario(&scenario_id("scenario-existing"))
        .expect("Existing")
        .clone();
    session
        .commit(&transaction(
            "duplicate-locked",
            0,
            "Duplicate Existing as Alternative A",
            vec![Command::DuplicateScenario {
                source_scenario_id: scenario_id("scenario-existing"),
                new_scenario_id: scenario_id("scenario-alt-a"),
                new_name: "Alternative A".to_owned(),
                role: ScenarioRole::Alternative,
                locked: false,
            }],
        ))
        .expect("duplicate locked source");
    assert_eq!(
        session
            .project()
            .scenario(&scenario_id("scenario-existing"))
            .expect("Existing"),
        &source
    );
    assert_eq!(
        session
            .project()
            .scenario(&scenario_id("scenario-alt-a"))
            .expect("Alternative")
            .network(),
        source.network()
    );

    let mut session = ProjectSession::new(single_existing_project()).expect("session");
    session
        .commit(&transaction(
            "unlock-existing",
            0,
            "Unlock Existing",
            vec![Command::SetScenarioLock {
                scenario_id: scenario_id("scenario-existing"),
                locked: false,
            }],
        ))
        .expect("explicit unlock");
    session
        .commit(&transaction(
            "edit-unlocked",
            1,
            "Add Road R03 after unlock",
            vec![Command::AddRoad {
                scenario_id: scenario_id("scenario-existing"),
                road: line_road("R03", Point2::new(300.0, 0.0), Point2::new(420.0, 0.0), 3.5),
            }],
        ))
        .expect("edit after unlock");
    assert_eq!(session.revision(), 2);

    let mut session = ProjectSession::new(representative_project()).expect("session");
    session
        .commit(&transaction(
            "lock-alternative",
            0,
            "Lock Alternative A",
            vec![Command::SetScenarioLock {
                scenario_id: scenario_id("scenario-alt-a"),
                locked: true,
            }],
        ))
        .expect("explicit lock");
    assert!(session
        .project()
        .scenario(&scenario_id("scenario-alt-a"))
        .expect("Alternative")
        .is_locked());
    session
        .commit(&transaction(
            "unlock-alternative",
            1,
            "Unlock Alternative A",
            vec![Command::SetScenarioLock {
                scenario_id: scenario_id("scenario-alt-a"),
                locked: false,
            }],
        ))
        .expect("explicit unlock");
    assert!(!session
        .project()
        .scenario(&scenario_id("scenario-alt-a"))
        .expect("Alternative")
        .is_locked());
}

#[test]
fn scenario_local_targets_do_not_cross_mutate_equal_local_ids() {
    let mut session = representative_session();
    let before_project = session.project().clone();
    let existing = scenario_id("scenario-existing");
    let alternative = scenario_id("scenario-alt-a");
    let existing_before = session
        .project()
        .scenario(&existing)
        .expect("Existing")
        .clone();
    let alternative_before = session
        .project()
        .scenario(&alternative)
        .expect("Alternative")
        .clone();
    assert_eq!(
        existing_before.network().junctions()[0].id(),
        alternative_before.network().junctions()[0].id()
    );

    let corner = corner_id(&session.project().clone(), &alternative);
    session
        .commit(&transaction(
            "alternative-corner",
            0,
            "Set Alternative corner radius",
            vec![Command::SetCornerRadius {
                scenario_id: alternative.clone(),
                junction_id: junction_id("J01"),
                corner_id: corner,
                radius_m: 16.0,
            }],
        ))
        .expect("Alternative edit");
    assert_eq!(
        session.project().scenario(&existing).expect("Existing"),
        &existing_before
    );
    assert_ne!(
        session
            .project()
            .scenario(&alternative)
            .expect("Alternative"),
        &alternative_before
    );
    session.undo(1).expect("undo Alternative edit");
    assert_eq!(session.project(), &before_project);
    assert_eq!(
        session.project().scenario(&existing).expect("Existing"),
        &existing_before
    );
    assert_eq!(
        session
            .project()
            .scenario(&alternative)
            .expect("Alternative"),
        &alternative_before
    );
    session.redo(2).expect("redo Alternative edit");
    assert_eq!(
        session.project().scenario(&existing).expect("Existing"),
        &existing_before
    );
    assert_eq!(
        session
            .project()
            .scenario(&alternative)
            .expect("Alternative")
            .network()
            .junction(&junction_id("J01"))
            .expect("junction")
            .corner_radii_m()[0],
        16.0
    );
}

#[test]
fn add_and_replace_road_are_scenario_scoped_and_replace_regenerates_junctions() {
    let mut session = representative_session();
    let existing_before = session
        .project()
        .scenario(&scenario_id("scenario-existing"))
        .expect("Existing")
        .network()
        .clone();
    session
        .commit(&transaction(
            "add-alternative-road",
            0,
            "Add Road R03",
            vec![Command::AddRoad {
                scenario_id: scenario_id("scenario-alt-a"),
                road: line_road("R03", Point2::new(300.0, 0.0), Point2::new(420.0, 0.0), 3.5),
            }],
        ))
        .expect("AddRoad");
    assert!(session
        .project()
        .scenario(&scenario_id("scenario-alt-a"))
        .expect("Alternative")
        .network()
        .road(&street_concept_designer_kernel::RoadId::new("R03").expect("R03"))
        .is_some());
    assert_eq!(
        session
            .project()
            .scenario(&scenario_id("scenario-existing"))
            .expect("Existing")
            .network(),
        &existing_before
    );

    let replacement = line_road_with_lanes(
        "R01",
        Point2::new(-150.0, 0.0),
        Point2::new(150.0, 0.0),
        4.0,
        &["lane-1", "lane-2"],
    );
    session
        .commit(&transaction(
            "replace-alternative-road",
            1,
            "Replace Road R01",
            vec![Command::ReplaceRoad {
                scenario_id: scenario_id("scenario-alt-a"),
                road: replacement,
            }],
        ))
        .expect("ReplaceRoad");
    let alternative_network = &session
        .project()
        .scenario(&scenario_id("scenario-alt-a"))
        .expect("Alternative")
        .network();
    assert_eq!(
        alternative_network
            .junction(&junction_id("J01"))
            .expect("junction")
            .status(),
        JunctionStatus::Fresh
    );
    assert_eq!(
        alternative_network
            .junction(&junction_id("J01"))
            .expect("junction")
            .authored_lane_connections()[0]
            .id(),
        valid_manual_connection("manual-1").id()
    );
}

#[test]
fn replace_road_accepts_kernel_stale_regeneration_and_retains_authored_intent() {
    let mut session = representative_session();
    session
        .commit(&transaction(
            "replace-stale-road",
            0,
            "Move Road R01 away from junction",
            vec![Command::ReplaceRoad {
                scenario_id: scenario_id("scenario-alt-a"),
                road: line_road_with_lanes(
                    "R01",
                    Point2::new(-150.0, 400.0),
                    Point2::new(150.0, 400.0),
                    4.0,
                    &["lane-1", "lane-2"],
                ),
            }],
        ))
        .expect("stale regeneration is an accepted result");
    let junction = session
        .project()
        .scenario(&scenario_id("scenario-alt-a"))
        .expect("Alternative")
        .network()
        .junction(&junction_id("J01"))
        .expect("stale junction");
    assert_eq!(junction.status(), JunctionStatus::Stale);
    assert_eq!(junction.candidate().road_ids()[0].as_str(), "R01");
    session
        .project()
        .validate_default()
        .expect("stale project valid");
}

#[test]
fn create_junction_revalidates_candidate_and_rejects_stale_candidate_atomically() {
    let project = project_without_junction();
    let scenario = scenario_id("scenario-editable");
    let candidate = junction_candidate(project.scenario(&scenario).expect("scenario").network());
    let mut session = ProjectSession::new(project.clone()).expect("session");
    session
        .commit(&transaction(
            "create-junction",
            0,
            "Create Junction J01",
            vec![Command::CreateJunctionFromCandidate {
                scenario_id: scenario.clone(),
                candidate: candidate.clone(),
                junction_id: junction_id("J01"),
                options: JunctionOptions::new(),
            }],
        ))
        .expect("create junction");
    assert_eq!(
        session
            .project()
            .scenario(&scenario)
            .expect("scenario")
            .network()
            .junctions()
            .len(),
        1
    );

    let project = project_without_junction();
    let before = project.clone();
    let mut session = ProjectSession::new(project).expect("session");
    let error = session
        .commit(&transaction(
            "stale-candidate",
            0,
            "Move road then create stale junction",
            vec![
                Command::ReplaceRoad {
                    scenario_id: scenario.clone(),
                    road: line_road_with_lanes(
                        "R01",
                        Point2::new(-150.0, 400.0),
                        Point2::new(150.0, 400.0),
                        4.0,
                        &["lane-1", "lane-2"],
                    ),
                },
                Command::CreateJunctionFromCandidate {
                    scenario_id: scenario,
                    candidate,
                    junction_id: junction_id("J01"),
                    options: JunctionOptions::new(),
                },
            ],
        ))
        .expect_err("kernel must reject stale candidate");
    assert_eq!(error, SessionError::Kernel(KernelError::CandidateStale));
    assert_eq!(session.project(), &before);
    assert_eq!(session.revision(), 0);
    assert!(session.undo_history().is_empty());
}

#[test]
fn corner_and_manual_connectivity_commands_use_kernel_validation() {
    let mut session = representative_session();
    let target = scenario_id("scenario-alt-a");
    let corner = corner_id(session.project(), &target);
    session
        .commit(&transaction(
            "corner-command",
            0,
            "Set NE corner radius to 14 m",
            vec![Command::SetCornerRadius {
                scenario_id: target.clone(),
                junction_id: junction_id("J01"),
                corner_id: corner,
                radius_m: 14.0,
            }],
        ))
        .expect("corner command");
    let manual = valid_manual_connection("manual-2");
    session
        .commit(&transaction(
            "connectivity-command",
            1,
            "Replace manual lane connections",
            vec![Command::ReplaceLaneConnections {
                scenario_id: target.clone(),
                junction_id: junction_id("J01"),
                connections: vec![manual.clone()],
            }],
        ))
        .expect("connectivity command");
    let junction = session
        .project()
        .scenario(&target)
        .expect("Alternative")
        .network()
        .junction(&junction_id("J01"))
        .expect("junction");
    assert_eq!(
        junction.connectivity_mode(),
        street_concept_designer_kernel::LaneConnectivityMode::Manual
    );
    assert_eq!(junction.authored_lane_connections(), &[manual]);

    let mut session = representative_session();
    let before = session.project().clone();
    let invalid = LaneConnection::new(
        "invalid-manual",
        ApproachId::new("R01::start").expect("approach"),
        street_concept_designer_kernel::ComponentId::new("missing-lane").expect("lane"),
        ApproachId::new("R02::end").expect("approach"),
        street_concept_designer_kernel::ComponentId::new("lane-1").expect("lane"),
        Movement::Left,
    )
    .expect("typed but incompatible connection");
    assert_eq!(
        session.commit(&transaction(
            "invalid-manual",
            0,
            "Invalid manual connectivity",
            vec![Command::ReplaceLaneConnections {
                scenario_id: scenario_id("scenario-alt-a"),
                junction_id: junction_id("J01"),
                connections: vec![invalid],
            }],
        )),
        Err(SessionError::Kernel(KernelError::MissingLane))
    );
    assert_eq!(session.project(), &before);
}

#[test]
fn invalid_targets_and_payloads_leave_the_session_unchanged() {
    let cases = vec![
        Command::AddRoad {
            scenario_id: scenario_id("missing-scenario"),
            road: line_road("R03", Point2::new(300.0, 0.0), Point2::new(420.0, 0.0), 3.5),
        },
        Command::ReplaceRoad {
            scenario_id: scenario_id("scenario-alt-a"),
            road: line_road("R99", Point2::new(300.0, 0.0), Point2::new(420.0, 0.0), 3.5),
        },
        Command::AddRoad {
            scenario_id: scenario_id("scenario-alt-a"),
            road: line_road("R01", Point2::new(300.0, 0.0), Point2::new(420.0, 0.0), 3.5),
        },
        Command::SetCornerRadius {
            scenario_id: scenario_id("scenario-alt-a"),
            junction_id: junction_id("missing-junction"),
            corner_id: corner_id_value("missing-corner"),
            radius_m: 10.0,
        },
        Command::SetCornerRadius {
            scenario_id: scenario_id("scenario-alt-a"),
            junction_id: junction_id("J01"),
            corner_id: corner_id_value("missing-corner"),
            radius_m: 10.0,
        },
        Command::SetCornerRadius {
            scenario_id: scenario_id("scenario-alt-a"),
            junction_id: junction_id("J01"),
            corner_id: corner_id(&representative_project(), &scenario_id("scenario-alt-a")),
            radius_m: -1.0,
        },
        Command::DuplicateScenario {
            source_scenario_id: scenario_id("scenario-existing"),
            new_scenario_id: scenario_id("scenario-existing"),
            new_name: "Duplicate id".to_owned(),
            role: ScenarioRole::Alternative,
            locked: false,
        },
    ];

    for (index, command) in cases.into_iter().enumerate() {
        let mut session = representative_session();
        let before = session.project().clone();
        let undo = session.undo_history();
        let redo = session.redo_history();
        let error = session
            .commit(&transaction(
                &format!("invalid-{index}"),
                0,
                "Invalid operation",
                vec![command],
            ))
            .expect_err("invalid command");
        assert!(matches!(
            error,
            SessionError::Project(_)
                | SessionError::Kernel(KernelError::MissingRoad)
                | SessionError::Kernel(KernelError::DuplicateRoadId)
                | SessionError::Kernel(KernelError::MissingJunction)
                | SessionError::Kernel(KernelError::InvalidCornerId)
                | SessionError::Kernel(KernelError::InvalidParameter { .. })
        ));
        unchanged(&session, &before, 0, &undo, &redo);
    }
}

#[test]
fn no_op_transactions_are_rejected_without_revision_or_history() {
    let mut session = representative_session();
    let before = session.project().clone();
    let tx = transaction(
        "no-op-lock",
        0,
        "Set already-unlocked lock state",
        vec![Command::SetScenarioLock {
            scenario_id: scenario_id("scenario-alt-a"),
            locked: false,
        }],
    );
    assert!(matches!(
        session.preview(&tx),
        Err(SessionError::NoOpTransaction)
    ));
    assert!(matches!(
        session.commit(&tx),
        Err(SessionError::NoOpTransaction)
    ));
    assert_eq!(session.project(), &before);
    assert_eq!(session.revision(), 0);
    assert!(session.undo_history().is_empty());
}

#[test]
fn persistence_round_trip_and_clean_r1c_rebuild_work_for_commit_undo_and_redo() {
    let mut session = ProjectSession::new(single_existing_project()).expect("session");
    session
        .commit(&transaction(
            "duplicate-for-persistence",
            0,
            "Duplicate Existing as Alternative A",
            vec![Command::DuplicateScenario {
                source_scenario_id: scenario_id("scenario-existing"),
                new_scenario_id: scenario_id("scenario-alt-a"),
                new_name: "Alternative A".to_owned(),
                role: ScenarioRole::Alternative,
                locked: false,
            }],
        ))
        .expect("duplicate");
    let corner = corner_id(session.project(), &scenario_id("scenario-alt-a"));
    let preview_tx = transaction(
        "preview-before-edit",
        1,
        "Preview lock alternative",
        vec![Command::SetScenarioLock {
            scenario_id: scenario_id("scenario-alt-a"),
            locked: true,
        }],
    );
    let project_before_preview = session.project().clone();
    session.preview(&preview_tx).expect("preview");
    assert_eq!(session.project(), &project_before_preview);
    session
        .commit(&transaction(
            "corner-for-persistence",
            1,
            "Set corner radius to 14 m",
            vec![Command::SetCornerRadius {
                scenario_id: scenario_id("scenario-alt-a"),
                junction_id: junction_id("J01"),
                corner_id: corner,
                radius_m: 14.0,
            }],
        ))
        .expect("corner edit");
    let committed = session.project().clone();
    assert_persisted_state(&committed);

    session.undo(2).expect("undo");
    let undone = session.project().clone();
    assert_persisted_state(&undone);
    assert_eq!(session.revision(), 3);

    session.redo(3).expect("redo");
    let redone = session.project().clone();
    assert_persisted_state(&redone);
    assert_eq!(redone, committed);
    assert_eq!(session.revision(), 4);
}

#[test]
fn project_persistence_contains_no_session_revision_or_history_state() {
    let mut session = representative_session();
    session
        .commit(&transaction(
            "history-only-id",
            0,
            "History-only summary",
            vec![Command::RenameScenario {
                scenario_id: scenario_id("scenario-alt-a"),
                new_name: "Alternative session state".to_owned(),
            }],
        ))
        .expect("commit");
    let json = street_concept_designer_project_io::encode_project_to_json(session.project())
        .expect("encode project only");
    assert!(!json.contains("history-only-id"));
    assert!(!json.contains("History-only summary"));
    assert!(!json.contains("revision"));
    assert!(!json.contains("undo"));
    assert!(!json.contains("redo"));
    assert!(!json.contains("history"));
}

#[test]
fn revision_mismatch_rejects_preview_without_mutation() {
    let session = representative_session();
    let tx = transaction(
        "stale-preview",
        1,
        "Stale preview",
        vec![Command::SetScenarioLock {
            scenario_id: scenario_id("scenario-alt-a"),
            locked: true,
        }],
    );
    let before = session.project().clone();
    assert!(matches!(
        session.preview(&tx),
        Err(SessionError::StaleRevision {
            expected_revision: 1,
            current_revision: 0
        })
    ));
    assert_eq!(session.project(), &before);
}

#[test]
fn candidate_disposition_is_not_rewritten_by_command_layer() {
    let project = project_without_junction();
    let scenario = scenario_id("scenario-editable");
    let ignored = junction_candidate(project.scenario(&scenario).expect("scenario").network())
        .with_disposition(CandidateDisposition::Ignore);
    let mut session = ProjectSession::new(project.clone()).expect("session");
    let error = session
        .commit(&transaction(
            "ignored-candidate",
            0,
            "Create from ignored candidate",
            vec![Command::CreateJunctionFromCandidate {
                scenario_id: scenario,
                candidate: ignored,
                junction_id: junction_id("J01"),
                options: JunctionOptions::new(),
            }],
        ))
        .expect_err("ignored candidate");
    assert_eq!(error, SessionError::Kernel(KernelError::CandidateIgnored));
    assert_eq!(session.project(), &project);
}
