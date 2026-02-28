use std::{fs};

pub fn temp(all: bool) {

    match search_service() {

        Some(path) => {
            let sensors = read_temps(path);
            
            if all {
                println!("SENSORS");
                for (label, celsius) in &sensors {
                    println!("{}: {:.1}°C", label, celsius);
                }

            } else {

                if let Some((label, celsius)) = sensors.iter().find(|(l, _)| l == "Package id 0") {
                    println!("TEMPERATURE");
                    println!("{}: {:.1}°C", label, celsius);
                }
            }
        },
        None => eprintln!("Error: No compatible temperature sensor found"),
    }
}


fn search_service() -> Option<std::path::PathBuf> {

    let entries = fs::read_dir("/sys/class/hwmon/").unwrap();
    let drivers = ["coretemp", "cpu_thermal", "acpitz"];

    for entry in entries {

        let entry = entry.unwrap();
        let path = entry.path();
        let name_path = path.join("name");
        let name = fs::read_to_string(&name_path).unwrap_or_default();

        if drivers.contains(&name.trim()) {
            return Some(path);
        }
    }

   None
}

fn read_temps(path: std::path::PathBuf) -> Vec<(String, f64)> {

    let name = fs::read_to_string(path.join("name")).unwrap_or_default();

    // In the event that any sensor does not share the standard structure
    match name.trim() {
        "coretemp" | "cpu_thermal" | "acpitz" => read_generic(&path),
        _ => {
            eprintln!("Driver no soportado");
            Vec::new()
        }
    }
}


// It reads all relevant sensors and exposes them; this works on most sensor services.
fn read_generic(path: &std::path::PathBuf) -> Vec<(String, f64)> {

    let entries = fs::read_dir(path).unwrap();
    let mut sensors: Vec<(String, f64)> = Vec::new();

    for entry in entries {
        let entry = entry.unwrap();
        let filename = entry.file_name();
        let filename = filename.to_string_lossy();

        if filename.ends_with("_label") {
            let label = fs::read_to_string(entry.path()).unwrap_or_default();
            let input_path = path.join(filename.replace("_label", "_input"));
            let raw = fs::read_to_string(input_path).unwrap_or_default();
            let celsius = raw.trim().parse::<f64>().unwrap_or(0.0) / 1000.0;

            sensors.push((label.trim().to_string(), celsius));
        }
    }

    sensors
}