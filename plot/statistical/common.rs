pub const PALETTE: &[u32] = &[
    0x636EFA, 0xEF553B, 0x00CC96, 0xAB63FA, 0xFFA15A, 0x19D3F3, 0xFF6692, 0xB6E880, 0xFF97FF,
    0xFECB52,
];

pub const PALETTE_ACCESSIBLE: &[u32] = &[
    0x0072B2, 0xD55E00, 0x009E73, 0xCC79A7, 0xB8720A, 0x6A3D9A, 0x666666, 0x8B4513, 0x9A8B00,
    0xA6183D,
];

#[inline(always)]
pub fn palette_color(custom: &[u32], i: usize) -> u32 {
    let p = if custom.is_empty() { PALETTE } else { custom };
    p[i % p.len()]
}

#[macro_export]
macro_rules! chart_config {
    ($name:ident, $dw:expr, $dh:expr;
     struct { $($s:tt)* }
     defaults { $($d:tt)* }) => {
        pub struct $name<'a> {
            pub title: &'a str,
            pub x_label: &'a str,
            pub y_label: &'a str,
            pub gridlines: bool,
            pub sort_order: &'a str,
            pub hover: &'a [crate::html::hover::HoverSlot],
            pub legend_position: &'a str,
            pub width: i32,
            pub height: i32,
            $($s)*
        }
        impl<'a> Default for $name<'a> {
            fn default() -> Self {
                Self {
                    title: "",
                    x_label: "",
                    y_label: "",
                    gridlines: false,
                    sort_order: "none",
                    hover: &[],
                    legend_position: "none",
                    width: $dw,
                    height: $dh,
                    $($d)*
                }
            }
        }
    };
}

pub fn apply_sort(labels: &[String], values: &[f64], order: &str) -> (Vec<String>, Vec<f64>) {
    if order.is_empty() || order == "none" {
        return (labels.to_vec(), values.to_vec());
    }
    let n = labels.len().min(values.len());
    let idx = sort_indices(n, values, labels, order);
    (sorted(&idx, labels), sorted(&idx, values))
}

pub fn apply_sort_groups(
    labels: &[String],
    values: &[f64],
    groups: &[String],
    order: &str,
) -> (Vec<String>, Vec<f64>, Vec<String>) {
    if order.is_empty() || order == "none" {
        return (labels.to_vec(), values.to_vec(), groups.to_vec());
    }
    let n = labels.len().min(values.len()).min(groups.len());
    let idx = sort_indices(n, values, labels, order);
    (
        sorted(&idx, labels),
        sorted(&idx, values),
        sorted(&idx, groups),
    )
}

pub fn sort_indices<L: Ord>(n: usize, vals: &[f64], labels: &[L], order: &str) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..n).collect();
    match order {
        "asc" | "ascending" => idx.sort_by(|&a, &b| {
            vals[a]
                .partial_cmp(&vals[b])
                .unwrap_or(std::cmp::Ordering::Equal)
        }),
        "desc" | "descending" => idx.sort_by(|&a, &b| {
            vals[b]
                .partial_cmp(&vals[a])
                .unwrap_or(std::cmp::Ordering::Equal)
        }),
        "alpha" | "alphabetical" => idx.sort_by(|&a, &b| labels[a].cmp(&labels[b])),
        "alpha_desc" => idx.sort_by(|&a, &b| labels[b].cmp(&labels[a])),
        _ => {}
    }
    idx
}

pub fn sorted<T: Clone>(idx: &[usize], data: &[T]) -> Vec<T> {
    idx.iter().map(|&i| data[i].clone()).collect()
}

