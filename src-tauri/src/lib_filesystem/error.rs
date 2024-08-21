use std::fmt;

#[derive(Debug)]
pub enum FileSystemError {
    NotADisk,
    FolderNotFound,
}

impl fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileSystemError::NotADisk => write!(f, "The provided folder is not a disk."),
            FileSystemError::FolderNotFound => write!(f, "The provided Subfolder was not found in the main Folder")
        }
    }
}