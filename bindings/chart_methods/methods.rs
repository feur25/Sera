use seraplot_macros::sera_impl;
use crate::{
    Chart, json_str, apply_3d_cfg,
    inject_labels, hover_dedup_images,
    GLOBAL_COLOR_BINDINGS,
};
use super::apply::*;
use super::js::*;
#[sera_impl(html_display, pickle, export)]
impl Chart {
    #[sera_python_skip]
    pub fn html_str(&self) -> &str {
        &self.html
    }

    #[sera_python_skip]
    pub fn doc_str_val(&self) -> &'static str {
        self.doc_str
    }

    #[sera_python_skip]
    pub fn iframe(&self) -> String {
        self.chart_iframe()
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/export.md",
        en = "Saves the chart HTML to a file at the given path.",
        fr = "Enregistre le HTML du graphique dans un fichier au chemin indiqué.",
        aliases("save_html", "write", "export_html"),
        param(
            name = "path",
            ty = "str",
            en = "Destination file path (e.g. 'chart.html').",
            fr = "Chemin du fichier de destination (ex: 'chart.html')."
        )
    )]
    #[sera_sig(path)]
    #[sera_wasm_skip]
    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        std::fs::write(path, &self.html)
    }

    #[sera_doc(
        category = "chart_method",
        aliases("bg", "background"),
        file = "charts/chart.md",
        en = "Sets the background color of the chart. Pass None to remove the background.",
        fr = "Définit la couleur d'arrière-plan du graphique. Passez None pour supprimer l'arrière-plan.",
        param(
            name = "color",
            ty = "str | None",
            en = "CSS color string (hex, rgb, named). None removes the background.",
            fr = "Couleur CSS (hex, rgb, nommée). None supprime l'arrière-plan."
        )
    )]
    #[sera_sig(color=None)]
    pub fn set_bg(&self, color: Option<&str>) -> Chart {
        self.propagate(crate::html::hover::apply_bg(self.html.clone(), color))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("transparent_bg", "no_bg"),
        file = "charts/chart.md",
        en = "Removes every chart background layer and keeps the output transparent.",
        fr = "Supprime toutes les couches d'arrière-plan du graphique et conserve une sortie transparente."
    )]
    pub fn no_background(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            "<style>html,body,.chart-container,.c3w,.sp-wrap,svg,canvas{background:transparent!important}.sp-bg{fill:transparent!important}body>:first-child{box-shadow:none!important}</style></head>",
            1,
        ))
    }

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
        aliases("hide_x", "no_x", "remove_x", "hide_x_axis", "drop_x"),
        file = "charts/chart.md",
        en = "Hides the X axis, its ticks, and its label.",
        fr = "Masque l'axe X, ses graduations et son étiquette."
    )]
    pub fn no_x_axis(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            "<style>.sp-ax-x,.sp-xt,.sp-xl{display:none}</style></head>",
            1,
        ))
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
        aliases("hide_y", "no_y", "remove_y", "hide_y_axis", "drop_y"),
        file = "charts/chart.md",
        en = "Hides the Y axis, its ticks, and its label.",
        fr = "Masque l'axe Y, ses graduations et son étiquette."
    )]
    pub fn no_y_axis(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            "<style>.sp-ax-y,.sp-yt,.sp-yl{display:none}</style></head>",
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("hide_axes", "no_axis", "hide_axis", "remove_axes", "drop_axes"),
        file = "charts/chart.md",
        en = "Hides both X and Y axes along with their ticks and labels.",
        fr = "Masque les axes X et Y ainsi que leurs graduations et étiquettes."
    )]
    pub fn no_axes(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            "<style>.sp-ax-x,.sp-ax-y,.sp-xt,.sp-yt,.sp-xl,.sp-yl{display:none}</style></head>",
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("grid"),
        file = "charts/chart.md",
        en = "Shows horizontal and vertical grid lines on the chart background.",
        fr = "Affiche les lignes de grille horizontales et verticales en arrière-plan du graphique."
    )]
    pub fn show_grid(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            "<style>.sp-gl{display:block!important;opacity:1!important}</style></head>",
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("no_grid"),
        file = "charts/chart.md",
        en = "Hides the grid lines if they were previously enabled.",
        fr = "Masque les lignes de grille si elles étaient précédemment activées."
    )]
    pub fn hide_grid(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            "<style>.sp-gl{display:none!important}</style></head>",
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Replaces grid lines with white stripes drawn directly inside each bar.",
        fr = "Remplace la grille par des segments blancs."
    )]
    pub fn segment_bars(&self) -> Chart {
        use super::js::SP_SEGMENT_BARS_JS;
        let html = if self.html.contains("</body>") {
            self.html.replacen("</body>", &(SP_SEGMENT_BARS_JS.to_string() + "</body>"), 1)
        } else {
            self.html.clone()
        };
        self.propagate(html)
    }

    #[sera_doc(
        category = "chart_method",
        aliases("font_size"),
        file = "charts/chart.md",
        en = "Overrides all SVG text elements to the specified font size in pixels.",
        fr = "Remplace la taille de police de tous les éléments texte SVG par la valeur spécifiée en pixels.",
        param(
            name = "px",
            ty = "int",
            en = "Font size in pixels.",
            fr = "Taille de police en pixels."
        )
    )]
    #[sera_sig(px)]
    pub fn set_font_size(&self, px: u32) -> Chart {
        let style = format!(
            "<style>svg text{{font-size:{}px!important}}</style></head>",
            px
        );
        self.propagate(self.html.replacen("</head>", &style, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("resize"),
        file = "charts/chart.md",
        en = "Scales the entire SVG by a given factor from the top-left origin.",
        fr = "Met à l'échelle l'intégralité du SVG par un facteur donné depuis le coin supérieur gauche.",
        param(
            name = "factor",
            ty = "float",
            en = "Scale multiplier (e.g. 1.5 for 150%).",
            fr = "Multiplicateur d'échelle (ex: 1.5 pour 150%)."
        )
    )]
    #[sera_sig(factor)]
    pub fn scale(&self, factor: f64) -> Chart {
        let style = format!(
            "<style>svg{{transform:scale({});transform-origin:top left}}</style></head>",
            factor
        );
        self.propagate(self.html.replacen("</head>", &style, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("frame"),
        file = "charts/chart.md",
        en = "Sets the background color of the SVG/canvas frame. Use 'transparent' or None to remove it.",
        fr = "Définit la couleur d'arrière-plan du cadre SVG/canvas. Utilisez 'transparent' ou None pour le supprimer.",
        param(
            name = "color",
            ty = "str | None",
            en = "CSS color for the frame background.",
            fr = "Couleur CSS pour l'arrière-plan du cadre."
        )
    )]
    #[sera_sig(color=None)]
    pub fn set_frame(&self, color: Option<&str>) -> Chart {
        let bg = match color {
            None | Some("none") | Some("transparent") | Some("") => "transparent".to_string(),
            Some(c) => c.to_string(),
        };
        let style = format!("<style>svg{{background:{bg}!important}}.c3w canvas{{background:{bg}!important}}.c3w{{background:{bg}!important}}</style></head>");
        self.propagate(self.html.replacen("</head>", &style, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("labels", "legend_labels"),
        file = "charts/chart.md",
        en = "Adds an interactive series filter overlay with clickable pill-shaped labels.",
        fr = "Ajoute une superposition de filtre de séries interactif avec des étiquettes en forme de pilule cliquables.",
        param(
            name = "position",
            ty = "str",
            en = "Position of the overlay: 'top', 'bottom', 'left', 'right'.",
            fr = "Position de la superposition: 'top', 'bottom', 'left', 'right'."
        ),
        param(
            name = "labels",
            ty = "list[str] | None",
            en = "Custom label names. Auto-detected if None.",
            fr = "Noms d'étiquettes personnalisés. Détection automatique si None."
        ),
        param(
            name = "colors",
            ty = "list[str] | None",
            en = "Custom color hex strings matching labels.",
            fr = "Couleurs hex personnalisées correspondant aux étiquettes."
        )
    )]
    #[sera_sig(position="bottom", labels=None, colors=None)]
    pub fn show_labels(
        &self,
        position: &str,
        labels: Option<Vec<String>>,
        colors: Option<Vec<String>>,
    ) -> Chart {
        let lb = labels.unwrap_or_default();
        let co = colors.unwrap_or_default();
        self.propagate(inject_labels(&self.html, position, &lb, &co))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/export.md",
        en = "Extracts and returns the raw SVG string from the chart HTML, or None if not present.",
        fr = "Extrait et retourne la chaîne SVG brute depuis le HTML du graphique, ou None si absente.",
        aliases("svg", "as_svg", "get_svg")
    )]
    pub fn to_svg(&self) -> Option<String> {
        let h = &self.html;
        let start = h.find("<svg")?;
        let end = h.rfind("</svg>")? + 6;
        Some(h[start..end].to_string())
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/export.md",
        en = "Saves the chart's SVG to a file.",
        fr = "Enregistre le SVG du graphique dans un fichier.",
        aliases("save_svg", "svg_export", "write_svg"),
        param(
            name = "path",
            ty = "str",
            en = "Destination .svg file path.",
            fr = "Chemin du fichier .svg de destination."
        )
    )]
    #[sera_sig(path)]
    #[sera_wasm_skip]
    pub fn export_svg(&self, path: &str) -> Result<(), std::io::Error> {
        let svg = self
            .to_svg()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "No SVG in chart"))?;
        std::fs::write(path, svg)
    }

    #[sera_doc(
        category = "chart_method",
        aliases("set_font", "font_family"),
        file = "charts/chart.md",
        en = "Sets the font family for all SVG text and body text in the chart.",
        fr = "Définit la famille de polices pour tous les textes SVG et les textes du corps du graphique.",
        param(
            name = "name",
            ty = "str",
            en = "Font family name (e.g. 'Roboto', 'Inter').",
            fr = "Nom de la famille de polices (ex: 'Roboto', 'Inter')."
        )
    )]
    #[sera_sig(name)]
    pub fn font(&self, name: &str) -> Chart {
        self.propagate(self.html.replacen("</head>", &format!("<style>svg text,body{{font-family:'{}',system-ui,sans-serif!important}}</style></head>", name), 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("ttl_size"),
        file = "charts/chart.md",
        en = "Sets the font size of the chart title in pixels.",
        fr = "Définit la taille de police du titre du graphique en pixels.",
        param(
            name = "px",
            ty = "int",
            en = "Title font size in pixels.",
            fr = "Taille de police du titre en pixels."
        )
    )]
    #[sera_sig(px)]
    pub fn title_size(&self, px: i32) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            &format!(
                "<style>.sp-ttl{{font-size:{}px!important}}</style></head>",
                px
            ),
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("xhair"),
        file = "charts/chart.md",
        en = "Adds an interactive crosshair that follows the mouse cursor across the SVG.",
        fr = "Ajoute un réticule interactif qui suit le curseur de la souris sur le SVG."
    )]
    pub fn crosshair(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</body>",
            &format!("<script>{}</script></body>", SP_CROSSHAIR_JS),
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("magnify"),
        file = "charts/chart.md",
        en = "Enables mouse-wheel zoom and click-drag panning on the chart. Double-click to reset.",
        fr = "Active le zoom à la molette et le déplacement par glisser-cliquer. Double-clic pour réinitialiser."
    )]
    pub fn zoom(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</body>",
            &format!("<script>{}</script></body>", SP_ZOOM_JS),
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("group_hover", "hover_dim"),
        file = "charts/chart.md",
        en = "When hovering a group element, dims all elements outside that group. Accepts an optional opacity level (0.0–1.0, default 0.15).",
        fr = "Au survol d'un élément de groupe, assombrit tous les éléments hors groupe. Accepte un niveau d'opacité optionnel (0.0–1.0, défaut 0.15)."
    )]
    pub fn group_hover_opacity(&self, dim: f64) -> Chart {
        let js = format!(
            "window.__sp_group_dim__={:.2};{}",
            dim.clamp(0.0, 1.0),
            SP_GROUP_HOVER_JS
        );
        self.propagate(self.html.replacen(
            "</body>",
            &format!("<script>{}</script></body>", js),
            1,
        ))
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
        aliases("dashed_grid", "dotted_grid"),
        file = "charts/chart.md",
        en = "Converts solid grid lines to a dashed pattern, reducing visual ink weight (Tufte data-ink principle).",
        fr = "Convertit les lignes de grille pleines en pointillés, réduisant le poids visuel (principe data-ink de Tufte)."
    )]
    pub fn sparse_grid(&self) -> Chart {
        let style = "<style>line.sp-gl{stroke-dasharray:4 4!important;stroke-opacity:.4!important}</style></head>";
        self.propagate(self.html.replacen("</head>", style, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("grid_horizontal", "y_grid"),
        file = "charts/chart.md",
        en = "Shows only horizontal (Y-axis value level) grid lines, hiding vertical category lines.",
        fr = "Affiche uniquement les lignes de grille horizontales (niveaux de valeur Y), masquant les lignes verticales."
    )]
    pub fn grid_y(&self) -> Chart {
        let mut html = self.html.replacen("</head>", "<style>.sp-gl{display:block!important;opacity:1!important}</style></head>", 1);
        html = html.replacen("</body>", &format!("<script>{}</script></body>", SP_GRID_Y_ONLY_JS), 1);
        self.propagate(html)
    }

    #[sera_doc(
        category = "chart_method",
        aliases("grid_vertical", "x_grid"),
        file = "charts/chart.md",
        en = "Shows only vertical (X-axis category separator) grid lines, hiding horizontal value lines.",
        fr = "Affiche uniquement les lignes de grille verticales (séparatrices de catégories X), masquant les lignes horizontales."
    )]
    pub fn grid_x(&self) -> Chart {
        let mut html = self.html.replacen("</head>", "<style>.sp-gl{display:block!important;opacity:1!important}</style></head>", 1);
        html = html.replacen("</body>", &format!("<script>{}</script></body>", SP_GRID_X_ONLY_JS), 1);
        self.propagate(html)
    }

    #[sera_doc(
        category = "chart_method",
        aliases("ref_line", "baseline", "threshold_line", "value_line"),
        file = "charts/chart.md",
        en = "Draws a solid horizontal reference line at the given data value. Unlike hline(), this line is solid and rendered below the bars, acting as a threshold grid marker.",
        fr = "Trace une ligne de référence horizontale solide à la valeur de données indiquée. Contrairement à hline(), cette ligne est pleine et rendue sous les barres, servant de marqueur de seuil.",
        param(
            name = "value",
            ty = "float",
            en = "Data value at which to draw the reference line.",
            fr = "Valeur de données à laquelle tracer la ligne de référence."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Line color as a CSS string (default '#64748b').",
            fr = "Couleur de la ligne en CSS (défaut '#64748b')."
        ),
        param(
            name = "label",
            ty = "str | None",
            en = "Optional text label shown at the right end of the line.",
            fr = "Étiquette optionnelle affichée à l'extrémité droite de la ligne."
        )
    )]
    #[sera_sig(value, color = "#64748b", label = None)]
    pub fn grid_at(&self, value: f64, color: &str, label: Option<&str>) -> Chart {
        let lbl = match label {
            Some(l) => json_str(l),
            None => "null".into(),
        };
        let cfg = format!(
            "window.__sp_grid_at__={{vs:[{}],c:{},ls:[{}]}};",
            value, json_str(color), lbl
        );
        self.propagate(self.html.replacen("</body>", &format!("<script>{}{}</script></body>", cfg, SP_GRID_AT_JS), 1))
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
        aliases("whiteboard", "sketch", "paintbrush", "annotate_tool", "pen"),
        file = "charts/chart.md",
        en = "Enables a freehand drawing overlay on the chart: pen mode, text mode, color picker, clear and SVG download. Drawings are stored as SVG paths inside the chart and survive save().",
        fr = "Active un calque de dessin libre sur le graphique : mode crayon, mode texte, sélecteur de couleur, effacer et téléchargement SVG. Les tracés sont des chemins SVG intégrés dans le graphique et survivent à save().",
        param(
            name = "color",
            ty = "str",
            en = "Initial stroke and text color as a CSS hex string (default '#ff0000').",
            fr = "Couleur initiale du trait et du texte en CSS hex (défaut '#ff0000')."
        )
    )]
    #[sera_sig(color = "#ff0000")]
    pub fn draw_tool(&self, color: &str) -> Chart {
        let cfg = format!("window.__sp_draw_cfg__={{color:{}}};", json_str(color));
        self.propagate(self.html.replacen("</body>", &format!("<script>{}{}</script></body>", cfg, SP_DRAW_TOOL_JS), 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("disable_select", "prevent_select", "unselectable", "no_text_select"),
        file = "charts/chart.md",
        en = "Disables the chart's drag-to-select overlay and browser text selection — combine with draw_tool() so dragging only draws, never activates the selection rectangle.",
        fr = "Désactive le rectangle de sélection par glisser du graphique et la sélection de texte — à combiner avec draw_tool() pour que le glisser ne dessine que, sans activer la sélection."
    )]
    pub fn no_select(&self) -> Chart {
        const JS: &str = "(function(){var ov=document.querySelector('.sp-sel-ov');if(!ov)return;var obs=new MutationObserver(function(){if(ov.style.display!=='none'){obs.disconnect();ov.style.display='none';obs.observe(ov,{attributes:true,attributeFilter:['style']});}});obs.observe(ov,{attributes:true,attributeFilter:['style']});})()";
        let h = self.html.replacen(
            "</head>",
            "<style>body,body *{user-select:none!important;-webkit-user-select:none!important}</style></head>",
            1,
        );
        self.propagate(h.replacen("</body>", &format!("<script>{}</script></body>", JS), 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("set_hover", "hover_data", "tooltip_slots", "with_hover", "hover_json"),
        file = "charts/chart.md",
        en = "Replaces the hover tooltip data with a JSON array of slot objects. Each slot maps to the existing tooltip fields: title (header), kv (key/value rows), image (url or data-uri — supports GIFs), video (url), html (raw HTML block). Pass one slot per data point.",
        fr = "Remplace les données de l'infobulle par un tableau JSON de slots. Chaque slot correspond aux champs existants : title (en-tête), kv (lignes clé/valeur), image (url ou data-uri — GIFs supportés), video (url), html (bloc HTML brut). Un slot par point de données.",
        param(name = "slots_json", ty = "str", desc_en = "JSON array of slot objects, e.g. [{\"title\":\"A\",\"image\":\"data:...\",\"kv\":[[\"k\",\"v\"]]}].", desc_fr = "Tableau JSON de slots, ex. [{\"title\":\"A\",\"image\":\"data:...\",\"kv\":[[\"k\",\"v\"]]}].")
    )]
    #[sera_sig(slots_json)]
    pub fn hover_slots(&self, slots_json: &str) -> Chart {
        let needle = "var data=";
        let term = ";\n\n";
        let (deduped_json, imgs_js) = hover_dedup_images(slots_json);
        let new_html = if let Some(pos) = self.html.find(needle) {
            let start = pos + needle.len();
            if let Some(end_rel) = self.html[start..].find(term) {
                let data_term = if imgs_js.is_empty() {
                    term.to_string()
                } else {
                    format!(";\nvar _hsImgs={};\n\n", imgs_js)
                };
                format!(
                    "{}{}{}{}",
                    &self.html[..start],
                    deduped_json,
                    data_term,
                    &self.html[start + end_rel + term.len()..]
                )
            } else {
                self.html.clone()
            }
        } else {
            self.html.clone()
        };
        self.propagate(new_html)
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
        aliases("colorize", "value_gradient", "recolor_by_value"),
        file = "charts/chart.md",
        en = "Colors each data element by its own value on a continuous color scale — the chainable, cross-chart replacement for a family-specific \"gradient\" variant. Works on any chart whose elements carry a data-v attribute (bars, points, nodes, cells...). `colorscale` picks one of the shared scales (\"viridis\", \"plasma\", \"inferno\", \"magma\", \"cividis\", \"turbo\", \"rdbu\", \"blues\", \"reds\", \"greens\"); omit it for the default indigo → cyan → green → amber → red density gradient.",
        fr = "Colore chaque élément de données selon sa propre valeur sur une échelle de couleur continue — le remplaçant chaînable et inter-graphiques d'une variante « gradient » propre à une famille. Fonctionne sur tout graphique dont les éléments portent un attribut data-v (barres, points, nœuds, cellules...). `colorscale` choisit l'une des échelles partagées (\"viridis\", \"plasma\", \"inferno\", \"magma\", \"cividis\", \"turbo\", \"rdbu\", \"blues\", \"reds\", \"greens\") ; l'omettre pour le gradient de densité indigo → cyan → vert → ambre → rouge par défaut.",
        param(
            name = "colorscale",
            ty = "str | None",
            en = "Named continuous colorscale, or None for the default density gradient.",
            fr = "Échelle de couleur continue nommée, ou None pour le gradient de densité par défaut."
        )
    )]
    #[sera_sig(colorscale = None)]
    pub fn gradient(&self, colorscale: Option<&str>) -> Chart {
        let cfg = match colorscale {
            Some(s) => format!("window.__sp_gradient__={{scale:{}}};", json_str(s)),
            None => String::new(),
        };
        self.propagate(self.html.replacen(
            "</body>",
            &format!("<script>{}{}</script></body>", cfg, SP_COLOR_DENSITY_JS),
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("highlight_labels", "focus_group"),
        file = "charts/chart.md",
        en = "Statically highlights elements whose label matches one of the given labels, dimming all others to the specified opacity.",
        fr = "Met en avant de façon statique les éléments dont le label correspond à la liste donnée, en assombrissant les autres au niveau d'opacité spécifié."
    )]
    #[sera_sig(labels, dim = 0.15)]
    pub fn highlight_group(&self, labels: Vec<String>, dim: f64) -> Chart {
        let labels_json = format!("[{}]", labels.iter().map(|l| format!("\"{}\"", l.replace('"', "\\\""))).collect::<Vec<_>>().join(","));
        let js = format!("window.__sp_hl_grp__={{g:{},d:{:.2}}};{}", labels_json, dim.clamp(0.0, 1.0), SP_HIGHLIGHT_STATIC_JS);
        self.propagate(self.html.replacen("</body>", &format!("<script>{}</script></body>", js), 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("apply_bindings", "color_from_bindings"),
        file = "charts/chart.md",
        en = "Applies the globally registered label→color bindings (set via sp.bind_colors()) to this chart.",
        fr = "Applique les correspondances label→couleur globales (définies via sp.bind_colors()) à ce graphique."
    )]
    pub fn apply_color_bindings(&self) -> Chart {
        let bindings = match GLOBAL_COLOR_BINDINGS.lock() {
            Ok(g) => g.clone(),
            Err(_) => {
                eprintln!(
                    "seraplot: global color-bindings mutex poisoned in apply_color_bindings -- \
                     returning the chart with no color bindings applied"
                );
                return self.propagate(self.html.clone());
            }
        };
        if bindings.is_empty() {
            return self.propagate(self.html.clone());
        }
        let entries: String = bindings.iter()
            .map(|(lbl, col)| format!("\"{}\":\"#{:06X}\"", lbl.replace('"', "\\\""), col))
            .collect::<Vec<_>>()
            .join(",");
        let js = format!(
            "(function(){{var m={{{}}};var svg=document.querySelector('svg');if(!svg)return;svg.querySelectorAll('[data-lbl]').forEach(function(el){{var lbl=el.getAttribute('data-lbl');if(m[lbl])el.style.setProperty('fill',m[lbl],'important');}});}})()",
            entries
        );
        self.propagate(self.html.replacen("</body>", &format!("<script>{}</script></body>", js), 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("swap_axes", "transpose"),
        file = "charts/chart.md",
        en = "Flips a vertical bar chart into a horizontal bar chart by recalculating bar positions.",
        fr = "Transforme un graphique à barres verticales en graphique à barres horizontales en recalculant les positions."
    )]
    pub fn flip(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</body>",
            &format!("<script>{}</script></body>", SP_FLIP_JS),
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("hbar", "flip_h"),
        file = "charts/chart.md",
        en = "Alias for flip(). Renders the chart with horizontal bars.",
        fr = "Alias de flip(). Affiche le graphique avec des barres horizontales."
    )]
    pub fn horizontal(&self) -> Chart {
        self.flip()
    }

    #[sera_doc(
        category = "chart_method",
        aliases("spin"),
        file = "charts/chart.md",
        en = "Rotates the entire chart by a snapped angle (0, 90, 180 or 270 degrees).",
        fr = "Fait pivoter l'intégralité du graphique selon un angle arrondi (0, 90, 180 ou 270 degrés).",
        param(
            name = "deg",
            ty = "int",
            en = "Rotation in degrees, snapped to nearest 90°. Default: 90.",
            fr = "Rotation en degrés, arrondie au 90° le plus proche. Défaut: 90."
        )
    )]
    #[sera_sig(deg = 90)]
    pub fn rotate(&self, deg: i32) -> Chart {
        let d = ((deg % 360) + 360) % 360;
        let snapped = match d {
            0..=44 | 316..=359 => 0,
            45..=134 => 90,
            135..=224 => 180,
            _ => 270,
        };
        self.propagate(crate::html::hover::apply_rotation(
            self.html.clone(),
            snapped,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("sort", "order_by"),
        file = "charts/chart.md",
        en = "Sorts chart bars by value or label using a client-side JavaScript re-render.",
        fr = "Trie les barres du graphique par valeur ou étiquette via un rendu JavaScript côté client.",
        param(
            name = "order",
            ty = "str",
            en = "Sort order: 'desc' (default), 'asc', 'alpha', 'alpha_desc', 'none'.",
            fr = "Ordre de tri: 'desc' (défaut), 'asc', 'alpha', 'alpha_desc', 'none'."
        )
    )]
    #[sera_sig(order = "desc")]
    pub fn sort_by(&self, order: &str) -> Chart {
        let ord = match order {
            "asc" | "desc" | "alpha" | "alpha_desc" | "none" => order,
            _ => "desc",
        };
        let snippet = format!(
            "<script>window.__sp_sort__={};{}</script></body>",
            json_str(ord),
            SP_SORT_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("show_leg"),
        file = "charts/chart.md",
        en = "Repositions the chart legend and enables interactive series toggling by click.",
        fr = "Repositionne la légende du graphique et active la bascule interactive des séries au clic.",
        param(
            name = "position",
            ty = "str",
            en = "Legend position: 'right' (default), 'left', 'top', 'bottom', 'none'.",
            fr = "Position de la légende: 'right' (défaut), 'left', 'top', 'bottom', 'none'."
        )
    )]
    #[sera_sig(position = "right")]
    pub fn legend(&self, position: &str) -> Chart {
        let pos = match position {
            "right" | "left" | "top" | "bottom" | "none" => position,
            _ => "right",
        };
        if pos == "none" {
            return self.no_legend();
        }
        let snippet = format!(
            "<script>window.__sp_legend_pos__={};{}</script></body>",
            json_str(pos),
            SP_LEGEND_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("tilt_labels"),
        file = "charts/chart.md",
        en = "Rotates X axis tick labels by the specified angle in degrees.",
        fr = "Fait pivoter les étiquettes de graduation de l'axe X de l'angle spécifié en degrés.",
        param(
            name = "angle",
            ty = "int",
            en = "Rotation angle in degrees (e.g. -45 for diagonal labels).",
            fr = "Angle de rotation en degrés (ex: -45 pour des étiquettes diagonales)."
        )
    )]
    #[sera_sig(angle)]
    pub fn rotate_labels(&self, angle: i32) -> Chart {
        let css = format!("<style>.sp-xt{{transform-box:fill-box;transform-origin:center;transform:rotate({}deg)}}</style></head>", angle);
        self.propagate(self.html.replacen("</head>", &css, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("fluid", "auto_size"),
        file = "charts/chart.md",
        en = "Makes the SVG width 100% of its container while keeping proportional height.",
        fr = "Rend la largeur du SVG égale à 100% de son conteneur tout en conservant une hauteur proportionnelle."
    )]
    pub fn responsive(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            "<style>svg{width:100%!important;height:auto!important}</style></head>",
            1,
        ))
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
        aliases("margin"),
        file = "charts/chart.md",
        en = "Adds internal padding to the chart and adjusts data element positions accordingly.",
        fr = "Ajoute un espacement interne au graphique et ajuste en conséquence les positions des éléments de données.",
        param(
            name = "px",
            ty = "int",
            en = "Margin in pixels applied to all four sides.",
            fr = "Marge en pixels appliquée aux quatre côtés."
        )
    )]
    #[sera_sig(px)]
    pub fn set_margin(&self, px: i32) -> Chart {
        let css = format!("<style>body{{padding:{px}px!important;box-sizing:border-box}}[id^='spp'],.c3w{{margin:{px}px!important}}</style></head>");
        let gap_ratio = ((px as f64) / 80.0).clamp(0.0, 0.7);
        let mut snippet = String::new();
        snippet.push_str("<script>window.__sp_margin_px__=");
        snippet.push_str(&px.to_string());
        snippet.push_str(";window.__sp_bar_gap__=");
        snippet.push_str(&format!("{:.4}", gap_ratio));
        snippet.push(';');
        snippet.push_str(SP_MARGIN_JS);
        snippet.push(';');
        snippet.push_str(SP_BAR_GAP_JS);
        snippet.push_str(";</script></body>");
        let h = self.html.replacen("</head>", &css, 1);
        self.propagate(h.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("gap"),
        file = "charts/chart.md",
        en = "Adjusts the gap ratio between bars. Higher values create thinner bars with more space.",
        fr = "Ajuste le ratio d'espacement entre les barres. Des valeurs plus élevées créent des barres plus fines.",
        param(
            name = "ratio",
            ty = "float",
            en = "Gap ratio between 0.0 (no gap) and 0.95 (almost no bar). Default: 0.3.",
            fr = "Ratio d'espacement entre 0.0 (sans espacement) et 0.95 (presque sans barre). Défaut: 0.3."
        )
    )]
    #[sera_sig(ratio = 0.3)]
    pub fn bar_gap(&self, ratio: f64) -> Chart {
        let r = ratio.clamp(0.0, 0.95);
        let snippet = format!(
            "<script>window.__sp_bar_gap__={:.4};{}</script></body>",
            r, SP_BAR_GAP_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("padding"),
        file = "charts/chart.md",
        en = "Applies CSS padding to the chart container element.",
        fr = "Applique un padding CSS à l'élément conteneur du graphique.",
        param(
            name = "px",
            ty = "int",
            en = "Padding in pixels applied to all four sides.",
            fr = "Padding en pixels appliqué aux quatre côtés."
        )
    )]
    #[sera_sig(px)]
    pub fn set_padding(&self, px: i32) -> Chart {
        let css = format!("<style>[id^='spp'],.c3w{{padding:{px}px!important;box-sizing:border-box}}</style></head>");
        self.propagate(self.html.replacen("</head>", &css, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("tick_angle", "label_angle"),
        file = "charts/chart.md",
        en = "Rotates X and/or Y axis tick labels independently.",
        fr = "Fait pivoter indépendamment les étiquettes de graduation des axes X et/ou Y.",
        param(
            name = "x_angle",
            ty = "int | None",
            en = "Rotation angle for X axis labels in degrees.",
            fr = "Angle de rotation des étiquettes de l'axe X en degrés."
        ),
        param(
            name = "y_angle",
            ty = "int | None",
            en = "Rotation angle for Y axis labels in degrees.",
            fr = "Angle de rotation des étiquettes de l'axe Y en degrés."
        )
    )]
    #[sera_sig(x_angle=None, y_angle=None)]
    pub fn axis_label_angle(&self, x_angle: Option<i32>, y_angle: Option<i32>) -> Chart {
        let mut css = String::from("<style>");
        if let Some(a) = x_angle {
            css.push_str(&format!(
                ".sp-xt{{transform-box:fill-box;transform-origin:center;transform:rotate({}deg)}}",
                a
            ));
        }
        if let Some(a) = y_angle {
            css.push_str(&format!(
                ".sp-yt{{transform-box:fill-box;transform-origin:center;transform:rotate({}deg)}}",
                a
            ));
        }
        css.push_str("</style></head>");
        self.propagate(self.html.replacen("</head>", &css, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("hide_legend"),
        file = "charts/chart.md",
        en = "Hides the chart legend.",
        fr = "Masque la légende du graphique."
    )]
    pub fn no_legend(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            "<style>g[data-legend]{display:none!important}</style></head>",
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("hide_title"),
        file = "charts/chart.md",
        en = "Hides the chart title.",
        fr = "Masque le titre du graphique."
    )]
    pub fn no_title(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</head>",
            "<style>.sp-ttl{display:none!important}</style></head>",
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("title_on"),
        file = "charts/chart.md",
        en = "Forces the chart title to be visible with a contrast stroke for readability.",
        fr = "Force le titre du graphique à être visible avec un contour de contraste pour la lisibilité."
    )]
    pub fn show_title(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>.sp-ttl{display:block!important;visibility:visible!important;opacity:1!important;fill:#e2e8f0!important;paint-order:stroke;stroke:rgba(0,0,0,.6);stroke-width:.6px}</style></head>", 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("leg"),
        file = "charts/chart.md",
        en = "Forces the chart legend to be visible even if it was hidden.",
        fr = "Force la légende du graphique à être visible même si elle était masquée."
    )]
    pub fn show_legend(&self) -> Chart {
        self.propagate(self.html.replacen("</head>", "<style>g[data-legend]{display:block!important;visibility:visible!important}</style></head>", 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("leg_pos"),
        file = "charts/chart.md",
        en = "Alias for legend(position). Repositions the chart legend.",
        fr = "Alias de legend(position). Repositionne la légende du graphique.",
        param(
            name = "position",
            ty = "str",
            en = "Legend position: 'right', 'left', 'top', 'bottom', 'none'.",
            fr = "Position de la légende: 'right', 'left', 'top', 'bottom', 'none'."
        )
    )]
    #[sera_sig(position = "right")]
    pub fn legend_position(&self, position: &str) -> Chart {
        self.legend(position)
    }

    #[sera_doc(
        category = "chart_method",
        aliases("labels_at"),
        file = "charts/chart.md",
        en = "Alias for show_labels(position). Adds an interactive legend overlay at the given position.",
        fr = "Alias de show_labels(position). Ajoute une superposition de légende interactive à la position donnée.",
        param(
            name = "position",
            ty = "str",
            en = "Overlay position: 'top', 'bottom', 'left', 'right'.",
            fr = "Position de la superposition: 'top', 'bottom', 'left', 'right'."
        )
    )]
    #[sera_sig(position = "bottom")]
    pub fn label_position(&self, position: &str) -> Chart {
        self.show_labels(position, None, None)
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/export.md",
        en = "Adds a floating download button to the chart that saves the full HTML on click.",
        fr = "Ajoute un bouton de téléchargement flottant au graphique qui sauvegarde le HTML complet au clic.",
        aliases("download_button", "export_btn", "with_export_button")
    )]
    pub fn export_button(&self) -> Chart {
        if self.html.contains("class=\"c3w\"") {
            return self.propagate(apply_3d_cfg(self.html.clone(), "{\"exportBtn\":true}"));
        }
        self.propagate(self.html.replacen(
            "</body>",
            &format!("<script>{}</script></body>", SP_EXPORT_JS),
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("ttl_color", "tc"),
        file = "charts/chart.md",
        en = "Sets the title color of a 3D chart (works on 3D canvas charts only). Default: white.",
        fr = "Définit la couleur du titre d'un graphique 3D (fonctionne uniquement sur les graphiques canvas 3D). Défaut : blanc.",
        param(
            name = "color",
            ty = "str",
            en = "CSS color for the title text.",
            fr = "Couleur CSS pour le texte du titre."
        )
    )]
    #[sera_sig(color = "#ffffff")]
    pub fn title_color(&self, color: &str) -> Chart {
        if self.html.contains("class=\"c3w\"") {
            return self.propagate(apply_3d_cfg(
                self.html.clone(),
                &format!("{{\"titleColor\":{}}}", json_str(color)),
            ));
        }
        let css = format!(
            "<style>.sp-ttl{{fill:{}!important}}</style></head>",
            color
        );
        self.propagate(self.html.replacen("</head>", &css, 1))
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
    pub fn text_auto(
        &self,
        format: Option<&str>,
        position: Option<&str>,
        angle: Option<i32>,
        font_size: Option<i32>,
        color: Option<&str>,
    ) -> Chart {
        let mut opts = String::from("window.__sp_text__={");
        if let Some(f) = format {
            opts.push_str(&format!("format:{},", json_str(f)));
        }
        if let Some(p) = position {
            opts.push_str(&format!("position:{},", json_str(p)));
        }
        if let Some(a) = angle {
            opts.push_str(&format!("angle:{},", a));
        }
        if let Some(s) = font_size {
            opts.push_str(&format!("font_size:{},", s));
        }
        if let Some(c) = color {
            opts.push_str(&format!("color:{},", json_str(c)));
        }
        opts.push_str("};");
        let snippet = format!("<script>{}{}</script></body>", opts, SP_TEXT_JS);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("text_pos"),
        file = "charts/chart.md",
        en = "Sets the position for data value labels on chart elements.",
        fr = "Définit la position des étiquettes de valeurs de données sur les éléments du graphique.",
        param(
            name = "position",
            ty = "str",
            en = "Position: 'auto', 'inside', 'outside'.",
            fr = "Position: 'auto', 'inside', 'outside'."
        )
    )]
    #[sera_sig(position)]
    pub fn text_position(&self, position: &str) -> Chart {
        let snippet = format!("<script>window.__sp_text__=Object.assign(window.__sp_text__||{{}},{{position:{}}});{}</script></body>", json_str(position), SP_TEXT_JS);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("rotate_text"),
        file = "charts/chart.md",
        en = "Sets the rotation angle for data value labels.",
        fr = "Définit l'angle de rotation des étiquettes de valeurs de données.",
        param(
            name = "degrees",
            ty = "int",
            en = "Rotation angle in degrees.",
            fr = "Angle de rotation en degrés."
        )
    )]
    #[sera_sig(degrees)]
    pub fn text_angle(&self, degrees: i32) -> Chart {
        let snippet = format!("<script>window.__sp_text__=Object.assign(window.__sp_text__||{{}},{{angle:{}}});{}</script></body>", degrees, SP_TEXT_JS);
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
        aliases("uniform"),
        file = "charts/chart.md",
        en = "Enforces a minimum font size for data labels; hides or shows labels that don't fit.",
        fr = "Impose une taille de police minimale pour les étiquettes de données; masque ou affiche les étiquettes qui ne tiennent pas.",
        param(
            name = "min_size",
            ty = "int",
            en = "Minimum font size in pixels.",
            fr = "Taille de police minimale en pixels."
        ),
        param(
            name = "mode",
            ty = "str",
            en = "Behaviour when label doesn't fit: 'hide' or 'show'.",
            fr = "Comportement quand l'étiquette ne tient pas: 'hide' ou 'show'."
        )
    )]
    #[sera_sig(min_size = 8, mode = "hide")]
    pub fn uniform_text(&self, min_size: i32, mode: &str) -> Chart {
        let snippet = format!("<script>window.__sp_text__=Object.assign(window.__sp_text__||{{}},{{uniform_min:{},uniform_mode:{}}});{}</script></body>", min_size, json_str(mode), SP_TEXT_JS);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("bar_radius"),
        file = "charts/chart.md",
        en = "Applies a corner radius to all bar rectangles in the chart.",
        fr = "Applique un rayon de coin à tous les rectangles de barres du graphique.",
        param(
            name = "radius",
            ty = "str",
            en = "Radius in pixels as string or percentage (e.g. '8' or '50%').",
            fr = "Rayon en pixels sous forme de chaîne ou pourcentage (ex: '8' ou '50%')."
        )
    )]
    #[sera_sig(radius)]
    pub fn corner_radius_bars(&self, radius: &str) -> Chart {
        let val = if radius.ends_with('%') {
            json_str(radius)
        } else {
            radius
                .parse::<f64>()
                .map(|v| v.to_string())
                .unwrap_or_else(|_| json_str(radius))
        };
        let snippet = format!(
            "<script>window.__sp_bar_r__={};{}</script></body>",
            val, SP_BAR_RADIUS_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("safe_csp"),
        file = "charts/chart.md",
        en = "Removes inline event handlers to make the chart compatible with strict Content-Security-Policy environments.",
        fr = "Supprime les gestionnaires d'événements inline pour rendre le graphique compatible avec les environnements à politique de sécurité de contenu stricte."
    )]
    pub fn csp_safe(&self) -> Chart {
        let mut out = String::with_capacity(self.html.len());
        let mut rest = self.html.as_str();
        let mut blob = String::new();
        loop {
            match rest.find("<script>") {
                None => {
                    out.push_str(rest);
                    break;
                }
                Some(i) => {
                    out.push_str(&rest[..i]);
                    let after = &rest[i + 8..];
                    match after.find("</script>") {
                        None => {
                            out.push_str("<script>");
                            out.push_str(after);
                            break;
                        }
                        Some(j) => {
                            blob.push_str(&after[..j]);
                            blob.push_str(";\n");
                            rest = &after[j + 9..];
                        }
                    }
                }
            }
        }
        let injected = if blob.is_empty() {
            out
        } else {
            let id = format!("sp-csp-{}", blob.len());
            let tag = format!("<script type=\"application/json\" id=\"{id}\">{}</script><script nonce=\"sp-nonce\">eval(document.getElementById('{id}').textContent)</script></body>", blob.replace("</script>", "<\\/script>"));
            out.replacen("</body>", &tag, 1)
        };
        self.propagate(injected)
    }

    #[sera_doc(
        category = "chart_method",
        aliases("accessibility", "aria"),
        file = "charts/chart.md",
        en = "Adds ARIA accessibility attributes (title and description) to the SVG element.",
        fr = "Ajoute des attributs d'accessibilité ARIA (titre et description) à l'élément SVG.",
        param(
            name = "title",
            ty = "str",
            en = "Accessible title for screen readers.",
            fr = "Titre accessible pour les lecteurs d'écran."
        ),
        param(
            name = "desc",
            ty = "str",
            en = "Accessible description for screen readers.",
            fr = "Description accessible pour les lecteurs d'écran."
        )
    )]
    #[sera_sig(title = "", desc = "")]
    pub fn a11y(&self, title: &str, desc: &str) -> Chart {
        let esc_attr = title.replace('&', "&amp;").replace('"', "&quot;");
        let esc_title = title.replace('&', "&amp;").replace('<', "&lt;");
        let esc_desc = desc.replace('&', "&amp;").replace('<', "&lt;");
        let html = match self.html.find("<svg") {
            Some(open_start) => match self.html[open_start..].find('>') {
                Some(rel_close) => {
                    let tag_end = open_start + rel_close;
                    let mut out = String::with_capacity(self.html.len() + esc_attr.len() + esc_title.len() + esc_desc.len() + 64);
                    out.push_str(&self.html[..tag_end]);
                    out.push_str(" aria-label=\"");
                    out.push_str(&esc_attr);
                    out.push('"');
                    out.push_str(&self.html[tag_end..=tag_end]);
                    out.push_str("<title>");
                    out.push_str(&esc_title);
                    out.push_str("</title><desc>");
                    out.push_str(&esc_desc);
                    out.push_str("</desc>");
                    out.push_str(&self.html[tag_end + 1..]);
                    out
                }
                None => self.html.clone(),
            },
            None => self.html.clone(),
        };
        self.propagate(html)
    }

    #[sera_doc(
        category = "chart_method",
        aliases("compare"),
        file = "charts/chart.md",
        en = "Returns a textual diff between this chart's HTML and another chart's HTML.",
        fr = "Retourne un diff textuel entre le HTML de ce graphique et celui d'un autre graphique.",
        param(
            name = "other",
            ty = "Chart",
            en = "The other Chart instance to compare against.",
            fr = "L'autre instance Chart à comparer."
        )
    )]
    pub fn diff(&self, other: &Chart) -> String {
        crate::bindings::commands::charts::chart_diff(
            &serde_json::json!({"a": self.html, "b": other.html}).to_string(),
        )
    }

    #[sera_doc(
        category = "chart_method",
        aliases("decimate", "lod"),
        file = "charts/chart.md",
        en = "Downsamples line chart data using the LTTB algorithm to reduce visual clutter.",
        fr = "Réduit les données du graphique en courbes via l'algorithme LTTB pour diminuer l'encombrement visuel.",
        param(
            name = "n",
            ty = "int",
            en = "Target number of data points after downsampling.",
            fr = "Nombre cible de points de données après réduction."
        ),
        param(
            name = "method",
            ty = "str",
            en = "Downsampling method. Currently only 'lttb' is supported.",
            fr = "Méthode de réduction. Seul 'lttb' est actuellement supporté."
        )
    )]
    #[sera_sig(n = 2000, method = "lttb")]
    pub fn downsample(&self, n: usize, method: &str) -> Chart {
        let _ = method;
        let h = &self.html;
        let mut out = String::with_capacity(h.len());
        let mut rest = h.as_str();
        loop {
            match rest.find("data-x=\"") {
                None => {
                    out.push_str(rest);
                    break;
                }
                Some(i) => {
                    out.push_str(&rest[..i]);
                    let after = &rest[i + 8..];
                    let end = match after.find('"') {
                        Some(e) => e,
                        None => {
                            out.push_str("data-x=\"");
                            out.push_str(after);
                            break;
                        }
                    };
                    let xs_raw = &after[..end];
                    let after2 = &after[end + 1..];
                    let after_y = match after2.find("data-y=\"") {
                        Some(j) => j,
                        None => {
                            out.push_str("data-x=\"");
                            out.push_str(after);
                            break;
                        }
                    };
                    let ys_section = &after2[after_y + 8..];
                    let ys_end = match ys_section.find('"') {
                        Some(e) => e,
                        None => {
                            out.push_str("data-x=\"");
                            out.push_str(after);
                            break;
                        }
                    };
                    let ys_raw = &ys_section[..ys_end];
                    let xs: Vec<f64> = xs_raw.split(',').filter_map(|s| s.parse().ok()).collect();
                    let ys: Vec<f64> = ys_raw.split(',').filter_map(|s| s.parse().ok()).collect();
                    if xs.len() == ys.len() && xs.len() > n && n >= 3 {
                        let payload = serde_json::json!({"x":xs,"y":ys,"threshold":n}).to_string();
                        let res = crate::bindings::commands::charts::downsample_lttb(&payload);
                        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&res) {
                            if v.get("ok").and_then(|b| b.as_bool()).unwrap_or(false) {
                                let nx: Vec<String> = v
                                    .get("x")
                                    .and_then(|a| a.as_array())
                                    .map(|a| {
                                        a.iter()
                                            .filter_map(|n| n.as_f64().map(|x| x.to_string()))
                                            .collect()
                                    })
                                    .unwrap_or_default();
                                let ny: Vec<String> = v
                                    .get("y")
                                    .and_then(|a| a.as_array())
                                    .map(|a| {
                                        a.iter()
                                            .filter_map(|n| n.as_f64().map(|x| x.to_string()))
                                            .collect()
                                    })
                                    .unwrap_or_default();
                                out.push_str(&format!("data-x=\"{}\"", nx.join(",")));
                                out.push_str(&after2[..after_y]);
                                out.push_str(&format!("data-y=\"{}\"", ny.join(",")));
                                rest = &ys_section[ys_end + 1..];
                                continue;
                            }
                        }
                    }
                    out.push_str("data-x=\"");
                    out.push_str(xs_raw);
                    out.push('"');
                    out.push_str(&after2[..after_y]);
                    out.push_str("data-y=\"");
                    out.push_str(ys_raw);
                    out.push('"');
                    rest = &ys_section[ys_end + 1..];
                }
            }
        }
        self.propagate(out)
    }

    #[sera_doc(
        category = "chart_method",
        aliases("tab_group"),
        file = "charts/chart.md",
        en = "Groups this chart with every other chart sharing the same group name into one navigable, clickable, interchangeable tabbed section. Works for any 2D or 3D chart, in a notebook or any HTML page.",
        fr = "Regroupe ce graphique avec tous les autres graphiques partageant le même nom de groupe en une section à onglets navigable, cliquable et interchangeable. Fonctionne pour tout graphique 2D ou 3D, dans un notebook ou n'importe quelle page HTML.",
        param(
            name = "name",
            ty = "str",
            en = "Group identifier shared by every chart that should appear together.",
            fr = "Identifiant de groupe partagé par tous les graphiques qui doivent apparaître ensemble."
        ),
        param(
            name = "position",
            ty = "str",
            en = "Where the tab strip sits relative to the chart: 'top' (default), 'bottom', 'left' or 'right'.",
            fr = "Où se place la barre d'onglets par rapport au graphique : 'top' (défaut), 'bottom', 'left' ou 'right'."
        )
    )]
    #[sera_sig(name, position = "top")]
    pub fn group(&self, name: &str, position: &str) -> Chart {
        let pos = match position {
            "bottom" | "left" | "right" => position,
            _ => "top",
        };
        let snippet = format!(
            "<script>window.__sp_group_name__={};window.__sp_group_pos__={};{}</script></body>",
            json_str(name),
            json_str(pos),
            SP_GROUP_JS
        );
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
        aliases("orientation3d", "tilt3d", "rotate3d"),
        file = "charts/chart.md",
        en = "Sets the initial camera orientation of a 3D chart: 'iso' (default), 'horizontal' (side-on), 'vertical' (top-down) or 'front'.",
        fr = "Définit l'orientation initiale de la caméra d'un graphique 3D : 'iso' (défaut), 'horizontal' (vue de côté), 'vertical' (vue de dessus) ou 'front'.",
        param(
            name = "mode",
            ty = "str",
            en = "'iso', 'horizontal', 'vertical' or 'front'.",
            fr = "'iso', 'horizontal', 'vertical' ou 'front'."
        )
    )]
    #[sera_sig(mode = "iso")]
    pub fn orient3d(&self, mode: &str) -> Chart {
        self.propagate(apply_orient3d(self.html.clone(), mode))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("color_scale", "heat_legend", "color_legend"),
        file = "charts/chart.md",
        en = "Adds a heat colorbar legend showing the value range used to color the data points, in both 2D and 3D charts. Position can be 'right' (default), 'left', 'top' or 'bottom'.",
        fr = "Ajoute une jauge de couleur (colorbar) montrant la plage de valeurs utilisée pour colorer les points, en 2D comme en 3D. La position peut être 'right' (défaut), 'left', 'top' ou 'bottom'.",
        param(
            name = "position",
            ty = "str",
            en = "'right', 'left', 'top' or 'bottom'.",
            fr = "'right', 'left', 'top' ou 'bottom'."
        )
    )]
    #[sera_sig(position = "right")]
    pub fn colorbar(&self, position: &str) -> Chart {
        self.propagate(apply_colorbar(self.html.clone(), position))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("heat_gauge", "value_heatmap"),
        file = "charts/chart.md",
        en = "Turns any plot into a heat-colored view in one chainable call: recolors every data element by its value on an indigo → cyan → green → amber → red gradient and attaches a matching colorbar gauge showing the value range, so the legend always matches the colors actually used. Position can be 'right' (default), 'left', 'top' or 'bottom'.",
        fr = "Transforme n'importe quel graphique en vue chaleur en un seul appel chaînable : recolore chaque élément de données selon sa valeur sur un gradient indigo → cyan → vert → ambre → rouge et ajoute une jauge (colorbar) assortie montrant la plage de valeurs, pour que la légende corresponde toujours aux couleurs réellement utilisées. La position peut être 'right' (défaut), 'left', 'top' ou 'bottom'.",
        param(
            name = "position",
            ty = "str",
            en = "'right', 'left', 'top' or 'bottom'.",
            fr = "'right', 'left', 'top' ou 'bottom'."
        )
    )]
    #[sera_sig(position = "right")]
    pub fn heatify(&self, position: &str) -> Chart {
        self.propagate(apply_heatify(self.html.clone(), position))
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
        aliases("mark", "spotlight"),
        file = "charts/chart.md",
        en = "Highlights a single data element by index with a distinct color and glow, dimming nothing else.",
        fr = "Met en évidence un seul élément de données par son index avec une couleur distincte et une lueur, sans assombrir le reste.",
        param(
            name = "index",
            ty = "int",
            en = "Zero-based index of the data element to highlight.",
            fr = "Index (à partir de 0) de l'élément de données à mettre en évidence."
        ),
        param(
            name = "color",
            ty = "str | None",
            en = "Highlight color. Default: #f59e0b.",
            fr = "Couleur de mise en évidence. Défaut : #f59e0b."
        )
    )]
    #[sera_sig(index, color = None)]
    pub fn highlight(&self, index: usize, color: Option<&str>) -> Chart {
        self.propagate(apply_highlight(
            self.html.clone(),
            index,
            color.unwrap_or("#f59e0b"),
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("hrule", "target_line"),
        file = "charts/chart.md",
        en = "Draws a dashed horizontal reference/threshold line at the given data value, on value-based bar charts.",
        fr = "Trace une ligne de référence/seuil en pointillés à la valeur donnée, sur les graphiques en barres basés sur des valeurs.",
        param(
            name = "value",
            ty = "float",
            en = "Data value where the line is drawn.",
            fr = "Valeur de donnée où la ligne est tracée."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Line color. Default: #ef4444.",
            fr = "Couleur de la ligne. Défaut : #ef4444."
        ),
        param(
            name = "label",
            ty = "str | None",
            en = "Optional label drawn next to the line.",
            fr = "Étiquette optionnelle affichée près de la ligne."
        )
    )]
    #[sera_sig(value, color = "#ef4444", label = None)]
    pub fn hline(&self, value: f64, color: &str, label: Option<&str>) -> Chart {
        self.propagate(apply_hline(self.html.clone(), value, color, label))
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

    #[sera_doc(
        category = "chart_method",
        aliases("trend", "regline"),
        file = "charts/chart.md",
        en = "Overlays a linear regression trend line computed from the chart's own values, on value-based bar charts.",
        fr = "Superpose une droite de régression linéaire calculée à partir des valeurs du graphique, sur les graphiques en barres basés sur des valeurs.",
        param(
            name = "color",
            ty = "str",
            en = "Trend line color. Default: #10b981.",
            fr = "Couleur de la droite de tendance. Défaut : #10b981."
        ),
        param(
            name = "width",
            ty = "float",
            en = "Trend line width in pixels. Default: 2.0.",
            fr = "Épaisseur de la droite de tendance en pixels. Défaut : 2.0."
        )
    )]
    #[sera_sig(color = "#10b981", width = 2.0)]
    pub fn trendline(&self, color: &str, width: f64) -> Chart {
        self.propagate(apply_trendline(self.html.clone(), color, width))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("mark_max", "ann_max"),
        file = "charts/chart.md",
        en = "Highlights and labels the data element with the highest value, on value-based bar charts.",
        fr = "Met en évidence et étiquette l'élément de données ayant la valeur la plus élevée, sur les graphiques en barres basés sur des valeurs.",
        param(
            name = "label",
            ty = "str | None",
            en = "Optional custom label text. Defaults to the value itself.",
            fr = "Texte d'étiquette personnalisé optionnel. Par défaut, la valeur elle-même."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Highlight and label color. Default: #22c55e.",
            fr = "Couleur de mise en évidence et de l'étiquette. Défaut : #22c55e."
        )
    )]
    #[sera_sig(label = None, color = "#22c55e")]
    pub fn annotate_max(&self, label: Option<&str>, color: &str) -> Chart {
        self.propagate(apply_annotate_extreme(self.html.clone(), "max", color, label))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("mark_min", "ann_min"),
        file = "charts/chart.md",
        en = "Highlights and labels the data element with the lowest value, on value-based bar charts.",
        fr = "Met en évidence et étiquette l'élément de données ayant la valeur la plus basse, sur les graphiques en barres basés sur des valeurs.",
        param(
            name = "label",
            ty = "str | None",
            en = "Optional custom label text. Defaults to the value itself.",
            fr = "Texte d'étiquette personnalisé optionnel. Par défaut, la valeur elle-même."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Highlight and label color. Default: #ef4444.",
            fr = "Couleur de mise en évidence et de l'étiquette. Défaut : #ef4444."
        )
    )]
    #[sera_sig(label = None, color = "#ef4444")]
    pub fn annotate_min(&self, label: Option<&str>, color: &str) -> Chart {
        self.propagate(apply_annotate_extreme(self.html.clone(), "min", color, label))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("ref_band", "target_zone"),
        file = "charts/chart.md",
        en = "Draws a shaded reference band between two data values, on value-based bar charts (e.g. a target range).",
        fr = "Trace une bande de référence ombrée entre deux valeurs, sur les graphiques en barres basés sur des valeurs (ex : une plage cible).",
        param(
            name = "low",
            ty = "float",
            en = "Lower bound of the band.",
            fr = "Borne basse de la bande."
        ),
        param(
            name = "high",
            ty = "float",
            en = "Upper bound of the band.",
            fr = "Borne haute de la bande."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Band fill color. Default: #f59e0b.",
            fr = "Couleur de remplissage de la bande. Défaut : #f59e0b."
        ),
        param(
            name = "opacity",
            ty = "float",
            en = "Band opacity between 0.0 and 1.0. Default: 0.12.",
            fr = "Opacité de la bande entre 0.0 et 1.0. Défaut : 0.12."
        )
    )]
    #[sera_sig(low, high, color = "#f59e0b", opacity = 0.12)]
    pub fn reference_band(&self, low: f64, high: f64, color: &str, opacity: f64) -> Chart {
        self.propagate(apply_reference_band(self.html.clone(), low, high, color, opacity))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("vlabels", "bar_labels"),
        file = "charts/chart.md",
        en = "Prints the numeric value above each bar, like matplotlib's bar_label() or Plotly's textposition='auto'.",
        fr = "Affiche la valeur numérique au-dessus de chaque barre, comme bar_label() de matplotlib ou textposition='auto' de Plotly.",
        param(
            name = "decimals",
            ty = "int",
            en = "Number of decimal places to show. Default: 0.",
            fr = "Nombre de décimales à afficher. Défaut : 0."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Label text color. Default: #475569.",
            fr = "Couleur du texte du label. Défaut : #475569."
        )
    )]
    #[sera_sig(decimals = 0, color = "#475569")]
    pub fn value_labels(&self, decimals: i32, color: &str) -> Chart {
        self.propagate(apply_value_labels(self.html.clone(), decimals, color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Draws symmetric error-bar whiskers above and below each bar's top, like matplotlib's errorbar() or yerr.",
        fr = "Dessine des moustaches symétriques (barres d'erreur) au-dessus et en-dessous du sommet de chaque barre, comme errorbar() ou yerr de matplotlib.",
        aliases("errorbar"),
        param(
            name = "margin",
            ty = "float",
            en = "Error margin, expressed in the same units as the chart's values.",
            fr = "Marge d'erreur, exprimée dans la même unité que les valeurs du graphique."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Whisker color. Default: #475569.",
            fr = "Couleur des moustaches. Défaut : #475569."
        )
    )]
    #[sera_sig(margin, color = "#475569")]
    pub fn error_bars(&self, margin: f64, color: &str) -> Chart {
        self.propagate(apply_error_bars(self.html.clone(), margin, color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Labels each bar with its percentage change versus the previous bar, handy for tracking growth (QoQ, MoM, ...).",
        fr = "Annote chaque barre avec son évolution en pourcentage par rapport à la barre précédente, pratique pour suivre une croissance (trimestre, mois, ...).",
        aliases("deltas", "pct_change"),
        param(
            name = "pos_color",
            ty = "str",
            en = "Color used for positive changes. Default: #22c55e.",
            fr = "Couleur utilisée pour les évolutions positives. Défaut : #22c55e."
        ),
        param(
            name = "neg_color",
            ty = "str",
            en = "Color used for negative changes. Default: #ef4444.",
            fr = "Couleur utilisée pour les évolutions négatives. Défaut : #ef4444."
        )
    )]
    #[sera_sig(pos_color = "#22c55e", neg_color = "#ef4444")]
    pub fn delta_labels(&self, pos_color: &str, neg_color: &str) -> Chart {
        self.propagate(apply_delta_labels(self.html.clone(), pos_color, neg_color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Overlays a cumulative-sum line (running total over the running grand total), the classic Pareto-chart finishing touch.",
        fr = "Superpose une courbe de cumul (somme courante sur le total général), la touche finale classique d'un diagramme de Pareto.",
        aliases("cumline"),
        param(
            name = "color",
            ty = "str",
            en = "Line and marker color. Default: #6366f1.",
            fr = "Couleur de la courbe et des marqueurs. Défaut : #6366f1."
        )
    )]
    #[sera_sig(color = "#6366f1")]
    pub fn cumulative_line(&self, color: &str) -> Chart {
        self.propagate(apply_cumulative_line(self.html.clone(), color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Pins numbered rank badges (1, 2, 3, ...) on the top N highest-value bars, for quick leaderboard-style reads.",
        fr = "Épingle des badges de classement numérotés (1, 2, 3, ...) sur les N barres aux valeurs les plus élevées, pour une lecture rapide façon classement.",
        aliases("ranks"),
        param(
            name = "top_n",
            ty = "int",
            en = "How many of the highest-value bars to badge. Default: 3.",
            fr = "Nombre de barres (les plus élevées) à marquer d'un badge. Défaut : 3."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Badge fill color. Default: #6366f1.",
            fr = "Couleur de remplissage du badge. Défaut : #6366f1."
        )
    )]
    #[sera_sig(top_n = 3, color = "#6366f1")]
    pub fn rank_badges(&self, top_n: usize, color: &str) -> Chart {
        self.propagate(apply_rank_badges(self.html.clone(), top_n, color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Switches the value axis to a logarithmic scale, like matplotlib's plt.yscale('log') — great for data spanning several orders of magnitude.",
        fr = "Passe l'axe des valeurs en échelle logarithmique, comme plt.yscale('log') de matplotlib — idéal pour des données s'étalant sur plusieurs ordres de grandeur.",
        aliases("logy")
    )]
    pub fn log_scale(&self) -> Chart {
        self.propagate(apply_log_scale(self.html.clone()))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Overlays a moving-average line (trailing window), like pandas' .rolling(window).mean() — smooths out noisy series to reveal the underlying trend.",
        fr = "Superpose une courbe de moyenne mobile (fenêtre glissante), comme .rolling(window).mean() de pandas — lisse une série bruitée pour révéler la tendance sous-jacente.",
        aliases("rolling_mean"),
        param(
            name = "window",
            ty = "int",
            en = "Number of trailing points to average over.",
            fr = "Nombre de points (précédents) sur lesquels moyenner."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Line color. Default: #f59e0b.",
            fr = "Couleur de la courbe. Défaut : #f59e0b."
        )
    )]
    #[sera_sig(window = 3, color = "#f59e0b")]
    pub fn moving_average(&self, window: usize, color: &str) -> Chart {
        self.propagate(apply_moving_average(self.html.clone(), window, color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Flags statistical outliers with a warning marker and a highlighted color. Works on any chart: bars/lollipops/etc are flagged by z-score on their value, scatter points by residual distance from their best-fit line.",
        fr = "Signale les valeurs aberrantes statistiques avec un marqueur d'avertissement et une couleur de mise en évidence. Fonctionne sur tout type de graphique : barres/lollipops/etc sont signalés par z-score sur leur valeur, les points de scatter par distance résiduelle à leur droite de régression.",
        aliases("flag_outliers", "anomalies", "scatter_outliers"),
        param(
            name = "threshold_std",
            ty = "float",
            en = "Number of standard deviations beyond which a value is flagged. Default: 2.0.",
            fr = "Nombre d'écarts-types au-delà duquel une valeur est signalée. Défaut : 2.0."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Highlight color for flagged bars. Default: #ef4444.",
            fr = "Couleur de mise en évidence des barres signalées. Défaut : #ef4444."
        )
    )]
    #[sera_sig(threshold_std = 2.0, color = "#ef4444")]
    pub fn outliers(&self, threshold_std: f64, color: &str) -> Chart {
        self.propagate(apply_outliers(self.html.clone(), threshold_std, color))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("robust_average", "median_trend"),
        file = "charts/chart.md",
        en = "Overlays a rolling-median line (trailing window), the robust-statistics counterpart to moving_average() — a median is not dragged around by a single spike the way a mean is, so this reads as a steadier trend on noisy or outlier-heavy series.",
        fr = "Superpose une courbe de médiane mobile (fenêtre glissante), le pendant robuste de moving_average() — une médiane n'est pas déplacée par un seul pic comme peut l'être une moyenne, donc cette courbe lit une tendance plus stable sur des séries bruitées ou pleines de valeurs aberrantes.",
        param(
            name = "window",
            ty = "int",
            en = "Number of trailing points the median is computed over.",
            fr = "Nombre de points (précédents) sur lesquels la médiane est calculée."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Line color. Default: #a855f7.",
            fr = "Couleur de la courbe. Défaut : #a855f7."
        )
    )]
    #[sera_sig(window = 3, color = "#a855f7")]
    pub fn rolling_median(&self, window: usize, color: &str) -> Chart {
        self.propagate(apply_rolling_median(self.html.clone(), window, color))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("regime_change", "spike_detector"),
        file = "charts/chart.md",
        en = "Marks points where the value jumps abnormally from the previous point — a sequential, point-to-point check, unlike outliers() which flags values far from the dataset's overall distribution. Useful for spotting regime changes or sudden spikes in ordered/time-series data.",
        fr = "Marque les points où la valeur bondit anormalement par rapport au point précédent — une vérification séquentielle, point à point, à la différence de outliers() qui signale les valeurs éloignées de la distribution globale du jeu de données. Utile pour repérer des changements de régime ou des pics soudains dans des données ordonnées/temporelles.",
        param(
            name = "threshold_std",
            ty = "float",
            en = "Number of standard deviations (of the point-to-point deltas) beyond which a jump is flagged. Default: 2.0.",
            fr = "Nombre d'écarts-types (des variations point à point) au-delà duquel un saut est signalé. Défaut : 2.0."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Marker color. Default: #f97316.",
            fr = "Couleur du marqueur. Défaut : #f97316."
        )
    )]
    #[sera_sig(threshold_std = 2.0, color = "#f97316")]
    pub fn change_points(&self, threshold_std: f64, color: &str) -> Chart {
        self.propagate(apply_change_points(self.html.clone(), threshold_std, color))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("rank_all", "full_ranks"),
        file = "charts/chart.md",
        en = "Labels every bar with its rank (#1, #2, ...) by value, without needing to pre-sort the data — the full-dataset counterpart to rank_badges(), which only marks the top N.",
        fr = "Annote chaque barre de son rang (#1, #2, ...) selon sa valeur, sans avoir à trier les données au préalable — le pendant sur tout le jeu de données de rank_badges(), qui ne marque que les N premiers.",
        param(
            name = "ascending",
            ty = "bool",
            en = "If true, rank #1 goes to the smallest value instead of the largest. Default: False.",
            fr = "Si vrai, le rang #1 va à la plus petite valeur plutôt qu'à la plus grande. Défaut : False."
        )
    )]
    #[sera_sig(ascending = false)]
    pub fn rank_labels(&self, ascending: bool) -> Chart {
        self.propagate(apply_rank_labels(self.html.clone(), ascending))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("fill_area", "area_fill"),
        file = "charts/chart.md",
        en = "Shades the area under a line chart's curve down to the baseline, like matplotlib's fill_between(x, y, 0).",
        fr = "Colore la zone sous la courbe d'un line chart jusqu'à la ligne de base, comme fill_between(x, y, 0) de matplotlib.",
        param(
            name = "color",
            ty = "str",
            en = "Fill color. Default: #6366f1.",
            fr = "Couleur de remplissage. Défaut : #6366f1."
        ),
        param(
            name = "opacity",
            ty = "float",
            en = "Fill opacity between 0.0 and 1.0. Default: 0.15.",
            fr = "Opacité du remplissage entre 0.0 et 1.0. Défaut : 0.15."
        )
    )]
    #[sera_sig(color = "#6366f1", opacity = 0.15)]
    pub fn fill_between(&self, color: &str, opacity: f64) -> Chart {
        self.propagate(apply_fill_between(self.html.clone(), color, opacity))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("boxplot"),
        file = "charts/chart.md",
        en = "Draws a box-and-whisker summary (min, Q1, median, Q3, max) of the chart's own values as a small inset glyph, like a built-in boxplot companion.",
        fr = "Dessine un résumé boîte-à-moustaches (min, Q1, médiane, Q3, max) des valeurs du graphique sous forme de petit glyphe en marge, comme un boxplot d'accompagnement intégré.",
        param(
            name = "color",
            ty = "str",
            en = "Box, whiskers and median line color. Default: #6366f1.",
            fr = "Couleur de la boîte, des moustaches et de la médiane. Défaut : #6366f1."
        )
    )]
    #[sera_sig(color = "#6366f1")]
    pub fn box_annotate(&self, color: &str) -> Chart {
        self.propagate(apply_box_annotate(self.html.clone(), color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Labels each bar with its share of the total (%) instead of its raw value.",
        fr = "Annote chaque barre avec sa part du total (%) plutôt que sa valeur brute.",
        aliases("pct"),
        param(
            name = "decimals",
            ty = "int",
            en = "Number of decimal places to show. Default: 1.",
            fr = "Nombre de décimales à afficher. Défaut : 1."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Label text color. Default: #475569.",
            fr = "Couleur du texte du label. Défaut : #475569."
        )
    )]
    #[sera_sig(decimals = 1, color = "#475569")]
    pub fn pct_of_total(&self, decimals: i32, color: &str) -> Chart {
        self.propagate(apply_pct_of_total(self.html.clone(), decimals, color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Computes and displays the Pearson correlation coefficient (r) of a scatter chart's x/y values as a small badge.",
        fr = "Calcule et affiche le coefficient de corrélation de Pearson (r) des valeurs x/y d'un scatter sous forme de petit badge.",
        aliases("corr"),
        param(
            name = "color",
            ty = "str",
            en = "Badge color. Default: #6366f1.",
            fr = "Couleur du badge. Défaut : #6366f1."
        )
    )]
    #[sera_sig(color = "#6366f1")]
    pub fn correlation_badge(&self, color: &str) -> Chart {
        self.propagate(apply_correlation_badge(self.html.clone(), color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Shades a contiguous range of bars/points by index, distinct from reference_band which shades by Y value.",
        fr = "Colore une plage contiguë de barres/points par index, à la différence de reference_band qui colore par valeur Y.",
        aliases("hl_range", "idx_band"),
        param(
            name = "low",
            ty = "int",
            en = "Start index of the range (inclusive).",
            fr = "Index de début de la plage (inclus)."
        ),
        param(
            name = "high",
            ty = "int",
            en = "End index of the range (inclusive).",
            fr = "Index de fin de la plage (inclus)."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Highlight fill color. Default: #6366f1.",
            fr = "Couleur de remplissage de la mise en évidence. Défaut : #6366f1."
        ),
        param(
            name = "opacity",
            ty = "float",
            en = "Fill opacity between 0.0 and 1.0. Default: 0.12.",
            fr = "Opacité du remplissage entre 0.0 et 1.0. Défaut : 0.12."
        )
    )]
    #[sera_sig(low, high, color = "#6366f1", opacity = 0.12)]
    pub fn highlight_range(&self, low: usize, high: usize, color: &str, opacity: f64) -> Chart {
        self.propagate(apply_highlight_range(self.html.clone(), low, high, color, opacity))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("iqr"),
        file = "charts/chart.md",
        en = "Shades a horizontal band across the interquartile range (Q1 to Q3) of the chart's own values, to see at a glance which bars fall inside the 'normal' middle 50%.",
        fr = "Colore une bande horizontale couvrant l'écart interquartile (Q1 à Q3) des valeurs du graphique, pour voir d'un coup d'œil quelles barres se trouvent dans les 50% centraux 'normaux'.",
        param(
            name = "color",
            ty = "str",
            en = "Band fill color. Default: #6366f1.",
            fr = "Couleur de remplissage de la bande. Défaut : #6366f1."
        ),
        param(
            name = "opacity",
            ty = "float",
            en = "Fill opacity between 0.0 and 1.0. Default: 0.10.",
            fr = "Opacité du remplissage entre 0.0 et 1.0. Défaut : 0.10."
        )
    )]
    #[sera_sig(color = "#6366f1", opacity = 0.10)]
    pub fn iqr_band(&self, color: &str, opacity: f64) -> Chart {
        self.propagate(apply_iqr_band(self.html.clone(), color, opacity))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Displays a badge with the total % change between the first and last bar — a quick growth-rate summary.",
        fr = "Affiche un badge avec la variation totale en % entre la première et la dernière barre — un résumé rapide du taux de croissance.",
        aliases("growth"),
        param(
            name = "color",
            ty = "str",
            en = "Badge color. Default: #22c55e.",
            fr = "Couleur du badge. Défaut : #22c55e."
        )
    )]
    #[sera_sig(color = "#22c55e")]
    pub fn growth_badge(&self, color: &str) -> Chart {
        self.propagate(apply_growth_badge(self.html.clone(), color))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("zheat", "heatbars"),
        file = "charts/chart.md",
        en = "Recolors every bar on a diverging blue-white-red scale based on its z-score, turning the chart into a heatmap-as-bars view of which values are unusually high or low.",
        fr = "Recolore chaque barre sur une échelle divergente bleu-blanc-rouge selon son z-score, transformant le graphique en vue façon heatmap des valeurs anormalement hautes ou basses."
    )]
    pub fn zscore_heat(&self) -> Chart {
        self.propagate(apply_zscore_heat(self.html.clone()))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Marks the bar at which the cumulative total crosses a threshold percentage (80% by default), the classic Pareto '80/20' cutoff point. Pairs well with cumulative_line().",
        fr = "Marque la barre à partir de laquelle le total cumulé dépasse un pourcentage seuil (80% par défaut), le point de coupure Pareto '80/20' classique. Se combine bien avec cumulative_line().",
        aliases("pareto", "eighty_twenty"),
        param(
            name = "threshold_pct",
            ty = "float",
            en = "Cumulative percentage threshold to mark. Default: 80.0.",
            fr = "Pourcentage cumulé seuil à marquer. Défaut : 80.0."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Marker line and label color. Default: #ef4444.",
            fr = "Couleur du trait et du label du marqueur. Défaut : #ef4444."
        )
    )]
    #[sera_sig(threshold_pct = 80.0, color = "#ef4444")]
    pub fn pareto_marker(&self, threshold_pct: f64, color: &str) -> Chart {
        self.propagate(apply_pareto_marker(self.html.clone(), threshold_pct, color))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("mean_diff"),
        file = "charts/chart.md",
        en = "Labels each bar with its absolute deviation from the dataset's mean, instead of percentage change versus the previous bar like delta_labels().",
        fr = "Annote chaque barre avec son écart absolu par rapport à la moyenne du jeu de données, à la différence de delta_labels() qui compare au pourcentage de variation par rapport à la barre précédente.",
        param(
            name = "pos_color",
            ty = "str",
            en = "Color used for above-average bars. Default: #22c55e.",
            fr = "Couleur utilisée pour les barres au-dessus de la moyenne. Défaut : #22c55e."
        ),
        param(
            name = "neg_color",
            ty = "str",
            en = "Color used for below-average bars. Default: #ef4444.",
            fr = "Couleur utilisée pour les barres en-dessous de la moyenne. Défaut : #ef4444."
        )
    )]
    #[sera_sig(pos_color = "#22c55e", neg_color = "#ef4444")]
    pub fn diff_from_mean(&self, pos_color: &str, neg_color: &str) -> Chart {
        self.propagate(apply_diff_from_mean(self.html.clone(), pos_color, neg_color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Overlays a Bollinger-band-style shaded region (rolling mean ± rolling standard deviation) over a trailing window, the classic volatility envelope from financial charting.",
        fr = "Superpose une zone ombrée façon bandes de Bollinger (moyenne mobile ± écart-type mobile) sur une fenêtre glissante, l'enveloppe de volatilité classique des graphiques financiers.",
        aliases("bollinger_band", "volatility_band"),
        param(
            name = "window",
            ty = "int",
            en = "Number of trailing points used for the rolling mean and standard deviation.",
            fr = "Nombre de points (précédents) utilisés pour la moyenne et l'écart-type mobiles."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Band fill color. Default: #6366f1.",
            fr = "Couleur de remplissage de la bande. Défaut : #6366f1."
        ),
        param(
            name = "opacity",
            ty = "float",
            en = "Fill opacity between 0.0 and 1.0. Default: 0.15.",
            fr = "Opacité du remplissage entre 0.0 et 1.0. Défaut : 0.15."
        )
    )]
    #[sera_sig(window = 5, color = "#6366f1", opacity = 0.15)]
    pub fn rolling_std_band(&self, window: usize, color: &str, opacity: f64) -> Chart {
        self.propagate(apply_rolling_std_band(self.html.clone(), window, color, opacity))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Extrapolates a linear-regression trend beyond the last data point, with a dashed line and ghost markers for each forecast step.",
        fr = "Extrapole une tendance de régression linéaire au-delà du dernier point de données, avec un trait pointillé et des marqueurs fantômes pour chaque pas de prévision.",
        aliases("forecast", "extrapolate"),
        param(
            name = "periods",
            ty = "int",
            en = "Number of future periods to forecast. Default: 3.",
            fr = "Nombre de périodes futures à prévoir. Défaut : 3."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Forecast line and marker color. Default: #8b5cf6.",
            fr = "Couleur du trait et des marqueurs de prévision. Défaut : #8b5cf6."
        )
    )]
    #[sera_sig(periods = 3, color = "#8b5cf6")]
    pub fn forecast_line(&self, periods: usize, color: &str) -> Chart {
        self.propagate(apply_forecast_line(self.html.clone(), periods, color))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Shades a horizontal band between two arbitrary percentiles of the chart's own values, a generalization of iqr_band for any custom range (e.g. 5th-95th).",
        fr = "Colore une bande horizontale entre deux percentiles arbitraires des valeurs du graphique, une généralisation de iqr_band pour toute plage personnalisée (ex : 5e-95e).",
        aliases("pct_band"),
        param(
            name = "low_pct",
            ty = "float",
            en = "Lower percentile (0-100).",
            fr = "Percentile bas (0-100)."
        ),
        param(
            name = "high_pct",
            ty = "float",
            en = "Upper percentile (0-100).",
            fr = "Percentile haut (0-100)."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Band fill color. Default: #6366f1.",
            fr = "Couleur de remplissage de la bande. Défaut : #6366f1."
        ),
        param(
            name = "opacity",
            ty = "float",
            en = "Fill opacity between 0.0 and 1.0. Default: 0.10.",
            fr = "Opacité du remplissage entre 0.0 et 1.0. Défaut : 0.10."
        )
    )]
    #[sera_sig(low_pct, high_pct, color = "#6366f1", opacity = 0.10)]
    pub fn percentile_band(&self, low_pct: f64, high_pct: f64, color: &str, opacity: f64) -> Chart {
        self.propagate(apply_percentile_band(self.html.clone(), low_pct, high_pct, color, opacity))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Fits and draws a linear regression line through a scatter chart's points, like seaborn's regplot() or numpy.polyfit(x, y, 1).",
        fr = "Calcule et trace une droite de régression linéaire à travers les points d'un scatter, comme regplot() de seaborn ou numpy.polyfit(x, y, 1).",
        aliases("regression_line", "lmplot"),
        param(
            name = "color",
            ty = "str",
            en = "Line color. Default: #ef4444.",
            fr = "Couleur de la droite. Défaut : #ef4444."
        ),
        param(
            name = "width",
            ty = "float",
            en = "Line stroke width. Default: 2.0.",
            fr = "Épaisseur de la droite. Défaut : 2.0."
        )
    )]
    #[sera_sig(color = "#ef4444", width = 2.0)]
    pub fn scatter_regression(&self, color: &str, width: f64) -> Chart {
        self.propagate(apply_scatter_regression(self.html.clone(), color, width))
    }

    #[sera_doc(
        category = "chart_method",
        file = "charts/chart.md",
        en = "Runs DBSCAN density clustering and recolors each element by its cluster, in-browser and dependency-free. Works on any chart: uses x/y for scatter points, or falls back to the single value for bars/lollipops/etc. Noise points (no dense neighborhood) are colored gray.",
        fr = "Exécute un clustering par densité DBSCAN et recolore chaque élément selon son cluster, directement dans le navigateur sans dépendance. Fonctionne sur tout type de graphique : utilise x/y pour un scatter, ou se rabat sur la valeur seule pour les barres/lollipops/etc. Les points de bruit (sans voisinage dense) sont colorés en gris.",
        aliases("dbscan"),
        param(
            name = "eps",
            ty = "float",
            en = "Maximum distance (in data units) between two points for them to be considered neighbors.",
            fr = "Distance maximale (en unités de données) entre deux points pour qu'ils soient considérés voisins."
        ),
        param(
            name = "min_samples",
            ty = "int",
            en = "Minimum neighborhood size (including the point itself) to seed a cluster. Default: 3.",
            fr = "Taille minimale de voisinage (point inclus) pour amorcer un cluster. Défaut : 3."
        )
    )]
    #[sera_sig(eps, min_samples = 3)]
    pub fn cluster(&self, eps: f64, min_samples: usize) -> Chart {
        self.propagate(apply_cluster(self.html.clone(), eps, min_samples))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("select_compare", "element_compare"),
        file = "charts/chart.md",
        en = "Enables intra-chart shift-click element comparison. Shift-click any two or more bars or points to select them — unselected elements dim to 18% opacity, and a badge above the plot shows the selected values ranked with >, >>, >>> arrows. Shift-click a selected element again to deselect it.",
        fr = "Active la comparaison d'éléments dans le graphique par shift-clic. Shift-cliquez deux barres ou points ou plus — les éléments non sélectionnés s'estompent à 18% d'opacité, et un badge au-dessus affiche les valeurs sélectionnées classées avec >, >>, >>>. Shift-cliquez à nouveau pour désélectionner."
    )]
    pub fn pick(&self) -> Chart {
        self.propagate(
            self.html
                .replacen("</body>", &format!("<script>{}</script></body>", SP_PICK_JS), 1),
        )
    }

    #[sera_doc(
        category = "chart_method",
        aliases("sunburst_focus", "layer_focus", "slice_focus", "hover_dim_slices", "plotly_focus"),
        file = "charts/chart.md",
        en = "On hover, dims all non-hovered plotly slices (sunburst, pie, treemap) to near-invisible and hides their text labels — only the hovered segment and its ancestors stay fully visible. Double-click resets. Ideal for exploring multi-level sunburst hierarchies.",
        fr = "Au survol, estompe toutes les tranches Plotly non survolées (sunburst, pie, treemap) et masque leurs labels — seul le segment survolé et ses ancêtres restent visibles. Double-clic pour réinitialiser. Idéal pour explorer les hiérarchies sunburst multi-niveaux."
    )]
    pub fn focus_mode(&self) -> Chart {
        self.propagate(
            self.html
                .replacen("</body>", &format!("<script>{}</script></body>", SP_FOCUS_MODE_JS), 1),
        )
    }

    #[sera_doc(
        category = "chart_method",
        aliases("focus", "downscale", "highlight_top", "fade_others", "keep_only", "isolate", "dim_rest", "show_only"),
        file = "charts/chart.md",
        en = "Dims every element NOT in the given index list. Pass a list of indices to keep at full opacity — everything else fades to `opacity`. Chainable with dim_below / dim_above for combined effects.",
        fr = "Estompe tous les éléments qui ne sont PAS dans la liste d'indices. Passez une liste d'indices à garder en pleine opacité — tout le reste s'estompe à `opacity`. Chaînable avec dim_below / dim_above.",
        param(name = "keep", ty = "list[int]", en = "Indices of elements to keep at full opacity.", fr = "Indices des éléments à garder en pleine opacité."),
        param(name = "opacity", ty = "float", en = "Opacity applied to dimmed elements (0.0–1.0). Default 0.2.", fr = "Opacité des éléments estompés (0.0–1.0). Défaut 0.2.")
    )]
    #[sera_sig(keep, opacity = 0.2)]
    pub fn dim_others(&self, keep: Vec<usize>, opacity: Option<f64>) -> Chart {
        let dim = opacity.unwrap_or(0.2_f64).clamp(0.0, 1.0);
        let keep_js = format!("[{}]", keep.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","));
        let js = format!(
            "(function(){{var svg=document.querySelector('svg');if(!svg)return;var keep={};var dim={};svg.querySelectorAll('[data-idx]').forEach(function(e){{var bright=keep.indexOf(parseInt(e.getAttribute('data-idx')))>=0;e.style.opacity=bright?'1':String(dim);e.style.transition='opacity .2s';}});}})();",
            keep_js, dim
        );
        self.propagate(self.html.replacen("</body>", &format!("<script>{}</script></body>", js), 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("dim_under", "fade_below", "fade_under", "highlight_above", "focus_above", "dim_low", "below_threshold"),
        file = "charts/chart.md",
        en = "Dims elements whose value is strictly below the threshold — those at or above stay bright. Use for emphasizing top performers or highlighting bars that clear a target line.",
        fr = "Estompe les éléments dont la valeur est strictement inférieure au seuil — ceux au-dessus ou égaux restent lumineux. Pratique pour mettre en avant les meilleures performances.",
        param(name = "threshold", ty = "float", en = "Value threshold. Elements with value >= threshold stay at full opacity.", fr = "Seuil de valeur. Les éléments avec valeur >= seuil restent en pleine opacité."),
        param(name = "opacity", ty = "float", en = "Opacity applied to dimmed elements. Default 0.2.", fr = "Opacité des éléments estompés. Défaut 0.2.")
    )]
    #[sera_sig(threshold, opacity = 0.2)]
    pub fn dim_below(&self, threshold: f64, opacity: Option<f64>) -> Chart {
        let dim = opacity.unwrap_or(0.2_f64).clamp(0.0, 1.0);
        let js = format!(
            "(function(){{var svg=document.querySelector('svg');if(!svg)return;var thr={};var dim={};svg.querySelectorAll('[data-idx][data-v],[data-idx][data-y]').forEach(function(e){{var v=parseFloat(e.getAttribute('data-v')||e.getAttribute('data-y'));var bright=!isNaN(v)&&v>=thr;e.style.opacity=bright?'1':String(dim);e.style.transition='opacity .2s';}});}})();",
            threshold, dim
        );
        self.propagate(self.html.replacen("</body>", &format!("<script>{}</script></body>", js), 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("dim_over", "fade_above", "fade_over", "highlight_below", "focus_below", "dim_high", "above_threshold"),
        file = "charts/chart.md",
        en = "Dims elements whose value is strictly above the threshold — those at or below stay bright. Useful for emphasizing low-value regions or anomalies under a cap.",
        fr = "Estompe les éléments dont la valeur est strictement supérieure au seuil — ceux en dessous ou égaux restent lumineux. Utile pour mettre en avant les régions de faibles valeurs.",
        param(name = "threshold", ty = "float", en = "Value threshold. Elements with value <= threshold stay at full opacity.", fr = "Seuil de valeur. Les éléments avec valeur <= seuil restent en pleine opacité."),
        param(name = "opacity", ty = "float", en = "Opacity applied to dimmed elements. Default 0.2.", fr = "Opacité des éléments estompés. Défaut 0.2.")
    )]
    #[sera_sig(threshold, opacity = 0.2)]
    pub fn dim_above(&self, threshold: f64, opacity: Option<f64>) -> Chart {
        let dim = opacity.unwrap_or(0.2_f64).clamp(0.0, 1.0);
        let js = format!(
            "(function(){{var svg=document.querySelector('svg');if(!svg)return;var thr={};var dim={};svg.querySelectorAll('[data-idx][data-v],[data-idx][data-y]').forEach(function(e){{var v=parseFloat(e.getAttribute('data-v')||e.getAttribute('data-y'));var bright=!isNaN(v)&&v<=thr;e.style.opacity=bright?'1':String(dim);e.style.transition='opacity .2s';}});}})();",
            threshold, dim
        );
        self.propagate(self.html.replacen("</body>", &format!("<script>{}</script></body>", js), 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("cmp_ticker"),
        file = "charts/chart.md",
        en = "Enables shift-click comparison ticker mode. Shift-click two or more charts to spawn a floating ticker showing which chart scores higher (>, >>, >>>). Shift-click a selected chart again to deselect it.",
        fr = "Active le mode ticker de comparaison par shift-clic. Shift-cliquez deux graphiques ou plus pour faire apparaître un ticker flottant. Shift-cliquez à nouveau pour désélectionner."
    )]
    #[sera_python_skip]
    pub fn compare_ticker(&self) -> Chart {
        self.propagate(
            self.html
                .replacen("</body>", &format!("<script>{}</script></body>", SP_COMPARE_JS), 1),
        )
    }

    #[sera_doc(
        category = "chart_method",
        aliases("draw_line", "draw", "overlay_line", "rule", "add_line", "annotate_line", "line", "guideline", "marker_line"),
        file = "charts/chart.md",
        en = "Draws a line on the chart. Fractional plot-area coordinates (0.0 = bottom/left, 1.0 = top/right) for straight lines; pass `values` for a data-driven polyline overlay; `vertical=True` draws a vertical rule at x1. Chainable.",
        fr = "Trace une ligne sur le graphique. Coordonnées fractionnelles pour lignes droites ; `values` pour une superposition de polyline ; `vertical=True` pour une règle verticale. Chaînable.",
        param(name = "x1", ty = "float", en = "Start X fraction (0.0–1.0). Used as the vertical rule position when vertical=True.", fr = "X de départ (0.0–1.0). Position de la règle verticale si vertical=True."),
        param(name = "y1", ty = "float", en = "Start Y fraction (0.0 = bottom, 1.0 = top).", fr = "Y de départ (0.0 = bas, 1.0 = haut)."),
        param(name = "x2", ty = "float", en = "End X fraction (0.0–1.0).", fr = "X de fin (0.0–1.0)."),
        param(name = "y2", ty = "float", en = "End Y fraction (0.0 = bottom, 1.0 = top).", fr = "Y de fin (0.0 = bas, 1.0 = haut)."),
        param(name = "color", ty = "str | None", en = "Line color. Default: #ef4444.", fr = "Couleur de la ligne. Défaut : #ef4444."),
        param(name = "dash", ty = "str | None", en = "Stroke style: 'solid' (default), 'dashed', or 'dotted'.", fr = "Style du trait : 'solid' (défaut), 'dashed' ou 'dotted'."),
        param(name = "width", ty = "float", en = "Stroke width in pixels. Default: 2.0.", fr = "Épaisseur du trait en pixels. Défaut : 2.0."),
        param(name = "vertical", ty = "bool", en = "When True, draws a vertical rule at x1 spanning y1→y2 (defaults to full height).", fr = "Si True, trace une règle verticale en x1 de y1 à y2 (hauteur totale par défaut)."),
        param(name = "values", ty = "list[float] | None", en = "Data values to draw as a polyline overlay, mapped to the chart's Y-axis scale and distributed evenly across the X range.", fr = "Valeurs à tracer comme polyline superposée, mappées sur l'axe Y du graphique et distribuées sur la plage X.")
    )]
    #[sera_sig(x1 = 0.0, y1 = 0.0, x2 = 1.0, y2 = 1.0, color = None, dash = None, width = 2.0, vertical = false, values = None)]
    pub fn trace(
        &self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        color: Option<&str>,
        dash: Option<&str>,
        width: Option<f64>,
        vertical: Option<bool>,
        values: Option<Vec<f64>>,
    ) -> Chart {
        let c = json_str(color.unwrap_or("#ef4444"));
        let lw = width.unwrap_or(2.0);
        let da = match dash.unwrap_or("solid") {
            "dashed" => "\"6,4\"",
            "dotted" => "\"2,4\"",
            _ => "null",
        };
        let js = if let Some(vals) = values {
            if vals.len() < 2 {
                return self.propagate(self.html.clone());
            }
            let vals_js = format!("[{}]", vals.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(","));
            format!(
                "(function(){{var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pL=sp[0]||0,pT=sp[1]||0,pW=sp[2]||parseFloat(svg.getAttribute('width'))||900,pH=sp[3]||parseFloat(svg.getAttribute('height'))||500;var c={c};var lw={lw};var da={da};var vals={vals};var yts=svg.querySelectorAll('.sp-yt');var tvs=[];yts.forEach(function(t){{var v=parseFloat(t.textContent);if(!isNaN(v))tvs.push(v);}});var minV=tvs.length>=2?Math.min.apply(null,tvs):Math.min.apply(null,vals);var maxV=tvs.length>=2?Math.max.apply(null,tvs):Math.max.apply(null,vals);if(maxV<=minV)return;var n=vals.length;var pts=vals.map(function(v,i){{var x=pL+i/(n-1)*pW;var y=pT+pH-(v-minV)/(maxV-minV)*pH;return x+','+y;}}).join(' ');var ns='http://www.w3.org/2000/svg';var pl=document.createElementNS(ns,'polyline');pl.setAttribute('points',pts);pl.setAttribute('fill','none');pl.setAttribute('stroke',c);pl.setAttribute('stroke-width',lw);if(da)pl.setAttribute('stroke-dasharray',da);pl.setAttribute('stroke-linecap','round');pl.setAttribute('stroke-linejoin','round');pl.setAttribute('pointer-events','none');svg.appendChild(pl);}})();",
                c = c, lw = lw, da = da, vals = vals_js
            )
        } else if vertical.unwrap_or(false) {
            format!(
                "(function(){{var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pL=sp[0]||0,pT=sp[1]||0,pW=sp[2]||parseFloat(svg.getAttribute('width'))||900,pH=sp[3]||parseFloat(svg.getAttribute('height'))||500;var c={c};var lw={lw};var da={da};var ns='http://www.w3.org/2000/svg';var ln=document.createElementNS(ns,'line');var vx=pL+{x1:.4}*pW;ln.setAttribute('x1',vx);ln.setAttribute('x2',vx);ln.setAttribute('y1',pT+(1-{y1:.4})*pH);ln.setAttribute('y2',pT+(1-{y2:.4})*pH);ln.setAttribute('stroke',c);ln.setAttribute('stroke-width',lw);ln.setAttribute('stroke-linecap','round');if(da)ln.setAttribute('stroke-dasharray',da);ln.setAttribute('pointer-events','none');svg.appendChild(ln);}})();",
                c = c, lw = lw, da = da, x1 = x1, y1 = y1, y2 = y2
            )
        } else {
            format!(
                "(function(){{var svg=document.querySelector('svg');if(!svg)return;var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pL=sp[0]||0,pT=sp[1]||0,pW=sp[2]||parseFloat(svg.getAttribute('width'))||900,pH=sp[3]||parseFloat(svg.getAttribute('height'))||500;var c={c};var lw={lw};var da={da};var ns='http://www.w3.org/2000/svg';var ln=document.createElementNS(ns,'line');ln.setAttribute('x1',pL+{x1:.4}*pW);ln.setAttribute('y1',pT+(1-{y1:.4})*pH);ln.setAttribute('x2',pL+{x2:.4}*pW);ln.setAttribute('y2',pT+(1-{y2:.4})*pH);ln.setAttribute('stroke',c);ln.setAttribute('stroke-width',lw);ln.setAttribute('stroke-linecap','round');if(da)ln.setAttribute('stroke-dasharray',da);ln.setAttribute('pointer-events','none');svg.appendChild(ln);}})();",
                c = c, lw = lw, da = da, x1 = x1, y1 = y1, x2 = x2, y2 = y2
            )
        };
        self.propagate(
            self.html
                .replacen("</body>", &format!("<script>{}</script></body>", js), 1),
        )
    }

    #[sera_doc(
        category = "chart_method",
        aliases("mean_line", "avg_line"),
        file = "charts/chart.md",
        en = "Draws a dashed horizontal mean line across the plot area, computed from the chart's data values (works on bar, scatter, and other value-based charts).",
        fr = "Trace une ligne horizontale en pointillés au niveau de la moyenne, calculée à partir des données du graphique (fonctionne sur les barres, scatter et autres graphiques à valeurs).",
        param(name = "color", ty = "str | None", en = "Line color. Default: #ef4444.", fr = "Couleur de la ligne. Défaut : #ef4444.")
    )]
    #[sera_sig(color = None)]
    pub fn mean(&self, color: Option<&str>) -> Chart {
        let c = json_str(color.unwrap_or("#ef4444"));
        let snippet = format!(
            "<script>window.__sp_mean__={{c:{}}};{}</script></body>",
            c, SP_MEAN_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("median_line", "med_line"),
        file = "charts/chart.md",
        en = "Draws a dashed horizontal median line across the plot area, computed from the chart's data values (works on bar, scatter, and other value-based charts).",
        fr = "Trace une ligne horizontale en pointillés au niveau de la médiane, calculée à partir des données du graphique (fonctionne sur les barres, scatter et autres graphiques à valeurs).",
        param(name = "color", ty = "str | None", en = "Line color. Default: #6366f1.", fr = "Couleur de la ligne. Défaut : #6366f1.")
    )]
    #[sera_sig(color = None)]
    pub fn median(&self, color: Option<&str>) -> Chart {
        let c = json_str(color.unwrap_or("#6366f1"));
        let snippet = format!(
            "<script>window.__sp_median__={{c:{}}};{}</script></body>",
            c, SP_MEDIAN_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("frame_cols", "highlight_cols", "cluster_frame"),
        file = "charts/chart.md",
        en = "Draws a visual cluster frame around specified heatmap columns: a subtle fill, column separator grid lines, a border rectangle with corner L-brackets, and a looping zoom animation that pulses the viewBox in and out of the cluster region.",
        fr = "Dessine un cadrage visuel autour de colonnes de heatmap spécifiées : un fond subtil, des séparateurs de colonnes, un rectangle de bordure avec des coins en L, et une animation de zoom en boucle qui pulse le viewBox sur la zone du cluster.",
        param(
            name = "cols",
            ty = "list[str]",
            en = "Column labels to frame (must match the heatmap col_labels).",
            fr = "Étiquettes de colonnes à encadrer (doivent correspondre aux col_labels de la heatmap)."
        ),
        param(
            name = "rows",
            ty = "list[str] | None",
            en = "Row labels to restrict the frame to. Default: all rows.",
            fr = "Étiquettes de lignes pour restreindre le cadrage. Défaut : toutes les lignes."
        ),
        param(
            name = "color",
            ty = "str | None",
            en = "Frame and separator color. Default: #6366f1.",
            fr = "Couleur du cadre et des séparateurs. Défaut : #6366f1."
        )
    )]
    #[sera_sig(cols, rows = None, color = None)]
    pub fn cadrage(&self, cols: Vec<String>, rows: Option<Vec<String>>, color: Option<&str>) -> Chart {
        let cols_json = format!("[{}]", cols.iter().map(|c| json_str(c)).collect::<Vec<_>>().join(","));
        let rows_json = match rows {
            Some(ref r) => format!("[{}]", r.iter().map(|s| json_str(s)).collect::<Vec<_>>().join(",")),
            None => "null".into(),
        };
        let fc = json_str(color.unwrap_or("#6366f1"));
        let snippet = format!(
            "<script>window.__sp_cadrage__={{c:{},r:{},f:{}}};{}</script></body>",
            cols_json, rows_json, fc, SP_CADRAGE_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("round_x_ticks", "x_ticks_nice", "format_x_axis"),
        file = "charts/chart.md",
        en = "Reformats x-axis tick labels to rounded values. If step is given, ticks become 0, step, 2*step… (uniform); otherwise each tick is rounded to the nearest auto-computed nice step.",
        fr = "Reformate les étiquettes de l'axe x en valeurs arrondies. Si step est fourni, les ticks deviennent 0, step, 2*step… ; sinon chaque tick est arrondi au step automatique.",
        param(name = "step", ty = "float | None", en = "Tick interval for uniform spacing. None = auto.", fr = "Intervalle uniforme ou None pour auto.")
    )]
    #[sera_sig(step = None)]
    pub fn nice_x_axis(&self, step: Option<f64>) -> Chart {
        let step_js = match step {
            Some(s) => format!("{}", s),
            None => "null".to_string(),
        };
        let js = format!(
            "(function(){{var ts=Array.prototype.slice.call(document.querySelectorAll('.sp-xt'));if(ts.length<2)return;ts.sort(function(a,b){{return parseFloat(a.getAttribute('x'))-parseFloat(b.getAttribute('x'));}});function parseV(t){{var s=t.textContent.trim();return s.indexOf('k')>=0?parseFloat(s)*1000:parseFloat(s);}}function fmt(v){{if(v===0)return'0';if(v>=10000)return Math.round(v/1000)+'k';return''+Math.round(v);}}var step={};if(step!==null){{ts.forEach(function(t,i){{t.textContent=fmt(i*step);}});}}else{{var maxV=parseV(ts[ts.length-1]);if(maxV<=0)return;var rawStep=maxV/(ts.length-1);var mag=Math.pow(10,Math.floor(Math.log10(rawStep)));var niceStep=Math.round(rawStep/mag)*mag||mag;ts.forEach(function(t){{var v=parseV(t);t.textContent=fmt(Math.round(v/niceStep)*niceStep);}});}}}})()",
            step_js
        );
        let snippet = format!("<script>{}</script></body>", js);
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("bloom", "neon_glow", "plasma"),
        file = "charts/chart.md",
        en = "Adds a multi-layer neon bloom effect to SVG elements — 3 stacked feGaussianBlur passes create a deep luminous glow around bars, lines and circles.",
        fr = "Ajoute un effet neon bloom multicouche aux elements SVG — 3 passes feGaussianBlur creent une lueur lumineuse profonde autour des barres, lignes et cercles.",
        param(
            name = "color",
            ty = "str | None",
            en = "Bloom glow color as a CSS hex string. Default: #6366f1.",
            fr = "Couleur de lueur en hexadecimal. Defaut : #6366f1."
        )
    )]
    #[sera_sig(color = None)]
    pub fn neon_bloom(&self, color: Option<&str>) -> Chart {
        self.propagate(apply_neon_bloom(self.html.clone(), color.unwrap_or("#6366f1")))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("dark_bg", "space_bg", "cosmos_bg", "night_bg"),
        file = "charts/chart.md",
        en = "Applies a dark void background with animated star particles injected into the SVG — turns any chart into a cinematic space scene.",
        fr = "Applique un fond sombre avec des particules stellaires animees injectees dans le SVG — transforme tout graphique en scene spatiale cinematique.",
        param(
            name = "color",
            ty = "str | None",
            en = "Base background color. Default: #07090f.",
            fr = "Couleur de fond de base. Defaut : #07090f."
        )
    )]
    #[sera_sig(color = None)]
    pub fn void_bg(&self, color: Option<&str>) -> Chart {
        self.propagate(apply_void(self.html.clone(), color.unwrap_or("#07090f")))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("pattern_fill", "hatch", "hatching", "fill_pattern", "shape_texture"),
        file = "charts/chart.md",
        en = "Overlays a repeating SVG pattern (dots, diagonal, crosshatch, grid, noise) on every filled element, without touching each element's own color underneath — a texture layer that reads consistently across bars, points, arcs and any other shape, on any chart family. Same idea as neon_bloom()/void_bg() as a chainable aesthetic theme, but for texture instead of glow or background.",
        fr = "Superpose un motif SVG répétitif (points, diagonal, croisillon, grille, bruit) sur chaque élément rempli, sans toucher à la couleur propre de chaque élément en dessous — une couche de texture cohérente sur les barres, points, arcs et toute autre forme, sur n'importe quelle famille de graphique. Même principe que neon_bloom()/void_bg() en tant que thème esthétique chaînable, mais pour la texture plutôt que la lueur ou le fond.",
        param(
            name = "pattern",
            ty = "str",
            en = "Pattern name: 'dots', 'diagonal', 'crosshatch', 'grid', or 'noise'. Default: 'diagonal'.",
            fr = "Nom du motif : 'dots', 'diagonal', 'crosshatch', 'grid' ou 'noise'. Défaut : 'diagonal'."
        ),
        param(
            name = "opacity",
            ty = "float",
            en = "Overlay opacity between 0.0 and 1.0. Default: 0.2.",
            fr = "Opacité de la superposition entre 0.0 et 1.0. Défaut : 0.2."
        )
    )]
    #[sera_sig(pattern = "diagonal", opacity = 0.2)]
    pub fn pattern_overlay(&self, pattern: &str, opacity: f64) -> Chart {
        self.propagate(apply_texture(self.html.clone(), pattern, opacity))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("gap_annotation", "compare_bars", "delta_bracket", "bracket_diff"),
        file = "charts/chart.md",
        en = "Draws a bracket connecting two specific data-point indices with the absolute and percentage difference between them — for comparing two arbitrary elements directly (e.g. Q1 vs Q4), unlike delta_labels() which only shows each point's change versus its immediate predecessor.",
        fr = "Trace une accolade reliant deux index de points de donnée précis avec la différence absolue et en pourcentage entre eux — pour comparer deux éléments arbitraires directement (ex. T1 vs T4), contrairement à delta_labels() qui ne montre que le changement de chaque point face à son prédécesseur immédiat.",
        param(name = "idx1", ty = "int", en = "Index of the first data point.", fr = "Index du premier point de donnée."),
        param(name = "idx2", ty = "int", en = "Index of the second data point.", fr = "Index du second point de donnée."),
        param(name = "color", ty = "str", en = "Bracket and label color. Default: #6366f1.", fr = "Couleur de l'accolade et du label. Défaut : #6366f1.")
    )]
    #[sera_sig(idx1, idx2, color = "#6366f1")]
    pub fn annotate_gap(&self, idx1: usize, idx2: usize, color: &str) -> Chart {
        self.propagate(apply_annotate_gap(self.html.clone(), idx1, idx2, color))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("annotate", "speech_bubble", "point_out", "flag_point"),
        file = "charts/chart.md",
        en = "Draws a free-text speech-bubble callout pointing at a specific data-point index — for a manual note ('data revised', 'one-off event') rather than a computed label like value_labels()/delta_labels().",
        fr = "Dessine une bulle de texte libre pointant vers un index de point de donnée précis — pour une note manuelle ('donnée révisée', 'événement ponctuel') plutôt qu'un label calculé comme value_labels()/delta_labels().",
        param(name = "index", ty = "int", en = "Data-point index to point at.", fr = "Index du point de donnée visé."),
        param(name = "text", ty = "str", en = "Callout text.", fr = "Texte de la bulle."),
        param(name = "color", ty = "str", en = "Callout background color. Default: #1e293b.", fr = "Couleur de fond de la bulle. Défaut : #1e293b.")
    )]
    #[sera_sig(index, text, color = "#1e293b")]
    pub fn callout(&self, index: usize, text: &str, color: &str) -> Chart {
        self.propagate(apply_callout(self.html.clone(), index, text, color))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("crossing_markers", "threshold_markers", "cross_points"),
        file = "charts/chart.md",
        en = "Marks every point where the series crosses a threshold value, with an up or down arrow, plus a dashed reference line at the threshold — answers 'when did we cross $1M' rather than flagging every value above/below it like reference_band()/highlight_range().",
        fr = "Marque chaque point où la série franchit une valeur seuil, avec une flèche haut ou bas, plus une ligne de référence en pointillés au niveau du seuil — répond à « quand a-t-on dépassé 1M€ » plutôt que de signaler toutes les valeurs au-dessus/en-dessous comme reference_band()/highlight_range().",
        param(name = "threshold", ty = "float", en = "Threshold value to detect crossings of.", fr = "Valeur seuil dont on détecte les franchissements."),
        param(name = "up_color", ty = "str", en = "Color for upward crossings. Default: #22c55e.", fr = "Couleur des franchissements à la hausse. Défaut : #22c55e."),
        param(name = "down_color", ty = "str", en = "Color for downward crossings. Default: #ef4444.", fr = "Couleur des franchissements à la baisse. Défaut : #ef4444.")
    )]
    #[sera_sig(threshold, up_color = "#22c55e", down_color = "#ef4444")]
    pub fn threshold_crossings(&self, threshold: f64, up_color: &str, down_color: &str) -> Chart {
        self.propagate(apply_threshold_crossings(self.html.clone(), threshold, up_color, down_color))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("peaks_troughs", "local_extrema", "peak_finder", "swing_points"),
        file = "charts/chart.md",
        en = "Marks every local peak and trough along the series — not just the global max/min like annotate_max()/annotate_min() — with a small triangle above each peak and below each trough. Works on bar and scatter charts (ordered by data-idx).",
        fr = "Marque chaque pic et creux local le long de la série — pas seulement le max/min global comme annotate_max()/annotate_min() — avec un petit triangle au-dessus de chaque pic et en dessous de chaque creux. Fonctionne sur les charts bar et scatter (ordonnés par data-idx).",
        param(
            name = "peak_color",
            ty = "str",
            en = "Peak marker color. Default: #22c55e.",
            fr = "Couleur du marqueur de pic. Défaut : #22c55e."
        ),
        param(
            name = "trough_color",
            ty = "str",
            en = "Trough marker color. Default: #ef4444.",
            fr = "Couleur du marqueur de creux. Défaut : #ef4444."
        )
    )]
    #[sera_sig(peak_color = "#22c55e", trough_color = "#ef4444")]
    pub fn turning_points(&self, peak_color: &str, trough_color: &str) -> Chart {
        self.propagate(apply_turning_points(self.html.clone(), peak_color, trough_color))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("control_band", "process_band", "std_envelope"),
        file = "charts/chart.md",
        en = "Shades a static mean ± n·σ band across the whole series with a dashed mean line — the classic statistical process control envelope, computed once over the full dataset (unlike rolling_std_band(), which is a trailing window). Works on bar, scatter, and other value-based charts.",
        fr = "Affiche une bande statique moyenne ± n·σ sur toute la série avec une ligne de moyenne en pointillés — l'enveloppe classique de contrôle statistique de processus, calculée une fois sur tout le dataset (contrairement à rolling_std_band(), qui utilise une fenêtre glissante). Fonctionne sur les charts bar, scatter et autres charts basés sur des valeurs.",
        param(
            name = "n",
            ty = "float",
            en = "Number of standard deviations on each side of the mean. Default: 2.0.",
            fr = "Nombre d'écarts-types de chaque côté de la moyenne. Défaut : 2.0."
        ),
        param(
            name = "color",
            ty = "str",
            en = "Band and mean-line color. Default: #6366f1.",
            fr = "Couleur de la bande et de la ligne de moyenne. Défaut : #6366f1."
        ),
        param(
            name = "opacity",
            ty = "float",
            en = "Band fill opacity between 0.0 and 1.0. Default: 0.15.",
            fr = "Opacité de remplissage entre 0.0 et 1.0. Défaut : 0.15."
        )
    )]
    #[sera_sig(n = 2.0, color = "#6366f1", opacity = 0.15)]
    pub fn sigma_bands(&self, n: f64, color: &str, opacity: f64) -> Chart {
        self.propagate(apply_sigma_bands(self.html.clone(), n, color, opacity))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("summary_badge", "kpi_stats", "stats_panel", "describe_badge"),
        file = "charts/chart.md",
        en = "Draws a floating summary box with n, mean, median, std, min and max computed from the chart's own rendered data — a pandas describe() at a glance, directly on the chart. Works on bar, scatter, and other value-based charts.",
        fr = "Affiche un encadré flottant avec n, moyenne, médiane, écart-type, min et max calculés depuis les données déjà rendues du graphique — un describe() pandas en un coup d'œil, directement sur le graphique. Fonctionne sur les charts bar, scatter et autres charts basés sur des valeurs.",
        param(
            name = "color",
            ty = "str",
            en = "Box border and value text color. Default: #6366f1.",
            fr = "Couleur de la bordure et des valeurs. Défaut : #6366f1."
        )
    )]
    #[sera_sig(color = "#6366f1")]
    pub fn stats_badge(&self, color: &str) -> Chart {
        self.propagate(apply_stats_badge(self.html.clone(), color))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("annotate_events", "milestones", "mark_events", "event_lines"),
        file = "charts/chart.md",
        en = "Drops a dashed vertical line with a rotated label at each given data-point index — mark real-world events (a launch, a policy change, an incident) directly on the timeline instead of computing pixel coordinates by hand like trace(vertical=True). Works on bar and scatter charts.",
        fr = "Ajoute une ligne verticale en pointillés avec un label pivoté à chaque index de point de donnée indiqué — marquez des événements réels (lancement, changement de politique, incident) directement sur la chronologie sans calculer de coordonnées pixel à la main comme avec trace(vertical=True). Fonctionne sur les charts bar et scatter.",
        param(name = "indices", ty = "list[int]", en = "Data-point indices to mark, matching each element's data-idx.", fr = "Indices des points de donnée à marquer, correspondant au data-idx de chaque élément."),
        param(name = "labels", ty = "list[str]", en = "Label text for each index, same length as indices.", fr = "Texte du label pour chaque indice, même longueur que indices."),
        param(name = "color", ty = "str", en = "Line and label color. Default: #f59e0b.", fr = "Couleur de la ligne et du label. Défaut : #f59e0b.")
    )]
    #[sera_sig(indices, labels, color = "#f59e0b")]
    pub fn event_markers(&self, indices: Vec<usize>, labels: Vec<String>, color: &str) -> Chart {
        let idx_js = format!("[{}]", indices.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","));
        let labels_js = format!(
            "[{}]",
            labels.iter().map(|s| json_str(s)).collect::<Vec<_>>().join(",")
        );
        let js = format!(
            "(function(){{var svg=document.querySelector('svg');if(!svg)return;var idxs={};var labels={};var color={};var d=svg.getAttribute('data-sp')||'';var sp=d.split(',').map(Number);var pT=sp[1]||36,pH=sp[3]||360;var ns='http://www.w3.org/2000/svg';var els=Array.prototype.slice.call(svg.querySelectorAll('[data-idx]'));idxs.forEach(function(idx,i){{var match=null;for(var k=0;k<els.length;k++){{if(parseInt(els[k].getAttribute('data-idx'))===idx){{match=els[k];break;}}}}if(!match)return;var x=match.hasAttribute('cx')?parseFloat(match.getAttribute('cx')):(parseFloat(match.getAttribute('x')||0)+parseFloat(match.getAttribute('width')||0)/2);var ln=document.createElementNS(ns,'line');ln.setAttribute('x1',x);ln.setAttribute('x2',x);ln.setAttribute('y1',pT);ln.setAttribute('y2',pT+pH);ln.setAttribute('stroke',color);ln.setAttribute('stroke-width','1.5');ln.setAttribute('stroke-dasharray','4,3');ln.setAttribute('opacity','0.85');svg.appendChild(ln);var t=document.createElementNS(ns,'text');t.setAttribute('x',x+4);t.setAttribute('y',pT+12);t.setAttribute('font-size','10.5');t.setAttribute('font-weight','700');t.setAttribute('fill',color);t.setAttribute('transform','rotate(-90 '+(x+4)+' '+(pT+12)+')');t.textContent=labels[i]||'';svg.appendChild(t);}});}})();",
            idx_js, labels_js, json_str(color)
        );
        self.propagate(self.html.replacen("</body>", &format!("<script>{}</script></body>", js), 1))
    }
}
