use tauri::{AppHandle, Manager};

use crate::game::Tetromino;

#[derive(Clone, serde::Serialize)]
struct NumberPayload {
  value: f64,
}

#[derive(Clone, serde::Serialize)]
struct StringPayload {
  value: String,
}

#[derive(Clone, serde::Serialize)]
struct TetrominoPayload<'a> {
    name: char,
    occupied_positions: &'a Vec<(i32, i32)>
}

pub struct Emitter {
    app_handle: AppHandle
}

impl Emitter {
    pub fn new(app_handle: AppHandle) -> Self {
        return Emitter{ app_handle };
    }

    pub fn emit_string(&self, event_name: &str, payload: String) {
        self.app_handle
            .emit_all(event_name, StringPayload { value: payload })
            .unwrap();
    }

    pub fn emit_number<N>(&self, event_name: &str, payload: N)
    where N: Into<f64> + Copy,
    {
        let payload = payload.into();
        self.app_handle
            .emit_all(event_name, NumberPayload { value: payload })
            .unwrap();
    }

    pub fn emit_tetromino(&self, event_name: &str, tetromino: &Tetromino) {
        let payload = TetrominoPayload{
            name: tetromino.get_shape_name(),
            occupied_positions: tetromino.get_occupied_positions()
        };
        self.app_handle
            .emit_all(event_name, payload)
            .unwrap();
    }
}
