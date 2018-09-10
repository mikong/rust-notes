extern crate csv;
extern crate walkdir;

use std::error::Error;
use std::path::{Path, PathBuf};
use csv::{Writer, Reader};
use walkdir::{DirEntry, WalkDir};

fn process_csvs(output: &str, paths: Vec<PathBuf>) -> Result<(), Box<Error>> {
    let mut writer = Writer::from_path(output)?;

    for path in paths {
        let mut reader = Reader::from_path(path)?;
        for row in reader.records() {
            if let Ok(r) = row {

                // Only get the filename from the path in the first column.
                let filename = Path::new(&r[0]).file_name().unwrap().to_string_lossy();
                let steering = r[1].to_string();

                writer.write_record(&[filename.into_owned(), steering])?;
            }
        }
        writer.flush()?;
    }

    Ok(())
}

fn is_csv(entry: &DirEntry) -> bool {
    match entry.path().extension() {
        Some(ext) => ext == "csv",
        None => false,
    }
}

fn main() {
    let mut csv_file_paths = vec![];

    let parent_dir = "/path/to/parent/dir/";
    let output_file = "/full/path/to/output.csv";

    let walker = WalkDir::new(parent_dir).into_iter();
    for entry in walker.filter_map(|e| e.ok()).filter(|e| is_csv(e)) {
        csv_file_paths.push(entry.path().to_path_buf());
    }

    process_csvs(output_file, csv_file_paths).unwrap();
}
