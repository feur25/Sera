use pyo3::prelude::*;

use crate::for_each_chart_class;

pub fn register_submodules(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[allow(unused_macros)]
    macro_rules! add_class {
        ($cls:ident) => { m.add_class::<$cls>()?; }
    }
    for_each_chart_class!(add_class);

    crate::bindings::fn_registry::register_all(m)?;

    for (alias, canonical) in crate::bindings::registry_macro::CHART_ALIASES {
        if let Ok(func) = m.getattr(*canonical) {
            let _ = m.add(*alias, func);
        }
    }

    crate::ml::bindings::register_ml_classes(m)?;

    Ok(())
}
