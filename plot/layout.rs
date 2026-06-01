pub fn build_grid_impl(
    html_parts: Vec<String>,
    cols: usize,
    gap: i32,
    bg_color: &str,
    title: &str,
    cell_height: Option<i32>,
) -> String {
    let cols = if cols < 1 { 1 } else { cols };
    let mut cells = String::new();
    for html in &html_parts {
        let h = if let Some(ch) = cell_height {
            ch as u32
        } else {
            let mut found = 480u32;
            let mut start = 0usize;
            loop {
                match html[start..].find("height=\"") {
                    None => break,
                    Some(rel) => {
                        let abs = start + rel + 8;
                        if let Some(end) = html[abs..].find('"') {
                            if let Ok(v) = html[abs..abs + end].parse::<u32>() {
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
        let esc = html.replace('&', "&amp;").replace('"', "&quot;");
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
    format!(
        r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><style>*{{box-sizing:border-box}}html,body{{margin:0;padding:0;background:{bg_color};font-family:system-ui,-apple-system,Arial,sans-serif}}body{{padding:16px}}.sp-gtitle{{color:#e2e8f0;font-size:22px;font-weight:700;text-align:center;margin:0 0 16px;letter-spacing:-.01em;border:none}}.sp-grid{{display:grid;grid-template-columns:repeat({cols},1fr);gap:{gap}px}}.sp-gc{{border-radius:10px;overflow:hidden;background:#0d1117;box-shadow:0 4px 20px rgba(0,0,0,.5)}}</style></head><body>{title_html}<div class="sp-grid">{cells}</div></body></html>"#
    )
}

#[cfg(not(target_arch = "wasm32"))]
pub fn build_sysmon_html(bg_color: &str, update_interval_ms: u32) -> String {
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
            if total == 0 { return None; }
            let name = d.name().to_string_lossy().to_string();
            let name = if name.is_empty() { String::from("Disk") } else { name };
            let avail = d.available_space();
            let used = total.saturating_sub(avail);
            let pct = (used as f64 / total as f64 * 100.0) as u32;
            let name_esc = name.replace('"', "\\\"");
            Some(format!("{{\"name\":\"{name_esc}\",\"used\":{used},\"total\":{total},\"pct\":{pct}}}"))
        })
        .collect();

    let core_parts: Vec<String> = core_usage.iter().take(8).map(|v| format!("{:.1}", v)).collect();

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

    format!(
        r#"<!DOCTYPE html><html><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>SeraPlot Sysmon</title><style>{css}</style></head><body><div class="sm-header"><div class="sm-title">System Monitor</div><div class="sm-sub">{host_display} &middot; {os_display} &middot; {cpu_count} cores</div></div><div class="sm-grid" id="sm-grid"></div><div class="sm-ts" id="sm-ts">Snapshot taken</div><script>{js}</script></body></html>"#
    )
}

pub fn build_sysmon(input: &str) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = input;
        return "<!DOCTYPE html><html><body style=\"margin:0;display:flex;align-items:center;justify-content:center;min-height:100vh;background:#0a0f1c;color:#e2e8f0;font-family:system-ui\">System monitor is unavailable on wasm targets.</body></html>".to_string();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        #[derive(serde::Deserialize, Default)]
        struct In { bg_color: Option<String>, update_interval_ms: Option<u32> }
        let i: In = serde_json::from_str(input).unwrap_or_default();
        build_sysmon_html(
            i.bg_color.as_deref().unwrap_or("#0a0f1c"),
            i.update_interval_ms.unwrap_or(2000),
        )
    }
}

pub fn sysmon(input: &str) -> String {
    build_sysmon(input)
}
