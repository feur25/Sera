use crate::core::*;

pub struct Plot {
    pub title: String,
    pub traces: Vec<Trace>,
    pub layout: Layout,
}

impl Plot {
    pub fn new(title: impl Into<String>, layout: Layout) -> Self {
        Self {
            title: title.into(),
            traces: Vec::new(),
            layout,
        }
    }

    pub fn add_trace(&mut self, trace: Trace) {
        self.traces.push(trace);
    }

    pub fn trace_count(&self) -> usize {
        self.traces.len()
    }

    pub fn get_bounds(&self) -> Option<(Range, Range)> {
        if self.traces.is_empty() {
            return None;
        }

        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;

        for trace in &self.traces {
            if let Some(x_range) = Range::from_slice(&trace.x) {
                x_min = x_min.min(x_range.min);
                x_max = x_max.max(x_range.max);
            }
            if let Some(y_range) = Range::from_slice(&trace.y) {
                y_min = y_min.min(y_range.min);
                y_max = y_max.max(y_range.max);
            }
        }

        if x_min.is_finite() && x_max.is_finite() && y_min.is_finite() && y_max.is_finite() {
            Some((Range::new(x_min, x_max), Range::new(y_min, y_max)))
        } else {
            None
        }
    }
}
