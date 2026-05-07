use pyo3::prelude::*;
use pyo3::types::PyDict;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(mean, module)?)?;
    module.add_function(wrap_pyfunction!(variance, module)?)?;
    module.add_function(wrap_pyfunction!(summary, module)?)?;
    module.add_function(wrap_pyfunction!(covariance, module)?)?;
    module.add_function(wrap_pyfunction!(correlation, module)?)?;
    module.add_function(wrap_pyfunction!(linear_regression, module)?)?;
    module.add_function(wrap_pyfunction!(polynomial_regression, module)?)?;
    Ok(())
}

#[pyfunction]
fn mean(values: Vec<f64>) -> PyResult<f64> {
    crate::core::stats::mean(&values).map_err(pyo3::exceptions::PyValueError::new_err)
}

#[pyfunction]
#[pyo3(signature = (values, sample = false))]
fn variance(values: Vec<f64>, sample: bool) -> PyResult<f64> {
    crate::core::stats::variance(&values, sample).map_err(pyo3::exceptions::PyValueError::new_err)
}

#[pyfunction]
fn summary(py: Python<'_>, values: Vec<f64>) -> PyResult<Py<PyAny>> {
    let result =
        crate::core::stats::summary(&values).map_err(pyo3::exceptions::PyValueError::new_err)?;
    let dict = PyDict::new(py);
    dict.set_item("count", result.count)?;
    dict.set_item("mean", result.mean)?;
    dict.set_item("variance", result.variance)?;
    dict.set_item("std_dev", result.std_dev)?;
    dict.set_item("min", result.min)?;
    dict.set_item("max", result.max)?;
    Ok(dict.into())
}

#[pyfunction]
#[pyo3(signature = (left, right, sample = false))]
fn covariance(left: Vec<f64>, right: Vec<f64>, sample: bool) -> PyResult<f64> {
    crate::core::stats::covariance(&left, &right, sample)
        .map_err(pyo3::exceptions::PyValueError::new_err)
}

#[pyfunction]
fn correlation(left: Vec<f64>, right: Vec<f64>) -> PyResult<f64> {
    crate::core::stats::correlation(&left, &right).map_err(pyo3::exceptions::PyValueError::new_err)
}

#[pyfunction]
fn linear_regression(py: Python<'_>, x: Vec<f64>, y: Vec<f64>) -> PyResult<Py<PyAny>> {
    let result = crate::core::stats::linear_regression(&x, &y)
        .map_err(pyo3::exceptions::PyValueError::new_err)?;
    let dict = PyDict::new(py);
    dict.set_item("slope", result.slope)?;
    dict.set_item("intercept", result.intercept)?;
    dict.set_item("r2", result.r2)?;
    Ok(dict.into())
}

#[pyfunction]
fn polynomial_regression(x: Vec<f64>, y: Vec<f64>, degree: usize) -> PyResult<Vec<f64>> {
    crate::core::stats::polynomial_regression(&x, &y, degree)
        .map_err(pyo3::exceptions::PyValueError::new_err)
}
