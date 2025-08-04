use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use csv::ReaderBuilder;
use serde_json::from_str;

/// Given the CSV produced above, read each row into:
///   boost_id → Vec<[low, high]> for each zone
pub fn get_zones_from_csv(
    csv_path: &Path,
) -> Result<HashMap<String, Vec<[f64;2]>>, Box<dyn std::error::Error>> {
    let file = File::open(csv_path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut zones_map = HashMap::new();
    for result in rdr.records() {
        let record = result?;
        let boost_id = record.get(0)
            .ok_or("Missing BOOST ID")?
            .to_string();

        // Parse each zone column (1..=5) as JSON list
        let mut zone_vec = Vec::with_capacity(5);
        for idx in 1..=5 {
            let field = record.get(idx).ok_or("Missing zone field")?;
            let arr: Vec<f64> = from_str(field)?;
            if arr.len() == 2 {
                zone_vec.push([arr[0], arr[1]]);
            } else {
                return Err(format!("Zone {} list has wrong length", idx).into());
            }
        }

        zones_map.insert(boost_id, zone_vec);
    }

    Ok(zones_map)
}
