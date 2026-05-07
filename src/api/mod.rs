mod callback;

pub mod derivative;
pub mod integrate;
pub mod interpolate;
pub mod linalg;
pub mod ode;
pub mod optimize;
pub mod polynomial;
pub mod root;
pub mod signal;
pub mod stats;
pub mod trig;
pub mod visualize;

use pyo3::prelude::*;

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    derivative::register(module)?;
    root::register(module)?;
    linalg::register(module)?;
    optimize::register(module)?;
    ode::register(module)?;
    integrate::register(module)?;
    trig::register(module)?;
    visualize::register(module)?;
    stats::register(module)?;
    interpolate::register(module)?;
    polynomial::register(module)?;
    signal::register(module)?;
    Ok(())
}
