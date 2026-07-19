pub struct Assets;

impl Assets {
    pub const STYLE_CSS: &'static str = r#"*{margin:0;padding:0;box-sizing:border-box}body{background:#f5f5f5;font:12px -apple-system,BlinkMacSystemFont,Segoe UI,Roboto}.chart-container{width:100%;height:100vh;display:flex;align-items:center;justify-content:center;background:#fff;position:relative;overflow:hidden}.zoom-controls{position:absolute;top:10px;right:20px;display:flex;gap:8px;z-index:999;background:hsla(0,0%,100%,.95);padding:8px;border-radius:8px;opacity:0;pointer-events:none;will-change:opacity;contain:layout}.chart-container:hover .zoom-controls{opacity:1;pointer-events:auto}svg{width:90%;height:90%;max-width:1200px;max-height:600px;display:block;contain:layout style paint}rect[data-index]{pointer-events:auto}circle[data-index]{pointer-events:auto}polyline[data-index]{pointer-events:auto}.tooltip{position:fixed;background:rgba(0,0,0,.95);color:#fff;padding:8px 12px;border-radius:4px;font-size:10px;pointer-events:none;z-index:9999;opacity:0;max-width:280px;will-change:opacity,top,left;contain:layout;backface-visibility:hidden}.tooltip.visible{opacity:1}.tooltip-image{max-width:240px;max-height:180px;margin-top:4px;border-radius:2px;display:block}.selection-rect{position:absolute;border:2px solid #17b;background:rgba(17,119,180,.06);pointer-events:none;z-index:900;user-select:none;will-change:width,height,top,left}.modal{display:none;position:fixed;z-index:2000;inset:0;background:rgba(0,0,0,.4)}.modal.show{display:flex;align-items:center;justify-content:center}.modal-content{background:#fefefe;padding:0;border-radius:5px;width:95%;height:95%;max-width:1500px;max-height:850px;display:flex;flex-direction:column;box-shadow:0 4px 16px rgba(0,0,0,.3)}.modal-header{display:flex;justify-content:space-between;align-items:center;padding:10px 14px;border-bottom:1px solid #e0e0e0;background:#f5f5f5}.modal-header h2{margin:0;font-size:15px;color:#333}.modal-close{background:0;border:0;font-size:24px;cursor:pointer;color:#999;width:32px;height:32px;display:flex;align-items:center;justify-content:center;line-height:1;padding:0}.modal-close:hover{color:#333}.modal-body{flex:1;overflow:auto;display:flex;align-items:center;justify-content:center;padding:12px;background:#fff;contain:layout}.modal-body svg{width:100%;height:100%;max-width:100%;max-height:100%}.zoom-btn{width:32px;height:32px;border:0;background:#17b;color:#fff;border-radius:3px;cursor:pointer;font-size:14px;font-weight:600;display:flex;align-items:center;justify-content:center;padding:0;will-change:background,transform;backface-visibility:hidden}.zoom-btn:hover{background:#138;transform:scale(1.1)}.zoom-btn:active{transform:scale(.9)}"#;

