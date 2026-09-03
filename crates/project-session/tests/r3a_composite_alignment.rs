#[allow(dead_code)]
mod support;

use street_concept_designer_kernel::{
    Alignment, AlignmentSegment, ComponentKind, CrossSection, CrossSectionComponent,
    JunctionStatus, PiecewiseLinearWidthProfile, Point2, Road, TolerancePolicy,
};
use street_concept_designer_project_core::ScenarioId;
use street_concept_designer_project_session::{
    Command, ProjectSession, Transaction, TransactionId,
};

fn policy() -> TolerancePolicy {
    TolerancePolicy::default()
}

fn transaction(id: &str, revision: u64, summary: &str, command: Command) -> Transaction {
    Transaction::new(
        TransactionId::new(id).expect("transaction id"),
        revision,
        summary,
        vec![command],
    )
    .expect("transaction")
}

fn composite_alignment(offset_x: f64) -> Alignment {
    let policy = policy();
    Alignment::from_segments(
        vec![
            AlignmentSegment::line(
                "segment-a",
                Point2::new(offset_x, 0.0),
                Point2::new(offset_x + 60.0, 0.0),
                &policy,
            )
            .expect("segment-a"),
            AlignmentSegment::line(
                "segment-b",
                Point2::new(offset_x + 60.0, 0.0),
                Point2::new(offset_x + 120.0, 0.0),
                &policy,
            )
            .expect("segment-b"),
        ],
        &policy,
    )
    .expect("composite alignment")
}

fn composite_road(id: &str, offset_x: f64) -> Road {
    let policy = policy();
    let alignment = composite_alignment(offset_x);
    let range = alignment.station_range();
    let lane = CrossSectionComponent::traffic_lane(
        "lane-composite",
        PiecewiseLinearWidthProfile::constant(range, 3.5, &policy).expect("lane profile"),
    )
    .expect("lane");
    let cross_section = CrossSection::new(range, vec![lane], &policy).expect("cross-section");
    Road::new(id, alignment, cross_section, &policy).expect("road")
}

fn road_with_replaced_composite_alignment(old: &Road) -> Road {
    let policy = policy();
    let alignment = Alignment::from_segments(
        vec![
            AlignmentSegment::line(
                "segment-a",
                Point2::new(-150.0, 0.0),
                Point2::new(0.0, 0.0),
                &policy,
            )
            .expect("replacement segment-a"),
            AlignmentSegment::line(
                "segment-b",
                Point2::new(0.0, 0.0),
                Point2::new(150.0, 0.0),
                &policy,
            )
            .expect("replacement segment-b"),
        ],
        &policy,
    )
    .expect("replacement alignment");
    let directions = old
        .traffic_lane_ids()
        .into_iter()
        .map(|lane_id| {
            let direction = old.lane_direction(&lane_id).expect("lane direction");
            (lane_id, direction)
        })
        .collect();
    Road::with_lane_directions(
        old.id().as_str(),
        alignment,
        old.cross_section().clone(),
        directions,
        &policy,
    )
    .expect("replacement road")
}

#[test]
fn composite_add_preview_commit_undo_redo_is_snapshot_exact() {
    let mut session = ProjectSession::new(support::project_without_junction()).expect("session");
    let scenario_id = support::scenario_id("scenario-editable");
    let road = composite_road("composite-added", 300.0);
    let transaction = transaction(
        "add-composite-road",
        0,
        "Add composite road",
        Command::AddRoad {
            scenario_id: scenario_id.clone(),
            road,
        },
    );
    let before = session.project().clone();
    let preview = session.preview(&transaction).expect("preview");
    assert_eq!(session.project(), &before);
    let preview_project = preview.candidate_project().clone();
    let preview_road = preview_project
        .scenario(&scenario_id)
        .expect("scenario")
        .network()
        .road(&support::road_id("composite-added"))
        .expect("composite road");
    assert_eq!(preview_road.alignment().segment_count(), 2);
    assert_eq!(
        preview_road.alignment().segment_ids()[1].as_str(),
        "segment-b"
    );

    session.commit(&transaction).expect("commit");
    let after = session.project().clone();
    assert_eq!(after, preview_project);
    assert_eq!(session.revision(), 1);
    session.undo(1).expect("undo");
    assert_eq!(session.project(), &before);
    session.redo(2).expect("redo");
    assert_eq!(session.project(), &after);
}

#[test]
fn composite_replace_uses_existing_history_and_junction_regeneration_path() {
    let mut session = ProjectSession::new(support::representative_project()).expect("session");
    let scenario_id = ScenarioId::new("scenario-alt-a").expect("scenario id");
    let old = session
        .project()
        .scenario(&scenario_id)
        .expect("scenario")
        .network()
        .road(&support::road_id("R01"))
        .expect("source road")
        .clone();
    let replacement = road_with_replaced_composite_alignment(&old);
    let transaction = transaction(
        "replace-with-composite",
        0,
        "Replace road with composite alignment",
        Command::ReplaceRoad {
            scenario_id: scenario_id.clone(),
            road: replacement,
        },
    );

    let before = session.project().clone();
    let preview = session.preview(&transaction).expect("preview replacement");
    let preview_project = preview.candidate_project();
    let preview_scenario = preview_project.scenario(&scenario_id).expect("scenario");
    let preview_road = preview_scenario
        .network()
        .road(&support::road_id("R01"))
        .expect("replacement road");
    assert_eq!(preview_road.id().as_str(), "R01");
    assert_eq!(preview_road.alignment().segment_count(), 2);
    assert_eq!(
        preview_scenario.network().junctions()[0].status(),
        JunctionStatus::Fresh
    );
    assert_eq!(session.project(), &before);

    session.commit(&transaction).expect("commit replacement");
    let after = session.project().clone();
    assert_eq!(&after, preview_project);
    assert_eq!(
        after
            .scenario(&scenario_id)
            .expect("scenario")
            .network()
            .junctions()[0]
            .status(),
        JunctionStatus::Fresh
    );
    session.undo(1).expect("undo replacement");
    assert_eq!(session.project(), &before);
    session.redo(2).expect("redo replacement");
    assert_eq!(session.project(), &after);
}

#[test]
fn composite_transaction_persists_and_renders_after_history_operations() {
    let mut session = ProjectSession::new(support::project_without_junction()).expect("session");
    let scenario_id = support::scenario_id("scenario-editable");
    let transaction = transaction(
        "persist-composite",
        0,
        "Add persisted composite",
        Command::AddRoad {
            scenario_id: scenario_id.clone(),
            road: composite_road("composite-persisted", 300.0),
        },
    );
    session.commit(&transaction).expect("commit");
    let bytes = street_concept_designer_project_io::encode_project_to_bytes(session.project())
        .expect("encode");
    let loaded =
        street_concept_designer_project_io::decode_project_from_bytes(&bytes).expect("decode");
    assert_eq!(&loaded, session.project());
    let network = loaded.scenario(&scenario_id).expect("scenario").network();
    let road = network
        .road(&support::road_id("composite-persisted"))
        .expect("composite road");
    assert_eq!(
        road.cross_section().components()[0].kind(),
        ComponentKind::TrafficLane
    );
    assert_eq!(road.alignment().segment_count(), 2);
    let snapshot = street_concept_designer_kernel::DerivedEngineeringSnapshot::derive(
        network,
        Point2::new(0.0, 0.0),
        &policy(),
    )
    .expect("shared derivation");
    snapshot.validate(&policy()).expect("snapshot");
}
