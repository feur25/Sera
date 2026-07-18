pub struct HtmlTemplate;

impl HtmlTemplate {
    pub const INTERACTIVE_JS: &'static str = r#"
    <style>
    .interactive-bar, .interactive-point {
        cursor: pointer;
        transition: all 0.2s ease;
        filter: drop-shadow(0 2px 4px rgba(0,0,0,0.1));
    }
    .interactive-bar:hover, .interactive-point:hover {
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
    .tooltip.visible { opacity: 1; }
    .tooltip-image {
        max-width: 250px;
        max-height: 200px;
        margin-top: 8px;
        border-radius: 4px;
        display: block;
    }
    </style>

    <script>
    const tooltip = document.createElement('div');
    tooltip.className = 'tooltip';
    document.body.appendChild(tooltip);

    const state = window.__SERAPLOT_STATE__ || {};
    const labels = state.labels || [];
    const values = state.values || [];
    const hoverData = state.hover_data || [];
    const images = state.images_base64 || [];

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
        if (images[idx]) {
            let img = document.createElement('img');
            img.className = 'tooltip-image';
            img.src = images[idx];
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

    document.addEventListener('DOMContentLoaded', () => {
        let svg = document.querySelector('.chart-container svg');
        if (!svg) return;
        
        [svg.querySelectorAll('rect[data-index]'), svg.querySelectorAll('circle[data-index]')]
            .forEach(els => els.forEach(el => {
                el.classList.add('interactive-' + (el.tagName === 'CIRCLE' ? 'point' : 'bar'));
                let idx = parseInt(el.getAttribute('data-index'));
                el.addEventListener('mouseenter', (e) => showTooltip(e, idx));
                el.addEventListener('mouseleave', hideTooltip);
                el.addEventListener('mousemove', (e) => {
                    let r = e.target.getBoundingClientRect();
                    tooltip.style.left = (r.left + r.width / 2 - tooltip.offsetWidth / 2) + 'px';
                    tooltip.style.top = (r.top - tooltip.offsetHeight - 10) + 'px';
                });
            }));
    });
    </script>
    "#;

    pub const EXPORT_BUTTONS: &'static str = r#"
    <div class="controls-panel">
        <h3>Export</h3>
        <button onclick="downloadState()" class="control-btn">⬇️ State</button>
        <button onclick="exportSVG()" class="control-btn">🖼️ SVG</button>
        <button onclick="exportHTML()" class="control-btn">💾 HTML</button>
    </div>
    "#;

    pub fn build_style(
        _theme: &str,
        accent: &str,
        bg: &str,
        text: &str,
        dark_bg: &str,
        border: &str,
    ) -> String {
        format!(
            r#"
    <style>
    * {{ margin: 0; padding: 0; box-sizing: border-box; }}
    body {{
        background-color: {bg};
        color: {text};
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        line-height: 1.6;
    }}
    .container {{
        max-width: 1400px;
        margin: 0 auto;
        padding: 40px 20px;
    }}
    .header {{
        text-align: center;
        margin-bottom: 40px;
        border-bottom: 2px solid {accent};
        padding-bottom: 20px;
    }}
    h1 {{
        color: {accent};
        font-size: 2.5em;
        margin-bottom: 10px;
    }}
    .subtitle {{
        color: {text};
        opacity: 0.7;
        font-size: 1.1em;
    }}
    .controls-panel {{
        background-color: {dark_bg};
        border-radius: 8px;
        padding: 20px;
        margin-bottom: 20px;
        box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    }}
    .controls-panel h3 {{
        margin-bottom: 15px;
        color: {accent};
    }}
    .control-btn {{
        background-color: {accent};
        color: white;
        border: none;
        padding: 10px 20px;
        margin-right: 10px;
        margin-bottom: 10px;
        border-radius: 4px;
        cursor: pointer;
        font-size: 14px;
        transition: opacity 0.3s;
    }}
    .control-btn:hover {{
        opacity: 0.8;
    }}
    .chart-container {{
        background-color: {dark_bg};
        border-radius: 8px;
        padding: 20px;
        box-shadow: 0 2px 8px rgba(0,0,0,0.1);
        overflow: auto;
        margin-bottom: 20px;
    }}
    svg {{
        width: auto;
        height: auto;
        max-width: 100%;
    }}
    .stats {{
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
        gap: 20px;
        margin-bottom: 20px;
    }}
    .stat-card {{
        background-color: {dark_bg};
        border-left: 4px solid {accent};
        padding: 20px;
        border-radius: 4px;
    }}
    .stat-label {{
        font-size: 0.9em;
        color: {text};
        opacity: 0.7;
        margin-bottom: 8px;
    }}
    .stat-value {{
        color: {accent};
        font-size: 1.8em;
        font-weight: bold;
    }}
    .state-info {{
        background-color: {dark_bg};
        border: 1px solid {border};
        border-radius: 4px;
        padding: 15px;
        margin-top: 20px;
        font-family: monospace;
        font-size: 12px;
        max-height: 300px;
        overflow-y: auto;
    }}
    .footer {{
        text-align: center;
        margin-top: 40px;
        padding-top: 20px;
        border-top: 1px solid {border};
        color: {text};
        opacity: 0.7;
        font-size: 0.9em;
    }}
    </style>
            "#
        )
    }
}
