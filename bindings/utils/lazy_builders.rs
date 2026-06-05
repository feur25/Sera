use super::bitset::BitSet;

pub struct LazyJsonBuilder {
    labels: Vec<String>,
    values: Vec<f64>,
    cached: Option<String>,
    dirty: bool,
}

impl LazyJsonBuilder {
    #[inline(always)]
    pub fn new(capacity: usize) -> Self {
        Self {
            labels: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
            cached: None,
            dirty: true,
        }
    }

    #[inline(always)]
    pub fn add_point(&mut self, label: String, value: f64) {
        self.labels.push(label);
        self.values.push(value);
        self.dirty = true;
    }

    #[inline(always)]
    pub fn build(&mut self) -> &str {
        if self.dirty || self.cached.is_none() {
            let mut json = String::with_capacity(self.labels.len() * 25 + self.values.len() * 15);
            json.push_str("{\"l\":[");

            for (i, label) in self.labels.iter().enumerate() {
                if i > 0 {
                    json.push(',');
                }
                json.push('"');
                for &ch in label.as_bytes() {
                    match ch {
                        b'"' => json.push_str("\\\""),
                        b'\\' => json.push_str("\\\\"),
                        b'\n' => json.push_str("\\n"),
                        b'\r' => json.push_str("\\r"),
                        b'\t' => json.push_str("\\t"),
                        _ if ch < 32 => {
                            use std::fmt::Write;
                            let _ = write!(json, "\\u{:04x}", ch);
                        }
                        _ => json.push(ch as char),
                    }
                }
                json.push('"');
            }

            json.push_str("],\"v\":[");
            for (i, &val) in self.values.iter().enumerate() {
                if i > 0 {
                    json.push(',');
                }
                use std::fmt::Write;
                let _ = write!(json, "{:.2}", val);
            }
            json.push_str("]}");

            self.cached = Some(json);
            self.dirty = false;
        }

        self.cached.as_ref().unwrap()
    }

    pub fn reset(&mut self) {
        self.labels.clear();
        self.values.clear();
        self.cached.take();
        self.dirty = true;
    }
}

pub struct VectorizedHtmlBuilder {
    title: String,
    svg: String,
    json: String,
    template_cache: String,
}

impl VectorizedHtmlBuilder {
    #[inline(always)]
    pub fn new(title: &str, svg: &str, json: &str) -> Self {
        let capacity = title.len() + svg.len() + json.len() + 4096;
        Self {
            title: title.to_string(),
            svg: svg.to_string(),
            json: json.to_string(),
            template_cache: String::with_capacity(capacity),
        }
    }

    #[inline(always)]
    pub fn build(&mut self) -> String {
        use std::fmt::Write;

        self.template_cache.clear();
        let _ = write!(
            self.template_cache,
            "<!DOCTYPE html><html><head><meta charset=UTF-8><title>{}</title><style>*{{margin:0;padding:0;box-sizing:border-box}}body{{background:#f5f5f5;font:12px sans-serif}}.chart-container{{width:100%;height:100vh;display:flex;align-items:center;justify-content:center;background:#fff;position:relative;overflow:hidden}}svg{{width:90%;height:90%;max-width:1200px;max-height:600px;display:block}}</style></head><body><div class=chart-container>{}</div><script>window.__SERAPLOT_STATE__={}</script></body></html>",
            self.title, self.svg, self.json
        );

        self.template_cache.clone()
    }

    #[inline(always)]
    pub fn build_streaming<F>(&self, mut handler: F)
    where
        F: FnMut(&str),
    {
        handler("<!DOCTYPE html><html><head><meta charset=UTF-8><title>");
        handler(&self.title);
        handler("</title><style>*{margin:0;padding:0;box-sizing:border-box}body{background:#f5f5f5;font:12px sans-serif}.chart-container{width:100%;height:100vh;display:flex;align-items:center;justify-content:center;background:#fff;position:relative;overflow:hidden}svg{width:90%;height:90%;max-width:1200px;max-height:600px;display:block}</style></head><body><div class=chart-container>");
        handler(&self.svg);
        handler("</div><script>window.__SERAPLOT_STATE__=");
        handler(&self.json);
        handler("</script></body></html>");
    }
}

pub struct StatefulExporter {
    labels: Vec<String>,
    values: Vec<f64>,
    visibility: BitSet,
    selection: BitSet,
}

impl StatefulExporter {
    pub fn new(capacity: usize) -> Self {
        Self {
            labels: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
            visibility: BitSet::new(capacity),
            selection: BitSet::new(capacity),
        }
    }

    #[inline(always)]
    pub fn add_point(&mut self, label: String, value: f64) {
        self.labels.push(label);
        self.values.push(value);
    }

    #[inline(always)]
    pub fn set_visible(&mut self, idx: usize, visible: bool) {
        if visible {
            self.visibility.set(idx);
        } else {
            self.visibility.clear(idx);
        }
    }

    #[inline(always)]
    pub fn set_selected(&mut self, idx: usize, selected: bool) {
        if selected {
            self.selection.set(idx);
        } else {
            self.selection.clear(idx);
        }
    }

    pub fn export_filtered(&self) -> (Vec<String>, Vec<f64>) {
        let mut filtered_labels = Vec::new();
        let mut filtered_values = Vec::new();

        for idx in self.visibility.iter_set() {
            if idx < self.labels.len() && self.selection.get(idx) {
                filtered_labels.push(self.labels[idx].clone());
                filtered_values.push(self.values[idx]);
            }
        }

        (filtered_labels, filtered_values)
    }

    #[inline(always)]
    pub fn visible_count(&self) -> usize {
        self.visibility
            .iter_set()
            .take_while(|&idx| self.selection.get(idx))
            .count()
    }

    pub fn reset(&mut self) {
        self.labels.clear();
        self.values.clear();
        self.visibility.clear_all();
        self.selection.clear_all();
    }
}
