use pyo3::prelude::*;
use std::cell::RefCell;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(tangent, module)?)?;
    module.add_function(wrap_pyfunction!(gradient, module)?)?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (function, x, method = "central"))]
fn tangent(py: Python<'_>, function: Py<PyAny>, x: f64, method: &str) -> PyResult<f64> {
    let _ = method;
    let error = RefCell::new(None);
    let closure =
        |value: f64| -> f64 { crate::api::callback::scalar(py, &function, &error, value) };
    let result = crate::core::derivative::tangent(&closure, x);
    if let Some(err) = error.into_inner() {
        Err(err)
    } else {
        Ok(result)
    }
}

#[pyfunction]
fn gradient(py: Python<'_>, function: Py<PyAny>, point: Vec<f64>) -> PyResult<Vec<f64>> {
    let error = RefCell::new(None);
    let closure = |value: &[f64]| -> f64 {
        crate::api::callback::vector_scalar(py, &function, &error, value)
    };
    let result = crate::core::derivative::gradient(&closure, &point);
    if let Some(err) = error.into_inner() {
        Err(err)
    } else {
        Ok(result)
    }
}
