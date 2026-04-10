pub const PALETTE: &[u32] = &[
    0x6366F1, 0xF43F5E, 0x10B981, 0xF59E0B, 0x8B5CF6,
    0x06B6D4, 0xEC4899, 0x84CC16, 0xEF4444, 0x14B8A6,
];

#[inline(always)]
pub fn palette_color(custom: &[u32], i: usize) -> u32 {
    let p = if custom.is_empty() { PALETTE } else { custom };
    p[i % p.len()]
}

pub fn apply_sort(labels: &[String], values: &[f64], order: &str) -> (Vec<String>, Vec<f64>) {
    if order.is_empty() || order == "none" { return (labels.to_vec(), values.to_vec()); }
    let n = labels.len().min(values.len());
    let mut idx: Vec<usize> = (0..n).collect();
    match order {
        "asc" | "ascending" => idx.sort_by(|&a, &b| values[a].partial_cmp(&values[b]).unwrap_or(std::cmp::Ordering::Equal)),
        "desc" | "descending" => idx.sort_by(|&a, &b| values[b].partial_cmp(&values[a]).unwrap_or(std::cmp::Ordering::Equal)),
        "alpha" | "alphabetical" => idx.sort_by(|&a, &b| labels[a].cmp(&labels[b])),
        "alpha_desc" => idx.sort_by(|&a, &b| labels[b].cmp(&labels[a])),
        _ => {}
    }
    let sl: Vec<String> = idx.iter().map(|&i| labels[i].clone()).collect();
    let sv: Vec<f64> = idx.iter().map(|&i| values[i]).collect();
    (sl, sv)
}

pub fn apply_sort_groups(labels: &[String], values: &[f64], groups: &[String], order: &str) -> (Vec<String>, Vec<f64>, Vec<String>) {
    if order.is_empty() || order == "none" { return (labels.to_vec(), values.to_vec(), groups.to_vec()); }
    let n = labels.len().min(values.len()).min(groups.len());
    let mut idx: Vec<usize> = (0..n).collect();
    match order {
        "asc" | "ascending" => idx.sort_by(|&a, &b| values[a].partial_cmp(&values[b]).unwrap_or(std::cmp::Ordering::Equal)),
        "desc" | "descending" => idx.sort_by(|&a, &b| values[b].partial_cmp(&values[a]).unwrap_or(std::cmp::Ordering::Equal)),
        "alpha" | "alphabetical" => idx.sort_by(|&a, &b| labels[a].cmp(&labels[b])),
        "alpha_desc" => idx.sort_by(|&a, &b| labels[b].cmp(&labels[a])),
        _ => {}
    }
    let sl: Vec<String> = idx.iter().map(|&i| labels[i].clone()).collect();
    let sv: Vec<f64> = idx.iter().map(|&i| values[i]).collect();
    let sg: Vec<String> = idx.iter().map(|&i| groups[i].clone()).collect();
    (sl, sv, sg)
}

pub fn svg_open(buf: &mut Vec<u8>, w: i32, h: i32) {
    push_b(buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(buf, w); push_b(buf, b"\" height=\"");
    push_i(buf, h); push_b(buf, b"\" viewBox=\"0 0 ");
    push_i(buf, w); push_b(buf, b" ");
    push_i(buf, h); push_b(buf, b"\">");
    push_b(buf, b"<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>");
}

pub fn svg_open_rescalable(buf: &mut Vec<u8>, w: i32, h: i32, pad_l: i32, pad_t: i32, plot_w: i32, plot_h: i32) {
    push_b(buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"");
    push_i(buf, w); push_b(buf, b"\" height=\"");
    push_i(buf, h); push_b(buf, b"\" viewBox=\"0 0 ");
    push_i(buf, w); push_b(buf, b" ");
    push_i(buf, h); push_b(buf, b"\" data-sp=\"");
    push_i(buf, pad_l); push_b(buf, b",");
    push_i(buf, pad_t); push_b(buf, b",");
    push_i(buf, plot_w); push_b(buf, b",");
    push_i(buf, plot_h); push_b(buf, b"\">");
    push_b(buf, b"<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>");
}

pub fn svg_title(buf: &mut Vec<u8>, title: &str, cx: i32, y: i32) {
    if title.is_empty() { return; }
    push_b(buf, b"<text x=\""); push_i(buf, cx);
    push_b(buf, b"\" y=\""); push_i(buf, y);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"15\" font-weight=\"700\" fill=\"#1a202c\">");
    escape_xml(buf, title);
    push_b(buf, b"</text>");
}

