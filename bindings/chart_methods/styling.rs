use seraplot_macros::{sera_impl, sera_doc, sera_sig, sera_python_skip};
#[allow(unused_imports)]
use crate::{Chart, json_str, apply_neon_bloom, apply_void};
#[allow(unused_imports)]
use super::apply::*;
#[allow(unused_imports)]
use super::js::*;
#[sera_impl]
impl Chart {
    #[sera_doc(
        category = "chart_method",
        aliases("css", "custom_css"),
        file = "charts/chart.md",
        en = "Injects a raw CSS string into the chart's <head> element.",
        fr = "Injecte une chaîne CSS brute dans l'élément <head> du graphique.",
        param(
            name = "css",
            ty = "str",
            en = "Raw CSS rules to inject.",
            fr = "Règles CSS brutes à injecter."
        )
    )]
    #[sera_sig(css)]
    pub fn inject_css(&self, css: &str) -> Chart {
        self.propagate(
            self.html
                .replacen("</head>", &format!("<style>{css}</style></head>"), 1),
        )
    }

    #[sera_doc(
        category = "chart_method",
        aliases("js", "custom_js"),
        file = "charts/chart.md",
        en = "Injects a raw JavaScript string into the chart's <body> element.",
        fr = "Injecte une chaîne JavaScript brute dans l'élément <body> du graphique.",
        param(
            name = "js",
            ty = "str",
            en = "Raw JavaScript code to inject.",
            fr = "Code JavaScript brut à injecter."
        )
    )]
    #[sera_sig(js)]
    pub fn inject_js(&self, js: &str) -> Chart {
        self.propagate(
            self.html
                .replacen("</body>", &format!("<script>{js}</script></body>"), 1),
        )
    }

    #[sera_doc(
        category = "chart_method",
        aliases("no_tooltip"),
        file = "charts/chart.md",
        en = "Disables the hover tooltip and removes hover highlighting on data elements.",
        fr = "Désactive l'infobulle au survol et supprime le surlignage des éléments au survol."
    )]
    pub fn no_hover(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>#sp-tip{display:none!important}[data-idx]{pointer-events:none!important}[data-idx]:hover{filter:none!important}</style></head>", 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("desat", "grayscale_elements"),
        file = "charts/chart.md",
        en = "Reduces the color saturation of specific data elements (by index) or all elements if no indices given. factor=0.0 is full grayscale, 1.0 is original color.",
        fr = "Réduit la saturation de certains éléments de données (par indice) ou de tous si aucun indice fourni. factor=0.0 = niveaux de gris, 1.0 = couleur originale."
    )]
    #[sera_sig(indices = None, factor = 0.15)]
    pub fn desaturate(&self, indices: Option<Vec<usize>>, factor: f64) -> Chart {
        let idx_json = match &indices {
            Some(v) => format!("[{}]", v.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",")),
            None => "null".into(),
        };
        let js = format!("window.__sp_desat__={{i:{},f:{:.2}}};{}", idx_json, factor.clamp(0.0, 1.0), SP_DESATURATE_JS);
        self.propagate(self.html.replacen("</body>", &format!("<script>{}</script></body>", js), 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("tufte_bars", "unit_bars", "bars_cut"),
        file = "charts/chart.md",
        en = "Overlays white horizontal cuts on every bar at regular pixel intervals, creating a segmented / Tufte data-ink effect.",
        fr = "Superpose des coupures blanches horizontales sur chaque barre à intervalles réguliers, créant un effet de barres segmentées / Tufte data-ink.",
        param(
            name = "step",
            ty = "float | None",
            en = "Pixel height of each segment. None = auto (plot-height / 12).",
            fr = "Hauteur en pixels de chaque segment. None = auto (hauteur-graphique / 12)."
        ),
        param(
            name = "gap",
            ty = "int",
            en = "Pixel height of the white gap between segments (default 2).",
            fr = "Hauteur en pixels de la coupure blanche entre les segments (défaut 2)."
        ),
        param(
            name = "color",
            ty = "str | None",
            en = "Cut color (default '#ffffff'). Use the chart background color to blend seamlessly.",
            fr = "Couleur de la coupure (défaut '#ffffff'). Utiliser la couleur de fond du graphique pour un rendu homogène."
        )
    )]
    #[sera_sig(step = None, gap = 2, color = None)]
    pub fn cut_bars(&self, step: Option<f64>, gap: i32, color: Option<&str>) -> Chart {
        let sp = step.filter(|&s| s > 0.0).unwrap_or(0.0);
        let cfg = format!(
            "window.__sp_cut_bars__={{sp:{},gp:{},c:{}}};",
            sp, gap.max(1), json_str(color.unwrap_or("#ffffff"))
        );
        self.propagate(self.html.replacen("</body>", &format!("<script>{}{}</script></body>", cfg, SP_CUT_BARS_JS), 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("density_color", "gradient_by_value"),
        file = "charts/chart.md",
        en = "Colors each data element by its value on a density gradient: low values → blue/indigo, high values → yellow/red.",
        fr = "Colore chaque élément de données selon sa valeur sur un gradient de densité : faibles valeurs → bleu/indigo, hautes valeurs → jaune/rouge."
    )]
    pub fn color_density(&self) -> Chart {
        self.propagate(self.html.replacen("</body>", &format!("<script>{}</script></body>", SP_COLOR_DENSITY_JS), 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("animation"),
        file = "charts/chart.md",
        en = "Adds a staggered entry animation to data elements (bars, circles, areas).",
        fr = "Ajoute une animation d'entrée décalée aux éléments de données (barres, cercles, zones).",
        param(
            name = "duration",
            ty = "int",
            en = "Animation duration in milliseconds. Default: 300.",
            fr = "Durée de l'animation en millisecondes. Défaut: 300."
        )
    )]
    #[sera_sig(duration = 300)]
    pub fn animate(&self, duration: i32) -> Chart {
        let css = format!("<style>@keyframes sp-in{{from{{opacity:0;transform:translateY(8px)}}to{{opacity:1;transform:none}}}}svg rect[data-idx],svg circle[data-idx],svg path.sp-area{{animation:sp-in {}ms ease-out both}}</style></head>", duration);
        let js = "<script>(function(){if(window.__spa__)return;window.__spa__=1;var els=document.querySelectorAll('svg [data-idx]');for(var i=0;i<els.length;i++)els[i].style.animationDelay=i*30+'ms';})();</script></body>";
        self.propagate(
            self.html
                .replacen("</head>", &css, 1)
                .replacen("</body>", js, 1),
        )
    }

    #[sera_doc(
        category = "chart_method",
        aliases("radius", "corners"),
        file = "charts/chart.md",
        en = "Applies a CSS border-radius to the chart container element.",
        fr = "Applique un border-radius CSS à l'élément conteneur du graphique.",
        param(
            name = "px",
            ty = "int",
            en = "Corner radius in pixels.",
            fr = "Rayon des coins en pixels."
        )
    )]
    #[sera_sig(px)]
    pub fn border_radius(&self, px: i32) -> Chart {
        self.propagate(self.html.replacen("</head>", &format!("<style>[id^='spp'],.c3w{{border-radius:{}px!important;overflow:hidden}}</style></head>", px), 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("opacity", "alpha"),
        file = "charts/chart.md",
        en = "Sets the opacity of all data elements (bars, circles, areas) in the chart.",
        fr = "Définit l'opacité de tous les éléments de données (barres, cercles, zones) du graphique.",
        param(
            name = "value",
            ty = "float",
            en = "Opacity between 0.0 (invisible) and 1.0 (fully opaque).",
            fr = "Opacité entre 0.0 (invisible) et 1.0 (totalement opaque)."
        )
    )]
    #[sera_sig(value)]
    pub fn set_opacity(&self, value: f64) -> Chart {
        let v = value.clamp(0.0, 1.0);
        self.propagate(self.html.replacen("</head>", &format!("<style>svg rect[data-idx],svg circle[data-idx],svg path.sp-area{{opacity:{}!important}}</style></head>", v), 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("auto_text", "data_labels"),
        file = "charts/chart.md",
        en = "Overlays data value labels on all chart elements. Supports format strings and positioning.",
        fr = "Superpose des étiquettes de valeurs de données sur tous les éléments du graphique. Supporte les chaînes de format et le positionnement.",
        param(
            name = "format",
            ty = "str | None",
            en = "Format string (e.g. '.2f', '.0%') or empty string for auto. None disables.",
            fr = "Chaîne de format (ex: '.2f', '.0%') ou chaîne vide pour automatique. None désactive."
        ),
        param(
            name = "position",
            ty = "str | None",
            en = "Label position: 'auto', 'inside', 'outside'.",
            fr = "Position de l'étiquette: 'auto', 'inside', 'outside'."
        ),
        param(
            name = "angle",
            ty = "int | None",
            en = "Label rotation angle in degrees.",
            fr = "Angle de rotation des étiquettes en degrés."
        ),
        param(
            name = "font_size",
            ty = "int | None",
            en = "Font size of the data labels in pixels.",
            fr = "Taille de police des étiquettes de données en pixels."
        ),
        param(
            name = "color",
            ty = "str | None",
            en = "Color of the data labels.",
            fr = "Couleur des étiquettes de données."
        )
    )]
    #[sera_sig(format=None, position=None, angle=None, font_size=None, color=None)]
    pub fn text_auto(&self, format: Option<&str>, position: Option<&str>, angle: Option<i32>, font_size: Option<i32>, color: Option<&str>) -> Chart {
        let mut opts = String::from("window.__sp_text__=Object.assign(window.__sp_text__||{},{");
        if let Some(f) = format {
            opts.push_str(&format!("format:{},", json_str(f)));
        }
        if let Some(p) = position {
            opts.push_str(&format!("position:{},", json_str(p)));
        }
        if let Some(a) = angle {
            opts.push_str(&format!("angle:{},", a));
        }
        if let Some(fs) = font_size {
            opts.push_str(&format!("font_size:{},", fs));
        }
        if let Some(c) = color {
            opts.push_str(&format!("color:{},", json_str(c)));
        }
        opts.push_str("});");
        let snippet = format!("<script>{}{}</script></body>", opts, SP_TEXT_JS);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("font_text"),
        file = "charts/chart.md",
        en = "Sets font family, size, and color for data value labels.",
        fr = "Définit la famille de polices, la taille et la couleur des étiquettes de valeurs de données.",
        param(
            name = "family",
            ty = "str | None",
            en = "Font family name.",
            fr = "Nom de la famille de polices."
        ),
        param(
            name = "size",
            ty = "int | None",
            en = "Font size in pixels.",
            fr = "Taille de police en pixels."
        ),
        param(
            name = "color",
            ty = "str | None",
            en = "Label text color.",
            fr = "Couleur du texte des étiquettes."
        )
    )]
    #[sera_sig(family=None, size=None, color=None)]
    pub fn text_font(&self, family: Option<&str>, size: Option<i32>, color: Option<&str>) -> Chart {
        let mut opts = String::from("window.__sp_text__=Object.assign(window.__sp_text__||{},{");
        if let Some(f) = family {
            opts.push_str(&format!("font_family:{},", json_str(f)));
        }
        if let Some(s) = size {
            opts.push_str(&format!("font_size:{},", s));
        }
        if let Some(c) = color {
            opts.push_str(&format!("color:{},", json_str(c)));
        }
        opts.push_str("});");
        let snippet = format!("<script>{}{}</script></body>", opts, SP_TEXT_JS);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("no_spines"),
        file = "charts/chart.md",
        en = "Removes the axis spine lines, keeping ticks and labels (seaborn-style despine).",
        fr = "Retire les traits d'axe (spines) tout en gardant les graduations et les libellés (despine façon seaborn)."
    )]
    pub fn despine(&self) -> Chart {
        self.propagate(apply_despine(self.html.clone()))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("wm"),
        file = "charts/chart.md",
        en = "Overlays a large diagonal watermark text across the whole chart, for branding or draft marks.",
        fr = "Superpose un grand texte en filigrane diagonal sur tout le graphique, pour le branding ou les brouillons.",
        param(
            name = "text",
            ty = "str",
            en = "Watermark text.",
            fr = "Texte du filigrane."
        ),
        param(
            name = "opacity",
            ty = "float",
            en = "Watermark opacity between 0.0 and 1.0. Default: 0.08.",
            fr = "Opacité du filigrane entre 0.0 et 1.0. Défaut : 0.08."
        )
    )]
    #[sera_sig(text, opacity = 0.08)]
    pub fn watermark(&self, text: &str, opacity: f64) -> Chart {
        self.propagate(apply_watermark(self.html.clone(), text, opacity))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("footnote"),
        file = "charts/chart.md",
        en = "Adds a small footnote caption centered below the chart.",
        fr = "Ajoute une petite légende de bas de page centrée sous le graphique.",
        param(
            name = "text",
            ty = "str",
            en = "Caption text.",
            fr = "Texte de la légende."
        )
    )]
    #[sera_sig(text)]
    pub fn caption(&self, text: &str) -> Chart {
        self.propagate(apply_caption(self.html.clone(), text))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("neon"),
        file = "charts/chart.md",
        en = "Adds a neon drop-shadow glow behind every data element (bars, lines, points).",
        fr = "Ajoute une lueur néon (drop-shadow) derrière chaque élément de données (barres, lignes, points).",
        param(
            name = "color",
            ty = "str | None",
            en = "Glow color. Defaults to the chart's accent color (#6366f1).",
            fr = "Couleur de la lueur. Par défaut, la couleur d'accent du graphique (#6366f1)."
        )
    )]
    #[sera_sig(color = None)]
    pub fn glow(&self, color: Option<&str>) -> Chart {
        self.propagate(apply_glow(self.html.clone(), color.unwrap_or("#6366f1")))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("sub"),
        file = "charts/chart.md",
        en = "Adds a small subtitle directly under the chart title.",
        fr = "Ajoute un petit sous-titre directement sous le titre du graphique.",
        param(
            name = "text",
            ty = "str",
            en = "Subtitle text.",
            fr = "Texte du sous-titre."
        )
    )]
    #[sera_sig(text)]
    pub fn subtitle(&self, text: &str) -> Chart {
        self.propagate(apply_subtitle(self.html.clone(), text))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("drop_shadow"),
        file = "charts/chart.md",
        en = "Adds a soft drop-shadow behind the whole chart container.",
        fr = "Ajoute une ombre douce derrière tout le conteneur du graphique.",
        param(
            name = "blur",
            ty = "int",
            en = "Shadow blur radius in pixels. Default: 24.",
            fr = "Rayon de flou de l'ombre en pixels. Défaut : 24."
        ),
        param(
            name = "color",
            ty = "str | None",
            en = "Shadow color. Defaults to a neutral dark shadow.",
            fr = "Couleur de l'ombre. Par défaut, une ombre neutre sombre."
        )
    )]
    #[sera_sig(blur = 24, color = None)]
    pub fn shadow(&self, blur: i32, color: Option<&str>) -> Chart {
        self.propagate(apply_shadow(
            self.html.clone(),
            blur,
            color.unwrap_or("rgba(0,0,0,.35)"),
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("blink", "beacon"),
        file = "charts/chart.md",
        en = "Makes data elements gently pulse, drawing attention to the chart (e.g. for live dashboards). By default pulses every element; target specific ones with index, or every element above a value with above.",
        fr = "Fait pulser des éléments de données, pour attirer l'attention (ex : tableaux de bord en direct). Par défaut, pulse tous les éléments ; cible des éléments précis avec index, ou tout élément au-dessus d'une valeur avec above.",
        param(
            name = "duration",
            ty = "float",
            en = "Pulse cycle duration in seconds. Default: 2.0.",
            fr = "Durée du cycle de pulsation en secondes. Défaut : 2.0."
        ),
        param(
            name = "index",
            ty = "list[int] | None",
            en = "Zero-based indices of the data elements to pulse (e.g. [0] for the first). Defaults to every element.",
            fr = "Index (à partir de 0) des éléments à faire pulser (ex : [0] pour le premier). Par défaut, tous les éléments."
        ),
        param(
            name = "above",
            ty = "float | None",
            en = "Only pulse data elements whose value exceeds this threshold.",
            fr = "Ne fait pulser que les éléments de données dont la valeur dépasse ce seuil."
        ),
        param(
            name = "color",
            ty = "str | None",
            en = "Force the pulsing elements to flash this color. Defaults to None, which keeps each element's normal color and only pulses its opacity.",
            fr = "Force les éléments à clignoter dans cette couleur. Par défaut None, ce qui conserve la couleur normale de chaque élément et ne fait pulser que son opacité."
        )
    )]
    #[sera_sig(duration = 2.0, index = None, above = None, color = None)]
    pub fn pulse(&self, duration: f64, index: Option<Vec<usize>>, above: Option<f64>, color: Option<&str>) -> Chart {
        self.propagate(apply_pulse(
            self.html.clone(),
            duration,
            index.as_deref(),
            above,
            color,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("border", "stroke"),
        file = "charts/chart.md",
        en = "Draws a solid colored outline around every data element, without blurring like glow().",
        fr = "Trace un contour coloré et net autour de chaque élément de données, sans flou comme glow().",
        param(
            name = "color",
            ty = "str | None",
            en = "Outline color. Defaults to the chart's accent color (#6366f1).",
            fr = "Couleur du contour. Par défaut, la couleur d'accent du graphique (#6366f1)."
        ),
        param(
            name = "width",
            ty = "float",
            en = "Outline width in pixels. Default: 2.0.",
            fr = "Épaisseur du contour en pixels. Défaut : 2.0."
        )
    )]
    #[sera_sig(color = None, width = 2.0)]
    pub fn outline(&self, color: Option<&str>, width: f64) -> Chart {
        self.propagate(apply_outline(
            self.html.clone(),
            color.unwrap_or("#6366f1"),
            width,
        ))
    }

}
