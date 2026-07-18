use serde::Deserialize;
use std::cell::RefCell;

thread_local! {
    static GLOBAL_BG: RefCell<Option<String>> = RefCell::new(None);
    static GLOBAL_PAL: RefCell<Vec<u32>> = RefCell::new(Vec::new());
    static GLOBAL_GRID: std::cell::Cell<bool> = std::cell::Cell::new(false);
    static GLOBAL_DESPINE: std::cell::Cell<bool> = std::cell::Cell::new(false);
    static GLOBAL_WATERMARK: RefCell<Option<(String, f64)>> = RefCell::new(None);
    static GLOBAL_SHADOW: RefCell<Option<(i32, Option<String>)>> = RefCell::new(None);
}

pub fn set_global_despine(v: bool) {
    GLOBAL_DESPINE.with(|g| g.set(v));
}

pub fn get_global_despine() -> bool {
    GLOBAL_DESPINE.with(|g| g.get())
}

pub fn set_global_watermark(v: Option<(String, f64)>) {
    GLOBAL_WATERMARK.with(|g| *g.borrow_mut() = v);
}

pub fn get_global_watermark() -> Option<(String, f64)> {
    GLOBAL_WATERMARK.with(|g| g.borrow().clone())
}

pub fn set_global_shadow(v: Option<(i32, Option<String>)>) {
    GLOBAL_SHADOW.with(|g| *g.borrow_mut() = v);
}

pub fn get_global_shadow() -> Option<(i32, Option<String>)> {
    GLOBAL_SHADOW.with(|g| g.borrow().clone())
}

pub fn set_global_bg(color: Option<String>) {
    GLOBAL_BG.with(|bg| *bg.borrow_mut() = color);
}

pub fn set_global_pal(pal: Vec<u32>) {
    GLOBAL_PAL.with(|p| *p.borrow_mut() = pal);
}

pub fn set_global_grid(v: bool) {
    GLOBAL_GRID.with(|g| g.set(v));
}

pub fn get_global_bg() -> Option<String> {
    GLOBAL_BG.with(|bg| bg.borrow().clone())
}

pub fn get_global_pal() -> Vec<u32> {
    GLOBAL_PAL.with(|p| p.borrow().clone())
}

pub fn get_global_grid() -> bool {
    GLOBAL_GRID.with(|g| g.get())
}

#[crate::sera_register]
pub fn set_global_background(input: &str) -> String {
    let color = input.trim().trim_matches('"');
    set_global_bg(if color.is_empty() {
        None
    } else {
        Some(color.to_string())
    });
    String::new()
}

#[crate::sera_register]
pub fn reset_global_background(_: &str) -> String {
    set_global_bg(None);
    String::new()
}

#[crate::sera_register]
pub fn set_theme(input: &str) -> String {
    let name = input.trim().trim_matches('"');
    let (bg, pal, grid): (Option<&str>, &[u32], bool) = match name {
        "dark" => (
            Some("#0f172a"),
            &[
                0x818CF8, 0x34D399, 0xFBBF24, 0xF87171, 0x60A5FA, 0xA78BFA, 0xFB7185, 0x2DD4BF,
                0xF472B6, 0xFACC15,
            ],
            true,
        ),
        "light" => (
            None,
            &[
                0x636EFA, 0xEF553B, 0x00CC96, 0xAB63FA, 0xFFA15A, 0x19D3F3, 0xFF6692, 0xB6E880,
                0xFF97FF, 0xFECB52,
            ],
            false,
        ),
        "scientific" => (
            Some("#fafafa"),
            &[
                0x1F77B4, 0xFF7F0E, 0x2CA02C, 0xD62728, 0x9467BD, 0x8C564B, 0xE377C2, 0x7F7F7F,
                0xBCBD22, 0x17BECF,
            ],
            true,
        ),
        "apple" => (
            Some("#000000"),
            &[
                0xFF375F, 0x30D158, 0x0A84FF, 0xFFD60A, 0xFF9F0A, 0x5E5CE6, 0x64D2FF, 0xBF5AF2,
                0xFF6961, 0x32ADE6,
            ],
            false,
        ),
        "notion" => (
            Some("#191919"),
            &[
                0xE3E3E3, 0xA0A0A0, 0xCB9D6D, 0x7C9E7E, 0x7B8FC4, 0xC17B7B, 0xD4A76A, 0x8BA4B0,
                0xB39DDB, 0x80CBC4,
            ],
            false,
        ),
        "minimal" => (
            None,
            &[
                0x222222, 0x444444, 0x666666, 0x888888, 0xAAAAAA, 0xCCCCCC, 0x111111, 0x333333,
                0x555555, 0x777777,
            ],
            false,
        ),
        "neon" => (
            Some("#0a0a0a"),
            &[
                0x00FFF0, 0xFF00FF, 0x00FF41, 0xFF6B00, 0xFFFF00, 0xFF1493, 0x00BFFF, 0xFF4500,
                0x7FFF00, 0xDA70D6,
            ],
            false,
        ),
        _ => return String::new(),
    };
    set_global_bg(bg.map(str::to_string));
    set_global_pal(pal.to_vec());
    set_global_grid(grid);
    String::new()
}

