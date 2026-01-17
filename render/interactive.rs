use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct InteractiveElement {
    pub id: String,
    pub label: String,
    pub value: f64,
    pub metadata: HashMap<String, String>,
    pub hover_data: HashMap<String, String>,
}

pub struct InteractiveSvgRenderer {
    width: f32,
    height: f32,
    elements: Vec<InteractiveElement>,
    title: String,
    x_label: String,
    y_label: String,
    zoom_level: f32,
    zoom_x: f32,
    zoom_y: f32,
    zoom_width: f32,
    zoom_height: f32,
}

impl InteractiveSvgRenderer {
    pub fn new(width: f32, height: f32, title: impl Into<String>) -> Self {
        Self {
            width,
            height,
            elements: Vec::new(),
            title: title.into(),
            x_label: String::new(),
            y_label: String::new(),
            zoom_level: 1.0,
            zoom_x: 0.0,
            zoom_y: 0.0,
            zoom_width: 1000.0,
            zoom_height: 500.0,
        }
    }

    pub fn set_zoom(&mut self, zoom: f32, x: f32, y: f32, w: f32, h: f32) {
        self.zoom_level = zoom;
        self.zoom_x = x;
        self.zoom_y = y;
        self.zoom_width = w;
        self.zoom_height = h;
    }

    pub fn x_label(mut self, label: impl Into<String>) -> Self {
        self.x_label = label.into();
        self
    }

    pub fn y_label(mut self, label: impl Into<String>) -> Self {
        self.y_label = label.into();
        self
    }

    pub fn add_element(&mut self, element: InteractiveElement) {
        self.elements.push(element);
    }

    pub fn add_bar(
        &mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        value: f64,
        color: impl Into<String>,
    ) {
        let mut metadata = HashMap::new();
        metadata.insert("color".to_string(), color.into());
        self.elements.push(InteractiveElement {
            id: id.into(),
            label: label.into(),
            value,
            metadata,
            hover_data: HashMap::new(),
        });
    }

    pub fn add_bar_with_hover(
        &mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        value: f64,
        color: impl Into<String>,
        image_url: Option<String>,
        description: impl Into<String>,
    ) {
        let mut metadata = HashMap::new();
        metadata.insert("color".to_string(), color.into());
        let mut hover = HashMap::new();
        if let Some(img) = image_url {
            hover.insert("image".to_string(), img);
        }
        hover.insert("description".to_string(), description.into());
        self.elements.push(InteractiveElement {
            id: id.into(),
            label: label.into(),
            value,
            metadata,
            hover_data: hover,
        });
    }

