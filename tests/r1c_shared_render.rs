mod support;

use std::collections::HashSet;

use street_concept_designer_kernel::{
    derive_diagnostic_2d, derive_diagnostic_3d, Alignment, ComponentId, ComponentKind,
    CrossSection, CrossSectionComponent, CrossingRelation, DerivedComponent,
    DerivedEngineeringSnapshot, DiagnosticPrimitive2D, JunctionId, JunctionOptions, LaneConnection,
    LaneConnectionId, MeshTopology, PiecewiseLinearWidthProfile, Point2, PrimitiveRole, Road,
    RoadId, RoadNetwork, SamplingOptions, SemanticRef, TolerancePolicy,
    FLOAT32_LOCAL_COORDINATE_TOLERANCE_M,
};

use support::{
    circular_90deg, range, right_turn_pocket_cross_section, straight_500m,
    straight_two_lane_cross_section, width_profile,
};

fn policy() -> TolerancePolicy {
    TolerancePolicy::default()
}

fn constant_cross_section(
    station_range: street_concept_designer_kernel::StationRange,
    components: &[(&str, ComponentKind, f64)],
) -> CrossSection {
    let policy = policy();
    let components = components
        .iter()
        .map(|(id, kind, width)| {
            let profile = PiecewiseLinearWidthProfile::constant(station_range, *width, &policy)
                .expect("constant width profile");
            CrossSectionComponent::new(*id, *kind, profile).expect("component")
        })
        .collect();
    CrossSection::new(station_range, components, &policy).expect("cross-section")
}

fn road(id: &str, alignment: Alignment, cross_section: CrossSection) -> Road {
    Road::new(id, alignment, cross_section, &policy()).expect("road")
}

fn line_road(id: &str, start: Point2, end: Point2, widths: &[f64]) -> Road {
    let alignment = Alignment::line(start, end, &policy()).expect("line alignment");
    let range = alignment.station_range();
    let components = widths
        .iter()
        .enumerate()
        .map(|(index, width)| {
            let profile = PiecewiseLinearWidthProfile::constant(range, *width, &policy())
                .expect("lane profile");
            CrossSectionComponent::traffic_lane(format!("lane-{}", index + 1), profile)
                .expect("traffic lane")
        })
        .collect();
    let cross_section = CrossSection::new(range, components, &policy()).expect("cross-section");
    road(id, alignment, cross_section)
}

fn create_junction(
    network: &mut RoadNetwork,
    road_a: &str,
    road_b: &str,
    junction_id: &str,
) -> JunctionId {
    let candidate = network
        .detect_candidate(
            &RoadId::new(road_a).expect("road id"),
            &RoadId::new(road_b).expect("road id"),
            CrossingRelation::AtGrade,
            &policy(),
        )
        .expect("candidate detection")
        .expect("candidate exists");
    network
        .create_junction(&candidate, junction_id, JunctionOptions::new(), &policy())
        .expect("junction creation")
}

fn install_manual_connection(network: &mut RoadNetwork, junction_id: &JunctionId) {
    let automatic = network
        .junction(junction_id)
        .expect("junction")
        .lane_connections()
        .first()
        .cloned()
        .expect("automatic connection");
    let manual = LaneConnection::new(
        "manual-connection",
        automatic.from_approach_id().clone(),
        automatic.from_lane_id().clone(),
        automatic.to_approach_id().clone(),
        automatic.to_lane_id().clone(),
        automatic.movement(),
    )
    .expect("manual connection");
    network
        .junction_mut(junction_id)
        .expect("mutable junction")
        .replace_lane_connections(vec![manual])
        .expect("manual connection replacement");
}

fn component<'a>(
    snapshot: &'a DerivedEngineeringSnapshot,
    road_id: &str,
    component_id: &str,
) -> &'a DerivedComponent {
    snapshot
        .roads()
        .iter()
        .find(|road| road.id().as_str() == road_id)
        .expect("derived road")
        .components()
        .iter()
        .find(|component| component.component_id().as_str() == component_id)
        .expect("derived component")
}

