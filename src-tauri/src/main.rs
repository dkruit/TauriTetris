// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex, atomic};
use std::thread;
use std::time::Duration;

use tauri::{State, Manager};
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

mod tetris;
mod emitter;
use tetris::{Counter, CounterRunner};
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
    // Early return if the counter is already started
    if counter_runner.running.load(atomic::Ordering::SeqCst) {
        println!("Counter is already running!");
        return;
    }

    // Set running flag to true
    counter_runner.running.store(true, atomic::Ordering::SeqCst);

    // Clone the counter and running flag to move them to the thread
    let counter = counter_runner.counter.clone();
    let running_flag = counter_runner.running.clone();

    // Spawn a thread to increment the counter at set intervals
    thread::spawn(move || {
        let mut sleep_time;

        while running_flag.load(atomic::Ordering::SeqCst) {
            {
                let mut counter = counter.lock().unwrap();
                counter.increment();
                sleep_time = counter.get_sleep_time();
            }
            thread::sleep(Duration::from_secs_f64(sleep_time));
        }
    });
}

#[tauri::command]
fn add_value(value: i32, counter_runner: State<CounterRunner>) {
    // Add a number to the value of the counter.
    let mut counter = counter_runner.counter.lock().unwrap();
    for _i in 0..value { counter.increment(); }
}

#[tauri::command]
fn stop_counter(counter_runner: State<CounterRunner>) {
    // Stop the counter by setting the running flag to false
    counter_runner.running.store(false, atomic::Ordering::SeqCst);
}


fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let emitter = Emitter::new(app_handle);

            // Create a Counter instance with a sleep time of 1 second
            let counter_runner = CounterRunner {
                counter: Arc::new(Mutex::new(Counter::new(1.0, emitter))),
                running: Arc::new(atomic::AtomicBool::new(false))
            };
            app.manage(counter_runner);

            Ok(())
        })
        .menu(make_menu())
        .invoke_handler(tauri::generate_handler![greet, start_counter, stop_counter, add_value])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
