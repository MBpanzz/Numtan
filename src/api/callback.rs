use pyo3::prelude::*;
use std::cell::RefCell;

pub fn scalar(
    py: Python<'_>,
    function: &Py<PyAny>,
    error: &RefCell<Option<PyErr>>,
    value: f64,
) -> f64 {
    if error.borrow().is_some() {
        return f64::NAN;
    }
    match function
        .call1(py, (value,))
        .and_then(|result| result.extract(py))
    {
        Ok(value) => value,
        Err(err) => {
            *error.borrow_mut() = Some(err);
            f64::NAN
        }
    }
}

pub fn vector_scalar(
    py: Python<'_>,
    function: &Py<PyAny>,
    error: &RefCell<Option<PyErr>>,
    value: &[f64],
) -> f64 {
    if error.borrow().is_some() {
        return f64::NAN;
    }
    match function
        .call1(py, (value.to_vec(),))
        .and_then(|result| result.extract(py))
    {
        Ok(value) => value,
        Err(err) => {
            *error.borrow_mut() = Some(err);
            f64::NAN
        }
    }
}

pub fn scalar2(
    py: Python<'_>,
    function: &Py<PyAny>,
    error: &RefCell<Option<PyErr>>,
    x: f64,
    y: f64,
) -> f64 {
    if error.borrow().is_some() {
        return f64::NAN;
    }
    match function
        .call1(py, (x, y))
        .and_then(|result| result.extract(py))
    {
        Ok(value) => value,
        Err(err) => {
            *error.borrow_mut() = Some(err);
            f64::NAN
        }
    }
}

pub fn vector_vector(
    py: Python<'_>,
    function: &Py<PyAny>,
    error: &RefCell<Option<PyErr>>,
    value: &[f64],
) -> Vec<f64> {
    if error.borrow().is_some() {
        return Vec::new();
    }
    match function
        .call1(py, (value.to_vec(),))
        .and_then(|result| result.extract(py))
    {
        Ok(value) => value,
        Err(err) => {
            *error.borrow_mut() = Some(err);
            Vec::new()
        }
    }
}
