// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Builder, generate_context};

mod menu;
mod filesystem;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    Builder::default()
        .menu(menu::build_menu())
        .on_menu_event(|event| {
            println!("Menu item selected: {}", event.menu_item_id());
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(generate_context!())
        .expect("error while running tauri application");
}
