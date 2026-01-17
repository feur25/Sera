use crate::core::*;
use serde_json::json;

pub struct JsonRenderer;

impl JsonRenderer {
    pub fn render(traces: &[Trace], layout: &Layout) -> Result<String, Box<dyn std::error::Error>> {
        let mut traces_json = Vec::new();
        
        for trace in traces {
            let x_vals: Vec<_> = trace.x.iter().map(|v| json!(v)).collect();
            let y_vals: Vec<_> = trace.y.iter().map(|v| json!(v)).collect();
            
            traces_json.push(json!({
                "name": trace.name,
                "type": format!("{:?}", trace.kind).to_lowercase(),
                "x": x_vals,
                "y": y_vals,
                "marker": {
                    "color": format!("rgb({}, {}, {})", trace.marker.color.r, trace.marker.color.g, trace.marker.color.b)
                }
            }));
        }
        
        let plot_json = json!({
            "data": traces_json,
            "layout": {
                "title": layout.title,
                "width": layout.width,
                "height": layout.height,
                "xaxis": {
                    "title": layout.x_axis.title
                },
                "yaxis": {
                    "title": layout.y_axis.title
                }
            }
        });
        
        Ok(plot_json.to_string())
    }
}
