#[cfg(feature = "js")]
crate::impl_wasm_bindings!();

#[cfg(feature = "ffi")]
crate::impl_cffi_bindings!();

#[cfg(feature = "python")]
crate::impl_python_bindings!();