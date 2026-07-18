use base64::Engine;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

mod anchors;

static MARKER_ID: AtomicUsize = AtomicUsize::new(0);

fn next_mid() -> usize {
    MARKER_ID.fetch_add(1, Ordering::Relaxed)
}

fn parse_pts(pts: Vec<Vec<f64>>) -> Vec<(f64, f64)> {
    pts.into_iter()
        .filter_map(|p| {
            if p.len() >= 2 {
                Some((p[0], p[1]))
            } else {
                None
            }
        })
        .collect()
}

fn line_intersect(p1: (f64, f64), p2: (f64, f64), a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let d1 = a * p1.0 + b * p1.1 - c;
    let d2 = a * p2.0 + b * p2.1 - c;
    let denom = d1 - d2;
    if denom.abs() < 1e-12 {
        return None;
    }
    let t = d1 / denom;
    Some((p1.0 + t * (p2.0 - p1.0), p1.1 + t * (p2.1 - p1.1)))
}

fn clip_half_plane(poly: &[(f64, f64)], a: f64, b: f64, c: f64) -> Vec<(f64, f64)> {
    let n = poly.len();
    if n == 0 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(n + 1);
    for i in 0..n {
        let cur = poly[i];
        let prev = poly[(i + n - 1) % n];
        let cur_in = a * cur.0 + b * cur.1 <= c;
        let prev_in = a * prev.0 + b * prev.1 <= c;
        if cur_in != prev_in {
            if let Some(ix) = line_intersect(prev, cur, a, b, c) {
                out.push(ix);
            }
        }
        if cur_in {
            out.push(cur);
        }
    }
    out
}

/// Bounded Voronoi diagram via iterative half-plane clipping: each site's
/// cell starts as the bounding box and is cut down by the perpendicular
/// bisector against every other site. O(n^2) per cell, O(n^3) total —
/// fine at the tens-to-low-hundreds site counts these diagrams are drawn
/// at, and far simpler to get right than a Fortune's-algorithm sweep line.
fn voronoi_cells(sites: &[(f64, f64)], bx: f64, by: f64, bw: f64, bh: f64) -> Vec<Vec<(f64, f64)>> {
    let bbox = vec![(bx, by), (bx + bw, by), (bx + bw, by + bh), (bx, by + bh)];
    sites
        .iter()
        .enumerate()
        .map(|(i, &(sx, sy))| {
            let mut poly = bbox.clone();
            for (j, &(ox, oy)) in sites.iter().enumerate() {
                if i == j || poly.is_empty() {
                    continue;
                }
                let mx = (sx + ox) / 2.0;
                let my = (sy + oy) / 2.0;
                let a = ox - sx;
                let b = oy - sy;
                let c = a * mx + b * my;
                poly = clip_half_plane(&poly, a, b, c);
            }
            poly
        })
        .collect()
}

fn pts_to_svg(pts: &[(f64, f64)]) -> String {
    pts.iter()
        .map(|(x, y)| format!("{:.2},{:.2}", x, y))
        .collect::<Vec<_>>()
        .join(" ")
}

