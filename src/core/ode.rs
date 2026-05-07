#[derive(Clone, Debug)]
/// Represents one sampled point on a scalar ODE trajectory.
pub struct OdePoint {
    pub t: f64,
    pub y: f64,
}

/// Integrates a scalar ODE with the explicit Euler method.
pub fn euler<F>(function: &F, y0: f64, t0: f64, t1: f64, step: f64) -> Vec<OdePoint>
where
    F: Fn(f64, f64) -> f64,
{
    march(function, y0, t0, t1, step, euler_step)
}

/// Integrates a scalar ODE with the explicit midpoint method.
pub fn midpoint<F>(function: &F, y0: f64, t0: f64, t1: f64, step: f64) -> Vec<OdePoint>
where
    F: Fn(f64, f64) -> f64,
{
    march(function, y0, t0, t1, step, midpoint_step)
}

/// Integrates a scalar ODE with the classical fourth-order Runge-Kutta method.
pub fn rk4<F>(function: &F, y0: f64, t0: f64, t1: f64, step: f64) -> Vec<OdePoint>
where
    F: Fn(f64, f64) -> f64,
{
    march(function, y0, t0, t1, step, rk4_step)
}

/// Integrates a scalar ODE with step-doubling adaptive RK4 control.
pub fn adaptive_rk4<F>(
    function: &F,
    y0: f64,
    t0: f64,
    t1: f64,
    initial_step: f64,
    tol: f64,
) -> Vec<OdePoint>
where
    F: Fn(f64, f64) -> f64,
{
    let mut t = t0;
    let mut y = y0;
    let mut step = initial_step.abs().max(1e-12) * (t1 - t0).signum();
    let mut points = vec![OdePoint { t, y }];

    while (t1 - t) * step.signum() > 1e-14 {
        if (t + step - t1) * step.signum() > 0.0 {
            step = t1 - t;
        }

        let full = rk4_step(function, t, y, step);
        let half = rk4_step(function, t, y, step / 2.0);
        let refined = rk4_step(function, t + step / 2.0, half, step / 2.0);
        let error = (refined - full).abs();

        if error <= tol || step.abs() <= 1e-12 {
            t += step;
            y = refined;
            points.push(OdePoint { t, y });
        }

        let factor = if error == 0.0 {
            2.0
        } else {
            (tol / error).powf(0.2).clamp(0.2, 2.0)
        };
        step *= 0.9 * factor;
    }

    points
}

fn march<F, S>(function: &F, y0: f64, t0: f64, t1: f64, step: f64, stepper: S) -> Vec<OdePoint>
where
    F: Fn(f64, f64) -> f64,
    S: Fn(&F, f64, f64, f64) -> f64,
{
    let mut t = t0;
    let mut y = y0;
    let signed_step = step.abs().max(1e-12) * (t1 - t0).signum();
    let mut points = vec![OdePoint { t, y }];

    while (t1 - t) * signed_step.signum() > 1e-14 {
        let current_step = if (t + signed_step - t1) * signed_step.signum() > 0.0 {
            t1 - t
        } else {
            signed_step
        };
        y = stepper(function, t, y, current_step);
        t += current_step;
        points.push(OdePoint { t, y });
    }

    points
}

fn euler_step<F>(function: &F, t: f64, y: f64, step: f64) -> f64
where
    F: Fn(f64, f64) -> f64,
{
    y + step * function(t, y)
}

fn midpoint_step<F>(function: &F, t: f64, y: f64, step: f64) -> f64
where
    F: Fn(f64, f64) -> f64,
{
    let k1 = function(t, y);
    y + step * function(t + step / 2.0, y + step * k1 / 2.0)
}

fn rk4_step<F>(function: &F, t: f64, y: f64, step: f64) -> f64
where
    F: Fn(f64, f64) -> f64,
{
    let k1 = function(t, y);
    let k2 = function(t + step / 2.0, y + step * k1 / 2.0);
    let k3 = function(t + step / 2.0, y + step * k2 / 2.0);
    let k4 = function(t + step, y + step * k3);
    y + step * (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0
}
