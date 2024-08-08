use std::fmt;

#[derive(Debug)]
pub enum DiskError {
    NotADisk,
}

impl fmt::Display for DiskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiskError::NotADisk => write!(f, "The provided folder is not a disk."),
        }
    }
}

impl std::error::Error for DiskError {}

#[derive(Debug)]
pub enum MergeError {
    FolderNotFound,

}

impl fmt::Display for MergeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MergeError::FolderNotFound => write!(f, "The provided Subfolder was not found in the main Folder"),
        }
    }
}

impl std::error::Error for MergeError {}