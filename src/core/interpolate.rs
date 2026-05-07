#[derive(Clone, Debug)]
/// Stores one sampled function value on a one-dimensional grid.
pub struct GridSample {
    pub x: f64,
    pub y: f64,
}

/// Generates evenly spaced values between two endpoints.
pub fn linspace(start: f64, end: f64, count: usize) -> Vec<f64> {
    match count {
        0 => Vec::new(),
        1 => vec![start],
        _ => (0..count)
            .map(|index| start + (end - start) * index as f64 / (count - 1) as f64)
            .collect(),
    }
}

/// Samples a scalar function on an evenly spaced grid.
pub fn sample_grid<F>(function: &F, start: f64, end: f64, count: usize) -> Vec<GridSample>
where
    F: Fn(f64) -> f64,
{
    linspace(start, end, count)
        .into_iter()
        .map(|x| GridSample { x, y: function(x) })
        .collect()
}

/// Performs piecewise-linear interpolation for a sorted grid.
pub fn linear_interpolate(x: &[f64], y: &[f64], query: f64) -> Result<f64, String> {
    validate_xy(x, y)?;
    if query < x[0] || query > x[x.len() - 1] {
        return Err("query is outside interpolation range".to_string());
    }
    for index in 0..x.len() - 1 {
        if query >= x[index] && query <= x[index + 1] {
            let ratio = (query - x[index]) / (x[index + 1] - x[index]);
            return Ok(y[index] * (1.0 - ratio) + y[index + 1] * ratio);
        }
    }
    Ok(y[y.len() - 1])
}

/// Evaluates the global Lagrange interpolating polynomial.
pub fn lagrange_interpolate(x: &[f64], y: &[f64], query: f64) -> Result<f64, String> {
    validate_xy(x, y)?;
    let mut result = 0.0;
    for i in 0..x.len() {
        let mut basis = 1.0;
        for j in 0..x.len() {
            if i != j {
                basis *= (query - x[j]) / (x[i] - x[j]);
            }
        }
        result += y[i] * basis;
    }
    Ok(result)
}

/// Estimates first derivatives on an equally spaced grid.
pub fn finite_difference(values: &[f64], spacing: f64) -> Result<Vec<f64>, String> {
    if values.len() < 2 || spacing == 0.0 {
        return Err("need at least two values and nonzero spacing".to_string());
    }
    let mut result = vec![0.0; values.len()];
    result[0] = (values[1] - values[0]) / spacing;
    for index in 1..values.len() - 1 {
        result[index] = (values[index + 1] - values[index - 1]) / (2.0 * spacing);
    }
    let last = values.len() - 1;
    result[last] = (values[last] - values[last - 1]) / spacing;
    Ok(result)
}

fn validate_xy(x: &[f64], y: &[f64]) -> Result<(), String> {
    if x.len() != y.len() || x.len() < 2 {
        return Err("x and y must have matching length of at least two".to_string());
    }
    for index in 1..x.len() {
        if x[index] <= x[index - 1] {
            return Err("x values must be strictly increasing".to_string());
        }
    }
    Ok(())
}
