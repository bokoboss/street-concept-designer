mod support;

use street_concept_designer_kernel::{
    derive_diagnostic_2d, derive_diagnostic_3d, Alignment, AlignmentPrimitive,
    DerivedEngineeringSnapshot, JunctionStatus, Point2,
};
use street_concept_designer_project_core::{Project, Scenario};
use street_concept_designer_project_io::{
    decode_project_from_bytes, decode_project_from_json, encode_project_to_bytes,
    encode_project_to_json, PersistenceError,
};

use support::{
    all_alignment_project, automatic_project, finite_corpus_project, policy, representative_project,
};

fn representative_json_value() -> serde_json::Value {
    serde_json::from_str(&encode_project_to_json(&representative_project()).expect("encode"))
        .expect("valid JSON")
}

fn representative_json() -> String {
    encode_project_to_json(&representative_project()).expect("encode")
}

fn duplicate_raw_scalar_field(json: &str, field: &str, first: &str, second: &str) -> String {
    let key = format!("\"{field}\":");
    let key_start = json
        .find(&key)
        .unwrap_or_else(|| panic!("field {field} not found in representative JSON"));
    let value_start = key_start + key.len();
    let value_end = scalar_value_end(json, value_start);
    format!(
        "{}{}{}{}{}{}{}",
        &json[..key_start],
        key,
        first,
        ',',
        key,
        second,
        &json[value_end..]
    )
}

fn scalar_value_end(json: &str, value_start: usize) -> usize {
    let bytes = json.as_bytes();
    if bytes[value_start] == b'"' {
        let mut escaped = false;
        for (offset, byte) in bytes.iter().enumerate().skip(value_start + 1) {
            if *byte == b'"' && !escaped {
                return offset + 1;
            }
            escaped = *byte == b'\\' && !escaped;
            if *byte != b'\\' {
                escaped = false;
            }
        }
        panic!("unterminated JSON string")
    }

    json[value_start..]
        .char_indices()
        .find_map(|(offset, character)| {
            matches!(character, ',' | '}' | ']').then_some(value_start + offset)
        })
        .expect("scalar JSON value terminator")
}

fn assert_raw_duplicate_rejected(json: &str, field: &str) {
    assert!(
        json.matches(&format!("\"{field}\":")).count() >= 2,
        "test fixture must contain at least two raw {field} keys"
    );
    assert!(matches!(
        decode_project_from_bytes(json.as_bytes()),
        Err(PersistenceError::JsonDecode { .. })
    ));
}

fn decode_value(value: serde_json::Value) -> Result<Project, PersistenceError> {
    decode_project_from_json(&serde_json::to_string(&value).expect("serialize test value"))
}

fn scenario_ids(project: &Project) -> Vec<&str> {
    project
        .scenarios()
        .iter()
        .map(|scenario| scenario.id().as_str())
        .collect()
}

fn r1c_outputs(
    project: &Project,
    render_origin: Point2,
) -> Vec<(
    DerivedEngineeringSnapshot,
    street_concept_designer_kernel::Diagnostic2D,
    street_concept_designer_kernel::Diagnostic3D,
)> {
    project
        .scenarios()
        .iter()
        .map(|scenario| {
            let snapshot =
                DerivedEngineeringSnapshot::derive(scenario.network(), render_origin, &policy())
                    .expect("R1C snapshot");
            let diagnostic_2d = derive_diagnostic_2d(&snapshot).expect("R1C 2D diagnostic");
            let diagnostic_3d = derive_diagnostic_3d(&snapshot).expect("R1C 3D diagnostic");
            (snapshot, diagnostic_2d, diagnostic_3d)
        })
        .collect()
}

fn assert_f64_bits_equal(before: f64, after: f64, field: &str) {
    assert_eq!(
        before.to_bits(),
        after.to_bits(),
        "f64 bits differ for {field}: {before:?} vs {after:?}"
    );
}

