use std::ffi::{CStr, c_char};

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

#[no_mangle]
pub extern "C" fn sera_chart_kind_name(kind: u8) -> *const u8 {
    let name = match kind {
        0 => "Line",
        1 => "Scatter",
        2 => "Bar",
        3 => "Area",
        4 => "Histogram",
        5 => "Box",
        6 => "Violin",
        7 => "Heatmap",
        8 => "Contour",
        9 => "Surface",
        10 => "Bubble",
        11 => "Candlestick",
        12 => "Waterfall",
        13 => "Funnel",
        14 => "Sunburst",
        15 => "Treemap",
        16 => "Sankey",
        _ => "Unknown",
    };
    name.as_ptr()
}

#[no_mangle]
pub extern "C" fn sera_chart_can_transform(from_kind: u8, to_kind: u8) -> bool {
    let compatible_1d = [0, 1, 2, 3, 4, 5, 6, 12, 13];
    let compatible_2d = [7, 8, 9, 15, 16];
    let compatible_dist = [4, 5, 6];
    let compatible_special = [10, 11, 14];

    let from_cat = if compatible_1d.contains(&from_kind) { 0 }
        else if compatible_2d.contains(&from_kind) { 1 }
        else if compatible_dist.contains(&from_kind) { 2 }
        else if compatible_special.contains(&from_kind) { 3 }
        else { 99 };

    let to_cat = if compatible_1d.contains(&to_kind) { 0 }
        else if compatible_2d.contains(&to_kind) { 1 }
        else if compatible_dist.contains(&to_kind) { 2 }
        else if compatible_special.contains(&to_kind) { 3 }
        else { 99 };

    from_cat == to_cat
}

use std::sync::Mutex;

static PLOT_SORT: std::sync::OnceLock<Mutex<i32>> = std::sync::OnceLock::new();
static PLOT_ZOOM: std::sync::OnceLock<Mutex<f32>> = std::sync::OnceLock::new();
static PLOT_ORIENTATION: std::sync::OnceLock<Mutex<bool>> = std::sync::OnceLock::new();
static PLOT_PAN_X: std::sync::OnceLock<Mutex<f32>> = std::sync::OnceLock::new();
static PLOT_CHART_KIND: std::sync::OnceLock<Mutex<u8>> = std::sync::OnceLock::new();
static PLOT_VARIANTS: std::sync::OnceLock<Mutex<Vec<(u8, String)>>> = std::sync::OnceLock::new();
static PLOT_SHOW_SELECTOR: std::sync::OnceLock<Mutex<bool>> = std::sync::OnceLock::new();

fn get_plot_sort() -> &'static Mutex<i32> {
    PLOT_SORT.get_or_init(|| Mutex::new(0))
}

fn get_plot_zoom() -> &'static Mutex<f32> {
    PLOT_ZOOM.get_or_init(|| Mutex::new(1.0))
}

fn get_plot_orientation() -> &'static Mutex<bool> {
    PLOT_ORIENTATION.get_or_init(|| Mutex::new(true))
}

fn get_plot_pan_x() -> &'static Mutex<f32> {
    PLOT_PAN_X.get_or_init(|| Mutex::new(0.0))
}

fn get_plot_chart_kind() -> &'static Mutex<u8> {
    PLOT_CHART_KIND.get_or_init(|| Mutex::new(2))
}

fn get_plot_variants() -> &'static Mutex<Vec<(u8, String)>> {
    PLOT_VARIANTS.get_or_init(|| Mutex::new(Vec::new()))
}

fn get_plot_show_selector() -> &'static Mutex<bool> {
    PLOT_SHOW_SELECTOR.get_or_init(|| Mutex::new(false))
}

#[no_mangle]
pub extern "C" fn sera_set_plot_sort(mode: i32) {
    if let Ok(mut s) = get_plot_sort().lock() {
        *s = mode;
    }
}

#[no_mangle]
pub extern "C" fn sera_get_plot_sort() -> i32 {
    get_plot_sort().lock().map(|s| *s).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn sera_set_plot_zoom(zoom: f32) {
    if let Ok(mut z) = get_plot_zoom().lock() {
        *z = zoom;
    }
}

#[no_mangle]
pub extern "C" fn sera_get_plot_zoom() -> f32 {
    get_plot_zoom().lock().map(|z| *z).unwrap_or(1.0)
}

#[no_mangle]
pub extern "C" fn sera_set_plot_orientation(vertical: bool) {
    if let Ok(mut o) = get_plot_orientation().lock() {
        *o = vertical;
    }
}

#[no_mangle]
pub extern "C" fn sera_get_plot_orientation() -> bool {
    get_plot_orientation().lock().map(|o| *o).unwrap_or(true)
}

#[no_mangle]
pub extern "C" fn sera_set_plot_pan_x(pan: f32) {
    if let Ok(mut p) = get_plot_pan_x().lock() {
        *p = pan;
    }
}

#[no_mangle]
pub extern "C" fn sera_get_plot_pan_x() -> f32 {
    get_plot_pan_x().lock().map(|p| *p).unwrap_or(0.0)
}

#[no_mangle]
pub extern "C" fn sera_set_plot_chart_kind(kind: u8) {
    if let Ok(mut k) = get_plot_chart_kind().lock() {
        *k = kind;
    }
}

#[no_mangle]
pub extern "C" fn sera_get_plot_chart_kind() -> u8 {
    get_plot_chart_kind().lock().map(|k| *k).unwrap_or(2)
}

#[no_mangle]
pub extern "C" fn sera_add_plot_variant(kind: u8, title: *const c_char) {
    if let Ok(title_str) = unsafe { CStr::from_ptr(title as *const i8).to_str() } {
        if let Ok(mut variants) = get_plot_variants().lock() {
            variants.push((kind, title_str.to_string()));
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_clear_plot_variants() {
    if let Ok(mut variants) = get_plot_variants().lock() {
        variants.clear();
    }
}

#[no_mangle]
pub extern "C" fn sera_get_plot_variants_count() -> usize {
    get_plot_variants().lock().map(|v| v.len()).unwrap_or(0)
}

pub fn get_chart_variants_internal() -> Vec<(u8, String)> {
    get_plot_variants().lock().map(|v| v.clone()).unwrap_or_default()
}

#[no_mangle]
pub extern "C" fn sera_set_plot_show_selector(show: bool) {
    if let Ok(mut s) = get_plot_show_selector().lock() {
        *s = show;
    }
}

#[no_mangle]
pub extern "C" fn sera_get_plot_show_selector() -> bool {
    get_plot_show_selector().lock().map(|s| *s).unwrap_or(false)
}
