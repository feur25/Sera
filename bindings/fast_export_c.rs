use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use crate::html::fast_exporter::FastHtmlExporter;

#[no_mangle]
pub extern "C" fn sera_fast_export_create(
    width: c_int,
    height: c_int,
    title: *const c_char,
) -> *mut FastHtmlExporter {
    if title.is_null() {
        return std::ptr::null_mut();
    }
    
    let title_str = unsafe { CStr::from_ptr(title).to_string_lossy().into_owned() };
    let exporter = FastHtmlExporter::new(width, height, title_str);
    Box::into_raw(Box::new(exporter))
}

#[no_mangle]
pub extern "C" fn sera_fast_build_html(
    exporter: *mut FastHtmlExporter,
    labels: *const *const c_char,
    values: *const f64,
    count: usize,
) -> *mut c_char {
    if exporter.is_null() || labels.is_null() || values.is_null() {
        return std::ptr::null_mut();
    }

    let mut label_vec = Vec::with_capacity(count);
    unsafe {
        for i in 0..count {
            let label_ptr = *labels.add(i);
            if !label_ptr.is_null() {
                label_vec.push(CStr::from_ptr(label_ptr).to_string_lossy().into_owned());
            } else {
                label_vec.push(String::new());
            }
        }
    }

    let value_vec = unsafe { std::slice::from_raw_parts(values, count).to_vec() };
    
    let exporter = unsafe { &*exporter };
    let html = exporter.build_optimized(label_vec, value_vec);
    
    let c_string = CString::new(html).unwrap_or_default();
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn sera_free_html(html: *mut c_char) {
    if !html.is_null() {
        unsafe {
            let _ = CString::from_raw(html);
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_fast_export_destroy(exporter: *mut FastHtmlExporter) {
    if !exporter.is_null() {
        unsafe {
            let _ = Box::from_raw(exporter);
        }
    }
}
