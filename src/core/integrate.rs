#[derive(Clone, Debug)]
/// Stores a quadrature estimate, convergence error, and refinement level.
pub struct QuadResult {
    pub value: f64,
    pub error: f64,
    pub levels: usize,
}

/// Integrates a finite interval using a tanh-sinh double-exponential transform.
pub fn tanh_sinh<F>(function: &F, a: f64, b: f64, tol: f64, max_levels: usize) -> QuadResult
where
    F: Fn(f64) -> f64,
{
    if a == b {
        return QuadResult {
            value: 0.0,
            error: 0.0,
            levels: 0,
        };
    }

    let center = (a + b) / 2.0;
    let radius = (b - a) / 2.0;
    let mut previous = f64::NAN;

    for level in 3..=max_levels {
        let h = 0.5_f64.powi(level as i32);
        let limit = (6.0 / h).ceil() as i32;
        let mut sum = 0.0;

        for k in -limit..=limit {
            let t = k as f64 * h;
            let sinh_t = t.sinh();
            let u = std::f64::consts::FRAC_PI_2 * sinh_t;
            let x = u.tanh();
            let weight = std::f64::consts::FRAC_PI_2 * t.cosh() / u.cosh().powi(2);
            let mapped = center + radius * x;
            sum += function(mapped) * weight;
        }

        let value = radius * h * sum;
        if previous.is_finite() {
            let error = (value - previous).abs();
            if error <= tol {
                return QuadResult {
                    value,
                    error,
                    levels: level,
                };
            }
        }
        previous = value;
    }

    QuadResult {
        value: previous,
        error: f64::NAN,
        levels: max_levels,
    }
}

/// Integrates a semi-infinite interval using a tangent transform.
pub fn tan_sinh<F>(function: &F, a: f64, tol: f64, max_levels: usize) -> QuadResult
where
    F: Fn(f64) -> f64,
{
    let transformed = |theta: f64| {
        let x = a + theta.tan();
        function(x) / theta.cos().powi(2)
    };
    tanh_sinh(
        &transformed,
        0.0,
        std::f64::consts::FRAC_PI_2,
        tol,
        max_levels,
    )
}

/// Integrates the whole real line through a tangent change of variables.
pub fn quad_inf<F>(function: &F, tol: f64, max_levels: usize) -> QuadResult
where
    F: Fn(f64) -> f64,
{
    let transformed = |theta: f64| {
        let x = theta.tan();
        function(x) / theta.cos().powi(2)
    };
    tanh_sinh(
        &transformed,
        -std::f64::consts::FRAC_PI_2,
        std::f64::consts::FRAC_PI_2,
        tol,
        max_levels,
    )
}
