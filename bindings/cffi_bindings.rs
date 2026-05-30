#[macro_export]
macro_rules! impl_cffi_bindings {
    () => {
        macro_rules! impl_cffi_chart {
            ($fn:ident, $_js:literal) => {
                #[no_mangle]
                pub unsafe extern "C" fn $fn(input: *const std::os::raw::c_char) -> *mut std::os::raw::c_char {
                    let s = std::ffi::CStr::from_ptr(input).to_str().unwrap_or("");
                    std::ffi::CString::new(crate::bindings::commands::charts::$fn(s))
                        .unwrap_or_default()
                        .into_raw()
                }
            };
        }
        crate::for_each_json_chart_fn!(impl_cffi_chart);

        macro_rules! impl_cffi_ml {
            ($fn:ident, $_js:literal) => {
                #[no_mangle]
                pub unsafe extern "C" fn $fn(input: *const std::os::raw::c_char) -> *mut std::os::raw::c_char {
                    let s = std::ffi::CStr::from_ptr(input).to_str().unwrap_or("");
                    std::ffi::CString::new(crate::bindings::commands::ml::$fn(s))
                        .unwrap_or_default()
                        .into_raw()
                }
            };
        }
        crate::for_each_ml_oneshot_fn!(impl_cffi_ml);

        macro_rules! impl_cffi_util {
            ($fn:ident, $_js:literal) => {
                #[no_mangle]
                pub unsafe extern "C" fn $fn(input: *const std::os::raw::c_char) -> *mut std::os::raw::c_char {
                    let s = std::ffi::CStr::from_ptr(input).to_str().unwrap_or("");
                    std::ffi::CString::new(crate::bindings::commands::charts::$fn(s))
                        .unwrap_or_default()
                        .into_raw()
                }
            };
        }
        crate::for_each_util_fn!(impl_cffi_util);
        #[no_mangle]
        pub unsafe extern "C" fn seraplot_free(ptr: *mut std::os::raw::c_char) {
            if !ptr.is_null() { drop(std::ffi::CString::from_raw(ptr)); }
        }
    };
}
