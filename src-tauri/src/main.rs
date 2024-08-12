// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Builder, generate_context};

mod menu;
mod drives;
mod memory;

fn main() {
    Builder::default()
        .menu(menu::build_menu())
        .on_menu_event(|event| {
            println!("Menu item selected: {}", event.menu_item_id());
        })
        .invoke_handler(tauri::generate_handler![
            drives::get_drives,
            memory::get_memory_usage,
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}

// use std::path::Path;

// mod lib_filesystem;

// use lib_filesystem::{filesystem::{Folder, File}, tree_builder, tree_display, tree_destructor};

// fn main() {
//     let start_path: &Path = Path::new("tmp/test_1");
//     let mut folder_tree: Folder = tree_builder::start_folder_tree(start_path);
//     // folder_tree = tree_builder::grow_folder_tree(folder_tree, 2);

//     let all_folders: Vec<Folder> = tree_destructor::get_all_folders(folder_tree.clone());
//     let all_files: Vec<File> = tree_destructor::get_all_files(folder_tree.clone());

//     println!("Folders:");
//     for folder in &all_folders {
//         println!("{}", folder);
//     }

//     println!("Files:");
//     for file in &all_files {
//         println!("{}", file);
//     }


//     // Print the constructed folder tree with details
//     tree_display::print_folder_tree(&folder_tree, 0);
// }
