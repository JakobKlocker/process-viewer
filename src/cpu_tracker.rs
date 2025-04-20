use crate::process::ProcessInfo;
use num_cpus;
use std::collections::HashMap;
use std::time::Instant;

pub struct CpuTracker {
    pub last_total_ticks: u64,
    pub last_proc_ticks: HashMap<u32, u64>,
    pub last_check: Instant,
}

impl CpuTracker {
    pub fn new() -> Self {
        Self {
            last_total_ticks: read_total_system_ticks(),
            last_proc_ticks: HashMap::new(),
            last_check: Instant::now(),
        }
    }

    pub fn update_process_cpu(&mut self, processes: &mut Vec<ProcessInfo>) {
        let now = Instant::now();
        let delta_total_ticks = read_total_system_ticks() - self.last_total_ticks;
        let _elapsed = now.duration_since(self.last_check).as_secs_f64();
        let num_cpus = num_cpus::get() as f64;

        for proc in processes {
            let previous = self.last_proc_ticks.get(&proc.pid).copied().unwrap_or(0);
            let delta_proc = proc.cpu_time.saturating_sub(previous);

            let cpu_percent = if delta_total_ticks > 0 {
                (delta_proc as f64 / delta_total_ticks as f64) * 100.0 * num_cpus
            } else {
                0.0
            };

            proc.cpu_percent = cpu_percent;

            self.last_proc_ticks.insert(proc.pid, proc.cpu_time);
        }

        self.last_check = now;
        self.last_total_ticks += delta_total_ticks;
    }
}

fn read_total_system_ticks() -> u64 {
    let stat = std::fs::read_to_string("/proc/stat").unwrap();
    let line = stat.lines().next().unwrap();
    line.split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse::<u64>().ok())
        .sum()
}
