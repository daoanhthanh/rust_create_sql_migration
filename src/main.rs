use chrono::prelude::Local;
use std::env;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use convert_case::{Case, Casing};

#[warn(dead_code)]
fn get_file_serial(path: &str, prefix: &String) -> i32 {
    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(e) => panic!("Error reading directory: {}", e),
    };

    let mut count = 0;

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => panic!("Error reading directory entry: {}", e),
        };

        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap();
        if file_name.starts_with(prefix) {
            count += 1;
        }
    }

    return count + 1;
}

// fn get_file_name(args: & mut [String]) -> &String {

//     &args[1..].join("_")
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    let folder = "src/main/resources/db/migration/default";
    // let folder = ".";

    let current_date = Local::now().format("%Y.%m.%d.%H.%M.%S").to_string();
    // let file_prefix = format!("V{}", current_date);

    // let file_serial = format!("{:03}", get_file_serial(folder, &file_prefix));

    let filename = &args[1..].join("_");
    let file_path = format!("{}/V{}__{}.{}", folder, current_date, filename.to_case(Case::Snake), "sql");

    match File::create(&file_path) {
        Ok(_) => println!("Created migration file: {}", file_path),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("Folder '{}' not found", folder),
            _ => println!("Error creating file: {}", error),
        },
    }
}
