pub fn convert(value: String, from: String, to: String) {

    let clean_value = value
        .trim_start_matches("0x")
        .trim_start_matches("0X")
        .trim_start_matches("0o")
        .trim_start_matches("0O")
        .trim_start_matches("0b")
        .trim_start_matches("0B");

    let number: u64 = match parse_value(clean_value, &from) {
        Ok(n) => n,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    println!("{} -> {}", &from, &to);

    match to.as_str() {
        "dec" => println!("{}", number),
        "bin" => println!("{:b}", number),
        "hex" => println!("{:x}", number),
        "oct" => println!("{:o}", number),
        _ => println!("Error: base '{}' not soported", to),
    };
}

fn parse_value(value: &str, from: &str) -> Result<u64, String> {

    let base = match from {
        "dec" => 10,
        "bin" => 2,
        "hex" => 16,
        "oct" => 8,
        _ => return Err(format!("base '{}' not soported", from)),
    };

    u64::from_str_radix(value, base)
        .map_err(|_| format!("'{}' It is not a valid value based on {}", value, from))
}