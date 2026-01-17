#[no_mangle]
pub extern "C" fn sera_trace_new(id: *const u8, name: *const u8, kind: u8) -> *mut crate::core::Trace {
    let id_str = unsafe { std::ffi::CStr::from_ptr(id as *const i8).to_string_lossy().to_string() };
    let name_str = unsafe { std::ffi::CStr::from_ptr(name as *const i8).to_string_lossy().to_string() };

    let chart_kind = match kind {
        0 => crate::core::ChartKind::Line,
        1 => crate::core::ChartKind::Scatter,
        2 => crate::core::ChartKind::Bar,
        3 => crate::core::ChartKind::Area,
        4 => crate::core::ChartKind::Histogram,
        5 => crate::core::ChartKind::Box,
        6 => crate::core::ChartKind::Violin,
        7 => crate::core::ChartKind::Heatmap,
        8 => crate::core::ChartKind::Contour,
        9 => crate::core::ChartKind::Surface,
        10 => crate::core::ChartKind::Bubble,
        11 => crate::core::ChartKind::Candlestick,
        12 => crate::core::ChartKind::Waterfall,
        13 => crate::core::ChartKind::Funnel,
        14 => crate::core::ChartKind::Sunburst,
        15 => crate::core::ChartKind::Treemap,
        16 => crate::core::ChartKind::Sankey,
        _ => crate::core::ChartKind::Scatter,
    };

    Box::into_raw(Box::new(crate::core::Trace::new(id_str, name_str, chart_kind, Vec::new(), Vec::new())))
}

#[no_mangle]
pub extern "C" fn sera_trace_add_x(trace: *mut crate::core::Trace, value: f64) {
    unsafe {
        if !trace.is_null() {
            (*trace).x.push(value);
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_trace_add_y(trace: *mut crate::core::Trace, value: f64) {
    unsafe {
        if !trace.is_null() {
            (*trace).y.push(value);
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_trace_get_count(trace: *const crate::core::Trace) -> usize {
    unsafe {
        if trace.is_null() {
            0
        } else {
            (*trace).count()
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_trace_free(trace: *mut crate::core::Trace) {
    if !trace.is_null() {
        unsafe {
            let _ = Box::from_raw(trace);
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_layout_new() -> *mut crate::core::Layout {
    Box::into_raw(Box::new(crate::core::Layout::default()))
}

#[no_mangle]
pub extern "C" fn sera_layout_set_title(layout: *mut crate::core::Layout, title: *const u8) {
    unsafe {
        if !layout.is_null() {
            let title_str = std::ffi::CStr::from_ptr(title as *const i8).to_string_lossy().to_string();
            (*layout).title = title_str;
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_layout_set_size(layout: *mut crate::core::Layout, width: u32, height: u32) {
    unsafe {
        if !layout.is_null() {
            (*layout).width = width;
            (*layout).height = height;
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_layout_free(layout: *mut crate::core::Layout) {
    if !layout.is_null() {
        unsafe {
            let _ = Box::from_raw(layout);
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_canvas_new(width: f32, height: f32, layout: *const crate::core::Layout) -> *mut crate::plot::Canvas {
    unsafe {
        if layout.is_null() {
            return std::ptr::null_mut();
        }
        let canvas = crate::plot::Canvas::new(width, height, (*layout).clone());
        Box::into_raw(Box::new(canvas))
    }
}

#[no_mangle]
pub extern "C" fn sera_canvas_add_trace(canvas: *mut crate::plot::Canvas, trace: *mut crate::core::Trace) {
    unsafe {
        if !canvas.is_null() && !trace.is_null() {
            let trace_val = std::ptr::read(trace);
            (*canvas).add_trace_mut(trace_val);
            let _ = Box::from_raw(trace);
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_canvas_auto_scale(canvas: *mut crate::plot::Canvas) {
    unsafe {
        if !canvas.is_null() {
            (*canvas).auto_scale();
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_canvas_zoom(canvas: *mut crate::plot::Canvas, cx: f64, cy: f64, factor: f32) {
    unsafe {
        if !canvas.is_null() {
            (*canvas).zoom(cx, cy, factor);
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_canvas_pan(canvas: *mut crate::plot::Canvas, dx: f64, dy: f64) {
    unsafe {
        if !canvas.is_null() {
            (*canvas).pan(dx, dy);
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_canvas_free(canvas: *mut crate::plot::Canvas) {
    if !canvas.is_null() {
        unsafe {
            let _ = Box::from_raw(canvas);
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_math_mean(values: *const f64, len: usize) -> f64 {
    unsafe {
        let slice = std::slice::from_raw_parts(values, len);
        crate::math::mean(slice).unwrap_or(0.0)
    }
}

#[no_mangle]
pub extern "C" fn sera_math_median(values: *const f64, len: usize) -> f64 {
    unsafe {
        let slice = std::slice::from_raw_parts(values, len).to_vec();
        crate::math::median(slice).unwrap_or(0.0)
    }
}

#[no_mangle]
pub extern "C" fn sera_math_std_dev(values: *const f64, len: usize) -> f64 {
    unsafe {
        let slice = std::slice::from_raw_parts(values, len);
        crate::math::std_dev(slice).unwrap_or(0.0)
    }
}

#[no_mangle]
pub extern "C" fn sera_math_variance(values: *const f64, len: usize) -> f64 {
    unsafe {
        let slice = std::slice::from_raw_parts(values, len);
        crate::math::variance(slice).unwrap_or(0.0)
    }
}

#[no_mangle]
pub extern "C" fn sera_math_percentile(values: *const f64, len: usize, p: f64) -> f64 {
    unsafe {
        let slice = std::slice::from_raw_parts(values, len).to_vec();
        crate::math::percentile(slice, p).unwrap_or(0.0)
    }
}

#[no_mangle]
pub extern "C" fn sera_math_normalize(values: *const f64, len: usize, output: *mut f64) {
    unsafe {
        let input_slice = std::slice::from_raw_parts(values, len);
        let normalized = crate::math::normalize(input_slice);
        let output_slice = std::slice::from_raw_parts_mut(output, len);
        output_slice.copy_from_slice(&normalized[..len.min(normalized.len())]);
    }
}

#[no_mangle]
pub extern "C" fn sera_math_standardize(values: *const f64, len: usize, output: *mut f64) {
    unsafe {
        let input_slice = std::slice::from_raw_parts(values, len);
        let standardized = crate::math::standardize(input_slice);
        let output_slice = std::slice::from_raw_parts_mut(output, len);
        output_slice.copy_from_slice(&standardized[..len.min(standardized.len())]);
    }
}

#[no_mangle]
pub extern "C" fn sera_math_correlation(x: *const f64, y: *const f64, len: usize) -> f64 {
    unsafe {
        let x_slice = std::slice::from_raw_parts(x, len);
        let y_slice = std::slice::from_raw_parts(y, len);
        crate::math::correlation(x_slice, y_slice).unwrap_or(0.0)
    }
}

#[no_mangle]
pub extern "C" fn sera_math_moving_average(values: *const f64, len: usize, window: usize, output: *mut f64) {
    unsafe {
        let input_slice = std::slice::from_raw_parts(values, len);
        let smoothed = crate::math::moving_average(input_slice, window);
        let output_slice = std::slice::from_raw_parts_mut(output, len);
        output_slice.copy_from_slice(&smoothed[..len.min(smoothed.len())]);
    }
}

#[no_mangle]
pub extern "C" fn sera_version() -> *const u8 {
    crate::VERSION.as_ptr()
}
