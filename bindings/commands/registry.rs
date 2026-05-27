#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use crate::Chart;

#[cfg(feature = "python")]
use super::native::*;

use crate::for_each_chart_fn;
use crate::for_each_chart_class;
use crate::bindings::registry_macro::{for_each_json_chart_fn, for_each_ml_oneshot_fn};

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
    for_each_ml_oneshot_fn!(add_fn);

    macro_rules! add_class {
        ($cls:ident) => { m.add_class::<$cls>()?; }
    }
    for_each_chart_class!(add_class);

    m.add_function(wrap_pyfunction!(build_hover_json, m)?)?;
    m.add_function(wrap_pyfunction!(plot_chart, m)?)?;
    m.add_function(wrap_pyfunction!(savefig, m)?)?;
    m.add_function(wrap_pyfunction!(export_svg, m)?)?;
    m.add_function(wrap_pyfunction!(export_data_url, m)?)?;
    m.add_function(wrap_pyfunction!(export_png, m)?)?;
    m.add_function(wrap_pyfunction!(chart_info, m)?)?;
    m.add_class::<LiveStream>()?;
    m.add_function(wrap_pyfunction!(facet, m)?)?;
    m.add_function(wrap_pyfunction!(drift, m)?)?;
    m.add_function(wrap_pyfunction!(lttb, m)?)?;
    m.add_function(wrap_pyfunction!(show, m)?)?;
    m.add_function(wrap_pyfunction!(auto_classify, m)?)?;
    m.add_class::<Pipeline>()?;

    for (alias, canonical) in crate::bindings::registry_macro::CHART_ALIASES {
        if let Ok(func) = m.getattr(*canonical) {
            let _ = m.add(*alias, func);
        }
    }

    super::ml::register_ml_classes(m)?;

    Ok(())
}

