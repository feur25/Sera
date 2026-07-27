use super::canvas_core::Canvas;
use super::element::{El, Layer};
use super::render::{el_layer, render_el, render_frame, render_image};
use pyo3::prelude::*;

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

#[pymethods]
impl Canvas {
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
                "function qS(g){return root.querySelectorAll('div[data-sp-grp=\"'+g+'\"],rect[data-sp-grp=\"'+g+'\"],text[data-sp-grp=\"'+g+'\"],path.sp-wedge[data-sp-grp=\"'+g+'\"]');}",
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
                "root.querySelectorAll('.sp-hvh,div[data-sp-grp],rect[data-sp-grp],text[data-sp-grp],circle.sp-anch[data-sp-grp],path.sp-wedge[data-sp-grp]').forEach(function(hit){",
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
                "html,body{{margin:0;padding:0;overflow:hidden;background:{bg};",
                "width:100%;height:100%;display:flex;align-items:center;justify-content:center}}\n",
                "#sp-canvas-root{{position:relative;width:{w}px;height:{h}px;flex-shrink:0;",
                "overflow:hidden;background:{bg};transform-origin:center}}\n",
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
        let html = crate::html::hover::inject_before_body(&chart.html, &format!("<script>{}</script></body>", CANVAS_DEV_JS));
        crate::Chart { html, doc_str: "" }
    }
}
