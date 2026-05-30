use pyo3::prelude::*;
use pyo3::types::PyAny;

use crate::bindings::registry_macro::{for_each_auto_util_fn, for_each_json_chart_fn, for_each_ml_oneshot_fn};

pub fn python_chart_from_html(html: String, doc: &'static str) -> crate::Chart {
    crate::Chart::new_doc(html, doc)
}

pub fn python_chart_from_payload(
    builder: fn(&str) -> String,
    payload: String,
    doc: &'static str,
) -> crate::Chart {
    python_chart_from_html(builder(&payload), doc)
}

pub fn python_chart_refs_to_html(charts: &[pyo3::PyRef<'_, crate::Chart>]) -> Vec<String> {
    charts.iter().map(|c| c.html.clone()).collect()
}

pub fn python_extract_chart_html(html: &pyo3::Bound<'_, pyo3::types::PyAny>) -> pyo3::PyResult<String> {
    if let Ok(chart) = html.extract::<pyo3::PyRef<'_, crate::Chart>>() {
        Ok(chart.html.clone())
    } else {
        html.extract::<String>()
    }
}

pub fn python_py_any_to_json(val: &pyo3::Bound<'_, pyo3::types::PyAny>) -> serde_json::Value {
    if val.is_none() { return serde_json::Value::Null; }
    if let Ok(b) = val.extract::<bool>() { return serde_json::Value::Bool(b); }
    if let Ok(i) = val.extract::<i64>() { return serde_json::json!(i); }
    if let Ok(f) = val.extract::<f64>() { return serde_json::json!(f); }
    if let Ok(s) = val.extract::<String>() { return serde_json::Value::String(s); }
    if let Ok(list) = val.extract::<Vec<Vec<f64>>>() { return serde_json::json!(list); }
    if let Ok(list) = val.extract::<Vec<i64>>() { return serde_json::json!(list); }
    if let Ok(list) = val.extract::<Vec<f64>>() { return serde_json::json!(list); }
    if let Ok(list) = val.extract::<Vec<String>>() { return serde_json::json!(list); }
    let py = val.py();
    if let Ok(json_mod) = py.import_bound("json") {
        if let Ok(s) = json_mod.call_method1("dumps", (val,)) {
            if let Ok(ss) = s.extract::<String>() {
                if let Ok(jv) = serde_json::from_str::<serde_json::Value>(&ss) {
                    return jv;
                }
            }
        }
    }
    serde_json::Value::Null
}

pub fn python_py_args_to_json(
    title: &str,
    labels: Option<&pyo3::Bound<'_, pyo3::types::PyAny>>,
    values: Option<&pyo3::Bound<'_, pyo3::types::PyAny>>,
    theme: Option<&str>,
    kwargs: Option<&pyo3::Bound<'_, pyo3::types::PyDict>>,
) -> String {
    let mut m = serde_json::Map::new();
    if !title.is_empty() { m.insert("title".into(), serde_json::json!(title)); }
    if let Some(v) = labels {
        let jv = python_py_any_to_json(v);
        if jv != serde_json::Value::Null { m.insert("labels".into(), jv); }
    }
    if let Some(v) = values {
        if let Ok(vv) = v.extract::<Vec<Vec<f64>>>() {
            m.insert("series".into(), serde_json::json!(vv));
        } else {
            let jv = python_py_any_to_json(v);
            if jv != serde_json::Value::Null { m.insert("values".into(), jv); }
        }
    }
    if let Some(t) = theme {
        m.insert("theme".into(), serde_json::json!(t));
    }
    if let Some(d) = kwargs {
        for (key, val) in d.iter() {
            if let Ok(ks) = key.str() {
                let ks = ks.to_string_lossy().to_string();
                let v = python_py_any_to_json(&val);
                if v != serde_json::Value::Null { m.insert(ks, v); }
            }
        }
    }
    serde_json::Value::Object(m).to_string()
}

