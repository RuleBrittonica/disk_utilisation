use crate::lib_filesystem::filesystem::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// Function to build a folder tree from a starting folder.
pub fn build_folder_tree(start_path: &Path) -> Folder {
    let folder_name = start_path.file_name().unwrap_or_default().to_string_lossy().into_owned();
    let mut root_folder = Folder::new(folder_name, start_path.to_path_buf());

    for entry in WalkDir::new(start_path).min_depth(1).max_depth(1) {
        let entry = entry.unwrap();
        let path = entry.path().to_path_buf();
        let name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();

        if entry.file_type().is_dir() {
            // Recursively build subfolder tree
            let subfolder = build_folder_tree(&path);
            root_folder.subfolders.push(subfolder);
        } else if entry.file_type().is_file() {
            // Create a file and add it to the current folder
            let extn = path.extension().unwrap_or_default().to_string_lossy().into_owned();
            let file = File::new(name, extn, path);
            root_folder.subfiles.push(file);
        }
    }

    root_folder
}