fn assert_shared_component_trace(
    snapshot: &DerivedEngineeringSnapshot,
    road_id: &str,
    component_id: &str,
) {
    let semantic_ref = SemanticRef::RoadComponent {
        road_id: RoadId::new(road_id).expect("road id"),
        component_id: ComponentId::new(component_id).expect("component id"),
    };
    let two_d = derive_diagnostic_2d(snapshot).expect("2D derivation");
    let three_d = derive_diagnostic_3d(snapshot).expect("3D derivation");
    let two_d_matches = two_d.matching_semantic_ref(&semantic_ref);
    let three_d_matches = three_d.matching_semantic_ref(&semantic_ref);
    assert!(!two_d_matches.is_empty(), "2D semantic trace missing");
    assert_eq!(two_d_matches.len(), three_d_matches.len());

    let shared = component(snapshot, road_id, component_id);
    assert_eq!(shared.strips().len(), two_d_matches.len());
    for ((two_d_primitive, three_d_mesh), strip) in two_d_matches
        .iter()
        .zip(three_d_matches.iter())
        .zip(shared.strips())
    {
        let DiagnosticPrimitive2D::Polygon(polygon) = two_d_primitive else {
            panic!("component must be represented by 2D polygons");
        };
        assert_eq!(polygon.role(), PrimitiveRole::RoadComponentSurface);
        assert_eq!(three_d_mesh.role(), PrimitiveRole::RoadComponentSurface);
        assert_eq!(three_d_mesh.topology(), MeshTopology::Triangles);
        assert_eq!(polygon.vertices().len(), strip.vertices().len());
        assert_eq!(three_d_mesh.positions().len(), strip.vertices().len());
        for (shared_point, local_point) in strip.vertices().iter().zip(polygon.vertices()) {
            let expected_local = Point2::new(
                shared_point.x - snapshot.render_origin().x,
                shared_point.y - snapshot.render_origin().y,
            );
            assert_eq!(*local_point, expected_local);
        }
        for (position, local_point) in three_d_mesh.positions().iter().zip(polygon.vertices()) {
            assert!((f64::from(position[0]) - local_point.x).abs() <= 1.0e-5);
            assert!((f64::from(position[1]) - local_point.y).abs() <= 1.0e-5);
            assert_eq!(position[2], 0.0);
        }
    }
}

fn assert_junction_trace(network: &RoadNetwork, junction_id: &JunctionId) {
    let snapshot = DerivedEngineeringSnapshot::derive(network, Point2::new(0.0, 0.0), &policy())
        .expect("junction snapshot");
    snapshot
        .validate(&policy())
        .expect("valid junction snapshot");
    let two_d = derive_diagnostic_2d(&snapshot).expect("2D junction derivation");
    let three_d = derive_diagnostic_3d(&snapshot).expect("3D junction derivation");
    two_d.validate(&policy()).expect("valid junction 2D");
    three_d.validate().expect("valid junction 3D");
    for primitive in two_d.primitives() {
        if let DiagnosticPrimitive2D::Polyline(polyline) = primitive {
            assert!(polyline
                .points()
                .windows(2)
                .any(|pair| pair[0].distance_to(pair[1]) > policy().coordinate_coincidence_m));
        }
    }
    for mesh in three_d.meshes() {
        if mesh.topology() == MeshTopology::LineStrip {
            assert!(mesh.indices().windows(2).any(|pair| {
                let first = mesh.positions()[pair[0] as usize];
                let second = mesh.positions()[pair[1] as usize];
                let dx = f64::from(second[0] - first[0]);
                let dy = f64::from(second[1] - first[1]);
                dx.hypot(dy) > FLOAT32_LOCAL_COORDINATE_TOLERANCE_M
            }));
        }
    }
    let junction_ref = SemanticRef::Junction(junction_id.clone());
    assert!(two_d.contains_semantic_ref(&junction_ref));
    assert!(three_d.contains_semantic_ref(&junction_ref));

    let source = network.junction(junction_id).expect("source junction");
    let derived = snapshot
        .junctions()
        .iter()
        .find(|junction| junction.id() == junction_id)
        .expect("derived junction");
    assert_eq!(
        derived.surface(),
        source.surface().expect("source surface").vertices()
    );

    let surface_2d = two_d
        .matching_semantic_ref(&junction_ref)
        .into_iter()
        .find_map(|primitive| match primitive {
            DiagnosticPrimitive2D::Polygon(polygon)
                if polygon.role() == PrimitiveRole::JunctionSurface =>
            {
                Some(polygon)
            }
            _ => None,
        })
        .expect("2D junction surface");
    for (project, local) in derived.surface().iter().zip(surface_2d.vertices()) {
        assert_eq!(
            *local,
            Point2::new(
                project.x - snapshot.render_origin().x,
                project.y - snapshot.render_origin().y,
            )
        );
    }

    for corner in derived.corners() {
        let corner_ref = SemanticRef::Corner(corner.id().clone());
        assert!(two_d.contains_semantic_ref(&corner_ref));
        assert!(three_d.contains_semantic_ref(&corner_ref));
        assert!(!two_d.matching_semantic_ref(&corner_ref).is_empty());
        assert!(!three_d.matching_semantic_ref(&corner_ref).is_empty());
    }
}

