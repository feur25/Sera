pub enum SvgPrimitive {
    Polygon {
        points: Vec<egui::Pos2>,
        fill: Option<egui::Color32>,
        stroke: Option<(egui::Color32, f32)>,
    },
    Polyline {
        points: Vec<egui::Pos2>,
        stroke: Option<(egui::Color32, f32)>,
    },
    Circle {
        center: egui::Pos2,
        radius: f32,
        fill: Option<egui::Color32>,
        stroke: Option<(egui::Color32, f32)>,
    },
    Text {
        pos: egui::Pos2,
        anchor: TextAnchor,
        text: String,
        size: f32,
        color: egui::Color32,
    },
}

#[derive(Clone, Copy)]
pub enum TextAnchor {
    Start,
    Middle,
    End,
}

pub struct SvgScene {
    pub view_box: egui::Rect,
    pub primitives: Vec<SvgPrimitive>,
}

impl Default for SvgScene {
    fn default() -> Self {
        Self {
            view_box: egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(100.0, 100.0)),
            primitives: Vec::new(),
        }
    }
}

struct Attrs<'a> {
    pairs: Vec<(&'a str, &'a str)>,
}

impl<'a> Attrs<'a> {
    fn parse(tag: &'a str) -> Self {
        let mut pairs = Vec::new();
        let bytes = tag.as_bytes();
        let mut i = 0usize;
        while i < bytes.len() {
            while i < bytes.len() && (bytes[i] as char).is_whitespace() {
                i += 1;
            }
            let key_start = i;
            while i < bytes.len() && bytes[i] != b'=' && !(bytes[i] as char).is_whitespace() {
                i += 1;
            }
            if key_start == i {
                break;
            }
            let key = &tag[key_start..i];
            while i < bytes.len() && (bytes[i] as char).is_whitespace() {
                i += 1;
            }
            if i >= bytes.len() || bytes[i] != b'=' {
                continue;
            }
            i += 1;
            while i < bytes.len() && (bytes[i] as char).is_whitespace() {
                i += 1;
            }
            if i >= bytes.len() || bytes[i] != b'"' {
                continue;
            }
            i += 1;
            let val_start = i;
            while i < bytes.len() && bytes[i] != b'"' {
                i += 1;
            }
            let val = &tag[val_start..i.min(bytes.len())];
            pairs.push((key, val));
            i += 1;
        }
        Self { pairs }
    }

    fn get(&self, key: &str) -> Option<&'a str> {
        self.pairs.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
    }

    fn is_hidden(&self) -> bool {
        let style = self.get("style").unwrap_or("");
        style.contains("display:none") || style.contains("visibility:hidden")
    }
}

fn parse_len(s: &str, whole: f32) -> f32 {
    let s = s.trim();
    if let Some(pct) = s.strip_suffix('%') {
        return pct.trim().parse::<f32>().unwrap_or(0.0) / 100.0 * whole;
    }
    s.parse::<f32>().unwrap_or(0.0)
}

fn parse_color(s: &str) -> Option<egui::Color32> {
    let s = s.trim();
    if s.is_empty() || s.eq_ignore_ascii_case("none") || s.eq_ignore_ascii_case("transparent") {
        return None;
    }
    if let Some(hex) = s.strip_prefix('#') {
        return match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                Some(egui::Color32::from_rgb(r, g, b))
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(egui::Color32::from_rgb(r, g, b))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(egui::Color32::from_rgba_unmultiplied(r, g, b, a))
            }
            _ => None,
        };
    }
    match s.to_ascii_lowercase().as_str() {
        "white" => Some(egui::Color32::WHITE),
        "black" => Some(egui::Color32::BLACK),
        _ => None,
    }
}

fn parse_fill(attrs: &Attrs, default_none: bool) -> Option<egui::Color32> {
    match attrs.get("fill") {
        Some(v) => parse_color(v),
        None => {
            if default_none {
                None
            } else {
                Some(egui::Color32::from_gray(80))
            }
        }
    }
}

