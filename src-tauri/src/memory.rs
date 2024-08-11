use sysinfo::System;
use tauri::command;
use serde::{Serialize, Deserialize};

const MEMORY_LIMIT: u64 = 4096 * 1024 * 1024;

#[derive(Serialize, Deserialize)]
pub struct MemoryUsage {
    max_memory: u64,
    used_memory: u64,
    memory_limit: u64,
}

#[command]
pub fn get_memory_usage() -> MemoryUsage {
    let mut sys = System::new_all();
    sys.refresh_memory(); // Refresh memory information

    let max_memory = sys.total_memory() / 1024; // Convert to MB
    let used_memory = sys.used_memory() / 1024; // Convert to MB
    let memory_limit = MEMORY_LIMIT;

    MemoryUsage {
        max_memory,
        used_memory,
        memory_limit,
    }
}