#[test]
fn straight_and_curved_roads_trace_one_shared_component_derivation() {
    let mut network = RoadNetwork::new();
    network
        .add_road(road(
            "straight",
            straight_500m(),
            straight_two_lane_cross_section(),
        ))
        .expect("straight road");
    let arc = circular_90deg();
    let accepted_arc_samples = arc
        .sample(SamplingOptions::from_policy(&policy()), &policy())
        .expect("accepted curved alignment samples");
    let arc_cross_section = CrossSection::new(
        arc.station_range(),
        vec![CrossSectionComponent::traffic_lane(
            "lane-1",
            width_profile(
                arc.station_range(),
                &[(0.0, 3.5), (17.3, 4.0), (46.1, 3.0), (arc.length(), 3.0)],
            ),
        )
        .expect("curved variable lane")],
        &policy(),
    )
    .expect("curved cross-section");
    network
        .add_road(road("curved", arc, arc_cross_section))
        .expect("curved road");

    let snapshot = DerivedEngineeringSnapshot::derive(&network, Point2::new(0.0, 0.0), &policy())
        .expect("shared snapshot");
    snapshot.validate(&policy()).expect("valid shared snapshot");
    assert_eq!(snapshot.roads().len(), 2);
    let curved_road = snapshot
        .roads()
        .iter()
        .find(|road| road.id().as_str() == "curved")
        .expect("derived curved road");
    assert!(curved_road.alignment().len() > 2);
    for station_m in [17.3, 46.1] {
        assert!(curved_road
            .alignment()
            .iter()
            .any(|sample| (sample.station_m - station_m).abs() <= policy().station_bound_m));
    }
    for accepted_sample in accepted_arc_samples {
        assert!(curved_road.alignment().iter().any(|sample| {
            (sample.station_m - accepted_sample.station_m).abs() <= policy().station_bound_m
        }));
    }
    assert_shared_component_trace(&snapshot, "straight", "lane-1");
    assert_shared_component_trace(&snapshot, "curved", "lane-1");

    let two_d = derive_diagnostic_2d(&snapshot).expect("2D scene");
    let three_d = derive_diagnostic_3d(&snapshot).expect("3D scene");
    for road_id in ["straight", "curved"] {
        let semantic_ref = SemanticRef::Road(RoadId::new(road_id).expect("road id"));
        assert!(two_d.contains_semantic_ref(&semantic_ref));
        assert!(three_d.contains_semantic_ref(&semantic_ref));
    }
}

#[test]
fn variable_width_add_drop_and_right_turn_pocket_use_general_component_lifecycle() {
    let station_range = range(200.0);
    let variable_profile = width_profile(
        station_range,
        &[(0.0, 3.0), (70.0, 4.5), (140.0, 2.0), (200.0, 3.0)],
    );
    let variable_id = ComponentId::new("lane-variable").expect("variable id");
    let (pocket_cross_section, pocket_id) = right_turn_pocket_cross_section();
    let cross_section = CrossSection::new(
        station_range,
        vec![
            CrossSectionComponent::traffic_lane("lane-variable", variable_profile)
                .expect("variable lane"),
            pocket_cross_section
                .components()
                .iter()
                .find(|component| component.id() == &pocket_id)
                .expect("pocket component")
                .clone(),
        ],
        &policy(),
    )
    .expect("lifecycle cross-section");
    let alignment = Alignment::line(Point2::new(0.0, 0.0), Point2::new(200.0, 0.0), &policy())
        .expect("lifecycle alignment");
    let mut network = RoadNetwork::new();
    network
        .add_road(road("lifecycle", alignment, cross_section))
        .expect("lifecycle road");

    let snapshot = DerivedEngineeringSnapshot::derive(&network, Point2::new(0.0, 0.0), &policy())
        .expect("lifecycle snapshot");
    let variable = component(&snapshot, "lifecycle", variable_id.as_str());
    let pocket = component(&snapshot, "lifecycle", pocket_id.as_str());
    assert!(variable
        .samples()
        .iter()
        .map(|sample| sample.width_m())
        .any(|width| width > 4.0));
    assert_eq!(
        pocket.samples().first().expect("pocket start").width_m(),
        0.0
    );
    assert_eq!(pocket.samples().last().expect("pocket end").width_m(), 0.0);
    assert!(pocket.samples().iter().any(|sample| sample.width_m() > 3.0));
    assert!(!pocket.strips().is_empty());
    snapshot
        .validate(&policy())
        .expect("valid lifecycle snapshot");
    assert_shared_component_trace(&snapshot, "lifecycle", variable_id.as_str());
    assert_shared_component_trace(&snapshot, "lifecycle", pocket_id.as_str());
    derive_diagnostic_2d(&snapshot)
        .expect("2D lifecycle scene")
        .validate(&policy())
        .expect("valid 2D lifecycle scene");
    derive_diagnostic_3d(&snapshot)
        .expect("3D lifecycle scene")
        .validate()
        .expect("valid 3D lifecycle scene");
}

