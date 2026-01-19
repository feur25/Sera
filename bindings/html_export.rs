use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub struct HtmlExportConfig {
    pub width: i32,
    pub height: i32,
    pub title: String,
    pub theme: HtmlTheme,
}

#[repr(u8)]
pub enum HtmlTheme {
    Light = 0,
    Dark = 1,
    Professional = 2,
}

pub struct HtmlExporter {
    config: HtmlExportConfig,
    svg_content: String,
    labels: Vec<String>,
    values: Vec<f64>,
}

impl HtmlExportConfig {
    pub fn default() -> Self {
        Self {
            width: 1200,
            height: 800,
            title: "SeraPlot Chart".to_string(),
            theme: HtmlTheme::Light,
        }
    }

    pub fn with_dimensions(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_theme(mut self, theme: HtmlTheme) -> Self {
        self.theme = theme;
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }
}

impl HtmlExporter {
    pub fn new(config: HtmlExportConfig) -> Self {
        Self {
            config,
            svg_content: String::new(),
            labels: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn with_svg(mut self, svg: String) -> Self {
        self.svg_content = svg;
        self
    }

    pub fn with_data(mut self, labels: Vec<String>, values: Vec<f64>) -> Self {
        self.labels = labels;
        self.values = values;
        self
    }

    pub fn build_html(&self) -> String {
        let theme_bg = match self.config.theme {
            HtmlTheme::Light => "#ffffff",
            HtmlTheme::Dark => "#1e1e1e",
            HtmlTheme::Professional => "#f5f5f5",
        };

        let theme_text = match self.config.theme {
            HtmlTheme::Light => "#333333",
            HtmlTheme::Dark => "#ffffff",
            HtmlTheme::Professional => "#2c3e50",
        };

        let theme_accent = match self.config.theme {
            HtmlTheme::Light => "#1f77b4",
            HtmlTheme::Dark => "#00d4ff",
            HtmlTheme::Professional => "#2980b9",
        };

        let mut html = String::with_capacity(65536);

        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"en\">\n");
        html.push_str("<head>\n");
        html.push_str("  <meta charset=\"UTF-8\">\n");
        html.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str(&format!("  <title>{}</title>\n", self.config.title));
        html.push_str("  <style>\n");
        html.push_str("    * { margin: 0; padding: 0; box-sizing: border-box; }\n");
        html.push_str(&format!("    body {{\n"));
        html.push_str(&format!("      background-color: {};\n", theme_bg));
        html.push_str(&format!("      color: {};\n", theme_text));
        html.push_str("      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;\n");
        html.push_str("      line-height: 1.6;\n");
        html.push_str("    }\n");
        html.push_str("    .container {\n");
        html.push_str("      max-width: 1400px;\n");
        html.push_str("      margin: 0 auto;\n");
        html.push_str("      padding: 40px 20px;\n");
        html.push_str("    }\n");
        html.push_str("    .header {\n");
        html.push_str("      text-align: center;\n");
        html.push_str("      margin-bottom: 40px;\n");
        html.push_str("      border-bottom: 2px solid " );
        html.push_str(theme_accent);
        html.push_str(";\n");
        html.push_str("      padding-bottom: 20px;\n");
        html.push_str("    }\n");
        html.push_str("    h1 {\n");
        html.push_str(&format!("      color: {};\n", theme_accent));
        html.push_str("      font-size: 2.5em;\n");
        html.push_str("      margin-bottom: 10px;\n");
        html.push_str("    }\n");
        html.push_str("    .subtitle {\n");
        html.push_str(&format!("      color: {};\n", if matches!(self.config.theme, HtmlTheme::Dark) { "#aaa" } else { "#666" }));
        html.push_str("      font-size: 1.1em;\n");
        html.push_str("    }\n");
        html.push_str("    .chart-container {\n");
        html.push_str(&format!("      background-color: {};\n", if matches!(self.config.theme, HtmlTheme::Dark) { "#2a2a2a" } else { "#fff" }));
        html.push_str("      border-radius: 8px;\n");
        html.push_str("      padding: 20px;\n");
        html.push_str("      box-shadow: 0 2px 8px rgba(0,0,0,0.1);\n");
        html.push_str("      overflow: auto;\n");
        html.push_str("    }\n");
        html.push_str("    svg {\n");
        html.push_str("      width: 100%;\n");
        html.push_str("      height: auto;\n");
        html.push_str("    }\n");
        html.push_str("    .stats {\n");
        html.push_str("      display: grid;\n");
        html.push_str("      grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));\n");
        html.push_str("      gap: 20px;\n");
        html.push_str("      margin-top: 40px;\n");
        html.push_str("    }\n");
        html.push_str("    .stat-card {\n");
        html.push_str(&format!("      background-color: {};\n", if matches!(self.config.theme, HtmlTheme::Dark) { "#2a2a2a" } else { "#f9f9f9" }));
        html.push_str("      border-left: 4px solid ");
        html.push_str(theme_accent);
        html.push_str(";\n");
        html.push_str("      padding: 20px;\n");
        html.push_str("      border-radius: 4px;\n");
        html.push_str("    }\n");
        html.push_str("    .stat-label {\n");
        html.push_str("      font-size: 0.9em;\n");
        html.push_str(&format!("      color: {};\n", if matches!(self.config.theme, HtmlTheme::Dark) { "#888" } else { "#999" }));
        html.push_str("      margin-bottom: 8px;\n");
        html.push_str("    }\n");
        html.push_str("    .stat-value {\n");
        html.push_str(&format!("      color: {};\n", theme_accent));
        html.push_str("      font-size: 1.8em;\n");
        html.push_str("      font-weight: bold;\n");
        html.push_str("    }\n");
        html.push_str("    .footer {\n");
        html.push_str("      text-align: center;\n");
        html.push_str("      margin-top: 40px;\n");
        html.push_str("      padding-top: 20px;\n");
        html.push_str(&format!("      border-top: 1px solid {};\n", if matches!(self.config.theme, HtmlTheme::Dark) { "#444" } else { "#eee" }));
        html.push_str(&format!("      color: {};\n", if matches!(self.config.theme, HtmlTheme::Dark) { "#888" } else { "#999" }));
        html.push_str("      font-size: 0.9em;\n");
        html.push_str("    }\n");
        html.push_str("  </style>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <div class=\"container\">\n");
        html.push_str("    <div class=\"header\">\n");
        html.push_str(&format!("      <h1>{}</h1>\n", self.config.title));
        html.push_str("      <div class=\"subtitle\">Generated by SeraPlot Fast</div>\n");
        html.push_str("    </div>\n");
        html.push_str("    <div class=\"chart-container\">\n");

        if !self.svg_content.is_empty() {
            html.push_str(&self.svg_content);
        } else {
            html.push_str("      <p>No chart data</p>\n");
        }

        html.push_str("    </div>\n");

        if !self.values.is_empty() {
            html.push_str("    <div class=\"stats\">\n");
            
            let max_val = self.values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let min_val = self.values.iter().copied().fold(f64::INFINITY, f64::min);
            let avg_val = self.values.iter().sum::<f64>() / self.values.len() as f64;
            let sum_val = self.values.iter().sum::<f64>();

            html.push_str("      <div class=\"stat-card\">\n");
            html.push_str("        <div class=\"stat-label\">Count</div>\n");
            html.push_str(&format!("        <div class=\"stat-value\">{}</div>\n", self.values.len()));
            html.push_str("      </div>\n");

            html.push_str("      <div class=\"stat-card\">\n");
            html.push_str("        <div class=\"stat-label\">Max</div>\n");
            html.push_str(&format!("        <div class=\"stat-value\">{:.1}</div>\n", max_val));
            html.push_str("      </div>\n");

            html.push_str("      <div class=\"stat-card\">\n");
            html.push_str("        <div class=\"stat-label\">Min</div>\n");
            html.push_str(&format!("        <div class=\"stat-value\">{:.1}</div>\n", min_val));
            html.push_str("      </div>\n");

            html.push_str("      <div class=\"stat-card\">\n");
            html.push_str("        <div class=\"stat-label\">Average</div>\n");
            html.push_str(&format!("        <div class=\"stat-value\">{:.1}</div>\n", avg_val));
            html.push_str("      </div>\n");

            html.push_str("      <div class=\"stat-card\">\n");
            html.push_str("        <div class=\"stat-label\">Sum</div>\n");
            html.push_str(&format!("        <div class=\"stat-value\">{:.0}</div>\n", sum_val));
            html.push_str("      </div>\n");

            html.push_str("    </div>\n");
        }

        html.push_str("    <div class=\"footer\">\n");
        html.push_str("      <p>Generated by SeraPlot Fast HTML Exporter | Instant SVG Rendering</p>\n");
        html.push_str("    </div>\n");
        html.push_str("  </div>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");

        html
    }

    pub fn export_to_file(&self, path: &str) -> Result<(), std::io::Error> {
        let html = self.build_html();
        std::fs::write(path, html)
    }
}

#[no_mangle]
pub extern "C" fn sera_html_export_create(
    width: i32,
    height: i32,
    title: *const c_char,
    theme: u8,
) -> *mut HtmlExporter {
    let title_str = unsafe { CStr::from_ptr(title).to_string_lossy().to_string() };
    
    let theme_obj = match theme {
        1 => HtmlTheme::Dark,
        2 => HtmlTheme::Professional,
        _ => HtmlTheme::Light,
    };

    let config = HtmlExportConfig {
        width,
        height,
        title: title_str,
        theme: theme_obj,
    };

    Box::into_raw(Box::new(HtmlExporter::new(config)))
}

#[no_mangle]
pub extern "C" fn sera_html_export_set_svg(exporter: *mut HtmlExporter, svg: *const c_char) {
    if !exporter.is_null() && !svg.is_null() {
        unsafe {
            let svg_str = CStr::from_ptr(svg).to_string_lossy().to_string();
            (*exporter).svg_content = svg_str;
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_html_export_set_data(
    exporter: *mut HtmlExporter,
    labels: *const *const c_char,
    values: *const f64,
    count: usize,
) {
    if exporter.is_null() || labels.is_null() || values.is_null() {
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
        (*exporter).labels = chart_labels;
        (*exporter).values = chart_values;
    }
}

#[no_mangle]
pub extern "C" fn sera_html_export_build(exporter: *const HtmlExporter) -> *mut c_char {
    if exporter.is_null() {
        return std::ptr::null_mut();
    }

    let html = unsafe { (*exporter).build_html() };
    let c_str = CString::new(html).unwrap_or_else(|_| CString::new("").unwrap());
    c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn sera_html_export_save(
    exporter: *const HtmlExporter,
    path: *const c_char,
) -> bool {
    if exporter.is_null() || path.is_null() {
        return false;
    }

    let path_str = unsafe { CStr::from_ptr(path).to_string_lossy().to_string() };
    
    match unsafe { (*exporter).export_to_file(&path_str) } {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[no_mangle]
pub extern "C" fn sera_html_export_free(exporter: *mut HtmlExporter) {
    if !exporter.is_null() {
        unsafe {
            let _ = Box::from_raw(exporter);
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_string_free_html(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}
