use super::process::ProcessInfo;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct SystemSnapshot {
    pub uptime_secs: u64,
    pub global_cpu_usage: f32,       // 0.0 to 100.0%
    pub core_cpu_usages: Vec<f32>,   // Per-core usage %
    pub total_memory: u64,           // Total RAM in bytes
    pub used_memory: u64,            // Used RAM in bytes
    pub total_swap: u64,             // Total Swap in bytes
    pub used_swap: u64,              // Used Swap in bytes
    pub processes: Vec<ProcessInfo>,
}

/// Fixed-capacity ring buffer for time-series sparklines
#[derive(Debug, Clone)]
pub struct HistoryBuffer {
    capacity: usize,
    data: VecDeque<u64>,
}

impl HistoryBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: VecDeque::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, val: u64) {
        if self.data.len() >= self.capacity {
            self.data.pop_front();
        }
        self.data.push_back(val);
    }

    pub fn as_slice(&self) -> &[u64] {
        self.data.make_contiguous()
    }
}
