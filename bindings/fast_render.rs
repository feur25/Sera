use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uint};
use crate::data::Dataset;

#[repr(C)]
pub struct FastChartConfig {
    pub chart_type: u8,
    pub width: f32,
    pub height: f32,
    pub padding: f32,
    pub vertical: bool,
}

impl FastChartConfig {
    pub fn bar_horizontal() -> Self {
        Self {
            chart_type: 0,
            width: 1000.0,
            height: 800.0,
            padding: 300.0,
            vertical: false,
        }
    }

    pub fn vertical(chart_type: u8) -> Self {
        Self {
            chart_type,
            width: 1600.0,
            height: 900.0,
            padding: 80.0,
            vertical: true,
        }
    }
}

pub struct FastChartRenderer {
    config: FastChartConfig,
    title: String,
    labels: Vec<String>,
    values: Vec<f64>,
    colors: Vec<&'static str>,
}

impl FastChartRenderer {
    pub fn new(config: FastChartConfig, title: &str) -> Self {
        Self {
            config,
            title: title.to_string(),
            labels: Vec::new(),
            values: Vec::new(),
            colors: vec![
                "#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd",
                "#8c564b", "#e377c2", "#7f7f7f", "#bcbd22", "#17becf",
            ],
        }
    }

    pub fn with_data(mut self, labels: Vec<String>, values: Vec<f64>) -> Self {
        self.labels = labels;
        self.values = values;
        self
    }

    #[inline]
    pub fn from_dataset(config: FastChartConfig, title: &str, data: Dataset<f64>) -> Self {
        let labels = data.to_labels_vec();
        let values = data.to_values_vec();
        Self::new(config, title).with_data(labels, values)
    }

    fn render_svg_axes(&self, svg: &mut String, pad: i32, plot_width: i32, plot_height: i32, max_val: f64) {
        for i in 0..=5 {
            let y = pad + ((1.0 - i as f64 / 5.0) * plot_height as f64) as i32;
            let val = (max_val / 5.0) * i as f64;
            svg.push_str(&format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" class=\"grid\"/><text x=\"{}\" y=\"{}\" class=\"label\" text-anchor=\"end\">{:.1}</text>",
                pad - 5, y, pad + plot_width, y, pad - 10, y + 4, val
            ));
        }

        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" class=\"axis\"/><line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" class=\"axis\"/>",
            pad, pad, pad, pad + plot_height, pad, pad + plot_height, pad + plot_width, pad + plot_height
        ));
    }

    fn render_chart(&self, svg: &mut String, pad: i32, plot_width: i32, plot_height: i32, max_val: f64) {
        crate::plot::default::render_chart_by_type(
            self.config.chart_type,
            svg,
            &self.values,
            &self.colors,
            pad,
            plot_width,
            plot_height,
            max_val,
            self.config.vertical,
        );
    }

    pub fn render_svg(&self) -> String {
        let w = self.config.width as i32;
        let h = self.config.height as i32;
        let pad = self.config.padding as i32;

        let mut svg = String::with_capacity(65536);
        svg.push_str(&format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\" style=\"background:white\">",
            w, h, w, h
        ));

        svg.push_str("<defs><style>text{font-family:Arial,sans-serif;font-size:12px}.title{font-size:18px;font-weight:bold}.label{font-size:11px;fill:#666}.axis{stroke:#333;stroke-width:1.5}.grid{stroke:#e0e0e0;stroke-width:0.5}</style></defs>");

        svg.push_str(&format!(
            "<text x=\"{}\" y=\"30\" class=\"title\" text-anchor=\"middle\">{}</text>",
            w / 2, self.title
        ));

        let max_val = self.values.iter().copied().fold(0.0_f64, f64::max).max(1.0);
        let plot_width = (self.config.width - self.config.padding * 2.0).max(1.0);
        let plot_height = (self.config.height - self.config.padding * 2.0).max(1.0);

        self.render_svg_axes(&mut svg, pad, plot_width as i32, plot_height as i32, max_val);
        self.render_chart(&mut svg, pad, plot_width as i32, plot_height as i32, max_val);

        svg.push_str("</svg>");
        svg
    }

}

#[no_mangle]
pub extern "C" fn sera_create_fast_chart(
    chart_type: c_uint,
    title: *const c_char,
    labels: *const *const c_char,
    values: *const f64,
    count: c_uint,
) -> *mut c_char {
    if title.is_null() || labels.is_null() || values.is_null() {
        return std::ptr::null_mut();
    }

    let config = match chart_type {
        0 => FastChartConfig::bar_horizontal(),
        _ => FastChartConfig::vertical(chart_type as u8),
    };

    let title_str = unsafe { CStr::from_ptr(title).to_string_lossy().to_string() };

    let mut chart_labels = Vec::with_capacity(count as usize);
    for i in 0..count as usize {
        let label_ptr = unsafe { *labels.add(i) };
        let label_str = if !label_ptr.is_null() {
            unsafe { CStr::from_ptr(label_ptr).to_string_lossy().to_string() }
        } else {
            String::new()
        };
        chart_labels.push(label_str);
    }

    let chart_values = unsafe { std::slice::from_raw_parts(values, count as usize) }.to_vec();

    let renderer = FastChartRenderer::new(config, &title_str).with_data(chart_labels, chart_values);
    let svg = renderer.render_svg();

    let c_str = CString::new(svg).unwrap_or_else(|_| CString::new("").unwrap());
    c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn sera_fast_chart_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}


