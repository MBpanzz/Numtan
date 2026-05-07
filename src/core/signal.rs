#[derive(Clone, Debug)]
/// Represents a local maximum detected in a one-dimensional signal.
pub struct Peak {
    pub index: usize,
    pub value: f64,
}

/// Smooths a signal with a trailing moving average window.
pub fn moving_average(values: &[f64], window: usize) -> Result<Vec<f64>, String> {
    if window == 0 || values.is_empty() {
        return Err("window and values must be nonempty".to_string());
    }
    let mut result = Vec::with_capacity(values.len());
    let mut sum = 0.0;
    for index in 0..values.len() {
        sum += values[index];
        if index >= window {
            sum -= values[index - window];
        }
        let count = window.min(index + 1);
        result.push(sum / count as f64);
    }
    Ok(result)
}

/// Smooths a signal with first-order exponential smoothing.
pub fn exponential_smooth(values: &[f64], alpha: f64) -> Result<Vec<f64>, String> {
    if values.is_empty() || !(0.0..=1.0).contains(&alpha) {
        return Err("values must be nonempty and alpha must be within [0, 1]".to_string());
    }
    let mut result = Vec::with_capacity(values.len());
    let mut previous = values[0];
    result.push(previous);
    for value in &values[1..] {
        previous = alpha * value + (1.0 - alpha) * previous;
        result.push(previous);
    }
    Ok(result)
}

/// Computes the full discrete convolution of a signal and kernel.
pub fn convolve(signal: &[f64], kernel: &[f64]) -> Result<Vec<f64>, String> {
    if signal.is_empty() || kernel.is_empty() {
        return Err("signal and kernel must be nonempty".to_string());
    }
    let mut result = vec![0.0; signal.len() + kernel.len() - 1];
    for (i, signal_value) in signal.iter().enumerate() {
        for (j, kernel_value) in kernel.iter().enumerate() {
            result[i + j] += signal_value * kernel_value;
        }
    }
    Ok(result)
}

/// Scales values linearly into the range `[0, 1]`.
pub fn normalize(values: &[f64]) -> Result<Vec<f64>, String> {
    if values.is_empty() {
        return Err("values must be nonempty".to_string());
    }
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    if (max - min).abs() < f64::EPSILON {
        return Ok(vec![0.0; values.len()]);
    }
    Ok(values
        .iter()
        .map(|value| (value - min) / (max - min))
        .collect())
}

/// Finds strict local maxima above a threshold.
pub fn find_peaks(values: &[f64], threshold: f64) -> Vec<Peak> {
    if values.len() < 3 {
        return Vec::new();
    }
    let mut peaks = Vec::new();
    for index in 1..values.len() - 1 {
        if values[index] >= threshold
            && values[index] > values[index - 1]
            && values[index] > values[index + 1]
        {
            peaks.push(Peak {
                index,
                value: values[index],
            });
        }
    }
    peaks
}
