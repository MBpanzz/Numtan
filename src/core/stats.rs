use crate::core::linalg::solve;

#[derive(Clone, Debug)]
/// Holds descriptive statistics for a numeric sample.
pub struct Summary {
    pub count: usize,
    pub mean: f64,
    pub variance: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
}

#[derive(Clone, Debug)]
/// Holds slope, intercept, and coefficient of determination for a linear fit.
pub struct LinearRegression {
    pub slope: f64,
    pub intercept: f64,
    pub r2: f64,
}

/// Computes the arithmetic mean of a nonempty slice.
pub fn mean(values: &[f64]) -> Result<f64, String> {
    if values.is_empty() {
        return Err("values must not be empty".to_string());
    }
    Ok(values.iter().sum::<f64>() / values.len() as f64)
}

/// Computes population or sample variance for a numeric slice.
pub fn variance(values: &[f64], sample: bool) -> Result<f64, String> {
    if values.len() < if sample { 2 } else { 1 } {
        return Err("not enough values for variance".to_string());
    }
    let center = mean(values)?;
    let denominator = if sample {
        values.len() - 1
    } else {
        values.len()
    } as f64;
    Ok(values
        .iter()
        .map(|value| (value - center).powi(2))
        .sum::<f64>()
        / denominator)
}

/// Computes a compact descriptive-statistics summary.
pub fn summary(values: &[f64]) -> Result<Summary, String> {
    if values.is_empty() {
        return Err("values must not be empty".to_string());
    }
    let mean_value = mean(values)?;
    let variance_value = variance(values, false)?;
    Ok(Summary {
        count: values.len(),
        mean: mean_value,
        variance: variance_value,
        std_dev: variance_value.sqrt(),
        min: values.iter().copied().fold(f64::INFINITY, f64::min),
        max: values.iter().copied().fold(f64::NEG_INFINITY, f64::max),
    })
}

/// Computes population or sample covariance between two vectors.
pub fn covariance(left: &[f64], right: &[f64], sample: bool) -> Result<f64, String> {
    if left.len() != right.len() || left.is_empty() {
        return Err("vectors must have equal nonzero length".to_string());
    }
    if sample && left.len() < 2 {
        return Err("not enough values for sample covariance".to_string());
    }
    let left_mean = mean(left)?;
    let right_mean = mean(right)?;
    let denominator = if sample { left.len() - 1 } else { left.len() } as f64;
    Ok(left
        .iter()
        .zip(right)
        .map(|(a, b)| (a - left_mean) * (b - right_mean))
        .sum::<f64>()
        / denominator)
}

/// Computes Pearson correlation between two vectors.
pub fn correlation(left: &[f64], right: &[f64]) -> Result<f64, String> {
    Ok(covariance(left, right, false)?
        / (variance(left, false)?.sqrt() * variance(right, false)?.sqrt()))
}

/// Fits a least-squares line to paired observations.
pub fn linear_regression(x: &[f64], y: &[f64]) -> Result<LinearRegression, String> {
    if x.len() != y.len() || x.len() < 2 {
        return Err("x and y must have the same length of at least two".to_string());
    }
    let slope = covariance(x, y, false)? / variance(x, false)?;
    let intercept = mean(y)? - slope * mean(x)?;
    let y_mean = mean(y)?;
    let ss_total = y.iter().map(|value| (value - y_mean).powi(2)).sum::<f64>();
    let ss_error = x
        .iter()
        .zip(y)
        .map(|(x_value, y_value)| (y_value - (slope * x_value + intercept)).powi(2))
        .sum::<f64>();
    Ok(LinearRegression {
        slope,
        intercept,
        r2: 1.0 - ss_error / ss_total,
    })
}

/// Fits a least-squares polynomial and returns coefficients in ascending degree order.
pub fn polynomial_regression(x: &[f64], y: &[f64], degree: usize) -> Result<Vec<f64>, String> {
    if x.len() != y.len() || x.len() <= degree {
        return Err("not enough data for requested polynomial degree".to_string());
    }
    let size = degree + 1;
    let mut normal = vec![vec![0.0; size]; size];
    let mut rhs = vec![0.0; size];

    for row in 0..size {
        for col in 0..size {
            normal[row][col] = x.iter().map(|value| value.powi((row + col) as i32)).sum();
        }
        rhs[row] = x
            .iter()
            .zip(y)
            .map(|(x_value, y_value)| y_value * x_value.powi(row as i32))
            .sum();
    }

    solve(normal, rhs)
}