fn catmull_rom(pts: &[(f64, f64)], tension: f64) -> String {
    let n = pts.len();
    if n == 0 {
        return String::new();
    }
    if n == 1 {
        return format!("M {:.2},{:.2}", pts[0].0, pts[0].1);
    }
    let mut d = format!("M {:.2},{:.2}", pts[0].0, pts[0].1);
    for i in 0..n - 1 {
        let p0 = if i == 0 { pts[0] } else { pts[i - 1] };
        let p1 = pts[i];
        let p2 = pts[i + 1];
        let p3 = if i + 2 < n { pts[i + 2] } else { pts[n - 1] };
        let cp1x = p1.0 + (p2.0 - p0.0) * tension / 6.0;
        let cp1y = p1.1 + (p2.1 - p0.1) * tension / 6.0;
        let cp2x = p2.0 - (p3.0 - p1.0) * tension / 6.0;
        let cp2y = p2.1 - (p3.1 - p1.1) * tension / 6.0;
        d.push_str(&format!(
            " C {:.2},{:.2} {:.2},{:.2} {:.2},{:.2}",
            cp1x, cp1y, cp2x, cp2y, p2.0, p2.1
        ));
    }
    d
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn clip_path_css(clip: &str) -> String {
    match clip {
        "circle" => "circle(50%)".into(),
        "diamond" => "polygon(50% 0%, 100% 50%, 50% 100%, 0% 50%)".into(),
        "hex" | "hexagon" => "polygon(25% 0%, 75% 0%, 100% 50%, 75% 100%, 25% 100%, 0% 50%)".into(),
        "tri" | "triangle" => "polygon(50% 0%, 100% 100%, 0% 100%)".into(),
        "pent" | "pentagon" => "polygon(50% 0%, 100% 38%, 82% 100%, 18% 100%, 0% 38%)".into(),
        _ => String::new(),
    }
}

fn name_attr(name: &str) -> String {
    if name.is_empty() {
        String::new()
    } else {
        format!(" data-sp-name=\"{}\"", escape_html(name))
    }
}

fn grp_attr(group: &str) -> String {
    if group.is_empty() {
        String::new()
    } else {
        format!(" data-sp-grp=\"{}\"", escape_html(group))
    }
}

fn guess_mime(path: &str) -> &'static str {
    let lower = path.to_lowercase();
    if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg"
    } else if lower.ends_with(".gif") {
        "image/gif"
    } else if lower.ends_with(".webp") {
        "image/webp"
    } else if lower.ends_with(".svg") {
        "image/svg+xml"
    } else {
        "application/octet-stream"
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
enum Layer {
    Bg,
    Fg,
}

impl Layer {
    fn from_str(s: &str) -> Self {
        if s == "bg" {
            Layer::Bg
        } else {
            Layer::Fg
        }
    }
}

#[derive(Clone)]
struct PlacedInfo {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    native_w: f64,
    native_h: f64,
    rotation: f64,
    element_idx: usize,
    plot: Option<anchors::PlotFrame>,
}

#[derive(Clone, Serialize, Deserialize)]
enum El {
    Chart {
        html: String,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        native_w: f64,
        native_h: f64,
        rotation: f64,
        opacity: f64,
        clip: String,
        group: String,
        name: String,
        ref_id: usize,
    },
    Image {
        src: String,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        rotation: f64,
        opacity: f64,
        clip: String,
        group: String,
        name: String,
    },
    Text {
        content: String,
        x: f64,
        y: f64,
        size: f64,
        color: String,
        opacity: f64,
        rotation: f64,
        anchor: String,
        weight: String,
        ls: f64,
        font: String,
        layer: Layer,
        name: String,
        group: String,
    },
    Line {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        color: String,
        width: f64,
        dash: String,
        opacity: f64,
        cap: String,
        layer: Layer,
        group: String,
        name: String,
    },
    Curve {
        pts: Vec<(f64, f64)>,
        color: String,
        width: f64,
        opacity: f64,
        tension: f64,
        fill: String,
        layer: Layer,
        name: String,
    },
    Connector {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        color: String,
        width: f64,
        opacity: f64,
        bend: f64,
        layer: Layer,
        name: String,
    },
    Circle {
        cx: f64,
        cy: f64,
        r: f64,
        fill: String,
        stroke: String,
        sw: f64,
        opacity: f64,
        layer: Layer,
        group: String,
        name: String,
    },
    Ring {
        cx: f64,
        cy: f64,
        r_inner: f64,
        r_outer: f64,
        fill: String,
        stroke: String,
        sw: f64,
        opacity: f64,
        layer: Layer,
        name: String,
    },
    Rect {
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        fill: String,
        stroke: String,
        sw: f64,
        rx: f64,
        opacity: f64,
        rotation: f64,
        layer: Layer,
        name: String,
        group: String,
    },
    Polygon {
        pts: Vec<(f64, f64)>,
        fill: String,
        stroke: String,
        sw: f64,
        opacity: f64,
        layer: Layer,
        name: String,
    },
    RawPath {
        d: String,
        fill: String,
        stroke: String,
        sw: f64,
        opacity: f64,
        layer: Layer,
        name: String,
    },
    Arrow {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        color: String,
        width: f64,
        head_size: f64,
        opacity: f64,
        layer: Layer,
        name: String,
    },
    Annotate {
        text: String,
        ax: f64,
        ay: f64,
        tx: f64,
        ty: f64,
        color: String,
        size: f64,
        line_dash: String,
        lw: f64,
        bg: String,
        layer: Layer,
        name: String,
    },
    GradDef {
        id: String,
        from_color: String,
        to_color: String,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    },
}

fn render_el(el: &El, defs: &mut String, body: &mut String) {
    match el {
        El::Text {
            content,
            x,
            y,
            size,
            color,
            opacity,
            rotation,
            anchor,
            weight,
            ls,
            font,
            name,
            group,
            ..
        } => {
            let rot = if rotation.abs() > 0.001 {
                format!(" transform=\"rotate({:.2},{:.2},{:.2})\"", rotation, x, y)
            } else {
                String::new()
            };
            let lines: Vec<&str> = content.split('\n').collect();
            body.push_str(&format!(
                "<text x=\"{:.2}\" y=\"{:.2}\" font-size=\"{:.2}\" fill=\"{}\" \
                 opacity=\"{:.4}\" font-weight=\"{}\" text-anchor=\"{}\" \
                 font-family=\"{}\" letter-spacing=\"{:.2}\"{}{}{}>\n",
                x, y, size, color, opacity, weight, anchor, font, ls, rot, name_attr(name), grp_attr(group)
            ));
            for (i, line) in lines.iter().enumerate() {
                let dy = if i == 0 { 0.0 } else { size * 1.25 };
                body.push_str(&format!(
                    "<tspan x=\"{:.2}\" dy=\"{:.2}\">{}</tspan>",
                    x,
                    dy,
                    escape_html(line)
                ));
            }
            body.push_str("</text>\n");
        }

        El::Line {
            x1,
            y1,
            x2,
            y2,
            color,
            width,
            dash,
            opacity,
            cap,
            group,
            name,
            ..
        } => {
            let da = if dash.is_empty() {
                String::new()
            } else {
                format!(" stroke-dasharray=\"{}\"", dash)
            };
            if group.is_empty() {
                body.push_str(&format!(
                    "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" \
                     stroke=\"{}\" stroke-width=\"{:.2}\" opacity=\"{:.4}\" \
                     stroke-linecap=\"{}\"{}{}/>\n",
                    x1, y1, x2, y2, color, width, opacity, cap, da, name_attr(name)
                ));
            } else {
                body.push_str(&format!(
                    "<line class=\"sp-hvl\" data-sp-grp=\"{}\" data-sw=\"{:.2}\" data-op=\"{:.4}\" \
                     x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" \
                     stroke=\"{}\" stroke-width=\"{:.2}\" stroke-opacity=\"{:.4}\" \
                     stroke-linecap=\"{}\"{}{} pointer-events=\"none\"/>\n\
                     <line class=\"sp-hvh\" data-sp-grp=\"{}\" \
                     x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" \
                     stroke=\"transparent\" stroke-width=\"12\" pointer-events=\"stroke\" fill=\"none\"/>\n",
                    group, width, opacity,
                    x1, y1, x2, y2, color, width, opacity, cap, da, name_attr(name),
                    group, x1, y1, x2, y2
                ));
            }
        }

        El::Curve {
            pts,
            color,
            width,
            opacity,
            tension,
            fill,
            name,
            ..
        } => {
            let d = catmull_rom(pts, *tension);
            body.push_str(&format!(
                "<path d=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{:.2}\" \
                 opacity=\"{:.4}\" stroke-linejoin=\"round\" stroke-linecap=\"round\"{}/>\n",
                d, fill, color, width, opacity, name_attr(name)
            ));
        }

        El::Connector {
            x1,
            y1,
            x2,
            y2,
            color,
            width,
            opacity,
            bend,
            name,
            ..
        } => {
            let dx = x2 - x1;
            let dy = y2 - y1;
            let (cp1x, cp1y, cp2x, cp2y) = if dx.abs() >= dy.abs() {
                let mx = x1 + dx * bend;
                (mx, *y1, mx, *y2)
            } else {
                let my = y1 + dy * bend;
                (*x1, my, *x2, my)
            };
            body.push_str(&format!(
                "<path d=\"M {:.2},{:.2} C {:.2},{:.2} {:.2},{:.2} {:.2},{:.2}\" \
                 fill=\"none\" stroke=\"{}\" stroke-width=\"{:.2}\" opacity=\"{:.4}\" \
                 stroke-linecap=\"round\"{}/>\n",
                x1, y1, cp1x, cp1y, cp2x, cp2y, x2, y2, color, width, opacity, name_attr(name)
            ));
        }

        El::Circle {
            cx,
            cy,
            r,
            fill,
            stroke,
            sw,
            opacity,
            group,
            name,
            ..
        } => {
            if group.is_empty() {
                body.push_str(&format!(
                    "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"{:.2}\" fill=\"{}\" \
                     stroke=\"{}\" stroke-width=\"{:.2}\" opacity=\"{:.4}\"{}/>\n",
                    cx, cy, r, fill, stroke, sw, opacity, name_attr(name)
                ));
            } else {
                body.push_str(&format!(
                    "<circle class=\"sp-anch\" data-sp-grp=\"{}\" data-r=\"{:.2}\" \
                     cx=\"{:.2}\" cy=\"{:.2}\" r=\"{:.2}\" fill=\"{}\" \
                     stroke=\"{}\" stroke-width=\"{:.2}\" opacity=\"{:.4}\"{} pointer-events=\"none\"/>\n",
                    group, r, cx, cy, r, fill, stroke, sw, opacity, name_attr(name)
                ));
            }
        }

        El::Ring {
            cx,
            cy,
            r_inner,
            r_outer,
            fill,
            stroke,
            sw,
            opacity,
            ..
        } => {
            let ro = r_outer.max(*r_inner);
            let ri = r_outer.min(*r_inner);
            let d = format!(
                "M {:.2},{:.2} \
                 A {:.2},{:.2} 0 1,0 {:.2},{:.2} \
                 A {:.2},{:.2} 0 1,0 {:.2},{:.2} Z \
                 M {:.2},{:.2} \
                 A {:.2},{:.2} 0 1,1 {:.2},{:.2} \
                 A {:.2},{:.2} 0 1,1 {:.2},{:.2} Z",
                cx + ro,
                cy,
                ro,
                ro,
                cx - ro,
                *cy,
                ro,
                ro,
                cx + ro,
                *cy,
                cx + ri,
                cy,
                ri,
                ri,
                cx - ri,
                *cy,
                ri,
                ri,
                cx + ri,
                *cy,
            );
            body.push_str(&format!(
                "<path d=\"{}\" fill=\"{}\" fill-rule=\"evenodd\" \
                 stroke=\"{}\" stroke-width=\"{:.2}\" opacity=\"{:.4}\"/>\n",
                d, fill, stroke, sw, opacity
            ));
        }

        El::Rect {
            x,
            y,
            w,
            h,
            fill,
            stroke,
            sw,
            rx,
            opacity,
            rotation,
            name,
            group,
            ..
        } => {
            let rot = if rotation.abs() > 0.001 {
                let cx = x + w / 2.0;
                let cy = y + h / 2.0;
                format!(" transform=\"rotate({:.2},{:.2},{:.2})\"", rotation, cx, cy)
            } else {
                String::new()
            };
            body.push_str(&format!(
                "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" \
                 fill=\"{}\" stroke=\"{}\" stroke-width=\"{:.2}\" rx=\"{:.2}\" \
                 opacity=\"{:.4}\"{}{}{}/>\n",
                x, y, w, h, fill, stroke, sw, rx, opacity, rot, name_attr(name), grp_attr(group)
            ));
        }

        El::Polygon {
            pts,
            fill,
            stroke,
            sw,
            opacity,
            name,
            ..
        } => {
            body.push_str(&format!(
                "<polygon points=\"{}\" fill=\"{}\" stroke=\"{}\" \
                 stroke-width=\"{:.2}\" opacity=\"{:.4}\"{}/>\n",
                pts_to_svg(pts),
                fill,
                stroke,
                sw,
                opacity,
                name_attr(name)
            ));
        }

        El::RawPath {
            d,
            fill,
            stroke,
            sw,
            opacity,
            name,
            ..
        } => {
            body.push_str(&format!(
                "<path d=\"{}\" fill=\"{}\" stroke=\"{}\" \
                 stroke-width=\"{:.2}\" opacity=\"{:.4}\"{}/>\n",
                d, fill, stroke, sw, opacity, name_attr(name)
            ));
        }

        El::Arrow {
            x1,
            y1,
            x2,
            y2,
            color,
            width,
            head_size,
            opacity,
            name,
            ..
        } => {
            let mid = next_mid();
            let ms = head_size * 2.5;
            let hw = head_size;
            defs.push_str(&format!(
                "<marker id=\"sp-arr-{}\" markerWidth=\"{:.1}\" markerHeight=\"{:.1}\" \
                 refX=\"{:.1}\" refY=\"{:.1}\" orient=\"auto\" markerUnits=\"strokeWidth\">\
                 <polygon points=\"0,0 {:.1},{:.1} 0,{:.1}\" fill=\"{}\"/>\
                 </marker>\n",
                mid,
                ms,
                ms,
                ms - 0.5,
                ms / 2.0,
                ms,
                ms / 2.0,
                ms,
                color
            ));
            body.push_str(&format!(
                "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" \
                 stroke=\"{}\" stroke-width=\"{:.2}\" opacity=\"{:.4}\" \
                 marker-end=\"url(#sp-arr-{})\"{}/>\n",
                x1, y1, x2, y2, color, width, opacity, mid, name_attr(name)
            ));
            let _ = hw;
        }

        El::Annotate {
            text,
            ax,
            ay,
            tx,
            ty,
            color,
            size,
            line_dash,
            lw,
            bg,
            name,
            ..
        } => {
            let da = if line_dash.is_empty() {
                String::new()
            } else {
                format!(" stroke-dasharray=\"{}\"", line_dash)
            };
            let use_h = (tx - ax).abs() >= (ty - ay).abs();
            let (lx, ly) = if use_h { (*tx, *ay) } else { (*ax, *ty) };
            body.push_str(&format!(
                "<polyline points=\"{:.2},{:.2} {:.2},{:.2} {:.2},{:.2}\" \
                 fill=\"none\" stroke=\"{}\" stroke-width=\"{:.2}\" \
                 stroke-linejoin=\"round\" opacity=\"0.65\"{}/>\n",
                ax, ay, lx, ly, tx, ty, color, lw, da
            ));
            let lines: Vec<&str> = text.split('\n').collect();
            if !bg.is_empty() && bg != "none" {
                let bg_w = lines
                    .iter()
                    .map(|l| l.len() as f64 * size * 0.58)
                    .fold(0.0_f64, f64::max)
                    + 10.0;
                let bg_h = size * 1.3 * lines.len() as f64 + 6.0;
                body.push_str(&format!(
                    "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" \
                     fill=\"{}\" rx=\"3\"/>\n",
                    tx - 5.0,
                    ty - size * 0.85,
                    bg_w,
                    bg_h,
                    bg
                ));
            }
            body.push_str(&format!(
                "<text x=\"{:.2}\" y=\"{:.2}\" font-size=\"{:.2}\" fill=\"{}\"{}>\n",
                tx, ty, size, color, name_attr(name)
            ));
            for (i, line) in lines.iter().enumerate() {
                let dy = if i == 0 { 0.0 } else { size * 1.3 };
                body.push_str(&format!(
                    "<tspan x=\"{:.2}\" dy=\"{:.2}\">{}</tspan>",
                    tx,
                    dy,
                    escape_html(line)
                ));
            }
            body.push_str("</text>\n");
        }

        El::GradDef {
            id,
            from_color,
            to_color,
            x1,
            y1,
            x2,
            y2,
        } => {
            defs.push_str(&format!(
                "<linearGradient id=\"{}\" x1=\"{:.3}\" y1=\"{:.3}\" \
                 x2=\"{:.3}\" y2=\"{:.3}\" gradientUnits=\"objectBoundingBox\">\
                 <stop offset=\"0\" stop-color=\"{}\"/>\
                 <stop offset=\"1\" stop-color=\"{}\"/>\
                 </linearGradient>\n",
                id, x1, y1, x2, y2, from_color, to_color
            ));
        }

        El::Chart { .. } | El::Image { .. } => {}
    }
}

fn render_frame(el: &El) -> Option<String> {
    let El::Chart {
        html,
        x,
        y,
        w,
        h,
        native_w,
        native_h,
        rotation,
        opacity,
        clip,
        group,
        name,
        ..
    } = el
    else {
        return None;
    };
    let nw = if native_w.is_finite() && *native_w > 0.0 {
        *native_w
    } else {
        *w
    };
    let nh = if native_h.is_finite() && *native_h > 0.0 {
        *native_h
    } else {
        *h
    };
    let mt = anchors::NativeTransform::new(*w, *h, nw, nh);
    let mut style = format!(
        "position:absolute;left:{:.2}px;top:{:.2}px;width:{:.2}px;height:{:.2}px;overflow:hidden;",
        x, y, w, h
    );
    let cv = clip_path_css(clip);
    if !cv.is_empty() {
        style.push_str(&format!("clip-path:{};-webkit-clip-path:{};", cv, cv));
    }
    if rotation.abs() > 0.001 {
        style.push_str(&format!(
            "transform:rotate({:.4}deg);transform-origin:center;",
            rotation
        ));
    }
    if *opacity < 0.9999 {
        style.push_str(&format!("opacity:{:.4};", opacity));
    }
    let escaped = html
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;");
    let grp_attr = if group.is_empty() {
        String::new()
    } else {
        format!(" data-sp-grp=\"{}\"", group)
    };
    let pos_attr = if name.is_empty() {
        String::new()
    } else {
        format!(
            " data-sp-x=\"{:.2}\" data-sp-y=\"{:.2}\" data-sp-w=\"{:.2}\" data-sp-h=\"{:.2}\"",
            x, y, w, h
        )
    };
    Some(format!(
        "<div style=\"{}\"{}{}{}>\n<iframe scrolling=\"no\" allowtransparency=\"true\" style=\"position:absolute;left:{:.2}px;top:{:.2}px;width:{:.2}px;height:{:.2}px;border:none;background:transparent;display:block;overflow:hidden;transform:scale({:.8});transform-origin:top left;\" srcdoc=\"{}\"></iframe>\n</div>\n",
        style, grp_attr, name_attr(name), pos_attr, mt.x, mt.y, nw, nh, mt.scale, escaped
    ))
}

fn render_image(el: &El) -> Option<String> {
    let El::Image {
        src,
        x,
        y,
        w,
        h,
        rotation,
        opacity,
        clip,
        group,
        name,
    } = el
    else {
        return None;
    };
    let mut style = format!(
        "position:absolute;left:{:.2}px;top:{:.2}px;width:{:.2}px;height:{:.2}px;overflow:hidden;",
        x, y, w, h
    );
    let cv = clip_path_css(clip);
    if !cv.is_empty() {
        style.push_str(&format!("clip-path:{};-webkit-clip-path:{};", cv, cv));
    }
    if rotation.abs() > 0.001 {
        style.push_str(&format!(
            "transform:rotate({:.4}deg);transform-origin:center;",
            rotation
        ));
    }
    if *opacity < 0.9999 {
        style.push_str(&format!("opacity:{:.4};", opacity));
    }
    let grp_attr = if group.is_empty() {
        String::new()
    } else {
        format!(" data-sp-grp=\"{}\"", group)
    };
    let pos_attr = if name.is_empty() {
        String::new()
    } else {
        format!(
            " data-sp-x=\"{:.2}\" data-sp-y=\"{:.2}\" data-sp-w=\"{:.2}\" data-sp-h=\"{:.2}\"",
            x, y, w, h
        )
    };
    Some(format!(
        "<div style=\"{}\"{}{}{}>\n<img src=\"{}\" style=\"width:100%;height:100%;display:block;object-fit:fill;\" draggable=\"false\"/>\n</div>\n",
        style, grp_attr, name_attr(name), pos_attr, src
    ))
}

fn el_layer(el: &El) -> Option<&Layer> {
    match el {
        El::Text { layer, .. }
        | El::Line { layer, .. }
        | El::Curve { layer, .. }
        | El::Connector { layer, .. }
        | El::Circle { layer, .. }
        | El::Ring { layer, .. }
        | El::Rect { layer, .. }
        | El::Polygon { layer, .. }
        | El::RawPath { layer, .. }
        | El::Arrow { layer, .. }
        | El::Annotate { layer, .. } => Some(layer),
        _ => None,
    }
}

fn translate_element(el: &mut El, dx: f64, dy: f64) {
    match el {
        El::Chart { x, y, .. } => {
            *x += dx;
            *y += dy;
        }
        El::Image { x, y, .. } => {
            *x += dx;
            *y += dy;
        }
        El::Text { x, y, .. } => {
            *x += dx;
            *y += dy;
        }
        El::Line { x1, y1, x2, y2, .. } => {
            *x1 += dx;
            *y1 += dy;
            *x2 += dx;
            *y2 += dy;
        }
        El::Curve { pts, .. } | El::Polygon { pts, .. } => {
            for p in pts.iter_mut() {
                p.0 += dx;
                p.1 += dy;
            }
        }
        El::Connector { x1, y1, x2, y2, .. } => {
            *x1 += dx;
            *y1 += dy;
            *x2 += dx;
            *y2 += dy;
        }
        El::Circle { cx, cy, .. } => {
            *cx += dx;
            *cy += dy;
        }
        El::Ring { cx, cy, .. } => {
            *cx += dx;
            *cy += dy;
        }
        El::Rect { x, y, .. } => {
            *x += dx;
            *y += dy;
        }
        El::Arrow { x1, y1, x2, y2, .. } => {
            *x1 += dx;
            *y1 += dy;
            *x2 += dx;
            *y2 += dy;
        }
        El::Annotate { ax, ay, tx, ty, .. } => {
            *ax += dx;
            *ay += dy;
            *tx += dx;
            *ty += dy;
        }
        El::RawPath { .. } | El::GradDef { .. } => {}
    }
}

fn resize_element(el: &mut El, dw: f64, dh: f64) -> bool {
    match el {
        El::Chart { w, h, .. } | El::Image { w, h, .. } | El::Rect { w, h, .. } => {
            *w = (*w + dw).max(4.0);
            *h = (*h + dh).max(4.0);
            true
        }
        _ => false,
    }
}

const CANVAS_DEV_JS: &str = concat!(
    "(function(){",
    "if(window.__spcvdev__)return;window.__spcvdev__=1;",
    "var root=document.getElementById('sp-canvas-root');if(!root)return;",
    "var deltas={};",
    "function getTr(el){var m=(el.getAttribute('transform')||'').match(/translate\\(\\s*([^,\\s)]+)[,\\s]+([^,\\s)]+)\\s*\\)/);return m?{x:parseFloat(m[1])||0,y:parseFloat(m[2])||0}:{x:0,y:0};}",
    "function setTr(el,x,y){var rest=(el.getAttribute('transform')||'').replace(/translate\\([^)]*\\)/,'').trim();el.setAttribute('transform','translate('+x+','+y+')'+(rest?' '+rest:''));}",
    "var panel=document.createElement('div');",
    "panel.style.cssText='position:fixed;bottom:16px;right:16px;z-index:99999;background:linear-gradient(180deg,#171724,#0d0d14);border:1px solid rgba(148,163,184,.16);border-radius:16px;font-family:-apple-system,\"Segoe UI\",system-ui,sans-serif;font-size:12px;color:#f8fafc;width:320px;box-shadow:0 24px 60px -16px rgba(0,0,0,.7);user-select:none;overflow:hidden';",
    "var accent=document.createElement('div');accent.style.cssText='height:3px;background:linear-gradient(90deg,#3987e5,#9085e9)';panel.appendChild(accent);",
    "var hdr=document.createElement('div');",
    "hdr.style.cssText='display:flex;align-items:center;justify-content:space-between;padding:12px 16px;border-bottom:1px solid rgba(148,163,184,.12);cursor:move';",
    "var hdrLeft=document.createElement('div');hdrLeft.style.cssText='display:flex;align-items:center;gap:8px';",
    "var dot=document.createElement('span');dot.style.cssText='width:7px;height:7px;border-radius:50%;background:#3987e5;box-shadow:0 0 8px #3987e5aa;flex-shrink:0';",
    "var hdrTtl=document.createElement('span');hdrTtl.style.cssText='color:#f8fafc;font-weight:700;letter-spacing:.3px;font-size:12.5px';hdrTtl.textContent='Canvas dev mode';",
    "hdrLeft.appendChild(dot);hdrLeft.appendChild(hdrTtl);hdr.appendChild(hdrLeft);",
    "var btnClose=document.createElement('button');btnClose.textContent='\\u2715';btnClose.style.cssText='background:rgba(148,163,184,.1);border:none;color:#c3c2b7;cursor:pointer;font-size:11px;width:22px;height:22px;border-radius:6px;line-height:1;font-family:inherit';hdr.appendChild(btnClose);",
    "var info=document.createElement('div');info.style.cssText='padding:12px 16px;color:#898781;font-size:11px;line-height:1.6;white-space:pre-wrap;border-bottom:1px solid rgba(148,163,184,.10)';info.textContent='Drag any named element to move it.\\nDrag the corner handle to resize charts/images.';",
    "var statusEl=document.createElement('div');statusEl.style.cssText='padding:8px 16px;color:#a5b4fc;font-size:10.5px;line-height:1.5;border-bottom:1px solid rgba(148,163,184,.10);min-height:14px';statusEl.textContent='';",
    "var listEl=document.createElement('div');listEl.style.cssText='padding:10px 16px;max-height:110px;overflow-y:auto;font-size:11px;color:#c3c2b7;line-height:1.7;border-bottom:1px solid rgba(148,163,184,.10)';",
    "var codeSect=document.createElement('div');codeSect.style.cssText='padding:12px 16px 16px';",
    "var codeEl=document.createElement('pre');codeEl.style.cssText='background:rgba(0,0,0,.35);border:1px solid rgba(148,163,184,.10);border-radius:8px;padding:10px;margin:0 0 10px 0;color:#a5b4fc;font-family:ui-monospace,monospace;font-size:10.5px;overflow-x:auto;white-space:pre;line-height:1.6;max-height:110px;overflow-y:auto';codeEl.textContent='# drag elements to generate code';",
    "var btnRow=document.createElement('div');btnRow.style.cssText='display:flex;gap:8px';",
    "var btnCopy=document.createElement('button');btnCopy.textContent='Copy Python';btnCopy.style.cssText='flex:1;padding:8px;background:linear-gradient(135deg,#3987e5,#2a78d6);color:#fff;border:none;border-radius:8px;cursor:pointer;font-size:11.5px;font-weight:600;font-family:inherit;box-shadow:0 6px 16px -6px rgba(57,135,229,.6)';",
    "var btnDl=document.createElement('button');btnDl.textContent='Download JSON';btnDl.style.cssText='flex:1;padding:8px;background:rgba(148,163,184,.12);color:#f8fafc;border:1px solid rgba(148,163,184,.16);border-radius:8px;cursor:pointer;font-size:11.5px;font-weight:600;font-family:inherit';",
    "var btnReset=document.createElement('button');btnReset.textContent='Reset';btnReset.style.cssText='padding:8px 10px;background:rgba(214,81,81,.12);color:#f3a5a5;border:1px solid rgba(214,81,81,.25);border-radius:8px;cursor:pointer;font-size:11.5px;font-weight:600;font-family:inherit';",
    "btnRow.appendChild(btnCopy);btnRow.appendChild(btnDl);btnRow.appendChild(btnReset);",
    "codeSect.appendChild(codeEl);codeSect.appendChild(btnRow);",
    "panel.appendChild(hdr);panel.appendChild(info);panel.appendChild(statusEl);panel.appendChild(listEl);panel.appendChild(codeSect);",
    "document.body.appendChild(panel);",
    "function genCode(){var lines=[];Object.keys(deltas).forEach(function(name){var d=deltas[name];if(d.dx||d.dy)lines.push('cv.nudge(\"'+name+'\", '+d.dx.toFixed(1)+', '+d.dy.toFixed(1)+')');if(d.dw||d.dh)lines.push('cv.resize(\"'+name+'\", '+d.dw.toFixed(1)+', '+d.dh.toFixed(1)+')');});return lines.length?lines.join('\\n'):'# drag elements to generate code';}",
    "function renderList(){codeEl.textContent=genCode();while(listEl.firstChild)listEl.removeChild(listEl.firstChild);var names=Object.keys(deltas);if(!names.length){var ph=document.createElement('div');ph.style.color='#52514e';ph.textContent='no changes yet';listEl.appendChild(ph);return;}names.forEach(function(name){var d=deltas[name];var row=document.createElement('div');row.innerHTML='<b style=\"color:#f8fafc\">'+name+'</b>  dx='+d.dx.toFixed(0)+' dy='+d.dy.toFixed(0)+(d.dw||d.dh?'  dw='+d.dw.toFixed(0)+' dh='+d.dh.toFixed(0):'');listEl.appendChild(row);});}",
    "function accum(name,dx,dy,dw,dh){var d=deltas[name]||{dx:0,dy:0,dw:0,dh:0};d.dx+=dx;d.dy+=dy;d.dw+=dw;d.dh+=dh;deltas[name]=d;renderList();}",
    "renderList();",
    "var pdDrag=false,pdDx=0,pdDy=0;",
    "hdr.addEventListener('mousedown',function(e){if(e.target===btnClose)return;pdDrag=true;var r=panel.getBoundingClientRect();pdDx=e.clientX-r.left;pdDy=e.clientY-r.top;e.preventDefault();});",
    "document.addEventListener('mousemove',function(e){if(!pdDrag)return;panel.style.left=(e.clientX-pdDx)+'px';panel.style.top=(e.clientY-pdDy)+'px';panel.style.bottom='auto';panel.style.right='auto';});",
    "document.addEventListener('mouseup',function(){pdDrag=false;});",
    "btnClose.addEventListener('click',function(){panel.style.display='none';});",
    "btnCopy.addEventListener('click',function(){var code=genCode();if(navigator.clipboard)navigator.clipboard.writeText(code);btnCopy.textContent='Copied!';setTimeout(function(){btnCopy.textContent='Copy Python';},1200);});",
    "btnDl.addEventListener('click',function(){var blob=new Blob([JSON.stringify(deltas,null,2)],{type:'application/json'});var a=document.createElement('a');a.href=URL.createObjectURL(blob);a.download='canvas_layout.json';document.body.appendChild(a);a.click();document.body.removeChild(a);});",
    "btnReset.addEventListener('click',function(){location.reload();});",
    "root.querySelectorAll('[data-sp-name]').forEach(function(el){",
    "var nm=el.getAttribute('data-sp-name');var g=el.getAttribute('data-sp-grp');",
    "el.addEventListener('mouseenter',function(){statusEl.textContent=g?(nm+'  ·  linked group: '+g):nm;});",
    "el.addEventListener('mouseleave',function(){statusEl.textContent='';});",
    "});",
    "function makeMovable(el,name){",
    "el.style.cursor='move';",
    "var isDiv=el.tagName==='DIV';",
    "if(isDiv){var ifr=el.querySelector('iframe');if(ifr)ifr.style.pointerEvents='none';}",
    "var drag=false,sCX=0,sCY=0,sX=0,sY=0;",
    "el.addEventListener('mousedown',function(e){",
    "if(e.button!==0)return;",
    "drag=true;sCX=e.clientX;sCY=e.clientY;",
    "if(isDiv){sX=parseFloat(el.style.left)||0;sY=parseFloat(el.style.top)||0;}else{var t=getTr(el);sX=t.x;sY=t.y;}",
    "function onMove(me){",
    "if(!drag)return;",
    "var dx=me.clientX-sCX,dy=me.clientY-sCY;",
    "if(isDiv){el.style.left=(sX+dx)+'px';el.style.top=(sY+dy)+'px';}else{setTr(el,sX+dx,sY+dy);}",
    "el.__spLastDx=dx;el.__spLastDy=dy;",
    "}",
    "function onUp(){",
    "if(!drag)return;drag=false;",
    "document.removeEventListener('mousemove',onMove);document.removeEventListener('mouseup',onUp);",
    "var dx=el.__spLastDx||0,dy=el.__spLastDy||0;",
    "if(dx||dy)accum(name,dx,dy,0,0);",
    "el.__spLastDx=0;el.__spLastDy=0;",
    "}",
    "document.addEventListener('mousemove',onMove);document.addEventListener('mouseup',onUp);",
    "e.preventDefault();e.stopPropagation();",
    "});",
    "}",
    "function makeResizable(el,name){",
    "var handle=document.createElement('div');",
    "handle.style.cssText='position:absolute;right:-5px;bottom:-5px;width:13px;height:13px;background:linear-gradient(135deg,#3987e5,#9085e9);border:2px solid #0a0a0f;border-radius:4px;cursor:nwse-resize;z-index:10;box-shadow:0 2px 8px rgba(0,0,0,.5)';",
    "el.appendChild(handle);",
    "var drag=false,sCX=0,sCY=0,sW=0,sH=0;",
    "handle.addEventListener('mousedown',function(e){",
    "drag=true;sCX=e.clientX;sCY=e.clientY;",
    "sW=parseFloat(el.style.width)||el.getBoundingClientRect().width;",
    "sH=parseFloat(el.style.height)||el.getBoundingClientRect().height;",
    "function onMove(me){",
    "if(!drag)return;",
    "var dw=me.clientX-sCX,dh=me.clientY-sCY;",
    "var nw=Math.max(8,sW+dw),nh=Math.max(8,sH+dh);",
    "el.style.width=nw+'px';el.style.height=nh+'px';",
    "handle.__spLastDw=nw-sW;handle.__spLastDh=nh-sH;",
    "}",
    "function onUp(){",
    "if(!drag)return;drag=false;",
    "document.removeEventListener('mousemove',onMove);document.removeEventListener('mouseup',onUp);",
    "var dw=handle.__spLastDw||0,dh=handle.__spLastDh||0;",
    "if(dw||dh)accum(name,0,0,dw,dh);",
    "}",
    "document.addEventListener('mousemove',onMove);document.addEventListener('mouseup',onUp);",
    "e.preventDefault();e.stopPropagation();",
    "});",
    "}",
    "root.querySelectorAll('[data-sp-name]').forEach(function(el){",
    "var name=el.getAttribute('data-sp-name');if(!name)return;",
    "makeMovable(el,name);",
    "if(el.hasAttribute('data-sp-w')&&el.hasAttribute('data-sp-h')){makeResizable(el,name);}",
    "});",
    "})();"
);

#[pyclass]
#[derive(Clone)]
pub struct Canvas {
    width: u32,
    height: u32,
    background: String,
    elements: Vec<El>,
    placed: Vec<PlacedInfo>,
    pins: HashMap<String, (f64, f64)>,
    names: HashMap<String, usize>,
    groups: HashMap<String, Vec<String>>,
    custom_css: Vec<(String, String)>,
    custom_js: Vec<String>,
    slots: HashMap<String, (f64, f64, f64, f64)>,
}

#[derive(Serialize, Deserialize)]
struct CanvasState {
    version: u32,
    width: u32,
    height: u32,
    background: String,
    elements: Vec<El>,
    pins: HashMap<String, (f64, f64)>,
    names: HashMap<String, usize>,
    groups: HashMap<String, Vec<String>>,
    custom_css: Vec<(String, String)>,
    custom_js: Vec<String>,
    #[serde(default)]
    slots: HashMap<String, (f64, f64, f64, f64)>,
}

impl Canvas {
    fn pin_key(chart_ref: usize, name: &str) -> String {
        format!("{}::{}", chart_ref, name)
    }

    fn register_name(&mut self, name: &str, element_idx: usize) {
        if !name.is_empty() {
            self.names.insert(name.to_string(), element_idx);
        }
    }

    fn apply_delta_by_name(&mut self, name: &str, dx: f64, dy: f64, dw: f64, dh: f64) -> bool {
        let Some(&element_idx) = self.names.get(name) else {
            return false;
        };
        let chart_ref = match self.elements.get(element_idx) {
            Some(El::Chart { ref_id, .. }) => Some(*ref_id),
            _ => None,
        };
        let mut changed = false;
        if let Some(el) = self.elements.get_mut(element_idx) {
            if dx != 0.0 || dy != 0.0 {
                translate_element(el, dx, dy);
                changed = true;
            }
            if dw != 0.0 || dh != 0.0 {
                changed = resize_element(el, dw, dh) || changed;
            }
        }
        if !changed {
            return false;
        }
        let Some(chart_ref) = chart_ref else {
            return true;
        };
        if dx != 0.0 || dy != 0.0 {
            if let Some(info) = self.placed.get_mut(chart_ref) {
                info.x += dx;
                info.y += dy;
            }
            let prefix = format!("{}::", chart_ref);
            for (key, point) in self.pins.iter_mut() {
                if key.starts_with(&prefix) {
                    point.0 += dx;
                    point.1 += dy;
                }
            }
        }
        if dw != 0.0 || dh != 0.0 {
            if let Some(info) = self.placed.get_mut(chart_ref) {
                info.w = (info.w + dw).max(4.0);
                info.h = (info.h + dh).max(4.0);
            }
        }
        true
    }

    fn rebuild_placed(&mut self) {
        self.placed.clear();
        for (element_idx, el) in self.elements.iter().enumerate() {
            if let El::Chart {
                html,
                x,
                y,
                w,
                h,
                native_w,
                native_h,
                rotation,
                ..
            } = el
            {
                let plot = anchors::chart_frame(html, *w, *h).plot;
                self.placed.push(PlacedInfo {
                    x: *x,
                    y: *y,
                    w: *w,
                    h: *h,
                    native_w: *native_w,
                    native_h: *native_h,
                    rotation: *rotation,
                    element_idx,
                    plot,
                });
            }
        }
    }

    fn to_state(&self) -> CanvasState {
        CanvasState {
            version: 1,
            width: self.width,
            height: self.height,
            background: self.background.clone(),
            elements: self.elements.clone(),
            pins: self.pins.clone(),
            names: self.names.clone(),
            groups: self.groups.clone(),
            custom_css: self.custom_css.clone(),
            custom_js: self.custom_js.clone(),
            slots: self.slots.clone(),
        }
    }

    fn from_state(state: CanvasState) -> Canvas {
        let mut canvas = Canvas {
            width: state.width,
            height: state.height,
            background: state.background,
            elements: state.elements,
            placed: Vec::new(),
            pins: state.pins,
            names: state.names,
            groups: state.groups,
            custom_css: state.custom_css,
            custom_js: state.custom_js,
            slots: state.slots,
        };
        canvas.rebuild_placed();
        canvas
    }

    fn clear_pins_for(&mut self, chart_ref: usize) {
        let prefix = format!("{}::", chart_ref);
        self.pins.retain(|key, _| !key.starts_with(&prefix));
    }

    fn place_internal(
        &mut self,
        chart: &crate::Chart,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        rotation: f64,
        opacity: f64,
        clip: &str,
        group: &str,
        name: &str,
    ) -> usize {
        if !name.is_empty() {
            if let Some(&existing_idx) = self.names.get(name) {
                if let Some(El::Chart { ref_id, .. }) = self.elements.get(existing_idx) {
                    let chart_ref = *ref_id;
                    let frame = anchors::chart_frame(&chart.html, w, h);
                    self.clear_pins_for(chart_ref);
                    if let Some(info) = self.placed.get_mut(chart_ref) {
                        info.x = x;
                        info.y = y;
                        info.w = w;
                        info.h = h;
                        info.native_w = frame.native_w;
                        info.native_h = frame.native_h;
                        info.rotation = rotation;
                        info.plot = frame.plot;
                    }
                    if let Some(El::Chart {
                        html: el_html,
                        x: el_x,
                        y: el_y,
                        w: el_w,
                        h: el_h,
                        native_w: el_nw,
                        native_h: el_nh,
                        rotation: el_rot,
                        opacity: el_op,
                        clip: el_clip,
                        group: el_grp,
                        ..
                    }) = self.elements.get_mut(existing_idx)
                    {
                        *el_html = chart.html.clone();
                        *el_x = x;
                        *el_y = y;
                        *el_w = w;
                        *el_h = h;
                        *el_nw = frame.native_w;
                        *el_nh = frame.native_h;
                        *el_rot = rotation;
                        *el_op = opacity;
                        *el_clip = clip.to_string();
                        *el_grp = group.to_string();
                    }
                    return chart_ref;
                }
            }
        }
        let idx = self.placed.len();
        let frame = anchors::chart_frame(&chart.html, w, h);
        let element_idx = self.elements.len();
        self.placed.push(PlacedInfo {
            x,
            y,
            w,
            h,
            native_w: frame.native_w,
            native_h: frame.native_h,
            rotation,
            element_idx,
            plot: frame.plot,
        });
        self.register_name(name, element_idx);
        self.elements.push(El::Chart {
            html: chart.html.clone(),
            x,
            y,
            w,
            h,
            native_w: frame.native_w,
            native_h: frame.native_h,
            rotation,
            opacity,
            clip: clip.to_string(),
            group: group.to_string(),
            name: name.to_string(),
            ref_id: idx,
        });
        idx
    }

    fn map_native_point(info: &PlacedInfo, local_x: f64, local_y: f64) -> (f64, f64) {
        let mt = anchors::NativeTransform::new(info.w, info.h, info.native_w, info.native_h);
        let (lx, ly) = mt.map(local_x, local_y);
        let mut ax = info.x + lx;
        let mut ay = info.y + ly;
        if info.rotation.abs() > 0.001 {
            let cx = info.x + info.w * 0.5;
            let cy = info.y + info.h * 0.5;
            let a = info.rotation.to_radians();
            let s = a.sin();
            let c = a.cos();
            let dx = ax - cx;
            let dy = ay - cy;
            ax = cx + dx * c - dy * s;
            ay = cy + dx * s + dy * c;
        }
        (ax, ay)
    }

    fn register_native_pin(&mut self, chart_ref: usize, name: &str, local_x: f64, local_y: f64) {
        if let Some(info) = self.placed.get(chart_ref) {
            let point = Self::map_native_point(info, local_x, local_y);
            self.pins.insert(Self::pin_key(chart_ref, name), point);
        }
    }

    fn chart_html(&self, chart_ref: usize) -> Option<&str> {
        let info = self.placed.get(chart_ref)?;
        match self.elements.get(info.element_idx)? {
            El::Chart { html, .. } => Some(html.as_str()),
            _ => None,
        }
    }

    fn update_chart_space(
        &mut self,
        chart_ref: usize,
        native_w: f64,
        native_h: f64,
        plot: Option<anchors::PlotFrame>,
    ) {
        if let Some(info) = self.placed.get_mut(chart_ref) {
            if native_w.is_finite() && native_w > 0.0 {
                info.native_w = native_w;
            }
            if native_h.is_finite() && native_h > 0.0 {
                info.native_h = native_h;
            }
            if info.plot.is_none() {
                info.plot = plot;
            }
            if let Some(El::Chart {
                native_w: el_w,
                native_h: el_h,
                ..
            }) = self.elements.get_mut(info.element_idx)
            {
                *el_w = info.native_w;
                *el_h = info.native_h;
            }
        }
    }

    fn plot_or(&self, chart_ref: usize, fallback: anchors::PlotFrame) -> anchors::PlotFrame {
        self.placed
            .get(chart_ref)
            .and_then(|info| info.plot)
            .unwrap_or(fallback)
    }
}

#[pymethods]
impl Canvas {
    #[new]
    #[pyo3(signature = (width, height, bg = "#0a0a0f"))]
    pub fn new(width: u32, height: u32, bg: &str) -> Self {
        Canvas {
            width,
            height,
            background: bg.to_string(),
            elements: Vec::new(),
            placed: Vec::new(),
            pins: HashMap::new(),
            names: HashMap::new(),
            groups: HashMap::new(),
            custom_css: Vec::new(),
            custom_js: Vec::new(),
            slots: HashMap::new(),
        }
    }

    #[pyo3(signature = (chart, x, y, w, h, rotation = 0.0, opacity = 1.0, clip = "", group = "", name = ""))]
    pub fn place(
        &mut self,
        chart: &crate::Chart,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        rotation: f64,
        opacity: f64,
        clip: &str,
        group: &str,
        name: &str,
    ) -> usize {
        self.place_internal(chart, x, y, w, h, rotation, opacity, clip, group, name)
    }

    pub fn slot(&mut self, name: &str, x: f64, y: f64, w: f64, h: f64) {
        self.slots.insert(name.to_string(), (x, y, w, h));
    }

    pub fn slot_rect(&self, name: &str) -> Option<(f64, f64, f64, f64)> {
        self.slots.get(name).copied()
    }

    #[pyo3(signature = (slot_name, chart, rotation = 0.0, opacity = 1.0, clip = "", group = "", name = ""))]
    pub fn fill(
        &mut self,
        slot_name: &str,
        chart: &crate::Chart,
        rotation: f64,
        opacity: f64,
        clip: &str,
        group: &str,
        name: &str,
    ) -> PyResult<usize> {
        let (x, y, w, h) = self.slots.get(slot_name).copied().ok_or_else(|| {
            pyo3::exceptions::PyKeyError::new_err(format!("no slot named '{}'", slot_name))
        })?;
        let resolved_name = if name.is_empty() { slot_name } else { name };
        Ok(self.place_internal(chart, x, y, w, h, rotation, opacity, clip, group, resolved_name))
    }

    #[pyo3(signature = (x, y, w, h, rows, cols, gap_x = 0.0, gap_y = 0.0))]
    pub fn grid(
        &mut self,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        rows: usize,
        cols: usize,
        gap_x: f64,
        gap_y: f64,
    ) -> Vec<String> {
        let rows = rows.max(1);
        let cols = cols.max(1);
        let cell_w = (w - gap_x * (cols as f64 - 1.0)) / cols as f64;
        let cell_h = (h - gap_y * (rows as f64 - 1.0)) / rows as f64;
        let mut names = Vec::with_capacity(rows * cols);
        for r in 0..rows {
            for c in 0..cols {
                let cx = x + c as f64 * (cell_w + gap_x);
                let cy = y + r as f64 * (cell_h + gap_y);
                let name = format!("cell_{}_{}", r, c);
                self.slots.insert(name.clone(), (cx, cy, cell_w, cell_h));
                names.push(name);
            }
        }
        names
    }

    pub fn refill(&mut self, name: &str, chart: &crate::Chart) -> PyResult<bool> {
        let Some(&element_idx) = self.names.get(name) else {
            return Ok(false);
        };
        let (chart_ref, w, h) = match self.elements.get(element_idx) {
            Some(El::Chart { ref_id, w, h, .. }) => (*ref_id, *w, *h),
            _ => return Ok(false),
        };
        let frame = anchors::chart_frame(&chart.html, w, h);
        if let Some(El::Chart {
            html,
            native_w,
            native_h,
            ..
        }) = self.elements.get_mut(element_idx)
        {
            *html = chart.html.clone();
            *native_w = frame.native_w;
            *native_h = frame.native_h;
        }
        if let Some(info) = self.placed.get_mut(chart_ref) {
            info.native_w = frame.native_w;
            info.native_h = frame.native_h;
            info.plot = frame.plot;
        }
        self.clear_pins_for(chart_ref);
        Ok(true)
    }

    pub fn derive(&self) -> Canvas {
        self.clone()
    }

    pub fn template(&self) -> Canvas {
        let mut kept_elements: Vec<El> = Vec::new();
        let mut old_to_new: HashMap<usize, usize> = HashMap::new();
        for (old_idx, el) in self.elements.iter().enumerate() {
            if matches!(el, El::Chart { .. } | El::Image { .. }) {
                continue;
            }
            let new_idx = kept_elements.len();
            old_to_new.insert(old_idx, new_idx);
            kept_elements.push(el.clone());
        }
        let names: HashMap<String, usize> = self
            .names
            .iter()
            .filter_map(|(k, v)| old_to_new.get(v).map(|nv| (k.clone(), *nv)))
            .collect();
        let groups: HashMap<String, Vec<String>> = self
            .groups
            .iter()
            .map(|(k, members)| {
                (
                    k.clone(),
                    members
                        .iter()
                        .filter(|m| names.contains_key(*m))
                        .cloned()
                        .collect::<Vec<String>>(),
                )
            })
            .filter(|(_, members)| !members.is_empty())
            .collect();
        Canvas {
            width: self.width,
            height: self.height,
            background: self.background.clone(),
            elements: kept_elements,
            placed: Vec::new(),
            pins: HashMap::new(),
            names,
            groups,
            custom_css: self.custom_css.clone(),
            custom_js: self.custom_js.clone(),
            slots: self.slots.clone(),
        }
    }

    pub fn link(&mut self, group_name: &str, member_names: Vec<String>) -> usize {
        let mut linked = 0;
        for member in &member_names {
            let Some(&idx) = self.names.get(member) else {
                continue;
            };
            let Some(el) = self.elements.get_mut(idx) else {
                continue;
            };
            let target = match el {
                El::Chart { group, .. } => Some(group),
                El::Line { group, .. } => Some(group),
                El::Circle { group, .. } => Some(group),
                El::Text { group, .. } => Some(group),
                El::Rect { group, .. } => Some(group),
                _ => None,
            };
            if let Some(g) = target {
                *g = group_name.to_string();
                linked += 1;
            }
        }
        linked
    }

    #[pyo3(signature = (src, x, y, w, h, rotation = 0.0, opacity = 1.0, clip = "", group = "", name = ""))]
    pub fn image(
        &mut self,
        src: &str,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        rotation: f64,
        opacity: f64,
        clip: &str,
        group: &str,
        name: &str,
    ) -> PyResult<usize> {
        let resolved = if src.starts_with("data:")
            || src.starts_with("http://")
            || src.starts_with("https://")
        {
            src.to_string()
        } else {
            let bytes = fs::read(src).map_err(|e| {
                pyo3::exceptions::PyIOError::new_err(format!(
                    "cannot read image '{}': {}",
                    src, e
                ))
            })?;
            let mime = guess_mime(src);
            format!(
                "data:{};base64,{}",
                mime,
                base64::engine::general_purpose::STANDARD.encode(bytes)
            )
        };
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Image {
            src: resolved,
            x,
            y,
            w,
            h,
            rotation,
            opacity,
            clip: clip.to_string(),
            group: group.to_string(),
            name: name.to_string(),
        });
        Ok(element_idx)
    }

    #[pyo3(signature = (group_name, members))]
    pub fn group(&mut self, group_name: &str, members: Vec<String>) {
        self.groups.insert(group_name.to_string(), members);
    }

    #[pyo3(signature = (group_name, dx, dy))]
    pub fn move_group(&mut self, group_name: &str, dx: f64, dy: f64) -> usize {
        let Some(members) = self.groups.get(group_name).cloned() else {
            return 0;
        };
        let mut moved = 0;
        for member in &members {
            if self.apply_delta_by_name(member, dx, dy, 0.0, 0.0) {
                moved += 1;
            }
        }
        moved
    }

    pub fn group_members(&self, group_name: &str) -> Vec<String> {
        self.groups.get(group_name).cloned().unwrap_or_default()
    }

    pub fn nudge(&mut self, name: &str, dx: f64, dy: f64) -> bool {
        self.apply_delta_by_name(name, dx, dy, 0.0, 0.0)
    }

    pub fn resize(&mut self, name: &str, dw: f64, dh: f64) -> bool {
        self.apply_delta_by_name(name, 0.0, 0.0, dw, dh)
    }

    pub fn apply_deltas_json(&mut self, json_str: &str) -> PyResult<usize> {
        #[derive(Deserialize)]
        struct Delta {
            #[serde(default)]
            dx: f64,
            #[serde(default)]
            dy: f64,
            #[serde(default)]
            dw: f64,
            #[serde(default)]
            dh: f64,
        }
        let parsed: HashMap<String, Delta> = serde_json::from_str(json_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let mut applied = 0;
        for (name, d) in parsed {
            if self.apply_delta_by_name(&name, d.dx, d.dy, d.dw, d.dh) {
                applied += 1;
            }
        }
        Ok(applied)
    }

    pub fn style(&mut self, name: &str, css: &str) {
        self.custom_css.push((name.to_string(), css.to_string()));
    }

    pub fn script(&mut self, js: &str) {
        self.custom_js.push(js.to_string());
    }

    pub fn element_ref(&self, name: &str) -> Option<usize> {
        self.names.get(name).copied()
    }

    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string_pretty(&self.to_state())
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    pub fn save(&self, path: &str) -> PyResult<()> {
        if let Some(parent) = std::path::Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).map_err(|e| {
                    pyo3::exceptions::PyIOError::new_err(format!(
                        "cannot create directory for '{}': {}",
                        path, e
                    ))
                })?;
            }
        }
        let json = self.to_json()?;
        fs::write(path, json).map_err(|e| {
            pyo3::exceptions::PyIOError::new_err(format!("cannot write '{}': {}", path, e))
        })
    }

    pub fn set_group(&mut self, chart_ref: usize, group: &str) {
        if let Some(info) = self.placed.get(chart_ref) {
            let idx = info.element_idx;
            if let Some(El::Chart { group: g, .. }) = self.elements.get_mut(idx) {
                *g = group.to_string();
            }
        }
    }

    pub fn pin(&mut self, chart_ref: usize, name: &str, local_x: f64, local_y: f64) {
        self.register_native_pin(chart_ref, name, local_x, local_y);
    }

    pub fn pin_frac(&mut self, chart_ref: usize, name: &str, fx: f64, fy: f64) {
        if let Some(info) = self.placed.get(chart_ref) {
            let local_x = info.native_w * fx;
            let local_y = info.native_h * fy;
            let point = Self::map_native_point(info, local_x, local_y);
            self.pins.insert(Self::pin_key(chart_ref, name), point);
        }
    }

    pub fn pin_xy(&self, chart_ref: usize, name: &str) -> Option<(f64, f64)> {
        self.pins.get(&Self::pin_key(chart_ref, name)).copied()
    }

    #[pyo3(signature = (chart_ref, values, chart_w, chart_h,
                        has_groups = false, has_ylabel = false, has_xlabel = false))]
    pub fn attach_bar(
        &mut self,
        chart_ref: usize,
        values: Vec<f64>,
        chart_w: f64,
        chart_h: f64,
        has_groups: bool,
        has_ylabel: bool,
        has_xlabel: bool,
    ) {
        let n = values.len();
        if n == 0 {
            return;
        }
        let rects = self
            .chart_html(chart_ref)
            .map(anchors::bar_rects)
            .unwrap_or_default();
        let fallback = anchors::bar_fallback(chart_w, chart_h, has_groups, has_ylabel, has_xlabel);
        self.update_chart_space(chart_ref, chart_w, chart_h, Some(fallback));
        if !rects.is_empty() {
            for rect in rects {
                let cx = rect.x + rect.w * 0.5;
                let cy = rect.y + rect.h * 0.5;
                self.register_native_pin(chart_ref, &format!("bar:{}:top", rect.idx), cx, rect.y);
                self.register_native_pin(chart_ref, &format!("bar:{}:center", rect.idx), cx, cy);
                self.register_native_pin(
                    chart_ref,
                    &format!("bar:{}:bottom", rect.idx),
                    cx,
                    rect.y + rect.h,
                );
                self.register_native_pin(chart_ref, &format!("bar:{}:left", rect.idx), rect.x, cy);
                self.register_native_pin(
                    chart_ref,
                    &format!("bar:{}:right", rect.idx),
                    rect.x + rect.w,
                    cy,
                );
            }
            return;
        }
        let max_val = values
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max)
            .max(1.0);
        let plot = self.plot_or(chart_ref, fallback);
        let bar_w = plot.width / n as f64;
        for (i, &val) in values.iter().enumerate() {
            let bh = (val / max_val).max(0.0) * plot.height;
            let cx = plot.left + (i as f64 + 0.5) * bar_w;
            let yt = plot.top + plot.height - bh;
            let yc = plot.top + plot.height - bh * 0.5;
            let yb = plot.top + plot.height;
            self.register_native_pin(chart_ref, &format!("bar:{}:top", i), cx, yt);
            self.register_native_pin(chart_ref, &format!("bar:{}:center", i), cx, yc);
            self.register_native_pin(chart_ref, &format!("bar:{}:bottom", i), cx, yb);
        }
    }

    #[pyo3(signature = (chart_ref, x_vals, y_vals, labels, chart_w, chart_h, has_groups = false))]
    pub fn attach_scatter(
        &mut self,
        chart_ref: usize,
        x_vals: Vec<f64>,
        y_vals: Vec<f64>,
        labels: Vec<String>,
        chart_w: f64,
        chart_h: f64,
        has_groups: bool,
    ) {
        let n = x_vals.len().min(y_vals.len());
        if n == 0 {
            return;
        }
        let fallback = anchors::scatter_fallback(chart_w, chart_h, has_groups);
        self.update_chart_space(chart_ref, chart_w, chart_h, Some(fallback));
        let plot = self.plot_or(chart_ref, fallback);
        let Some(bounds) = anchors::scatter_bounds(&x_vals, &y_vals, n) else {
            return;
        };
        for i in 0..n {
            let (px, py) = anchors::project_scatter(plot, bounds, x_vals[i], y_vals[i]);
            if let Some(label) = labels.get(i) {
                if !label.is_empty() {
                    self.register_native_pin(chart_ref, label, px, py);
                }
            }
            self.register_native_pin(chart_ref, &format!("point:{}", i), px, py);
        }
    }

    #[pyo3(signature = (from_ref, from_name, to_ref, to_name, color = "#ffffff",
                        width = 1.5, opacity = 0.8, bend = 0.0, layer = "fg", name = ""))]
    pub fn connect(
        &mut self,
        from_ref: usize,
        from_name: &str,
        to_ref: usize,
        to_name: &str,
        color: &str,
        width: f64,
        opacity: f64,
        bend: f64,
        layer: &str,
        name: &str,
    ) {
        let p1 = self.pins.get(&Self::pin_key(from_ref, from_name)).copied();
        let p2 = self.pins.get(&Self::pin_key(to_ref, to_name)).copied();
        if let (Some((x1, y1)), Some((x2, y2))) = (p1, p2) {
            let dx = x2 - x1;
            let dy = y2 - y1;
            let dist = (dx * dx + dy * dy).sqrt().max(1.0);
            let mx = (x1 + x2) / 2.0 + (-dy / dist) * dist * bend;
            let my = (y1 + y2) / 2.0 + (dx / dist) * dist * bend;
            let pts = vec![(x1, y1), (mx, my), (x2, y2)];
            let d = catmull_rom(&pts, 1.0);
            let element_idx = self.elements.len();
            self.register_name(name, element_idx);
            self.elements.push(El::RawPath {
                d,
                fill: "none".to_string(),
                stroke: color.to_string(),
                sw: width,
                opacity,
                layer: Layer::from_str(layer),
                name: name.to_string(),
            });
        }
    }

    #[pyo3(signature = (chart_ref, pin_name, text, offset_x = 60.0, offset_y = -30.0,
                        color = "#ffffff", size = 12.0, line_dash = "3,4",
                        line_width = 1.0, bg = "", layer = "fg", name = ""))]
    pub fn annotate_at(
        &mut self,
        chart_ref: usize,
        pin_name: &str,
        text: &str,
        offset_x: f64,
        offset_y: f64,
        color: &str,
        size: f64,
        line_dash: &str,
        line_width: f64,
        bg: &str,
        layer: &str,
        name: &str,
    ) {
        if let Some((ax, ay)) = self.pins.get(&Self::pin_key(chart_ref, pin_name)).copied() {
            let element_idx = self.elements.len();
            self.register_name(name, element_idx);
            self.elements.push(El::Annotate {
                text: text.to_string(),
                ax,
                ay,
                tx: ax + offset_x,
                ty: ay + offset_y,
                color: color.to_string(),
                size,
                line_dash: line_dash.to_string(),
                lw: line_width,
                bg: bg.to_string(),
                layer: Layer::from_str(layer),
                name: name.to_string(),
            });
        }
    }

    #[pyo3(signature = (content, x, y, size = 24.0, color = "#ffffff", weight = "normal",
                        anchor = "start", rotation = 0.0, letter_spacing = 0.0,
                        font = "sans-serif", opacity = 1.0, layer = "fg", name = ""))]
    pub fn text(
        &mut self,
        content: &str,
        x: f64,
        y: f64,
        size: f64,
        color: &str,
        weight: &str,
        anchor: &str,
        rotation: f64,
        letter_spacing: f64,
        font: &str,
        opacity: f64,
        layer: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Text {
            content: content.to_string(),
            x,
            y,
            size,
            color: color.to_string(),
            opacity,
            rotation,
            anchor: anchor.to_string(),
            weight: weight.to_string(),
            ls: letter_spacing,
            font: font.to_string(),
            layer: Layer::from_str(layer),
            name: name.to_string(),
            group: String::new(),
        });
        element_idx
    }

    #[pyo3(signature = (x1, y1, x2, y2, color = "#ffffff", width = 1.5, dash = "",
                        opacity = 1.0, cap = "round", layer = "fg", hover_group = "", name = ""))]
    pub fn line(
        &mut self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        color: &str,
        width: f64,
        dash: &str,
        opacity: f64,
        cap: &str,
        layer: &str,
        hover_group: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Line {
            x1,
            y1,
            x2,
            y2,
            color: color.to_string(),
            width,
            dash: dash.to_string(),
            opacity,
            cap: cap.to_string(),
            layer: Layer::from_str(layer),
            group: hover_group.to_string(),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (points, color = "#ffffff", width = 1.5, opacity = 1.0,
                        tension = 1.0, fill = "none", layer = "fg", name = ""))]
    pub fn curve(
        &mut self,
        points: Vec<Vec<f64>>,
        color: &str,
        width: f64,
        opacity: f64,
        tension: f64,
        fill: &str,
        layer: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Curve {
            pts: parse_pts(points),
            color: color.to_string(),
            width,
            opacity,
            tension,
            fill: fill.to_string(),
            layer: Layer::from_str(layer),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (x1, y1, x2, y2, color = "#ffffff", width = 1.5, opacity = 1.0,
                        bend = 0.5, layer = "fg", name = ""))]
    pub fn connector(
        &mut self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        color: &str,
        width: f64,
        opacity: f64,
        bend: f64,
        layer: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Connector {
            x1,
            y1,
            x2,
            y2,
            color: color.to_string(),
            width,
            opacity,
            bend,
            layer: Layer::from_str(layer),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (cx, cy, r, fill = "none", stroke = "#ffffff",
                        stroke_width = 1.5, opacity = 1.0, layer = "fg", hover_group = "", name = ""))]
    pub fn circle(
        &mut self,
        cx: f64,
        cy: f64,
        r: f64,
        fill: &str,
        stroke: &str,
        stroke_width: f64,
        opacity: f64,
        layer: &str,
        hover_group: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Circle {
            cx,
            cy,
            r,
            fill: fill.to_string(),
            stroke: stroke.to_string(),
            sw: stroke_width,
            opacity,
            layer: Layer::from_str(layer),
            group: hover_group.to_string(),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (cx, cy, inner_r, outer_r, fill = "#ffffff", stroke = "none",
                        stroke_width = 0.0, opacity = 1.0, layer = "fg", name = ""))]
    pub fn ring(
        &mut self,
        cx: f64,
        cy: f64,
        inner_r: f64,
        outer_r: f64,
        fill: &str,
        stroke: &str,
        stroke_width: f64,
        opacity: f64,
        layer: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Ring {
            cx,
            cy,
            r_inner: inner_r,
            r_outer: outer_r,
            fill: fill.to_string(),
            stroke: stroke.to_string(),
            sw: stroke_width,
            opacity,
            layer: Layer::from_str(layer),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (x, y, w, h, fill = "none", stroke = "#ffffff",
                        stroke_width = 1.5, rx = 0.0, opacity = 1.0, rotation = 0.0,
                        layer = "fg", name = ""))]
    pub fn rect(
        &mut self,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        fill: &str,
        stroke: &str,
        stroke_width: f64,
        rx: f64,
        opacity: f64,
        rotation: f64,
        layer: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Rect {
            x,
            y,
            w,
            h,
            fill: fill.to_string(),
            stroke: stroke.to_string(),
            sw: stroke_width,
            rx,
            opacity,
            rotation,
            layer: Layer::from_str(layer),
            name: name.to_string(),
            group: String::new(),
        });
        element_idx
    }

    #[pyo3(signature = (points, fill = "none", stroke = "#ffffff",
                        stroke_width = 1.5, opacity = 1.0, layer = "fg", name = ""))]
    pub fn polygon(
        &mut self,
        points: Vec<Vec<f64>>,
        fill: &str,
        stroke: &str,
        stroke_width: f64,
        opacity: f64,
        layer: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Polygon {
            pts: parse_pts(points),
            fill: fill.to_string(),
            stroke: stroke.to_string(),
            sw: stroke_width,
            opacity,
            layer: Layer::from_str(layer),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (sites, x, y, w, h, fills = None, stroke = "#0d1117",
                        stroke_width = 1.0, opacity = 1.0, layer = "fg", name_prefix = "cell"))]
    pub fn voronoi(
        &mut self,
        sites: Vec<Vec<f64>>,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        fills: Option<Vec<String>>,
        stroke: &str,
        stroke_width: f64,
        opacity: f64,
        layer: &str,
        name_prefix: &str,
    ) -> Vec<usize> {
        let pts = parse_pts(sites);
        let cells = voronoi_cells(&pts, x, y, w, h);
        let fills = fills.unwrap_or_default();
        let mut out = Vec::with_capacity(cells.len());
        for (i, cell) in cells.into_iter().enumerate() {
            if cell.len() < 3 {
                continue;
            }
            let fill = fills.get(i).cloned().unwrap_or_else(|| "none".to_string());
            let name = format!("{}{}", name_prefix, i);
            let element_idx = self.elements.len();
            self.register_name(&name, element_idx);
            self.elements.push(El::Polygon {
                pts: cell,
                fill,
                stroke: stroke.to_string(),
                sw: stroke_width,
                opacity,
                layer: Layer::from_str(layer),
                name,
            });
            out.push(element_idx);
        }
        out
    }

    #[pyo3(signature = (d, fill = "none", stroke = "#ffffff",
                        stroke_width = 1.5, opacity = 1.0, layer = "fg", name = ""))]
    pub fn path(
        &mut self,
        d: &str,
        fill: &str,
        stroke: &str,
        stroke_width: f64,
        opacity: f64,
        layer: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::RawPath {
            d: d.to_string(),
            fill: fill.to_string(),
            stroke: stroke.to_string(),
            sw: stroke_width,
            opacity,
            layer: Layer::from_str(layer),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (x1, y1, x2, y2, color = "#ffffff", width = 1.5,
                        head_size = 4.0, opacity = 1.0, layer = "fg", name = ""))]
    pub fn arrow(
        &mut self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        color: &str,
        width: f64,
        head_size: f64,
        opacity: f64,
        layer: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Arrow {
            x1,
            y1,
            x2,
            y2,
            color: color.to_string(),
            width,
            head_size,
            opacity,
            layer: Layer::from_str(layer),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (text, ax, ay, tx, ty, color = "#ffffff", size = 13.0,
                        line_dash = "", line_width = 1.0, bg = "", layer = "fg", name = ""))]
    pub fn annotate(
        &mut self,
        text: &str,
        ax: f64,
        ay: f64,
        tx: f64,
        ty: f64,
        color: &str,
        size: f64,
        line_dash: &str,
        line_width: f64,
        bg: &str,
        layer: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Annotate {
            text: text.to_string(),
            ax,
            ay,
            tx,
            ty,
            color: color.to_string(),
            size,
            line_dash: line_dash.to_string(),
            lw: line_width,
            bg: bg.to_string(),
            layer: Layer::from_str(layer),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (id, from_color, to_color, x1 = 0.0, y1 = 0.0, x2 = 1.0, y2 = 0.0))]
    pub fn gradient(
        &mut self,
        id: &str,
        from_color: &str,
        to_color: &str,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    ) {
        self.elements.push(El::GradDef {
            id: id.to_string(),
            from_color: from_color.to_string(),
            to_color: to_color.to_string(),
            x1,
            y1,
            x2,
            y2,
        });
    }

    pub fn build(&self) -> crate::Chart {
        let w = self.width;
        let h = self.height;
        let bg = &self.background;

        let has_hover = self.elements.iter().any(|el| match el {
            El::Line { group, .. }
            | El::Circle { group, .. }
            | El::Chart { group, .. }
            | El::Text { group, .. }
            | El::Rect { group, .. } => !group.is_empty(),
            _ => false,
        });
        let hover_css = if has_hover {
            "@keyframes sp-sat-pulse{0%,100%{filter:none;transform:scale(1)}50%{filter:drop-shadow(0 0 14px rgba(99,102,241,0.5));transform:scale(1.04)}}\n"
        } else {
            ""
        };
        let hover_js = if has_hover {
            concat!(
                "<script>(function(){",
                "var root=document.getElementById('sp-canvas-root');if(!root)return;",
                "var aG=null;",
                "function qL(g){return root.querySelectorAll('.sp-hvl[data-sp-grp=\"'+g+'\"]');}",
                "function qA(g){return root.querySelectorAll('.sp-anch[data-sp-grp=\"'+g+'\"]');}",
                "function qS(g){return root.querySelectorAll('div[data-sp-grp=\"'+g+'\"],rect[data-sp-grp=\"'+g+'\"],text[data-sp-grp=\"'+g+'\"]');}",
                "function act(g){",
                  "if(aG===g)return;if(aG)deact(aG);aG=g;",
                  "qL(g).forEach(function(l){",
                    "var c=l.getAttribute('stroke')||'#6366f1';",
                    "l.style.filter='drop-shadow(0 0 4px '+c+') drop-shadow(0 0 10px '+c+'80)';",
                    "l.setAttribute('stroke-opacity','1');",
                    "l.setAttribute('stroke-width',String(parseFloat(l.getAttribute('data-sw')||'1.2')*2.2));",
                  "});",
                  "qA(g).forEach(function(c){",
                    "var col=c.getAttribute('fill')||'#6366f1';",
                    "c.setAttribute('r',String(parseFloat(c.getAttribute('data-r')||'5')*1.9));",
                    "c.style.filter='drop-shadow(0 0 6px '+col+') drop-shadow(0 0 16px '+col+'60)';",
                  "});",
                  "qS(g).forEach(function(d){",
                    "d.style.animation='sp-sat-pulse 1.4s ease-in-out infinite';",
                    "d.style.transformOrigin='center center';",
                    "d.style.transformBox='fill-box';",
                  "});",
                "}",
                "function deact(g){",
                  "if(!g)return;aG=null;",
                  "qL(g).forEach(function(l){",
                    "l.style.filter='';",
                    "l.setAttribute('stroke-opacity',l.getAttribute('data-op')||'0.29');",
                    "l.setAttribute('stroke-width',l.getAttribute('data-sw')||'1.2');",
                  "});",
                  "qA(g).forEach(function(c){",
                    "c.setAttribute('r',c.getAttribute('data-r')||'5');",
                    "c.style.filter='';",
                  "});",
                  "qS(g).forEach(function(d){d.style.animation='';});",
                "}",
                "root.querySelectorAll('.sp-hvh,div[data-sp-grp],rect[data-sp-grp],text[data-sp-grp],circle.sp-anch[data-sp-grp]').forEach(function(hit){",
                  "var g=hit.getAttribute('data-sp-grp');if(!g)return;",
                  "hit.style.cursor='pointer';",
                  "hit.addEventListener('mouseenter',function(){act(g);});",
                  "hit.addEventListener('mouseleave',function(e){",
                    "var rt=e.relatedTarget;",
                    "if(!rt||!rt.closest||!rt.closest('[data-sp-grp=\"'+g+'\"]')){deact(g);}",
                  "});",
                "});",
                "})();</script>\n"
            )
        } else {
            ""
        };

        let mut bg_defs = String::new();
        let mut bg_body = String::new();
        let mut fg_defs = String::new();
        let mut fg_body = String::new();
        let mut frames = String::new();

        for el in &self.elements {
            match el {
                El::Chart { .. } => {
                    if let Some(f) = render_frame(el) {
                        frames.push_str(&f);
                    }
                }
                El::Image { .. } => {
                    if let Some(f) = render_image(el) {
                        frames.push_str(&f);
                    }
                }
                El::GradDef { .. } => {
                    render_el(el, &mut bg_defs, &mut bg_body);
                }
                _ => {
                    let layer = el_layer(el).unwrap_or(&Layer::Fg);
                    if *layer == Layer::Bg {
                        render_el(el, &mut bg_defs, &mut bg_body);
                    } else {
                        render_el(el, &mut fg_defs, &mut fg_body);
                    }
                }
            }
        }

        let mut extra_css = String::new();
        for (name, css) in &self.custom_css {
            if name.is_empty() {
                extra_css.push_str(css);
                extra_css.push('\n');
            } else {
                extra_css.push_str(&format!("[data-sp-name=\"{}\"]{{{}}}\n", name, css));
            }
        }

        let mut extra_js = String::new();
        for js in &self.custom_js {
            extra_js.push_str("<script>");
            extra_js.push_str(js);
            extra_js.push_str("</script>\n");
        }

        let html = format!(
            concat!(
                "<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"utf-8\">\n",
                "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n",
                "<style>\n*{{box-sizing:border-box}}\n",
                "html,body{{margin:0;padding:0;overflow:hidden;background:{bg}}}\n",
                "#sp-canvas-root{{position:relative;width:{w}px;height:{h}px;",
                "overflow:hidden;background:{bg};transform-origin:top left}}\n",
                ".sp-cv{{position:absolute;top:0;left:0}}\n",
                "{hover_css}",
                "{extra_css}",
                "</style>\n</head>\n<body>\n",
                "<div id=\"sp-canvas-root\">\n",
                "<svg class=\"sp-cv\" style=\"z-index:0;pointer-events:none\" ",
                "width=\"{w}\" height=\"{h}\" xmlns=\"http://www.w3.org/2000/svg\">\n",
                "<defs>{bg_defs}</defs>\n{bg_body}</svg>\n",
                "{frames}",
                "<svg class=\"sp-cv\" style=\"z-index:9999;pointer-events:none\" ",
                "width=\"{w}\" height=\"{h}\" xmlns=\"http://www.w3.org/2000/svg\">\n",
                "<defs>{fg_defs}</defs>\n{fg_body}</svg>\n",
                "</div>\n",
                "<script>(function(){{",
                "var W={w},H={h},root=document.getElementById('sp-canvas-root');",
                "function fit(){{",
                "var vw=window.innerWidth||document.documentElement.clientWidth||W;",
                "var vh=window.innerHeight||document.documentElement.clientHeight||H;",
                "var s=Math.min(vw/W,vh/H,1);",
                "if(!isFinite(s)||s<=0)s=1;",
                "root.style.transform='scale('+s+')';",
                "document.documentElement.style.width=(W*s)+'px';",
                "document.documentElement.style.height=(H*s)+'px';",
                "document.body.style.width=(W*s)+'px';",
                "document.body.style.height=(H*s)+'px';",
                "}}",
                "fit();window.addEventListener('resize',fit,{{passive:true}});",
                "}})();</script>\n",
                "{hover_js}",
                "{extra_js}",
                "</body>\n</html>"
            ),
            w = w,
            h = h,
            bg = bg,
            bg_defs = bg_defs,
            bg_body = bg_body,
            frames = frames,
            fg_defs = fg_defs,
            fg_body = fg_body,
            hover_css = hover_css,
            hover_js = hover_js,
            extra_css = extra_css,
            extra_js = extra_js,
        );

        crate::Chart { html, doc_str: "" }
    }

    pub fn dev(&self) -> crate::Chart {
        let chart = self.build();
        let html = chart.html.replacen(
            "</body>",
            &format!("<script>{}</script></body>", CANVAS_DEV_JS),
            1,
        );
        crate::Chart { html, doc_str: "" }
    }
}

