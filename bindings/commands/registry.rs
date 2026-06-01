#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use crate::python::*;

use crate::for_each_chart_class;
use crate::bindings::registry_macro::{for_each_json_chart_py_wrapper_fn, for_each_ml_oneshot_fn, for_each_auto_util_fn};

#[cfg(feature = "python")]
pub fn register_submodules(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(__sera_py_set_bg, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_show_chart_value, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_bench_chart_value, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_set_chart_kind, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_set_chart_orientation, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_bench_pure_rust, m)?)?;

    macro_rules! add_chart_py_fn {
        ($fn:ident) => { m.add_function(wrap_pyfunction!($fn, m)?)?; }
    }
    for_each_json_chart_py_wrapper_fn!(add_chart_py_fn);

    macro_rules! add_fn {
        ($fn:ident, $_js:literal) => { m.add_function(wrap_pyfunction!($fn, m)?)?; }
    }
    for_each_ml_oneshot_fn!(add_fn);
    for_each_auto_util_fn!(add_fn);

    #[allow(unused_macros)]
    macro_rules! add_class {
        ($cls:ident) => { m.add_class::<$cls>()?; }
    }
    for_each_chart_class!(add_class);

    m.add_function(wrap_pyfunction!(build_grid, m)?)?;
    m.add_function(wrap_pyfunction!(grid, m)?)?;
    m.add_function(wrap_pyfunction!(build_slideshow, m)?)?;
    m.add_function(wrap_pyfunction!(build_sysmon, m)?)?;
    m.add_function(wrap_pyfunction!(sysmon, m)?)?;
    m.add_function(wrap_pyfunction!(build_hover_json, m)?)?;
    m.add_function(wrap_pyfunction!(plot_chart, m)?)?;
    m.add_function(wrap_pyfunction!(push_telemetry, m)?)?;

    for (alias, canonical) in crate::bindings::registry_macro::CHART_ALIASES {
        if let Ok(func) = m.getattr(*canonical) {
            let _ = m.add(*alias, func);
        }
    }

    super::ml::register_ml_classes(m)?;

    Ok(())
}