macro_rules! impl_python_json_chart_binding {
    ($fn:ident, $_js:literal) => {
        #[crate::sera_binding(python, py_json, chart)]
        fn $fn(input: &str) -> String {
            crate::bindings::commands::charts::$fn(input)
        }
    };
}
for_each_json_chart_fn!(impl_python_json_chart_binding);

#[allow(unused_macros)]
macro_rules! impl_python_json_ml {
    ($fn:ident, $_js:literal) => {
        #[pyfunction]
        pub fn $fn(input: &str) -> String {
            crate::bindings::commands::ml::$fn(input)
        }
    };
}
for_each_ml_oneshot_fn!(impl_python_json_ml);

macro_rules! impl_python_json_util {
    ($fn:ident, $_js:literal) => {
        #[pyfunction]
        pub fn $fn(input: &str) -> String {
            crate::bindings::commands::charts::$fn(input)
        }
    };
}
for_each_auto_util_fn!(impl_python_json_util);

#[crate::sera_sig(html, color=None)]
#[crate::sera_binding(python, py_chart, chart)]
fn set_bg(html: &str, color: Option<&str>) -> String {
    crate::html::hover::apply_bg(html.to_string(), color)
}

#[crate::sera_binding(python, py_chart)]
fn show_chart_value(chart_json: &str) -> bool {
    crate::bindings::commands::charts::show_chart_value(chart_json)
}

#[crate::sera_binding(python, py_chart)]
fn bench_chart_value(chart_json: &str) -> bool {
    crate::bindings::commands::charts::bench_chart_value(chart_json)
}

#[crate::sera_binding(python, py_true)]
fn set_chart_kind(kind: u8) {
    crate::bindings::commands::charts::set_chart_kind(kind)
}

#[crate::sera_binding(python, py_true)]
fn set_chart_orientation(vertical: bool) {
    crate::bindings::commands::charts::set_chart_orientation(vertical)
}

#[crate::sera_sig(n=2000)]
#[crate::sera_binding(python)]
fn bench_pure_rust(n: usize) -> (f64, f64, f64, f64) {
    crate::bindings::commands::charts::bench_pure_rust(n)
}

