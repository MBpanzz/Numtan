use pyo3::prelude::*;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(tanpi, module)?)?;
    module.add_function(wrap_pyfunction!(tanint, module)?)?;
    module.add_function(wrap_pyfunction!(atanint, module)?)?;
    module.add_function(wrap_pyfunction!(complex_tan, module)?)?;
    module.add_function(wrap_pyfunction!(tan_deg, module)?)?;
    module.add_function(wrap_pyfunction!(tan_grad, module)?)?;
    Ok(())
}

#[pyfunction]
fn tanpi(x: f64) -> f64 {
    crate::core::trig::tanpi(x)
}

#[pyfunction]
fn tanint(x: f64) -> f64 {
    crate::core::trig::tanint(x)
}

#[pyfunction]
fn atanint(x: f64) -> f64 {
    crate::core::trig::atanint(x)
}

#[pyfunction]
fn complex_tan(re: f64, im: f64) -> (f64, f64) {
    crate::core::trig::complex_tan(re, im)
}

#[pyfunction]
fn tan_deg(x: f64) -> f64 {
    crate::core::trig::tan_deg(x)
}

#[pyfunction]
fn tan_grad(x: f64) -> f64 {
    crate::core::trig::tan_grad(x)
}
