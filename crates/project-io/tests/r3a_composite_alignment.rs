#[allow(dead_code)]
mod support;

use std::f64::consts::FRAC_PI_2;

use street_concept_designer_kernel::{
    Alignment, AlignmentSegment, CrossSection, CrossSectionComponent, JunctionOptions, KernelError,
    PiecewiseLinearWidthProfile, Point2, Road, RoadId, RoadNetwork, TolerancePolicy,
};
use street_concept_designer_project_core::{
    CoordinateContext, Project, ProjectId, Scenario, ScenarioId, ScenarioRole, TrafficSide,
};
use street_concept_designer_project_io::{
    decode_project_from_json, encode_project_to_bytes, encode_project_to_json, PersistenceError,
    CURRENT_SCHEMA_VERSION,
};

fn policy() -> TolerancePolicy {
    TolerancePolicy::default()
}

fn composite_alignment() -> Alignment {
    let policy = policy();
    Alignment::from_segments(
        vec![
            AlignmentSegment::line(
                "segment-line",
                Point2::new(-100.0, 0.0),
                Point2::new(0.0, 0.0),
                &policy,
            )
            .expect("line segment"),
            AlignmentSegment::circular_arc(
                "segment-arc",
                Point2::new(0.0, 50.0),
                50.0,
                -FRAC_PI_2,
                FRAC_PI_2,
                &policy,
            )
            .expect("arc segment"),
            AlignmentSegment::line(
                "segment-end",
                Point2::new(50.0, 50.0),
                Point2::new(50.0, 150.0),
                &policy,
            )
            .expect("end segment"),
        ],
        &policy,
    )
    .expect("composite alignment")
}

fn composite_road(id: &str) -> Road {
    let policy = policy();
    let alignment = composite_alignment();
    let range = alignment.station_range();
    let component = CrossSectionComponent::traffic_lane(
        "lane-composite",
        PiecewiseLinearWidthProfile::constant(range, 3.5, &policy).expect("lane profile"),
    )
    .expect("lane");
    let cross_section = CrossSection::new(range, vec![component], &policy).expect("cross-section");
    Road::new(id, alignment, cross_section, &policy).expect("road")
}

fn composite_project() -> Project {
    let mut network = RoadNetwork::new();
    network
        .add_road(composite_road("composite-road"))
        .expect("road");
    let scenario = Scenario::new(
        ScenarioId::new("scenario-composite").expect("scenario id"),
        "Composite",
        ScenarioRole::Existing,
        false,
        network,
    )
    .expect("scenario");
    Project::with_scenario(
        ProjectId::new("project-composite").expect("project id"),
        "Composite alignment project",
        TrafficSide::LeftHand,
        CoordinateContext::empty(),
        scenario,
    )
    .expect("project")
}

fn composite_junction_project() -> Project {
    let policy = policy();
    let main = {
        let alignment = Alignment::from_segments(
            vec![
                AlignmentSegment::line(
                    "main-a",
                    Point2::new(-100.0, 0.0),
                    Point2::new(0.0, 0.0),
                    &policy,
                )
                .expect("main-a"),
                AlignmentSegment::line(
                    "main-b",
                    Point2::new(0.0, 0.0),
                    Point2::new(100.0, 0.0),
                    &policy,
                )
                .expect("main-b"),
            ],
            &policy,
        )
        .expect("main alignment");
        road_with_one_lane("main", alignment)
    };
    let cross = road_with_one_lane(
        "cross",
        Alignment::line(Point2::new(0.0, -100.0), Point2::new(0.0, 100.0), &policy)
            .expect("cross alignment"),
    );
    let mut network = RoadNetwork::new();
    network.add_road(main).expect("main road");
    network.add_road(cross).expect("cross road");
    let main_id = RoadId::new("main").expect("main id");
    let cross_id = RoadId::new("cross").expect("cross id");
    let candidate = network
        .detect_candidate(
            &main_id,
            &cross_id,
            street_concept_designer_kernel::CrossingRelation::AtGrade,
            &policy,
        )
        .expect("candidate")
        .expect("candidate exists");
    network
        .create_junction(
            &candidate,
            "composite-junction",
            JunctionOptions::new(),
            &policy,
        )
        .expect("junction");
    let scenario = Scenario::new(
        ScenarioId::new("scenario-junction").expect("scenario id"),
        "Composite junction",
        ScenarioRole::Existing,
        false,
        network,
    )
    .expect("scenario");
    Project::with_scenario(
        ProjectId::new("project-junction").expect("project id"),
        "Composite junction project",
        TrafficSide::LeftHand,
        CoordinateContext::empty(),
        scenario,
    )
    .expect("project")
}

