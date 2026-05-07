use crate::core::root::newton;

/// Evaluates a polynomial with coefficients ordered by ascending degree.
pub fn evaluate(coefficients: &[f64], x: f64) -> f64 {
    coefficients
        .iter()
        .rev()
        .fold(0.0, |acc, coefficient| acc * x + coefficient)
}

/// Computes coefficients of the formal polynomial derivative.
pub fn derivative(coefficients: &[f64]) -> Vec<f64> {
    coefficients
        .iter()
        .enumerate()
        .skip(1)
        .map(|(index, coefficient)| *coefficient * index as f64)
        .collect()
}

/// Computes coefficients of the formal polynomial antiderivative.
pub fn integral(coefficients: &[f64], constant: f64) -> Vec<f64> {
    let mut result = Vec::with_capacity(coefficients.len() + 1);
    result.push(constant);
    for (index, coefficient) in coefficients.iter().enumerate() {
        result.push(*coefficient / (index + 1) as f64);
    }
    result
}

/// Adds two polynomials represented by ascending-degree coefficients.
pub fn add(left: &[f64], right: &[f64]) -> Vec<f64> {
    let size = left.len().max(right.len());
    (0..size)
        .map(|index| {
            left.get(index).copied().unwrap_or(0.0) + right.get(index).copied().unwrap_or(0.0)
        })
        .collect()
}

/// Multiplies two polynomials represented by ascending-degree coefficients.
pub fn multiply(left: &[f64], right: &[f64]) -> Vec<f64> {
    if left.is_empty() || right.is_empty() {
        return Vec::new();
    }
    let mut result = vec![0.0; left.len() + right.len() - 1];
    for (i, left_value) in left.iter().enumerate() {
        for (j, right_value) in right.iter().enumerate() {
            result[i + j] += left_value * right_value;
        }
    }
    result
}

/// Finds a real polynomial root near an initial guess using Newton's method.
pub fn root(coefficients: &[f64], guess: f64, tol: f64, max_iter: usize) -> Result<f64, String> {
    let function = |x: f64| evaluate(coefficients, x);
    Ok(newton(&function, guess, tol, max_iter)?.root)
}