#[pyfunction]
#[pyo3(signature = (width, height, bg = "#0a0a0f"))]
pub fn canvas(width: u32, height: u32, bg: &str) -> Canvas {
    Canvas::new(width, height, bg)
}

fn seraplot_home_dir() -> std::path::PathBuf {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    std::path::Path::new(&home).join(".seraplot").join("canvas")
}

fn canvas_index_path() -> std::path::PathBuf {
    seraplot_home_dir().join("index.json")
}

#[derive(Serialize, Deserialize, Default)]
struct CanvasIndexEntry {
    path: String,
    updated_at: u64,
}

#[derive(Serialize, Deserialize, Default)]
struct CanvasIndex {
    version: u32,
    canvases: HashMap<String, CanvasIndexEntry>,
}

fn load_index() -> CanvasIndex {
    let path = canvas_index_path();
    fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or(CanvasIndex {
            version: 1,
            canvases: HashMap::new(),
        })
}

fn save_index(index: &CanvasIndex) -> PyResult<()> {
    let dir = seraplot_home_dir();
    fs::create_dir_all(&dir).map_err(|e| {
        pyo3::exceptions::PyIOError::new_err(format!("cannot create '{}': {}", dir.display(), e))
    })?;
    let json = serde_json::to_string_pretty(index)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    fs::write(canvas_index_path(), json).map_err(|e| {
        pyo3::exceptions::PyIOError::new_err(format!("cannot write canvas index: {}", e))
    })
}

