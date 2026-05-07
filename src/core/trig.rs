use num_complex::Complex64;

/// Computes `tan(pi * x)` after reducing the input period.
pub fn tanpi(x: f64) -> f64 {
    let reduced = x.rem_euclid(1.0);
    let angle = std::f64::consts::PI * reduced;
    angle.sin() / angle.cos()
}

/// Computes tangent for an angle measured in degrees.
pub fn tan_deg(x: f64) -> f64 {
    (x.to_radians()).tan()
}

/// Computes tangent for an angle measured in gradians.
pub fn tan_grad(x: f64) -> f64 {
    (x * std::f64::consts::PI / 200.0).tan()
}

/// Computes the tangent of a complex number represented by real and imaginary parts.
pub fn complex_tan(re: f64, im: f64) -> (f64, f64) {
    let value = Complex64::new(re, im).tan();
    (value.re, value.im)
}

/// Evaluates the inverse-tangent integral with adaptive Simpson quadrature.
pub fn atanint(x: f64) -> f64 {
    if x == 0.0 {
        return 0.0;
    }

    let sign = x.signum();
    let upper = x.abs();
    sign * adaptive_simpson(
        &|t| if t == 0.0 { 1.0 } else { t.atan() / t },
        0.0,
        upper,
        1e-10,
        20,
    )
}

/// Evaluates the tangent integral with adaptive Simpson quadrature.
pub fn tanint(x: f64) -> f64 {
    if x == 0.0 {
        return 0.0;
    }

    let sign = x.signum();
    let upper = x.abs();
    sign * adaptive_simpson(
        &|t| if t == 0.0 { 1.0 } else { t.tan() / t },
        0.0,
        upper,
        1e-10,
        20,
    )
}

fn adaptive_simpson<F>(function: &F, a: f64, b: f64, tol: f64, depth: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let c = (a + b) / 2.0;
    let whole = simpson(function, a, b);
    recurse(function, a, b, c, whole, tol, depth)
}

fn recurse<F>(function: &F, a: f64, b: f64, c: f64, whole: f64, tol: f64, depth: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    let left_mid = (a + c) / 2.0;
    let right_mid = (c + b) / 2.0;
    let left = simpson(function, a, c);
    let right = simpson(function, c, b);
    let delta = left + right - whole;

    if depth == 0 || delta.abs() <= 15.0 * tol {
        return left + right + delta / 15.0;
    }

    recurse(function, a, c, left_mid, left, tol / 2.0, depth - 1)
        + recurse(function, c, b, right_mid, right, tol / 2.0, depth - 1)
}

fn simpson<F>(function: &F, a: f64, b: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let c = (a + b) / 2.0;
    (b - a) * (function(a) + 4.0 * function(c) + function(b)) / 6.0
}
