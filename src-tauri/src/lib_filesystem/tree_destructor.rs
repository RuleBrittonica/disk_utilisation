use core::fmt;

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
        empty: folder.empty,
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