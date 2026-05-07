use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::cell::RefCell;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(linspace, module)?)?;
    module.add_function(wrap_pyfunction!(sample_grid, module)?)?;
    module.add_function(wrap_pyfunction!(linear_interpolate, module)?)?;
    module.add_function(wrap_pyfunction!(lagrange_interpolate, module)?)?;
    module.add_function(wrap_pyfunction!(finite_difference, module)?)?;
    Ok(())
}

#[pyfunction]
fn linspace(start: f64, end: f64, count: usize) -> Vec<f64> {
    crate::core::interpolate::linspace(start, end, count)
}

#[pyfunction]
fn sample_grid(
    py: Python<'_>,
    function: Py<PyAny>,
    start: f64,
    end: f64,
    count: usize,
) -> PyResult<Py<PyAny>> {
    let error = RefCell::new(None);
    let closure =
        |value: f64| -> f64 { crate::api::callback::scalar(py, &function, &error, value) };
    let samples = crate::core::interpolate::sample_grid(&closure, start, end, count);
    if let Some(err) = error.into_inner() {
        return Err(err);
    }
    let list = PyList::empty(py);
    for sample in samples {
        let item = PyDict::new(py);
        item.set_item("x", sample.x)?;
        item.set_item("y", sample.y)?;
        list.append(item)?;
    }
    Ok(list.into())
}

#[pyfunction]
fn linear_interpolate(x: Vec<f64>, y: Vec<f64>, query: f64) -> PyResult<f64> {
    crate::core::interpolate::linear_interpolate(&x, &y, query)
        .map_err(pyo3::exceptions::PyValueError::new_err)
}

#[pyfunction]
fn lagrange_interpolate(x: Vec<f64>, y: Vec<f64>, query: f64) -> PyResult<f64> {
    crate::core::interpolate::lagrange_interpolate(&x, &y, query)
        .map_err(pyo3::exceptions::PyValueError::new_err)
}

#[pyfunction]
fn finite_difference(values: Vec<f64>, spacing: f64) -> PyResult<Vec<f64>> {
    crate::core::interpolate::finite_difference(&values, spacing)
        .map_err(pyo3::exceptions::PyValueError::new_err)
}
