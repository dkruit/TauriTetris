use tauri::{AppHandle, Manager};

#[derive(Clone, serde::Serialize)]
struct Payload {
  value: String,
}

pub struct Emitter {
    app_handle: AppHandle
}

impl Emitter {
    pub fn new(app_handle: AppHandle) -> Self {
        Emitter{ app_handle }
    }

    pub fn emit(&self, event_name: &str, payload: String) {
        self.app_handle
            .emit_all(event_name, Payload { value: payload })
            .unwrap();
    }
}
