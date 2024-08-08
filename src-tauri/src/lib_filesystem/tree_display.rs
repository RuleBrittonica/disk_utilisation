use crate::lib_filesystem::filesystem::Folder;
use crate::lib_filesystem::filesystem::format_size;

pub fn print_folder_tree(folder: &Folder, indent: usize) {
    let indent_str = " ".repeat(indent);
    // Display the folder's own details
    println!(
        "{}Folder: {} | # Subfolders: {} | # Subfiles: {} | Size: {}",
        indent_str,
        folder.name,
        folder.subfolders.len(),
        folder.subfiles.len(),
        format_size(folder.size)
    );

    // Display files in the folder
    for file in &folder.subfiles {
        println!(
            "{}|- File: {} | Type: {} | Size: {}",
            indent_str,
            file.name,
            file.extn,
            format_size(file.size)
        );
    }

    // Recursively display subfolders
    for subfolder in &folder.subfolders {
        println!(
            "{}|- Folder: {} | # Subfolders: {} | # Subfiles: {} | Size: {}",
            indent_str,
            subfolder.name,
            subfolder.subfolders.len(),
            subfolder.subfiles.len(),
            format_size(subfolder.size)
        );
        // Call the recursive function to print further subfolder details
        print_folder_tree(subfolder, indent + 2);
    }
}

