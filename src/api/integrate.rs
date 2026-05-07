use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::cell::RefCell;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(tanh_sinh, module)?)?;
    module.add_function(wrap_pyfunction!(tan_sinh, module)?)?;
    module.add_function(wrap_pyfunction!(quad_inf, module)?)?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (function, a, b, tol = 1e-10, max_levels = 12))]
fn tanh_sinh(
    py: Python<'_>,
    function: Py<PyAny>,
    a: f64,
    b: f64,
    tol: f64,
    max_levels: usize,
) -> PyResult<Py<PyAny>> {
    let error = RefCell::new(None);
    let closure =
        |value: f64| -> f64 { crate::api::callback::scalar(py, &function, &error, value) };
    let result = crate::core::integrate::tanh_sinh(&closure, a, b, tol, max_levels);
    if let Some(err) = error.into_inner() {
        Err(err)
    } else {
        quad_result_to_dict(py, result)
    }
}

#[pyfunction]
#[pyo3(signature = (function, a = 0.0, tol = 1e-10, max_levels = 12))]
fn tan_sinh(
    py: Python<'_>,
    function: Py<PyAny>,
    a: f64,
    tol: f64,
    max_levels: usize,
) -> PyResult<Py<PyAny>> {
    let error = RefCell::new(None);
    let closure =
        |value: f64| -> f64 { crate::api::callback::scalar(py, &function, &error, value) };
    let result = crate::core::integrate::tan_sinh(&closure, a, tol, max_levels);
    if let Some(err) = error.into_inner() {
        Err(err)
    } else {
        quad_result_to_dict(py, result)
    }
}

#[pyfunction]
#[pyo3(signature = (function, tol = 1e-10, max_levels = 12))]
fn quad_inf(
    py: Python<'_>,
    function: Py<PyAny>,
    tol: f64,
    max_levels: usize,
) -> PyResult<Py<PyAny>> {
    let error = RefCell::new(None);
    let closure =
        |value: f64| -> f64 { crate::api::callback::scalar(py, &function, &error, value) };
    let result = crate::core::integrate::quad_inf(&closure, tol, max_levels);
    if let Some(err) = error.into_inner() {
        Err(err)
    } else {
        quad_result_to_dict(py, result)
    }
}

fn quad_result_to_dict(
    py: Python<'_>,
    result: crate::core::integrate::QuadResult,
) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(py);
    dict.set_item("value", result.value)?;
    dict.set_item("error", result.error)?;
    dict.set_item("levels", result.levels)?;
    Ok(dict.into())
}
