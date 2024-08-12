use sysinfo::{Pid, System};
use tauri::command;
use serde::{Serialize, Deserialize};

const MEMORY_LIMIT: u64 = 4096 * 1024 * 1024;

#[derive(Serialize, Deserialize)]
pub struct MemoryUsage {
    used_memory: u64,
    memory_limit: u64,
}

#[command]
pub fn get_memory_usage() -> MemoryUsage {
    let mut sys: System = System::new_all();
    sys.refresh_memory();

    // Get the current process ID
    let current_id: u32 = std::process::id();
    let current_pid: Pid = Pid::from_u32(current_id);

    // Find the current process information
    if let Some(process) = sys.process(current_pid) {
        let used_memory: u64 = process.memory() * 1024; // Memory in bytes
        let used_memory_mb: u64 = used_memory / (1024 * 1024); // Convert to MB

        MemoryUsage {
            used_memory: used_memory_mb,
            memory_limit: MEMORY_LIMIT / (1024 * 1024), // Convert limit to MB
        }
    } else {
        // Return 0 if the process information is not available
        MemoryUsage {
            used_memory: 0,
            memory_limit: MEMORY_LIMIT / (1024 * 1024), // Convert limit to MB
        }
    }
}