fn assert_authored_f64_bits_equal(before: &Project, after: &Project) {
    for (scenario_before, scenario_after) in before.scenarios().iter().zip(after.scenarios()) {
        for (road_before, road_after) in scenario_before
            .network()
            .roads()
            .iter()
            .zip(scenario_after.network().roads())
        {
            let before_segments = road_before.alignment().segments();
            let after_segments = road_after.alignment().segments();
            assert_eq!(before_segments.len(), after_segments.len());
            for (before, after) in before_segments.iter().zip(&after_segments) {
                assert_eq!(before.id(), after.id());
                assert_primitive_f64_bits_equal(before.primitive(), after.primitive());
            }
            assert_f64_bits_equal(
                road_before.station_range().start_m,
                road_after.station_range().start_m,
                "road station start",
            );
            assert_f64_bits_equal(
                road_before.station_range().end_m,
                road_after.station_range().end_m,
                "road station end",
            );
            for (component_before, component_after) in road_before
                .cross_section()
                .components()
                .iter()
                .zip(road_after.cross_section().components())
            {
                assert_eq!(component_before.id(), component_after.id());
                assert_eq!(component_before.kind(), component_after.kind());
                assert_f64_bits_equal(
                    component_before.width_profile().station_range().start_m,
                    component_after.width_profile().station_range().start_m,
                    "profile station start",
                );
                assert_f64_bits_equal(
                    component_before.width_profile().station_range().end_m,
                    component_after.width_profile().station_range().end_m,
                    "profile station end",
                );
                for (index, (before, after)) in component_before
                    .width_profile()
                    .knots()
                    .iter()
                    .zip(component_after.width_profile().knots())
                    .enumerate()
                {
                    assert_f64_bits_equal(
                        before.station_m,
                        after.station_m,
                        &format!("width knot {index} station"),
                    );
                    assert_f64_bits_equal(
                        before.width_m,
                        after.width_m,
                        &format!("width knot {index} width"),
                    );
                }
            }
        }
        for (junction_before, junction_after) in scenario_before
            .network()
            .junctions()
            .iter()
            .zip(scenario_after.network().junctions())
        {
            let before = junction_before.authored_snapshot();
            let after = junction_after.authored_snapshot();
            assert_f64_bits_equal(
                before.candidate().point().x,
                after.candidate().point().x,
                "candidate x",
            );
            assert_f64_bits_equal(
                before.candidate().point().y,
                after.candidate().point().y,
                "candidate y",
            );
            assert_f64_bits_equal(
                before.candidate().station_a_m(),
                after.candidate().station_a_m(),
                "candidate station a",
            );
            assert_f64_bits_equal(
                before.candidate().station_b_m(),
                after.candidate().station_b_m(),
                "candidate station b",
            );
            assert_f64_bits_equal(
                before.options().default_corner_radius_m(),
                after.options().default_corner_radius_m(),
                "default corner radius",
            );
            for (before, after) in before
                .options()
                .corner_radii_m()
                .iter()
                .zip(after.options().corner_radii_m())
            {
                assert_f64_bits_equal(*before, *after, "option corner radius");
            }
            for ((before_id, before), (after_id, after)) in
                before.corner_radii_m().iter().zip(after.corner_radii_m())
            {
                assert_eq!(before_id, after_id);
                assert_f64_bits_equal(*before, *after, "authored corner radius");
            }
        }
    }
}