pub fn svg_open(buf: &mut Vec<u8>, w: i32, h: i32) {
    push_b(buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(buf, w);
    push_b(buf, b"\" height=\"");
    push_i(buf, h);
    push_b(buf, b"\" viewBox=\"0 0 ");
    push_i(buf, w);
    push_b(buf, b" ");
    push_i(buf, h);
    push_b(buf, b"\">");
    push_b(
        buf,
        b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>",
    );
}

pub fn svg_open_rescalable(
    buf: &mut Vec<u8>,
    w: i32,
    h: i32,
    pad_l: i32,
    pad_t: i32,
    plot_w: i32,
    plot_h: i32,
) {
    push_b(buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(buf, w);
    push_b(buf, b"\" height=\"");
    push_i(buf, h);
    push_b(buf, b"\" viewBox=\"0 0 ");
    push_i(buf, w);
    push_b(buf, b" ");
    push_i(buf, h);
    push_b(buf, b"\" data-sp=\"");
    push_i(buf, pad_l);
    push_b(buf, b",");
    push_i(buf, pad_t);
    push_b(buf, b",");
    push_i(buf, plot_w);
    push_b(buf, b",");
    push_i(buf, plot_h);
    push_b(buf, b"\">");
    push_b(
        buf,
        b"<rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>",
    );
}

pub fn svg_title(buf: &mut Vec<u8>, title: &str, cx: i32, y: i32) {
    push_b(buf, b"<title>");
    escape_xml(buf, if title.is_empty() { "Chart" } else { title });
    push_b(buf, b"</title>");
    if title.is_empty() {
        return;
    }
    push_b(buf, b"<text x=\"");
    push_i(buf, cx);
    push_b(buf, b"\" y=\"");
    push_i(buf, y);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\" class=\"sp-ttl\">");
    escape_xml(buf, title);
    push_b(buf, b"</text>");
}

pub fn svg_axis_lines(buf: &mut Vec<u8>, pad_l: i32, pad_t: i32, plot_w: i32, plot_h: i32) {
    push_b(buf, b"<line x1=\"");
    push_i(buf, pad_l);
    push_b(buf, b"\" y1=\"");
    push_i(buf, pad_t);
    push_b(buf, b"\" x2=\"");
    push_i(buf, pad_l);
    push_b(buf, b"\" y2=\"");
    push_i(buf, pad_t + plot_h);
    push_b(
        buf,
        b"\" stroke=\"#6b7280\" stroke-width=\"1\" class=\"sp-ax-y\"/>",
    );
    push_b(buf, b"<line x1=\"");
    push_i(buf, pad_l);
    push_b(buf, b"\" y1=\"");
    push_i(buf, pad_t + plot_h);
    push_b(buf, b"\" x2=\"");
    push_i(buf, pad_l + plot_w);
    push_b(buf, b"\" y2=\"");
    push_i(buf, pad_t + plot_h);
    push_b(
        buf,
        b"\" stroke=\"#6b7280\" stroke-width=\"1\" class=\"sp-ax-x\"/>",
    );
}

pub fn svg_x_label(buf: &mut Vec<u8>, label: &str, cx: i32, y: i32) {
    if label.is_empty() {
        return;
    }
    push_b(buf, b"<text x=\"");
    push_i(buf, cx);
    push_b(buf, b"\" y=\"");
    push_i(buf, y);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\" class=\"sp-xl\">");
    escape_xml(buf, label);
    push_b(buf, b"</text>");
}

pub fn svg_y_label(buf: &mut Vec<u8>, label: &str, x: i32, pad_t: i32, plot_h: i32) {
    if label.is_empty() {
        return;
    }
    let ym = pad_t + plot_h / 2;
    push_b(buf, b"<text x=\"");
    push_i(buf, x);
    push_b(buf, b"\" y=\"");
    push_i(buf, ym);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\" transform=\"rotate(-90,");
    push_i(buf, x);
    push_b(buf, b",");
    push_i(buf, ym);
    push_b(buf, b")\" class=\"sp-yl\">");
    escape_xml(buf, label);
    push_b(buf, b"</text>");
}

pub fn svg_legend_item(
    buf: &mut Vec<u8>,
    si: i32,
    name: &str,
    color: u32,
    x: i32,
    y: i32,
    max_len: usize,
) {
    if name.is_empty() {
        return;
    }
    let hx = hex6(color);
    push_b(buf, b"<g data-legend=\"1\" tabindex=\"0\" role=\"button\" aria-label=\"");
    escape_xml(buf, truncate(name, max_len));
    push_b(buf, b"\" data-series=\"");
    push_i(buf, si);
    push_b(buf, b"\">");
    push_b(buf, b"<rect x=\"");
    push_i(buf, x);
    push_b(buf, b"\" y=\"");
    push_i(buf, y);
    push_b(buf, b"\" width=\"12\" height=\"12\" rx=\"2\" fill=\"#");
    buf.extend_from_slice(&hx);
    push_b(buf, b"\"/>");
    push_b(buf, b"<text x=\"");
    push_i(buf, x + 16);
    push_b(buf, b"\" y=\"");
    push_i(buf, y + 10);
    push_b(
        buf,
        b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">",
    );
    escape_xml(buf, truncate(name, max_len));
    push_b(buf, b"</text></g>");
}

