// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{State, Manager};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

mod counter;
mod emitter;
mod tetromino;
mod game;
use counter::{Counter, CounterRunner};
use game::{Game, GameRunner, BOARD_ROWS, BOARD_COLS};
use emitter::Emitter;

fn make_menu() -> Menu {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu)
}

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
fn process_arrow_key(key: &str, game_runner: State<GameRunner>) -> bool {
    // Move the tetromino left, right or down. Returns success if the move can be made, fail if
    // the move can not be made.

    // Early return if game is not running
    if !game_runner.get_running() { return false; }

    let mut game = game_runner.game.lock().unwrap();
    let success = game.proces_arrow_key(key);
    success
}

#[tauri::command]
fn process_spacebar(game_runner: State<GameRunner>) -> bool {
    // Hard drop on space: Move the tetromino all the way down

    // Early return if game is not running
    if !game_runner.get_running() { return false; }

    let mut game = game_runner.game.lock().unwrap();
    game.process_hard_drop();
    true
}

#[tauri::command]
fn process_rotation(direction: &str, game_runner: State<GameRunner>) -> bool {
    // Rotate the tetromino clockwise or counter-clockwise.
    // Returns success if the move can be made, fail if the move can not be made.

    // Early return if game is not running
    if !game_runner.get_running() { return false; }

    let mut game = game_runner.game.lock().unwrap();
    let success = game.process_rotation(direction);
    success
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
        .menu(make_menu())
        .invoke_handler(tauri::generate_handler![
            greet,
            start_counter,
            pause_counter,
            reset_counter,
            add_value,
            get_board_dimensions,
            start_game,
            reset_game,
            process_arrow_key,
            process_spacebar,
            process_rotation,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
