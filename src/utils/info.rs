use std::fs;
use std::process::Command;

pub fn info() {
    let (so_name, kernel_name, host_name) = so_info();
    let (model, cores, threads) = cpu_info();
    let gpu = gpu_info();

    // Print block
    println!("SYSTEM INFORMATION");

    println!("OS:     {}", so_name);
    println!("Kernel: {}", kernel_name);
    println!("Host:   {}", host_name);

    println!();

    println!(); // For make a space on terminal

    println!("CPU     : {}", model);
    println!("Cores   : {}", cores);
    println!("Threads : {}", threads);

    println!();

    println!("GPU : {}", gpu);
}

// ---------------------
// Recolection data part
// ---------------------

fn so_info() -> (String, String, String) {
    let content = fs::read_to_string("/etc/os-release").unwrap();
    let so_name = parse_value_eq(&content, "PRETTY_NAME");

    let kernel_name = fs::read_to_string("/proc/version").unwrap();
    let host_name = fs::read_to_string("/proc/sys/kernel/hostname").unwrap();

    (so_name, kernel_name, host_name)
}

fn cpu_info() -> (String, u64, u64) {
    let content = fs::read_to_string("/proc/cpuinfo").unwrap();

    let model_name = parse_value_str(&content, "model name");
    let cpu_cores = parse_value_u64(&content, "cpu cores");
    let siblings = parse_value_u64(&content, "siblings");

    (model_name, cpu_cores, siblings)
}

fn gpu_info() -> String {
    let output = Command::new("lspci").arg("-mm").output().unwrap();

    let content = String::from_utf8_lossy(&output.stdout).to_string();

    let line = content
        .lines()
        .find(|line| line.contains("VGA") || line.contains("Display"))
        .unwrap_or("");

    let parts: Vec<&str> = line.split('"').collect();

    if parts.len() >= 6 {
        format!("{} {}", parts[3], parts[5])
    } else {
        "GPU not found".to_string()
    }
}

// Functions to facilitate parsing
fn parse_value_u64(content: &str, key: &str) -> u64 {
    content
        .lines()
        .find(|line| line.starts_with(key))
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .parse::<u64>()
        .unwrap()
}

fn parse_value_str(content: &str, key: &str) -> String {
    content
        .lines()
        .find(|line| line.starts_with(key))
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .to_string()
}

fn parse_value_eq(content: &str, key: &str) -> String {
    content
        .lines()
        .find(|line| line.starts_with(key))
        .unwrap()
        .split('=')
        .nth(1)
        .unwrap()
        .trim_matches('"')
        .to_string()
}