#[pyfunction]
pub fn canvas_default_dir() -> String {
    seraplot_home_dir().to_string_lossy().into_owned()
}

#[pyfunction]
pub fn canvas_load(path: &str) -> PyResult<Canvas> {
    let json = fs::read_to_string(path).map_err(|e| {
        pyo3::exceptions::PyIOError::new_err(format!("cannot read '{}': {}", path, e))
    })?;
    let state: CanvasState = serde_json::from_str(&json)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
    Ok(Canvas::from_state(state))
}

#[pyfunction]
pub fn canvas_save_named(canvas: &Canvas, name: &str) -> PyResult<String> {
    let dir = seraplot_home_dir();
    let path = dir.join(format!("{}.json", name));
    canvas.save(&path.to_string_lossy())?;
    let mut index = load_index();
    let updated_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    index.canvases.insert(
        name.to_string(),
        CanvasIndexEntry {
            path: path.to_string_lossy().into_owned(),
            updated_at,
        },
    );
    save_index(&index)?;
    Ok(path.to_string_lossy().into_owned())
}

#[pyfunction]
pub fn canvas_load_named(name: &str) -> PyResult<Canvas> {
    let index = load_index();
    let entry = index.canvases.get(name).ok_or_else(|| {
        pyo3::exceptions::PyKeyError::new_err(format!("no saved canvas named '{}'", name))
    })?;
    canvas_load(&entry.path)
}
