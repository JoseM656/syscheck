use std::{fs, thread, time::Duration};

pub fn cpu(ghz: bool, all: bool) {

    if all {
        println!("[syscheck - CPU - all]:");
        println!("Usage: {:.2}%", cpu_usage());
        println!("Freq: {:.2} GHz", cpu_freq_ghz());
        return;
    }

    if ghz {
        println!("[syscheck - CPU - GHz]: {:.2} GHz", cpu_freq_ghz());
    } else {
        println!("[syscheck - CPU]: {:.2}%", cpu_usage());
    }
}


fn read_cpu_times() -> Vec<u64> {
    let content = fs::read_to_string("/proc/stat").unwrap();
    let line = content.lines().next().unwrap();

    line.split_whitespace()
        .skip(1)
        .map(|v| v.parse::<u64>().unwrap())
        .collect()
}

/// CPU usage percentage
pub fn cpu_usage() -> f64 {
    let t1 = read_cpu_times();
    thread::sleep(Duration::from_millis(500));
    let t2 = read_cpu_times();

    let idle1 = t1[3];
    let idle2 = t2[3];

    let total1: u64 = t1.iter().sum();
    let total2: u64 = t2.iter().sum();

    let total_delta = total2 - total1;
    let idle_delta = idle2 - idle1;

    100.0 * (1.0 - idle_delta as f64 / total_delta as f64)
}


/// Processor frequency
pub fn cpu_freq_ghz() -> f64 {
    let freq = fs::read_to_string(
        "/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq"
    )
    .unwrap();

    let khz: f64 = freq.trim().parse().unwrap();

    khz / 1_000_000.0
}