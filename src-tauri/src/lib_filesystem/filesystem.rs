use std::fmt;
use std::path::PathBuf;
use crate::lib_filesystem::constants::{KB, MB, GB, TB};
use crate::lib_filesystem::error::DiskError;


// Trait to represent common behaviors for Disk and Folder.
pub trait FileSystemEntity {
    fn name(&self) -> &String;
    fn path(&self) -> &PathBuf;
    fn size(&self) -> u64;
    fn percentage_of_root(&self, root_size: u64) -> f32 {
        (self.size() as f32) / (root_size as f32)
    }
}

// Root node representing the filesystem.
#[derive(Debug)]
pub struct Computer {
    pub name: String,
    pub disks: Vec<Folder>,
    pub total_size: u64, // Total size of all disks combined.
}

// Folder containing subfolders and files.
#[derive(Debug, Clone)]
pub struct Folder {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub subfiles: Vec<File>,
    pub subfolders: Vec<Folder>,
    pub disk: bool,
    pub empty: bool,
}

// File with name, extension, and size.
#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub extn: String,
    pub path: PathBuf,
    pub size: u64,
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

impl fmt::Display for Computer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Computer Name: {} | Total Size: {}\n", self.name, format_size(self.total_size))?;

        for disk in &self.disks {
            writeln!(f, "{}", disk)?;
        }

        Ok(())
    }
}

impl fmt::Display for Folder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Folder Name: {} | # Subfolders: {} | # Subfiles: {} | Size: {}",
            self.name,
            self.subfolders.len(),
            self.subfiles.len(),
            format_size(self.size),
        )
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "File Name: {} | Type: {} | Size: {}",
            self.name,
            self.extn,
            format_size(self.size),
        )
    }
}

impl Computer {
    pub fn new(name: String) -> Self {
        Computer {
            name: name,
            disks: Vec::new(),
            total_size: 0,
        }
    }

    pub fn add_disk(&mut self, disk: Folder) -> Result<(), DiskError> {
        if disk.disk {
            self.total_size += disk.size;
            self.disks.push(disk);
            Ok(())
        }
        else {
            Err(DiskError::NotADisk)
        }
    }
}

impl Folder {
    pub fn new(path: PathBuf) -> Self {
        Folder {
            name: Folder::name_from_path(&path),
            path,
            size: 0,
            subfiles: Vec::new(),
            subfolders: Vec::new(),
            disk: false,
            empty: false,
        }
    }

    pub fn new_disk(path: PathBuf) -> Self {
        let mut folder = Folder::new(path);
        folder.disk = true;
        folder
    }

    fn name_from_path(path: &PathBuf) -> String {
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        name
    }
}

impl File {
    pub fn new(path: PathBuf) -> Self {
        File {
            name: File::name_from_path(&path),
            extn: File::extn_from_path(&path),
            path,
            size: 0,
        }
    }

    fn name_from_path(path: &PathBuf) -> String {
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        name
    }

    fn extn_from_path(path: &PathBuf) -> String {
        let extn = path
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        extn
    }
}

pub fn format_size(size: u64) -> String {
    if size >= (TB + 10 * GB) {
        format!("{:.2} TB", size as f64 / TB as f64)
    } else if size >= (GB + 10 * MB) {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= (MB + 10 * KB) {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= (KB + 100) {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} bytes", size)
    }
}