fn assert_primitive_f64_bits_equal(before: &AlignmentPrimitive, after: &AlignmentPrimitive) {
    match (before, after) {
        (AlignmentPrimitive::Line(before), AlignmentPrimitive::Line(after)) => {
            assert_f64_bits_equal(before.start().x, after.start().x, "line start x");
            assert_f64_bits_equal(before.start().y, after.start().y, "line start y");
            assert_f64_bits_equal(before.end().x, after.end().x, "line end x");
            assert_f64_bits_equal(before.end().y, after.end().y, "line end y");
        }
        (AlignmentPrimitive::CircularArc(before), AlignmentPrimitive::CircularArc(after)) => {
            assert_f64_bits_equal(before.center().x, after.center().x, "arc center x");
            assert_f64_bits_equal(before.center().y, after.center().y, "arc center y");
            assert_f64_bits_equal(before.radius(), after.radius(), "arc radius");
            assert_f64_bits_equal(before.start_angle(), after.start_angle(), "arc start angle");
            assert_f64_bits_equal(before.sweep_angle(), after.sweep_angle(), "arc sweep angle");
        }
        (
            AlignmentPrimitive::SmoothConceptualCurve(before),
            AlignmentPrimitive::SmoothConceptualCurve(after),
        ) => {
            for (index, (before, after)) in before
                .control_points()
                .into_iter()
                .zip(after.control_points())
                .enumerate()
            {
                assert_f64_bits_equal(before.x, after.x, &format!("smooth p{index} x"));
                assert_f64_bits_equal(before.y, after.y, &format!("smooth p{index} y"));
            }
        }
        (before, after) => panic!(
            "alignment primitive kind changed: {:?} -> {:?}",
            before.kind(),
            after.kind()
        ),
    }
}

fn assert_no_derived_schema_keys(value: &serde_json::Value) {
    const FORBIDDEN: &[&str] = &[
        "rendererCache",
        "pavementSurface",
        "surfaceVertices",
        "approaches",
        "approachFrames",
        "cornerArcPoints",
        "alignmentSamples",
        "derivedEngineeringSnapshot",
        "renderOrigin",
        "gpuBuffers",
        "meshBuffers",
        "localCoordinates",
    ];
    match value {
        serde_json::Value::Object(object) => {
            for (key, child) in object {
                assert!(
                    !FORBIDDEN.contains(&key.as_str()),
                    "derived key persisted: {key}"
                );
                assert_no_derived_schema_keys(child);
            }
        }
        serde_json::Value::Array(array) => {
            for child in array {
                assert_no_derived_schema_keys(child);
            }
        }
        _ => {}
    }
}

#[test]
fn representative_project_round_trips_with_authored_order_and_clean_r1c_rebuild() {
    let project = representative_project();
    let json = encode_project_to_json(&project).expect("encode representative project");
    let value: serde_json::Value = serde_json::from_str(&json).expect("JSON");
    assert_eq!(value["schemaVersion"], 2);
    assert_eq!(value["canonicalUnits"], "m");
    assert_eq!(value["trafficSide"], "LHT");
    assert_eq!(
        value["scenarios"]
            .as_array()
            .expect("scenario array")
            .iter()
            .map(|scenario| scenario["scenarioId"].as_str().expect("scenario id"))
            .collect::<Vec<_>>(),
        vec![
            "scenario-z-existing",
            "scenario-m-alt-b",
            "scenario-a-alt-a"
        ]
    );
    assert!(!json.contains("\n"));
    assert!(!json.contains(": "));
    assert_no_derived_schema_keys(&value);

    let render_origin = Point2::new(1_000_000_000.0, -2_000_000_000.0);
    let before_outputs = r1c_outputs(&project, render_origin);
    let decoded = decode_project_from_json(&json).expect("decode representative project");
    let decoded_from_bytes = decode_project_from_bytes(
        &encode_project_to_bytes(&project).expect("encode representative bytes"),
    )
    .expect("decode representative bytes");
    assert_eq!(
        project, decoded,
        "semantic Project equality must round-trip"
    );
    assert_eq!(decoded, decoded_from_bytes);
    assert_authored_f64_bits_equal(&project, &decoded);
    assert_eq!(
        encode_project_to_bytes(&project).expect("encode bytes"),
        encode_project_to_bytes(&decoded).expect("re-encode bytes"),
        "equal canonical Projects emit equal JSON bytes"
    );
    assert_eq!(
        scenario_ids(&project),
        vec![
            "scenario-z-existing",
            "scenario-m-alt-b",
            "scenario-a-alt-a"
        ]
    );
    assert_eq!(scenario_ids(&project), scenario_ids(&decoded));
    let after_outputs = r1c_outputs(&decoded, render_origin);
    assert_eq!(
        before_outputs, after_outputs,
        "R1C rebuild changed after load"
    );

    let existing_junction = &project.scenarios()[0].network().junctions()[0];
    let loaded_junction = &decoded.scenarios()[0].network().junctions()[0];
    assert_eq!(existing_junction.id(), loaded_junction.id());
    assert_eq!(existing_junction.road_ids(), loaded_junction.road_ids());
    assert_eq!(
        existing_junction.authored_corner_radii_m(),
        loaded_junction.authored_corner_radii_m()
    );
    assert_eq!(
        existing_junction.authored_lane_connections(),
        loaded_junction.authored_lane_connections()
    );
    assert_eq!(
        existing_junction.lane_connections(),
        loaded_junction.lane_connections()
    );
    assert_eq!(existing_junction.status(), JunctionStatus::Fresh);
    assert_eq!(loaded_junction.status(), JunctionStatus::Fresh);
}