pub fn svg_hgrid(buf: &mut Vec<u8>, x1: i32, x2: i32, y: i32) {
    push_b(buf, b"<line x1=\"");
    push_i(buf, x1);
    push_b(buf, b"\" y1=\"");
    push_i(buf, y);
    push_b(buf, b"\" x2=\"");
    push_i(buf, x2);
    push_b(buf, b"\" y2=\"");
    push_i(buf, y);
    push_b(
        buf,
        b"\" stroke=\"#6b7280\" stroke-width=\"0.8\" stroke-opacity=\"0.65\" class=\"sp-gl\"/>",
    );
}

pub fn svg_vgrid(buf: &mut Vec<u8>, x: i32, y1: i32, y2: i32) {
    push_b(buf, b"<line x1=\"");
    push_i(buf, x);
    push_b(buf, b"\" y1=\"");
    push_i(buf, y1);
    push_b(buf, b"\" x2=\"");
    push_i(buf, x);
    push_b(buf, b"\" y2=\"");
    push_i(buf, y2);
    push_b(
        buf,
        b"\" stroke=\"#6b7280\" stroke-width=\"0.8\" stroke-opacity=\"0.65\" class=\"sp-gl\"/>",
    );
}

pub fn svg_tick_y(buf: &mut Vec<u8>, x: i32, y: i32, val: f64) {
    push_b(buf, b"<text x=\"");
    push_i(buf, x);
    push_b(buf, b"\" y=\"");
    push_i(buf, y);
    push_b(buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-yt\">");
    if val.abs() >= 1000.0 {
        push_i(buf, val as i32);
    } else {
        push_f2(buf, val);
    }
    push_b(buf, b"</text>");
}

pub fn svg_tick_x(buf: &mut Vec<u8>, x: i32, y: i32, val: f64) {
    push_b(buf, b"<text x=\"");
    push_i(buf, x);
    push_b(buf, b"\" y=\"");
    push_i(buf, y);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-xt\">");
    if val >= 10000.0 {
        push_i(buf, (val / 1000.0) as i32);
        push_b(buf, b"k");
    } else if val >= 1000.0 {
        push_i(buf, val as i32);
    } else {
        push_f2(buf, val);
    }
    push_b(buf, b"</text>");
}

#[inline(always)]
pub fn push(buf: &mut Vec<u8>, s: &str) {
    buf.extend_from_slice(s.as_bytes());
}

#[inline(always)]
pub fn push_b(buf: &mut Vec<u8>, b: &[u8]) {
    buf.extend_from_slice(b);
}

pub fn hex6(c: u32) -> [u8; 6] {
    let h = b"0123456789abcdef";
    [
        h[((c >> 20) & 0xF) as usize],
        h[((c >> 16) & 0xF) as usize],
        h[((c >> 12) & 0xF) as usize],
        h[((c >> 8) & 0xF) as usize],
        h[((c >> 4) & 0xF) as usize],
        h[(c & 0xF) as usize],
    ]
}

#[inline]
pub fn push_hex(buf: &mut Vec<u8>, c: u32) {
    buf.push(b'#');
    buf.extend_from_slice(&hex6(c));
}

pub fn escape_xml(buf: &mut Vec<u8>, s: &str) {
    if !s
        .bytes()
        .any(|b| b == b'&' || b == b'<' || b == b'>' || b == b'"' || b == b'\'')
    {
        buf.extend_from_slice(s.as_bytes());
        return;
    }
    for ch in s.chars() {
        match ch {
            '&' => buf.extend_from_slice(b"&amp;"),
            '<' => buf.extend_from_slice(b"&lt;"),
            '>' => buf.extend_from_slice(b"&gt;"),
            '"' => buf.extend_from_slice(b"&quot;"),
            '\'' => buf.extend_from_slice(b"&#39;"),
            c => {
                let mut tmp = [0u8; 4];
                buf.extend_from_slice(c.encode_utf8(&mut tmp).as_bytes());
            }
        }
    }
}

