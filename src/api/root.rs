use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::cell::RefCell;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(newton, module)?)?;
    module.add_function(wrap_pyfunction!(halley, module)?)?;
    module.add_function(wrap_pyfunction!(householder, module)?)?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (function, x0, tol = 1e-10, max_iter = 50))]
fn newton(
    py: Python<'_>,
    function: Py<PyAny>,
    x0: f64,
    tol: f64,
    max_iter: usize,
) -> PyResult<Py<PyAny>> {
    let error = RefCell::new(None);
    let closure =
        |value: f64| -> f64 { crate::api::callback::scalar(py, &function, &error, value) };
    let result = crate::core::root::newton(&closure, x0, tol, max_iter)
        .map_err(pyo3::exceptions::PyValueError::new_err)?;
    if let Some(err) = error.into_inner() {
        return Err(err);
    }
    root_result_to_dict(py, result)
}

#[pyfunction]
#[pyo3(signature = (function, x0, tol = 1e-10, max_iter = 50))]
fn halley(
    py: Python<'_>,
    function: Py<PyAny>,
    x0: f64,
    tol: f64,
    max_iter: usize,
) -> PyResult<Py<PyAny>> {
    let error = RefCell::new(None);
    let closure =
        |value: f64| -> f64 { crate::api::callback::scalar(py, &function, &error, value) };
    let result = crate::core::root::halley(&closure, x0, tol, max_iter)
        .map_err(pyo3::exceptions::PyValueError::new_err)?;
    if let Some(err) = error.into_inner() {
        return Err(err);
    }
    root_result_to_dict(py, result)
}

#[pyfunction]
#[pyo3(signature = (function, x0, order = 3, tol = 1e-10, max_iter = 50))]
fn householder(
    py: Python<'_>,
    function: Py<PyAny>,
    x0: f64,
    order: usize,
    tol: f64,
    max_iter: usize,
) -> PyResult<Py<PyAny>> {
    let error = RefCell::new(None);
    let closure =
        |value: f64| -> f64 { crate::api::callback::scalar(py, &function, &error, value) };
    let result = crate::core::root::householder(&closure, x0, order, tol, max_iter)
        .map_err(pyo3::exceptions::PyValueError::new_err)?;
    if let Some(err) = error.into_inner() {
        return Err(err);
    }
    root_result_to_dict(py, result)
}

fn root_result_to_dict(
    py: Python<'_>,
    result: crate::core::root::RootResult,
) -> PyResult<Py<PyAny>> {
    let dict = PyDict::new(py);
    dict.set_item("root", result.root)?;
    dict.set_item("converged", result.converged)?;
    dict.set_item("iterations", result.iterations)?;
    let history = PyList::empty(py);
    for step in result.history {
        let item = PyDict::new(py);
        item.set_item("iteration", step.iteration)?;
        item.set_item("x", step.x)?;
        item.set_item("fx", step.fx)?;
        item.set_item("slope", step.slope)?;
        item.set_item("next_x", step.next_x)?;
        history.append(item)?;
    }
    dict.set_item("history", history)?;
    Ok(dict.into())
}
