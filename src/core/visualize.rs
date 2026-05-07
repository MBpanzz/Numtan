use crate::core::derivative::tangent;
use crate::core::root::newton;

#[derive(Clone, Debug)]
/// Describes a line segment suitable for plotting tangent or iteration geometry.
pub struct LineSegment {
    pub x_start: f64,
    pub y_start: f64,
    pub x_end: f64,
    pub y_end: f64,
    pub center: f64,
    pub slope: f64,
}

#[derive(Clone, Debug)]
/// Describes one normalized vector in a direction field.
pub struct DirectionSample {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
}

/// Generates tangent line segments over a shared plotting range.
pub fn tangent_lines<F>(function: &F, x_start: f64, x_end: f64, centers: &[f64]) -> Vec<LineSegment>
where
    F: Fn(f64) -> f64,
{
    centers
        .iter()
        .map(|center| {
            let y = function(*center);
            let slope = tangent(function, *center);
            LineSegment {
                x_start,
                y_start: y + slope * (x_start - center),
                x_end,
                y_end: y + slope * (x_end - center),
                center: *center,
                slope,
            }
        })
        .collect()
}

/// Converts Newton iterations into tangent-line animation segments.
pub fn newton_animation<F>(
    function: &F,
    x0: f64,
    tol: f64,
    max_iter: usize,
) -> Result<Vec<LineSegment>, String>
where
    F: Fn(f64) -> f64,
{
    let result = newton(function, x0, tol, max_iter)?;
    Ok(result
        .history
        .iter()
        .map(|step| LineSegment {
            x_start: step.x,
            y_start: step.fx,
            x_end: step.next_x,
            y_end: 0.0,
            center: step.x,
            slope: step.slope,
        })
        .collect())
}

/// Samples a scalar ODE slope function into normalized direction-field vectors.
pub fn direction_field<F>(
    function: &F,
    x_range: (f64, f64),
    y_range: (f64, f64),
    x_count: usize,
    y_count: usize,
) -> Vec<DirectionSample>
where
    F: Fn(f64, f64) -> f64,
{
    let mut samples = Vec::with_capacity(x_count.saturating_mul(y_count));
    if x_count == 0 || y_count == 0 {
        return samples;
    }

    for xi in 0..x_count {
        let x = linspace_value(x_range.0, x_range.1, xi, x_count);
        for yi in 0..y_count {
            let y = linspace_value(y_range.0, y_range.1, yi, y_count);
            let slope = function(x, y);
            let length = (1.0 + slope * slope).sqrt();
            samples.push(DirectionSample {
                x,
                y,
                dx: 1.0 / length,
                dy: slope / length,
            });
        }
    }

    samples
}

fn linspace_value(start: f64, end: f64, index: usize, count: usize) -> f64 {
    if count == 1 {
        start
    } else {
        start + (end - start) * index as f64 / (count - 1) as f64
    }
}
