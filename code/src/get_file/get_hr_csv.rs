use std::fs::{self, File};
use std::path::{PathBuf};
use crate::config;
use csv::ReaderBuilder;

pub fn list_hr_files(project: &str, subject: &str, session: &str) -> Vec<PathBuf> {
    let base = config::get_base_path();
    let dir = PathBuf::from(base)
        .join(project);
        .join("3-Experiment");
        .join("data");
        .join("polarhrcsv");
        .join(session);
        .join(subject);

    let Ok(entries) = fs::read_dir(&dir) else {
        panic!("Could not read dir: {:?}", dir);
    };

    entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension()?.to_str()? == "csv" {
                Some(path)
            } else {
                None
            }
        })
        .collect()
}

/// Reads each CSV in `paths`, skips empty files, drops the header,
/// and for each remaining row extracts columns 0 and 1 as (time, hr) strings.
/// Returns a Vec of (PathBuf, Vec<(time, hr)>), omitting any files that
/// end up with zero data rows.
pub fn read_hr_files(paths: &[PathBuf]) -> Vec<(PathBuf, Vec<(String, String)>)> {
    paths
        .iter()
        .filter_map(|path| {
            // Try to open the file; skip on error
            let file = File::open(path).ok()?;
            // CSV reader: auto-detect comma delimiter, skip first row as header
            let mut rdr = ReaderBuilder::new()
                .has_headers(true)
                .from_reader(file);

            let mut rows = Vec::new();
            // Iterate over each record (line), handling parse errors gracefully
            for result in rdr.records() {
                if let Ok(record) = result {
                    // Only keep records with at least two columns
                    if record.len() >= 2 {
                        rows.push((
                            record[0].trim().to_string(),
                            record[1].trim().to_string(),
                        ));
                    }
                }
            }

            if rows.is_empty() {
                // If no data rows, skip this file
                None
            } else {
                // Otherwise return the path and its parsed rows
                Some((path.clone(), rows))
            }
        })
        .collect()
}