#[test]
fn automatic_connectivity_is_regenerated_and_not_persisted_as_authored_state() {
    let project = automatic_project();
    let before = &project.scenarios()[0].network().junctions()[0];
    assert_eq!(
        before.connectivity_mode(),
        street_concept_designer_kernel::LaneConnectivityMode::Automatic
    );
    assert!(!before.lane_connections().is_empty());
    assert!(before.authored_lane_connections().is_empty());
    let json = encode_project_to_json(&project).expect("encode automatic project");
    assert!(json.contains("\"manualLaneConnections\":[]"));
    assert!(!json.contains("\"laneConnections\""));
    let decoded = decode_project_from_json(&json).expect("decode automatic project");
    let after = &decoded.scenarios()[0].network().junctions()[0];
    assert_eq!(after.connectivity_mode(), before.connectivity_mode());
    assert_eq!(after.lane_connections(), before.lane_connections());
    assert!(after.authored_lane_connections().is_empty());
}

#[test]
fn every_accepted_alignment_primitive_round_trips_authored_parameters() {
    let project = all_alignment_project();
    let json = encode_project_to_json(&project).expect("encode alignment project");
    let decoded = decode_project_from_json(&json).expect("decode alignment project");
    assert_eq!(project, decoded);
    assert_authored_f64_bits_equal(&project, &decoded);
    assert!(json.contains("\"kind\":\"line\""));
    assert!(json.contains("\"kind\":\"circularArc\""));
    assert!(json.contains("\"kind\":\"smoothConceptualCurve\""));
    assert!(!json.contains("lookup"));
    assert!(!json.contains("sample"));
}

#[test]
fn finite_engineering_f64_corpus_round_trips_bit_exactly() {
    let project = finite_corpus_project();
    let decoded = decode_project_from_json(&encode_project_to_json(&project).expect("encode"))
        .expect("decode");
    assert_eq!(project, decoded);
    assert_authored_f64_bits_equal(&project, &decoded);
}

#[test]
fn synthetic_pre_release_v0_migration_is_explicit_deterministic_and_lossless() {
    let project = representative_project();
    let current_json = encode_project_to_json(&project).expect("encode current");
    let mut historical: serde_json::Value = serde_json::from_str(&current_json).expect("JSON");
    historical["schemaVersion"] = serde_json::json!(0);
    historical["canonicalUnits"] = serde_json::json!("metres");
    let historical_json = serde_json::to_string(&historical).expect("synthetic v0 fixture");

    let migrated = decode_project_from_json(&historical_json).expect("migrate v0");
    let migrated_again = decode_project_from_json(&historical_json).expect("repeat v0 migration");
    assert_eq!(migrated, project);
    assert_eq!(migrated, migrated_again);
    assert_eq!(
        encode_project_to_json(&migrated).expect("encode migrated project"),
        current_json,
        "migration emits current schema v2 only"
    );
}

#[test]
fn raw_duplicate_schema_version_is_rejected_in_both_orders() {
    let json = representative_json();
    let first_then_second = duplicate_raw_scalar_field(&json, "schemaVersion", "2", "3");
    assert_raw_duplicate_rejected(&first_then_second, "schemaVersion");

    let second_then_first = duplicate_raw_scalar_field(&json, "schemaVersion", "3", "2");
    assert_raw_duplicate_rejected(&second_then_first, "schemaVersion");
}

