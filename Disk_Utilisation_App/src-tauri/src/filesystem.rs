use std::path::PathBuf;

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
        self.root_folder.update_size();
        self.update_folder_percentage(&mut self.root_folder, self.root_folder.size);
    }

    // Recursive method to calculate folder size
    fn calculate_folder_size(&self, folder: &Folder) -> u64 {
        let files_size: u64 = folder.subfiles.iter().map(|file| file.size).sum();
        let subfolders_size: u64 = folder
            .subfolders
            .iter()
            .map(|subfolder| self.calculate_folder_size(subfolder))
            .sum();
        files_size + subfolders_size
    }

    // Recursive method to update percentage of each folder and file relative to the root
    fn update_folder_percentage(&self, folder: &mut Folder, root_size: u64) {
        for file in &mut folder.subfiles {
            file.percentage_of_root = (file.size as f64 / root_size as f64) * 100.0;
        }
        for subfolder in &mut folder.subfolders {
            subfolder.percentage_of_root = (subfolder.size as f64 / root_size as f64) * 100.0;
            self.update_folder_percentage(subfolder, root_size);
        }
    }
}