    pub const SCRIPT_JS: &'static str = r#"
let tooltip = null;
let state = window.__SERAPLOT_STATE__ || {};
let labels = state.labels || [];
let values = state.values || [];
let hoverData = state.hover_data || {};

let svg = null;
let resetBtn = null;
let ss = null;
let se = null;
let selectionRect = null;
let selecting = 0;
let currentZoom = 1.0;

function showTooltip(e) {
    if (!tooltip) return;
    tooltip.innerHTML = '';
    let idx = parseInt(e.target.dataset.index, 10);
    let div = document.createElement('div');
    
    if (labels[idx]) {
        let h = document.createElement('div');
        h.textContent = labels[idx];
        h.style.fontWeight = 'bold';
        h.style.marginBottom = '4px';
        div.appendChild(h);
    }
    
    if (values[idx] !== undefined) {
        let v = document.createElement('div');
        v.textContent = 'Value: ' + values[idx].toFixed(2);
        v.style.marginBottom = '4px';
        div.appendChild(v);
    }
    
    if (hoverData[idx.toString()] && hoverData[idx.toString()].fields) {
        let fields = hoverData[idx.toString()].fields;
        for (let fi = 0; fi < fields.length; fi++) {
            let pair = fields[fi];
            if (!Array.isArray(pair) || pair.length !== 2) continue;
            let k = pair[0], val = pair[1];
            if (!val) continue;
            
            if (k === 'image' && typeof val === 'string') {
                let img = document.createElement('img');
                img.className = 'tooltip-image';
                img.src = val;
                img.onerror = () => img.remove();
                img.style.marginTop = '4px';
                div.appendChild(img);
            } else if (typeof val === 'string') {
                let line = document.createElement('div');
                line.style.fontSize = '10px';
                line.style.marginBottom = '2px';
                let display = val.length > 80 ? val.substring(0, 77) + '...' : val;
                line.textContent = k + ': ' + display;
                div.appendChild(line);
            }
        }
    }
    
    tooltip.appendChild(div);
    tooltip.classList.add('visible');
    let r = e.target.getBoundingClientRect();
    let tx = r.left + r.width / 2 - tooltip.offsetWidth / 2;
    let ty = r.top - tooltip.offsetHeight - 10;
    tooltip.style.left = Math.max(10, tx) + 'px';
    tooltip.style.top = Math.max(10, ty) + 'px';
}

function hideTooltip() {
    if (tooltip) tooltip.classList.remove('visible');
}

function attachListeners() {
    if (!svg) return;
    svg.querySelectorAll('[data-index]').forEach(el => {
        if (el.style.display !== 'none') {
            el.addEventListener('mouseenter', showTooltip, false);
            el.addEventListener('mouseleave', hideTooltip, false);
        }
    });
}

function getSelectedItems(mx, Mx, my, My) {
    let selected = [];
    let ctm = svg.getScreenCTM();
    if (!ctm) return selected;
    let svgRect = svg.getBoundingClientRect();
    
    svg.querySelectorAll('[data-index]').forEach(el => {
        if (el.style.display === 'none') return;
        try {
            let b = el.getBBox();
            let sx = b.x * ctm.a + ctm.e - svgRect.left;
            let sy = b.y * ctm.d + ctm.f - svgRect.top;
            let sw = b.width * Math.abs(ctm.a);
            let sh = b.height * Math.abs(ctm.d);
            if (mx < sx + sw && Mx > sx && my < sy + sh && My > sy) {
                selected.push(parseInt(el.dataset.index, 10));
            }
        } catch (E) {}
    });
    return selected;
}

function applySelection() {
    if (!svg || !ss || !se) return;
    let svgRect = svg.getBoundingClientRect();
    let mx = Math.min(ss.x, se.x) - svgRect.left;
    let Mx = Math.max(ss.x, se.x) - svgRect.left;
    let my = Math.min(ss.y, se.y) - svgRect.top;
    let My = Math.max(ss.y, se.y) - svgRect.top;
    if (Mx - mx <= 5 || My - my <= 5) return;
    
    let selectedIndices = getSelectedItems(mx, Mx, my, My);
    let els = svg.querySelectorAll('[data-index]');
    if (selectedIndices.length > 0) {
        els.forEach(el => {
            let idx = parseInt(el.dataset.index, 10);
            let isSelected = selectedIndices.indexOf(idx) > -1;
            el.style.display = isSelected ? 'block' : 'none';
            el.style.opacity = isSelected ? 1 : 0.2;
        });
    }
}

function resetSelection() {
    if (!svg) return;
    let ov = svg.dataset.originalViewbox;
    let visibleCount = state.visible_count || svg.querySelectorAll('[data-index]').length;
    
    svg.querySelectorAll('[data-index]').forEach(el => {
        let idx = parseInt(el.dataset.index, 10);
        if (idx < visibleCount) {
            el.style.display = 'block';
            el.style.opacity = 1;
        } else {
            el.style.display = 'none';
            el.style.opacity = 0.2;
        }
    });
    if (ov) svg.setAttribute('viewBox', ov);
    ss = se = null;
    selecting = 0;
    if (selectionRect) selectionRect.style.display = 'none';
    if (resetBtn) resetBtn.style.display = 'none';
    attachListeners();
}

function zoomOut() {
    currentZoom = Math.max(0.1, currentZoom / 1.2);
    updateZoom();
}

function zoomReset() {
    currentZoom = 1.0;
    if (svg && svg.dataset.originalViewbox) {
        svg.setAttribute('viewBox', svg.dataset.originalViewbox);
    }
}

function zoomIn() {
    currentZoom *= 1.2;
    updateZoom();
}

function updateZoom() {
    if (!svg) return;
    let ov = svg.dataset.originalViewbox;
    if (!ov) return;
    let parts = ov.split(' ').map(Number);
    let x = parts[0], y = parts[1], w = parts[2], h = parts[3];
    let nw = w / currentZoom;
    let nh = h / currentZoom;
    let nx = x + (w - nw) / 2;
    let ny = y + (h - nh) / 2;
    svg.setAttribute('viewBox', nx + ' ' + ny + ' ' + nw + ' ' + nh);
}

document.addEventListener('DOMContentLoaded', () => {
    tooltip = document.createElement('div');
    tooltip.className = 'tooltip';
    document.body.appendChild(tooltip);
    svg = document.querySelector('svg');
    if (!svg) return;
    
    svg.dataset.originalViewbox = svg.getAttribute('viewBox') || '0 0 1200 700';
    
    let visibleCount = state.visible_count || svg.querySelectorAll('[data-index]').length;
    if (visibleCount < svg.querySelectorAll('[data-index]').length) {
        svg.querySelectorAll('[data-index]').forEach(el => {
            let idx = parseInt(el.dataset.index, 10);
            if (idx >= visibleCount) {
                el.style.display = 'none';
                el.style.opacity = 0.2;
            }
        });
    }
    
    attachListeners();
    
    selectionRect = document.createElement('div');
    selectionRect.className = 'selection-rect';
    selectionRect.style.display = 'none';
    svg.parentNode.appendChild(selectionRect);
    
    resetBtn = document.createElement('button');
    resetBtn.textContent = 'Reset Selection';
    resetBtn.style.cssText = 'position:absolute;top:10px;left:10px;z-index:1000;padding:8px 16px;background:#17b;color:white;border:none;border-radius:4px;cursor:pointer;display:none;font-family:sans-serif;font-size:14px';
    resetBtn.onclick = resetSelection;
    svg.parentNode.appendChild(resetBtn);
    
    let md = (e) => {
        if (e.button === 0) {
            ss = {x: e.clientX, y: e.clientY};
            se = {x: e.clientX, y: e.clientY};
            selecting = 1;
            selectionRect.style.display = 'block';
            resetBtn.style.display = 'block';
        }
    };
    
    let mm = (e) => {
        if (!selecting || !ss) return;
        se = {x: e.clientX, y: e.clientY};
        let mx = Math.min(ss.x, se.x);
        let Mx = Math.max(ss.x, se.x);
        let my = Math.min(ss.y, se.y);
        let My = Math.max(ss.y, se.y);
        selectionRect.style.left = mx + 'px';
        selectionRect.style.top = my + 'px';
        selectionRect.style.width = (Mx - mx) + 'px';
        selectionRect.style.height = (My - my) + 'px';
    };
    
    let mu = () => {
        if (selecting) {
            applySelection();
            selecting = 0;
            selectionRect.style.display = 'none';
        }
    };
    
    svg.addEventListener('mousedown', md, false);
    document.addEventListener('mousemove', mm, false);
    document.addEventListener('mouseup', mu, false);
    
    document.addEventListener('keydown', (e) => {
        if (e.key === 'Escape') {
            if (selecting) {
                resetSelection();
                selecting = 0;
            }
        }
    }, false);
}, false);

window.zo = zoomOut;
window.zr = zoomReset;
window.zi = zoomIn;
window.rbs = resetSelection;
window.of = () => alert('Fullscreen not implemented');
"#;
}
