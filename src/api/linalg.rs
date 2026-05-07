use pyo3::prelude::*;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(dot, module)?)?;
    module.add_function(wrap_pyfunction!(norm, module)?)?;
    module.add_function(wrap_pyfunction!(add, module)?)?;
    module.add_function(wrap_pyfunction!(sub, module)?)?;
    module.add_function(wrap_pyfunction!(scale, module)?)?;
    module.add_function(wrap_pyfunction!(mat_vec, module)?)?;
    module.add_function(wrap_pyfunction!(solve, module)?)?;
    Ok(())
}

#[pyfunction]
fn dot(left: Vec<f64>, right: Vec<f64>) -> PyResult<f64> {
    crate::core::linalg::dot(&left, &right).map_err(pyo3::exceptions::PyValueError::new_err)
}

#[pyfunction]
fn norm(vector: Vec<f64>) -> f64 {
    crate::core::linalg::norm(&vector)
}

#[pyfunction]
fn add(left: Vec<f64>, right: Vec<f64>) -> PyResult<Vec<f64>> {
    crate::core::linalg::add(&left, &right).map_err(pyo3::exceptions::PyValueError::new_err)
}

#[pyfunction]
fn sub(left: Vec<f64>, right: Vec<f64>) -> PyResult<Vec<f64>> {
    crate::core::linalg::sub(&left, &right).map_err(pyo3::exceptions::PyValueError::new_err)
}

#[pyfunction]
fn scale(vector: Vec<f64>, factor: f64) -> Vec<f64> {
    crate::core::linalg::scale(&vector, factor)
}

#[pyfunction]
fn mat_vec(matrix: Vec<Vec<f64>>, vector: Vec<f64>) -> PyResult<Vec<f64>> {
    crate::core::linalg::mat_vec(&matrix, &vector).map_err(pyo3::exceptions::PyValueError::new_err)
}

#[pyfunction]
fn solve(matrix: Vec<Vec<f64>>, rhs: Vec<f64>) -> PyResult<Vec<f64>> {
    crate::core::linalg::solve(matrix, rhs).map_err(pyo3::exceptions::PyValueError::new_err)
}
