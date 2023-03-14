use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use std::time::Duration;
use std::{env, thread};

#[derive(Deserialize)]
struct InputData {
    millis: u64,
}

#[derive(Serialize)]
struct OutputData {
    message: String,
}

fn main() {
    // ------ read arguments ------
    let args: Vec<String> = env::args().collect();
    let input_filename = args[1].parse::<String>().unwrap();

    // ------ load data from input file ------
    let inputs_dir = Path::new("input");
    let data_file = File::open(inputs_dir.join(input_filename)).unwrap();
    let data_file_reader = BufReader::new(data_file);
    let input_data: InputData = serde_json::from_reader(data_file_reader).unwrap();

    // ------ execute computation ------
    thread::sleep(Duration::from_millis(input_data.millis));

    // ------ output ------
    let now: DateTime<Local> = Local::now();
    let output_data = OutputData {
        message: format!("Completed the task at {}", now.format("%A, %B %e, %Y %r")),
    };
    let serialized_data = serde_json::to_string(&output_data).unwrap();
    let mut file = File::create("output/output.json").expect("Could not create output file");
    file.write_all(serialized_data.as_bytes())
        .expect("Could not write to output file");
}
