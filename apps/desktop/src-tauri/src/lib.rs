#[cfg(feature = "native-benchmark")]
use std::sync::Mutex;

#[cfg(feature = "native-benchmark")]
use serde::Serialize;
#[cfg(feature = "native-benchmark")]
use street_concept_designer_bridge_common::BridgeSession;
#[cfg(feature = "native-benchmark")]
use tauri::{ipc::Response, State};

#[cfg(feature = "native-benchmark")]
struct AppState(Mutex<BridgeSession>);

#[cfg(feature = "native-benchmark")]
#[derive(Debug, Serialize)]
struct SessionInfo {
    bridge_owner: &'static str,
    revision: u64,
}

#[cfg(feature = "native-benchmark")]
#[tauri::command]
fn bridge_info(state: State<'_, AppState>) -> Result<SessionInfo, String> {
    let session = state
        .0
        .lock()
        .map_err(|_| "session lock poisoned".to_owned())?;
    Ok(SessionInfo {
        bridge_owner: "native-tauri",
        revision: session.revision(),
    })
}

#[cfg(feature = "native-benchmark")]
#[tauri::command]
fn bridge_revision(state: State<'_, AppState>) -> Result<u64, String> {
    let session = state
        .0
        .lock()
        .map_err(|_| "session lock poisoned".to_owned())?;
    Ok(session.revision())
}

#[cfg(feature = "native-benchmark")]
#[tauri::command]
fn bridge_scene(state: State<'_, AppState>, size: String) -> Result<Response, String> {
    let session = state
        .0
        .lock()
        .map_err(|_| "session lock poisoned".to_owned())?;
    session
        .scene(
            street_concept_designer_bridge_common::SceneSize::parse(&size)
                .map_err(|e| e.to_string())?,
        )
        .map(Response::new)
        .map_err(|e| e.to_string())
}

#[cfg(feature = "native-benchmark")]
#[tauri::command]
fn bridge_preview_scene(state: State<'_, AppState>, sample: u32) -> Result<Response, String> {
    let session = state
        .0
        .lock()
        .map_err(|_| "session lock poisoned".to_owned())?;
    session
        .preview_scene(sample)
        .map(Response::new)
        .map_err(|e| e.to_string())
}

#[cfg(feature = "native-benchmark")]
#[tauri::command]
fn bridge_commit(state: State<'_, AppState>, sample: u32) -> Result<u64, String> {
    let mut session = state
        .0
        .lock()
        .map_err(|_| "session lock poisoned".to_owned())?;
    session.commit(sample).map_err(|e| e.to_string())
}

#[cfg(feature = "native-benchmark")]
#[tauri::command]
fn bridge_reset(state: State<'_, AppState>) -> Result<u64, String> {
    let mut session = state
        .0
        .lock()
        .map_err(|_| "session lock poisoned".to_owned())?;
    session.reset().map_err(|e| e.to_string())
}

#[cfg(feature = "native-benchmark")]
#[tauri::command]
fn bridge_stale_probe(state: State<'_, AppState>) -> Result<(), String> {
    let session = state
        .0
        .lock()
        .map_err(|_| "session lock poisoned".to_owned())?;
    session.stale_probe().map_err(|e| e.to_string())
}

#[cfg(feature = "native-benchmark")]
pub fn run() {
    let session = BridgeSession::new().expect("R3B fixture must be valid");
    tauri::Builder::default()
        .manage(AppState(Mutex::new(session)))
        .invoke_handler(tauri::generate_handler![
            bridge_info,
            bridge_revision,
            bridge_scene,
            bridge_preview_scene,
            bridge_commit,
            bridge_reset,
            bridge_stale_probe,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Street Concept Designer");
}

#[cfg(not(feature = "native-benchmark"))]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running Street Concept Designer");
}
