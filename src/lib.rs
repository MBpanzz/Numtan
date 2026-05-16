pub mod api;
pub mod core;

use pyo3::prelude::*;

#[pymodule]
fn numtan(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__version__", env!("CARGO_PKG_VERSION"))?;
    api::register(module)?;
    Ok(())
}
