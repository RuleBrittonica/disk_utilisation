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