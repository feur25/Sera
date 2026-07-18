use egui::mutex;

#[derive(Clone, Copy)]
pub(super) struct PlotFrame {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Copy)]
pub(super) struct ChartFrame {
    pub native_w: f64,
    pub native_h: f64,
    pub plot: Option<PlotFrame>,
}

#[derive(Clone, Copy)]
pub(super) struct NativeTransform {
    pub scale: f64,
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy)]
pub(super) struct BarRect {
    pub idx: usize,
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

#[derive(Clone, Copy)]
pub(super) struct DataBounds {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}

impl NativeTransform {
    pub fn new(frame_w: f64, frame_h: f64, native_w: f64, native_h: f64) -> Self {
        let nw = if native_w.is_finite() && native_w > 0.0 {
            native_w
        } else {
            frame_w.max(1.0)
        };
        let nh = if native_h.is_finite() && native_h > 0.0 {
            native_h
        } else {
            frame_h.max(1.0)
        };
        let scale = (frame_w / nw).min(frame_h / nh);
        let scale = if scale.is_finite() && scale > 0.0 {
            scale
        } else {
            1.0
        };
        Self {
            scale,
            x: (frame_w - nw * scale) * 0.5,
            y: (frame_h - nh * scale) * 0.5,
        }
    }

    pub fn map(self, x: f64, y: f64) -> (f64, f64) {
        (self.x + x * self.scale, self.y + y * self.scale)
    }
}

pub(super) fn chart_frame(html: &str, fallback_w: f64, fallback_h: f64) -> ChartFrame {
    let svg = first_tag(html, "svg");
    let native = svg
        .and_then(|tag| svg_size(tag))
        .unwrap_or((fallback_w, fallback_h));
    let plot = svg
        .and_then(|tag| attr(tag, "data-sp"))
        .and_then(parse_plot_frame);
    ChartFrame {
        native_w: positive_or(native.0, fallback_w),
        native_h: positive_or(native.1, fallback_h),
        plot,
    }
}

pub(super) fn bar_rects(html: &str) -> Vec<BarRect> {
    let mut rects = Vec::new();
    let mut rest = html;

    let mut bar_condition = html
        .split("<rect")
        .filter_map(|tag| attr_usize(tag, "data-idx"))
        .map(|idx| idx as i32)
        .collect::<Vec<_>>();
    
    while let Some(pos) = bar_condition.iter().position(|&idx| idx >= 0) {
        let idx = bar_condition[pos] as usize;
        bar_condition[pos] = -1;
        let Some(tag_start) = rest.find("<rect") else {
            break;
        };
        rest = &rest[tag_start + 5..];
        let Some(end) = rest.find('>') else {
            break;
        };
        let tag = &rest[..end];
        rest = &rest[end + 1..];
        if let (Some(x), Some(y), Some(w), Some(h)) = (
            attr_f64(tag, "x"),
            attr_f64(tag, "y"),
            attr_f64(tag, "width"),
            attr_f64(tag, "height"),
        ) {
            if w > 0.0 && h > 0.0 {
                rects.push(BarRect { idx, x, y, w, h });
            }
        }
    }

    rects.sort_by_key(|r| r.idx);
    rects
}

pub(super) fn scatter_bounds(x_vals: &[f64], y_vals: &[f64], n: usize) -> Option<DataBounds> {
    if n == 0 {
        return None;
    }
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for i in 0..n {
        min_x = min_x.min(x_vals[i]);
        max_x = max_x.max(x_vals[i]);
        min_y = min_y.min(y_vals[i]);
        max_y = max_y.max(y_vals[i]);
    }
    let range_x = (max_x - min_x).max(1.0);
    let range_y = (max_y - min_y).max(1.0);
    Some(DataBounds {
        min_x,
        max_x: min_x + range_x,
        min_y,
        max_y: min_y + range_y,
    })
}

pub(super) fn project_scatter(plot: PlotFrame, bounds: DataBounds, x: f64, y: f64) -> (f64, f64) {
    let rx = (bounds.max_x - bounds.min_x).max(1e-12);
    let ry = (bounds.max_y - bounds.min_y).max(1e-12);
    (
        plot.left + ((x - bounds.min_x) / rx) * plot.width,
        plot.top + plot.height - ((y - bounds.min_y) / ry) * plot.height,
    )
}

pub(super) fn scatter_fallback(chart_w: f64, chart_h: f64, has_groups: bool) -> PlotFrame {
    let left = 56.0;
    let top = 38.0;
    let bottom = 52.0;
    let right = if has_groups { 140.0 } else { 20.0 };
    PlotFrame {
        left,
        top,
        width: (chart_w - left - right).max(1.0),
        height: (chart_h - top - bottom).max(1.0),
    }
}

pub(super) fn bar_fallback(
    chart_w: f64,
    chart_h: f64,
    has_groups: bool,
    has_ylabel: bool,
    has_xlabel: bool,
) -> PlotFrame {
    let left = if has_ylabel { 68.0 } else { 52.0 };
    let top = 36.0;
    let bottom = if has_xlabel { 58.0 } else { 48.0 };
    let right = if has_groups { 148.0 } else { 20.0 };
    PlotFrame {
        left,
        top,
        width: (chart_w - left - right).max(1.0),
        height: (chart_h - top - bottom).max(1.0),
    }
}

fn positive_or(v: f64, fallback: f64) -> f64 {
    if v.is_finite() && v > 0.0 {
        v
    } else {
        fallback.max(1.0)
    }
}

fn first_tag<'a>(html: &'a str, name: &str) -> Option<&'a str> {
    let needle = format!("<{}", name);
    let start = html.find(&needle)?;
    let end = html[start..].find('>')?;
    Some(&html[start..start + end + 1])
}

fn attr<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let needle = format!("{}=", name);
    let start = tag.find(&needle)? + needle.len();
    let bytes = tag.as_bytes();
    let q = *bytes.get(start)?;
    if q == b'"' || q == b'\'' {
        let value_start = start + 1;
        let value_end = tag[value_start..].find(q as char)? + value_start;
        Some(&tag[value_start..value_end])
    } else {
        let value_end = tag[start..]
            .find(|c: char| c.is_whitespace() || c == '>')
            .map(|i| start + i)
            .unwrap_or(tag.len());
        Some(&tag[start..value_end])
    }
}

fn attr_f64(tag: &str, name: &str) -> Option<f64> {
    attr(tag, name)?.parse::<f64>().ok()
}

fn attr_usize(tag: &str, name: &str) -> Option<usize> {
    attr(tag, name)?.parse::<usize>().ok()
}

fn svg_size(tag: &str) -> Option<(f64, f64)> {
    if let (Some(w), Some(h)) = (attr_f64(tag, "width"), attr_f64(tag, "height")) {
        return Some((w, h));
    }
    let vb = attr(tag, "viewBox")?;
    let nums: Vec<f64> = vb
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter_map(|v| v.parse::<f64>().ok())
        .collect();
    if nums.len() == 4 {
        Some((nums[2], nums[3]))
    } else {
        None
    }
}

fn parse_plot_frame(data: &str) -> Option<PlotFrame> {
    let nums: Vec<f64> = data
        .split(',')
        .filter_map(|v| v.trim().parse::<f64>().ok())
        .collect();
    if nums.len() >= 4 && nums[2] > 0.0 && nums[3] > 0.0 {
        Some(PlotFrame {
            left: nums[0],
            top: nums[1],
            width: nums[2],
            height: nums[3],
        })
    } else {
        None
    }
}
