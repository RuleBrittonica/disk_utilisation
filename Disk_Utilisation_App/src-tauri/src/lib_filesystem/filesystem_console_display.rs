use crate::lib_filesystem::filesystem::Folder;

pub fn print_folder_tree(folder: &Folder, indent: usize) {
    let indent_str = " ".repeat(indent);
    // Display the folder's own details
    println!("{}Folder: {} | # Subfolders: {} | # Subfiles: {} | Size: {} bytes",
        indent_str,
        folder.name,
        folder.subfolders.len(),
        folder.subfiles.len(),
        folder.size
    );

    // Display files in the folder
    for file in &folder.subfiles {
        println!("{}|- File: {} | Type: {} | Size: {} bytes",
            indent_str,
            file.name,
            file.extn,
            file.size
        );
    }

    // Recursively display subfolders
    for subfolder in &folder.subfolders {
        println!("{}|- {}", indent_str, subfolder.name);
        print_folder_tree(subfolder, indent + 2);
    }
}