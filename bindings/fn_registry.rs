#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
pub struct PyFnEntry {
    pub register: fn(&Bound<'_, PyModule>) -> PyResult<()>,
}

#[cfg(feature = "python")]
inventory::collect!(PyFnEntry);

#[cfg(feature = "python")]
pub fn register_all(m: &Bound<'_, PyModule>) -> PyResult<()> {
    for entry in inventory::iter::<PyFnEntry>() {
        (entry.register)(m)?;
    }
    Ok(())
}