#[test]
fn non_grid_aligned_lifecycle_knots_are_preserved_in_shared_station_grid() {
    let station_range = range(200.0);
    let profile = width_profile(
        station_range,
        &[
            (0.0, 0.0),
            (23.4, 0.0),
            (41.7, 3.25),
            (87.3, 3.25),
            (103.6, 0.0),
            (200.0, 0.0),
        ],
    );
    let lane_id = ComponentId::new("lane-irregular-pocket").expect("lane id");
    let cross_section = CrossSection::new(
        station_range,
        vec![
            CrossSectionComponent::traffic_lane(lane_id.as_str(), profile.clone())
                .expect("irregular lifecycle lane"),
        ],
        &policy(),
    )
    .expect("irregular lifecycle cross-section");
    let alignment = Alignment::line(Point2::new(0.0, 0.0), Point2::new(200.0, 0.0), &policy())
        .expect("irregular lifecycle alignment");
    let mut network = RoadNetwork::new();
    network
        .add_road(road("irregular-pocket", alignment, cross_section))
        .expect("irregular lifecycle road");

    let snapshot = DerivedEngineeringSnapshot::derive(&network, Point2::new(0.0, 0.0), &policy())
        .expect("irregular lifecycle snapshot");
    let derived = component(&snapshot, "irregular-pocket", lane_id.as_str());
    let tolerance = policy().station_bound_m;
    for (station_m, expected_width) in profile
        .knots()
        .iter()
        .map(|knot| (knot.station_m, knot.width_m))
    {
        let sample = derived
            .samples()
            .iter()
            .find(|sample| (sample.station_m() - station_m).abs() <= tolerance)
            .unwrap_or_else(|| panic!("missing authored station {station_m}"));
        assert_eq!(sample.station_m(), station_m);
        assert_eq!(sample.width_m(), expected_width);
        assert_eq!(
            sample.width_m(),
            profile
                .width_at(station_m, &policy())
                .expect("profile knot width")
        );
    }
    assert!(derived
        .samples()
        .iter()
        .any(|sample| (sample.station_m() - 23.4).abs() <= tolerance));
    assert!(derived
        .samples()
        .iter()
        .any(|sample| (sample.station_m() - 41.7).abs() <= tolerance));
    assert!(derived
        .samples()
        .iter()
        .any(|sample| (sample.station_m() - 87.3).abs() <= tolerance));
    assert!(derived
        .samples()
        .iter()
        .any(|sample| (sample.station_m() - 103.6).abs() <= tolerance));

    let first_positive_strip = derived
        .strips()
        .iter()
        .min_by(|left, right| {
            left.station_start_m()
                .partial_cmp(&right.station_start_m())
                .expect("finite strip station")
        })
        .expect("positive lifecycle strips");
    let last_positive_strip = derived
        .strips()
        .iter()
        .max_by(|left, right| {
            left.station_end_m()
                .partial_cmp(&right.station_end_m())
                .expect("finite strip station")
        })
        .expect("positive lifecycle strips");
    assert_eq!(first_positive_strip.station_start_m(), 23.4);
    assert_eq!(last_positive_strip.station_end_m(), 103.6);
    assert!(derived
        .strips()
        .iter()
        .all(|strip| strip.station_start_m() + tolerance >= 23.4));
    assert!(derived
        .strips()
        .iter()
        .all(|strip| strip.station_end_m() - tolerance <= 103.6));
    assert_eq!(
        derived
            .samples()
            .iter()
            .find(|sample| (sample.station_m() - 87.3).abs() <= tolerance)
            .expect("storage end sample")
            .width_m(),
        3.25
    );
    assert_eq!(
        derived
            .samples()
            .iter()
            .find(|sample| (sample.station_m() - 103.6).abs() <= tolerance)
            .expect("drop sample")
            .width_m(),
        0.0
    );
    snapshot
        .validate(&policy())
        .expect("valid irregular snapshot");
    assert_shared_component_trace(&snapshot, "irregular-pocket", lane_id.as_str());
    let rebuilt = DerivedEngineeringSnapshot::derive(&network, Point2::new(0.0, 0.0), &policy())
        .expect("rebuilt irregular lifecycle snapshot");
    assert_eq!(snapshot, rebuilt);
}

#[test]
fn non_grid_aligned_slope_changes_and_shared_samples_match_canonical_state() {
    let station_range = range(120.0);
    let profile = width_profile(
        station_range,
        &[(0.0, 3.0), (37.3, 5.0), (63.8, 4.0), (120.0, 4.0)],
    );
    let cross_section = CrossSection::new(
        station_range,
        vec![
            CrossSectionComponent::traffic_lane("lane-slope", profile.clone())
                .expect("slope-change lane"),
        ],
        &policy(),
    )
    .expect("slope-change cross-section");
    let alignment = Alignment::line(Point2::new(10.0, -4.0), Point2::new(130.0, -4.0), &policy())
        .expect("slope-change alignment");
    let mut network = RoadNetwork::new();
    network
        .add_road(road("slope-change", alignment, cross_section))
        .expect("slope-change road");
    let snapshot = DerivedEngineeringSnapshot::derive(&network, Point2::new(10.0, -4.0), &policy())
        .expect("slope-change snapshot");
    let derived = component(&snapshot, "slope-change", "lane-slope");
    for knot in profile.knots() {
        let sample = derived
            .samples()
            .iter()
            .find(|sample| (sample.station_m() - knot.station_m).abs() <= policy().station_bound_m)
            .expect("slope knot in shared grid");
        assert_eq!(sample.width_m(), knot.width_m);
    }
    assert!(snapshot.roads()[0].alignment().len() > profile.knots().len());
    assert_shared_component_trace(&snapshot, "slope-change", "lane-slope");
}

