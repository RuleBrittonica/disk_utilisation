use crate::lib_filesystem::filesystem::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Function to start a folder tree from a given folder
///
/// # Arguments
///
/// * `start_path` - a reference to a `Path` where the tree will start from
pub fn start_folder_tree(start_path: &Path) -> Folder {
    let folder_name = start_path.file_name().unwrap_or_default().to_string_lossy().into_owned();
    let folder_path = start_path.to_path_buf();
    Folder::new(folder_name, folder_path)
}

/// Continues growing out a Folder Tree, will grow to the depth specified by the
/// user
///
/// # Arguments
///
/// * `folder` - The folder to continue growing
/// * `depth`  - The depth to grow the tree to
///
/// # Returns
///
/// The new folder object
pub fn grow_folder_tree(folder: Folder, depth: u32) -> Folder {
    folder
}


/// Merges two Folder Trees together. The sub folder must be a part of the main
/// folder at some level. If the folder is empty, then it is replaced with the
/// given sub_folder. If the folder is not empty, then its contents is either
/// overwritten, or added together. If the sub_folder isn't found in
/// the tree, then we throw an error
///
/// # Arguments
///
/// * `main_folder` - The `Folder` to merge into - This Folder will be returned
/// * `sub_folder`  - The `Folder` to be merged
/// * `overwrite`   - True - overwrite the items in the main folder with the items in the sub folder | False - merge the two folders together.
///
/// # Returns
///
/// The new Folder object (if the sub_folder was found in the main tree).
/// Otherwise Errors.
pub fn merge_folder_tree(main_folder: Folder, sub_folder: Folder, overwrite: bool) -> Folder {
    main_folder
}



// Function to build a folder tree from a starting folder.
// pub fn build_folder_tree(start_path: &Path) -> Folder {
//     let folder_name = start_path.file_name().unwrap_or_default().to_string_lossy().into_owned();
//     let mut root_folder = Folder::new(folder_name, start_path.to_path_buf());

//     for entry in WalkDir::new(start_path).min_depth(1).max_depth(1) {
//         let entry = entry.unwrap();
//         let path = entry.path().to_path_buf();
//         let name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();

//         if entry.file_type().is_dir() {
//             // Recursively build subfolder tree
//             let subfolder = build_folder_tree(&path);
//             root_folder.subfolders.push(subfolder);
//         } else if entry.file_type().is_file() {
//             // Create a file and add it to the current folder
//             let extn = path.extension().unwrap_or_default().to_string_lossy().into_owned();
//             let file = File::new(name, extn, path);
//             root_folder.subfiles.push(file);
//         }
//     }

//     root_folder
// }