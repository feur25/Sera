use crate::core::*;

pub struct TraceBuilder {
    name: String,
    kind: ChartKind,
    x: Vec<f64>,
    y: Vec<f64>,
}

impl TraceBuilder {
    pub fn new<S: Into<String>>(name: S, kind: ChartKind) -> Self {
        Self {
            name: name.into(),
            kind,
            x: Vec::new(),
            y: Vec::new(),
        }
    }

    pub fn add_point(mut self, x: f64, y: f64) -> Self {
        self.x.push(x);
        self.y.push(y);
        self
    }

    pub fn add_points(mut self, x: Vec<f64>, y: Vec<f64>) -> Self {
        self.x.extend(x);
        self.y.extend(y);
        self
    }

    pub fn build(self) -> Trace {
        Trace::new(self.name.clone(), self.name, self.kind, self.x, self.y)
    }
}

