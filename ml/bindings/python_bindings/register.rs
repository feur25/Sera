use crate::bindings::registry_macro::{for_each_ml_pyclass, for_each_ml_python_fn};

pub fn register_ml_classes(m: &Bound<'_, PyModule>) -> PyResult<()> {
    macro_rules! add_ml_class {
        ($cls:ident) => { m.add_class::<$cls>()?; };
    }
    for_each_ml_pyclass!(add_ml_class);
    macro_rules! add_ml_python_fn {
        ($fn:ident, $_js:literal) => { m.add_function(wrap_pyfunction!($fn, m)?)?; };
    }
    for_each_ml_python_fn!(add_ml_python_fn);
    Ok(())
}

pub fn register_full_ml(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_ml_classes(m)
}
