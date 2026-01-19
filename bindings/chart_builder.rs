use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(u8)]
pub enum ChartType {
    BarVertical = 0,
    BarHorizontal = 1,
    Line = 2,
    Scatter = 3,
}

pub struct ChartBuilder {
    chart_type: ChartType,
    title: String,
    labels: Vec<String>,
    values: Vec<f64>,
}

impl ChartBuilder {
    pub fn new(chart_type: u8) -> Self {
        let ctype = match chart_type {
            0 => ChartType::BarVertical,
            1 => ChartType::BarHorizontal,
            2 => ChartType::Line,
            _ => ChartType::Scatter,
        };

        Self {
            chart_type: ctype,
            title: String::new(),
            labels: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn with_data(mut self, labels: Vec<String>, values: Vec<f64>) -> Self {
        self.labels = labels;
        self.values = values;
        self
    }

    pub fn render(&self) -> String {
        match self.chart_type {
            ChartType::BarHorizontal => self.render_bar_horizontal(),
            ChartType::BarVertical => self.render_bar_vertical(),
            ChartType::Line => self.render_line(),
            ChartType::Scatter => self.render_scatter(),
        }
    }

    fn render_bar_horizontal(&self) -> String {
        let w = 1000i32;
        let h = 800i32;
        let pad = 300i32;
        let max_val = self.values.iter().copied().fold(0.0_f64, f64::max).max(1.0);
        let plot_width = w - pad * 2;
        let plot_height = h - pad * 2;
        let bar_spacing = plot_height as f64 / (self.values.len() as f64).max(1.0);
        let bar_thickness = (bar_spacing * 0.7).min(20.0);

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

        for i in 0..=5 {
            let x = pad + (i as f64 / 5.0 * plot_width as f64) as i32;
            let val = (max_val / 5.0) * i as f64;
            svg.push_str(&format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" class=\"grid\"/><text x=\"{}\" y=\"{}\" class=\"label\" text-anchor=\"middle\">{:.1}</text>",
                x, pad, x, pad + plot_height, x, pad + plot_height + 20, val
            ));
        }

        svg.push_str(&format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" class=\"axis\"/><line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" class=\"axis\"/>",
            pad, pad, pad, pad + plot_height, pad, pad + plot_height, pad + plot_width, pad + plot_height
        ));

        let colors = vec![
            "#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd",
            "#8c564b", "#e377c2", "#7f7f7f", "#bcbd22", "#17becf",
        ];

        for (idx, &val) in self.values.iter().enumerate() {
            let norm = (val / max_val).min(1.0);
            let bar_length = (norm * plot_width as f64) as i32;
            let y_center = pad + (idx as f64 * bar_spacing + bar_spacing / 2.0) as i32;
            let color = colors[idx % colors.len()];

            svg.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"#ccc\" stroke-width=\"0.5\"/>",
                pad, y_center - (bar_thickness as i32 / 2), bar_length, bar_thickness as i32, color
            ));