fn road_with_one_lane(id: &str, alignment: Alignment) -> Road {
    let policy = policy();
    let range = alignment.station_range();
    let lane = CrossSectionComponent::traffic_lane(
        "lane-1",
        PiecewiseLinearWidthProfile::constant(range, 3.5, &policy).expect("lane profile"),
    )
    .expect("lane");
    let cross_section = CrossSection::new(range, vec![lane], &policy).expect("cross-section");
    Road::new(id, alignment, cross_section, &policy).expect("road")
}

#[test]
fn current_writer_emits_v2_segments_and_composite_round_trip_is_bit_stable() {
    let project = composite_project();
    let json = encode_project_to_json(&project).expect("encode");
    let value: serde_json::Value = serde_json::from_str(&json).expect("JSON");
    assert_eq!(value["schemaVersion"], CURRENT_SCHEMA_VERSION);
    let segments = value["scenarios"][0]["network"]["roads"][0]["alignment"]["segments"]
        .as_array()
        .expect("segment array");
    assert_eq!(
        segments
            .iter()
            .map(|segment| segment["segmentId"].as_str().expect("segment id"))
            .collect::<Vec<_>>(),
        vec!["segment-line", "segment-arc", "segment-end"]
    );
    assert_eq!(segments[0]["primitive"]["kind"], "line");
    assert_eq!(segments[1]["primitive"]["kind"], "circularArc");
    assert_eq!(segments[2]["primitive"]["kind"], "line");
    assert!(!json.contains("rendererCache"));
    assert!(!json.contains("sessionRevision"));
    assert!(!json.contains("history"));

    let decoded = decode_project_from_json(&json).expect("decode");
    let decoded_again = decode_project_from_json(&json).expect("repeat decode");
    assert_eq!(decoded, project);
    assert_eq!(decoded, decoded_again);
    assert_eq!(
        encode_project_to_bytes(&decoded).expect("re-encode"),
        encode_project_to_bytes(&project).expect("encode bytes")
    );
    let alignment = decoded.scenarios()[0].network().roads()[0].alignment();
    assert_eq!(alignment.segment_ids()[0].as_str(), "segment-line");
    assert_eq!(alignment.segment_ids()[2].as_str(), "segment-end");
    assert_eq!(
        alignment.segment_ranges()[1].start_m,
        alignment.segment_ranges()[0].end_m
    );
}

#[test]
fn schema_v1_single_primitive_migrates_to_deterministic_segment_zero() {
    let source = support::representative_project();
    let current_json = encode_project_to_json(&source).expect("encode current");
    let mut v1: serde_json::Value = serde_json::from_str(&current_json).expect("JSON");
    v1["schemaVersion"] = serde_json::json!(1);
    for scenario in v1["scenarios"].as_array_mut().expect("scenarios") {
        for road in scenario["network"]["roads"].as_array_mut().expect("roads") {
            let primitive = road["alignment"]["segments"][0]["primitive"].clone();
            road["alignment"] = primitive;
        }
    }
    let v1_json = serde_json::to_string(&v1).expect("v1 fixture");

    let migrated = decode_project_from_json(&v1_json).expect("migrate v1");
    let migrated_again = decode_project_from_json(&v1_json).expect("repeat migration");
    assert_eq!(migrated, source);
    assert_eq!(migrated, migrated_again);
    for scenario in migrated.scenarios() {
        for road in scenario.network().roads() {
            assert_eq!(road.alignment().segment_count(), 1);
            assert_eq!(road.alignment().segment_ids()[0].as_str(), "segment-0");
        }
    }
    let migrated_json = encode_project_to_json(&migrated).expect("encode migrated");
    let migrated_value: serde_json::Value = serde_json::from_str(&migrated_json).expect("JSON");
    assert_eq!(migrated_value["schemaVersion"], CURRENT_SCHEMA_VERSION);
    assert!(
        migrated_value["scenarios"][0]["network"]["roads"][0]["alignment"]["segments"].is_array()
    );
    assert_eq!(migrated_json, current_json);
}