#[test]
fn every_shared_component_sample_matches_canonical_alignment_and_profile() {
    let station_range = range(120.0);
    let profile = width_profile(
        station_range,
        &[(0.0, 2.5), (37.3, 5.0), (63.8, 4.0), (120.0, 4.0)],
    );
    let cross_section = CrossSection::new(
        station_range,
        vec![
            CrossSectionComponent::traffic_lane("lane-invariant", profile).expect("invariant lane"),
        ],
        &policy(),
    )
    .expect("invariant cross-section");
    let alignment = Alignment::line(Point2::new(25.0, 40.0), Point2::new(145.0, 40.0), &policy())
        .expect("invariant alignment");
    let mut network = RoadNetwork::new();
    network
        .add_road(road("invariant", alignment, cross_section))
        .expect("invariant road");
    let snapshot = DerivedEngineeringSnapshot::derive(&network, Point2::new(25.0, 40.0), &policy())
        .expect("invariant snapshot");
    let source = network
        .road(&RoadId::new("invariant").expect("road id"))
        .expect("source road");
    let derived_road = &snapshot.roads()[0];
    for derived_component in derived_road.components() {
        for sample in derived_component.samples() {
            let source_point = source
                .alignment()
                .point_at(sample.station_m(), &policy())
                .expect("source point");
            let source_normal = source
                .alignment()
                .normal_at(sample.station_m(), &policy())
                .expect("source normal");
            let states = source
                .cross_section()
                .states_at(sample.station_m(), &policy())
                .expect("source states");
            let total_width: f64 = states.iter().map(|state| state.width_m).sum();
            let component_index = states
                .iter()
                .position(|state| state.id == *derived_component.component_id())
                .expect("component index");
            assert_eq!(sample.width_m(), states[component_index].width_m);
            let lateral_before: f64 = states[..component_index]
                .iter()
                .map(|state| state.width_m)
                .sum();
            let expected_left =
                source_point + source_normal * (-total_width / 2.0 + lateral_before);
            let expected_right = expected_left + source_normal * sample.width_m();
            assert_eq!(
                derived_road
                    .alignment()
                    .iter()
                    .find(|alignment_sample| { alignment_sample.station_m == sample.station_m() })
                    .expect("shared alignment sample")
                    .point,
                source_point
            );
            assert!(
                sample.left_boundary().distance_to(expected_left)
                    <= policy().coordinate_coincidence_m
            );
            assert!(
                sample.right_boundary().distance_to(expected_right)
                    <= policy().coordinate_coincidence_m
            );
        }
    }
}

#[test]
fn t_four_leg_and_skewed_junctions_trace_accepted_r1b_geometry() {
    let mut t_network = RoadNetwork::new();
    t_network
        .add_road(line_road(
            "main",
            Point2::new(-100.0, 0.0),
            Point2::new(100.0, 0.0),
            &[3.5],
        ))
        .expect("T main");
    t_network
        .add_road(line_road(
            "stem",
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 100.0),
            &[3.5],
        ))
        .expect("T stem");
    let t_id = create_junction(&mut t_network, "main", "stem", "j-t");
    assert_junction_trace(&t_network, &t_id);
    assert_eq!(
        t_network.junction(&t_id).expect("T source").corners().len(),
        3
    );

    let mut four_leg_network = RoadNetwork::new();
    four_leg_network
        .add_road(line_road(
            "east-west",
            Point2::new(-100.0, 0.0),
            Point2::new(100.0, 0.0),
            &[3.5, 3.5],
        ))
        .expect("four-leg horizontal");
    four_leg_network
        .add_road(line_road(
            "north-south",
            Point2::new(0.0, -100.0),
            Point2::new(0.0, 100.0),
            &[3.5, 3.5],
        ))
        .expect("four-leg vertical");
    let four_id = create_junction(&mut four_leg_network, "east-west", "north-south", "j-four");
    assert_junction_trace(&four_leg_network, &four_id);
    assert_eq!(
        four_leg_network
            .junction(&four_id)
            .expect("four-leg source")
            .corners()
            .len(),
        4
    );

    let mut skew_network = RoadNetwork::new();
    skew_network
        .add_road(line_road(
            "skew-main",
            Point2::new(-150.0, 0.0),
            Point2::new(150.0, 0.0),
            &[3.5, 3.5],
        ))
        .expect("skew main");
    skew_network
        .add_road(line_road(
            "skew-stem",
            Point2::new(0.0, 0.0),
            Point2::new(100.0, 70.0),
            &[4.0],
        ))
        .expect("skew stem");
    let skew_id = create_junction(&mut skew_network, "skew-main", "skew-stem", "j-skew");
    assert_junction_trace(&skew_network, &skew_id);
}

