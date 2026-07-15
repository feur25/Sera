use seraplot_macros::sera_impl;
use crate::Chart;

fn extract_svg_body(html: &str) -> Option<String> {
    let start = html.find("<svg")?;
    let end = html.rfind("</svg>")? + 6;
    Some(html[start..end].to_string())
}

fn svg_dim(svg: &str) -> (f64, f64) {
    let w = svg.find("width=\"")
        .and_then(|i| svg[i + 7..].find('"').map(|e| svg[i + 7..i + 7 + e].parse::<f64>().ok()))
        .flatten()
        .unwrap_or(900.0);
    let h = svg.find("height=\"")
        .and_then(|i| svg[i + 8..].find('"').map(|e| svg[i + 8..i + 8 + e].parse::<f64>().ok()))
        .flatten()
        .unwrap_or(500.0);
    (w, h)
}

#[sera_impl]
impl Chart {
    #[sera_doc(
        category = "chart_method",
        file = "charts/compose.md",
        en = "Overlays another chart's SVG on top of this chart using an SVG blend mode. Both charts must have compatible dimensions for best results.",
        fr = "Superpose le SVG d'un autre graphique sur celui-ci avec un mode de fusion SVG.",
        aliases("overlay", "blend", "layer_on"),
        param(name = "other", ty = "Chart", en = "The chart to overlay.", fr = "Le graphique à superposer."),
        param(name = "blend", ty = "str | None", en = "CSS mix-blend-mode (e.g. 'multiply', 'screen', 'overlay'). Default 'screen'.", fr = "Mode de fusion CSS. Défaut 'screen'.")
    )]
    #[sera_sig(other, blend = None)]
    pub fn compose(&self, other: &Chart, blend: Option<&str>) -> Chart {
        let mode = blend.unwrap_or("screen");
        let other_svg = match extract_svg_body(&other.html) {
            Some(s) => s,
            None => return self.propagate(self.html.clone()),
        };
        let (ow, oh) = svg_dim(&other_svg);
        let inject = format!(
            "<g style=\"mix-blend-mode:{};pointer-events:none;\" width=\"{}\" height=\"{}\">{}</g></svg>",
            mode, ow, oh, &other_svg[other_svg.find('>').map(|i| i + 1).unwrap_or(0)..other_svg.len() - 6]
        );
        let html = self.html.replacen("</svg>", &inject, 1);
        self.propagate(html)
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/compose.md",
        en = "Injects a background image (URL or data URI) behind the chart SVG content.",
        fr = "Injecte une image de fond (URL ou data URI) derrière le contenu SVG du graphique.",
        aliases("bg_image", "background_image"),
        param(name = "url", ty = "str", en = "Image URL or base64 data URI.", fr = "URL ou data URI base64."),
        param(name = "opacity", ty = "float", en = "Opacity 0.0–1.0. Default 0.35.", fr = "Opacité 0.0–1.0. Défaut 0.35.")
    )]
    #[sera_sig(url, opacity = 0.35)]
    pub fn image_bg(&self, url: &str, opacity: f64) -> Chart {
        let op = opacity.clamp(0.0, 1.0);
        let insert = format!(
            "<image href=\"{}\" x=\"0\" y=\"0\" width=\"100%\" height=\"100%\" preserveAspectRatio=\"xMidYMid slice\" opacity=\"{:.2}\"/>",
            url, op
        );
        let html = if let Some(pos) = self.html.find("<svg") {
            if let Some(end) = self.html[pos..].find('>') {
                let after_open = pos + end + 1;
                let mut h = self.html.clone();
                h.insert_str(after_open, &insert);
                h
            } else {
                self.html.clone()
            }
        } else {
            self.html.clone()
        };
        self.propagate(html)
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/compose.md",
        en = "Masks the chart to a circle shape using an SVG clipPath.",
        fr = "Découpe le graphique en cercle via un clipPath SVG.",
        aliases("circle_mask", "round_mask", "clip_circle"),
        param(name = "cx", ty = "float | None", en = "Circle center X as fraction of width (0–1). Default 0.5.", fr = "Centre X en fraction de la largeur. Défaut 0.5."),
        param(name = "cy", ty = "float | None", en = "Circle center Y as fraction of height (0–1). Default 0.5.", fr = "Centre Y en fraction de la hauteur. Défaut 0.5."),
        param(name = "r", ty = "float | None", en = "Radius as fraction of min(width,height)/2. Default 0.48.", fr = "Rayon en fraction de min(l,h)/2. Défaut 0.48.")
    )]
    #[sera_sig(cx = None, cy = None, r = None)]
    pub fn mask_circle(&self, cx: Option<f64>, cy: Option<f64>, r: Option<f64>) -> Chart {
        let cx = cx.unwrap_or(0.5).clamp(0.0, 1.0);
        let cy = cy.unwrap_or(0.5).clamp(0.0, 1.0);
        let r = r.unwrap_or(0.48).clamp(0.0, 1.0);
        let id = "sp-cm";
        let defs = format!(
            "<defs><clipPath id=\"{}\"><circle cx=\"{:.1}%\" cy=\"{:.1}%\" r=\"{:.1}%\"/></clipPath></defs>",
            id, cx * 100.0, cy * 100.0, r * 100.0
        );
        let html = if let Some(pos) = self.html.find("<svg") {
            if let Some(end) = self.html[pos..].find('>') {
                let after_open = pos + end + 1;
                let mut h = self.html.clone();
                h.insert_str(after_open, &defs);
                if let Some(start_body) = h.find("<svg") {
                    if let Some(end_tag) = h[start_body..].find('>') {
                        let tag_end = start_body + end_tag;
                        let old_tag = &h[start_body..=tag_end];
                        if !old_tag.contains("clip-path") {
                            let new_tag = format!("{} clip-path=\"url(#{})\"", &old_tag[..old_tag.len()-1], id);
                            h = h.replacen(old_tag, &(new_tag + ">"), 1);
                        }
                    }
                }
                h
            } else {
                self.html.clone()
            }
        } else {
            self.html.clone()
        };
        self.propagate(html)
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/compose.md",
        en = "Masks the chart to a regular polygon (triangle, hexagon, star…) using an SVG clipPath.",
        fr = "Découpe le graphique selon un polygone régulier via clipPath SVG.",
        aliases("polygon_mask", "hex_mask", "shape_mask"),
        param(name = "sides", ty = "int", en = "Number of polygon sides (3=triangle, 6=hex, etc.).", fr = "Nombre de côtés (3=triangle, 6=hexagone…)."),
        param(name = "rotation", ty = "float", en = "Rotation in degrees. Default 0.", fr = "Rotation en degrés. Défaut 0.")
    )]
    #[sera_sig(sides = 6, rotation = 0.0)]
    pub fn mask_poly(&self, sides: usize, rotation: f64) -> Chart {
        let n = sides.max(3);
        let step = std::f64::consts::TAU / n as f64;
        let rot_r = rotation.to_radians();
        let points: String = (0..n)
            .map(|i| {
                let a = step * i as f64 + rot_r;
                let x = 50.0 + 48.0 * a.cos();
                let y = 50.0 + 48.0 * a.sin();
                format!("{:.2},{:.2}", x, y)
            })
            .collect::<Vec<_>>()
            .join(" ");
        let id = "sp-poly";
        let defs = format!(
            "<defs><clipPath id=\"{}\" clipPathUnits=\"objectBoundingBox\"><polygon points=\"{}\"/></clipPath></defs>",
            id,
            points.split(' ').map(|p| {
                let mut parts = p.splitn(2, ',');
                let x: f64 = parts.next().unwrap_or("0").parse().unwrap_or(0.0) / 100.0;
                let y: f64 = parts.next().unwrap_or("0").parse().unwrap_or(0.0) / 100.0;
                format!("{:.4},{:.4}", x, y)
            }).collect::<Vec<_>>().join(" ")
        );
        let html = if let Some(pos) = self.html.find("<svg") {
            if let Some(end) = self.html[pos..].find('>') {
                let after_open = pos + end + 1;
                let mut h = self.html.clone();
                h.insert_str(after_open, &defs);
                h = h.replacen("<svg", &format!("<svg clip-path=\"url(#{})\"", id), 1);
                h
            } else {
                self.html.clone()
            }
        } else {
            self.html.clone()
        };
        self.propagate(html)
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/compose.md",
        en = "Adds a decorative particle texture layer over the chart — tiny scattered dots that evoke Visual Cinnamon's signature textural depth.",
        fr = "Ajoute une couche de particules décoratives sur le graphique — petits points épars pour une profondeur visuelle signature.",
        aliases("particles", "texture", "grain", "dots"),
        param(name = "density", ty = "float", en = "Particle density 0.0–1.0. Default 0.4.", fr = "Densité de particules 0.0–1.0. Défaut 0.4."),
        param(name = "seed", ty = "int", en = "Random seed for reproducibility. Default 42.", fr = "Graine aléatoire. Défaut 42."),
        param(name = "color", ty = "str | None", en = "Particle color. Default '#ffffff'.", fr = "Couleur des particules. Défaut '#ffffff'.")
    )]
    #[sera_sig(density = 0.4, seed = 42, color = None)]
    pub fn flourish(&self, density: f64, seed: u64, color: Option<&str>) -> Chart {
        let col = color.unwrap_or("#ffffff");
        let d = density.clamp(0.0, 1.0);
        let count = (d * 300.0) as u64;
        let mut lcg = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let mut circles = String::with_capacity(count as usize * 60);
        for _ in 0..count {
            lcg = lcg.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let x = (lcg >> 33) as f64 / (u32::MAX as f64) * 100.0;
            lcg = lcg.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let y = (lcg >> 33) as f64 / (u32::MAX as f64) * 100.0;
            lcg = lcg.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let r = 0.3 + (lcg >> 33) as f64 / (u32::MAX as f64) * 1.2;
            lcg = lcg.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let op = 0.08 + (lcg >> 33) as f64 / (u32::MAX as f64) * 0.35;
            circles.push_str(&format!(
                "<circle cx=\"{:.1}%\" cy=\"{:.1}%\" r=\"{:.2}\" fill=\"{}\" opacity=\"{:.2}\" pointer-events=\"none\"/>",
                x, y, r, col, op
            ));
        }
        let layer = format!("<g class=\"sp-flourish\" style=\"pointer-events:none\">{}</g></svg>", circles);
        let html = self.html.replacen("</svg>", &layer, 1);
        self.propagate(html)
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/compose.md",
        en = "Stamps large artistic text over the chart — watermark, label, or Visual Cinnamon–style annotation.",
        fr = "Tampon de texte artistique grand format sur le graphique — filigrane, label ou annotation style Visual Cinnamon.",
        aliases("watermark_text", "label_stamp", "big_text"),
        param(name = "text", ty = "str", en = "Text content to stamp.", fr = "Texte à tamponner."),
        param(name = "x", ty = "float", en = "X position as percent of SVG width. Default 50.", fr = "Position X en % de la largeur SVG. Défaut 50."),
        param(name = "y", ty = "float", en = "Y position as percent of SVG height. Default 50.", fr = "Position Y en % de la hauteur SVG. Défaut 50."),
        param(name = "size", ty = "float", en = "Font size in px. Default 72.", fr = "Taille de police en px. Défaut 72."),
        param(name = "color", ty = "str", en = "Text color. Default '#ffffff'.", fr = "Couleur du texte. Défaut '#ffffff'."),
        param(name = "opacity", ty = "float", en = "Opacity 0.0–1.0. Default 0.08.", fr = "Opacité 0.0–1.0. Défaut 0.08.")
    )]
    #[sera_sig(text, x = 50.0, y = 50.0, size = 72.0, color = "#ffffff", opacity = 0.08)]
    pub fn stamp(&self, text: &str, x: f64, y: f64, size: f64, color: &str, opacity: f64) -> Chart {
        let op = opacity.clamp(0.0, 1.0);
        let escaped = text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
        let node = format!(
            "<text x=\"{:.1}%\" y=\"{:.1}%\" text-anchor=\"middle\" dominant-baseline=\"middle\" font-size=\"{:.0}\" font-weight=\"900\" fill=\"{}\" opacity=\"{:.3}\" pointer-events=\"none\" style=\"user-select:none;letter-spacing:0.08em;\">{}</text></svg>",
            x, y, size, color, op, escaped
        );
        let html = self.html.replacen("</svg>", &node, 1);
        self.propagate(html)
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/compose.md",
        en = "Animates the chart background through a gradient transition — cinematic color morphing that brings data to life.",
        fr = "Anime le fond du graphique via une transition de dégradé — morphing cinématique qui donne vie aux données.",
        aliases("gradient_bg", "animated_bg", "morph_gradient"),
        param(name = "from_color", ty = "str", en = "Start color (CSS). Default '#0f172a'.", fr = "Couleur de départ (CSS). Défaut '#0f172a'."),
        param(name = "to_color", ty = "str", en = "End color (CSS). Default '#1e1b4b'.", fr = "Couleur d'arrivée (CSS). Défaut '#1e1b4b'."),
        param(name = "duration", ty = "float", en = "Animation duration in seconds. Default 4.0.", fr = "Durée de l'animation en secondes. Défaut 4.0.")
    )]
    #[sera_sig(from_color = "#0f172a", to_color = "#1e1b4b", duration = 4.0)]
    pub fn morph_bg(&self, from_color: &str, to_color: &str, duration: f64) -> Chart {
        let dur = duration.max(0.5);
        let id = "sp-morph-bg";
        let defs = format!(
            "<defs><linearGradient id=\"{id}\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"100%\"><stop offset=\"0%\"><animate attributeName=\"stop-color\" values=\"{from};{to};{from}\" dur=\"{dur}s\" repeatCount=\"indefinite\"/></stop><stop offset=\"100%\"><animate attributeName=\"stop-color\" values=\"{to};{from};{to}\" dur=\"{dur}s\" repeatCount=\"indefinite\"/></stop></linearGradient></defs><rect width=\"100%\" height=\"100%\" fill=\"url(#{id})\" style=\"pointer-events:none\"/>",
            id = id, from = from_color, to = to_color, dur = dur
        );
        let html = if let Some(pos) = self.html.find("<svg") {
            if let Some(end) = self.html[pos..].find('>') {
                let after_open = pos + end + 1;
                let mut h = self.html.clone();
                h.insert_str(after_open, &defs);
                h
            } else {
                self.html.clone()
            }
        } else {
            self.html.clone()
        };
        self.propagate(html)
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/compose.md",
        en = "Places two charts side-by-side in a single HTML container — the fundamental Visual Cinnamon multi-panel layout.",
        fr = "Place deux graphiques côte à côte dans un seul conteneur HTML — le layout multi-panneau fondamental.",
        aliases("side_by_side", "columns", "two_col"),
        param(name = "other", ty = "Chart", en = "The second chart to place on the right.", fr = "Le second graphique à placer à droite."),
        param(name = "gap", ty = "int", en = "Gap between charts in pixels. Default 16.", fr = "Écart entre graphiques en pixels. Défaut 16.")
    )]
    #[sera_sig(other, gap = 16)]
    pub fn hstack(&self, other: &Chart, gap: i32) -> Chart {
        let html = format!(
            "<!DOCTYPE html><html><head><meta charset='utf-8'><style>body{{margin:0;background:#0a0a0f;}}.sp-hstack{{display:flex;flex-direction:row;gap:{}px;width:100%;box-sizing:border-box;}}.sp-hstack>iframe{{flex:1;border:none;min-width:0;}}</style></head><body><div class='sp-hstack'><iframe srcdoc=\"{}\" scrolling='no'></iframe><iframe srcdoc=\"{}\" scrolling='no'></iframe></div></body></html>",
            gap,
            self.html.replace('"', "&quot;"),
            other.html.replace('"', "&quot;")
        );
        self.propagate(html)
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/compose.md",
        en = "Stacks two charts vertically in a single HTML container.",
        fr = "Empile deux graphiques verticalement dans un seul conteneur HTML.",
        aliases("stack", "rows", "two_row"),
        param(name = "other", ty = "Chart", en = "The second chart to place below.", fr = "Le second graphique à placer en dessous."),
        param(name = "gap", ty = "int", en = "Gap between charts in pixels. Default 16.", fr = "Écart entre graphiques en pixels. Défaut 16.")
    )]
    #[sera_sig(other, gap = 16)]
    pub fn vstack(&self, other: &Chart, gap: i32) -> Chart {
        let html = format!(
            "<!DOCTYPE html><html><head><meta charset='utf-8'><style>body{{margin:0;background:#0a0a0f;}}.sp-vstack{{display:flex;flex-direction:column;gap:{}px;width:100%;box-sizing:border-box;}}.sp-vstack>iframe{{border:none;width:100%;}}</style></head><body><div class='sp-vstack'><iframe srcdoc=\"{}\" scrolling='no'></iframe><iframe srcdoc=\"{}\" scrolling='no'></iframe></div></body></html>",
            gap,
            self.html.replace('"', "&quot;"),
            other.html.replace('"', "&quot;")
        );
        self.propagate(html)
    }
}