#[crate::sera_register]
pub fn reset_theme(_: &str) -> String {
    set_global_bg(None);
    set_global_pal(Vec::new());
    set_global_grid(false);
    String::new()
}

#[crate::sera_register]
pub fn themes(_: &str) -> String {
    "[\"dark\",\"light\",\"scientific\",\"apple\",\"notion\",\"minimal\",\"neon\"]".to_string()
}

#[crate::sera_alias("accessible_palette", "wcag_palette", "colorblind_palette")]
#[crate::sera_register]
pub fn accessible_palette(_: &str) -> String {
    serde_json::to_string(crate::plot::statistical::common::PALETTE_ACCESSIBLE).unwrap_or_default()
}

#[crate::sera_alias("plot", "chart", "draw", "render")]
#[crate::sera_register(custom, chart)]
pub fn plot_chart(input: &str) -> String {
    #[derive(Deserialize, Default)]
    struct In {
        x: Option<Vec<f64>>,
        y: Option<Vec<f64>>,
        kind: Option<String>,
        title: Option<String>,
        color_hex: Option<u32>,
        width: Option<i32>,
        height: Option<i32>,
        x_label: Option<String>,
        y_label: Option<String>,
        gridlines: Option<bool>,
        palette: Option<Vec<u32>>,
        background: Option<String>,
        show_points: Option<bool>,
    }
    let sanitized = crate::plot::chart_input::sanitize_non_finite_json(input);
    let payload: In = serde_json::from_str(&sanitized).unwrap_or_default();
    let xs = payload.x.unwrap_or_default();
    let title = payload.title.unwrap_or_default();
    let color_hex = payload.color_hex.unwrap_or(0x636EFA);
    let width = payload.width.unwrap_or(900);
    let height = payload.height.unwrap_or(480);
    let x_label = payload.x_label.unwrap_or_default();
    let y_label = payload.y_label.unwrap_or_default();
    let gridlines = payload.gridlines.unwrap_or(false);
    let show_points = payload.show_points.unwrap_or(true);
    let kind = payload.kind.as_deref().unwrap_or("line");
    if let Some(ys) = payload.y {
        if kind == "scatter" {
            crate::plot::statistical::scatter::build(&serde_json::json!({"title":title,"x":xs,"y":ys,"color_hex":color_hex,"width":width,"height":height,"x_label":x_label,"y_label":y_label,"gridlines":gridlines,"palette":payload.palette,"background":payload.background}).to_string())
        } else {
            let labels: Vec<String> = xs.iter().map(|v| v.to_string()).collect();
            crate::plot::default::build_line_chart(&serde_json::json!({"title":title,"labels":labels,"values":ys,"color_hex":color_hex,"show_points":show_points,"width":width,"height":height,"x_label":x_label,"y_label":y_label,"gridlines":gridlines,"palette":payload.palette,"background":payload.background}).to_string())
        }
    } else {
        let labels: Vec<String> = (0..xs.len()).map(|idx| idx.to_string()).collect();
        crate::plot::default::build_line_chart(&serde_json::json!({"title":title,"labels":labels,"values":xs,"color_hex":color_hex,"show_points":show_points,"width":width,"height":height,"x_label":x_label,"y_label":y_label,"gridlines":gridlines,"palette":payload.palette,"background":payload.background}).to_string())
    }
}

#[crate::sera_alias("grid", "grids", "chart_grid", "subplot_grid")]
#[crate::sera_register(custom, chart)]
pub fn build_grid(input: &str) -> String {
    let (title_s, args, opts) = crate::plot::parse_all(input);
    let title = title_s.as_str();
    let charts = args.charts.unwrap_or_default();
    let cols = opts.cols.unwrap_or(2).max(1);
    let gap = opts.gap.unwrap_or(16).max(0);
    let bg_color = opts
        .background
        .clone()
        .or_else(crate::plot::get_global_bg)
        .unwrap_or_else(|| "transparent".to_string());
    let _ = opts.cell_height;
    crate::bindings::chart_methods::build_grid_page(&charts, cols, Some(title), gap as u32, &bg_color)
}

