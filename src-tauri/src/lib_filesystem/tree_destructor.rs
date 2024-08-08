use crate::lib_filesystem::filesystem::*;
use rayon::prelude::*;

/// Collapses a Folder tree to return a list of all Files in
/// the tree
///
/// # Arguments
/// * `folder` The Folder to collapse
///
/// # Returns
/// * `Vec<File>` An unordered list of all Files in the Folder. If the vector is
///     empty, there are no Files in the tree
pub fn get_all_files(folder: Folder) -> Vec<File> {
    let flattened_folder: Folder = flatten_folder(folder);
    flattened_folder.subfiles
}

/// Collapses a Folder tree to return a list of all Folders in the tree
///
/// # Arguments
/// * `folder` The Folder to collapse
///
/// # Returns
/// * `Vec<Folder>` An unordered list of all subfolders in the the Folder.
///     If the vector is empty, there are no Folders in the tree
pub fn get_all_folders(folder: Folder) -> Vec<Folder> {
    let flattened_folder: Folder = flatten_folder(folder);
    flattened_folder.subfolders
}


/// Collapses a Folder tree into a single Folder that contains a single list of
/// all sub-folders and files within that folder
///
/// # Arguments
/// * `folder` The Folder to collapse
///
/// # Returns
/// * A new Folder object
pub fn flatten_folder(folder: Folder) -> Folder {
    // Collect all folders and files, starting with the current folder
    let mut all_folders = vec![folder.clone()];
    let mut all_files = folder.subfiles.clone();

    // Process subfolders in parallel, collecting results into vectors
    let (sub_folders, sub_files): (Vec<_>, Vec<_>) = folder
        .subfolders
        .into_par_iter()
        .map(collect_all_parallel)
        .unzip();

    // Flatten the results
    for sublist in sub_folders {
        all_folders.extend(sublist);
    }

    for sublist in sub_files {
        all_files.extend(sublist);
    }

    Folder {
        name: folder.name,
        path: folder.path,
        size: folder.size,
        subfiles: all_files,
        subfolders: all_folders,
        disk: folder.disk,
        traversed: folder.traversed,
    }
}

fn collect_all_parallel(folder: Folder) -> (Vec<Folder>, Vec<File>) {
    let mut all_files = folder.subfiles.clone();
    let mut all_folders = vec![folder.clone()];

    // Process subfolders in parallel, collecting results into vectors
    let (sub_folders, sub_files): (Vec<_>, Vec<_>) = folder
        .subfolders
        .into_par_iter()
        .map(collect_all_parallel)
        .unzip();

    // Flatten the results
    for sublist in sub_folders {
        all_folders.extend(sublist);
    }

    for sublist in sub_files {
        all_files.extend(sublist);
    }

    (all_folders, all_files)
}


/// Gets all empty sub-Folders within a Folder. An empty Folder must have an
/// empty list of sub-Folders, and must have been traversed. If the given folder
/// has no subfolders, or hasn't been traversed, returns None instead.
///
/// # Arguments
///
/// * `folder` the Folder to be traversed
///
/// # Returns
///
/// * `Vec<Folder>` a list of all empty Folders
pub fn get_empty_folders(folder: Folder) -> Option<Vec<Folder>> {
    let mut empty_subfolders: Vec<Folder> = Vec::new();

    if !folder.traversed {
        return None;
    }

    else if folder.subfolders.len() == 0 && folder.traversed {
        return None;
    }

    else {
        // Recursively check subfolders
        for subfolder in &folder.subfolders {
            if let Some(mut subfolder_empty) = get_empty_folders(subfolder.clone()) {
                empty_subfolders.append(&mut subfolder_empty);
            }
        }
    }

    Some(empty_subfolders)
}

/// Marks the Current Folder, all sub-Folders, and all their sub-Folders ... as
/// not Traversed
///
/// # Arguments
///
/// * `folder` the Folder to be adjusted
///
/// # Returns
///
/// * The new Folder Object
pub fn mark_traversed(folder: Folder) -> Folder {
    // Create a new folder with the traversed flag set to true
    let updated_folder = Folder {
        traversed: true,
        // Keep other fields the same
        ..folder
    };

    // Recursively update subfolders
    let updated_subfolders: Vec<Folder> = updated_folder
        .subfolders
        .into_iter()
        .map(mark_traversed)
        .collect();

    // Return the updated folder with all subfolders updated
    Folder {
        subfolders: updated_subfolders,
        ..updated_folder
    }
}


/// Marks the Current Folder, all sub-Folders, and all their sub-Folders ... as
/// not Traversed
///
/// # Arguments
///
/// * `folder` the Folder to be adjusted
///
/// # Returns
///
/// * The new Folder Object
pub fn mark_not_traversed(folder: Folder) -> Folder {
    // Create a new folder with the traversed flag set to false
    let updated_folder = Folder {
        traversed: false,
        // Keep other fields the same
        ..folder
    };

    // Recursively update subfolders
    let updated_subfolders: Vec<Folder> = updated_folder
        .subfolders
        .into_iter()
        .map(mark_not_traversed)
        .collect();

    // Return the updated folder with all subfolders updated
    Folder {
        subfolders: updated_subfolders,
        ..updated_folder
    }
}