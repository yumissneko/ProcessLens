#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,          // e.g. 15.4%
    pub memory_bytes: u64,       // Physical RAM (RSS) in bytes
    pub virtual_memory_bytes: u64,
    pub status: String,
    pub disk_read_bytes: u64,
    pub disk_written_bytes: u64,
}

impl ProcessInfo {
    /// Helper to format memory in human-readable units (MB, GB)
    pub fn memory_formatted(&self) -> String {
        let mb = self.memory_bytes as f64 / 1_048_576.0;
        if mb >= 1024.0 {
            format!("{:.2} GB", mb / 1024.0)
        } else {
            format!("{:.1} MB", mb)
        }
    }
}