#[test]
fn raw_duplicate_project_id_is_rejected() {
    let json = representative_json();
    let duplicate =
        duplicate_raw_scalar_field(&json, "projectId", "\"project-a\"", "\"project-b\"");
    assert_raw_duplicate_rejected(&duplicate, "projectId");
}

#[test]
fn raw_duplicate_nested_identity_is_rejected() {
    let json = representative_json();
    let duplicate =
        duplicate_raw_scalar_field(&json, "scenarioId", "\"scenario-a\"", "\"scenario-b\"");
    assert_raw_duplicate_rejected(&duplicate, "scenarioId");
}

#[test]
fn raw_duplicate_engineering_number_is_rejected() {
    let json = representative_json();
    let duplicate = duplicate_raw_scalar_field(&json, "widthM", "3.25", "4.25");
    assert_raw_duplicate_rejected(&duplicate, "widthM");
}

#[test]
fn raw_duplicate_junction_field_is_rejected() {
    let json = representative_json();
    let duplicate = duplicate_raw_scalar_field(&json, "radiusM", "17.25", "9.75");
    assert_raw_duplicate_rejected(&duplicate, "radiusM");
}

#[test]
fn raw_duplicate_same_value_field_is_rejected() {
    let json = representative_json();
    let duplicate =
        duplicate_raw_scalar_field(&json, "projectId", "\"project-a\"", "\"project-a\"");
    assert_raw_duplicate_rejected(&duplicate, "projectId");
}

#[test]
fn schema_version_detection_rejects_missing_invalid_and_future_versions() {
    let base = representative_json_value();

    let mut missing = base.clone();
    missing
        .as_object_mut()
        .expect("object")
        .remove("schemaVersion");
    assert!(matches!(
        decode_value(missing),
        Err(PersistenceError::MissingSchemaVersion)
    ));

    for version in [serde_json::json!(-1), serde_json::json!(1.5)] {
        let mut invalid = base.clone();
        invalid["schemaVersion"] = version;
        assert!(matches!(
            decode_value(invalid),
            Err(PersistenceError::InvalidSchemaVersion { .. })
        ));
    }

    let mut future = base;
    future["schemaVersion"] = serde_json::json!(3);
    assert!(matches!(
        decode_value(future),
        Err(PersistenceError::UnsupportedSchemaVersion { version: 3 })
    ));
}