pub fn escape_xml_s(s: &str) -> String {
    let mut buf = Vec::with_capacity(s.len() + 8);
    escape_xml(&mut buf, s);
    unsafe { String::from_utf8_unchecked(buf) }
}

pub fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max {
        s
    } else {
        let mut end = max;
        while end > 0 && !s.is_char_boundary(end) {
            end -= 1;
        }
        &s[..end]
    }
}

pub fn write_f2(buf: &mut Vec<u8>, v: f64) {
    let s = format!("{:.2}", v);
    buf.extend_from_slice(s.as_bytes());
}

pub fn write_f1(buf: &mut Vec<u8>, v: f64) {
    let s = format!("{:.1}", v);
    buf.extend_from_slice(s.as_bytes());
}

#[inline]
pub fn write_i(buf: &mut Vec<u8>, v: i32) {
    let s = itoa_i32(v);
    buf.extend_from_slice(s.as_bytes());
}

#[inline(always)]
pub fn push_i(buf: &mut Vec<u8>, mut n: i32) {
    if n < 0 {
        buf.push(b'-');
        n = -n;
    }
    if n == 0 {
        buf.push(b'0');
        return;
    }
    let mut d = [0u8; 10];
    let mut len = 0;
    while n > 0 {
        d[len] = (n % 10) as u8 + b'0';
        n /= 10;
        len += 1;
    }
    for &b in d[..len].iter().rev() {
        buf.push(b);
    }
}

#[inline(always)]
pub fn push_f2(buf: &mut Vec<u8>, v: f64) {
    let neg = v < 0.0;
    let vi = (v.abs() * 100.0 + 0.5) as u64;
    let whole = vi / 100;
    let frac = vi % 100;
    if neg {
        buf.push(b'-');
    }
    let mut d = [0u8; 20];
    let mut len = 0;
    let mut w = whole;
    if w == 0 {
        d[0] = b'0';
        len = 1;
    } else {
        while w > 0 {
            d[len] = (w % 10) as u8 + b'0';
            w /= 10;
            len += 1;
        }
    }
    for &b in d[..len].iter().rev() {
        buf.push(b);
    }
    buf.push(b'.');
    buf.push((frac / 10) as u8 + b'0');
    buf.push((frac % 10) as u8 + b'0');
}

fn itoa_i32(mut n: i32) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let neg = n < 0;
    if neg {
        n = -n;
    }
    let mut digits = [0u8; 11];
    let mut pos = 11;
    while n > 0 {
        pos -= 1;
        digits[pos] = (n % 10) as u8 + b'0';
        n /= 10;
    }
    if neg {
        pos -= 1;
        digits[pos] = b'-';
    }
    String::from_utf8_lossy(&digits[pos..]).to_string()
}

pub fn lerp_color(t: f64, low: u32, mid: u32, high: u32) -> u32 {
    let (ca, cb, u) = if t <= 0.5 {
        (low, mid, t * 2.0)
    } else {
        (mid, high, (t - 0.5) * 2.0)
    };
    let lerp_ch = |shift: u32| -> u32 {
        let a = (ca >> shift) & 0xFF;
        let b = (cb >> shift) & 0xFF;
        (a as f64 + (b as f64 - a as f64) * u) as u32
    };
    (lerp_ch(16) << 16) | (lerp_ch(8) << 8) | lerp_ch(0)
}

pub struct Frame {
    pub buf: Vec<u8>,
    pub w: i32,
    pub h: i32,
    pub pl: i32,
    pub pt: i32,
    pub pw: i32,
    pub ph: i32,
    pub hid: u64,
}

impl Frame {
    #[inline]
    pub fn new(w: i32, h: i32, pl: i32, pt: i32, pb: i32, pr: i32, cap: usize) -> Self {
        Self {
            buf: Vec::with_capacity(cap),
            w,
            h,
            pl,
            pt,
            pw: w - pl - pr,
            ph: h - pt - pb,
            hid: 0,
        }
    }

