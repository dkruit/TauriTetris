// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{State, Manager};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

mod counter;
mod emitter;
mod game;
use counter::{Counter, CounterRunner};
use game::{Game, GameRunner};
use emitter::Emitter;

fn make_menu() -> Menu {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);
    return menu;
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    return format!("Hello, {}! You've been greeted from Rust!", name);
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
fn start_game(game_runner: State<GameRunner>) {
    game_runner.run();
}

#[tauri::command]
fn reset_game(game_runner: State<GameRunner>) {
    // Stop the game by setting the running flag to false
    game_runner.reset();
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

            return Ok(());
        })
        .menu(make_menu())
        .invoke_handler(tauri::generate_handler![
            greet,
            start_counter,
            pause_counter,
            reset_counter,
            add_value,
            start_game,
            reset_game,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