pub fn svg_axis_lines(buf: &mut Vec<u8>, pad_l: i32, pad_t: i32, plot_w: i32, plot_h: i32) {
    push_b(buf, b"<line x1=\""); push_i(buf, pad_l);
    push_b(buf, b"\" y1=\""); push_i(buf, pad_t);
    push_b(buf, b"\" x2=\""); push_i(buf, pad_l);
    push_b(buf, b"\" y2=\""); push_i(buf, pad_t + plot_h);
    push_b(buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");
    push_b(buf, b"<line x1=\""); push_i(buf, pad_l);
    push_b(buf, b"\" y1=\""); push_i(buf, pad_t + plot_h);
    push_b(buf, b"\" x2=\""); push_i(buf, pad_l + plot_w);
    push_b(buf, b"\" y2=\""); push_i(buf, pad_t + plot_h);
    push_b(buf, b"\" stroke=\"#cbd5e1\" stroke-width=\"1\"/>");
}

pub fn svg_x_label(buf: &mut Vec<u8>, label: &str, cx: i32, y: i32) {
    if label.is_empty() { return; }
    push_b(buf, b"<text x=\""); push_i(buf, cx);
    push_b(buf, b"\" y=\""); push_i(buf, y);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\">");
    escape_xml(buf, label);
    push_b(buf, b"</text>");
}

pub fn svg_y_label(buf: &mut Vec<u8>, label: &str, x: i32, pad_t: i32, plot_h: i32) {
    if label.is_empty() { return; }
    let ym = pad_t + plot_h / 2;
    push_b(buf, b"<text x=\""); push_i(buf, x);
    push_b(buf, b"\" y=\""); push_i(buf, ym);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#6b7280\" transform=\"rotate(-90,");
    push_i(buf, x); push_b(buf, b","); push_i(buf, ym); push_b(buf, b")\">");
    escape_xml(buf, label);
    push_b(buf, b"</text>");
}

pub fn svg_legend_item(buf: &mut Vec<u8>, si: i32, name: &str, color: u32, x: i32, y: i32, max_len: usize) {
    let hx = hex6(color);
    push_b(buf, b"<g data-legend=\"1\" data-series=\""); push_i(buf, si);
    push_b(buf, b"\">");
    push_b(buf, b"<rect x=\""); push_i(buf, x);
    push_b(buf, b"\" y=\""); push_i(buf, y);
    push_b(buf, b"\" width=\"12\" height=\"12\" rx=\"2\" fill=\"#"); buf.extend_from_slice(&hx);
    push_b(buf, b"\"/>");
    push_b(buf, b"<text x=\""); push_i(buf, x + 16);
    push_b(buf, b"\" y=\""); push_i(buf, y + 10);
    push_b(buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#374151\">");
    escape_xml(buf, truncate(name, max_len));
    push_b(buf, b"</text></g>");
}

pub fn svg_hgrid(buf: &mut Vec<u8>, x1: i32, x2: i32, y: i32) {
    push_b(buf, b"<line x1=\""); push_i(buf, x1);
    push_b(buf, b"\" y1=\""); push_i(buf, y);
    push_b(buf, b"\" x2=\""); push_i(buf, x2);
    push_b(buf, b"\" y2=\""); push_i(buf, y);
    push_b(buf, b"\" stroke=\"#e2e8f0\" stroke-width=\"0.5\"/>");
}

pub fn svg_vgrid(buf: &mut Vec<u8>, x: i32, y1: i32, y2: i32) {
    push_b(buf, b"<line x1=\""); push_i(buf, x);
    push_b(buf, b"\" y1=\""); push_i(buf, y1);
    push_b(buf, b"\" x2=\""); push_i(buf, x);
    push_b(buf, b"\" y2=\""); push_i(buf, y2);
    push_b(buf, b"\" stroke=\"#e2e8f0\" stroke-width=\"0.5\"/>");
}

pub fn svg_tick_y(buf: &mut Vec<u8>, x: i32, y: i32, val: f64) {
    push_b(buf, b"<text x=\""); push_i(buf, x);
    push_b(buf, b"\" y=\""); push_i(buf, y);
    push_b(buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\">");
    if val.abs() >= 1000.0 { push_i(buf, val as i32); } else { push_f2(buf, val); }
    push_b(buf, b"</text>");
}

pub fn svg_tick_x(buf: &mut Vec<u8>, x: i32, y: i32, val: f64) {
    push_b(buf, b"<text x=\""); push_i(buf, x);
    push_b(buf, b"\" y=\""); push_i(buf, y);
    push_b(buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\">");
    if val >= 10000.0 { push_i(buf, (val / 1000.0) as i32); push_b(buf, b"k"); }
    else if val >= 1000.0 { push_i(buf, val as i32); }
    else { push_f2(buf, val); }
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
        h[((c >> 20) & 0xF) as usize], h[((c >> 16) & 0xF) as usize],
        h[((c >> 12) & 0xF) as usize], h[((c >> 8) & 0xF) as usize],
        h[((c >> 4) & 0xF) as usize],  h[(c & 0xF) as usize],
    ]
}

#[inline]
pub fn push_hex(buf: &mut Vec<u8>, c: u32) {
    buf.push(b'#');
    buf.extend_from_slice(&hex6(c));
}

