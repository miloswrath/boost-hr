use pyo3::prelude::*;

/// Apply a sliding-window median filter to `data`.
///
/// # Arguments
/// * `data`   - vector of heart-rate samples (bpm)
/// * `window` - odd-sized window length (e.g., 5, 7, 9)
///
/// # Returns
/// A Vec<f64> of the same length; at each index, returns the median of
/// the points in the sliding window. Edges shrink the window automatically.
#[pyfunction]
fn median_filter(data: Vec<f64>, window: usize) -> PyResult<Vec<f64>> {
    let n = data.len();
    if window == 0 || n == 0 {
        return Ok(Vec::new());
    }
    assert!(window % 2 == 1, "window size must be odd");

    let half = window / 2;
    let mut out = Vec::with_capacity(n);
    let mut buf = Vec::with_capacity(window);

    for i in 0..n {
        // Determine slice bounds
        let start = if i < half { 0 } else { i - half };
        let end = (i + half).min(n - 1);

        // Copy window into buffer and sort
        buf.clear();
        buf.extend_from_slice(&data[start..=end]);
        buf.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        // Pick median
        let mid = buf.len() / 2;
        out.push(buf[mid]);
    }

    Ok(out)
}

#[pymodule]
fn hr_smooth(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(median_filter, m)?)?;
    Ok(())
}