#[test]
fn composite_authored_junction_reconstructs_from_candidate_not_derived_state() {
    let project = composite_junction_project();
    let json = encode_project_to_json(&project).expect("encode junction project");
    assert!(!json.contains("surfaceVertices"));
    assert!(!json.contains("approaches"));
    let decoded = decode_project_from_json(&json).expect("decode junction project");
    let source_network = project.scenarios()[0].network();
    let decoded_network = decoded.scenarios()[0].network();
    assert_eq!(source_network.junctions().len(), 1);
    assert_eq!(decoded_network.junctions().len(), 1);
    assert_eq!(source_network.junctions(), decoded_network.junctions());
    assert_eq!(
        decoded_network.junctions()[0].status(),
        street_concept_designer_kernel::JunctionStatus::Fresh
    );
    assert_eq!(
        decoded_network
            .road(&RoadId::new("main").expect("main id"))
            .expect("main road")
            .alignment()
            .segment_count(),
        2
    );
}

#[test]
fn composite_schema_rejects_future_unknown_empty_duplicate_gap_and_nonfinite_inputs() {
    let base = {
        let project = composite_project();
        serde_json::from_str::<serde_json::Value>(
            &encode_project_to_json(&project).expect("encode"),
        )
        .expect("JSON")
    };

    let mut future = base.clone();
    future["schemaVersion"] = serde_json::json!(CURRENT_SCHEMA_VERSION + 1);
    assert!(matches!(
        decode_project_from_json(&serde_json::to_string(&future).expect("future JSON")),
        Err(PersistenceError::UnsupportedSchemaVersion { version: 3 })
    ));

    let mut unknown = base.clone();
    unknown["scenarios"][0]["network"]["roads"][0]["alignment"]["unexpected"] =
        serde_json::json!(true);
    assert!(matches!(
        decode_project_from_json(&serde_json::to_string(&unknown).expect("unknown JSON")),
        Err(PersistenceError::JsonDecode { .. })
    ));

    let mut empty = base.clone();
    empty["scenarios"][0]["network"]["roads"][0]["alignment"]["segments"] = serde_json::json!([]);
    assert!(decode_project_from_json(&serde_json::to_string(&empty).expect("empty JSON")).is_err());

    let mut duplicate = base.clone();
    let first_segment =
        duplicate["scenarios"][0]["network"]["roads"][0]["alignment"]["segments"][0].clone();
    duplicate["scenarios"][0]["network"]["roads"][0]["alignment"]["segments"]
        .as_array_mut()
        .expect("segments")
        .push(first_segment);
    assert!(matches!(
        decode_project_from_json(&serde_json::to_string(&duplicate).expect("duplicate JSON")),
        Err(PersistenceError::Kernel(
            KernelError::DuplicateAlignmentSegmentId
        ))
    ));

    let mut gap = base.clone();
    gap["scenarios"][0]["network"]["roads"][0]["alignment"]["segments"][1]["primitive"]["center"]
        ["x"] = serde_json::json!(0.001);
    assert!(matches!(
        decode_project_from_json(&serde_json::to_string(&gap).expect("gap JSON")),
        Err(PersistenceError::Kernel(
            KernelError::AlignmentEndpointGap { .. }
        ))
    ));

    let mut nonfinite = base;
    nonfinite["scenarios"][0]["network"]["roads"][0]["alignment"]["segments"][0]["primitive"]
        ["start"]["x"] = serde_json::json!("NaN");
    assert!(
        decode_project_from_json(&serde_json::to_string(&nonfinite).expect("nonfinite JSON"))
            .is_err()
    );
}
