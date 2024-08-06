use std::fmt;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// Root node representing the filesystem.
#[derive(Debug)]
pub struct Root {
    pub disks: Vec<Disk>,
    pub total_size: u64, // Total size of all disks combined.
}

// Trait to represent common behaviors for Disk and Folder.
pub trait FileSystemEntity {
    fn name(&self) -> &String;
    fn path(&self) -> &PathBuf;
    fn size(&self) -> u64;
    fn percentage_of_root(&self, root_size: u64) -> f32 {
        (self.size() as f32) / (root_size as f32)
    }
}

// Disk containing folders and files.
#[derive(Debug)]
pub struct Disk {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub subfiles: Vec<File>,
    pub subfolders: Vec<Folder>,
}

// Folder containing subfolders and files.
#[derive(Debug)]
pub struct Folder {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub subfiles: Vec<File>,
    pub subfolders: Vec<Folder>,
}


// File with name, extension, and size.
#[derive(Debug)]
pub struct File {
    pub name: String,
    pub extn: String,
    pub path: PathBuf,
    pub size: u64,
}

impl FileSystemEntity for Disk {
    fn name(&self) -> &String {
        &self.name
    }

    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn size(&self) -> u64 {
        self.size
    }
}

impl FileSystemEntity for Folder {
    fn name(&self) -> &String {
        &self.name
    }

    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn size(&self) -> u64 {
        self.size
    }
}

impl FileSystemEntity for File {
    fn name(&self) -> &String {
        &self.name
    }

    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn size(&self) -> u64 {
        self.size
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Disk Name: {} | # Subfolders: {} | # Subfiles: {} | Size: {} bytes",
            self.name,
            self.subfolders.len(),
            self.subfiles.len(),
            self.size,
        )
    }
}

impl fmt::Display for Folder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Fodler Name: {} | # Subfolders: {} | # Subfiles: {} | Size: {} bytes",
            self.name,
            self.subfolders.len(),
            self.subfiles.len(),
            self.size,
        )
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "File Name: {} | Type: {} | Size: {} bytes",
            self.name,
            self.extn,
            self.size
        )
    }
}

impl Root {
    pub fn new() -> Self {
        Root {
            disks: Vec::new(),
            total_size: 0,
        }
    }

    pub fn add_disk(&mut self, disk: Disk) {
        self.total_size += disk.size;
        self.disks.push(disk);
    }
}

impl Disk {
    pub fn new(name: String, path: PathBuf) -> Self {
        Disk {
            name,
            path,
            size: 0,
            subfiles: Vec::new(),
            subfolders: Vec::new(),
        }
    }
}

impl Folder {
    pub fn new(name: String, path: PathBuf) -> Self {
        Folder {
            name,
            path,
            size: 0,
            subfiles: Vec::new(),
            subfolders: Vec::new(),
        }
    }
}

impl File {
    pub fn new(name: String, extn: String, path: PathBuf) -> Self {
        File {
            name,
            extn,
            path,
            size: 0,
        }
    }
}

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

fn main() {
    let start_path = Path::new("../tests/test_dir_1");
    let folder_tree = build_folder_tree(start_path);

    // Print the constructed folder tree for debugging purposes
    println!("{:#?}", folder_tree);
}