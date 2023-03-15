use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, BufReader, Write};
use std::path::PathBuf;

#[derive(Deserialize)]
struct InputData {
    input_id: u64,
}

#[derive(Serialize)]
struct OutputData {
    message: String,
}

fn list_files_in_dir(root: &str) -> io::Result<Vec<PathBuf>> {
    let mut result = vec![];

    for path in fs::read_dir(root)? {
        result.push(path?.path().to_owned());
    }

    Ok(result)
}

fn main() {
    // ------ load data from input file ------
    let files = list_files_in_dir("input").expect("Could not get files");
    let data_file = File::open(files.get(0).expect("No input files provided")).unwrap();
    let data_file_reader = BufReader::new(data_file);
    let input_data: InputData = serde_json::from_reader(data_file_reader).unwrap();

    // ------ output ------
    let now: DateTime<Local> = Local::now();
    let output_data = OutputData {
        message: format!(
            "Completed task #{} at {}",
            input_data.input_id,
            now.format("%A, %B %e, %Y %r")
        ),
    };
    let serialized_data = serde_json::to_string(&output_data).unwrap();
    let mut file = File::create("output/output.json").expect("Could not create output file");
    file.write_all(serialized_data.as_bytes())
        .expect("Could not write to output file");
}
