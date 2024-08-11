use sysinfo::Disks;
use sysinfo::DiskKind::{HDD, SSD};
use tauri::command;
use serde::{Serialize, Deserialize};
// use sysinfo::{DiskExt, System, SystemExt};

#[derive(Serialize, Deserialize)]
pub struct DriveInfo {
    name: String,
    kind: String,
    total_space: u64,
    available_space: u64,
    icon: String,
    file_system: String,
    removable: bool,
}

#[command]
pub fn get_drives() -> Vec<DriveInfo> {
    let mut drive_info_list: Vec<DriveInfo> = Vec::new();
    let disks: Disks = Disks::new_with_refreshed_list();

    // Iterate through the disks and collect information
    for disk in &disks {
        let total_space: u64 = disk.total_space();
        let available_space: u64 = disk.available_space();
        let name: String = disk.name().to_string_lossy().to_string();
        let kind: String = disk.kind().to_string();
        let file_system: String = disk.file_system().to_string_lossy().to_string();
        let removable: bool = disk.is_removable();
        // Determine the appropriate icon path based on the disk kind
        let icon: String = match disk.kind() {
            HDD => "../icons/drives/hdd.png",
            SSD => "../icons/drives/ssd.png",
            _   => "../icons/drives/default_disk.png",
        }.to_string();

        // Create a DriveInfo struct and add it to the list
        let drive_info: DriveInfo = DriveInfo {
            name,
            kind,
            total_space,
            available_space,
            icon,
            file_system,
            removable,
        };
        drive_info_list.push(drive_info);
    }

    // Return the vector of DriveInfo structs
    drive_info_list
}