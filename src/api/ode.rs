use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::cell::RefCell;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(euler, module)?)?;
    module.add_function(wrap_pyfunction!(midpoint, module)?)?;
    module.add_function(wrap_pyfunction!(rk4, module)?)?;
    module.add_function(wrap_pyfunction!(adaptive_rk4, module)?)?;
    Ok(())
}

#[pyfunction]
fn euler(
    py: Python<'_>,
    function: Py<PyAny>,
    y0: f64,
    t0: f64,
    t1: f64,
    step: f64,
) -> PyResult<Py<PyAny>> {
    let error = RefCell::new(None);
    let closure =
        |t: f64, y: f64| -> f64 { crate::api::callback::scalar2(py, &function, &error, t, y) };
    let result = crate::core::ode::euler(&closure, y0, t0, t1, step);
    if let Some(err) = error.into_inner() {
        Err(err)
    } else {
        ode_points_to_list(py, result)
    }
}

#[pyfunction]
fn midpoint(
    py: Python<'_>,
    function: Py<PyAny>,
    y0: f64,
    t0: f64,
    t1: f64,
    step: f64,
) -> PyResult<Py<PyAny>> {
    let error = RefCell::new(None);
    let closure =
        |t: f64, y: f64| -> f64 { crate::api::callback::scalar2(py, &function, &error, t, y) };
    let result = crate::core::ode::midpoint(&closure, y0, t0, t1, step);
    if let Some(err) = error.into_inner() {
        Err(err)
    } else {
        ode_points_to_list(py, result)
    }
}

#[pyfunction]
fn rk4(
    py: Python<'_>,
    function: Py<PyAny>,
    y0: f64,
    t0: f64,
    t1: f64,
    step: f64,
) -> PyResult<Py<PyAny>> {
    let error = RefCell::new(None);
    let closure =
        |t: f64, y: f64| -> f64 { crate::api::callback::scalar2(py, &function, &error, t, y) };
    let result = crate::core::ode::rk4(&closure, y0, t0, t1, step);
    if let Some(err) = error.into_inner() {
        Err(err)
    } else {
        ode_points_to_list(py, result)
    }
}

#[pyfunction]
#[pyo3(signature = (function, y0, t0, t1, initial_step, tol = 1e-8))]
fn adaptive_rk4(
    py: Python<'_>,
    function: Py<PyAny>,
    y0: f64,
    t0: f64,
    t1: f64,
    initial_step: f64,
    tol: f64,
) -> PyResult<Py<PyAny>> {
    let error = RefCell::new(None);
    let closure =
        |t: f64, y: f64| -> f64 { crate::api::callback::scalar2(py, &function, &error, t, y) };
    let result = crate::core::ode::adaptive_rk4(&closure, y0, t0, t1, initial_step, tol);
    if let Some(err) = error.into_inner() {
        Err(err)
    } else {
        ode_points_to_list(py, result)
    }
}

fn ode_points_to_list(
    py: Python<'_>,
    points: Vec<crate::core::ode::OdePoint>,
) -> PyResult<Py<PyAny>> {
    let list = PyList::empty(py);
    for point in points {
        let item = PyDict::new(py);
        item.set_item("t", point.t)?;
        item.set_item("y", point.y)?;
        list.append(item)?;
    }
    Ok(list.into())
}
