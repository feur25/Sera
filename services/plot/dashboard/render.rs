use super::element::{El, Layer};
use super::geometry::{catmull_rom, next_mid, polar_xy, pts_to_svg};
use super::html_util::{clip_path_css, escape_html, grp_attr, name_attr};

pub(super) fn render_el(el: &El, defs: &mut String, body: &mut String) {
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
            let pe = if group.is_empty() {
                String::new()
            } else {
                " pointer-events=\"all\"".to_string()
            };
            body.push_str(&format!(
                "<text x=\"{:.2}\" y=\"{:.2}\" font-size=\"{:.2}\" fill=\"{}\" \
                 opacity=\"{:.4}\" font-weight=\"{}\" text-anchor=\"{}\" \
                 font-family=\"{}\" letter-spacing=\"{:.2}\"{}{}{}{}>\n",
                x, y, size, color, opacity, weight, anchor, font, ls, rot, name_attr(name), grp_attr(group), pe
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
                    "<circle class=\"sp-anch\" data-sp-grp=\"{}\" data-group=\"{}\" data-r=\"{:.2}\" \
                     cx=\"{:.2}\" cy=\"{:.2}\" r=\"{:.2}\" fill=\"{}\" \
                     stroke=\"{}\" stroke-width=\"{:.2}\" opacity=\"{:.4}\"{} pointer-events=\"all\"/>\n",
                    group, group, r, cx, cy, r, fill, stroke, sw, opacity, name_attr(name)
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
            let pe = if group.is_empty() {
                String::new()
            } else {
                " pointer-events=\"all\"".to_string()
            };
            body.push_str(&format!(
                "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" \
                 fill=\"{}\" stroke=\"{}\" stroke-width=\"{:.2}\" rx=\"{:.2}\" \
                 opacity=\"{:.4}\"{}{}{}{}/>\n",
                x, y, w, h, fill, stroke, sw, rx, opacity, rot, name_attr(name), grp_attr(group), pe
            ));
        }

        El::Polygon {
            pts,
            fill,
            stroke,
            sw,
            opacity,
            group,
            name,
            ..
        } => {
            body.push_str(&format!(
                "<polygon points=\"{}\" fill=\"{}\" stroke=\"{}\" \
                 stroke-width=\"{:.2}\" opacity=\"{:.4}\"{}{}/>\n",
                pts_to_svg(pts),
                fill,
                stroke,
                sw,
                opacity,
                grp_attr(group),
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

        El::Arc {
            cx,
            cy,
            r,
            start_deg,
            end_deg,
            color,
            width,
            opacity,
            cap,
            name,
            ..
        } => {
            let (x1, y1) = polar_xy(*cx, *cy, *r, *start_deg);
            let (x2, y2) = polar_xy(*cx, *cy, *r, *end_deg);
            let large = if (end_deg - start_deg).abs() > 180.0 { 1 } else { 0 };
            let sweep = if *end_deg >= *start_deg { 1 } else { 0 };
            body.push_str(&format!(
                "<path d=\"M {:.2},{:.2} A {:.2},{:.2} 0 {},{} {:.2},{:.2}\" fill=\"none\" \
                 stroke=\"{}\" stroke-width=\"{:.2}\" opacity=\"{:.4}\" stroke-linecap=\"{}\"{}/>\n",
                x1, y1, r, r, large, sweep, x2, y2, color, width, opacity, cap, name_attr(name)
            ));
        }

        El::Wedge {
            cx,
            cy,
            r_inner,
            r_outer,
            start_deg,
            end_deg,
            fill,
            stroke,
            sw,
            opacity,
            group,
            name,
            ..
        } => {
            let (ox1, oy1) = polar_xy(*cx, *cy, *r_outer, *start_deg);
            let (ox2, oy2) = polar_xy(*cx, *cy, *r_outer, *end_deg);
            let large = if (end_deg - start_deg).abs() > 180.0 { 1 } else { 0 };
            let d = if *r_inner <= 0.001 {
                format!(
                    "M {:.2},{:.2} L {:.2},{:.2} A {:.2},{:.2} 0 {},1 {:.2},{:.2} Z",
                    cx, cy, ox1, oy1, r_outer, r_outer, large, ox2, oy2
                )
            } else {
                let (ix1, iy1) = polar_xy(*cx, *cy, *r_inner, *end_deg);
                let (ix2, iy2) = polar_xy(*cx, *cy, *r_inner, *start_deg);
                format!(
                    "M {:.2},{:.2} A {:.2},{:.2} 0 {},1 {:.2},{:.2} L {:.2},{:.2} \
                     A {:.2},{:.2} 0 {},0 {:.2},{:.2} Z",
                    ox1, oy1, r_outer, r_outer, large, ox2, oy2, ix1, iy1, r_inner, r_inner, large, ix2, iy2
                )
            };
            let (cls, pe) = if group.is_empty() {
                (String::new(), String::new())
            } else {
                (
                    " class=\"sp-wedge\"".to_string(),
                    " pointer-events=\"all\"".to_string(),
                )
            };
            body.push_str(&format!(
                "<path d=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{:.2}\" opacity=\"{:.4}\"{}{}{}{}/>\n",
                d, fill, stroke, sw, opacity, cls, name_attr(name), grp_attr(group), pe
            ));
        }

        El::Ribbon {
            cx,
            cy,
            r,
            a_start,
            a_end,
            b_start,
            b_end,
            fill,
            opacity,
            name,
            ..
        } => {
            let (s1x, s1y) = polar_xy(*cx, *cy, *r, *a_start);
            let (s2x, s2y) = polar_xy(*cx, *cy, *r, *a_end);
            let (t1x, t1y) = polar_xy(*cx, *cy, *r, *b_start);
            let (t2x, t2y) = polar_xy(*cx, *cy, *r, *b_end);
            let large_a = if (a_end - a_start).abs() > 180.0 { 1 } else { 0 };
            let large_b = if (b_end - b_start).abs() > 180.0 { 1 } else { 0 };
            body.push_str(&format!(
                "<path d=\"M {:.2},{:.2} A {:.2},{:.2} 0 {},1 {:.2},{:.2} Q {:.2},{:.2} {:.2},{:.2} \
                 A {:.2},{:.2} 0 {},1 {:.2},{:.2} Q {:.2},{:.2} {:.2},{:.2} Z\" \
                 fill=\"{}\" opacity=\"{:.4}\"{}/>\n",
                s1x, s1y, r, r, large_a, s2x, s2y, cx, cy, t2x, t2y,
                r, r, large_b, t1x, t1y, cx, cy, s1x, s1y,
                fill, opacity, name_attr(name)
            ));
        }

        El::RadialGradDef {
            id,
            from_color,
            to_color,
            cx,
            cy,
            r,
        } => {
            defs.push_str(&format!(
                "<radialGradient id=\"{}\" cx=\"{:.3}\" cy=\"{:.3}\" r=\"{:.3}\" gradientUnits=\"objectBoundingBox\">\
                 <stop offset=\"0\" stop-color=\"{}\"/>\
                 <stop offset=\"1\" stop-color=\"{}\"/>\
                 </radialGradient>\n",
                id, cx, cy, r, from_color, to_color
            ));
        }

        El::Chart { .. } | El::Image { .. } => {}
    }
}

pub(super) fn render_frame(el: &El) -> Option<String> {
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
    let mt = super::anchors::NativeTransform::new(*w, *h, nw, nh);
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

pub(super) fn render_image(el: &El) -> Option<String> {
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

pub(super) fn el_layer(el: &El) -> Option<&Layer> {
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
        | El::Annotate { layer, .. }
        | El::Arc { layer, .. }
        | El::Wedge { layer, .. }
        | El::Ribbon { layer, .. } => Some(layer),
        _ => None,
    }
}

pub(super) fn translate_element(el: &mut El, dx: f64, dy: f64) {
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
        El::Arc { cx, cy, .. } | El::Wedge { cx, cy, .. } | El::Ribbon { cx, cy, .. } => {
            *cx += dx;
            *cy += dy;
        }
        El::RawPath { .. } | El::GradDef { .. } | El::RadialGradDef { .. } => {}
    }
}

pub(super) fn resize_element(el: &mut El, dw: f64, dh: f64) -> bool {
    match el {
        El::Chart { w, h, .. } | El::Image { w, h, .. } | El::Rect { w, h, .. } => {
            *w = (*w + dw).max(4.0);
            *h = (*h + dh).max(4.0);
            true
        }
        _ => false,
    }
}