#[test]
fn strict_schema_and_corruption_inputs_fail_without_partial_project() {
    let base = representative_json_value();

    assert!(matches!(
        Alignment::line(
            Point2::new(f64::NAN, 0.0),
            Point2::new(10.0, 0.0),
            &policy(),
        ),
        Err(street_concept_designer_kernel::KernelError::NonFiniteInput { .. })
    ));

    let malformed = decode_project_from_json("{\"schemaVersion\":1");
    assert!(matches!(
        malformed,
        Err(PersistenceError::JsonSyntax { .. })
    ));
    let truncated = encode_project_to_json(&representative_project()).expect("encode");
    let truncated = &truncated[..truncated.len() - 1];
    assert!(matches!(
        decode_project_from_json(truncated),
        Err(PersistenceError::JsonSyntax { .. })
    ));

    let mut unknown = base.clone();
    unknown["rendererCache"] = serde_json::json!({});
    assert!(matches!(
        decode_value(unknown),
        Err(PersistenceError::JsonDecode { .. })
    ));

    let mut nested_unknown = base.clone();
    nested_unknown["scenarios"][0]["network"]["junctionDefinitions"][0]["rendererCache"] =
        serde_json::json!({});
    assert!(decode_value(nested_unknown).is_err());

    let mut missing_required = base.clone();
    missing_required["scenarios"][0]
        .as_object_mut()
        .expect("scenario object")
        .remove("network");
    assert!(decode_value(missing_required).is_err());

    let mut bad_units = base.clone();
    bad_units["canonicalUnits"] = serde_json::json!("ft");
    assert!(decode_value(bad_units).is_err());

    let mut bad_traffic_side = base.clone();
    bad_traffic_side["trafficSide"] = serde_json::json!("SIDEWAYS");
    assert!(decode_value(bad_traffic_side).is_err());

    let mut duplicate_scenario = base.clone();
    let first = duplicate_scenario["scenarios"][0].clone();
    duplicate_scenario["scenarios"]
        .as_array_mut()
        .expect("scenarios")
        .push(first);
    assert!(decode_value(duplicate_scenario).is_err());

    let mut missing_road = base.clone();
    missing_road["scenarios"][0]["network"]["junctionDefinitions"][0]["roadIds"][0] =
        serde_json::json!("R00");
    assert!(matches!(
        decode_value(missing_road),
        Err(PersistenceError::Kernel(
            street_concept_designer_kernel::KernelError::MissingRoad
        ))
    ));

    let mut invalid_alignment = base.clone();
    invalid_alignment["scenarios"][0]["network"]["roads"][0]["alignment"]["segments"][0]
        ["primitive"]["start"]["x"] = serde_json::json!(null);
    assert!(decode_value(invalid_alignment).is_err());

    let mut negative_width = base.clone();
    negative_width["scenarios"][0]["network"]["roads"][0]["crossSection"]["components"][0]
        ["widthProfile"]["knots"][0]["widthM"] = serde_json::json!(-1.0);
    assert!(decode_value(negative_width).is_err());

    let mut non_increasing = base.clone();
    let first_station = non_increasing["scenarios"][0]["network"]["roads"][0]["crossSection"]
        ["components"][0]["widthProfile"]["knots"][0]["stationM"]
        .clone();
    non_increasing["scenarios"][0]["network"]["roads"][0]["crossSection"]["components"][0]
        ["widthProfile"]["knots"][1]["stationM"] = first_station;
    assert!(decode_value(non_increasing).is_err());

    let mut duplicate_component = base.clone();
    let first_component = duplicate_component["scenarios"][0]["network"]["roads"][0]
        ["crossSection"]["components"][0]
        .clone();
    duplicate_component["scenarios"][0]["network"]["roads"][0]["crossSection"]["components"]
        .as_array_mut()
        .expect("components")
        .push(first_component);
    assert!(decode_value(duplicate_component).is_err());

    let mut invalid_corner_radius = base.clone();
    invalid_corner_radius["scenarios"][0]["network"]["junctionDefinitions"][0]
        ["authoredCornerRadii"][0]["radiusM"] = serde_json::json!(-1.0);
    assert!(decode_value(invalid_corner_radius).is_err());

    let mut invalid_manual_id = base.clone();
    invalid_manual_id["scenarios"][0]["network"]["junctionDefinitions"][0]
        ["manualLaneConnections"][0]["laneConnectionId"] = serde_json::json!("");
    assert!(decode_value(invalid_manual_id).is_err());

    let mut candidate_mismatch = base.clone();
    let x = candidate_mismatch["scenarios"][0]["network"]["junctionDefinitions"][0]["candidate"]
        ["point"]["x"]
        .as_f64()
        .expect("candidate x");
    candidate_mismatch["scenarios"][0]["network"]["junctionDefinitions"][0]["candidate"]["point"]
        ["x"] = serde_json::json!(x + 1.0);
    assert!(matches!(
        decode_value(candidate_mismatch),
        Err(PersistenceError::Kernel(
            street_concept_designer_kernel::KernelError::CandidateStale
        ))
    ));

    let mut topology_order = base.clone();
    let first_road =
        topology_order["scenarios"][0]["network"]["junctionDefinitions"][0]["roadIds"][0].clone();
    let second_road =
        topology_order["scenarios"][0]["network"]["junctionDefinitions"][0]["roadIds"][1].clone();
    topology_order["scenarios"][0]["network"]["junctionDefinitions"][0]["roadIds"][0] = second_road;
    topology_order["scenarios"][0]["network"]["junctionDefinitions"][0]["roadIds"][1] = first_road;
    assert!(decode_value(topology_order).is_err());

    let huge_exponent = encode_project_to_json(&representative_project())
        .expect("encode")
        .replacen("1000000000.125", "1e999", 1);
    assert!(decode_project_from_json(&huge_exponent).is_err());

    let mut null_number = base;
    null_number["scenarios"][0]["network"]["roads"][0]["alignment"]["segments"][0]["primitive"]
        ["start"]["x"] = serde_json::Value::Null;
    assert!(decode_value(null_number).is_err());
}

