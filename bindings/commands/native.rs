use crate::bindings::registry_macro::{for_each_json_chart_fn, for_each_ml_oneshot_fn, for_each_util_fn, for_each_auto_util_fn};

#[cfg(feature = "js")]
crate::impl_wasm_bindings!();

#[cfg(feature = "ffi")]
crate::impl_cffi_bindings!();

#[cfg(feature = "python")]
crate::impl_python_bindings!();