pub fn escape_xml(buf: &mut Vec<u8>, s: &str) {
    if !s.bytes().any(|b| b == b'&' || b == b'<' || b == b'>' || b == b'"' || b == b'\'') {
        buf.extend_from_slice(s.as_bytes());
        return;
    }
    for ch in s.chars() {
        match ch {
            '&'  => buf.extend_from_slice(b"&amp;"),
            '<'  => buf.extend_from_slice(b"&lt;"),
            '>'  => buf.extend_from_slice(b"&gt;"),
            '"'  => buf.extend_from_slice(b"&quot;"),
            '\'' => buf.extend_from_slice(b"&#39;"),
            c    => { let mut tmp = [0u8; 4]; buf.extend_from_slice(c.encode_utf8(&mut tmp).as_bytes()); }
        }
    }
}

pub fn escape_xml_s(s: &str) -> String {
    let mut buf = Vec::with_capacity(s.len() + 8);
    escape_xml(&mut buf, s);
    unsafe { String::from_utf8_unchecked(buf) }
}

pub fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max { s } else { &s[..max] }
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
    if n < 0 { buf.push(b'-'); n = -n; }
    if n == 0 { buf.push(b'0'); return; }
    let mut d = [0u8; 10]; let mut len = 0;
    while n > 0 { d[len] = (n % 10) as u8 + b'0'; n /= 10; len += 1; }
    for &b in d[..len].iter().rev() { buf.push(b); }
}

#[inline(always)]
pub fn push_f2(buf: &mut Vec<u8>, v: f64) {
    let neg = v < 0.0;
    let vi = (v.abs() * 100.0 + 0.5) as u64;
    let whole = vi / 100; let frac = vi % 100;
    if neg { buf.push(b'-'); }
    let mut d = [0u8; 20]; let mut len = 0; let mut w = whole;
    if w == 0 { d[0] = b'0'; len = 1; } else { while w > 0 { d[len] = (w % 10) as u8 + b'0'; w /= 10; len += 1; } }
    for &b in d[..len].iter().rev() { buf.push(b); }
    buf.push(b'.'); buf.push((frac / 10) as u8 + b'0'); buf.push((frac % 10) as u8 + b'0');
}

fn itoa_i32(mut n: i32) -> String {
    if n == 0 { return "0".to_string(); }
    let neg = n < 0;
    if neg { n = -n; }
    let mut digits = [0u8; 11];
    let mut pos = 11;
    while n > 0 {
        pos -= 1;
        digits[pos] = (n % 10) as u8 + b'0';
        n /= 10;
    }
    if neg { pos -= 1; digits[pos] = b'-'; }
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
        Self { buf: Vec::with_capacity(cap), w, h, pl, pt, pw: w - pl - pr, ph: h - pt - pb, hid: 0 }
    }

    pub fn open(&mut self, title: &str, rescalable: bool) {
        if rescalable {
            svg_open_rescalable(&mut self.buf, self.w, self.h, self.pl, self.pt, self.pw, self.ph);
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
            if grid && i > 0 { svg_hgrid(&mut self.buf, self.pl, self.pl + self.pw, y); }
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
                push_b(&mut self.buf, b"<line x1=\""); push_i(&mut self.buf, self.pl);
                push_b(&mut self.buf, b"\" y1=\""); push_i(&mut self.buf, y);
                push_b(&mut self.buf, b"\" x2=\""); push_i(&mut self.buf, self.pl + self.pw);
                push_b(&mut self.buf, b"\" y2=\""); push_i(&mut self.buf, y);
                push_b(&mut self.buf, b"\" stroke=\"#e2e8f0\" stroke-width=\".5\" class=\"sp-gl\"/>");
            }
            push_b(&mut self.buf, b"<text x=\""); push_i(&mut self.buf, self.pl - 4);
            push_b(&mut self.buf, b"\" y=\""); push_i(&mut self.buf, y + 3);
            push_b(&mut self.buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\" class=\"sp-yt\">");
            push_f2(&mut self.buf, v);
            push_b(&mut self.buf, b"</text>");
        }
    }

    pub fn y_grid_int(&mut self, n: i32, max_count: f64, grid: bool) {
        for i in 0..=n {
            let f = i as f64 / n as f64;
            let y = self.pt + ((1.0 - f) * self.ph as f64) as i32;
            let v = (f * max_count).round() as i32;
            if grid && i > 0 { svg_hgrid(&mut self.buf, self.pl, self.pl + self.pw, y); }
            push_b(&mut self.buf, b"<text x=\""); push_i(&mut self.buf, self.pl - 4);
            push_b(&mut self.buf, b"\" y=\""); push_i(&mut self.buf, y + 3);
            push_b(&mut self.buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#9ca3af\">");
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
            if grid && i > 0 { svg_vgrid(&mut self.buf, x, self.pt, self.pt + self.ph); }
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
            svg_legend_item(&mut self.buf, i as i32, name, palette_color(palette, i), x, self.pt + i as i32 * 22, 14);
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

    pub fn new_html(title: &str, w: i32, h: i32, pl: i32, pt: i32, pb: i32, pr: i32, cap: usize) -> Self {
        let hid = crate::html::hover::html_id();
        let mut buf = Vec::with_capacity(cap + 14_000);
        crate::html::hover::html_prefix(&mut buf, title, hid);
        Self { buf, w, h, pl, pt, pw: w - pl - pr, ph: h - pt - pb, hid }
    }
}
