use std::fs;

pub fn info() {
    let (model, cores, threads) = cpu_info();

    println!("SYSTEM INFORMATION");
    println!("CPU: {}\nCores: {}\nThreads: {}", model, cores, threads);
}

fn cpu_info() -> (String, u64, u64) {
    let content = fs::read_to_string("/proc/cpuinfo").unwrap();

    let model_name = parse_value_str(&content, "model name");
    let cpu_cores = parse_value_u64(&content, "cpu cores");
    let siblings = parse_value_u64(&content, "siblings");

    (model_name, cpu_cores, siblings)
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
