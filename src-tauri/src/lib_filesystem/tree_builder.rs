use crate::lib_filesystem::filesystem::*;
use crate::lib_filesystem::error::*;
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

/// Function to populate a folder with Subfolders and files
///
/// # Arguments
///
/// * `folder` a mutable reference to the `Folder` to populate
fn populate_folder(folder: Folder) -> Folder {

    let mut folder: Folder = folder;

    folder.traversed = true;

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

/// Grows out a folder tree by a specified number of levels.
///
/// # Arguments
///
/// * `folder` - The folder to grow out
/// * `levels` - The number of levels to grow out the folder tree by
///
/// # Returns
///
/// The new folder object
pub fn grow_folder_tree(folder: Folder, levels: u32) -> Folder {
    match levels {
        0 => folder, // Base case: if no levels remain, return the current folder
        _ => {
            let unfolded_folder = unfold_folder(folder);
            grow_folder_tree(unfolded_folder, levels - 1)
        }
    }
}


/// Unfolds a folder tree by a single level.
///
/// # Arguments
///
/// * `folder` - The folder to continue growing
///
/// # Returns
///
/// The new folder object
///
/// # Examples:
///
/// Say we have a folder tree defined as follows:
/// src
/// |- ABC
///    |- ABC_1
///       |- ABC_1_1
///       |- ABC_1_2
///       |- 123.txt
///    |- ABC_2
///       |- ABC_2_1
///       |- ABC_2_2
///       |- 456.txt
///    |- 789.txt
/// |- DEF
///    |- ** Defined Similarly **
/// |- xyz.txt
///
/// And an initial folder object containing:
/// src
/// |- ABC
/// |- DEF
/// |- xyz.txt
///
/// Then Calling unfold_folder once on this folder will return a folder containing:
/// src
/// |- ABC
///    |- ABC_1
///    |- ABC_2
///    |- 789.txt
/// |- DEF
///    |- ...
/// |- xyz.txt
///
/// Calling unfold_folder(unfold_folder(folder)) will return the whole tree from
/// the original folder
fn unfold_folder(mut folder: Folder) -> Folder {
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
/// Otherwise Throws a FolderNotFound error.
pub fn merge_folder_tree(
    main_folder: Folder,
    sub_folder: Folder,
    overwrite: bool
    ) -> Result<Folder, FileSystemError> {

    // Check if the current folder is the target folder to merge into
    if main_folder.path == sub_folder.path {
        if overwrite {
            main_folder.subfolders = sub_folder.subfolders;
            main_folder.subfiles = sub_folder.subfiles;
        } else {
            // Merge subfolders
            for sub in sub_folder.subfolders {
                match main_folder.subfolders.iter_mut().find(|f| f.path == sub.path) {
                    Some(existing_subfolder) => {
                        *existing_subfolder = Folder::merge_folder_tree(
                            existing_subfolder.clone(),
                            sub,
                            overwrite,
                        )?;
                    }
                    None => main_folder.subfolders.push(sub),
                }
            }
            // Merge subfiles
            for sub_file in sub_folder.subfiles {
                if overwrite {
                    main_folder
                        .subfiles
                        .retain(|f| f.path != sub_file.path);
                }
                if !main_folder.subfiles.iter().any(|f| f.path == sub_file.path) {
                    main_folder.subfiles.push(sub_file);
                }
            }
        }
        return Ok(main_folder);
    }

    // Recursively search in subfolders
    for subfolder in &mut main_folder.subfolders {
        if let Ok(merged_folder) =
            Folder::merge_folder_tree(subfolder.clone(), sub_folder.clone(), overwrite)
        {
            *subfolder = merged_folder;
            return Ok(main_folder);
        }
    }

    // If the subfolder is not found in the main tree, return an error
    Err(FolderError::FolderNotFound)
}


/// Traverses a Folder Tree and determines if it is complete. A Folder Tree is
/// defined as complete if:
/// * All folders in the tree, and all their subfolders, and all their
///     subfolders ... have been marked as Traversed.
///
/// # Arguments
///
/// * `folder` the Folder to be checked
///
/// # Returns
///
/// * true if the Folder has been fully traversed
pub fn file_tree_complete(folder: &Folder) -> bool {
    // Check if the current folder has been traversed
    if !folder.traversed {
        return false;
    }

    // Recursively check all subfolders
    for subfolder in &folder.subfolders {
        if !file_tree_complete(subfolder) {
            return false;
        }
    }

    // If all checks pass, the tree is complete
    true
}