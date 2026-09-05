use wasm_bindgen::prelude::*;

use street_concept_designer_bridge_common::{BridgeSession, SceneSize};

#[wasm_bindgen]
pub struct WasmBridge {
    session: BridgeSession,
}

#[wasm_bindgen]
impl WasmBridge {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WasmBridge, JsValue> {
        Ok(Self {
            session: BridgeSession::new().map_err(error)?,
        })
    }

    pub fn revision(&self) -> u64 {
        self.session.revision()
    }

    pub fn scene(&self, size: &str) -> Result<Vec<u8>, JsValue> {
        self.session
            .scene(SceneSize::parse(size).map_err(error)?)
            .map_err(error)
    }

    pub fn preview_scene(&self, sample: u32) -> Result<Vec<u8>, JsValue> {
        self.session.preview_scene(sample).map_err(error)
    }

    pub fn commit(&mut self, sample: u32) -> Result<u64, JsValue> {
        self.session.commit(sample).map_err(error)
    }

    pub fn reset(&mut self) -> Result<u64, JsValue> {
        self.session.reset().map_err(error)
    }

    pub fn stale_probe(&self) -> Result<(), JsValue> {
        self.session.stale_probe().map_err(error)
    }
}

fn error(error: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&error.to_string())
}