fn parse_stroke(attrs: &Attrs) -> Option<(egui::Color32, f32)> {
    let color = attrs.get("stroke").and_then(parse_color)?;
    let width = attrs
        .get("stroke-width")
        .and_then(|s| s.trim().parse::<f32>().ok())
        .unwrap_or(1.0);
    Some((color, width))
}

fn parse_points_attr(points: &str) -> Vec<egui::Pos2> {
    let mut out = Vec::new();
    let mut nums = points.split([',', ' ', '\n', '\t']).filter(|s| !s.is_empty());
    while let (Some(x), Some(y)) = (nums.next(), nums.next()) {
        if let (Ok(x), Ok(y)) = (x.parse::<f32>(), y.parse::<f32>()) {
            out.push(egui::pos2(x, y));
        }
    }
    out
}

struct PathCursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> PathCursor<'a> {
    fn new(d: &'a str) -> Self {
        Self {
            bytes: d.as_bytes(),
            pos: 0,
        }
    }

    fn skip_sep(&mut self) {
        while self.pos < self.bytes.len() {
            let c = self.bytes[self.pos] as char;
            if c.is_whitespace() || c == ',' {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn peek_cmd(&mut self) -> Option<char> {
        self.skip_sep();
        self.bytes.get(self.pos).map(|&b| b as char).filter(|c| c.is_ascii_alphabetic())
    }

    fn take_cmd(&mut self) -> Option<char> {
        let c = self.peek_cmd()?;
        self.pos += 1;
        Some(c)
    }

    fn take_num(&mut self) -> Option<f32> {
        self.skip_sep();
        let start = self.pos;
        if self.pos < self.bytes.len() && (self.bytes[self.pos] == b'-' || self.bytes[self.pos] == b'+') {
            self.pos += 1;
        }
        let mut seen_dot = false;
        while self.pos < self.bytes.len() {
            let c = self.bytes[self.pos] as char;
            if c.is_ascii_digit() {
                self.pos += 1;
            } else if c == '.' && !seen_dot {
                seen_dot = true;
                self.pos += 1;
            } else if (c == 'e' || c == 'E')
                && self.pos + 1 < self.bytes.len()
                && (self.bytes[self.pos + 1].is_ascii_digit()
                    || self.bytes[self.pos + 1] == b'-'
                    || self.bytes[self.pos + 1] == b'+')
            {
                self.pos += 2;
            } else {
                break;
            }
        }
        if self.pos == start {
            return None;
        }
        std::str::from_utf8(&self.bytes[start..self.pos]).ok()?.parse::<f32>().ok()
    }

    fn take_flag(&mut self) -> Option<bool> {
        self.skip_sep();
        let c = *self.bytes.get(self.pos)?;
        if c == b'0' || c == b'1' {
            self.pos += 1;
            Some(c == b'1')
        } else {
            None
        }
    }
}

fn arc_to_points(
    x1: f32,
    y1: f32,
    rx: f32,
    ry: f32,
    x_rot_deg: f32,
    large_arc: bool,
    sweep: bool,
    x2: f32,
    y2: f32,
    out: &mut Vec<egui::Pos2>,
) {
    if rx.abs() < 1e-4 || ry.abs() < 1e-4 {
        out.push(egui::pos2(x2, y2));
        return;
    }
    let phi = x_rot_deg.to_radians();
    let (cos_phi, sin_phi) = (phi.cos(), phi.sin());
    let dx2 = (x1 - x2) / 2.0;
    let dy2 = (y1 - y2) / 2.0;
    let x1p = cos_phi * dx2 + sin_phi * dy2;
    let y1p = -sin_phi * dx2 + cos_phi * dy2;

    let mut rx = rx.abs();
    let mut ry = ry.abs();
    let lambda = (x1p * x1p) / (rx * rx) + (y1p * y1p) / (ry * ry);
    if lambda > 1.0 {
        let s = lambda.sqrt();
        rx *= s;
        ry *= s;
    }

    let sign: f32 = if large_arc == sweep { -1.0 } else { 1.0 };
    let num = (rx * rx * ry * ry - rx * rx * y1p * y1p - ry * ry * x1p * x1p).max(0.0);
    let den = rx * rx * y1p * y1p + ry * ry * x1p * x1p;
    let co = if den.abs() < 1e-6 { 0.0 } else { sign * (num / den).sqrt() };

    let cxp = co * rx * y1p / ry;
    let cyp = -co * ry * x1p / rx;

    let cx = cos_phi * cxp - sin_phi * cyp + (x1 + x2) / 2.0;
    let cy = sin_phi * cxp + cos_phi * cyp + (y1 + y2) / 2.0;

    let angle = |ux: f32, uy: f32, vx: f32, vy: f32| -> f32 {
        let dot = ux * vx + uy * vy;
        let len = ((ux * ux + uy * uy) * (vx * vx + vy * vy)).sqrt();
        if len < 1e-6 {
            return 0.0;
        }
        let mut a = (dot / len).clamp(-1.0, 1.0).acos();
        if ux * vy - uy * vx < 0.0 {
            a = -a;
        }
        a
    };

    let ux = (x1p - cxp) / rx;
    let uy = (y1p - cyp) / ry;
    let vx = (-x1p - cxp) / rx;
    let vy = (-y1p - cyp) / ry;

    let theta1 = angle(1.0, 0.0, ux, uy);
    let mut delta_theta = angle(ux, uy, vx, vy);
    if !sweep && delta_theta > 0.0 {
        delta_theta -= std::f32::consts::TAU;
    }
    if sweep && delta_theta < 0.0 {
        delta_theta += std::f32::consts::TAU;
    }

    let steps = ((delta_theta.abs() / (std::f32::consts::TAU / 64.0)).ceil() as usize).max(4);
    for i in 1..=steps {
        let t = theta1 + delta_theta * (i as f32 / steps as f32);
        let x = cx + rx * t.cos() * cos_phi - ry * t.sin() * sin_phi;
        let y = cy + rx * t.cos() * sin_phi + ry * t.sin() * cos_phi;
        out.push(egui::pos2(x, y));
    }
}

fn cubic_to_points(p0: egui::Pos2, p1: egui::Pos2, p2: egui::Pos2, p3: egui::Pos2, out: &mut Vec<egui::Pos2>) {
    let steps = 16;
    for i in 1..=steps {
        let t = i as f32 / steps as f32;
        let mt = 1.0 - t;
        let x = mt * mt * mt * p0.x + 3.0 * mt * mt * t * p1.x + 3.0 * mt * t * t * p2.x + t * t * t * p3.x;
        let y = mt * mt * mt * p0.y + 3.0 * mt * mt * t * p1.y + 3.0 * mt * t * t * p2.y + t * t * t * p3.y;
        out.push(egui::pos2(x, y));
    }
}

fn quad_to_points(p0: egui::Pos2, p1: egui::Pos2, p2: egui::Pos2, out: &mut Vec<egui::Pos2>) {
    let steps = 12;
    for i in 1..=steps {
        let t = i as f32 / steps as f32;
        let mt = 1.0 - t;
        let x = mt * mt * p0.x + 2.0 * mt * t * p1.x + t * t * p2.x;
        let y = mt * mt * p0.y + 2.0 * mt * t * p1.y + t * t * p2.y;
        out.push(egui::pos2(x, y));
    }
}

fn parse_path_subpaths(d: &str) -> Vec<Vec<egui::Pos2>> {
    let mut subpaths = Vec::new();
    let mut current: Vec<egui::Pos2> = Vec::new();
    let mut cur = egui::pos2(0.0, 0.0);
    let mut sub_start = egui::pos2(0.0, 0.0);
    let mut cursor = PathCursor::new(d);
    let mut cmd = cursor.take_cmd();

    while let Some(c) = cmd {
        let relative = c.is_ascii_lowercase();
        match c.to_ascii_uppercase() {
            'M' => {
                if !current.is_empty() {
                    subpaths.push(std::mem::take(&mut current));
                }
                let (Some(x), Some(y)) = (cursor.take_num(), cursor.take_num()) else {
                    break;
                };
                cur = if relative {
                    egui::pos2(cur.x + x, cur.y + y)
                } else {
                    egui::pos2(x, y)
                };
                sub_start = cur;
                current.push(cur);
            }
            'L' => {
                let (Some(x), Some(y)) = (cursor.take_num(), cursor.take_num()) else {
                    break;
                };
                cur = if relative {
                    egui::pos2(cur.x + x, cur.y + y)
                } else {
                    egui::pos2(x, y)
                };
                current.push(cur);
            }
            'H' => {
                let Some(x) = cursor.take_num() else { break };
                cur = egui::pos2(if relative { cur.x + x } else { x }, cur.y);
                current.push(cur);
            }
            'V' => {
                let Some(y) = cursor.take_num() else { break };
                cur = egui::pos2(cur.x, if relative { cur.y + y } else { y });
                current.push(cur);
            }
            'C' => {
                let nums: Option<[f32; 6]> = (|| {
                    Some([
                        cursor.take_num()?,
                        cursor.take_num()?,
                        cursor.take_num()?,
                        cursor.take_num()?,
                        cursor.take_num()?,
                        cursor.take_num()?,
                    ])
                })();
                let Some([x1, y1, x2, y2, x, y]) = nums else {
                    break;
                };
                let base = cur;
                let to_abs = |px: f32, py: f32| {
                    if relative {
                        egui::pos2(base.x + px, base.y + py)
                    } else {
                        egui::pos2(px, py)
                    }
                };
                let p1 = to_abs(x1, y1);
                let p2 = to_abs(x2, y2);
                let p3 = to_abs(x, y);
                cubic_to_points(cur, p1, p2, p3, &mut current);
                cur = p3;
            }
            'Q' => {
                let nums: Option<[f32; 4]> = (|| {
                    Some([cursor.take_num()?, cursor.take_num()?, cursor.take_num()?, cursor.take_num()?])
                })();
                let Some([x1, y1, x, y]) = nums else {
                    break;
                };
                let base = cur;
                let to_abs = |px: f32, py: f32| {
                    if relative {
                        egui::pos2(base.x + px, base.y + py)
                    } else {
                        egui::pos2(px, py)
                    }
                };
                let p1 = to_abs(x1, y1);
                let p2 = to_abs(x, y);
                quad_to_points(cur, p1, p2, &mut current);
                cur = p2;
            }
            'A' => {
                let rx = cursor.take_num();
                let ry = cursor.take_num();
                let rot = cursor.take_num();
                let large_arc = cursor.take_flag();
                let sweep = cursor.take_flag();
                let x = cursor.take_num();
                let y = cursor.take_num();
                let (Some(rx), Some(ry), Some(rot), Some(large_arc), Some(sweep), Some(x), Some(y)) =
                    (rx, ry, rot, large_arc, sweep, x, y)
                else {
                    break;
                };
                let base = cur;
                let end = if relative {
                    egui::pos2(base.x + x, base.y + y)
                } else {
                    egui::pos2(x, y)
                };
                arc_to_points(cur.x, cur.y, rx, ry, rot, large_arc, sweep, end.x, end.y, &mut current);
                cur = end;
            }
            'Z' => {
                if current.last() != Some(&sub_start) {
                    current.push(sub_start);
                }
                cur = sub_start;
            }
            _ => {}
        }
        cmd = cursor.take_cmd();
    }

    if !current.is_empty() {
        subpaths.push(current);
    }
    subpaths
}

fn push_tag(scene: &mut SvgScene, name: &str, attrs: &Attrs, text: Option<&str>) {
    if attrs.is_hidden() {
        return;
    }
    match name {
        "svg" => {
            if let Some(vb) = attrs.get("viewBox") {
                let nums: Vec<f32> = vb
                    .split_whitespace()
                    .filter_map(|s| s.parse::<f32>().ok())
                    .collect();
                if nums.len() == 4 {
                    scene.view_box =
                        egui::Rect::from_min_size(egui::pos2(nums[0], nums[1]), egui::vec2(nums[2], nums[3]));
                    return;
                }
            }
            let w = attrs.get("width").map(|s| parse_len(s, 0.0)).unwrap_or(0.0);
            let h = attrs.get("height").map(|s| parse_len(s, 0.0)).unwrap_or(0.0);
            if w > 0.0 && h > 0.0 {
                scene.view_box = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(w, h));
            }
        }
        "rect" => {
            if attrs.get("class") == Some("sp-bg") {
                return;
            }
            let x = attrs.get("x").map(|s| parse_len(s, scene.view_box.width())).unwrap_or(0.0);
            let y = attrs.get("y").map(|s| parse_len(s, scene.view_box.height())).unwrap_or(0.0);
            let w = attrs.get("width").map(|s| parse_len(s, scene.view_box.width())).unwrap_or(0.0);
            let h = attrs.get("height").map(|s| parse_len(s, scene.view_box.height())).unwrap_or(0.0);
            if w <= 0.0 || h <= 0.0 {
                return;
            }
            let points = vec![
                egui::pos2(x, y),
                egui::pos2(x + w, y),
                egui::pos2(x + w, y + h),
                egui::pos2(x, y + h),
            ];
            scene.primitives.push(SvgPrimitive::Polygon {
                points,
                fill: parse_fill(attrs, false),
                stroke: parse_stroke(attrs),
            });
        }
        "circle" => {
            let cx = attrs.get("cx").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
            let cy = attrs.get("cy").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
            let r = attrs.get("r").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
            if r <= 0.0 {
                return;
            }
            scene.primitives.push(SvgPrimitive::Circle {
                center: egui::pos2(cx, cy),
                radius: r,
                fill: parse_fill(attrs, false),
                stroke: parse_stroke(attrs),
            });
        }
        "ellipse" => {
            let cx = attrs.get("cx").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
            let cy = attrs.get("cy").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
            let rx = attrs.get("rx").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
            let ry = attrs.get("ry").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
            if rx <= 0.0 || ry <= 0.0 {
                return;
            }
            scene.primitives.push(SvgPrimitive::Circle {
                center: egui::pos2(cx, cy),
                radius: (rx + ry) * 0.5,
                fill: parse_fill(attrs, false),
                stroke: parse_stroke(attrs),
            });
        }
        "line" => {
            let x1 = attrs.get("x1").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
            let y1 = attrs.get("y1").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
            let x2 = attrs.get("x2").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
            let y2 = attrs.get("y2").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
            let stroke = parse_stroke(attrs).or(Some((egui::Color32::from_gray(140), 1.0)));
            scene.primitives.push(SvgPrimitive::Polyline {
                points: vec![egui::pos2(x1, y1), egui::pos2(x2, y2)],
                stroke,
            });
        }
        "polyline" | "polygon" => {
            let Some(points) = attrs.get("points").map(parse_points_attr) else {
                return;
            };
            if points.len() < 2 {
                return;
            }
            if name == "polygon" {
                scene.primitives.push(SvgPrimitive::Polygon {
                    points,
                    fill: parse_fill(attrs, false),
                    stroke: parse_stroke(attrs),
                });
            } else {
                scene.primitives.push(SvgPrimitive::Polyline {
                    points,
                    stroke: parse_stroke(attrs).or(Some((egui::Color32::from_gray(80), 1.5))),
                });
            }
        }
        "path" => {
            let Some(d) = attrs.get("d") else {
                return;
            };
            let fill = parse_fill(attrs, true);
            let stroke = parse_stroke(attrs);
            for points in parse_path_subpaths(d) {
                if points.len() < 2 {
                    continue;
                }
                if fill.is_some() {
                    scene.primitives.push(SvgPrimitive::Polygon {
                        points,
                        fill,
                        stroke,
                    });
                } else {
                    scene.primitives.push(SvgPrimitive::Polyline {
                        points,
                        stroke: stroke.or(Some((egui::Color32::from_gray(80), 1.5))),
                    });
                }
            }
        }
        "text" => {
            let x = attrs.get("x").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
            let y = attrs.get("y").and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
            let Some(text) = text.map(|s| s.trim()).filter(|s| !s.is_empty()) else {
                return;
            };
            let anchor = match attrs.get("text-anchor") {
                Some("middle") => TextAnchor::Middle,
                Some("end") => TextAnchor::End,
                _ => TextAnchor::Start,
            };
            let size = attrs
                .get("font-size")
                .and_then(|s| s.parse::<f32>().ok())
                .unwrap_or(11.0);
            let color = attrs.get("fill").and_then(parse_color).unwrap_or(egui::Color32::from_gray(60));
            scene.primitives.push(SvgPrimitive::Text {
                pos: egui::pos2(x, y),
                anchor,
                text: html_unescape(text),
                size,
                color,
            });
        }
        _ => {}
    }
}

fn html_unescape(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

pub fn extract_svg(html: &str) -> Option<&str> {
    let start = html.find("<svg")?;
    let end = html[start..].find("</svg>")? + start + "</svg>".len();
    Some(&html[start..end])
}

pub fn parse_svg_scene(svg: &str) -> SvgScene {
    let mut scene = SvgScene::default();

    let bytes = svg.as_bytes();
    let mut cursor = 0usize;
    while let Some(lt) = svg[cursor..].find('<') {
        let start = cursor + lt;
        if svg[start..].starts_with("<!--") {
            cursor = svg[start..].find("-->").map(|i| start + i + 3).unwrap_or(svg.len());
            continue;
        }
        if svg[start..].starts_with("</") {
            let Some(gt) = svg[start..].find('>') else { break };
            cursor = start + gt + 1;
            continue;
        }
        if svg[start..].starts_with("<style") {
            cursor = svg[start..]
                .find("</style>")
                .map(|i| start + i + "</style>".len())
                .unwrap_or(svg.len());
            continue;
        }
        let Some(gt) = svg[start..].find('>') else { break };
        let gt = start + gt;
        let self_closing = bytes.get(gt.wrapping_sub(1)) == Some(&b'/');
        let inner_end = if self_closing { gt - 1 } else { gt };
        let tag_content = &svg[start + 1..inner_end];
        let name_end = tag_content
            .find(|c: char| c.is_whitespace())
            .unwrap_or(tag_content.len());
        let name = &tag_content[..name_end];
        let attrs = Attrs::parse(&tag_content[name_end..]);

        if name == "text" && !self_closing {
            let close = svg[gt + 1..].find("</text>");
            let inner = close.map(|i| &svg[gt + 1..gt + 1 + i]);
            push_tag(&mut scene, name, &attrs, inner);
            cursor = close.map(|i| gt + 1 + i + "</text>".len()).unwrap_or(gt + 1);
        } else {
            push_tag(&mut scene, name, &attrs, None);
            cursor = gt + 1;
        }
    }

    scene
}

fn fit_transform(view_box: egui::Rect, target: egui::Rect) -> (f32, egui::Vec2) {
    let vb_w = view_box.width().max(1e-3);
    let vb_h = view_box.height().max(1e-3);
    let scale = (target.width() / vb_w).min(target.height() / vb_h);
    let scaled = egui::vec2(vb_w * scale, vb_h * scale);
    let offset = target.min.to_vec2() + (target.size() - scaled) * 0.5 - view_box.min.to_vec2() * scale;
    (scale, offset)
}

pub fn paint_svg_scene(painter: &egui::Painter, scene: &SvgScene, target: egui::Rect) {
    let (scale, offset) = fit_transform(scene.view_box, target);
    let map = |p: egui::Pos2| egui::pos2(p.x * scale + offset.x, p.y * scale + offset.y);

    for primitive in &scene.primitives {
        match primitive {
            SvgPrimitive::Polygon { points, fill, stroke } => {
                let pts: Vec<egui::Pos2> = points.iter().copied().map(map).collect();
                if pts.len() < 3 {
                    continue;
                }
                let fill_color = fill.unwrap_or(egui::Color32::TRANSPARENT);
                let stroke = stroke
                    .map(|(c, w)| egui::Stroke::new((w * scale).max(0.5), c))
                    .unwrap_or(egui::Stroke::NONE);
                painter.add(egui::Shape::convex_polygon(pts, fill_color, stroke));
            }
            SvgPrimitive::Polyline { points, stroke } => {
                let pts: Vec<egui::Pos2> = points.iter().copied().map(map).collect();
                if pts.len() < 2 {
                    continue;
                }
                let (color, width) = stroke.unwrap_or((egui::Color32::from_gray(80), 1.0));
                painter.add(egui::Shape::line(pts, egui::Stroke::new((width * scale).max(0.5), color)));
            }
            SvgPrimitive::Circle { center, radius, fill, stroke } => {
                let c = map(*center);
                let r = radius * scale;
                if r <= 0.0 {
                    continue;
                }
                let fill_color = fill.unwrap_or(egui::Color32::TRANSPARENT);
                let stroke = stroke
                    .map(|(c, w)| egui::Stroke::new((w * scale).max(0.5), c))
                    .unwrap_or(egui::Stroke::NONE);
                painter.circle(c, r, fill_color, stroke);
            }
            SvgPrimitive::Text { pos, anchor, text, size, color } => {
                let p = map(*pos);
                let align = match anchor {
                    TextAnchor::Start => egui::Align2::LEFT_BOTTOM,
                    TextAnchor::Middle => egui::Align2::CENTER_BOTTOM,
                    TextAnchor::End => egui::Align2::RIGHT_BOTTOM,
                };
                painter.text(
                    p,
                    align,
                    text,
                    egui::FontId::proportional((size * scale).max(6.0)),
                    *color,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_svg_finds_the_svg_block_inside_a_full_html_page() {
        let html = "<html><body><svg width=\"10\" height=\"10\"><rect/></svg></body></html>";
        let svg = extract_svg(html).unwrap();
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn parse_svg_scene_reads_the_viewbox() {
        let scene = parse_svg_scene("<svg viewBox=\"0 0 900 480\"></svg>");
        assert_eq!(scene.view_box.width(), 900.0);
        assert_eq!(scene.view_box.height(), 480.0);
    }

    #[test]
    fn parse_svg_scene_turns_a_rect_into_a_four_point_polygon_with_its_fill() {
        let scene = parse_svg_scene(
            "<svg viewBox=\"0 0 100 100\"><rect x=\"1\" y=\"2\" width=\"10\" height=\"20\" fill=\"#ff0000\"/></svg>",
        );
        let SvgPrimitive::Polygon { points, fill, .. } = &scene.primitives[0] else {
            panic!("expected a polygon");
        };
        assert_eq!(points.len(), 4);
        assert_eq!(*fill, Some(egui::Color32::from_rgb(255, 0, 0)));
    }

    #[test]
    fn parse_svg_scene_skips_the_synthetic_background_rect() {
        let scene = parse_svg_scene(
            "<svg viewBox=\"0 0 100 100\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/></svg>",
        );
        assert!(scene.primitives.is_empty());
    }

    #[test]
    fn parse_svg_scene_skips_elements_hidden_via_inline_style() {
        let scene = parse_svg_scene(
            "<svg viewBox=\"0 0 100 100\"><line x1=\"0\" y1=\"0\" x2=\"10\" y2=\"10\" style=\"display:none\"/></svg>",
        );
        assert!(scene.primitives.is_empty());
    }

    #[test]
    fn parse_svg_scene_reads_text_content_and_anchor() {
        let scene = parse_svg_scene(
            "<svg viewBox=\"0 0 100 100\"><text x=\"5\" y=\"6\" text-anchor=\"middle\" fill=\"#123456\">Hello</text></svg>",
        );
        let SvgPrimitive::Text { text, anchor, color, .. } = &scene.primitives[0] else {
            panic!("expected text");
        };
        assert_eq!(text, "Hello");
        assert!(matches!(anchor, TextAnchor::Middle));
        assert_eq!(*color, egui::Color32::from_rgb(0x12, 0x34, 0x56));
    }

    #[test]
    fn parse_path_subpaths_handles_a_real_pie_wedge_arc_and_closes_it() {
        let d = "M223.20,243.20 L223.20,88.22 A154.98,154.98 0 0,1 223.20,398.18 Z";
        let subpaths = parse_path_subpaths(d);
        assert_eq!(subpaths.len(), 1);
        let pts = &subpaths[0];
        assert!(pts.len() > 4);
        assert_eq!(*pts.first().unwrap(), *pts.last().unwrap());
    }

    #[test]
    fn parse_path_subpaths_handles_a_real_two_arc_sunburst_ring_segment() {
        let d = "M 350.00 56.00 A 308 308 0 1 1 168.96 613.18 L 314.73 412.54 A 60 60 0 1 0 350.00 304.00 Z";
        let subpaths = parse_path_subpaths(d);
        assert_eq!(subpaths.len(), 1);
        assert!(subpaths[0].len() > 8);
    }

    #[test]
    fn parse_svg_scene_extracts_a_pie_chart_end_to_end_into_filled_polygons() {
        let svg = "<svg viewBox=\"0 0 720 440\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/><path data-idx=\"0\" d=\"M223.20,243.20 L223.20,88.22 A154.98,154.98 0 0,1 223.20,398.18 Z\" fill=\"#636efa\" stroke=\"#0d1117\" stroke-width=\"1.8\"/></svg>";
        let scene = parse_svg_scene(svg);
        assert_eq!(scene.primitives.len(), 1);
        let SvgPrimitive::Polygon { fill, stroke, .. } = &scene.primitives[0] else {
            panic!("expected a filled polygon for the pie wedge");
        };
        assert_eq!(*fill, Some(egui::Color32::from_rgb(0x63, 0x6e, 0xfa)));
        assert!(stroke.is_some());
    }

    #[test]
    fn arc_to_points_starts_moving_away_from_the_start_point() {
        let mut out = Vec::new();
        arc_to_points(0.0, 0.0, 50.0, 50.0, 0.0, false, true, 100.0, 0.0, &mut out);
        assert!(!out.is_empty());
        assert!(out.iter().any(|p| p.y.abs() > 1.0));
    }

    const MULTI_PANEL_COMPOSITE_FAMILIES: &[&str] = &["facet", "joint"];
    const CLIENT_SIDE_RENDERED_VARIANTS: &[(&str, &str)] = &[
        ("sunburst", "zoomable"),
        ("wordcloud", "labelmap"),
    ];

    #[test]
    fn every_2d_catalog_entry_converts_into_a_non_empty_native_scene() {
        let mut checked = 0usize;
        let mut empty: Vec<String> = Vec::new();
        for (family, variants) in crate::plot::chart_demo_registry::families_2d() {
            if MULTI_PANEL_COMPOSITE_FAMILIES.contains(&family.as_str()) {
                continue;
            }
            for (variant, entry) in variants {
                if CLIENT_SIDE_RENDERED_VARIANTS.contains(&(family.as_str(), variant.as_str())) {
                    continue;
                }
                let Some(html) = crate::plot::chart_demo_registry::render_demo_html(entry) else {
                    continue;
                };
                let Some(svg) = extract_svg(&html) else {
                    empty.push(format!("{family}/{variant} (no <svg> found)"));
                    continue;
                };
                checked += 1;
                let scene = parse_svg_scene(svg);
                if scene.primitives.is_empty() {
                    empty.push(format!("{family}/{variant}"));
                }
            }
        }
        assert!(
            checked > 50,
            "expected to convert a substantial share of the 2D catalog, only checked {checked}"
        );
        assert!(
            empty.is_empty(),
            "{} of {} 2D catalog entries converted to zero native primitives:\n{}",
            empty.len(),
            checked,
            empty.join("\n")
        );
    }

    #[test]
    fn multi_panel_composite_families_and_client_side_variants_are_still_registered_2d_entries() {
        let families = crate::plot::chart_demo_registry::families_2d();
        for name in MULTI_PANEL_COMPOSITE_FAMILIES {
            assert!(
                families.iter().any(|(f, _)| f == name),
                "'{name}' is no longer a registered 2D family -- drop it from MULTI_PANEL_COMPOSITE_FAMILIES"
            );
        }
        for (family, variant) in CLIENT_SIDE_RENDERED_VARIANTS {
            let found = families
                .iter()
                .find(|(f, _)| f == family)
                .is_some_and(|(_, vs)| vs.iter().any(|(v, _)| v == variant));
            assert!(
                found,
                "'{family}/{variant}' is no longer a registered 2D entry -- drop it from CLIENT_SIDE_RENDERED_VARIANTS"
            );
        }
    }
}
