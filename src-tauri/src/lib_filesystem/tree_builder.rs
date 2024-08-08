use crate::lib_filesystem::filesystem::*;
use std::path::{Path, PathBuf};
use std::fs::{self, DirEntry};

/// Function to start a folder tree from a given folder
///
/// # Arguments
///
/// * `start_path` - a reference to a `Path` where the tree will start from
///
/// # Returns
///
/// The new folder object. The folder object will be marked as a disk if the
/// path is to a root path like `C:\`
///
/// # Example:
///
/// Given a folder structured as follows:
///
/// ```text
/// /project/src
/// ├── utils
/// │   ├── mod.rs
/// │   └── error.rs
/// └── main.rs
/// ```
///
/// The folder object now contains:
///
/// - `name`: "src"
/// - `path`: "/project/src"
/// - `size`: 0
/// - `subfiles`: ["main.rs"]
/// - `subfolders`: ["utils"]
pub fn start_folder_tree(start_path: &Path) -> Folder {
    let folder_path: PathBuf = start_path.to_path_buf();

    let folder: Folder;
    if is_disk(start_path) {
        folder = Folder::new_disk(folder_path);
    } else {
        folder = Folder::new(folder_path);
    }

    populate_folder(folder)
}

fn is_disk(path: &Path) -> bool {
    // On Windows, check if the path is a root path like `C:\`
    path.parent().is_none()
}

/// Function to populate a foldfer with Subfolders and files
///
/// # Arguments
///
/// * `folder` a mutable reference to the `Folder` to populate
fn populate_folder(folder: Folder) -> Folder {

    let mut folder = folder;

    for entry in fs::read_dir(&folder.path).unwrap() {
        let entry: DirEntry = entry.unwrap();
        let path: PathBuf = entry.path();

        if path.is_dir() {
            folder.subfolders.push(Folder::new(path));
        }

        else if path.is_file() {
            folder.subfiles.push(File::new(path));
        }
    }

    folder
}


/// Recursively grows out a Folder Tree, will grow to the depth specified by the
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
    todo!()
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
    todo!()
}


/// Traverses a Folder Tree and determines if it is complete. A Folder Tree is
/// defined as complete if:
/// * All empty folders have the `empty` field marked as true
pub fn file_tree_complete(folder: Folder) -> bool {
    todo!()
}

// // Function to build a folder tree from a starting folder.
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