    pub fn render_bar_chart(&self) -> String {
        let mut svg = String::new();
        let margin = 60.0;
        let chart_width = self.width - 2.0 * margin;
        let chart_height = self.height - 2.0 * margin - 40.0;
        let extra_height = 600.0;
        let total_height = self.height + extra_height;

        svg.push_str(&format!(
            "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" style=\"background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%); overflow: visible;\" viewBox=\"0 0 {} {}\">",
            self.width, total_height, self.width, total_height
        ));

        svg.push_str("  <defs>\n");
        svg.push_str("    <linearGradient id=\"barGrad\" x1=\"0%\" y1=\"0%\" x2=\"0%\" y2=\"100%\">\n");
        svg.push_str("      <stop offset=\"0%\" style=\"stop-color:#667eea;stop-opacity:1\" />\n");
        svg.push_str("      <stop offset=\"100%\" style=\"stop-color:#764ba2;stop-opacity:1\" />\n");
        svg.push_str("    </linearGradient>\n");
        svg.push_str("    <style type=\"text/css\">\n");
        svg.push_str("      .bar { cursor: pointer; transition: all 0.3s ease; filter: drop-shadow(0 2px 4px rgba(0,0,0,0.1)); }\n");
        svg.push_str("      .bar:hover { filter: drop-shadow(0 4px 8px rgba(0,0,0,0.2)); transform-origin: bottom; }\n");
        svg.push_str("      .tooltip-box { pointer-events: none; opacity: 0; transition: opacity 0.2s; }\n");
        svg.push_str("      .bar:hover ~ .tooltip-box { opacity: 1; }\n");
        svg.push_str("      .tooltip-text { font-size: 12px; font-weight: bold; fill: white; }\n");
        svg.push_str("      .grid-line { stroke: #e0e0e0; stroke-width: 1; opacity: 0.6; }\n");
        svg.push_str("    </style>\n");
        svg.push_str("  </defs>\n");

        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"30\" font-size=\"26\" font-weight=\"bold\" text-anchor=\"middle\" fill=\"#2c3e50\">{}</text>\n",
            self.width / 2.0,
            self.title
        ));

        let max_value = self.elements
            .iter()
            .map(|e| e.value)
            .fold(0.0, f64::max);

        if max_value > 0.0 {
            let bar_width = chart_width / self.elements.len() as f32;
            let padding = bar_width * 0.15;

            for i in 1..5 {
                let y = margin + chart_height - (i as f32 / 4.0) * chart_height;
                svg.push_str(&format!(
                    "  <line class=\"grid-line\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>\n",
                    margin, y, self.width - margin, y
                ));
            }

            for (idx, element) in self.elements.iter().enumerate() {
                let x_pos = margin + idx as f32 * bar_width + padding;
                let bar_height = (element.value as f32 / max_value as f32) * chart_height;
                let y_pos = margin + chart_height - bar_height;

                let color = element
                    .metadata
                    .get("color")
                    .map(|c| c.as_str())
                    .unwrap_or("#667eea");

                svg.push_str(&format!(
                    "  <g data-idx=\"{}\"><rect class=\"bar\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" rx=\"3\" opacity=\"0.85\"/>\n",
                    idx, x_pos, y_pos,
                    bar_width - 2.0 * padding,
                    bar_height,
                    color
                ));

                let tooltip_x = x_pos + (bar_width - 2.0 * padding) / 2.0;
                let tooltip_y = y_pos - 10.0;
                
                let _label_short = if element.label.len() > 20 {
                    format!("{}...", &element.label[..17])
                } else {
                    element.label.clone()
                };

                let has_image = element.hover_data.contains_key("image");
                let field_count = element.hover_data.iter().filter(|(k, _)| k.as_str() != "image").count();
                let tooltip_width = 520.0;
                let base_height = if has_image { 120.0 } else { 85.0 };
                let tooltip_height = base_height + (field_count as f32 * 35.0) + 65.0;
                
                let mut tooltip_top = tooltip_y - tooltip_height - 20.0;
                let mut tooltip_left = tooltip_x - tooltip_width / 2.0;
                
                if tooltip_top < 10.0 {
                    tooltip_top = tooltip_y + 25.0;
                }
                if tooltip_left < 5.0 {
                    tooltip_left = 5.0;
                }
                if tooltip_left + tooltip_width > self.width - 5.0 {
                    tooltip_left = self.width - tooltip_width - 5.0;
                }

                svg.push_str(&format!(
                    "    <rect class=\"tooltip-box\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"rgba(44, 62, 80, 0.98)\" rx=\"8\" />\n",
                    tooltip_left, tooltip_top, tooltip_width, tooltip_height
                ));
                
                svg.push_str(&format!(
                    "    <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"white\" opacity=\"0.08\" rx=\"6\" />\n",
                    tooltip_left + 8.0, tooltip_top + 8.0, tooltip_width - 16.0, tooltip_height - 16.0
                ));

                let mut hover_offset = 15.0;
                let has_image = element.hover_data.contains_key("image");
                
                if has_image {
                    if let Some(img_url) = element.hover_data.get("image") {
                        svg.push_str(&format!(
                            "    <image x=\"{}\" y=\"{}\" width=\"90\" height=\"90\" href=\"{}\" opacity=\"0.95\" />\n",
                            tooltip_left + tooltip_width / 2.0 - 45.0, tooltip_top + 15.0, img_url
                        ));
                    }
                    hover_offset = 110.0;
                }
                
                for (key, val) in &element.hover_data {
                    if key != "image" {
                        let text_val = if val.len() > 55 {
                            format!("{}...", &val[..52])
                        } else {
                            val.clone()
                        };
                        svg.push_str(&format!(
                            "    <text class=\"tooltip-text\" x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"12\" fill=\"white\" font-weight=\"600\">{}: {}</text>\n",
                            tooltip_left + tooltip_width / 2.0, tooltip_top + hover_offset,
                            key, text_val
                        ));
                        hover_offset += 35.0;
                    }
                }

                svg.push_str(&format!(
                    "    <text class=\"tooltip-text\" x=\"{}\" y=\"{}\" text-anchor=\"middle\" font-size=\"15\" font-weight=\"bold\" fill=\"white\">{}</text>\n",
                    tooltip_left + tooltip_width / 2.0, tooltip_top + tooltip_height - 38.0, element.label
                ));

                svg.push_str(&format!(
                    "    <text class=\"tooltip-text\" x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"#1abc9c\" font-weight=\"bold\" font-size=\"18\">{}</text>\n",
                    tooltip_left + tooltip_width / 2.0, tooltip_top + tooltip_height - 12.0, element.value as i32
                ));
                svg.push_str("  </g>\n");

                let label_short = if element.label.len() > 20 {
                    format!("{}...", &element.label[..17])
                } else {
                    element.label.clone()
                };

                svg.push_str(&format!(
                    "  <text x=\"{}\" y=\"{}\" font-size=\"11\" text-anchor=\"middle\" fill=\"#555\">{}</text>\n",
                    x_pos + (bar_width - 2.0 * padding) / 2.0,
                    margin + chart_height + 25.0,
                    label_short
                ));
            }
        }

        svg.push_str(&format!(
            "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#34495e\" stroke-width=\"2.5\"/>\n",
            margin, margin + chart_height,
            self.width - margin, margin + chart_height
        ));
        svg.push_str(&format!(
            "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#34495e\" stroke-width=\"2.5\"/>\n",
            margin, margin,
            margin, margin + chart_height
        ));

        svg.push_str("</svg>\n");
        svg
    }

    pub fn render_scatter_chart(&self) -> String {
        let mut svg = String::new();
        let margin = 60.0;
        let chart_width = self.width - 2.0 * margin;
        let chart_height = self.height - 2.0 * margin - 40.0;

        svg.push_str(&format!(
            "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" style=\"background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%);\">\n",
            self.width, self.height
        ));

        svg.push_str("  <defs>\n");
        svg.push_str("    <style type=\"text/css\">\n");
        svg.push_str("      .point { cursor: pointer; transition: all 0.3s ease; filter: drop-shadow(0 2px 4px rgba(0,0,0,0.1)); }\n");
        svg.push_str("      .point:hover { r: 9px; filter: drop-shadow(0 4px 8px rgba(0,0,0,0.2)); }\n");
        svg.push_str("      .scatter-tooltip { pointer-events: none; opacity: 0; transition: opacity 0.2s; }\n");
        svg.push_str("      .point:hover ~ .scatter-tooltip { opacity: 1; }\n");
        svg.push_str("      .grid-line { stroke: #e0e0e0; stroke-width: 1; opacity: 0.6; }\n");
        svg.push_str("    </style>\n");
        svg.push_str("  </defs>\n");

        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"30\" font-size=\"26\" font-weight=\"bold\" text-anchor=\"middle\" fill=\"#2c3e50\">{}</text>\n",
            self.width / 2.0,
            self.title
        ));

        let max_value = self.elements
            .iter()
            .map(|e| e.value)
            .fold(0.0, f64::max);

        if max_value > 0.0 {
            let radius = 7.0;

            for i in 1..5 {
                let y = margin + chart_height - (i as f32 / 4.0) * chart_height;
                svg.push_str(&format!(
                    "  <line class=\"grid-line\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>\n",
                    margin, y, self.width - margin, y
                ));
            }

            for (idx, element) in self.elements.iter().enumerate() {
                let x = margin + (idx as f32 / self.elements.len() as f32) * chart_width;
                let y = margin + chart_height - (element.value as f32 / max_value as f32) * chart_height;

                let color = element
                    .metadata
                    .get("color")
                    .map(|c| c.as_str())
                    .unwrap_or("#667eea");

                svg.push_str(&format!(
                    "  <g data-idx=\"{}\"><circle class=\"point\" cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" stroke=\"white\" stroke-width=\"2\"/>\n",
                    idx, x, y, radius, color
                ));

                svg.push_str(&format!(
                    "    <rect class=\"scatter-tooltip\" x=\"{}\" y=\"{}\" width=\"160\" height=\"50\" fill=\"rgba(44, 62, 80, 0.95)\" rx=\"5\" />\n",
                    x - 80.0, y - 55.0
                ));

                svg.push_str(&format!(
                    "    <text class=\"scatter-tooltip\" x=\"{}\" y=\"{}\" font-size=\"12\" font-weight=\"bold\" fill=\"white\" text-anchor=\"middle\">{}</text>\n",
                    x, y - 35.0, element.label
                ));

                svg.push_str(&format!(
                    "    <text class=\"scatter-tooltip\" x=\"{}\" y=\"{}\" font-size=\"14\" font-weight=\"bold\" fill=\"#1abc9c\" text-anchor=\"middle\">{}</text>\n",
                    x, y - 18.0, element.value as i32
                ));
                svg.push_str("  </g>\n");
            }
        }

        svg.push_str(&format!(
            "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#34495e\" stroke-width=\"2.5\"/>\n",
            margin, margin + chart_height,
            self.width - margin, margin + chart_height
        ));
        svg.push_str(&format!(
            "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#34495e\" stroke-width=\"2.5\"/>\n",
            margin, margin,
            margin, margin + chart_height
        ));

        svg.push_str("</svg>\n");
        svg
    }

    pub fn render_line_chart(&self) -> String {
        let mut svg = String::new();
        let margin = 60.0;
        let chart_width = self.width - 2.0 * margin;
        let chart_height = self.height - 2.0 * margin - 40.0;

        svg.push_str(&format!(
            "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" style=\"background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%);\">\n",
            self.width, self.height
        ));

        svg.push_str("  <defs>\n");
        svg.push_str("    <style type=\"text/css\">\n");
        svg.push_str("      .line { fill: none; stroke-width: 2.5; stroke-linecap: round; stroke-linejoin: round; }\n");
        svg.push_str("      .line-point { cursor: pointer; transition: all 0.3s ease; filter: drop-shadow(0 2px 4px rgba(0,0,0,0.1)); }\n");
        svg.push_str("      .line-point:hover { r: 7px; filter: drop-shadow(0 4px 8px rgba(0,0,0,0.2)); }\n");
        svg.push_str("      .line-tooltip { pointer-events: none; opacity: 0; transition: opacity 0.2s; }\n");
        svg.push_str("      .line-point:hover ~ .line-tooltip { opacity: 1; }\n");
        svg.push_str("      .grid-line { stroke: #e0e0e0; stroke-width: 1; opacity: 0.6; }\n");
        svg.push_str("    </style>\n");
        svg.push_str("  </defs>\n");

        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"30\" font-size=\"26\" font-weight=\"bold\" text-anchor=\"middle\" fill=\"#2c3e50\">{}</text>\n",
            self.width / 2.0,
            self.title
        ));

        let max_value = self.elements
            .iter()
            .map(|e| e.value)
            .fold(0.0, f64::max);

        if max_value > 0.0 && !self.elements.is_empty() {
            for i in 1..5 {
                let y = margin + chart_height - (i as f32 / 4.0) * chart_height;
                svg.push_str(&format!(
                    "  <line class=\"grid-line\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>\n",
                    margin, y, self.width - margin, y
                ));
            }

            let mut path_data = String::new();

            for (idx, element) in self.elements.iter().enumerate() {
                let x = margin + (idx as f32 / (self.elements.len() - 1).max(1) as f32) * chart_width;
                let y = margin + chart_height - (element.value as f32 / max_value as f32) * chart_height;

                if idx == 0 {
                    path_data.push_str(&format!("M {} {}", x, y));
                } else {
                    path_data.push_str(&format!(" L {} {}", x, y));
                }
            }

            let color = self.elements[0]
                .metadata
                .get("color")
                .map(|c| c.as_str())
                .unwrap_or("#667eea");

            svg.push_str(&format!(
                "  <path class=\"line\" d=\"{}\" stroke=\"{}\" opacity=\"0.8\"/>\n",
                path_data, color
            ));

            for (idx, element) in self.elements.iter().enumerate() {
                let x = margin + (idx as f32 / (self.elements.len() - 1).max(1) as f32) * chart_width;
                let y = margin + chart_height - (element.value as f32 / max_value as f32) * chart_height;

                let color = element
                    .metadata
                    .get("color")
                    .map(|c| c.as_str())
                    .unwrap_or("#667eea");

                svg.push_str(&format!(
                    "  <g data-idx=\"{}\"><circle class=\"line-point\" cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"{}\" stroke=\"white\" stroke-width=\"2\"/>\n",
                    idx, x, y, color
                ));

                svg.push_str(&format!(
                    "    <rect class=\"line-tooltip\" x=\"{}\" y=\"{}\" width=\"160\" height=\"50\" fill=\"rgba(44, 62, 80, 0.95)\" rx=\"5\" />\n",
                    x - 80.0, y - 55.0
                ));

                svg.push_str(&format!(
                    "    <text class=\"line-tooltip\" x=\"{}\" y=\"{}\" font-size=\"12\" font-weight=\"bold\" fill=\"white\" text-anchor=\"middle\">{}</text>\n",
                    x, y - 35.0, element.label
                ));

                svg.push_str(&format!(
                    "    <text class=\"line-tooltip\" x=\"{}\" y=\"{}\" font-size=\"14\" font-weight=\"bold\" fill=\"#1abc9c\" text-anchor=\"middle\">{}</text>\n",
                    x, y - 18.0, element.value as i32
                ));
                svg.push_str("  </g>\n");
            }
        }

        svg.push_str(&format!(
            "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#34495e\" stroke-width=\"2.5\"/>\n",
            margin, margin + chart_height,
            self.width - margin, margin + chart_height
        ));
        svg.push_str(&format!(
            "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#34495e\" stroke-width=\"2.5\"/>\n",
            margin, margin,
            margin, margin + chart_height
        ));

        svg.push_str("</svg>\n");
        svg
    }
}
