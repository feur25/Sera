use crate::Chart;
use pyo3::prelude::*;
use seraplot_macros::sera_doc;

const FRAME_W: i32 = 640;
const FRAME_H: i32 = 460;
const FRAME_GAP: i32 = 56;

fn escape_attr(html: &str) -> String {
    html.replace('&', "&amp;").replace('"', "&quot;")
}

fn escape_html_text(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

fn grid_layout(n: usize) -> Vec<(i32, i32)> {
    if n == 0 {
        return Vec::new();
    }
    let cols = (n as f64).sqrt().ceil() as i32;
    (0..n as i32)
        .map(|i| {
            let col = i % cols;
            let row = i / cols;
            (col * (FRAME_W + FRAME_GAP), row * (FRAME_H + FRAME_GAP))
        })
        .collect()
}

pub(crate) fn build_board_html(entries: &[(String, String)], live_urls: &[Option<String>]) -> String {
    let positions = grid_layout(entries.len());
    let mut sidebar_items = String::new();
    let mut frames = String::new();
    for (i, ((title, html), (x, y))) in entries.iter().zip(positions.iter()).enumerate() {
        let safe_title = escape_html_text(if title.is_empty() { "Untitled" } else { title });
        let live_url = live_urls.get(i).and_then(|u| u.as_ref());
        let live_dot = if live_url.is_some() { "<span class=\"sp-board-live-dot\" title=\"Live\"></span>" } else { "" };
        sidebar_items.push_str(&format!(
            "<div class=\"sp-board-item\" data-target=\"{i}\"><span class=\"sp-board-item-dot\"></span>{safe_title}{live_dot}</div>"
        ));
        let body = match live_url {
            Some(url) => format!("<iframe class=\"sp-board-frame-body\" src=\"{}\"></iframe>", escape_attr(url)),
            None => format!("<iframe class=\"sp-board-frame-body\" srcdoc=\"{}\"></iframe>", escape_attr(html)),
        };
        frames.push_str(&format!(
            "<div class=\"sp-board-frame\" id=\"sp-board-frame-{i}\" data-x=\"{x}\" data-y=\"{y}\" style=\"left:{x}px;top:{y}px;width:{FRAME_W}px;height:{FRAME_H}px\">\
             <div class=\"sp-board-frame-bar\">{live_dot}{safe_title}</div>\
             {body}\
             </div>"
        ));
    }

    format!(
        "<!doctype html><html><head><meta charset=\"utf-8\"><title>Sera Board</title><style>\
         *{{box-sizing:border-box}}body{{margin:0;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Arial,sans-serif;overflow:hidden;background:var(--sp-bg,#f1f5f9);transition:background .15s}}\
         #sp-board-viewport{{position:fixed;inset:0;left:230px;overflow:hidden;cursor:grab}}\
         #sp-board-viewport.panning{{cursor:grabbing}}\
         #sp-board-world{{position:absolute;left:0;top:0;transform-origin:0 0}}\
         .sp-board-frame{{position:absolute;background:var(--sp-frame-bg,#fff);border-radius:10px;box-shadow:0 2px 14px rgba(0,0,0,.14);overflow:hidden;border:1px solid var(--sp-frame-border,#e2e8f0);transition:background .15s,border-color .15s}}\
         .sp-board-frame.sp-board-selected{{outline:2px solid #3b82f6;outline-offset:2px}}\
         .sp-board-frame-bar{{height:32px;line-height:32px;padding:0 12px;font-size:12px;font-weight:600;color:var(--sp-frame-bar-fg,#334155);background:var(--sp-frame-bar-bg,#f8fafc);border-bottom:1px solid var(--sp-frame-border,#e2e8f0);cursor:move;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;user-select:none;display:flex;align-items:center;gap:6px}}\
         .sp-board-live-dot{{width:7px;height:7px;border-radius:50%;background:#22c55e;flex:none;box-shadow:0 0 0 2px rgba(34,197,94,.25);animation:sp-board-pulse 1.6s ease-in-out infinite}}\
         @keyframes sp-board-pulse{{0%,100%{{opacity:1}}50%{{opacity:.35}}}}\
         .sp-board-frame-body{{width:100%;height:calc(100% - 32px);border:0;pointer-events:none}}\
         .sp-board-frame.sp-board-active .sp-board-frame-body{{pointer-events:auto}}\
         #sp-board-sidebar{{position:fixed;left:0;top:0;bottom:0;width:230px;background:linear-gradient(180deg,var(--sp-sidebar-bg,#0b0e18),var(--sp-sidebar-bg2,#12172a));color:var(--sp-sidebar-fg,#e2e8f0);overflow-y:auto;padding:18px 10px;z-index:5;user-select:none;box-shadow:2px 0 12px rgba(0,0,0,.25)}}\
         #sp-board-sidebar h1{{font-size:12px;font-weight:800;letter-spacing:.06em;text-transform:uppercase;padding:0 10px 14px;margin:0;color:var(--sp-sidebar-muted,#93a4c3);display:flex;align-items:center;gap:8px}}\
         #sp-board-sidebar h1::before{{content:'◆';color:#3b82f6;font-size:11px}}\
         .sp-board-item{{padding:10px 12px;margin:0 0 3px;border-radius:9px;font-size:12.5px;font-weight:500;cursor:pointer;display:flex;align-items:center;gap:9px;white-space:nowrap;overflow:hidden;text-overflow:ellipsis;transition:background .12s,transform .12s}}\
         .sp-board-item:hover{{background:var(--sp-item-hover,rgba(255,255,255,.09));transform:translateX(2px)}}\
         .sp-board-item:active{{background:rgba(59,130,246,.25)}}\
         .sp-board-item-dot{{width:7px;height:7px;border-radius:50%;background:#4183c4;flex:none}}\
         #sp-board-zoom{{position:fixed;right:14px;bottom:14px;z-index:5;background:var(--sp-zoom-bg,#fff);border:1px solid var(--sp-frame-border,#e2e8f0);border-radius:8px;padding:6px 12px;font-size:12px;color:var(--sp-zoom-fg,#475569);box-shadow:0 1px 4px rgba(0,0,0,.12)}}\
         </style></head><body>\
         <div id=\"sp-board-sidebar\"><h1>Sera Board</h1>{sidebar_items}</div>\
         <div id=\"sp-board-viewport\"><div id=\"sp-board-world\">{frames}</div></div>\
         <div id=\"sp-board-zoom\">100%</div>\
         <script>(function(){{\
         var world=document.getElementById('sp-board-world');\
         var viewport=document.getElementById('sp-board-viewport');\
         var zoomLabel=document.getElementById('sp-board-zoom');\
         var panX=40,panY=40,zoom=1;\
         function apply(){{world.style.transform='translate('+panX+'px,'+panY+'px) scale('+zoom+')';zoomLabel.textContent=Math.round(zoom*100)+'%';}}\
         apply();\
         var topZ=1;\
         function lockSelection(v){{document.body.style.userSelect=v?'none':'';document.body.style.webkitUserSelect=v?'none':'';}}\
         var panning=false,panStartX=0,panStartY=0,panOrigX=0,panOrigY=0;\
         viewport.addEventListener('mousedown',function(e){{\
         if(e.target.closest('.sp-board-frame')||e.target.closest('[data-sp-mark]')||(window.__spToolActive&&window.__spToolActive()))return;\
         e.preventDefault();\
         panning=true;panStartX=e.clientX;panStartY=e.clientY;panOrigX=panX;panOrigY=panY;viewport.classList.add('panning');lockSelection(true);\
         }});\
         window.addEventListener('mousemove',function(e){{\
         if(!panning)return;\
         panX=panOrigX+(e.clientX-panStartX);panY=panOrigY+(e.clientY-panStartY);apply();\
         }});\
         window.addEventListener('mouseup',function(){{panning=false;viewport.classList.remove('panning');lockSelection(false);}});\
         viewport.addEventListener('wheel',function(e){{\
         e.preventDefault();\
         var rect=viewport.getBoundingClientRect();\
         var cx=e.clientX-rect.left,cy=e.clientY-rect.top;\
         var worldX=(cx-panX)/zoom,worldY=(cy-panY)/zoom;\
         var factor=e.deltaY<0?1.12:1/1.12;\
         zoom=Math.min(8,Math.max(0.05,zoom*factor));\
         panX=cx-worldX*zoom;panY=cy-worldY*zoom;\
         apply();\
         }},{{passive:false}});\
         var selected={{}};\
         function isSelected(f){{return !!selected[f.id];}}\
         function setSelected(f,v){{if(v)selected[f.id]=true;else delete selected[f.id];f.classList.toggle('sp-board-selected',v);}}\
         function frameIndex(f){{return parseInt(f.id.slice('sp-board-frame-'.length),10);}}\
         function sendLayout(f){{if(window.__spLiveSend)window.__spLiveSend('layout:'+frameIndex(f)+':'+f.dataset.x+':'+f.dataset.y);}}\
         document.querySelectorAll('.sp-board-frame').forEach(function(frame){{\
         frame.addEventListener('mouseenter',function(){{if(!(window.__spToolActive&&window.__spToolActive()))frame.classList.add('sp-board-active');}});\
         frame.addEventListener('mouseleave',function(){{frame.classList.remove('sp-board-active');}});\
         }});\
         document.querySelectorAll('.sp-board-frame-bar').forEach(function(bar){{\
         var frame=bar.parentElement;\
         bar.addEventListener('mousedown',function(e){{\
         e.preventDefault();\
         e.stopPropagation();\
         if(e.shiftKey){{setSelected(frame,!isSelected(frame));return;}}\
         var group=isSelected(frame)?Object.keys(selected).map(function(id){{return document.getElementById(id);}}).filter(Boolean):[frame];\
         group.forEach(function(f){{f.style.zIndex=String(++topZ);}});\
         var startX=e.clientX,startY=e.clientY;\
         var origins=group.map(function(f){{return {{f:f,x:parseFloat(f.dataset.x),y:parseFloat(f.dataset.y)}};}});\
         lockSelection(true);\
         function move(ev){{\
         var dx=(ev.clientX-startX)/zoom,dy=(ev.clientY-startY)/zoom;\
         origins.forEach(function(o){{\
         var nx=o.x+dx,ny=o.y+dy;\
         o.f.style.left=nx+'px';o.f.style.top=ny+'px';\
         o.f.dataset.x=String(nx);o.f.dataset.y=String(ny);\
         }});\
         }}\
         function up(){{\
         window.removeEventListener('mousemove',move);window.removeEventListener('mouseup',up);lockSelection(false);\
         origins.forEach(function(o){{sendLayout(o.f);}});\
         }}\
         window.addEventListener('mousemove',move);window.addEventListener('mouseup',up);\
         }});\
         frame.addEventListener('mousedown',function(e){{if(!e.shiftKey)frame.style.zIndex=String(++topZ);}});\
         }});\
         document.querySelectorAll('.sp-board-item').forEach(function(item){{\
         item.addEventListener('click',function(){{\
         var frame=document.getElementById('sp-board-frame-'+item.dataset.target);\
         if(!frame)return;\
         var fx=parseFloat(frame.dataset.x),fy=parseFloat(frame.dataset.y);\
         window.__spBoardFocusAt(fx+{FRAME_W}/2,fy+{FRAME_H}/2,1);\
         }});\
         }});\
         window.__spBoardFocusAt=function(x,y,z){{\
         var rect=viewport.getBoundingClientRect();\
         if(typeof z==='number')zoom=z;\
         panX=rect.width/2-x*zoom;panY=rect.height/2-y*zoom;\
         apply();\
         }};\
         window.__spBoardWorld=world;\
         window.__spBoardGetTransform=function(){{return {{x:panX,y:panY,z:zoom}};}};\
         window.__spBoardApplyLayout=function(list){{\
         (list||[]).forEach(function(l){{\
         var f=document.getElementById('sp-board-frame-'+l.index);\
         if(!f)return;\
         f.style.left=l.x+'px';f.style.top=l.y+'px';f.dataset.x=String(l.x);f.dataset.y=String(l.y);\
         }});\
         }};\
         }})();</script>\
         </body></html>"
    )
}

#[pyfunction]
#[pyo3(signature = (charts, path, titles = None))]
#[sera_doc(
    category = "pulse_function",
    module = "board",
    file = "seraplot/pulse-pricing.md",
    en = "Lays out several charts on one infinite, pannable, zoomable canvas — Figma-style: a sidebar lists every chart, each one sits in its own draggable frame that can be freely repositioned or overlapped, scroll to zoom in/out with no fixed limit either way. Each chart keeps its own full interactivity (hover, its own push()/serve() wiring if any) inside its frame.",
    fr = "Dispose plusieurs charts sur une seule toile infinie, déplaçable et zoomable — façon Figma : une barre latérale liste chaque chart, chacun dans son propre cadre déplaçable librement ou superposable, molette pour zoomer/dézoomer sans limite fixe dans un sens ou l'autre. Chaque chart garde sa pleine interactivité (survol, son propre câblage push()/serve() s'il en a) à l'intérieur de son cadre.",
    param(name = "charts", ty = "list[Chart]", en = "Charts to place on the board, one frame each.", fr = "Charts à placer sur la toile, un cadre chacun."),
    param(name = "path", ty = "str", en = "Destination .html file path — open it directly in any browser, no server required.", fr = "Chemin du fichier .html de destination — à ouvrir directement dans un navigateur, sans serveur."),
    param(name = "titles", ty = "list[str] | None", en = "Optional per-frame title shown in the sidebar and the frame's title bar; must have exactly one entry per chart if given.", fr = "Titre optionnel par cadre affiché dans la barre latérale et la barre de titre du cadre ; doit avoir exactement une entrée par chart si fourni.")
)]
pub fn export_board_html(charts: Vec<PyRef<'_, Chart>>, path: &str, titles: Option<Vec<String>>) -> PyResult<()> {
    if charts.is_empty() {
        return Err(pyo3::exceptions::PyValueError::new_err("seraplot: export_board_html() needs at least one chart"));
    }
    if let Some(t) = &titles {
        if t.len() != charts.len() {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "seraplot: titles has {} entries but {} charts were given - they must match in length",
                t.len(),
                charts.len()
            )));
        }
    }
    let entries: Vec<(String, String)> = charts
        .iter()
        .enumerate()
        .map(|(i, chart)| {
            let title = titles.as_ref().map(|t| t[i].clone()).unwrap_or_else(|| format!("Chart {}", i + 1));
            (title, chart.html.clone())
        })
        .collect();
    let live_urls = vec![None; entries.len()];
    let html = build_board_html(&entries, &live_urls);
    super::atomic_write(path, &html).map_err(|e| pyo3::exceptions::PyOSError::new_err(format!("seraplot: failed to write {path}: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn no_live_urls(n: usize) -> Vec<Option<String>> {
        vec![None; n]
    }

    #[test]
    fn escape_attr_escapes_ampersand_and_double_quote_but_leaves_angle_brackets() {
        let out = escape_attr("<div a=\"x&y\">");
        assert_eq!(out, "<div a=&quot;x&amp;y&quot;>");
    }

    #[test]
    fn escape_html_text_escapes_the_three_dangerous_characters() {
        assert_eq!(escape_html_text("a<b>c&d"), "a&lt;b&gt;c&amp;d");
    }

    #[test]
    fn grid_layout_of_zero_is_empty() {
        assert!(grid_layout(0).is_empty());
    }

    #[test]
    fn grid_layout_never_places_two_frames_at_the_same_position() {
        let positions = grid_layout(9);
        let mut seen = std::collections::HashSet::new();
        for p in &positions {
            assert!(seen.insert(*p), "duplicate position {p:?}");
        }
        assert_eq!(positions.len(), 9);
    }

    #[test]
    fn grid_layout_starts_at_the_origin() {
        assert_eq!(grid_layout(1)[0], (0, 0));
    }

    #[test]
    fn build_board_html_embeds_one_frame_per_entry_with_matching_titles() {
        let entries = vec![
            ("Sales".to_string(), "<html><body>A</body></html>".to_string()),
            ("Latency".to_string(), "<html><body>B</body></html>".to_string()),
        ];
        let html = build_board_html(&entries, &no_live_urls(2));
        assert_eq!(html.matches("sp-board-frame\"").count(), 2);
        assert!(html.contains("Sales"));
        assert!(html.contains("Latency"));
        assert_eq!(html.matches("sp-board-item\"").count(), 2);
    }

    #[test]
    fn build_board_html_escapes_a_chart_containing_quotes_and_ampersands_without_breaking_the_srcdoc_attribute() {
        let entries = vec![("T".to_string(), "<html><body>A & \"B\"</body></html>".to_string())];
        let html = build_board_html(&entries, &no_live_urls(1));
        assert!(html.contains("A &amp; &quot;B&quot;"));
        assert!(!html.contains("B\"</body>"), "an unescaped quote inside srcdoc would terminate the attribute early");
    }

    #[test]
    fn build_board_html_defaults_an_empty_title_to_untitled() {
        let entries = vec![("".to_string(), "<html></html>".to_string())];
        let html = build_board_html(&entries, &no_live_urls(1));
        assert!(html.contains("Untitled"));
    }

    #[test]
    fn build_board_html_is_a_single_self_contained_document() {
        let entries = vec![("T".to_string(), "<html></html>".to_string())];
        let html = build_board_html(&entries, &no_live_urls(1));
        assert!(html.starts_with("<!doctype html>"));
        assert!(html.trim_end().ends_with("</html>"));
        assert_eq!(html.matches("<script>").count(), 1);
    }

    #[test]
    fn a_live_url_entry_embeds_a_real_src_iframe_instead_of_srcdoc_and_shows_a_live_dot() {
        let entries = vec![
            ("A".to_string(), "<html></html>".to_string()),
            ("B".to_string(), "<html></html>".to_string()),
        ];
        let live_urls = vec![Some("http://127.0.0.1:8788".to_string()), None];
        let html = build_board_html(&entries, &live_urls);
        assert!(html.contains("src=\"http://127.0.0.1:8788\""));
        assert_eq!(html.matches("srcdoc=").count(), 1);
        assert_eq!(html.matches("class=\"sp-board-live-dot\"").count(), 2, "one live dot in the sidebar row and one in the frame bar, only for the live entry");
    }

    #[test]
    fn perf_building_a_fifty_chart_board_stays_fast() {
        let entries: Vec<(String, String)> = (0..50)
            .map(|i| (format!("Chart {i}"), format!("<html><body>{}</body></html>", "x".repeat(2000))))
            .collect();
        let start = std::time::Instant::now();
        let html = build_board_html(&entries, &no_live_urls(50));
        let elapsed = start.elapsed();
        println!("board perf: 50 charts assembled in {elapsed:?}, {} bytes", html.len());
        assert!(elapsed.as_millis() < 200, "assembling 50 frames took too long: {elapsed:?}");
        assert_eq!(html.matches("sp-board-frame\"").count(), 50);
    }
}
