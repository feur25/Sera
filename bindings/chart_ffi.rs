use crate::Chart;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn sera_chart_from_html(html: *const c_char) -> *mut Chart {
    if html.is_null() {
        return std::ptr::null_mut();
    }
    let html_str = unsafe { CStr::from_ptr(html) }.to_string_lossy().into_owned();
    Box::into_raw(Box::new(Chart {
        html: html_str,
        doc_str: "",
    }))
}

#[no_mangle]
pub extern "C" fn sera_chart_html(chart: *const Chart) -> *mut c_char {
    if chart.is_null() {
        return std::ptr::null_mut();
    }
    let h = &unsafe { &*chart }.html;
    CString::new(h.as_str())
        .map(|s| s.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn sera_chart_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            drop(CString::from_raw(s));
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_chart_free(chart: *mut Chart) {
    if !chart.is_null() {
        unsafe {
            drop(Box::from_raw(chart));
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_chart_save(chart: *const Chart, path: *const c_char) -> i32 {
    if chart.is_null() || path.is_null() {
        return -1;
    }
    let h = &unsafe { &*chart }.html;
    let p = match unsafe { CStr::from_ptr(path) }.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };
    match std::fs::write(p, h.as_str()) {
        Ok(()) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "C" fn sera_chart_to_svg(chart: *const Chart) -> *mut c_char {
    if chart.is_null() {
        return std::ptr::null_mut();
    }
    let h = &unsafe { &*chart }.html;
    let result = h
        .find("<svg")
        .and_then(|start| h.rfind("</svg>").map(|end| h[start..end + 6].to_string()));
    result
        .and_then(|s| CString::new(s).ok())
        .map(|s| s.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn sera_chart_inject_css(chart: *const Chart, css: *const c_char) -> *mut Chart {
    if chart.is_null() || css.is_null() {
        return std::ptr::null_mut();
    }
    let c = unsafe { &*chart };
    let s = match unsafe { CStr::from_ptr(css) }.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    Box::into_raw(Box::new(c.inject_css(s)))
}

#[no_mangle]
pub extern "C" fn sera_chart_inject_js(chart: *const Chart, js: *const c_char) -> *mut Chart {
    if chart.is_null() || js.is_null() {
        return std::ptr::null_mut();
    }
    let c = unsafe { &*chart };
    let s = match unsafe { CStr::from_ptr(js) }.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    Box::into_raw(Box::new(c.inject_js(s)))
}

#[no_mangle]
pub extern "C" fn sera_chart_set_bg(chart: *const Chart, color: *const c_char) -> *mut Chart {
    if chart.is_null() {
        return std::ptr::null_mut();
    }
    let c = unsafe { &*chart };
    let col = if color.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(color) }.to_str().ok()
    };
    Box::into_raw(Box::new(c.set_bg(col)))
}

#[no_mangle]
pub extern "C" fn sera_chart_no_background(chart: *const Chart) -> *mut Chart {
    if chart.is_null() {
        return std::ptr::null_mut();
    }
    Box::into_raw(Box::new(unsafe { &*chart }.no_background()))
}

#[no_mangle]
pub extern "C" fn sera_chart_no_x_axis(chart: *const Chart) -> *mut Chart {
    if chart.is_null() {
        return std::ptr::null_mut();
    }
    Box::into_raw(Box::new(unsafe { &*chart }.no_x_axis()))
}

#[no_mangle]
pub extern "C" fn sera_chart_no_y_axis(chart: *const Chart) -> *mut Chart {
    if chart.is_null() {
        return std::ptr::null_mut();
    }
    Box::into_raw(Box::new(unsafe { &*chart }.no_y_axis()))
}

#[no_mangle]
pub extern "C" fn sera_chart_no_axes(chart: *const Chart) -> *mut Chart {
    if chart.is_null() {
        return std::ptr::null_mut();
    }
    Box::into_raw(Box::new(unsafe { &*chart }.no_axes()))
}

#[no_mangle]
pub extern "C" fn sera_chart_show_grid(chart: *const Chart) -> *mut Chart {
    if chart.is_null() {
        return std::ptr::null_mut();
    }
    Box::into_raw(Box::new(unsafe { &*chart }.show_grid()))
}

#[no_mangle]
pub extern "C" fn sera_chart_hide_grid(chart: *const Chart) -> *mut Chart {
    if chart.is_null() {
        return std::ptr::null_mut();
    }
    Box::into_raw(Box::new(unsafe { &*chart }.hide_grid()))
}

#[no_mangle]
pub extern "C" fn sera_chart_responsive(chart: *const Chart) -> *mut Chart {
    if chart.is_null() {
        return std::ptr::null_mut();
    }
    Box::into_raw(Box::new(unsafe { &*chart }.responsive()))
}

#[no_mangle]
pub extern "C" fn sera_chart_flip(chart: *const Chart) -> *mut Chart {
    if chart.is_null() {
        return std::ptr::null_mut();
    }
    Box::into_raw(Box::new(unsafe { &*chart }.flip()))
}

#[no_mangle]
pub extern "C" fn sera_chart_crosshair(chart: *const Chart) -> *mut Chart {
    if chart.is_null() {
        return std::ptr::null_mut();
    }
    Box::into_raw(Box::new(unsafe { &*chart }.crosshair()))
}

#[no_mangle]
pub extern "C" fn sera_chart_zoom(chart: *const Chart) -> *mut Chart {
    if chart.is_null() {
        return std::ptr::null_mut();
    }
    Box::into_raw(Box::new(unsafe { &*chart }.zoom()))
}

#[no_mangle]
pub extern "C" fn sera_chart_no_legend(chart: *const Chart) -> *mut Chart {
    if chart.is_null() {
        return std::ptr::null_mut();
    }
    Box::into_raw(Box::new(unsafe { &*chart }.no_legend()))
}

#[no_mangle]
pub extern "C" fn sera_chart_no_title(chart: *const Chart) -> *mut Chart {
    if chart.is_null() {
        return std::ptr::null_mut();
    }
    Box::into_raw(Box::new(unsafe { &*chart }.no_title()))
}

#[no_mangle]
pub extern "C" fn sera_chart_export_button(chart: *const Chart) -> *mut Chart {
    if chart.is_null() {
        return std::ptr::null_mut();
    }
    Box::into_raw(Box::new(unsafe { &*chart }.export_button()))
}

#[no_mangle]
pub unsafe extern "C" fn sera_call(
    name: *const c_char,
    json: *const c_char,
) -> *mut c_char {
    let name = unsafe { CStr::from_ptr(name) }.to_str().unwrap_or("");
    let json = unsafe { CStr::from_ptr(json) }.to_str().unwrap_or("{}");
    let resolved = crate::bindings::alias_registry::resolve(name);
    let target = resolved.as_deref().unwrap_or(name);
    for entry in crate::bindings::fn_registry::iter_entries() {
        if entry.name == target {
            let result = (entry.invoke)(json);
            return CString::new(result).unwrap_or_default().into_raw();
        }
    }
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn sera_list() -> *mut c_char {
    let names: Vec<&str> = crate::bindings::fn_registry::iter_entries()
        .map(|e| e.name)
        .collect();
    CString::new(serde_json::to_string(&names).unwrap_or_default())
        .unwrap_or_default()
        .into_raw()
}

#[no_mangle]
pub extern "C" fn sera_version() -> *mut c_char {
    CString::new(env!("CARGO_PKG_VERSION"))
        .unwrap_or_default()
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn sera_params_json(
    chart: *const c_char,
    variant: *const c_char,
) -> *mut c_char {
    let chart = if chart.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(chart) }.to_str().unwrap_or(""))
    };
    let variant = if variant.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(variant) }.to_str().unwrap_or(""))
    };
    let s = serde_json::to_string(&crate::params(chart, variant)).unwrap_or_default();
    CString::new(s).unwrap_or_default().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn sera_required_params_json(
    chart: *const c_char,
    variant: *const c_char,
) -> *mut c_char {
    let chart = if chart.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(chart) }.to_str().unwrap_or(""))
    };
    let variant = if variant.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(variant) }.to_str().unwrap_or(""))
    };
    let s =
        serde_json::to_string(&crate::required_params(chart, variant)).unwrap_or_default();
    CString::new(s).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn sera_chart_variants_json() -> *mut c_char {
    let s = serde_json::to_string(&crate::chart_variants()).unwrap_or_default();
    CString::new(s).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn sera_chart_themes_json() -> *mut c_char {
    let s = serde_json::to_string(&crate::chart_themes()).unwrap_or_default();
    CString::new(s).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn sera_scenes3d_json() -> *mut c_char {
    let s = serde_json::to_string(&crate::scenes3d()).unwrap_or_default();
    CString::new(s).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn sera_docs_json() -> *mut c_char {
    let s = serde_json::to_string(&crate::doc_registry::all_docs()).unwrap_or_default();
    CString::new(s).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn sera_themes_list() -> *mut c_char {
    let s = serde_json::to_string(&crate::themes()).unwrap_or_default();
    CString::new(s).unwrap_or_default().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn sera_set_theme(name: *const c_char) {
    let name = unsafe { CStr::from_ptr(name) }.to_str().unwrap_or("");
    if let Some(preset) = crate::resolve_theme(name) {
        if let Ok(mut bg) = crate::GLOBAL_BACKGROUND.lock() {
            *bg = preset.bg.map(|v| v.to_string());
        }
        if let Ok(mut palette) = crate::GLOBAL_PALETTE.lock() {
            *palette = Some(preset.palette.to_vec());
        }
        crate::GLOBAL_GRIDLINES.store(preset.gridlines, std::sync::atomic::Ordering::Relaxed);
        if let Ok(mut tn) = crate::GLOBAL_THEME_NAME.lock() {
            *tn = Some(name.to_string());
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn sera_set_bg(color: *const c_char) {
    let color = unsafe { CStr::from_ptr(color) }.to_str().unwrap_or("");
    crate::set_global_background(color);
}

#[no_mangle]
pub extern "C" fn sera_reset_bg() {
    crate::reset_global_background();
}

#[no_mangle]
pub extern "C" fn sera_demos_list() -> *mut c_char {
    let s = serde_json::to_string(&crate::demos()).unwrap_or_default();
    CString::new(s).unwrap_or_default().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn sera_demo_code(
    name: *const c_char,
    variant: *const c_char,
) -> *mut c_char {
    let name = unsafe { CStr::from_ptr(name) }.to_str().unwrap_or("");
    let variant = if variant.is_null() {
        None
    } else {
        let s = unsafe { CStr::from_ptr(variant) }.to_str().unwrap_or("");
        if s.is_empty() { None } else { Some(s) }
    };
    let result = crate::demo(name, variant).unwrap_or_default();
    CString::new(result).unwrap_or_default().into_raw()
}