#[crate::sera_doc(category = "layout", file = "charts/layout.md", en = "Arranges multiple Chart objects in a responsive grid layout and returns a single Chart.", fr = "Organise plusieurs objets Chart dans une mise en page grille responsive et retourne un seul Chart.", param(name = "charts", ty = "list[Chart]", en = "List of Chart objects to arrange in the grid.", fr = "Liste d'objets Chart a organiser dans la grille."), param(name = "cols", ty = "int", en = "Number of columns in the grid. Default: 3.", fr = "Nombre de colonnes dans la grille. Defaut: 3."), param(name = "gap", ty = "int", en = "Gap between cells in pixels. Default: 16.", fr = "Espacement entre les cellules en pixels. Defaut: 16."), param(name = "bg_color", ty = "str", en = "Background color for the grid container. Default: '#0a0f1c'.", fr = "Couleur d'arriere-plan du conteneur de grille. Defaut: '#0a0f1c'."), param(name = "title", ty = "str", en = "Optional title displayed above the grid.", fr = "Titre optionnel affiche au-dessus de la grille."), param(name = "cell_height", ty = "int | None", en = "Fixed height for each cell in pixels. Auto-detected if None.", fr = "Hauteur fixe de chaque cellule en pixels. Detection automatique si None."))]
#[pyfunction]
#[pyo3(signature = (charts, cols=3, gap=16, bg_color="#0a0f1c", title="", cell_height=None))]
pub fn build_grid(
    py: Python<'_>,
    charts: Vec<PyObject>,
    cols: usize,
    gap: i32,
    bg_color: &str,
    title: &str,
    cell_height: Option<i32>,
) -> PyResult<crate::Chart> {
    let cols = if cols < 1 { 1 } else { cols };
    let mut cells = String::new();
    for obj in &charts {
        let chart_ref: PyRef<crate::Chart> = obj.extract(py)?;
        let h = if let Some(ch) = cell_height {
            ch as u32
        } else {
            let mut found = 480u32;
            let s = &chart_ref.html;
            let mut start = 0usize;
            loop {
                match s[start..].find("height=\"") {
                    None => break,
                    Some(rel) => {
                        let abs = start + rel + 8;
                        if let Some(end) = s[abs..].find('"') {
                            if let Ok(v) = s[abs..abs + end].parse::<u32>() {
                                if v >= 150 && v <= 1600 {
                                    found = v;
                                    break;
                                }
                            }
                        }
                        start += rel + 1;
                    }
                }
            }
            found
        };
        let esc = chart_ref.html.replace('&', "&amp;").replace('"', "&quot;");
        cells.push_str(&format!(
            r#"<div class="sp-gc"><iframe srcdoc="{esc}" style="width:100%;height:{h}px;border:none;display:block" frameborder="0" loading="lazy"></iframe></div>"#
        ));
    }
    let title_html = if title.is_empty() {
        String::new()
    } else {
        let t = title.replace('<', "&lt;").replace('>', "&gt;");
        format!(r#"<h1 class="sp-gtitle">{t}</h1>"#)
    };
    let html = format!(
        r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><style>*{{box-sizing:border-box}}html,body{{margin:0;padding:0;background:{bg_color};font-family:system-ui,-apple-system,Arial,sans-serif}}body{{padding:16px}}.sp-gtitle{{color:#e2e8f0;font-size:22px;font-weight:700;text-align:center;margin:0 0 16px;letter-spacing:-.01em;border:none}}.sp-grid{{display:grid;grid-template-columns:repeat({cols},1fr);gap:{gap}px}}.sp-gc{{border-radius:10px;overflow:hidden;background:#0d1117;box-shadow:0 4px 20px rgba(0,0,0,.5)}}</style></head><body>{title_html}<div class="sp-grid">{cells}</div></body></html>"#
    );
    Ok(crate::Chart::new(html))
}

#[crate::sera_doc(category = "layout", file = "charts/layout.md", en = "Alias for build_grid(). Arranges multiple Chart objects in a responsive grid.", fr = "Alias de build_grid(). Organise plusieurs objets Chart dans une grille responsive.", param(name = "charts", ty = "list[Chart]", en = "List of Chart objects to arrange.", fr = "Liste d'objets Chart a organiser."), param(name = "cols", ty = "int", en = "Number of grid columns. Default: 3.", fr = "Nombre de colonnes de la grille. Defaut: 3."), param(name = "gap", ty = "int", en = "Gap in pixels. Default: 16.", fr = "Espacement en pixels. Defaut: 16."), param(name = "bg_color", ty = "str", en = "Grid background color. Default: '#0a0f1c'.", fr = "Couleur d'arriere-plan de la grille. Defaut: '#0a0f1c'."), param(name = "title", ty = "str", en = "Optional grid title.", fr = "Titre optionnel de la grille."), param(name = "cell_height", ty = "int | None", en = "Fixed cell height in pixels or None for auto.", fr = "Hauteur fixe des cellules en pixels ou None pour automatique."))]
#[pyfunction]
#[pyo3(signature = (charts, cols=3, gap=16, bg_color="#0a0f1c", title="", cell_height=None))]
pub fn grid(
    py: Python<'_>,
    charts: Vec<PyObject>,
    cols: usize,
    gap: i32,
    bg_color: &str,
    title: &str,
    cell_height: Option<i32>,
) -> PyResult<crate::Chart> {
    build_grid(py, charts, cols, gap, bg_color, title, cell_height)
}

#[pyfunction]
#[pyo3(signature = (charts, interval_ms=2500, title="", width=900, height=520))]
pub fn build_slideshow(
    charts: Vec<pyo3::PyRef<crate::Chart>>,
    interval_ms: u32,
    title: &str,
    width: i32,
    height: i32,
) -> crate::Chart {
    let html_strs = python_chart_refs_to_html(&charts);
    python_chart_from_payload(
        crate::bindings::commands::charts::build_slideshow,
        serde_json::json!({"charts": html_strs, "interval_ms": interval_ms, "title": title, "width": width, "height": height}).to_string(),
        crate::bindings::commands::docs::DOC_BUILD_SLIDESHOW,
    )
}

#[crate::sera_doc(category = "layout", file = "charts/layout.md", en = "Builds a live system monitor dashboard showing CPU, memory, disk usage, and system info.", fr = "Construit un tableau de bord de surveillance systeme en direct montrant l'utilisation du CPU, de la memoire, des disques et les informations systeme.", param(name = "bg_color", ty = "str", en = "Dashboard background color. Default: '#0a0f1c'.", fr = "Couleur d'arriere-plan du tableau de bord. Defaut: '#0a0f1c'."), param(name = "update_interval_ms", ty = "int", en = "Simulated refresh interval in milliseconds. Default: 2000.", fr = "Intervalle de rafraichissement simule en millisecondes. Defaut: 2000."))]
#[pyfunction]
#[pyo3(signature = (bg_color="#0a0f1c", update_interval_ms=2000u32))]
pub fn build_sysmon(bg_color: &str, update_interval_ms: u32) -> crate::Chart {
    use sysinfo::{Disks, System};
    let mut sys = System::new_all();
    sys.refresh_all();
    std::thread::sleep(std::time::Duration::from_millis(120));
    sys.refresh_all();

    let cpu_pct = sys.global_cpu_info().cpu_usage();
    let cpu_count = sys.cpus().len();
    let core_usage: Vec<f32> = sys.cpus().iter().map(|c| c.cpu_usage()).collect();
    let total_mem = sys.total_memory();
    let used_mem = sys.used_memory();
    let mem_pct = if total_mem > 0 {
        used_mem as f64 / total_mem as f64 * 100.0
    } else {
        0.0
    };
    let procs = sys.processes().len();
    let uptime_s = System::uptime();
    let hostname = System::host_name().unwrap_or_else(|| String::from("localhost"));
    let os_name = System::long_os_version()
        .or_else(|| System::os_version())
        .unwrap_or_else(|| String::from("Unknown OS"));

    let disks = Disks::new_with_refreshed_list();
    let disk_parts: Vec<String> = disks
        .list()
        .iter()
        .take(6)
        .filter_map(|d| {
            let total = d.total_space();
            if total == 0 {
                return None;
            }
            let name = d.name().to_string_lossy().to_string();
            let name = if name.is_empty() {
                String::from("Disk")
            } else {
                name
            };
            let avail = d.available_space();
            let used = total.saturating_sub(avail);
            let pct = (used as f64 / total as f64 * 100.0) as u32;
            let name_esc = name.replace('"', "\\\"");
            Some(format!(
                "{{\"name\":\"{name_esc}\",\"used\":{used},\"total\":{total},\"pct\":{pct}}}"
            ))
        })
        .collect();

    let core_parts: Vec<String> = core_usage
        .iter()
        .take(8)
        .map(|v| format!("{:.1}", v))
        .collect();

    let data_json = format!(
        "{{\"cpu_pct\":{cpu_pct:.1},\"cpu_count\":{cpu_count},\"cpu_cores\":[{cores}],\"mem_pct\":{mem_pct:.1},\"used_mem\":{used_mem},\"total_mem\":{total_mem},\"disks\":[{disks}],\"processes\":{procs},\"uptime_s\":{uptime_s}}}",
        cores = core_parts.join(","),
        disks = disk_parts.join(","),
    );

    let host_display: String = hostname.chars().take(28).collect();
    let os_display: String = os_name.chars().take(44).collect();

    let css = format!(
        "*{{box-sizing:border-box;margin:0;padding:0}}body{{background:{bg_color};color:#e2e8f0;font-family:system-ui,-apple-system,Arial,sans-serif;padding:20px;min-height:100vh}}.sm-header{{text-align:center;margin-bottom:24px}}.sm-title{{font-size:22px;font-weight:800;color:#f1f5f9;letter-spacing:-.02em}}.sm-sub{{font-size:13px;color:#475569;margin-top:5px}}.sm-grid{{display:grid;grid-template-columns:repeat(auto-fit,minmax(280px,1fr));gap:16px;margin-bottom:16px}}.sm-card{{background:#0d1117;border:1px solid #1e293b;border-radius:12px;padding:20px}}.sm-card-title{{font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.08em;color:#475569;margin-bottom:14px}}.sm-value{{font-size:32px;font-weight:800;line-height:1}}.sm-unit{{font-size:13px;font-weight:400;color:#94a3b8;margin-left:4px}}.sm-bar-bg{{width:100%;height:8px;background:#1e293b;border-radius:4px;margin-top:12px;overflow:hidden}}.sm-bar-fill{{height:100%;border-radius:4px;transition:width .6s ease}}.sm-disk-row{{display:flex;align-items:center;gap:10px;margin-bottom:8px;font-size:12px}}.sm-disk-name{{color:#94a3b8;width:80px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;flex-shrink:0}}.sm-disk-bar-bg{{flex:1;height:6px;background:#1e293b;border-radius:3px;overflow:hidden}}.sm-disk-bar{{height:100%;border-radius:3px;transition:width .6s ease}}.sm-disk-pct{{color:#e2e8f0;width:36px;text-align:right;flex-shrink:0}}.sm-stat-row{{display:flex;gap:12px;margin-top:12px}}.sm-stat{{flex:1;background:#0a0f1c;border-radius:8px;padding:10px;text-align:center}}.sm-stat-val{{font-size:18px;font-weight:700;color:#f1f5f9}}.sm-stat-lbl{{font-size:10px;color:#475569;text-transform:uppercase;letter-spacing:.06em;margin-top:2px}}.sm-gauge-wrap{{display:flex;align-items:center;justify-content:center}}.sm-ts{{font-size:10px;color:#334155;text-align:center;margin-top:12px}}"
    );

    let js_tpl = r##"const DATA=__DATA__;
let T=JSON.parse(JSON.stringify(DATA));
function clr(p){return p<50?'#22c55e':p<75?'#f59e0b':'#ef4444'}
function fmt(b){var u=['B','KB','MB','GB','TB'],i=0,v=b;while(v>=1024&&i<4){v/=1024;i++}return v.toFixed(i>0?1:0)+' '+u[i]}
function gauge(p,c,sz){var r=sz/2-10,cx=sz/2,cy=sz/2,circ=2*Math.PI*r,d=p/100*circ*.75;return'<svg width="'+sz+'" height="'+sz+'" viewBox="0 0 '+sz+' '+sz+'"><circle cx="'+cx+'" cy="'+cy+'" r="'+r+'" fill="none" stroke="#1e293b" stroke-width="14" stroke-dasharray="'+(circ*.75)+' '+(circ*.25)+'" stroke-linecap="round" transform="rotate(-225 '+cx+' '+cy+')"/><circle cx="'+cx+'" cy="'+cy+'" r="'+r+'" fill="none" stroke="'+c+'" stroke-width="14" stroke-dasharray="'+d+' '+(circ-d)+'" stroke-linecap="round" transform="rotate(-225 '+cx+' '+cy+')" style="transition:stroke-dasharray .6s ease"/><text x="'+cx+'" y="'+(cy+7)+'" text-anchor="middle" fill="#f1f5f9" font-size="22" font-weight="800" font-family="system-ui">'+Math.round(p)+'%</text></svg>'}
function render(d){
  var g=document.getElementById('sm-grid'),h='',cc=clr(d.cpu_pct);
  h+='<div class="sm-card"><div class="sm-card-title">CPU</div><div class="sm-gauge-wrap">'+gauge(d.cpu_pct,cc,150)+'</div><div class="sm-stat-row">';
  d.cpu_cores.slice(0,4).forEach(function(v,i){h+='<div class="sm-stat"><div class="sm-stat-val" style="color:'+clr(v)+'">'+Math.round(v)+'%</div><div class="sm-stat-lbl">Core '+(i+1)+'</div></div>'});
  h+='</div></div>';
  var mc=clr(d.mem_pct);
  h+='<div class="sm-card"><div class="sm-card-title">Memory</div><div class="sm-value" style="color:'+mc+'">'+Math.round(d.mem_pct)+'<span class="sm-unit">%</span></div><div class="sm-bar-bg"><div class="sm-bar-fill" style="width:'+d.mem_pct+'%;background:'+mc+'"></div></div><div class="sm-stat-row"><div class="sm-stat"><div class="sm-stat-val">'+fmt(d.used_mem)+'</div><div class="sm-stat-lbl">Used</div></div><div class="sm-stat"><div class="sm-stat-val">'+fmt(d.total_mem)+'</div><div class="sm-stat-lbl">Total</div></div></div></div>';
  h+='<div class="sm-card"><div class="sm-card-title">Disks</div>';
  d.disks.forEach(function(dk){var c=clr(dk.pct);h+='<div class="sm-disk-row"><div class="sm-disk-name">'+dk.name+'</div><div class="sm-disk-bar-bg"><div class="sm-disk-bar" style="width:'+dk.pct+'%;background:'+c+'"></div></div><div class="sm-disk-pct" style="color:'+c+'">'+dk.pct+'%</div></div>'});
  h+='</div>';
  var up=d.uptime_s?Math.floor(d.uptime_s/3600)+'h '+Math.floor(d.uptime_s%3600/60)+'m':'N/A';
  h+='<div class="sm-card"><div class="sm-card-title">System</div><div class="sm-stat-row"><div class="sm-stat"><div class="sm-stat-val">'+d.cpu_count+'</div><div class="sm-stat-lbl">Cores</div></div><div class="sm-stat"><div class="sm-stat-val">'+d.processes+'</div><div class="sm-stat-lbl">Processes</div></div></div><div class="sm-stat-row" style="margin-top:8px"><div class="sm-stat"><div class="sm-stat-val">'+fmt(d.total_mem)+'</div><div class="sm-stat-lbl">Total RAM</div></div><div class="sm-stat"><div class="sm-stat-val">'+up+'</div><div class="sm-stat-lbl">Uptime</div></div></div></div>';
  g.innerHTML=h}
render(T);
setInterval(function(){T=Object.assign({},T,{cpu_pct:Math.min(100,Math.max(0,T.cpu_pct+(Math.random()-.5)*4)),mem_pct:Math.min(100,Math.max(0,T.mem_pct+(Math.random()-.5)*.8)),cpu_cores:T.cpu_cores.map(function(v){return Math.min(100,Math.max(0,v+(Math.random()-.5)*6))})});render(T);document.getElementById('sm-ts').textContent='Updated: '+new Date().toLocaleTimeString()},__INTERVAL__);"##;

    let js = js_tpl
        .replace("__DATA__", &data_json)
        .replace("__INTERVAL__", &update_interval_ms.to_string());

    let html = format!(
        r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>SeraPlot Sysmon</title><style>{css}</style></head><body><div class="sm-header"><div class="sm-title">System Monitor</div><div class="sm-sub">{host_display} &middot; {os_display} &middot; {cpu_count} cores</div></div><div class="sm-grid" id="sm-grid"></div><div class="sm-ts" id="sm-ts">Snapshot taken</div><script>{js}</script></body></html>"#
    );

    crate::Chart::new(html)
}

#[crate::sera_doc(category = "layout", file = "charts/layout.md", en = "Alias for build_sysmon(). Returns a live system monitor dashboard as a Chart.", fr = "Alias de build_sysmon(). Retourne un tableau de bord de surveillance systeme en direct sous forme de Chart.", param(name = "bg_color", ty = "str", en = "Background color. Default: '#0a0f1c'.", fr = "Couleur d'arriere-plan. Defaut: '#0a0f1c'."), param(name = "update_interval_ms", ty = "int", en = "Refresh interval in milliseconds. Default: 2000.", fr = "Intervalle de rafraichissement en millisecondes. Defaut: 2000."))]
#[pyfunction]
#[pyo3(signature = (bg_color="#0a0f1c", update_interval_ms=2000u32))]
pub fn sysmon(bg_color: &str, update_interval_ms: u32) -> crate::Chart {
    build_sysmon(bg_color, update_interval_ms)
}

#[pyfunction]
#[pyo3(signature = (labels, images=None, descriptions=None))]
pub fn build_hover_json(
    labels: Vec<String>,
    images: Option<Vec<Option<String>>>,
    descriptions: Option<Vec<Vec<(String, String)>>>,
) -> String {
    let desc_json: Option<Vec<Vec<Vec<String>>>> = descriptions.map(|outer|
        outer.into_iter().map(|row| row.into_iter().map(|(k, v)| vec![k, v]).collect()).collect()
    );
    crate::bindings::commands::charts::build_hover_json(
        &serde_json::json!({"labels": labels, "images": images, "descriptions": desc_json}).to_string()
    )
}

#[pyfunction]
#[pyo3(name = "plot", signature = (x, y=None, *, title="", kind="line", color_hex=0x6366F1_u32, width=900_i32, height=480_i32, x_label="", y_label="", gridlines=false, palette=None, background=None, show_points=true))]
pub fn plot_chart(
    py: Python<'_>,
    x: &Bound<'_, PyAny>,
    y: Option<&Bound<'_, PyAny>>,
    title: &str,
    kind: &str,
    color_hex: u32,
    width: i32,
    height: i32,
    x_label: &str,
    y_label: &str,
    gridlines: bool,
    palette: Option<Vec<u32>>,
    background: Option<&str>,
    show_points: bool,
) -> PyResult<crate::Chart> {
    let t = std::time::Instant::now();
    let (bg, pal, grid) = crate::merge_global_opts(background, palette, gridlines);
    let pal_opt: Option<Vec<u32>> = if pal.is_empty() { None } else { Some(pal) };
    if let Ok(cols_obj) = x.getattr("columns") {
        let cols: Vec<String> = cols_obj.extract()?;
        if cols.len() >= 2 {
            let x_col = x.get_item(cols[0].as_str())?;
            let y_col = x.get_item(cols[1].as_str())?;
            let xv = crate::extract_f64_vec(py, &x_col)?;
            let yv = crate::extract_f64_vec(py, &y_col)?;
            let auto_title = if title.is_empty() { format!("{} vs {}", cols[0], cols[1]) } else { title.to_string() };
            let result = Ok(python_chart_from_payload(
                crate::bindings::commands::charts::plot_chart,
                serde_json::json!({"x":xv,"y":yv,"kind":"scatter","title":auto_title,"color_hex":color_hex,"width":width,"height":height,"x_label":x_label,"y_label":y_label,"gridlines":grid,"palette":pal_opt,"background":bg}).to_string(),
                "",
            ));
            crate::telemetry::record(crate::telemetry::TelemetryEvent::new("plot", t.elapsed().as_secs_f64() * 1000.0));
            return result;
        }
    }
    let xv = crate::extract_f64_vec(py, x)?;
    let yv: Option<Vec<f64>> = if let Some(yobj) = y { Some(crate::extract_f64_vec(py, yobj)?) } else { None };
    let res = Ok(python_chart_from_payload(
        crate::bindings::commands::charts::plot_chart,
        serde_json::json!({"x":xv,"y":yv,"kind":kind,"title":title,"color_hex":color_hex,"show_points":show_points,"width":width,"height":height,"x_label":x_label,"y_label":y_label,"gridlines":grid,"palette":pal_opt,"background":bg}).to_string(),
        "",
    ));
    crate::telemetry::record(crate::telemetry::TelemetryEvent::new("plot", t.elapsed().as_secs_f64() * 1000.0).with_data(xv.len() as u64, 0.0));
    res
}