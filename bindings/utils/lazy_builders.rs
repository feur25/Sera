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
