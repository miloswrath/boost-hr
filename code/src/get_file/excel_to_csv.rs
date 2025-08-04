use std::path::Path;
use std::error::Error;
use calamine::{open_workbook_auto, Reader, DataType};
use csv::Writer;
use serde_json;

/// Read the Excel sheet, pick out BOOST ID and each zone’s two columns,
/// then write a CSV with columns:
/// [ "BOOST ID", "Zone 1", "Zone 2", …, "Zone 5" ]
/// where each Zone column is a JSON list like `[91,99]`.
pub fn excel_to_zone_csv(
    in_path: &Path,
    sheet_name: &str,
    out_path: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut wb = open_workbook_auto(in_path)?;
    let range = wb
        .worksheet_range(sheet_name)
        .ok_or("Sheet not found")??;

    let mut wtr = Writer::from_path(out_path)?;
    // write header
    wtr.write_record(&[
        "BOOST ID", "Zone 1", "Zone 2", "Zone 3", "Zone 4", "Zone 5"
    ])?;

    for row in range.rows().skip(1) {
        // BOOST ID is column 0
        let boost_id = match &row[0] {
            DataType::String(s) => s.clone(),
            DataType::Int(i)    => i.to_string(),
            DataType::Float(f)  => f.to_string(),
            _                   => continue, // skip if no valid ID
        };

        // Now gather each zone’s [low, high] from cols 5+2*i and 6+2*i
        let mut record = Vec::with_capacity(6);
        record.push(boost_id);
        for i in 0..5 {
            let low_cell  = &row[5 + i*2];
            let high_cell = &row[6 + i*2];
            // extract numeric as f64 (or panic if bad)
            let low  = match low_cell  { DataType::Int(i)    => *i as f64, DataType::Float(f) => *f, _ => 0.0 };
            let high = match high_cell { DataType::Int(i)    => *i as f64, DataType::Float(f) => *f, _ => 0.0 };
            // serialize to JSON list
            let list = serde_json::to_string(&vec![low, high])?;
            record.push(list);
        }

        wtr.write_record(&record)?;
    }

    wtr.flush()?;
    Ok(())
}