    pub fn open(&mut self, title: &str, rescalable: bool) {
        if rescalable {
            svg_open_rescalable(
                &mut self.buf,
                self.w,
                self.h,
                self.pl,
                self.pt,
                self.pw,
                self.ph,
            );
        } else {
            svg_open(&mut self.buf, self.w, self.h);
        }
        svg_title(&mut self.buf, title, self.pl + self.pw / 2, 26);
    }

    pub fn y_grid(&mut self, n: i32, y_min: f64, y_max: f64, grid: bool) {
        let rng = y_max - y_min;
        for i in 0..=n {
            let f = i as f64 / n as f64;
            let y = self.pt + ((1.0 - f) * self.ph as f64) as i32;
            let v = y_min + f * rng;
            if grid && i > 0 {
                svg_hgrid(&mut self.buf, self.pl, self.pl + self.pw, y);
            }
            svg_tick_y(&mut self.buf, self.pl - 4, y + 4, v);
        }
    }

    /// Y-grid + ticks with CSS classes sp-gl / sp-yt for spRescale
    pub fn y_grid_rc(&mut self, n: i32, y_min: f64, y_max: f64, grid: bool) {
        let rng = y_max - y_min;
        for i in 0..=n {
            let f = i as f64 / n as f64;
            let y = self.pt + ((1.0 - f) * self.ph as f64) as i32;
            let v = y_min + f * rng;
            if grid && i > 0 {
                push_b(&mut self.buf, b"<line x1=\"");
                push_i(&mut self.buf, self.pl);
                push_b(&mut self.buf, b"\" y1=\"");
                push_i(&mut self.buf, y);
                push_b(&mut self.buf, b"\" x2=\"");
                push_i(&mut self.buf, self.pl + self.pw);
                push_b(&mut self.buf, b"\" y2=\"");
                push_i(&mut self.buf, y);
                push_b(
                    &mut self.buf,
                    b"\" stroke=\"#e2e8f0\" stroke-width=\".5\" class=\"sp-gl\"/>",
                );
            }
            push_b(&mut self.buf, b"<text x=\"");
            push_i(&mut self.buf, self.pl - 4);
            push_b(&mut self.buf, b"\" y=\"");
            push_i(&mut self.buf, y + 3);
            push_b(&mut self.buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-yt\">");
            push_f2(&mut self.buf, v);
            push_b(&mut self.buf, b"</text>");
        }
    }

