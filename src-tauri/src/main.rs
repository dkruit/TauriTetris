// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{State, Manager};

mod counter;
mod emitter;
mod tetromino;
mod game;
use counter::{Counter, CounterRunner};
use game::{Game, GameRunner, BOARD_ROWS, BOARD_COLS};
use emitter::Emitter;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn start_counter(counter_runner: State<CounterRunner>) {
    counter_runner.run();
}

#[tauri::command]
fn add_value(value: i32, counter_runner: State<CounterRunner>) {
    // Add a number to the value of the counter.
    let mut counter = counter_runner.counter.lock().unwrap();
    for _i in 0..value { counter.increment(); }
}

#[tauri::command]
fn pause_counter(counter_runner: State<CounterRunner>) {
    // Stop the counter by setting the running flag to false
    counter_runner.pause();
}

#[tauri::command]
fn reset_counter(counter_runner: State<CounterRunner>) {
    // Stop the counter by setting the running flag to false
    counter_runner.reset();
}

#[tauri::command]
fn get_board_dimensions() -> (i32, i32) {
    (BOARD_ROWS as i32, BOARD_COLS as i32)
}

#[tauri::command]
fn start_game(game_runner: State<GameRunner>) {
    game_runner.run();
}

#[tauri::command]
fn reset_game(game_runner: State<GameRunner>) {
    // Stop the game by setting the running flag to false
    game_runner.reset();
}

#[tauri::command]
fn process_command(command: &str, game_runner: State<GameRunner>) -> bool {
    // Early return if game is not running
    if !game_runner.get_running() { return false; }

    let mut game = game_runner.game.lock().unwrap();
    // Early return if game is over
    if game.get_game_over() {
        return false;
    }

    match command {
        "down" | "left" | "right" => { game.proces_arrow_key(command) },
        "clockwise" | "counter-clockwise" => { game.process_rotation(command) },
        "hard-drop" => { game.process_hard_drop(); true }
        _ => { false }
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let emitter = Emitter::new(app_handle);

            // Create a Counter instance with a sleep time of 1 second
            let counter_runner = CounterRunner::new(Counter::new(1.0, emitter));
            app.manage(counter_runner);

            let app_handle = app.handle();
            let emitter = Emitter::new(app_handle);
            let game_runner = GameRunner::new(Game::new(emitter));
            app.manage(game_runner);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            start_counter,
            pause_counter,
            reset_counter,
            add_value,
            get_board_dimensions,
            start_game,
            reset_game,
            process_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
