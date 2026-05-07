use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(moving_average, module)?)?;
    module.add_function(wrap_pyfunction!(exponential_smooth, module)?)?;
    module.add_function(wrap_pyfunction!(convolve, module)?)?;
    module.add_function(wrap_pyfunction!(normalize, module)?)?;
    module.add_function(wrap_pyfunction!(find_peaks, module)?)?;
    Ok(())
}

#[pyfunction]
fn moving_average(values: Vec<f64>, window: usize) -> PyResult<Vec<f64>> {
    crate::core::signal::moving_average(&values, window)
        .map_err(pyo3::exceptions::PyValueError::new_err)
}

#[pyfunction]
fn exponential_smooth(values: Vec<f64>, alpha: f64) -> PyResult<Vec<f64>> {
    crate::core::signal::exponential_smooth(&values, alpha)
        .map_err(pyo3::exceptions::PyValueError::new_err)
}

#[pyfunction]
fn convolve(signal: Vec<f64>, kernel: Vec<f64>) -> PyResult<Vec<f64>> {
    crate::core::signal::convolve(&signal, &kernel).map_err(pyo3::exceptions::PyValueError::new_err)
}

#[pyfunction]
fn normalize(values: Vec<f64>) -> PyResult<Vec<f64>> {
    crate::core::signal::normalize(&values).map_err(pyo3::exceptions::PyValueError::new_err)
}

#[pyfunction]
fn find_peaks(py: Python<'_>, values: Vec<f64>, threshold: f64) -> PyResult<Py<PyAny>> {
    let peaks = crate::core::signal::find_peaks(&values, threshold);
    let list = PyList::empty(py);
    for peak in peaks {
        let item = PyDict::new(py);
        item.set_item("index", peak.index)?;
        item.set_item("value", peak.value)?;
        list.append(item)?;
    }
    Ok(list.into())
}