    pub fn y_grid_int(&mut self, n: i32, max_count: f64, grid: bool) {
        for i in 0..=n {
            let f = i as f64 / n as f64;
            let y = self.pt + ((1.0 - f) * self.ph as f64) as i32;
            let v = (f * max_count).round() as i32;
            if grid && i > 0 {
                svg_hgrid(&mut self.buf, self.pl, self.pl + self.pw, y);
            }
            push_b(&mut self.buf, b"<text x=\"");
            push_i(&mut self.buf, self.pl - 4);
            push_b(&mut self.buf, b"\" y=\"");
            push_i(&mut self.buf, y + 3);
            push_b(&mut self.buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#6b7280\" class=\"sp-yt\">");
            push_i(&mut self.buf, v);
            push_b(&mut self.buf, b"</text>");
        }
    }

    pub fn x_grid(&mut self, n: i32, x_min: f64, x_max: f64, grid: bool) {
        let rng = x_max - x_min;
        for i in 0..=n {
            let f = i as f64 / n as f64;
            let x = self.pl + (f * self.pw as f64) as i32;
            let v = x_min + f * rng;
            if grid && i > 0 {
                svg_vgrid(&mut self.buf, x, self.pt, self.pt + self.ph);
            }
            svg_tick_x(&mut self.buf, x, self.pt + self.ph + 14, v);
        }
    }

    pub fn axes(&mut self, xl: &str, yl: &str) {
        svg_axis_lines(&mut self.buf, self.pl, self.pt, self.pw, self.ph);
        svg_x_label(&mut self.buf, xl, self.pl + self.pw / 2, self.h - 4);
        svg_y_label(&mut self.buf, yl, 14, self.pt, self.ph);
    }

    pub fn legend(&mut self, names: &[&str], palette: &[u32], x: i32) {
        for (i, name) in names.iter().enumerate() {
            svg_legend_item(
                &mut self.buf,
                i as i32,
                name,
                palette_color(palette, i),
                x,
                self.pt + i as i32 * 22,
                14,
            );
        }
    }

    pub fn legend_pos(&mut self, names: &[&str], palette: &[u32], pos: &str) {
        if pos.is_empty() || pos == "none" {
            return;
        }
        let n = names.len() as i32;
        let (x, y_start, anchor) = match pos {
            "left" | "top-left" => (self.pl + 8, self.pt + 4, "start"),
            "bottom-left" => (self.pl + 8, self.pt + self.ph - n * 22, "start"),
            "bottom" | "bottom-right" => (self.pl + self.pw - 4, self.pt + self.ph - n * 22, "end"),
            "top" | "top-right" => (self.pl + self.pw - 4, self.pt + 4, "end"),
            _ => (self.pl + self.pw + 14, self.pt, "start"),
        };
        let _ = anchor;
        for (i, name) in names.iter().enumerate() {
            svg_legend_item(
                &mut self.buf,
                i as i32,
                name,
                palette_color(palette, i),
                x,
                y_start + i as i32 * 22,
                14,
            );
        }
    }

    pub fn svg(mut self) -> String {
        push_b(&mut self.buf, b"</svg>");
        unsafe { String::from_utf8_unchecked(self.buf) }
    }

    pub fn html(mut self, hover_json: &str) -> String {
        push_b(&mut self.buf, b"</svg>");
        crate::html::hover::html_suffix(&mut self.buf, self.hid, hover_json);
        unsafe { String::from_utf8_unchecked(self.buf) }
    }

    pub fn new_html(
        title: &str,
        w: i32,
        h: i32,
        pl: i32,
        pt: i32,
        pb: i32,
        pr: i32,
        cap: usize,
    ) -> Self {
        let hid = crate::html::hover::html_id();
        let mut buf = Vec::with_capacity(cap + 14_000);
        crate::html::hover::html_prefix(&mut buf, title, hid);
        Self {
            buf,
            w,
            h,
            pl,
            pt,
            pw: w - pl - pr,
            ph: h - pt - pb,
            hid,
        }
    }
}

#[cfg(test)]
mod accessible_palette_tests {
    use super::PALETTE_ACCESSIBLE;

    fn srgb_to_linear(c: f64) -> f64 {
        if c <= 0.03928 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
    }

    fn luminance(hex: u32) -> f64 {
        let r = srgb_to_linear(((hex >> 16) & 0xFF) as f64 / 255.0);
        let g = srgb_to_linear(((hex >> 8) & 0xFF) as f64 / 255.0);
        let b = srgb_to_linear((hex & 0xFF) as f64 / 255.0);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    fn contrast(a: u32, b: u32) -> f64 {
        let (la, lb) = (luminance(a), luminance(b));
        let (hi, lo) = if la > lb { (la, lb) } else { (lb, la) };
        (hi + 0.05) / (lo + 0.05)
    }

    #[test]
    fn every_accessible_palette_color_meets_wcag_non_text_contrast_against_white() {
        for &c in PALETTE_ACCESSIBLE {
            let ratio = contrast(c, 0xFFFFFF);
            assert!(ratio >= 3.0, "color #{c:06X} has contrast {ratio:.2}:1 against white, below WCAG 1.4.11's 3:1 minimum");
        }
    }

    #[test]
    fn accessible_palette_has_ten_distinct_colors() {
        let mut seen = std::collections::HashSet::new();
        for &c in PALETTE_ACCESSIBLE {
            assert!(seen.insert(c), "duplicate color #{c:06X} in accessible palette");
        }
        assert_eq!(PALETTE_ACCESSIBLE.len(), 10);
    }
}

#[cfg(test)]
mod truncate_tests {
    use super::truncate;

    #[test]
    fn truncate_does_not_panic_when_max_falls_inside_a_multibyte_char() {
        let s = "日本語<script>🎉&\"'";
        let out = truncate(s, 18);
        assert!(out.len() <= 18);
        assert!(s.starts_with(out));
    }

    #[test]
    fn truncate_leaves_short_strings_untouched() {
        assert_eq!(truncate("abc", 10), "abc");
    }

    #[test]
    fn truncate_cuts_ascii_at_exact_byte_offset() {
        assert_eq!(truncate("abcdefgh", 3), "abc");
    }
}
