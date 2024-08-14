use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use std::fs::{File, read_dir};
use std::io::Read;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Config {
    env_start_date: String,
    env_end_date: String,
    directory_to_watch: String,
}

fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}

fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0 * 1024.0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the config file
    let mut file = File::open("config.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON
    let config: Config = serde_json::from_str(&contents)?;

    // Parse the dates
    let start_date = NaiveDateTime::parse_from_str(&config.env_start_date, "%Y-%m-%dT%H:%M:%S")?;
    let end_date = NaiveDateTime::parse_from_str(&config.env_end_date, "%Y-%m-%dT%H:%M:%S")?;

    // Calculate the difference in days
    let duration = end_date.signed_duration_since(start_date);
    let days = duration.num_days();

    println!("Number of days between the dates: {}", days);

    // Count files and calculate total size
    let path = Path::new(&config.directory_to_watch);
    let mut file_count = 0;
    let mut total_size: u64 = 0;

    for entry in read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_file() {
            file_count += 1;
            total_size += metadata.len();
        }
    }

    println!("Number of files in the directory: {}", file_count);
    println!("Total size of files: {:.2} MB ({:.2} GB)", 
             bytes_to_mb(total_size), 
             bytes_to_gb(total_size));

    if file_count > 0 {
        let average_size = total_size as f64 / file_count as f64;
        let result_mb = bytes_to_mb(average_size as u64) * days as f64;
        let result_gb = bytes_to_gb(average_size as u64) * days as f64;
        println!("Average file size * Number of days: {:.2} MB-days ({:.2} GB-days)", 
                 result_mb, 
                 result_gb);
    } else {
        println!("No files found in the directory.");
    }

    Ok(())
}
