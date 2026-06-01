use crate::bindings::registry_macro::for_each_ml_oneshot_fn;

#[allow(unused_macros)]
macro_rules! impl_python_json_ml {
    ($fn:ident, $_js:literal) => {
        #[pyfunction]
        pub fn $fn(input: &str) -> String {
            crate::bindings::commands::ml::$fn(input)
        }
    };
}
for_each_ml_oneshot_fn!(impl_python_json_ml);