            if let Some(label) = self.labels.get(idx) {
                let label_text = if label.len() > 35 {
                    format!("{}...", &label[..32])
                } else {
                    label.clone()
                };
                svg.push_str(&format!(
                    "<text x=\"{}\" y=\"{}\" class=\"label\" text-anchor=\"end\">{}</text>",
                    pad - 8, y_center + 4, label_text
                ));
            }
        }

        svg.push_str("</svg>");
        svg
    }

    fn render_bar_vertical(&self) -> String {
        let w = 800i32;
        let h = 600i32;
        let pad = 60i32;
        let max_val = self.values.iter().copied().fold(0.0_f64, f64::max).max(1.0);
        let plot_width = w - pad * 2;
        let plot_height = h - pad * 2;
        let bar_spacing = plot_width as f64 / (self.values.len() as f64).max(1.0);
        let bar_thickness = (bar_spacing * 0.7).min(20.0);

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

        for i in 0..=5 {
            let y = pad + plot_height - (i as f64 / 5.0 * plot_height as f64) as i32;
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

        let colors = vec![
            "#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd",
            "#8c564b", "#e377c2", "#7f7f7f", "#bcbd22", "#17becf",
        ];

        for (idx, &val) in self.values.iter().enumerate() {
            let norm = (val / max_val).min(1.0);
            let bar_height = (norm * plot_height as f64) as i32;
            let x_center = pad + (idx as f64 * bar_spacing + bar_spacing / 2.0) as i32;
            let color = colors[idx % colors.len()];

            svg.push_str(&format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"#ccc\" stroke-width=\"0.5\"/>",
                x_center - (bar_thickness as i32 / 2), pad + plot_height - bar_height, bar_thickness as i32, bar_height, color
            ));

            if let Some(label) = self.labels.get(idx) {
                let label_text = if label.len() > 10 {
                    format!("{}...", &label[..7])
                } else {
                    label.clone()
                };
                svg.push_str(&format!(
                    "<text x=\"{}\" y=\"{}\" class=\"label\" text-anchor=\"middle\">{}</text>",
                    x_center, pad + plot_height + 20, label_text
                ));
            }
        }

        svg.push_str("</svg>");
        svg
    }

    fn render_line(&self) -> String {
        let w = 800i32;
        let h = 600i32;
        let pad = 60i32;
        let max_val = self.values.iter().copied().fold(0.0_f64, f64::max).max(1.0);
        let plot_width = w - pad * 2;
        let plot_height = h - pad * 2;

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

        if self.values.len() > 1 {
            let mut path = String::with_capacity(4096);
            for (idx, &val) in self.values.iter().enumerate() {
                let norm_x = (idx as f64) / (self.values.len() as f64 - 1.0).max(1.0);
                let norm_y = (val / max_val).min(1.0);
                let x = pad + (norm_x * plot_width as f64) as i32;
                let y = pad + plot_height - (norm_y * plot_height as f64) as i32;

                if idx == 0 {
                    path.push_str(&format!("M{},{}", x, y));
                } else {
                    path.push_str(&format!("L{},{}", x, y));
                }
            }
            svg.push_str(&format!(
                "<path d=\"{}\" stroke=\"#1f77b4\" stroke-width=\"2\" fill=\"none\"/>",
                path
            ));
        }

        let step = (self.values.len() as f64 / 50.0).max(1.0) as usize;
        for (idx, &val) in self.values.iter().enumerate().step_by(step.max(1)) {
            let norm_x = (idx as f64) / (self.values.len() as f64).max(1.0);
            let norm_y = (val / max_val).min(1.0);
            let x = pad + (norm_x * plot_width as f64) as i32;
            let y = pad + plot_height - (norm_y * plot_height as f64) as i32;

            svg.push_str(&format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"3\" fill=\"#1f77b4\" stroke=\"white\" stroke-width=\"1\"/>",
                x, y
            ));
        }

        svg.push_str("</svg>");
        svg
    }

    fn render_scatter(&self) -> String {
        let w = 800i32;
        let h = 600i32;
        let pad = 60i32;
        let max_val = self.values.iter().copied().fold(0.0_f64, f64::max).max(1.0);
        let plot_width = w - pad * 2;
        let plot_height = h - pad * 2;

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

        let colors = vec![
            "#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd",
            "#8c564b", "#e377c2", "#7f7f7f", "#bcbd22", "#17becf",
        ];

        let step = (self.values.len() as f64 / plot_width as f64).max(1.0) as usize;
        for (idx, &val) in self.values.iter().enumerate().step_by(step.max(1)) {
            let norm_x = (idx as f64) / (self.values.len() as f64).max(1.0);
            let norm_y = (val / max_val).min(1.0);
            let x = pad + (norm_x * plot_width as f64) as i32;
            let y = pad + plot_height - (norm_y * plot_height as f64) as i32;
            let color = colors[idx % colors.len()];

            svg.push_str(&format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"4\" fill=\"{}\" stroke=\"white\" stroke-width=\"1\"/>",
                x, y, color
            ));
        }

        svg.push_str("</svg>");
        svg
    }
}

#[no_mangle]
pub extern "C" fn sera_builder_create(chart_type: u8) -> *mut ChartBuilder {
    Box::into_raw(Box::new(ChartBuilder::new(chart_type)))
}

#[no_mangle]
pub extern "C" fn sera_builder_set_title(builder: *mut ChartBuilder, title: *const c_char) {
    if !builder.is_null() && !title.is_null() {
        unsafe {
            let title_str = CStr::from_ptr(title).to_string_lossy().to_string();
            (*builder).title = title_str;
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_builder_set_data(
    builder: *mut ChartBuilder,
    labels: *const *const c_char,
    values: *const f64,
    count: usize,
) {
    if builder.is_null() || labels.is_null() || values.is_null() {
        return;
    }

    let mut chart_labels = Vec::with_capacity(count);
    for i in 0..count {
        let label_ptr = unsafe { *labels.add(i) };
        let label_str = if !label_ptr.is_null() {
            unsafe { CStr::from_ptr(label_ptr).to_string_lossy().to_string() }
        } else {
            String::new()
        };
        chart_labels.push(label_str);
    }

    let chart_values = unsafe { std::slice::from_raw_parts(values, count) }.to_vec();

    unsafe {
        (*builder).labels = chart_labels;
        (*builder).values = chart_values;
    }
}

#[no_mangle]
pub extern "C" fn sera_builder_render(builder: *const ChartBuilder) -> *mut c_char {
    if builder.is_null() {
        return std::ptr::null_mut();
    }

    let svg = unsafe { (*builder).render() };
    let c_str = CString::new(svg).unwrap_or_else(|_| CString::new("").unwrap());
    c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn sera_builder_free(builder: *mut ChartBuilder) {
    if !builder.is_null() {
        unsafe {
            let _ = Box::from_raw(builder);
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_string_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}
