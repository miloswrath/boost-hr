//! src/preproc/mod.rs
//! “preproc” — heart‐rate pre‐processing (sup/unsup)use chrono::NaiveTime;
use serde::Serialize;
use serde_json::{json, Value};
use std::error::Error;

/// Shared return type for both preprocessors.
#[derive(Serialize)]
struct SupPreprocResult {
    total_time: f64,
    time_in_zones: Vec<f64>,
}

/// Parses a slice of `(HH:MM:SS, hr_str)` into `(seconds, hr)` pairs,
/// skipping parse errors.
fn parse_hr_data(
    hr_data: &[(String, String)],
) -> Vec<(f64, f64)> {
    hr_data.iter().filter_map(|(t_s, hr_s)| {
        // chrono::NaiveTime parses "HH:MM:SS"
        let t = NaiveTime::parse_from_str(t_s, "%H:%M:%S").ok()?;
        let secs = t.num_seconds_from_midnight() as f64;
        let hr = hr_s.parse::<f64>().ok()?;
        Some((secs, hr))
    }).collect()
}

/// sup_preproc:
/// 1. Parse HH:MM:SS → seconds  
/// 2. Drop first & last 5 min  
/// 3. Sum time in each zone  
pub fn sup_preproc(
    hr_data: &[(String, String)],
    zones: &[[f64; 2]],
) -> Result<Value, Box<dyn Error>> {
    let mut points = parse_hr_data(hr_data);
    if points.len() < 2 {
        return Err("Not enough data".into());
    }

    // sort by time
    points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // compute cutoffs
    let start_cut = points.first().unwrap().0 + 5.0 * 60.0;
    let end_cut   = points.last().unwrap().0  - 5.0 * 60.0;

    // trim
    let trimmed: Vec<_> = points
        .into_iter()
        .filter(|&(t, _)| t >= start_cut && t <= end_cut)
        .collect();

    if trimmed.len() < 2 {
        return Err("Trimmed away all data".into());
    }

    // accumulate per‐zone
    let mut zone_times = vec![0.0; zones.len()];
    for window in trimmed.windows(2) {
        let (t0, hr) = window[0];
        let t1 = window[1].0;
        let dt = t1 - t0;
        for (i, &[low, high]) in zones.iter().enumerate() {
            // half‐open [low, high), except last zone includes high
            if (low..high).contains(&hr) || (i == zones.len()-1 && hr == high) {
                zone_times[i] += dt;
                break;
            }
        }
    }

    let total_time: f64 = zone_times.iter().sum();
    Ok(json!(SupPreprocResult { total_time, time_in_zones: zone_times }))
}

/// unsup_preproc:
/// 1. Parse HH:MM:SS → seconds  
/// 2. Drop any points after 40 min  
/// 3. Then drop first & last 5 min of what remains  
/// 4. Sum time in each zone  
pub fn unsup_preproc(
    hr_data: &[(String, String)],
    zones: &[[f64; 2]],
) -> Result<Value, Box<dyn Error>> {
    // 1. parse
    let mut points = parse_hr_data(hr_data);
    if points.len() < 2 {
        return Err("Not enough data".into());
    }

    // 2. drop after 40 min (2400 s)
    let mut truncated: Vec<_> = points
        .into_iter()
        .filter(|&(t, _)| t <= 40.0 * 60.0)
        .collect();
    if truncated.len() < 2 {
        return Err("No data ≤ 40 min".into());
    }

    // 3. sort & compute new cutoffs
    truncated.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let start_cut = truncated.first().unwrap().0 + 5.0 * 60.0;
    let end_cut   = truncated.last().unwrap().0  - 5.0 * 60.0;

    // 4. trim first/last 5 min
    let trimmed: Vec<_> = truncated
        .into_iter()
        .filter(|&(t, _)| t >= start_cut && t <= end_cut)
        .collect();
    if trimmed.len() < 2 {
        return Err("Trimmed away all data".into());
    }

    // 5. accumulate
    let mut zone_times = vec![0.0; zones.len()];
    for window in trimmed.windows(2) {
        let (t0, hr) = window[0];
        let t1 = window[1].0;
        let dt = t1 - t0;
        for (i, &[low, high]) in zones.iter().enumerate() {
            if (low..high).contains(&hr) || (i == zones.len()-1 && hr == high) {
                zone_times[i] += dt;
                break;
            }
        }
    }

    let total_time: f64 = zone_times.iter().sum();
    Ok(json!(SupPreprocResult { total_time, time_in_zones: zone_times }))
}