#[test]
fn lane_connection_refs_are_junction_scoped_when_geometry_is_deferred() {
    let mut network = RoadNetwork::new();
    network
        .add_road(line_road(
            "j1-main",
            Point2::new(-100.0, 0.0),
            Point2::new(100.0, 0.0),
            &[3.5],
        ))
        .expect("J1 main");
    network
        .add_road(line_road(
            "j1-stem",
            Point2::new(0.0, 0.0),
            Point2::new(0.0, 100.0),
            &[3.5],
        ))
        .expect("J1 stem");
    network
        .add_road(line_road(
            "j2-main",
            Point2::new(900.0, 0.0),
            Point2::new(1100.0, 0.0),
            &[3.5],
        ))
        .expect("J2 main");
    network
        .add_road(line_road(
            "j2-stem",
            Point2::new(1000.0, 0.0),
            Point2::new(1000.0, 100.0),
            &[3.5],
        ))
        .expect("J2 stem");
    let j1 = create_junction(&mut network, "j1-main", "j1-stem", "j1");
    let j2 = create_junction(&mut network, "j2-main", "j2-stem", "j2");
    install_manual_connection(&mut network, &j1);
    install_manual_connection(&mut network, &j2);

    let snapshot = DerivedEngineeringSnapshot::derive(&network, Point2::new(0.0, 0.0), &policy())
        .expect("scoped connection snapshot");
    snapshot.validate(&policy()).expect("valid scoped snapshot");
    let manual_id = LaneConnectionId::new("manual-connection").expect("manual id");
    let j1_ref = SemanticRef::LaneConnection {
        junction_id: j1.clone(),
        connection_id: manual_id.clone(),
    };
    let j2_ref = SemanticRef::LaneConnection {
        junction_id: j2.clone(),
        connection_id: manual_id,
    };
    assert_ne!(j1_ref, j2_ref);
    let j1_derived = snapshot
        .junctions()
        .iter()
        .find(|junction| junction.id() == &j1)
        .expect("derived J1");
    let j2_derived = snapshot
        .junctions()
        .iter()
        .find(|junction| junction.id() == &j2)
        .expect("derived J2");
    assert_eq!(j1_derived.lane_connections().len(), 1);
    assert_eq!(j2_derived.lane_connections().len(), 1);
    assert_eq!(j1_derived.lane_connections()[0].semantic_ref(), &j1_ref);
    assert_eq!(j2_derived.lane_connections()[0].semantic_ref(), &j2_ref);

    let two_d = derive_diagnostic_2d(&snapshot).expect("scoped 2D");
    let three_d = derive_diagnostic_3d(&snapshot).expect("scoped 3D");
    let two_d_refs: HashSet<SemanticRef> = two_d
        .primitives()
        .iter()
        .map(|primitive| primitive.semantic_ref().clone())
        .collect();
    let three_d_refs: HashSet<SemanticRef> = three_d
        .meshes()
        .iter()
        .map(|mesh| mesh.semantic_ref().clone())
        .collect();
    assert_eq!(two_d_refs, three_d_refs);
    // F-03 deliberately defers connection-path geometry, so neither adapter
    // fabricates a selectable zero-length primitive.
    assert!(!two_d.contains_semantic_ref(&j1_ref));
    assert!(!two_d.contains_semantic_ref(&j2_ref));
    assert!(!three_d.contains_semantic_ref(&j1_ref));
    assert!(!three_d.contains_semantic_ref(&j2_ref));

    let scoped_refs: HashSet<SemanticRef> = snapshot
        .junctions()
        .iter()
        .flat_map(|junction| {
            junction
                .lane_connections()
                .iter()
                .map(|connection| connection.semantic_ref().clone())
        })
        .collect();
    assert!(scoped_refs.contains(&j1_ref));
    assert!(scoped_refs.contains(&j2_ref));
    assert_eq!(scoped_refs.len(), 2);
    assert!(j1_derived
        .lane_connections()
        .iter()
        .all(|connection| connection.semantic_ref() != &j2_ref));
    assert!(j2_derived
        .lane_connections()
        .iter()
        .all(|connection| connection.semantic_ref() != &j1_ref));

    let rebuilt = DerivedEngineeringSnapshot::derive(&network, Point2::new(0.0, 0.0), &policy())
        .expect("rebuilt scoped snapshot");
    assert_eq!(snapshot, rebuilt);
}

#[test]
fn selection_refs_are_scoped_and_parity_is_shared_between_2d_and_3d() {
    let mut network = RoadNetwork::new();
    network
        .add_road(road(
            "road-a",
            straight_500m(),
            straight_two_lane_cross_section(),
        ))
        .expect("road A");
    network
        .add_road(road(
            "road-b",
            Alignment::line(Point2::new(0.0, 20.0), Point2::new(500.0, 20.0), &policy())
                .expect("road B alignment"),
            straight_two_lane_cross_section(),
        ))
        .expect("road B");
    let snapshot = DerivedEngineeringSnapshot::derive(&network, Point2::new(0.0, 0.0), &policy())
        .expect("selection snapshot");
    let two_d = derive_diagnostic_2d(&snapshot).expect("selection 2D");
    let three_d = derive_diagnostic_3d(&snapshot).expect("selection 3D");

    let road_a = SemanticRef::Road(RoadId::new("road-a").expect("road A id"));
    let road_b = SemanticRef::Road(RoadId::new("road-b").expect("road B id"));
    let lane_a = SemanticRef::RoadComponent {
        road_id: RoadId::new("road-a").expect("road A id"),
        component_id: ComponentId::new("lane-1").expect("lane id"),
    };
    let lane_b = SemanticRef::RoadComponent {
        road_id: RoadId::new("road-b").expect("road B id"),
        component_id: ComponentId::new("lane-1").expect("lane id"),
    };
    assert_ne!(lane_a, lane_b);
    for semantic_ref in [&road_a, &road_b, &lane_a, &lane_b] {
        assert!(!two_d.matching_semantic_ref(semantic_ref).is_empty());
        assert!(!three_d.matching_semantic_ref(semantic_ref).is_empty());
    }
    let two_d_refs: HashSet<SemanticRef> = two_d
        .primitives()
        .iter()
        .map(|primitive| primitive.semantic_ref().clone())
        .collect();
    let three_d_refs: HashSet<SemanticRef> = three_d
        .meshes()
        .iter()
        .map(|mesh| mesh.semantic_ref().clone())
        .collect();
    assert!(two_d_refs.contains(&lane_a));
    assert!(two_d_refs.contains(&lane_b));
    assert_eq!(two_d_refs, three_d_refs);
}

