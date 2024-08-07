use std::fmt;
use std::path::{Path, PathBuf};
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