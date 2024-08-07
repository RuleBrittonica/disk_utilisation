// // Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use tauri::{Builder, generate_context};

// mod menu;

// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

// fn main() {
//     Builder::default()
//         .menu(menu::build_menu())
//         .on_menu_event(|event| {
//             println!("Menu item selected: {}", event.menu_item_id());
//         })
//         .invoke_handler(tauri::generate_handler![greet])
//         .run(generate_context!())
//         .expect("error while running tauri application");
// }

use std::path::Path;

mod lib_filesystem;

use lib_filesystem::{filesystem::Folder, tree_builder, tree_display};

fn main() {
    let start_path: &Path = Path::new("tmp/test_1");
    let mut folder_tree: Folder = tree_builder::start_folder_tree(start_path);
    folder_tree = tree_builder::grow_folder_tree(folder_tree, 2);

    // Print the constructed folder tree with details
    tree_display::print_folder_tree(&folder_tree, 0);
}
