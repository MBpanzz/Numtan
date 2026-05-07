use pyo3::prelude::*;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(polyval, module)?)?;
    module.add_function(wrap_pyfunction!(polyder, module)?)?;
    module.add_function(wrap_pyfunction!(polyint, module)?)?;
    module.add_function(wrap_pyfunction!(polyadd, module)?)?;
    module.add_function(wrap_pyfunction!(polymul, module)?)?;
    module.add_function(wrap_pyfunction!(polyroot, module)?)?;
    Ok(())
}

#[pyfunction]
fn polyval(coefficients: Vec<f64>, x: f64) -> f64 {
    crate::core::polynomial::evaluate(&coefficients, x)
}

#[pyfunction]
fn polyder(coefficients: Vec<f64>) -> Vec<f64> {
    crate::core::polynomial::derivative(&coefficients)
}

#[pyfunction]
#[pyo3(signature = (coefficients, constant = 0.0))]
fn polyint(coefficients: Vec<f64>, constant: f64) -> Vec<f64> {
    crate::core::polynomial::integral(&coefficients, constant)
}

#[pyfunction]
fn polyadd(left: Vec<f64>, right: Vec<f64>) -> Vec<f64> {
    crate::core::polynomial::add(&left, &right)
}

#[pyfunction]
fn polymul(left: Vec<f64>, right: Vec<f64>) -> Vec<f64> {
    crate::core::polynomial::multiply(&left, &right)
}

#[pyfunction]
#[pyo3(signature = (coefficients, guess, tol = 1e-10, max_iter = 50))]
fn polyroot(coefficients: Vec<f64>, guess: f64, tol: f64, max_iter: usize) -> PyResult<f64> {
    crate::core::polynomial::root(&coefficients, guess, tol, max_iter)
        .map_err(pyo3::exceptions::PyValueError::new_err)
}
