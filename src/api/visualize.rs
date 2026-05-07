use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::cell::RefCell;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(tangent_lines, module)?)?;
    module.add_function(wrap_pyfunction!(newton_animation_data, module)?)?;
    module.add_function(wrap_pyfunction!(ode_direction_field, module)?)?;
    Ok(())
}

#[pyfunction]
fn tangent_lines(
    py: Python<'_>,
    function: Py<PyAny>,
    x_start: f64,
    x_end: f64,
    centers: Vec<f64>,
) -> PyResult<Py<PyAny>> {
    let error = RefCell::new(None);
    let closure =
        |value: f64| -> f64 { crate::api::callback::scalar(py, &function, &error, value) };
    let result = crate::core::visualize::tangent_lines(&closure, x_start, x_end, &centers);
    if let Some(err) = error.into_inner() {
        Err(err)
    } else {
        lines_to_list(py, result)
    }
}

#[pyfunction]
#[pyo3(signature = (function, x0, tol = 1e-10, max_iter = 50))]
fn newton_animation_data(
    py: Python<'_>,
    function: Py<PyAny>,
    x0: f64,
    tol: f64,
    max_iter: usize,
) -> PyResult<Py<PyAny>> {
    let error = RefCell::new(None);
    let closure =
        |value: f64| -> f64 { crate::api::callback::scalar(py, &function, &error, value) };
    let lines = crate::core::visualize::newton_animation(&closure, x0, tol, max_iter)
        .map_err(pyo3::exceptions::PyValueError::new_err)?;
    if let Some(err) = error.into_inner() {
        return Err(err);
    }
    lines_to_list(py, lines)
}

#[pyfunction]
fn ode_direction_field(
    py: Python<'_>,
    function: Py<PyAny>,
    x_range: (f64, f64),
    y_range: (f64, f64),
    x_count: usize,
    y_count: usize,
) -> PyResult<Py<PyAny>> {
    let error = RefCell::new(None);
    let closure =
        |x: f64, y: f64| -> f64 { crate::api::callback::scalar2(py, &function, &error, x, y) };
    let samples =
        crate::core::visualize::direction_field(&closure, x_range, y_range, x_count, y_count);
    if let Some(err) = error.into_inner() {
        return Err(err);
    }
    let list = PyList::empty(py);
    for sample in samples {
        let item = PyDict::new(py);
        item.set_item("x", sample.x)?;
        item.set_item("y", sample.y)?;
        item.set_item("dx", sample.dx)?;
        item.set_item("dy", sample.dy)?;
        list.append(item)?;
    }
    Ok(list.into())
}

fn lines_to_list(
    py: Python<'_>,
    lines: Vec<crate::core::visualize::LineSegment>,
) -> PyResult<Py<PyAny>> {
    let list = PyList::empty(py);
    for line in lines {
        let item = PyDict::new(py);
        item.set_item("x_start", line.x_start)?;
        item.set_item("y_start", line.y_start)?;
        item.set_item("x_end", line.x_end)?;
        item.set_item("y_end", line.y_end)?;
        item.set_item("center", line.center)?;
        item.set_item("slope", line.slope)?;
        list.append(item)?;
    }
    Ok(list.into())
}
