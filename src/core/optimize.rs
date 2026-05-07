use crate::core::derivative::{gradient, tangent};
use crate::core::linalg::{norm, solve};

#[derive(Clone, Debug)]
/// Records one optimization iteration with point, value, and local direction data.
pub struct OptimizeStep {
    pub iteration: usize,
    pub point: Vec<f64>,
    pub value: f64,
    pub gradient: Vec<f64>,
}

#[derive(Clone, Debug)]
/// Stores an optimization result and optional iteration history.
pub struct OptimizeResult {
    pub point: Vec<f64>,
    pub value: f64,
    pub converged: bool,
    pub iterations: usize,
    pub history: Vec<OptimizeStep>,
}

/// Minimizes a multivariate scalar function with fixed-step gradient descent.
pub fn gradient_descent<F>(
    function: &F,
    start: &[f64],
    step_size: f64,
    tol: f64,
    max_iter: usize,
) -> OptimizeResult
where
    F: Fn(&[f64]) -> f64,
{
    let mut point = start.to_vec();
    let mut history = Vec::new();

    for iteration in 0..max_iter {
        let value = function(&point);
        let grad = gradient(function, &point);
        let magnitude = norm(&grad);
        history.push(OptimizeStep {
            iteration,
            point: point.clone(),
            value,
            gradient: grad.clone(),
        });

        if magnitude <= tol {
            return OptimizeResult {
                point,
                value,
                converged: true,
                iterations: iteration + 1,
                history,
            };
        }

        for index in 0..point.len() {
            point[index] -= step_size * grad[index];
        }
    }

    let value = function(&point);
    OptimizeResult {
        point,
        value,
        converged: false,
        iterations: max_iter,
        history,
    }
}

/// Minimizes a scalar function with tangent-derived gradient descent.
pub fn tangent_minimize<F>(
    function: &F,
    start: f64,
    step_size: f64,
    tol: f64,
    max_iter: usize,
) -> OptimizeResult
where
    F: Fn(f64) -> f64,
{
    let vector_function = |point: &[f64]| function(point[0]);
    gradient_descent(&vector_function, &[start], step_size, tol, max_iter)
}

/// Solves a nonlinear least-squares problem with the Gauss-Newton method.
pub fn gauss_newton<F>(
    residuals: &F,
    start: &[f64],
    tol: f64,
    max_iter: usize,
) -> Result<OptimizeResult, String>
where
    F: Fn(&[f64]) -> Vec<f64>,
{
    let mut point = start.to_vec();
    let mut history = Vec::new();

    for iteration in 0..max_iter {
        let residual = residuals(&point);
        let value = residual.iter().map(|item| item * item).sum::<f64>() * 0.5;
        let jacobian = crate::core::derivative::jacobian(residuals, &point);
        let mut normal = vec![vec![0.0; point.len()]; point.len()];
        let mut rhs = vec![0.0; point.len()];

        for row in 0..residual.len() {
            for col in 0..point.len() {
                rhs[col] -= jacobian[row][col] * residual[row];
                for inner in 0..point.len() {
                    normal[col][inner] += jacobian[row][col] * jacobian[row][inner];
                }
            }
        }

        let delta = solve(normal, rhs)?;
        history.push(OptimizeStep {
            iteration,
            point: point.clone(),
            value,
            gradient: delta.clone(),
        });

        if norm(&delta) <= tol {
            return Ok(OptimizeResult {
                point,
                value,
                converged: true,
                iterations: iteration + 1,
                history,
            });
        }

        for index in 0..point.len() {
            point[index] += delta[index];
        }
    }

    let residual = residuals(&point);
    let value = residual.iter().map(|item| item * item).sum::<f64>() * 0.5;
    Ok(OptimizeResult {
        point,
        value,
        converged: false,
        iterations: max_iter,
        history,
    })
}

/// Finds a scalar stationary point by applying Newton's method to the derivative.
pub fn stationary_newton<F>(
    function: &F,
    start: f64,
    tol: f64,
    max_iter: usize,
) -> Result<OptimizeResult, String>
where
    F: Fn(f64) -> f64,
{
    let derivative = |x: f64| tangent(function, x);
    let result = crate::core::root::newton(&derivative, start, tol, max_iter)?;
    let point = vec![result.root];
    Ok(OptimizeResult {
        point,
        value: function(result.root),
        converged: result.converged,
        iterations: result.iterations,
        history: Vec::new(),
    })
}
