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
  position: relative;
}

.zoom-controls {
  position: absolute;
  top: 10px;
  right: 20px;
  left: auto;
  display: flex;
  gap: 8px;
  z-index: 1000;
  background: rgba(255, 255, 255, 0.95);
  padding: 8px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.15);
  opacity: 0;
  transition: opacity 0.3s ease;
  pointer-events: none;
}

.chart-container:hover .zoom-controls {
  opacity: 1;
  pointer-events: auto;
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

.selection-rect {
  position: absolute;
  border: 2px solid #1f77b4;
  background: rgba(31, 119, 180, 0.1);
  pointer-events: none;
  z-index: 999;
  user-select: none;
}

.selection-indicator {
  position: absolute;
  left: 0;
  width: 4px;
  background: #1f77b4;
  opacity: 0.3;
  pointer-events: none;
  z-index: 998;
  transition: height 0.1s ease, top 0.1s ease;
}

body.selecting {
  user-select: none;
}

.zoom-btn {
  width: 40px;
  height: 40px;
  border: none;
  background: #1f77b4;
  color: white;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
  font-weight: bold;
  transition: all 0.2s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.zoom-btn:hover {
  background: #155fa0;
  transform: scale(1.05);
}

.zoom-btn:active {
  transform: scale(0.95);
}

.modal {
  display: none;
  position: fixed;
  z-index: 2000;
  left: 0;
  top: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0,0,0,0.4);
}

.modal.show {
  display: flex;
  align-items: center;
  justify-content: center;
}

.modal-content {
  background-color: #fefefe;
  padding: 0;
  border-radius: 8px;
  width: 95%;
  height: 95%;
  max-width: 1600px;
  max-height: 900px;
  display: flex;
  flex-direction: column;
  box-shadow: 0 4px 20px rgba(0,0,0,0.3);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px 20px;
  border-bottom: 1px solid #e0e0e0;
  background: #f5f5f5;
}

.modal-header h2 {
  margin: 0;
  font-size: 18px;
  color: #333;
}

.modal-close {
  background: none;
  border: none;
  font-size: 28px;
  cursor: pointer;
  color: #666;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.modal-close:hover {
  color: #000;
}

.modal-body {
  flex: 1;
  overflow: auto;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
  background: white;
}

.modal-body svg {
  width: 100%;
  height: 100%;
  max-width: 100%;
  max-height: 100%;
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

let currentZoom = 1;
const svg = document.querySelector('svg');
const zoomContainer = document.querySelector('.chart-container');

if (svg && !svg.getAttribute('data-original-viewbox')) {
  svg.setAttribute('data-original-viewbox', svg.getAttribute('viewBox') || '0 0 1200 600');
}

let selectionStart = null;
let selectionEnd = null;
let selectionRect = null;
let selectionIndicator = null;
let isSelecting = false;
let originalVisibility = {};

function updateZoom() {
  if (svg) {
    svg.style.transform = `scale(${currentZoom})`;
    svg.style.transformOrigin = 'center center';
    zoomContainer.style.overflow = currentZoom > 1 ? 'auto' : 'hidden';
  }
}

function zoomIn() {
  currentZoom = Math.min(currentZoom + 0.2, 5);
  updateZoom();
}

function zoomOut() {
  currentZoom = Math.max(currentZoom - 0.2, 0.5);
  updateZoom();
}

function resetZoom() {
  currentZoom = 1;
  updateZoom();
}

function openPopup() {
  const modal = document.getElementById('seraplot-modal');
  if (modal) {
    modal.classList.add('show');
    const modalSvg = document.getElementById('modal-svg');
    if (svg && modalSvg) {
      modalSvg.innerHTML = svg.innerHTML;
      modalSvg.setAttribute('viewBox', svg.getAttribute('viewBox'));
      modalSvg.setAttribute('width', svg.getAttribute('width'));
      modalSvg.setAttribute('height', svg.getAttribute('height'));
      
      Array.from(svg.querySelectorAll('[data-index]')).forEach(srcEl => {
        const tgtEl = modalSvg.querySelector(`[data-index="${srcEl.getAttribute('data-index')}"]`);
        if (tgtEl) {
          tgtEl.style.display = srcEl.style.display;
          tgtEl.style.opacity = srcEl.style.opacity;
          tgtEl.style.pointerEvents = srcEl.style.pointerEvents;
        }
      });
      
      attachHoverListenersToSvg(modalSvg);
    }
  }
}

function closePopup() {
  const modal = document.getElementById('seraplot-modal');
  if (modal) {
    modal.classList.remove('show');
  }
}

function attachHoverListenersToSvg(targetSvg) {
  Array.from(targetSvg.querySelectorAll('rect[data-index]')).forEach(el => {
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
  
  Array.from(targetSvg.querySelectorAll('circle[data-index]')).forEach(el => {
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
  
  Array.from(targetSvg.querySelectorAll('path[data-index]')).forEach(el => {
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

function getElementsInSelection(minX, maxX, minY, maxY) {
  const elements = [];
  if (!svg) return elements;
  
  const svgRect = svg.getBoundingClientRect();
  const ctm = svg.getScreenCTM();
  if (!ctm) return elements;
  
  const visibleElements = Array.from(svg.querySelectorAll('rect[data-index]')).filter(el => 
    el.style.display !== 'none'
  );
  
  visibleElements.forEach(el => {
    try {
      const bbox = el.getBBox();
      const pt = svg.createSVGPoint();
      pt.x = bbox.x;
      pt.y = bbox.y;
      const screenPt = pt.matrixTransform(ctm);
      const barScreenX = screenPt.x - svgRect.left;
      const barScreenY = screenPt.y - svgRect.top;
      const barScreenW = bbox.width * ctm.a;
      const barScreenH = bbox.height * ctm.d;
      
      const barMinX = barScreenX;
      const barMaxX = barScreenX + barScreenW;
      const barMinY = barScreenY;
      const barMaxY = barScreenY + barScreenH;
      
      if (minX < barMaxX && maxX > barMinX && minY < barMaxY && maxY > barMinY) {
        elements.push(parseInt(el.getAttribute('data-index')));
      }
    } catch (e) {
      // ignore
    }
  });
  
  return elements;
}

function applyBoxSelection() {
  if (!svg || !selectionStart || !selectionEnd) return;
  
  const svgRect = svg.getBoundingClientRect();
  const minX = Math.min(selectionStart.x, selectionEnd.x) - svgRect.left;
  const maxX = Math.max(selectionStart.x, selectionEnd.x) - svgRect.left;
  const minY = Math.min(selectionStart.y, selectionEnd.y) - svgRect.top;
  const maxY = Math.max(selectionStart.y, selectionEnd.y) - svgRect.top;
  
  const threshold = 5;
  if ((maxX - minX) > threshold && (maxY - minY) > threshold) {
    const selectedIndices = getElementsInSelection(minX, maxX, minY, maxY);
    const allElements = Array.from(svg.querySelectorAll('[data-index]'));
    
    allElements.forEach(el => {
      const idx = parseInt(el.getAttribute('data-index'));
      if (selectedIndices.includes(idx)) {
        el.style.display = 'block';
        el.style.opacity = '1';
        el.style.pointerEvents = 'auto';
      } else {
        el.style.display = 'none';
        el.style.opacity = '0';
        el.style.pointerEvents = 'none';
      }
    });
    
    rescaleSvgToVisibleElements();
    
    if (selectionIndicator) {
      selectionIndicator.style.top = minY + 'px';
      selectionIndicator.style.height = (maxY - minY) + 'px';
      selectionIndicator.style.display = 'block';
    }
  }
}

function rescaleSvgToVisibleElements() {
  if (!svg) return;
  
  const visibleElements = Array.from(svg.querySelectorAll('[data-index]')).filter(el => 
    el.style.display !== 'none'
  );
  
  if (visibleElements.length === 0) return;
  
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity;
  
  visibleElements.forEach(el => {
    const bbox = el.getBBox();
    minX = Math.min(minX, bbox.x);
    minY = Math.min(minY, bbox.y);
    maxX = Math.max(maxX, bbox.x + bbox.width);
    maxY = Math.max(maxY, bbox.y + bbox.height);
  });
  
  const originalViewBox = svg.getAttribute('data-original-viewbox');
  const originalParts = originalViewBox ? originalViewBox.split(' ').map(Number) : [0, 0, 1200, 600];
  
  const padding = 40;
  const leftPadding = Math.max(100, originalParts[0] + padding);
  const topPadding = padding;
  const rightPadding = Math.max(100, originalParts[2] - (minX + (maxX - minX)) - padding);
  const bottomPadding = Math.max(100, originalParts[3] - (minY + (maxY - minY)) - padding);
  
  const newViewBox = `${minX - leftPadding} ${minY - topPadding} ${maxX - minX + leftPadding + rightPadding} ${maxY - minY + topPadding + bottomPadding}`;
  svg.setAttribute('viewBox', newViewBox);
}

function resetBoxSelection() {
  if (!svg) return;
  
  const originalViewBox = svg.getAttribute('data-original-viewbox');
  
  Array.from(svg.querySelectorAll('[data-index]')).forEach(el => {
    el.style.display = 'block';
    el.style.opacity = '1';
    el.style.pointerEvents = 'auto';
  });
  
  if (originalViewBox) {
    svg.setAttribute('viewBox', originalViewBox);
  }
  
  selectionStart = null;
  selectionEnd = null;
  isSelecting = false;
  
  if (selectionRect) {
    selectionRect.style.display = 'none';
  }
  
  if (selectionIndicator) {
    selectionIndicator.style.display = 'none';
  }
  
  document.body.classList.remove('selecting');
}

document.addEventListener('DOMContentLoaded', () => {
  if (svg) {
    selectionRect = document.createElement('div');
    selectionRect.className = 'selection-rect';
    selectionRect.style.display = 'none';
    svg.parentNode.appendChild(selectionRect);
    
    selectionIndicator = document.createElement('div');
    selectionIndicator.className = 'selection-indicator';
    selectionIndicator.style.display = 'none';
    svg.parentNode.appendChild(selectionIndicator);
    
    svg.addEventListener('mousedown', (e) => {
      if (e.button === 0) {
        selectionStart = { x: e.clientX, y: e.clientY };
        selectionEnd = { x: e.clientX, y: e.clientY };
        isSelecting = true;
        selectionRect.style.display = 'block';
        document.body.classList.add('selecting');
      }
    });
    
    document.addEventListener('mousemove', (e) => {
      if (!isSelecting || !selectionStart) return;
      
      selectionEnd = { x: e.clientX, y: e.clientY };
      
      const minX = Math.min(selectionStart.x, selectionEnd.x);
      const maxX = Math.max(selectionStart.x, selectionEnd.x);
      const minY = Math.min(selectionStart.y, selectionEnd.y);
      const maxY = Math.max(selectionStart.y, selectionEnd.y);
      
      selectionRect.style.left = minX + 'px';
      selectionRect.style.top = minY + 'px';
      selectionRect.style.width = (maxX - minX) + 'px';
      selectionRect.style.height = (maxY - minY) + 'px';
      
      const svgRect = svg.getBoundingClientRect();
      selectionIndicator.style.display = 'block';
      selectionIndicator.style.top = minY + 'px';
      selectionIndicator.style.height = (maxY - minY) + 'px';
    });
    
    document.addEventListener('mouseup', (e) => {
      if (isSelecting) {
        applyBoxSelection();
        isSelecting = false;
        selectionRect.style.display = 'none';
        document.body.classList.remove('selecting');
      }
    });
  }
  
  const modal = document.getElementById('seraplot-modal');
  if (modal) {
    modal.addEventListener('click', (e) => {
      if (e.target === modal) {
        closePopup();
      }
    });
  }
  
  const closeBtn = document.getElementById('modal-close-btn');
  if (closeBtn) {
    closeBtn.addEventListener('click', closePopup);
  }
  
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
      if (isSelecting) {
        resetBoxSelection();
        document.body.classList.remove('selecting');
      } else {
        closePopup();
      }
    }
  });
});
"#;
}
