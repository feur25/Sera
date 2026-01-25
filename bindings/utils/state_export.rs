use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChartState {
    pub labels: Vec<String>,
    pub values: Vec<f64>,
    pub title: String,
    pub hover_data: Vec<HashMap<String, String>>,
    pub tooltip_bg_color: (u8, u8, u8, u8),
    pub tooltip_text_color: (u8, u8, u8, u8),
    pub orientation: bool,
    pub sort_mode: i32,
    pub current_chart_kind: u8,
    pub is_3d_mode: bool,
    pub zoom: f32,
    pub filter_threshold: f64,
    pub visible_elements: Vec<bool>,
    pub limit_value: usize,
    pub images_base64: Vec<String>,
}

impl ChartState {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        use std::fmt::Write;
        let mut json = String::with_capacity(self.labels.len() * 50 + self.values.len() * 20);
        json.push('{');
        json.push_str("\"labels\":[");
        for (i, label) in self.labels.iter().enumerate() {
            if i > 0 { json.push(','); }
            json.push('"');
            for c in label.chars() {
                match c {
                    '"' => json.push_str("\\\""),
                    '\\' => json.push_str("\\\\"),
                    '\n' => json.push_str("\\n"),
                    '\r' => json.push_str("\\r"),
                    '\t' => json.push_str("\\t"),
                    c if c.is_control() => { let _ = write!(json, "\\u{:04x}", c as u32); },
                    c => json.push(c),
                }
            }
            json.push('"');
        }
        json.push_str("],\"values\":[");
        for (i, v) in self.values.iter().enumerate() {
            if i > 0 { json.push(','); }
            let _ = write!(json, "{}", v);
        }
        json.push(']');
        if !self.hover_data.is_empty() {
            json.push_str(",\"hover_data\":{");
            let mut first = true;
            for (i, hd) in self.hover_data.iter().enumerate() {
                if !first { json.push(','); }
                let _ = write!(json, "\"{}\":{{", i);
                let mut hdfirst = true;
                for (k, v) in hd.iter() {
                    if !hdfirst { json.push(','); }
                    json.push_str("\"");
                    for c in k.chars() {
                        if c == '"' { json.push_str("\\\""); } else { json.push(c); }
                    }
                    json.push_str("\":\"");
                    for c in v.chars() {
                        if c == '"' { json.push_str("\\\""); } else if c == '\\' { json.push_str("\\\\"); } else { json.push(c); }
                    }
                    json.push_str("\"");
                    hdfirst = false;
                }
                json.push('}');
                first = false;
            }
            json.push('}');
        }
        json.push('}');
        Ok(json)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

pub struct StateBuilder<T> {
    data: T,
}

impl StateBuilder<()> {
    pub fn new() -> Self {
        Self { data: () }
    }
}

impl<T> StateBuilder<T> {
    pub fn with_state(state: T) -> StateBuilder<T> {
        StateBuilder { data: state }
    }

    pub fn build(self) -> T {
        self.data
    }
}

pub struct ChartStateBuilder {
    labels: Vec<String>,
    values: Vec<f64>,
    title: String,
    hover_data: Vec<HashMap<String, String>>,
    tooltip_bg_color: (u8, u8, u8, u8),
    tooltip_text_color: (u8, u8, u8, u8),
    orientation: bool,
    sort_mode: i32,
    current_chart_kind: u8,
    is_3d_mode: bool,
    zoom: f32,
    filter_threshold: f64,
    visible_elements: Vec<bool>,
    limit_value: usize,
    images_base64: Vec<String>,
}

impl Default for ChartStateBuilder {
    fn default() -> Self {
        Self {
            labels: Vec::new(),
            values: Vec::new(),
            title: String::new(),
            hover_data: Vec::new(),
            tooltip_bg_color: (0, 0, 0, 0),
            tooltip_text_color: (0, 0, 0, 255),
            orientation: true,
            sort_mode: 0,
            current_chart_kind: 1,
            is_3d_mode: false,
            zoom: 1.0,
            filter_threshold: 0.0,
            visible_elements: Vec::new(),
            limit_value: 50,
            images_base64: Vec::new(),
        }
    }
}

impl ChartStateBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn labels(mut self, labels: Vec<String>) -> Self {
        self.labels = labels;
        self
    }

    pub fn values(mut self, values: Vec<f64>) -> Self {
        self.values = values;
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn hover_data(mut self, data: Vec<HashMap<String, String>>) -> Self {
        self.hover_data = data;
        self
    }

    pub fn tooltip_colors(mut self, bg: (u8, u8, u8, u8), text: (u8, u8, u8, u8)) -> Self {
        self.tooltip_bg_color = bg;
        self.tooltip_text_color = text;
        self
    }

    pub fn orientation(mut self, vertical: bool) -> Self {
        self.orientation = vertical;
        self
    }

    pub fn sort_mode(mut self, mode: i32) -> Self {
        self.sort_mode = mode;
        self
    }

    pub fn chart_kind(mut self, kind: u8) -> Self {
        self.current_chart_kind = kind;
        self
    }

    pub fn is_3d(mut self, is_3d: bool) -> Self {
        self.is_3d_mode = is_3d;
        self
    }

    pub fn zoom(mut self, zoom: f32) -> Self {
        self.zoom = zoom;
        self
    }

    pub fn filter_threshold(mut self, threshold: f64) -> Self {
        self.filter_threshold = threshold;
        self
    }

    pub fn visible_elements(mut self, visible: Vec<bool>) -> Self {
        self.visible_elements = visible;
        self
    }

    pub fn limit_value(mut self, limit: usize) -> Self {
        self.limit_value = limit;
        self
    }

    pub fn images_base64(mut self, images: Vec<String>) -> Self {
        self.images_base64 = images;
        self
    }

    pub fn build(self) -> ChartState {
        ChartState {
            labels: self.labels,
            values: self.values,
            title: self.title,
            hover_data: self.hover_data,
            tooltip_bg_color: self.tooltip_bg_color,
            tooltip_text_color: self.tooltip_text_color,
            orientation: self.orientation,
            sort_mode: self.sort_mode,
            current_chart_kind: self.current_chart_kind,
            is_3d_mode: self.is_3d_mode,
            zoom: self.zoom,
            filter_threshold: self.filter_threshold,
            visible_elements: self.visible_elements,
            limit_value: self.limit_value,
            images_base64: self.images_base64,
        }
    }
}

pub struct StateStorage {
    cache: HashMap<String, ChartState>,
}

impl StateStorage {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn save(&mut self, key: &str, state: ChartState) {
        self.cache.insert(key.to_string(), state);
    }

    pub fn load(&self, key: &str) -> Option<ChartState> {
        self.cache.get(key).cloned()
    }

    pub fn exists(&self, key: &str) -> bool {
        self.cache.contains_key(key)
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.cache.remove(key).is_some()
    }

    pub fn list_keys(&self) -> Vec<String> {
        self.cache.keys().cloned().collect()
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

impl Default for StateStorage {
    fn default() -> Self {
        Self::new()
    }
}