#[crate::sera_register(custom, chart)]
pub fn build_slideshow(input: &str) -> String {
    let (title_s, args, opts) = crate::plot::parse_all(input);
    let title = title_s.as_str();
    let charts = args.charts.unwrap_or_default();
    if charts.is_empty() {
        return String::new();
    }
    let ivms = opts.interval_ms.unwrap_or(2500);
    let width = opts.w(900);
    let height = opts.h(520);
    let show_title = if !title.is_empty() {
        title
    } else {
        opts.label.as_deref().unwrap_or("")
    };
    let title_html = if show_title.is_empty() {
        String::new()
    } else {
        format!("<div style=\"color:#1e293b;font-family:system-ui;font-size:22px;font-weight:700;text-align:center;margin-bottom:16px\">{}</div>", show_title)
    };
    let n = charts.len();
    let slideshow_label = if show_title.is_empty() { "Slideshow" } else { show_title };
    let mut frames_html = String::new();
    for (i, html) in charts.iter().enumerate() {
        let esc = html.replace('&', "&amp;").replace('"', "&quot;");
        let vis = if i == 0 { "" } else { "display:none;" };
        frames_html.push_str(&format!(
            "<iframe id=\"sp-s-{i}\" title=\"Slide {slide_n} of {n}\" style=\"{vis}width:{width}px;height:{height}px;border:none;border-radius:12px;overflow:hidden;box-shadow:0 2px 12px rgba(0,0,0,.1)\" srcdoc=\"{esc}\"></iframe>",
            slide_n = i + 1
        ));
    }
    format!(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><style>\
        body{{margin:0;padding:24px;background:#f0f2f5;display:flex;flex-direction:column;align-items:center;font-family:system-ui}}\
        .sp-ctrl{{display:flex;gap:10px;margin-top:14px;align-items:center}}\
        .sp-btn{{cursor:pointer;background:#6366f1;color:#fff;border:none;border-radius:8px;padding:7px 20px;font-size:14px;font-weight:600}}\
        .sp-btn:hover{{background:#4f46e5}}\
        .sp-ctr{{color:#64748b;font-size:13px;min-width:64px;text-align:center}}\
        .sp-prog{{width:{width}px;height:4px;background:#e2e8f0;border-radius:2px;margin-top:10px;overflow:hidden}}\
        .sp-bar{{height:100%;background:#6366f1;border-radius:2px;width:0%}}\
        </style></head><body>\
        {title_html}\
        <div role=\"region\" aria-label=\"{slideshow_label}\">\
        {frames_html}\
        </div>\
        <div class=\"sp-ctrl\">\
        <button class=\"sp-btn\" id=\"sp-p\" aria-label=\"Previous slide\">&#9664;</button>\
        <button class=\"sp-btn\" id=\"sp-play\" aria-label=\"Pause slideshow\">&#10074;&#10074;</button>\
        <div class=\"sp-ctr\" id=\"sp-c\" role=\"status\" aria-live=\"polite\">1 / {n}</div>\
        <button class=\"sp-btn\" id=\"sp-n\" aria-label=\"Next slide\">&#9654;</button>\
        </div>\
        <div class=\"sp-prog\"><div class=\"sp-bar\" id=\"sp-b\"></div></div>\
        <script>\
        const slides=document.querySelectorAll('[id^=\"sp-s-\"]');\
        let idx=0,timer,playing=true;\
        const reduceMotion=window.matchMedia&&window.matchMedia('(prefers-reduced-motion: reduce)').matches;\
        const playBtn=document.getElementById('sp-play');\
        function show(i){{idx=((i%slides.length)+slides.length)%slides.length;\
          slides.forEach((s,j)=>{{s.style.display=j===idx?'':'none';}});\
          document.getElementById('sp-c').textContent=(idx+1)+' / '+slides.length;\
          const b=document.getElementById('sp-b');\
          b.style.transition='none';b.style.width='0%';\
          if(playing)setTimeout(()=>{{b.style.transition='width {ivms}ms linear';b.style.width='100%';}},20);}}\
        function play(){{clearInterval(timer);playing=true;playBtn.textContent='\\u276c\\u276c';playBtn.setAttribute('aria-label','Pause slideshow');timer=setInterval(()=>{{show(idx+1);}},{ivms});}}\
        function pause(){{clearInterval(timer);playing=false;playBtn.textContent='\\u25b6';playBtn.setAttribute('aria-label','Play slideshow');const b=document.getElementById('sp-b');b.style.transition='none';}}\
        show(0);\
        if(reduceMotion)pause();else play();\
        document.getElementById('sp-p').onclick=()=>{{show(idx-1);if(playing)play();}};\
        document.getElementById('sp-n').onclick=()=>{{show(idx+1);if(playing)play();}};\
        playBtn.onclick=()=>{{playing?pause():play();}};\
        </script></body></html>"
    )
}

#[crate::sera_register(custom)]
pub fn build_hover_json(input: &str) -> String {
    #[derive(Deserialize, Default)]
    struct In {
        labels: Option<Vec<String>>,
        images: Option<Vec<Option<String>>>,
        descriptions: Option<Vec<Vec<Vec<String>>>>,
    }
    let payload: In = serde_json::from_str(input).unwrap_or_default();
    let labels = payload.labels.unwrap_or_default();
    use crate::html::hover::{slots_to_json, HoverSlot};
    let mut slots = Vec::with_capacity(labels.len());
    for idx in 0..labels.len() {
        let mut slot = HoverSlot::new(&labels[idx]);
        if let Some(ref images) = payload.images {
            if let Some(Some(ref url)) = images.get(idx) {
                slot = slot.image(url.clone());
            }
        }
        if let Some(ref descriptions) = payload.descriptions {
            if let Some(row) = descriptions.get(idx) {
                for kv in row {
                    if kv.len() >= 2 {
                        slot = slot.kv(kv[0].clone(), kv[1].clone());
                    }
                }
            }
        }
        slots.push(slot);
    }
    slots_to_json(&slots)
}

#[crate::sera_register]
pub fn chart_append(input: &str) -> String {
    #[derive(Deserialize)]
    struct In {
        kind: String,
        x: Option<Vec<f64>>,
        y: Option<Vec<f64>>,
        title: Option<String>,
        max_points: Option<usize>,
        color_hex: Option<u32>,
        width: Option<i32>,
        height: Option<i32>,
        prev_x: Option<Vec<f64>>,
        prev_y: Option<Vec<f64>>,
    }
    let payload: In = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(_) => return String::new(),
    };
    let mut xs = payload.prev_x.unwrap_or_default();
    let mut ys = payload.prev_y.unwrap_or_default();
    if let Some(nx) = payload.x {
        xs.extend(nx);
    }
    if let Some(ny) = payload.y {
        ys.extend(ny);
    }
    if let Some(limit) = payload.max_points {
        if xs.len() > limit {
            let cut = xs.len() - limit;
            xs.drain(..cut);
        }
        if ys.len() > limit {
            let cut = ys.len() - limit;
            ys.drain(..cut);
        }
    }
    let title = payload.title.unwrap_or_default();
    let color = payload.color_hex.unwrap_or(0x636EFA);
    let width = payload.width.unwrap_or(900);
    let height = payload.height.unwrap_or(420);
    let base = serde_json::json!({"title":title,"x":xs,"y":ys,"color_hex":color,"width":width,"height":height,"show_points":true});
    let html = match payload.kind.as_str() {
        "scatter" => crate::plot::statistical::scatter::build(&base.to_string()),
        _ => crate::plot::default::build_line_chart(&serde_json::json!({"title":title,"labels":xs.iter().map(|v| v.to_string()).collect::<Vec<_>>(),"values":ys,"color_hex":color,"width":width,"height":height,"show_points":true}).to_string()),
    };
    serde_json::json!({"html":html,"x":xs,"y":ys}).to_string()
}

#[crate::sera_register(custom)]
pub fn export_svg(input: &str) -> String {
    #[derive(serde::Deserialize)]
    struct In {
        html: String,
    }
    let v: In = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(_) => return String::new(),
    };
    let h = v.html;
    let start = match h.find("<svg") {
        Some(i) => i,
        None => return String::new(),
    };
    let end = match h[start..].find("</svg>") {
        Some(i) => start + i + 6,
        None => return String::new(),
    };
    h[start..end].to_string()
}

#[crate::sera_register(custom)]
pub fn export_data_url(input: &str) -> String {
    use base64::Engine;
    let svg = export_svg(input);
    if svg.is_empty() {
        return String::new();
    }
    let b64 = base64::engine::general_purpose::STANDARD.encode(svg.as_bytes());
    format!("data:image/svg+xml;base64,{b64}")
}

#[crate::sera_register]
pub fn export_html_file(input: &str) -> String {
    #[derive(serde::Deserialize)]
    struct In {
        html: String,
        path: String,
    }
    let v: In = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(_) => return "{\"ok\":false}".to_string(),
    };
    match std::fs::write(&v.path, v.html) {
        Ok(_) => format!(
            "{{\"ok\":true,\"path\":{}}}",
            serde_json::to_string(&v.path).unwrap_or_else(|_| "\"\"".into())
        ),
        Err(e) => format!(
            "{{\"ok\":false,\"error\":{}}}",
            serde_json::to_string(&e.to_string()).unwrap_or_else(|_| "\"\"".into())
        ),
    }
}

#[crate::sera_register(custom)]
pub fn chart_info(input: &str) -> String {
    #[derive(serde::Deserialize)]
    struct In {
        html: String,
    }
    let v: In = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(_) => return "{}".to_string(),
    };
    let h = v.html;
    let len = h.len();
    let n_paths = h.matches("<path").count();
    let n_rects = h.matches("<rect").count();
    let n_circles = h.matches("<circle").count();
    let has_svg = h.contains("<svg");
    serde_json::json!({"size":len,"paths":n_paths,"rects":n_rects,"circles":n_circles,"has_svg":has_svg}).to_string()
}

#[crate::sera_register]
pub fn validate_input(input: &str) -> String {
    #[derive(serde::Deserialize)]
    struct In {
        labels: Option<Vec<String>>,
        values: Option<Vec<f64>>,
        x: Option<Vec<f64>>,
        y: Option<Vec<f64>>,
        series: Option<Vec<Vec<f64>>>,
    }
    let v: In = match serde_json::from_str::<In>(input) {
        Ok(v) => v,
        Err(e) => {
            return serde_json::json!({"ok":false,"error":format!("invalid JSON: {e}")}).to_string()
        }
    };
    if let (Some(l), Some(va)) = (v.labels.as_ref(), v.values.as_ref()) {
        if l.len() != va.len() {
            return serde_json::json!({"ok":false,"error":format!("labels ({}) and values ({}) length mismatch", l.len(), va.len())}).to_string();
        }
    }
    if let (Some(x), Some(y)) = (v.x.as_ref(), v.y.as_ref()) {
        if x.len() != y.len() {
            return serde_json::json!({"ok":false,"error":format!("x ({}) and y ({}) length mismatch", x.len(), y.len())}).to_string();
        }
    }
    if let (Some(l), Some(s)) = (v.labels.as_ref(), v.series.as_ref()) {
        for (i, row) in s.iter().enumerate() {
            if row.len() != l.len() {
                return serde_json::json!({"ok":false,"error":format!("series[{i}] length {} != labels length {}", row.len(), l.len())}).to_string();
            }
        }
    }
    serde_json::json!({"ok":true}).to_string()
}

#[crate::sera_register]
pub fn downsample_lttb(input: &str) -> String {
    #[derive(serde::Deserialize)]
    struct In {
        x: Vec<f64>,
        y: Vec<f64>,
        threshold: usize,
    }
    let v: In = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(e) => {
            return serde_json::json!({"ok":false,"error":format!("invalid JSON: {e}")}).to_string()
        }
    };
    let n = v.x.len();
    if n != v.y.len() {
        return serde_json::json!({"ok":false,"error":"x and y length mismatch"}).to_string();
    }
    let th = v.threshold;
    if th >= n || th < 3 {
        return serde_json::json!({"ok":true,"x":v.x,"y":v.y}).to_string();
    }
    let bucket_size = (n - 2) as f64 / (th - 2) as f64;
    let mut out_x: Vec<f64> = Vec::with_capacity(th);
    let mut out_y: Vec<f64> = Vec::with_capacity(th);
    out_x.push(v.x[0]);
    out_y.push(v.y[0]);
    let mut a: usize = 0;
    for i in 0..(th - 2) {
        let avg_start = ((i + 1) as f64 * bucket_size).floor() as usize + 1;
        let avg_end = (((i + 2) as f64 * bucket_size).floor() as usize + 1).min(n);
        let avg_len = (avg_end - avg_start).max(1) as f64;
        let mut avg_x = 0.0;
        let mut avg_y = 0.0;
        for k in avg_start..avg_end {
            avg_x += v.x[k];
            avg_y += v.y[k];
        }
        avg_x /= avg_len;
        avg_y /= avg_len;
        let range_offs = (i as f64 * bucket_size).floor() as usize + 1;
        let range_to = ((i + 1) as f64 * bucket_size).floor() as usize + 1;
        let pax = v.x[a];
        let pay = v.y[a];
        let mut max_area = -1.0f64;
        let mut next_a = range_offs;
        for k in range_offs..range_to.min(n) {
            let area =
                ((pax - avg_x) * (v.y[k] - pay) - (pax - v.x[k]) * (avg_y - pay)).abs() * 0.5;
            if area > max_area {
                max_area = area;
                next_a = k;
            }
        }
        out_x.push(v.x[next_a]);
        out_y.push(v.y[next_a]);
        a = next_a;
    }
    out_x.push(v.x[n - 1]);
    out_y.push(v.y[n - 1]);
    serde_json::json!({"ok":true,"x":out_x,"y":out_y,"reduction":format!("{}->{}", n, out_x.len())})
        .to_string()
}

#[crate::sera_register]
pub fn chart_diff(input: &str) -> String {
    #[derive(serde::Deserialize)]
    struct In {
        a: String,
        b: String,
    }
    let v: In = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(_) => return serde_json::json!({"ok":false,"error":"invalid JSON"}).to_string(),
    };
    let svg_a = extract_svg(&v.a);
    let svg_b = extract_svg(&v.b);
    let identical = svg_a == svg_b;
    let len_a = svg_a.len();
    let len_b = svg_b.len();
    let common = svg_a
        .bytes()
        .zip(svg_b.bytes())
        .take_while(|(x, y)| x == y)
        .count();
    let similarity = if len_a == 0 && len_b == 0 {
        1.0
    } else {
        let max_len = len_a.max(len_b) as f64;
        common as f64 / max_len
    };
    serde_json::json!({
        "ok":true,
        "identical":identical,
        "size_a":len_a,
        "size_b":len_b,
        "common_prefix":common,
        "similarity":similarity
    })
    .to_string()
}

fn extract_svg(html: &str) -> String {
    let start = match html.find("<svg") {
        Some(i) => i,
        None => return String::new(),
    };
    let end = match html[start..].find("</svg>") {
        Some(i) => start + i + 6,
        None => return String::new(),
    };
    html[start..end].to_string()
}

#[crate::sera_register]
pub fn drift_ks(input: &str) -> String {
    #[derive(serde::Deserialize)]
    struct In {
        reference: Vec<f64>,
        current: Vec<f64>,
    }
    let v: In = match serde_json::from_str(input) {
        Ok(v) => v,
        Err(e) => {
            return serde_json::json!({"ok":false,"error":format!("invalid JSON: {e}")}).to_string()
        }
    };
    let mut a = v.reference.clone();
    let mut b = v.current.clone();
    if a.is_empty() || b.is_empty() {
        return serde_json::json!({"ok":false,"error":"empty array"}).to_string();
    }
    a.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    b.sort_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal));
    let n = a.len() as f64;
    let m = b.len() as f64;
    let mut i = 0usize;
    let mut j = 0usize;
    let mut max_d = 0.0f64;
    while i < a.len() && j < b.len() {
        let cdf_a = (i + 1) as f64 / n;
        let cdf_b = (j + 1) as f64 / m;
        let d = (cdf_a - cdf_b).abs();
        if d > max_d {
            max_d = d;
        }
        if a[i] <= b[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    let coeff = ((n * m) / (n + m)).sqrt();
    let lambda = (coeff + 0.12 + 0.11 / coeff) * max_d;
    let mut p_value = 0.0f64;
    let mut sign = 1.0f64;
    for k in 1..=100 {
        let term = sign * 2.0 * (-2.0 * lambda * lambda * (k * k) as f64).exp();
        p_value += term;
        sign = -sign;
        if term.abs() < 1e-10 {
            break;
        }
    }
    p_value = p_value.clamp(0.0, 1.0);
    let drift_detected = p_value < 0.05;
    serde_json::json!({
        "ok":true,
        "ks_statistic":max_d,
        "p_value":p_value,
        "drift_detected":drift_detected,
        "n_reference":a.len(),
        "n_current":b.len()
    })
    .to_string()
}

pub(crate) fn bench_chart_value_inner(s: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(s).is_ok()
}

#[crate::sera_register(custom)]
pub fn bench_chart_value(input: &str) -> String {
    bench_chart_value_inner(input).to_string()
}

#[cfg(any(feature = "python", feature = "gui"))]
pub(crate) fn set_chart_kind_raw(kind: u8) {
    crate::viewer::chart::sera_set_current_chart_kind(kind);
}
#[cfg(not(any(feature = "python", feature = "gui")))]
pub(crate) fn set_chart_kind_raw(_kind: u8) {}

#[crate::sera_register(custom)]
pub fn set_chart_kind(input: &str) -> String {
    let kind = serde_json::from_str::<serde_json::Value>(input)
        .ok()
        .and_then(|v| {
            v.get("kind")
                .and_then(|k| k.as_u64())
                .or_else(|| v.as_u64())
        })
        .unwrap_or(0) as u8;
    set_chart_kind_raw(kind);
    "true".to_string()
}

#[cfg(any(feature = "python", feature = "gui"))]
pub(crate) fn set_chart_orientation_raw(vertical: bool) {
    crate::viewer::chart::sera_set_chart_orientation(vertical);
}
#[cfg(not(any(feature = "python", feature = "gui")))]
pub(crate) fn set_chart_orientation_raw(_vertical: bool) {}

#[crate::sera_register(custom)]
pub fn set_chart_orientation(input: &str) -> String {
    let vertical = serde_json::from_str::<serde_json::Value>(input)
        .ok()
        .and_then(|v| {
            v.get("vertical")
                .and_then(|b| b.as_bool())
                .or_else(|| v.as_bool())
        })
        .unwrap_or(false);
    set_chart_orientation_raw(vertical);
    "true".to_string()
}

#[cfg(any(feature = "python", feature = "gui"))]
pub(crate) fn show_chart_value_inner(s: &str) -> bool {
    let c = std::ffi::CString::new(s).unwrap_or_default();
    crate::viewer::chart::sera_show_chart_value(c.as_ptr())
}
#[cfg(not(any(feature = "python", feature = "gui")))]
pub(crate) fn show_chart_value_inner(_s: &str) -> bool {
    false
}

#[crate::sera_register(custom)]
pub fn show_chart_value(input: &str) -> String {
    show_chart_value_inner(input).to_string()
}

pub(crate) fn bench_pure_rust_raw(n: usize) -> (f64, f64, f64, f64) {
    use std::time::Instant;
    let ages: Vec<f64> = (0..891).map(|i| 10.0 + (i % 70) as f64).collect();
    let fare: Vec<f64> = (0..891).map(|i| (i % 50) as f64 * 2.5).collect();
    let ages100: Vec<f64> = ages[..100].to_vec();
    let fare100: Vec<f64> = fare[..100].to_vec();
    let labs: Vec<String> = (0..30).map(|i| format!("Cat {i}")).collect();
    let vals: Vec<f64> = (0..30).map(|i| i as f64 * 3.7).collect();
    let n_lbl = 11usize;
    let corr_labels: Vec<String> = (0..n_lbl).map(|i| format!("F{i}")).collect();
    let flat: Vec<f64> = (0..n_lbl * n_lbl)
        .map(|i| ((i % 11) as f64 - 5.0) * 0.15)
        .collect();
    use crate::plot::statistical::{
        render_heatmap_html, render_histogram_html, HeatmapConfig, HistogramConfig,
    };
    let t0 = Instant::now();
    for _ in 0..n {
        let _ = render_histogram_html(&HistogramConfig {
            title: "B",
            values: &ages,
            bins: 20,
            width: 900,
            height: 400,
            ..HistogramConfig::default()
        });
    }
    let hist_ms = t0.elapsed().as_secs_f64() * 1000.0 / n as f64;
    let t0 = Instant::now();
    for _ in 0..n {
        let _ = crate::plot::default::render_bars_html(
            "B",
            &labs,
            &vals,
            900,
            480,
            &[],
            b'v',
            &[],
            false,
            "",
            "",
            &[],
            0,
            true,
            "",
            "none",
        );
    }
    let bar_ms = t0.elapsed().as_secs_f64() * 1000.0 / n as f64;
    let t0 = Instant::now();
    for _ in 0..n {
        let _ = crate::plot::default::render_scatter_html(
            "B",
            &ages100,
            &fare100,
            &[],
            900,
            540,
            &[],
            &[],
            &[],
            &[],
            "",
            "",
            0,
            true,
            false,
            false,
            "linear",
        );
    }
    let scatter_ms = t0.elapsed().as_secs_f64() * 1000.0 / n as f64;
    let t0 = Instant::now();
    for _ in 0..n {
        let _ = render_heatmap_html(&HeatmapConfig {
            title: "B",
            row_labels: &corr_labels,
            col_labels: &[],
            flat_matrix: &flat,
            width: 800,
            ..HeatmapConfig::default()
        });
    }
    let heatmap_ms = t0.elapsed().as_secs_f64() * 1000.0 / n as f64;
    (hist_ms, bar_ms, scatter_ms, heatmap_ms)
}

#[crate::sera_register(custom)]
pub fn bench_pure_rust(input: &str) -> String {
    let n = serde_json::from_str::<serde_json::Value>(input)
        .ok()
        .and_then(|v| v.get("n").and_then(|c| c.as_u64()).or_else(|| v.as_u64()))
        .unwrap_or(2000) as usize;
    let (hist_ms, bar_ms, scatter_ms, heatmap_ms) = bench_pure_rust_raw(n);
    serde_json::json!({
        "histogram_ms": hist_ms,
        "bar_ms": bar_ms,
        "scatter_ms": scatter_ms,
        "heatmap_ms": heatmap_ms,
    })
    .to_string()
}

#[crate::sera_alias("plan", "cloud_plan")]
#[crate::sera_register]
pub fn scale_plan(input: &str) -> String {
    #[derive(serde::Deserialize, Default)]
    struct In {
        n_rows: Option<usize>,
        n_cols: Option<usize>,
        mem_budget_mb: Option<u64>,
    }
    let v: In = serde_json::from_str(input).unwrap_or_default();
    let p = crate::cloud::planner::plan(
        v.n_rows.unwrap_or(0),
        v.n_cols.unwrap_or(0),
        v.mem_budget_mb.unwrap_or(512),
    );
    crate::cloud::planner::to_json(&p)
}

#[crate::sera_alias("profile", "cloud_profile", "system_info")]
#[crate::sera_register]
pub fn system_profile(_input: &str) -> String {
    let r = crate::cloud::profile::current();
    crate::cloud::profile::to_json(&r)
}

#[crate::sera_alias("count_rows", "csv_rows")]
#[crate::sera_register]
pub fn csv_count_rows(input: &str) -> String {
    #[derive(serde::Deserialize, Default)]
    struct In {
        path: String,
        has_header: Option<bool>,
    }
    let v: In = serde_json::from_str(input).unwrap_or_default();
    if v.path.is_empty() {
        return "{\"error\":\"path required\"}".to_string();
    }
    match crate::cloud::chunker::count_rows(&v.path, v.has_header.unwrap_or(true)) {
        Ok(n) => serde_json::json!({"rows": n}).to_string(),
        Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
    }
}

#[crate::sera_alias("read_chunk", "csv_chunk")]
#[crate::sera_register]
pub fn csv_chunk_read(input: &str) -> String {
    #[derive(serde::Deserialize, Default)]
    struct In {
        path: String,
        offset_rows: Option<usize>,
        chunk_rows: Option<usize>,
        has_header: Option<bool>,
        delimiter: Option<u8>,
    }
    let v: In = serde_json::from_str(input).unwrap_or_default();
    if v.path.is_empty() {
        return "{\"error\":\"path required\"}".to_string();
    }
    let delim = v.delimiter.unwrap_or(b',');
    let chunk = v.chunk_rows.unwrap_or(1000).max(1);
    let offset = v.offset_rows.unwrap_or(0);
    let has_header = v.has_header.unwrap_or(true);
    let mut reader = match crate::cloud::chunker::CsvChunkReader::open(
        &v.path,
        chunk + offset,
        has_header,
        delim,
    ) {
        Ok(r) => r,
        Err(e) => return serde_json::json!({"error": e.to_string()}).to_string(),
    };
    let header = reader.header.clone();
    let all = match reader.next_chunk() {
        Ok(Some(rows)) => rows,
        Ok(None) => vec![],
        Err(e) => return serde_json::json!({"error": e.to_string()}).to_string(),
    };
    let n = all.len();
    let data: Vec<Vec<f64>> = if offset < n {
        all.into_iter().skip(offset).collect()
    } else {
        vec![]
    };
    let rows = data.len();
    serde_json::json!({"header": header, "data": data, "rows": rows}).to_string()
}
