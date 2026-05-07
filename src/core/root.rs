use crate::core::derivative::{second_derivative, tangent};

#[derive(Clone, Debug)]
/// Records one tangent-based root-finding iteration.
pub struct IterStep {
    pub iteration: usize,
    pub x: f64,
    pub fx: f64,
    pub slope: f64,
    pub next_x: f64,
}

#[derive(Clone, Debug)]
/// Stores the final root estimate and full iteration history.
pub struct RootResult {
    pub root: f64,
    pub converged: bool,
    pub iterations: usize,
    pub history: Vec<IterStep>,
}

/// Solves a scalar equation with Newton's tangent method.
pub fn newton<F>(function: &F, x0: f64, tol: f64, max_iter: usize) -> Result<RootResult, String>
where
    F: Fn(f64) -> f64,
{
    let mut x = x0;
    let mut history = Vec::new();

    for iteration in 0..max_iter {
        let fx = function(x);
        let slope = tangent(function, x);
        if slope.abs() < 1e-14 {
            return Err("tangent slope is too small for Newton step".to_string());
        }

        let next_x = x - fx / slope;
        history.push(IterStep {
            iteration,
            x,
            fx,
            slope,
            next_x,
        });

        if (next_x - x).abs() <= tol || fx.abs() <= tol {
            return Ok(RootResult {
                root: next_x,
                converged: true,
                iterations: iteration + 1,
                history,
            });
        }

        x = next_x;
    }

    Ok(RootResult {
        root: x,
        converged: false,
        iterations: max_iter,
        history,
    })
}

/// Solves a scalar equation with Halley's third-order method.
pub fn halley<F>(function: &F, x0: f64, tol: f64, max_iter: usize) -> Result<RootResult, String>
where
    F: Fn(f64) -> f64,
{
    let mut x = x0;
    let mut history = Vec::new();

    for iteration in 0..max_iter {
        let fx = function(x);
        let first = tangent(function, x);
        let second = second_derivative(function, x);
        let denominator = 2.0 * first * first - fx * second;

        if denominator.abs() < 1e-14 {
            return Err("Halley denominator is too small".to_string());
        }

        let next_x = x - (2.0 * fx * first) / denominator;
        history.push(IterStep {
            iteration,
            x,
            fx,
            slope: first,
            next_x,
        });

        if (next_x - x).abs() <= tol || fx.abs() <= tol {
            return Ok(RootResult {
                root: next_x,
                converged: true,
                iterations: iteration + 1,
                history,
            });
        }

        x = next_x;
    }

    Ok(RootResult {
        root: x,
        converged: false,
        iterations: max_iter,
        history,
    })
}

/// Dispatches to supported Householder-family scalar root solvers.
pub fn householder<F>(
    function: &F,
    x0: f64,
    order: usize,
    tol: f64,
    max_iter: usize,
) -> Result<RootResult, String>
where
    F: Fn(f64) -> f64,
{
    match order {
        2 => newton(function, x0, tol, max_iter),
        3 => halley(function, x0, tol, max_iter),
        _ => Err("supported Householder orders are 2 and 3".to_string()),
    }
}