#[test]
fn incompatible_manual_state_is_retained_without_automatic_fallback() {
    let mut value = representative_json_value();
    value["scenarios"][0]["network"]["junctionDefinitions"][0]["manualLaneConnections"][0]
        ["fromComponentId"] = serde_json::json!("missing-lane");
    let decoded = decode_value(value).expect("incompatible manual intent remains persistable");
    let junction = &decoded.scenarios()[0].network().junctions()[0];
    assert_eq!(
        junction.status(),
        JunctionStatus::ManualConnectivityIncompatible
    );
    assert_eq!(
        junction.connectivity_mode(),
        street_concept_designer_kernel::LaneConnectivityMode::Manual
    );
    assert!(junction.lane_connections().is_empty());
    assert_eq!(junction.authored_lane_connections().len(), 1);
    assert_eq!(
        junction.authored_lane_connections()[0]
            .from_lane_id()
            .as_str(),
        "missing-lane"
    );
}

#[test]
fn stale_junction_intent_is_saved_but_load_fails_without_inventing_topology() {
    let project = representative_project();
    let source_scenario = &project.scenarios()[0];
    let mut stale_network = source_scenario.network().clone();
    let stale_road = stale_network
        .road(&street_concept_designer_kernel::RoadId::new("R02").expect("road id"))
        .expect("stale source road")
        .clone();
    let stale_alignment = Alignment::line(
        Point2::new(1_001_000_150.125, -2_000_000_150.75),
        Point2::new(1_001_000_150.125, -1_999_999_850.75),
        &policy(),
    )
    .expect("stale alignment");
    let directions = stale_road
        .traffic_lane_ids()
        .into_iter()
        .map(|lane_id| {
            let direction = stale_road
                .lane_direction(&lane_id)
                .expect("direction for traffic lane");
            (lane_id, direction)
        })
        .collect();
    let replacement = street_concept_designer_kernel::Road::with_lane_directions(
        stale_road.id().as_str(),
        stale_alignment,
        stale_road.cross_section().clone(),
        directions,
        &policy(),
    )
    .expect("replacement road");
    stale_network
        .replace_road(replacement)
        .expect("replace source road");
    assert_eq!(stale_network.junctions()[0].status(), JunctionStatus::Stale);

    let stale_scenario = Scenario::new(
        source_scenario.id().clone(),
        source_scenario.name(),
        source_scenario.role(),
        source_scenario.is_locked(),
        stale_network,
    )
    .expect("stale scenario remains valid authored state");
    let stale_project = Project::with_scenario(
        project.id().clone(),
        project.name(),
        project.traffic_side(),
        project.coordinate_context().clone(),
        stale_scenario,
    )
    .expect("stale project remains valid authored state");
    let json = encode_project_to_json(&stale_project).expect("stale intent is saveable");
    assert!(json.contains("\"junctionDefinitions\""));
    assert_no_derived_schema_keys(&serde_json::from_str(&json).expect("JSON"));
    assert!(matches!(
        decode_project_from_json(&json),
        Err(PersistenceError::Kernel(
            street_concept_designer_kernel::KernelError::CandidateStale
        ))
    ));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn bounded_path_save_and_load_uses_ordinary_io() {
    let project = representative_project();
    let path = std::env::temp_dir().join(format!(
        "street-concept-designer-r2b-{}.json",
        std::process::id()
    ));
    street_concept_designer_project_io::save_project_to_path(&path, &project).expect("save");
    let loaded = street_concept_designer_project_io::load_project_from_path(&path).expect("load");
    std::fs::remove_file(&path).expect("remove test file");
    assert_eq!(project, loaded);
}
