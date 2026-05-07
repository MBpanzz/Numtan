pub mod api;
pub mod core;

use pyo3::prelude::*;

#[pymodule]
fn numtan(module: &Bound<'_, PyModule>) -> PyResult<()> {
    api::register(module)?;
    Ok(())
}