#[test]
fn large_project_coordinates_use_f64_origin_then_float32_local_buffers() {
    let origin = Point2::new(1_000_000_000.0, 1_000_000_000.0);
    let start = Point2::new(origin.x + 0.375, origin.y + 0.625);
    let end = Point2::new(origin.x + 80.375, origin.y + 0.625);
    let alignment = Alignment::line(start, end, &policy()).expect("large alignment");
    let cross_section = constant_cross_section(
        alignment.station_range(),
        &[("lane-precision", ComponentKind::TrafficLane, 3.5)],
    );
    let mut network = RoadNetwork::new();
    network
        .add_road(road("large", alignment, cross_section))
        .expect("large road");
    let snapshot =
        DerivedEngineeringSnapshot::derive(&network, origin, &policy()).expect("large snapshot");
    let sample = snapshot.roads()[0].alignment()[0].point;
    assert!(sample.x > 1_000_000_000.0);
    assert_eq!(snapshot.metadata().render_origin(), origin);
    assert!(snapshot.project_extents().min().x > 1_000_000_000.0);

    let two_d = derive_diagnostic_2d(&snapshot).expect("large 2D");
    let three_d = derive_diagnostic_3d(&snapshot).expect("large 3D");
    let local_start = two_d
        .primitives()
        .iter()
        .find_map(|primitive| match primitive {
            DiagnosticPrimitive2D::Polyline(polyline)
                if polyline.role() == PrimitiveRole::RoadAlignment =>
            {
                polyline.points().first()
            }
            _ => None,
        })
        .expect("local alignment start");
    assert!((local_start.x - 0.375).abs() <= 1.0e-9);
    assert!((local_start.y - 0.625).abs() <= 1.0e-9);
    assert!(three_d
        .meshes()
        .iter()
        .flat_map(|mesh| mesh.positions())
        .all(|position| position.iter().all(|coordinate| coordinate.is_finite())));

    let bad_sequence = (start.x as f32) - (origin.x as f32);
    let good_sequence = (start.x - origin.x) as f32;
    assert_eq!(bad_sequence, 0.0);
    assert!((f64::from(good_sequence) - 0.375).abs() <= 1.0e-6);
    let first_buffer_position = three_d
        .meshes()
        .iter()
        .find(|mesh| mesh.role() == PrimitiveRole::RoadAlignment)
        .expect("alignment buffer")
        .positions()[0];
    assert!((f64::from(first_buffer_position[0]) - 0.375).abs() <= 1.0e-6);
    assert!((f64::from(first_buffer_position[1]) - 0.625).abs() <= 1.0e-6);
}

#[test]
fn changing_origin_only_changes_local_adapters_and_keeps_shared_ids() {
    let mut network = RoadNetwork::new();
    network
        .add_road(road(
            "origin-road",
            straight_500m(),
            straight_two_lane_cross_section(),
        ))
        .expect("origin road");
    let first_origin = Point2::new(0.0, 0.0);
    let second_origin = Point2::new(100.0, -50.0);
    let first = DerivedEngineeringSnapshot::derive(&network, first_origin, &policy())
        .expect("first snapshot");
    let second = first
        .with_render_origin(second_origin)
        .expect("second origin");
    assert_eq!(first.project_extents(), second.project_extents());
    assert_eq!(first.roads(), second.roads());
    assert_ne!(first.metadata(), second.metadata());
    let first_2d = derive_diagnostic_2d(&first).expect("first 2D");
    let second_2d = derive_diagnostic_2d(&second).expect("second 2D");
    assert_eq!(first_2d.render_origin(), first_origin);
    assert_eq!(second_2d.render_origin(), second_origin);
    let first_point = first_2d
        .primitives()
        .iter()
        .find_map(|primitive| match primitive {
            DiagnosticPrimitive2D::Polyline(polyline)
                if polyline.role() == PrimitiveRole::RoadAlignment =>
            {
                polyline.points().first()
            }
            _ => None,
        })
        .expect("first local point");
    let second_point = second_2d
        .primitives()
        .iter()
        .find_map(|primitive| match primitive {
            DiagnosticPrimitive2D::Polyline(polyline)
                if polyline.role() == PrimitiveRole::RoadAlignment =>
            {
                polyline.points().first()
            }
            _ => None,
        })
        .expect("second local point");
    assert_eq!(
        *second_point,
        Point2::new(
            first_point.x + first_origin.x - second_origin.x,
            first_point.y + first_origin.y - second_origin.y,
        )
    );
    let first_3d = derive_diagnostic_3d(&first).expect("first 3D");
    let second_3d = derive_diagnostic_3d(&second).expect("second 3D");
    let first_refs: HashSet<SemanticRef> = first_3d
        .meshes()
        .iter()
        .map(|mesh| mesh.semantic_ref().clone())
        .collect();
    let second_refs: HashSet<SemanticRef> = second_3d
        .meshes()
        .iter()
        .map(|mesh| mesh.semantic_ref().clone())
        .collect();
    assert_eq!(first_refs, second_refs);
}

