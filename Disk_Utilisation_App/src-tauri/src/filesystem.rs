use std::path::PathBuf;
use std::fmt;


// Struct to represent a file
#[derive(Debug)]
pub struct File {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub percentage_of_root: f64,
}

impl File {
    // Method to create a new File
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name: name,
            path: path,
            size: 0,
            percentage_of_root: 0.0,
        }
    }
}

// Implementing Display for File
impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "File: {}\nPath: {:?}\nSize: {} bytes\nPercentage of Root: {:.2}%",
            self.name, self.path, self.size, self.percentage_of_root
        )
    }
}

// Struct to represent a folder
#[derive(Debug)]
pub struct Folder {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub percentage_of_root: f64,
    pub subfiles_count: usize,
    pub subfolders_count: usize,
    pub subfiles: Vec<File>,
    pub subfolders: Vec<Folder>,
}

impl Folder {
    // Method to create a new Folder
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            size: 0,
            percentage_of_root: 0.0,
            subfiles_count: 0,
            subfolders_count: 0,
            subfiles: Vec::new(),
            subfolders: Vec::new(),
        }
    }

    // Method to add a file to the folder
    pub fn add_file(&mut self, file: File) {
        self.size += file.size;
        self.subfiles.push(file);
        self.subfiles_count += 1;
    }

    // Method to add a subfolder to the folder
    pub fn add_folder(&mut self, folder: Folder) {
        self.size += folder.size;
        self.subfolders.push(folder);
        self.subfolders_count += 1;
    }

    // Method to update the folder's size based on its contents
    pub fn update_size(&mut self) {
        self.size = self.subfiles.iter().map(|file| file.size).sum::<u64>()
            + self.subfolders.iter().map(|folder| folder.size).sum::<u64>();
    }

    // Method to print the folder hierarchy
    pub fn print_structure(&self, indent: usize) {
        println!("{}", self.to_string_with_indent(indent));
        for subfolder in &self.subfolders {
            subfolder.print_structure(indent + 2);
        }
    }

    // Helper method to format folder information with indentation
    fn to_string_with_indent(&self, indent: usize) -> String {
        format!(
            "{}Folder: {}\n{}Path: {:?}\n{}Size: {} bytes\n{}Percentage of Root: {:.2}%",
            " ".repeat(indent),
            self.name,
            " ".repeat(indent),
            self.path,
            " ".repeat(indent),
            self.size,
            " ".repeat(indent),
            self.percentage_of_root
        )
    }
}

// Implementing Display for Folder
impl fmt::Display for Folder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string_with_indent(0))
    }
}

// Struct to represent the entire disk (root)
#[derive(Debug)]
pub struct Disk {
    pub root_folder: Folder,
}

impl Disk {
    // Method to create a new Disk with a root folder
    pub fn new(root_name: String, root_path: PathBuf) -> Self {
        Self {
            root_folder: Folder::new(root_name, root_path),
        }
    }

    // Method to update the size and percentage for the entire disk
    pub fn update_disk_size(&mut self) {
        
    }

    // Recursive method to calculate folder size
    fn calculate_folder_size(&self, folder: &Folder) -> u64 {
        return 0;
    }

    // Recursive method to update percentage of each folder and file relative to the root
    fn update_folder_percentage(&self, folder: &mut Folder, root_size: u64) {

    }
}