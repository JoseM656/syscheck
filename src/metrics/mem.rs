use std::fs;

pub fn mem(cache: bool, swap: bool, all: bool) {

    let content = fs::read_to_string("/proc/meminfo").unwrap();

    if all {

        let mem_total     = parse_meminfo_value(&content, "MemTotal:");
        let mem_available = parse_meminfo_value(&content, "MemAvailable:");
        let swap_total    = parse_meminfo_value(&content, "SwapTotal:");
        let swap_free     = parse_meminfo_value(&content, "SwapFree:");
        let cached        = parse_meminfo_value(&content, "Cached:");

        println!("MEMORY");
        format_mem("RAM Usage  ",  mem_total - mem_available);
        format_mem("Total RAM  ",  mem_total);
        format_mem("SWAP Usage ", swap_total - swap_free);
        format_mem("Cache      ",     cached);

        return;
    }

    
    if swap {
            
        let swap_total    = parse_meminfo_value(&content, "SwapTotal:");
        let swap_free     = parse_meminfo_value(&content, "SwapFree:");

        println!("SWAP");
        format_mem("Usage  ", swap_total - swap_free);
        
        return;
    }


    if cache {

        let cached = parse_meminfo_value(&content, "Cached:");

        println!("CACHE");
        format_mem("Usage  ",cached);

    } else {

        let mem_total     = parse_meminfo_value(&content, "MemTotal:");
        let mem_available = parse_meminfo_value(&content, "MemAvailable:");

        println!("RAM");
        format_mem("Usage     ",  mem_total - mem_available);
        format_mem("Total     ",  mem_total);
    }

}


fn format_mem(label: &str, kb: u64) {
    println!("{}: {} MB ({} GB)", label, kb / 1024, kb / 1_048_576);
}

// Function to facilitate memory parsing
fn parse_meminfo_value(content: &str, key: &str) -> u64 {

    content
        .lines()                                    
        .find(|line| line.starts_with(key))         
        .unwrap()
        .split_whitespace()                         
        .nth(1)                                    
        .unwrap()
        .parse::<u64>()                             
        .unwrap()
}