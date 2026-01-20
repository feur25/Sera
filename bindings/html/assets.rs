pub struct Assets;

impl Assets {
    pub const STYLE_CSS: &'static str = r#"* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  background-color: #f5f5f5;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

.chart-container {
  width: 100%;
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #fff;
}

svg {
  width: 90%;
  height: 90%;
  max-width: 1200px;
  max-height: 600px;
}

.interactive-bar,
.interactive-point,
.interactive-line {
  cursor: pointer;
  transition: all 0.2s ease;
  filter: drop-shadow(0 2px 4px rgba(0,0,0,0.1));
}

.interactive-bar:hover,
.interactive-point:hover,
.interactive-line:hover {
  filter: drop-shadow(0 4px 8px rgba(0,0,0,0.25));
}

.tooltip {
  position: fixed;
  background: rgba(0, 0, 0, 0.95);
  color: #fff;
  padding: 12px 16px;
  border-radius: 6px;
  font-size: 12px;
  pointer-events: none;
  z-index: 10000;
  opacity: 0;
  transition: opacity 0.2s;
  max-width: 300px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.3);
}

.tooltip.visible {
  opacity: 1;
}

.tooltip-image {
  max-width: 250px;
  max-height: 200px;
  margin-top: 8px;
  border-radius: 4px;
  display: block;
}
"#;

    pub const SCRIPT_JS: &'static str = r#"const tooltip = document.createElement('div');
tooltip.className = 'tooltip';
document.body.appendChild(tooltip);

const state = window.__SERAPLOT_STATE__ || {};
const labels = state.labels || [];
const values = state.values || [];
const hoverData = state.hover_data || [];

function showTooltip(e, idx) {
  tooltip.innerHTML = '';
  let div = document.createElement('div');
  
  if (labels[idx]) {
    let h = document.createElement('div');
    h.style.fontWeight = 'bold';
    h.textContent = labels[idx];
    div.appendChild(h);
  }
  if (values[idx] !== undefined) {
    let v = document.createElement('div');
    v.textContent = 'Value: ' + values[idx].toFixed(2);
    div.appendChild(v);
  }
  if (hoverData[idx] && hoverData[idx].image) {
    let img = document.createElement('img');
    img.className = 'tooltip-image';
    img.src = hoverData[idx].image;
    img.crossOrigin = 'anonymous';
    img.onerror = () => img.style.display = 'none';
    div.appendChild(img);
  }
  if (hoverData[idx]) {
    for (let [k, v] of Object.entries(hoverData[idx])) {
      if (k === 'image') continue;
      let l = document.createElement('div');
      l.textContent = k + ': ' + v;
      div.appendChild(l);
    }
  }
  
  tooltip.appendChild(div);
  tooltip.classList.add('visible');
  
  let r = e.target.getBoundingClientRect();
  tooltip.style.left = (r.left + r.width / 2 - tooltip.offsetWidth / 2) + 'px';
  tooltip.style.top = (r.top - tooltip.offsetHeight - 10) + 'px';
}

function hideTooltip() {
  tooltip.classList.remove('visible');
}

function attachHoverListeners() {
  let svg = document.querySelector('svg');
  if (!svg) return;
  
  Array.from(svg.querySelectorAll('rect[data-index]')).forEach(el => {
    el.classList.add('interactive-bar');
    let idx = parseInt(el.getAttribute('data-index'));
    el.addEventListener('mouseenter', (e) => showTooltip(e, idx));
    el.addEventListener('mouseleave', hideTooltip);
    el.addEventListener('mousemove', (e) => {
      let r = e.target.getBoundingClientRect();
      tooltip.style.left = (r.left + r.width / 2 - tooltip.offsetWidth / 2) + 'px';
      tooltip.style.top = (r.top - tooltip.offsetHeight - 10) + 'px';
    });
  });
  
  Array.from(svg.querySelectorAll('circle[data-index]')).forEach(el => {
    el.classList.add('interactive-point');
    let idx = parseInt(el.getAttribute('data-index'));
    el.addEventListener('mouseenter', (e) => showTooltip(e, idx));
    el.addEventListener('mouseleave', hideTooltip);
    el.addEventListener('mousemove', (e) => {
      let r = e.target.getBoundingClientRect();
      tooltip.style.left = (r.left + r.width / 2 - tooltip.offsetWidth / 2) + 'px';
      tooltip.style.top = (r.top - tooltip.offsetHeight - 10) + 'px';
    });
  });
  
  Array.from(svg.querySelectorAll('path[data-index]')).forEach(el => {
    el.classList.add('interactive-line');
    let idx = parseInt(el.getAttribute('data-index'));
    el.addEventListener('mouseenter', (e) => showTooltip(e, idx));
    el.addEventListener('mouseleave', hideTooltip);
    el.addEventListener('mousemove', (e) => {
      let r = e.target.getBoundingClientRect();
      tooltip.style.left = (r.left + r.width / 2 - tooltip.offsetWidth / 2) + 'px';
      tooltip.style.top = (r.top - tooltip.offsetHeight - 10) + 'px';
    });
  });
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', attachHoverListeners);
} else {
  attachHoverListeners();
}
"#;
}
