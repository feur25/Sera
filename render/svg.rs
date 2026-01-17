use crate::core::*;

pub struct SvgRenderer;

impl SvgRenderer {
    pub fn render_traces(traces: &[Trace], layout: &Layout) -> String {
        let mut svg = String::with_capacity(65536);
        let width = layout.width as f64;
        let height = layout.height as f64;
        
        svg.push_str(&format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg" style="background:#fff">"#,
            width, height
        ));
        
        if !layout.title.is_empty() {
            svg.push_str(&format!(
                r#"<text x="{}" y="30" text-anchor="middle" font-size="18" font-weight="bold">{}</text>"#,
                width / 2.0, layout.title
            ));
        }
        
        let margin = 60.0;
        svg.push_str(&format!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="1"/>"#,
            margin, height - margin, width - margin, height - margin
        ));
        svg.push_str(&format!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="1"/>"#,
            margin, margin, margin, height - margin
        ));
        
        let plot_width = width - 2.0 * margin;
        let plot_height = height - 2.0 * margin;
        
        for (_idx, trace) in traces.iter().enumerate() {
            if trace.x.is_empty() || trace.y.is_empty() {
                continue;
            }
            
            let x_min = trace.x.iter().cloned().fold(f64::INFINITY, f64::min);
            let x_max = trace.x.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let y_min = trace.y.iter().cloned().fold(f64::INFINITY, f64::min);
            let y_max = trace.y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            
            let x_range = (x_max - x_min).max(0.0001);
            let y_range = (y_max - y_min).max(0.0001);
            
            for (x, y) in trace.x.iter().zip(trace.y.iter()) {
                let px = margin + ((x - x_min) / x_range) * plot_width;
                let py = (height - margin) - ((y - y_min) / y_range) * plot_height;
                
                svg.push_str(&format!(
                    r#"<circle cx="{}" cy="{}" r="3" fill="rgb({},{},{})" opacity="{}"/>"#,
                    px, py, trace.marker.color.r, trace.marker.color.g, trace.marker.color.b,
                    trace.marker.opacity
                ));
            }
        }
        
        svg.push_str("</svg>");
        svg
    }
}