#[test]
fn disposable_rebuild_is_deterministic_and_semantic_update_has_no_stale_geometry() {
    let mut network = RoadNetwork::new();
    let alignment = Alignment::line(Point2::new(0.0, 0.0), Point2::new(120.0, 0.0), &policy())
        .expect("update alignment");
    let initial_cross_section = constant_cross_section(
        alignment.station_range(),
        &[("lane-update", ComponentKind::TrafficLane, 3.5)],
    );
    network
        .add_road(road(
            "update-road",
            alignment.clone(),
            initial_cross_section,
        ))
        .expect("update road");

    let (first_snapshot, first_2d, first_3d) = {
        let snapshot =
            DerivedEngineeringSnapshot::derive(&network, Point2::new(0.0, 0.0), &policy())
                .expect("first snapshot");
        let two_d = derive_diagnostic_2d(&snapshot).expect("first 2D");
        let three_d = derive_diagnostic_3d(&snapshot).expect("first 3D");
        (snapshot, two_d, three_d)
    };
    let rebuilt_snapshot =
        DerivedEngineeringSnapshot::derive(&network, Point2::new(0.0, 0.0), &policy())
            .expect("rebuilt snapshot");
    let rebuilt_2d = derive_diagnostic_2d(&rebuilt_snapshot).expect("rebuilt 2D");
    let rebuilt_3d = derive_diagnostic_3d(&rebuilt_snapshot).expect("rebuilt 3D");
    assert_eq!(first_snapshot, rebuilt_snapshot);
    assert_eq!(first_2d, rebuilt_2d);
    assert_eq!(first_3d, rebuilt_3d);

    let changed_cross_section = constant_cross_section(
        alignment.station_range(),
        &[("lane-update", ComponentKind::TrafficLane, 4.5)],
    );
    network
        .replace_road(road("update-road", alignment, changed_cross_section))
        .expect("replace road");
    let updated_snapshot =
        DerivedEngineeringSnapshot::derive(&network, Point2::new(0.0, 0.0), &policy())
            .expect("updated snapshot");
    let updated_2d = derive_diagnostic_2d(&updated_snapshot).expect("updated 2D");
    let updated_3d = derive_diagnostic_3d(&updated_snapshot).expect("updated 3D");
    let semantic_ref = SemanticRef::RoadComponent {
        road_id: RoadId::new("update-road").expect("road id"),
        component_id: ComponentId::new("lane-update").expect("component id"),
    };
    assert_eq!(
        first_2d.matching_semantic_ref(&semantic_ref).len(),
        updated_2d.matching_semantic_ref(&semantic_ref).len()
    );
    assert_eq!(
        first_3d.matching_semantic_ref(&semantic_ref).len(),
        updated_3d.matching_semantic_ref(&semantic_ref).len()
    );
    assert_ne!(
        first_2d.matching_semantic_ref(&semantic_ref),
        updated_2d.matching_semantic_ref(&semantic_ref)
    );
    assert_ne!(
        first_3d.matching_semantic_ref(&semantic_ref),
        updated_3d.matching_semantic_ref(&semantic_ref)
    );
    assert_eq!(updated_snapshot.junctions().len(), 0);
}

#[test]
fn bounded_width_and_origin_matrix_remains_finite_and_deterministic() {
    for (index, width) in [0.5, 3.5, 6.0, 10.0].into_iter().enumerate() {
        let start = 1_000_000.0 * index as f64;
        let alignment = Alignment::line(
            Point2::new(start, -start),
            Point2::new(start + 80.0, -start + 5.0),
            &policy(),
        )
        .expect("matrix alignment");
        let cross_section = constant_cross_section(
            alignment.station_range(),
            &[("lane-matrix", ComponentKind::TrafficLane, width)],
        );
        let mut network = RoadNetwork::new();
        network
            .add_road(road("matrix", alignment, cross_section))
            .expect("matrix road");
        let origin = Point2::new(start, -start);
        let first = DerivedEngineeringSnapshot::derive(&network, origin, &policy())
            .expect("matrix snapshot");
        let second =
            DerivedEngineeringSnapshot::derive(&network, origin, &policy()).expect("matrix repeat");
        assert_eq!(first, second);
        first.validate(&policy()).expect("matrix shared validity");
        let two_d = derive_diagnostic_2d(&first).expect("matrix 2D");
        let three_d = derive_diagnostic_3d(&first).expect("matrix 3D");
        two_d.validate(&policy()).expect("matrix 2D validity");
        three_d.validate().expect("matrix 3D validity");
    }
}
