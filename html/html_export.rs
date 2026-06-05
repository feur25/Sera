use crate::bindings::utils::state_export::{ChartState, StateStorage};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub struct HtmlExportConfig {
    pub width: i32,
    pub height: i32,
    pub title: String,
    pub theme: HtmlTheme,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum HtmlTheme {
    Light = 0,
    Dark = 1,
    Professional = 2,
}

pub struct HtmlExporter {
    pub config: HtmlExportConfig,
    pub svg_content: String,
    pub labels: Vec<String>,
    pub values: Vec<f64>,
    pub hover_data: serde_json::Value,
    pub state: Option<ChartState>,
}

impl HtmlExportConfig {
    #[inline]
    pub fn new(w: i32, h: i32, t: &str, th: HtmlTheme) -> Self {
        Self {
            width: w,
            height: h,
            title: t.to_string(),
            theme: th,
        }
    }

    #[inline]
    pub fn default() -> Self {
        Self {
            width: 1200,
            height: 800,
            title: "SeraPlot".to_string(),
            theme: HtmlTheme::Light,
        }
    }

    #[inline]
    pub fn light(w: i32, h: i32) -> Self {
        Self {
            width: w,
            height: h,
            title: "SeraPlot".to_string(),
            theme: HtmlTheme::Light,
        }
    }

    #[inline]
    pub fn dark(w: i32, h: i32) -> Self {
        Self {
            width: w,
            height: h,
            title: "SeraPlot".to_string(),
            theme: HtmlTheme::Dark,
        }
    }

    #[inline]
    pub fn with_title(mut self, t: &str) -> Self {
        self.title = t.to_string();
        self
    }

    #[inline]
    pub fn with_theme(mut self, th: HtmlTheme) -> Self {
        self.theme = th;
        self
    }

    #[inline]
    pub fn with_state_export(self, _: bool) -> Self {
        self
    }

    #[inline]
    pub fn with_controls(self, _: bool) -> Self {
        self
    }
}

impl HtmlExporter {
    #[inline]
    pub fn new(config: HtmlExportConfig) -> Self {
        Self {
            config,
            svg_content: String::new(),
            labels: Vec::new(),
            values: Vec::new(),
            hover_data: serde_json::json!({}),
            state: None,
        }
    }

    #[inline]
    pub fn light(w: i32, h: i32) -> Self {
        Self::new(HtmlExportConfig::light(w, h))
    }

    #[inline]
    pub fn dark(w: i32, h: i32) -> Self {
        Self::new(HtmlExportConfig::dark(w, h))
    }

    #[inline]
    pub fn svg(mut self, s: String) -> Self {
        self.svg_content = s;
        self
    }

    #[inline]
    pub fn data(mut self, labels: Vec<String>, values: Vec<f64>) -> Self {
        self.labels = labels;
        self.values = values;
        self
    }

    #[inline]
    pub fn hover(mut self, h: serde_json::Value) -> Self {
        self.hover_data = h;
        self
    }

    #[inline]
    pub fn state(mut self, state: ChartState) -> Self {
        self.state = Some(state);
        self
    }

    #[inline]
    pub fn with_state(mut self, state: ChartState) -> Self {
        self.state = Some(state);
        self
    }

    #[inline]
    pub fn with_data(mut self, labels: Vec<String>, values: Vec<f64>) -> Self {
        self.labels = labels;
        self.values = values;
        self
    }

    #[inline]
    pub fn with_hover(mut self, h: serde_json::Value) -> Self {
        self.hover_data = h;
        self
    }

    #[inline]
    pub fn with_svg(mut self, s: String) -> Self {
        self.svg_content = s;
        self
    }

    #[inline]
    pub fn title(mut self, t: &str) -> Self {
        self.config.title = t.to_string();
        self
    }

    pub fn build_html(&self) -> String {
        use super::assets::Assets;
        let estimated_size = self.svg_content.len() + (self.labels.len() * 50) + 15000;
        let mut html = String::with_capacity(estimated_size);
        html.push_str("<!DOCTYPE html><html><head><meta charset=UTF-8><title>");
        html.push_str(&self.config.title);
        html.push_str("</title><style>");
        html.push_str(Assets::STYLE_CSS);
        html.push_str("</style></head><body><div class=chart-container><div class=zoom-controls><button class=zoom-btn onclick=zo()>−</button><button class=zoom-btn onclick=zr()>⊙</button><button class=zoom-btn onclick=zi()>+</button><button class=zoom-btn onclick=rbs()>✕</button><button class=zoom-btn onclick=of()>⛶</button></div>");
        html.push_str(&self.svg_content);
        html.push_str("</div><div id=seraplot-modal class=modal><div class=modal-content><div class=modal-header><h2>SeraPlot</h2><button id=modal-close-btn class=modal-close>&times;</button></div><div class=modal-body><svg id=modal-svg></svg></div></div></div><script>window.__SERAPLOT_STATE__=");

        if !self.hover_data.is_null() && self.hover_data.is_object() {
            html.push_str(&self.hover_data.to_string());
        } else {
            let state_json = serde_json::json!({
                "labels": &self.labels,
                "values": &self.values,
                "hover_data": &self.hover_data,
                "visible_count": self.labels.len()
            });
            html.push_str(&state_json.to_string());
        }

        html.push_str("</script><script>");
        html.push_str(Assets::SCRIPT_JS);
        html.push_str("</script></body></html>");
        html
    }

    #[inline]
    pub fn html_fast(&self) -> String {
        self.build_html()
    }

    #[inline]
    pub fn export_assets(&self, p: &str) -> Result<(), std::io::Error> {
        use super::assets::Assets;
        std::fs::write(format!("{}/s.css", p), Assets::STYLE_CSS)?;
        std::fs::write(format!("{}/s.js", p), Assets::SCRIPT_JS)?;
        Ok(())
    }

    pub fn export_to_file(&self, p: &str) -> Result<(), std::io::Error> {
        use std::path::Path;
        let bd = Path::new(p)
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_str()
            .unwrap_or(".");
        self.export_assets(bd)?;
        std::fs::write(p, self.build_html())
    }

    pub fn export_to_file_background(self, path: String) {
        std::thread::spawn(move || {
            use std::path::Path;
            let bd = Path::new(&path)
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_str()
                .unwrap_or(".");
            if let Err(e) = self.export_assets(bd) {
                eprintln!("✗ Export failed: {}", e);
                return;
            }
            if let Err(e) = std::fs::write(&path, self.build_html()) {
                eprintln!("✗ Write failed: {}", e);
            }
        });
    }
}

#[no_mangle]
pub extern "C" fn sera_html_export_create(
    w: i32,
    h: i32,
    t: *const c_char,
    th: u8,
) -> *mut HtmlExporter {
    let title = unsafe { CStr::from_ptr(t).to_string_lossy().into_owned() };
    let theme = match th {
        1 => HtmlTheme::Dark,
        2 => HtmlTheme::Professional,
        _ => HtmlTheme::Light,
    };
    Box::into_raw(Box::new(HtmlExporter::new(HtmlExportConfig {
        width: w,
        height: h,
        title,
        theme,
    })))
}

#[no_mangle]
pub extern "C" fn sera_html_export_create_light(w: i32, h: i32) -> *mut HtmlExporter {
    Box::into_raw(Box::new(HtmlExporter::light(w, h)))
}

#[no_mangle]
pub extern "C" fn sera_html_export_create_dark(w: i32, h: i32) -> *mut HtmlExporter {
    Box::into_raw(Box::new(HtmlExporter::dark(w, h)))
}

#[no_mangle]
pub extern "C" fn sera_html_export_set_svg(e: *mut HtmlExporter, s: *const c_char) {
    if !e.is_null() && !s.is_null() {
        unsafe {
            (*e).svg_content = CStr::from_ptr(s).to_string_lossy().into_owned();
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_html_export_set_data(
    e: *mut HtmlExporter,
    l: *const *const c_char,
    v: *const f64,
    c: usize,
) {
    if e.is_null() || l.is_null() || v.is_null() {
        return;
    }
    let mut labels = Vec::with_capacity(c);
    for i in 0..c {
        let lp = unsafe { *l.add(i) };
        labels.push(if lp.is_null() {
            "".into()
        } else {
            unsafe { CStr::from_ptr(lp).to_string_lossy().into_owned() }
        });
    }
    unsafe {
        (*e).labels = labels;
        (*e).values = std::slice::from_raw_parts(v, c).to_vec();
    }
}

#[no_mangle]
pub extern "C" fn sera_html_export_set_title(e: *mut HtmlExporter, t: *const c_char) {
    if !e.is_null() && !t.is_null() {
        unsafe {
            (*e).config.title = CStr::from_ptr(t).to_string_lossy().into_owned();
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_html_export_set_state_json(e: *mut HtmlExporter, sj: *const c_char) -> bool {
    if e.is_null() || sj.is_null() {
        return false;
    }
    let json = unsafe { CStr::from_ptr(sj).to_string_lossy() };
    if let Ok(state) = ChartState::from_json(&json) {
        unsafe {
            (*e).state = Some(state);
        }
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn sera_html_export_build(e: *const HtmlExporter) -> *mut c_char {
    if e.is_null() {
        return std::ptr::null_mut();
    }
    let html = unsafe { (*e).build_html() };
    CString::new(html).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn sera_html_export_build_fast(e: *const HtmlExporter) -> *mut c_char {
    if e.is_null() {
        return std::ptr::null_mut();
    }
    let html = unsafe { (*e).html_fast() };
    CString::new(html).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn sera_html_export_save(e: *const HtmlExporter, p: *const c_char) -> bool {
    if e.is_null() || p.is_null() {
        return false;
    }
    let path = unsafe { CStr::from_ptr(p).to_string_lossy() };
    unsafe { (*e).export_to_file(&path) }.is_ok()
}

#[no_mangle]
pub extern "C" fn sera_html_export_free(e: *mut HtmlExporter) {
    if !e.is_null() {
        unsafe {
            let _ = Box::from_raw(e);
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_string_free_html(p: *mut c_char) {
    if !p.is_null() {
        unsafe {
            let _ = CString::from_raw(p);
        }
    }
}

#[no_mangle]
pub extern "C" fn sera_state_storage_create() -> *mut StateStorage {
    Box::into_raw(Box::new(StateStorage::new()))
}

#[no_mangle]
pub extern "C" fn sera_state_storage_save(
    s: *mut StateStorage,
    k: *const c_char,
    sj: *const c_char,
) -> bool {
    if s.is_null() || k.is_null() || sj.is_null() {
        return false;
    }
    let key = unsafe { CStr::from_ptr(k).to_string_lossy() };
    let json = unsafe { CStr::from_ptr(sj).to_string_lossy() };
    if let Ok(state) = ChartState::from_json(&json) {
        unsafe {
            (*s).save(&key, state);
        }
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn sera_state_storage_load(s: *const StateStorage, k: *const c_char) -> *mut c_char {
    if s.is_null() || k.is_null() {
        return std::ptr::null_mut();
    }
    let key = unsafe { CStr::from_ptr(k).to_string_lossy() };
    if let Some(st) = unsafe { (*s).load(&key) } {
        if let Ok(json) = st.to_json() {
            return CString::new(json).unwrap_or_default().into_raw();
        }
    }
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn sera_state_storage_free(s: *mut StateStorage) {
    if !s.is_null() {
        unsafe {
            let _ = Box::from_raw(s);
        }
    }
}
