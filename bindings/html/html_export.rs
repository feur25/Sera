use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::bindings::utils::state_export::{ChartState, StateStorage};
use super::html_template::HtmlTemplate;

pub struct HtmlExportConfig {
    pub width: i32,
    pub height: i32,
    pub title: String,
    pub theme: HtmlTheme,
    pub include_state: bool,
    pub include_controls: bool,
}

#[repr(u8)]
pub enum HtmlTheme {
    Light = 0,
    Dark = 1,
    Professional = 2,
}

impl std::fmt::Display for HtmlTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HtmlTheme::Light => write!(f, "light"),
            HtmlTheme::Dark => write!(f, "dark"),
            HtmlTheme::Professional => write!(f, "professional"),
        }
    }
}

pub struct HtmlExporter {
    config: HtmlExportConfig,
    svg_content: String,
    labels: Vec<String>,
    values: Vec<f64>,
    state: Option<ChartState>,
}

impl HtmlExportConfig {
    pub fn default() -> Self {
        Self {
            width: 1200,
            height: 800,
            title: "SeraPlot Chart".to_string(),
            theme: HtmlTheme::Light,
            include_state: true,
            include_controls: true,
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

    pub fn with_state_export(mut self, include: bool) -> Self {
        self.include_state = include;
        self
    }

    pub fn with_controls(mut self, include: bool) -> Self {
        self.include_controls = include;
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
            state: None,
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

    pub fn with_state(mut self, state: ChartState) -> Self {
        self.state = Some(state);
        self
    }

    fn get_theme_colors(&self) -> (&'static str, &'static str, &'static str) {
        match self.config.theme {
            HtmlTheme::Light => ("#ffffff", "#333333", "#1f77b4"),
            HtmlTheme::Dark => ("#1e1e1e", "#ffffff", "#00d4ff"),
            HtmlTheme::Professional => ("#f5f5f5", "#2c3e50", "#2980b9"),
        }
    }

    fn build_state_json(&self) -> String {
        self.state
            .as_ref()
            .and_then(|s| s.to_json().ok())
            .unwrap_or_else(|| "{}".to_string())
    }

    pub fn build_html(&self) -> String {
        use super::assets::Assets;
        let state_json = self.build_state_json();

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>SeraPlot</title>
  <style>
{}
  </style>
</head>
<body>
  <div class="chart-container">
    <div class="zoom-controls">
      <button class="zoom-btn" onclick="zoomOut()" title="Zoom Out">−</button>
      <button class="zoom-btn" onclick="resetZoom()" title="Reset">⊙</button>
      <button class="zoom-btn" onclick="zoomIn()" title="Zoom In">+</button>
      <button class="zoom-btn" onclick="resetBoxSelection()" title="Reset Selection">✕</button>
      <button class="zoom-btn" onclick="openPopup()" title="Fullscreen">⛶</button>
    </div>
    {}
  </div>
  
  <div id="seraplot-modal" class="modal">
    <div class="modal-content">
      <div class="modal-header">
        <h2>SeraPlot Chart</h2>
        <button id="modal-close-btn" class="modal-close">&times;</button>
      </div>
      <div class="modal-body">
        <svg id="modal-svg"></svg>
      </div>
    </div>
  </div>
  
  <script>
    window.__SERAPLOT_STATE__ = {};
  </script>
  <script>
{}
  </script>
</body>
</html>"#,
            Assets::STYLE_CSS,
            self.svg_content,
            state_json,
            Assets::SCRIPT_JS
        )
    }

    pub fn export_assets(&self, base_path: &str) -> Result<(), std::io::Error> {
        use super::assets::Assets;
        
        let style_path = format!("{}/style.css", base_path);
        let script_path = format!("{}/script.js", base_path);
        
        std::fs::write(&style_path, Assets::STYLE_CSS)?;
        std::fs::write(&script_path, Assets::SCRIPT_JS)?;
        
        Ok(())
    }

    pub fn export_to_file_background(self, path: String) {
        std::thread::spawn(move || {
            use std::path::Path;
            
            let base_dir = Path::new(&path)
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_str()
                .unwrap_or(".");
            
            if let Err(e) = self.export_assets(base_dir) {
                eprintln!("✗ Asset export failed: {}", e);
                return;
            }
            
            let html = self.build_html();
            if let Err(e) = std::fs::write(&path, html) {
                eprintln!("✗ HTML export failed: {}", e);
            } else {
                println!("✓ Exported to {}", path);
            }
        });
    }

    pub fn export_to_file(&self, path: &str) -> Result<(), std::io::Error> {
        use std::path::Path;
        
        let base_dir = Path::new(path)
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_str()
            .unwrap_or(".");
        
        self.export_assets(base_dir)?;
        
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
        include_state: true,
        include_controls: true,
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
pub extern "C" fn sera_html_export_set_state_json(
    exporter: *mut HtmlExporter,
    state_json: *const c_char,
) -> bool {
    if exporter.is_null() || state_json.is_null() {
        return false;
    }

    let json_str = unsafe { CStr::from_ptr(state_json).to_string_lossy().to_string() };
    if let Ok(state) = ChartState::from_json(&json_str) {
        unsafe {
            (*exporter).state = Some(state);
        }
        true
    } else {
        false
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

#[no_mangle]
pub extern "C" fn sera_state_storage_create() -> *mut StateStorage {
    Box::into_raw(Box::new(StateStorage::new()))
}

#[no_mangle]
pub extern "C" fn sera_state_storage_save(
    storage: *mut StateStorage,
    key: *const c_char,
    state_json: *const c_char,
) -> bool {
    if storage.is_null() || key.is_null() || state_json.is_null() {
        return false;
    }

    let key_str = unsafe { CStr::from_ptr(key).to_string_lossy().to_string() };
    let json_str = unsafe { CStr::from_ptr(state_json).to_string_lossy().to_string() };

    if let Ok(state) = ChartState::from_json(&json_str) {
        unsafe {
            (*storage).save(&key_str, state);
        }
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn sera_state_storage_load(
    storage: *const StateStorage,
    key: *const c_char,
) -> *mut c_char {
    if storage.is_null() || key.is_null() {
        return std::ptr::null_mut();
    }

    let key_str = unsafe { CStr::from_ptr(key).to_string_lossy().to_string() };
    let state = unsafe { (*storage).load(&key_str) };

    if let Some(s) = state {
        if let Ok(json) = s.to_json() {
            let c_str = CString::new(json).unwrap_or_else(|_| CString::new("{}").unwrap());
            return c_str.into_raw();
        }
    }

    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn sera_state_storage_free(storage: *mut StateStorage) {
    if !storage.is_null() {
        unsafe {
            let _ = Box::from_raw(storage);
        }
    }
}
