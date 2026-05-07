use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::cell::RefCell;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(gradient_descent, module)?)?;
    module.add_function(wrap_pyfunction!(tangent_minimize, module)?)?;
    module.add_function(wrap_pyfunction!(stationary_newton, module)?)?;
    module.add_function(wrap_pyfunction!(gauss_newton, module)?)?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (function, start, step_size = 0.01, tol = 1e-8, max_iter = 1000))]
fn gradient_descent(
    py: Python<'_>,
    function: Py<PyAny>,
    start: Vec<f64>,
    step_size: f64,
    tol: f64,
    max_iter: usize,
) -> PyResult<Py<PyAny>> {
    let error = RefCell::new(None);
    let closure = |point: &[f64]| -> f64 {
        crate::api::callback::vector_scalar(py, &function, &error, point)
    };
    let result =
        crate::core::optimize::gradient_descent(&closure, &start, step_size, tol, max_iter);
    if let Some(err) = error.into_inner() {
        Err(err)
    } else {
        optimize_result_to_dict(py, result)
    }
}

#[pyfunction]
#[pyo3(signature = (function, start, step_size = 0.01, tol = 1e-8, max_iter = 1000))]
fn tangent_minimize(
    py: Python<'_>,
    function: Py<PyAny>,
    start: f64,
    step_size: f64,
    tol: f64,
    max_iter: usize,
) -> PyResult<Py<PyAny>> {
    let error = RefCell::new(None);
    let closure =
        |value: f64| -> f64 { crate::api::callback::scalar(py, &function, &error, value) };
    let result = crate::core::optimize::tangent_minimize(&closure, start, step_size, tol, max_iter);
    if let Some(err) = error.into_inner() {
        Err(err)
    } else {
        optimize_result_to_dict(py, result)
    }
}

#[pyfunction]
#[pyo3(signature = (function, start, tol = 1e-8, max_iter = 100))]
fn stationary_newton(
    py: Python<'_>,
    function: Py<PyAny>,
    start: f64,
    tol: f64,
    max_iter: usize,
) -> PyResult<Py<PyAny>> {
    let error = RefCell::new(None);
    let closure =
        |value: f64| -> f64 { crate::api::callback::scalar(py, &function, &error, value) };
    let result = crate::core::optimize::stationary_newton(&closure, start, tol, max_iter)
        .map_err(pyo3::exceptions::PyValueError::new_err)?;
    if let Some(err) = error.into_inner() {
        Err(err)
    } else {
        optimize_result_to_dict(py, result)
    }
}

#[pyfunction]
#[pyo3(signature = (function, start, tol = 1e-8, max_iter = 100))]
fn gauss_newton(
    py: Python<'_>,
    function: Py<PyAny>,
    start: Vec<f64>,
    tol: f64,
    max_iter: usize,
) -> PyResult<Py<PyAny>> {
    let error = RefCell::new(None);
    let closure = |point: &[f64]| -> Vec<f64> {
        crate::api::callback::vector_vector(py, &function, &error, point)
    };
    let result = crate::core::optimize::gauss_newton(&closure, &start, tol, max_iter)
        .map_err(pyo3::exceptions::PyValueError::new_err)?;
    if let Some(err) = error.into_inner() {
        Err(err)
    } else {
        optimize_result_to_dict(py, result)
    }
}

fn optimize_result_to_dict(
    py: Python<'_>,
    result: crate::core::optimize::OptimizeResult,
) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(py);
    dict.set_item("point", result.point)?;
    dict.set_item("value", result.value)?;
    dict.set_item("converged", result.converged)?;
    dict.set_item("iterations", result.iterations)?;
    let history = PyList::empty(py);
    for step in result.history {
        let item = PyDict::new(py);
        item.set_item("iteration", step.iteration)?;
        item.set_item("point", step.point)?;
        item.set_item("value", step.value)?;
        item.set_item("gradient", step.gradient)?;
        history.append(item)?;
    }
    dict.set_item("history", history)?;
    Ok(dict.into())
}
