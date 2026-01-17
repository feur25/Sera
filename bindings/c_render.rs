use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub struct ChartData {
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub chart_type: String,
    pub traces: Vec<(String, Vec<f64>, Vec<f64>)>,
}

pub fn render_svg(chart: &ChartData) -> String {
    let width = 800.0;
    let height = 400.0;
    let padding = 60.0;
    let plot_width = width - padding * 2.0;
    let plot_height = height - padding * 2.0;

    let mut all_x = Vec::new();
    let mut all_y = Vec::new();
    for (_, x, y) in &chart.traces {
        all_x.extend(x);
        all_y.extend(y);
    }

    let min_x = all_x.iter().copied().fold(f64::INFINITY, f64::min);
    let max_x = all_x.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let min_y = all_y.iter().copied().fold(f64::INFINITY, f64::min);
    let max_y = all_y.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    let range_x = if max_x > min_x { max_x - min_x } else { 1.0 };
    let range_y = if max_y > min_y { max_y - min_y } else { 1.0 };
    let scale_x = plot_width / range_x;
    let scale_y = plot_height / range_y;

    let mut svg = String::with_capacity(16384);
    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
        width as i32, height as i32, width as i32, height as i32
    ));

    svg.push_str(r#"<defs><style>text{font-family:Arial}.title{font-size:16px;font-weight:bold}.lbl{font-size:11px}.grid{stroke:#eee;stroke-width:0.5}.axis{stroke:#000;stroke-width:1.5}.line{stroke-width:2;fill:none}.bar{}.dot{r:4}</style></defs>"#);
    svg.push_str(&format!(r#"<rect width="{}" height="{}" fill="white"/>"#, width as i32, height as i32));
    svg.push_str(&format!(r#"<text x="{}" y="30" class="title" text-anchor="middle">{}</text>"#, (width/2.0) as i32, chart.title));

    for i in 0..=10 {
        let y = padding + (i as f64 / 10.0) * plot_height;
        svg.push_str(&format!(r#"<line x1="{}" y1="{}" x2="{}" y2="{}" class="grid"/>"#, padding as i32, y as i32, (padding + plot_width) as i32, y as i32));
    }

    svg.push_str(&format!(r#"<line x1="{}" y1="{}" x2="{}" y2="{}" class="axis"/><line x1="{}" y1="{}" x2="{}" y2="{}" class="axis"/>"#, padding as i32, padding as i32, padding as i32, (padding + plot_height) as i32, padding as i32, (padding + plot_height) as i32, (padding + plot_width) as i32, (padding + plot_height) as i32));

    svg.push_str(&format!(r#"<text x="{}" y="{}" class="lbl" text-anchor="middle">{}</text>"#, (width/2.0) as i32, (height-10.0) as i32, chart.x_label));
    svg.push_str(&format!(r#"<text x="20" y="{}" class="lbl" text-anchor="middle" transform="rotate(-90 20 {})">{}</text>"#, (padding + plot_height/2.0) as i32, (padding + plot_height/2.0) as i32, chart.y_label));

    let colors = vec!["#2196F3", "#FF9800", "#4CAF50", "#F44336", "#9C27B0"];

    for (idx, (_, x, y)) in chart.traces.iter().enumerate() {
        let color = colors[idx % colors.len()];

        match chart.chart_type.as_str() {
            "line" => {
                if x.len() > 1 {
                    let mut points = String::with_capacity(256);
                    for (i, (xi, yi)) in x.iter().zip(y.iter()).enumerate() {
                        let px = padding + (xi - min_x) * scale_x;
                        let py = padding + plot_height - (yi - min_y) * scale_y;
                        if i == 0 {
                            points.push_str(&format!("{},{}", px as i32, py as i32));
                        } else {
                            points.push_str(&format!(" {},{}", px as i32, py as i32));
                        }
                    }
                    svg.push_str(&format!(r#"<polyline points="{}" stroke="{}" class="line"/>"#, points, color));
                }
            },
            "scatter" => {
                for (xi, yi) in x.iter().zip(y.iter()) {
                    let px = padding + (xi - min_x) * scale_x;
                    let py = padding + plot_height - (yi - min_y) * scale_y;
                    svg.push_str(&format!(r#"<circle cx="{}" cy="{}" r="4" fill="{}" opacity="0.7"/>"#, px as i32, py as i32, color));
                }
            },
            "bar" => {
                let bar_width = plot_width / x.len().max(1) as f64 * 0.8;
                let bar_spacing = plot_width / x.len().max(1) as f64;
                for (i, yi) in y.iter().enumerate() {
                    let bar_x = padding + i as f64 * bar_spacing + (bar_spacing - bar_width) / 2.0;
                    let bar_h = (yi - min_y) * scale_y;
                    let bar_y = padding + plot_height - bar_h;
                    svg.push_str(&format!(r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" class="bar"/>"#, bar_x as i32, bar_y as i32, bar_width as i32, bar_h as i32, color));
                }
            },
            _ => {}
        }
    }

    svg.push_str("</svg>");
    svg
}

#[no_mangle]
pub extern "C" fn seraplot_create_line_chart(
    title: *const c_char,
    x_label: *const c_char,
    y_label: *const c_char,
    trace_name: *const c_char,
    x_data: *const f64,
    y_data: *const f64,
    data_len: usize,
) -> *mut c_char {
    let title = unsafe { CStr::from_ptr(title).to_string_lossy().to_string() };
    let x_label = unsafe { CStr::from_ptr(x_label).to_string_lossy().to_string() };
    let y_label = unsafe { CStr::from_ptr(y_label).to_string_lossy().to_string() };
    let trace_name = unsafe { CStr::from_ptr(trace_name).to_string_lossy().to_string() };

    let x_vec: Vec<f64> = unsafe { std::slice::from_raw_parts(x_data, data_len) }.to_vec();
    let y_vec: Vec<f64> = unsafe { std::slice::from_raw_parts(y_data, data_len) }.to_vec();

    let chart = ChartData {
        title,
        x_label,
        y_label,
        chart_type: "line".to_string(),
        traces: vec![(trace_name, x_vec, y_vec)],
    };

    let svg = render_svg(&chart);
    let c_str = CString::new(svg).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn seraplot_create_bar_chart(
    title: *const c_char,
    labels: *const *const c_char,
    values: *const f64,
    count: usize,
) -> *mut c_char {
    let title = unsafe { CStr::from_ptr(title).to_string_lossy().to_string() };
    let values_vec: Vec<f64> = unsafe { std::slice::from_raw_parts(values, count) }.to_vec();

    let _labels: Vec<String> = unsafe {
        std::slice::from_raw_parts(labels, count)
            .iter()
            .map(|&l| CStr::from_ptr(l).to_string_lossy().to_string())
            .collect()
    };

    let x_indices: Vec<f64> = (0..count).map(|i| i as f64).collect();

    let chart = ChartData {
        title: title.clone(),
        x_label: "Category".to_string(),
        y_label: "Value".to_string(),
        chart_type: "bar".to_string(),
        traces: vec![(title, x_indices, values_vec)],
    };

    let html = render_svg(&chart);
    let c_str = CString::new(html).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn seraplot_create_scatter_chart(
    title: *const c_char,
    x_data: *const f64,
    y_data: *const f64,
    data_len: usize,
) -> *mut c_char {
    let title = unsafe { CStr::from_ptr(title).to_string_lossy().to_string() };
    let x_vec: Vec<f64> = unsafe { std::slice::from_raw_parts(x_data, data_len) }.to_vec();
    let y_vec: Vec<f64> = unsafe { std::slice::from_raw_parts(y_data, data_len) }.to_vec();

    let chart = ChartData {
        title: title.clone(),
        x_label: "X".to_string(),
        y_label: "Y".to_string(),
        chart_type: "scatter".to_string(),
        traces: vec![(title, x_vec, y_vec)],
    };

    let svg = render_svg(&chart);
    let c_str = CString::new(svg).unwrap();
    c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn seraplot_mean(data: *const f64, len: usize) -> f64 {
    if len == 0 {
        return 0.0;
    }
    let slice = unsafe { std::slice::from_raw_parts(data, len) };
    crate::mean(slice).unwrap_or(0.0)
}

#[no_mangle]
pub extern "C" fn seraplot_median(data: *const f64, len: usize) -> f64 {
    if len == 0 {
        return 0.0;
    }
    let slice = unsafe { std::slice::from_raw_parts(data, len) };
    crate::median(slice.to_vec()).unwrap_or(0.0)
}

#[no_mangle]
pub extern "C" fn seraplot_std_dev(data: *const f64, len: usize) -> f64 {
    if len == 0 {
        return 0.0;
    }
    let slice = unsafe { std::slice::from_raw_parts(data, len) };
    crate::std_dev(slice).unwrap_or(0.0)
}

#[no_mangle]
pub extern "C" fn seraplot_min(data: *const f64, len: usize) -> f64 {
    if len == 0 {
        return 0.0;
    }
    unsafe { std::slice::from_raw_parts(data, len) }
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min)
}

#[no_mangle]
pub extern "C" fn seraplot_max(data: *const f64, len: usize) -> f64 {
    if len == 0 {
        return 0.0;
    }
    unsafe { std::slice::from_raw_parts(data, len) }
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max)
}

#[no_mangle]
pub extern "C" fn seraplot_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}
