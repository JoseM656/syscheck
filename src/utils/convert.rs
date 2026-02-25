pub fn convert(value: String, from: String, to: String) {

    
    let number: u64 = match from.as_str() {

        "dec" => u64::from_str_radix(&value, 10).unwrap(),
        "bin" => u64::from_str_radix(&value, 2).unwrap(),
        "hex" => u64::from_str_radix(&value, 16).unwrap(),
        "oct" => u64::from_str_radix(&value, 8).unwrap(),

        _ => panic!("base not supported")
    };


    println!("{} -> {}", &from, &to);
    match to.as_str() {

        "dec" => println!("{}", number),
        "bin" => println!("{:b}", number),
        "hex" => println!("{:x}", number),
        "oct" => println!("{:o}", number),

        _ => panic!("base not supported")
    };


}