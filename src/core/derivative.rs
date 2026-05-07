/// Estimates the tangent slope of a scalar function at `x` using a central finite difference.
pub fn tangent<F>(function: &F, x: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    central_difference(function, x, 1e-6)
}

/// Computes a first derivative estimate with caller-provided spacing.
pub fn central_difference<F>(function: &F, x: f64, h: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    (function(x + h) - function(x - h)) / (2.0 * h)
}

/// Estimates the second derivative of a scalar function at `x`.
pub fn second_derivative<F>(function: &F, x: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let h = 1e-4;
    (function(x + h) - 2.0 * function(x) + function(x - h)) / (h * h)
}

/// Estimates the gradient of a multivariate scalar function.
pub fn gradient<F>(function: &F, point: &[f64]) -> Vec<f64>
where
    F: Fn(&[f64]) -> f64,
{
    let h = 1e-6;
    let mut result = Vec::with_capacity(point.len());

    for index in 0..point.len() {
        let mut forward = point.to_vec();
        let mut backward = point.to_vec();
        forward[index] += h;
        backward[index] -= h;
        result.push((function(&forward) - function(&backward)) / (2.0 * h));
    }

    result
}

/// Estimates the Jacobian matrix of a multivariate vector function.
pub fn jacobian<F>(function: &F, point: &[f64]) -> Vec<Vec<f64>>
where
    F: Fn(&[f64]) -> Vec<f64>,
{
    let h = 1e-6;
    let base = function(point);
    let mut matrix = vec![vec![0.0; point.len()]; base.len()];

    for column in 0..point.len() {
        let mut forward = point.to_vec();
        let mut backward = point.to_vec();
        forward[column] += h;
        backward[column] -= h;
        let forward_value = function(&forward);
        let backward_value = function(&backward);

        for row in 0..base.len() {
            matrix[row][column] = (forward_value[row] - backward_value[row]) / (2.0 * h);
        }
    }

    matrix
}
