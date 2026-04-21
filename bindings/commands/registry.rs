#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use crate::Chart;

#[cfg(feature = "python")]
use super::native::*;

use crate::for_each_chart_fn;
use crate::for_each_chart_class;

#[cfg(feature = "python")]
pub fn register_submodules(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(set_bg_fn, m)?)?;
    m.add_function(wrap_pyfunction!(show_chart_value, m)?)?;
    m.add_function(wrap_pyfunction!(bench_chart_value, m)?)?;
    m.add_function(wrap_pyfunction!(set_chart_kind, m)?)?;
    m.add_function(wrap_pyfunction!(set_chart_orientation, m)?)?;
    m.add_function(wrap_pyfunction!(bench_pure_rust, m)?)?;

    macro_rules! add_fn {
        ($fn:ident, $_js:literal) => { m.add_function(wrap_pyfunction!($fn, m)?)?; }
    }
    for_each_chart_fn!(add_fn);

    macro_rules! add_class {
        ($cls:ident) => { m.add_class::<$cls>()?; }
    }
    for_each_chart_class!(add_class);

    m.add_function(wrap_pyfunction!(build_hover_json, m)?)?;
    m.add_function(wrap_pyfunction!(plot_chart, m)?)?;
    m.add_function(wrap_pyfunction!(savefig, m)?)?;

    for (alias, canonical) in crate::bindings::registry_macro::CHART_ALIASES {
        if let Ok(func) = m.getattr(*canonical) {
            let _ = m.add(*alias, func);
        }
    }

    super::ml::register_ml_classes(m)?;

    Ok(())
}