include!(concat!(env!("OUT_DIR"), "/chart_fn_macro.rs"));
include!(concat!(env!("OUT_DIR"), "/chart_py_wrapper_macro.rs"));
include!(concat!(env!("OUT_DIR"), "/ml_fn_macro.rs"));
include!(concat!(env!("OUT_DIR"), "/ml_pyclass_macro.rs"));
include!(concat!(env!("OUT_DIR"), "/ml_pyfn_macro.rs"));
include!(concat!(env!("OUT_DIR"), "/util_fn_macro.rs"));
include!(concat!(env!("OUT_DIR"), "/auto_util_fn_macro.rs"));

#[macro_export]
macro_rules! for_each_chart_fn {
    ($mac:ident) => {
        for_each_json_chart_fn!($mac);
    };
}

#[macro_export]
macro_rules! for_each_chart_class {
    ($mac:ident) => {};
}

pub use crate::CHART_ALIAS_REGISTRY as CHART_ALIASES;

#[macro_export]
macro_rules! for_each_fn {
    ($mac:ident) => {
        for_each_json_chart_fn!($mac);
        for_each_ml_oneshot_fn!($mac);
        for_each_util_fn!($mac);
    };
}
