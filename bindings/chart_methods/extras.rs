use super::js::{
    SP_BAR_SCALE_JS, SP_EXPORT_PNG_JS, SP_FRAMES_JS, SP_LASSO_JS, SP_RANGE_BUTTONS_JS,
    SP_RANGE_SLIDER_JS, SP_SECONDARY_Y_JS,
};
use crate::{json_str, Chart};
#[allow(unused_imports)]
use seraplot_macros::{sera_doc, sera_impl, sera_sig};

#[sera_impl]
impl Chart {
    #[sera_doc(
        category = "chart_method",
        aliases("export_png_button", "download_png", "png_download"),
        file = "charts/export.md",
        en = "Adds a PNG download button to the chart. Clicking it rasterizes the SVG to a canvas, then triggers a PNG download without a server.",
        fr = "Ajoute un bouton de téléchargement PNG au graphique. Au clic, le SVG est rasterise dans un canvas puis telecharge en PNG sans serveur."
    )]
    pub fn png_button(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</body>",
            &format!("<script>{}</script></body>", SP_EXPORT_PNG_JS),
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("box_select", "rect_select", "rubber_band"),
        file = "charts/chart.md",
        en = "Enables box selection on scatter charts. Shift+drag to select points inside a rectangle; unselected points are dimmed. Shift+click an empty area clears the selection.",
        fr = "Active la selection rectangulaire sur les nuages de points. Shift+drag selectionne les points dans un rectangle ; les autres points sont attenues. Shift+clic dans une zone vide efface la selection."
    )]
    pub fn lasso(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</body>",
            &format!("<script>{}</script></body>", SP_LASSO_JS),
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("time_slider", "slider", "filter_range", "date_range"),
        file = "charts/chart.md",
        en = "Appends a dual-handle range slider below the chart. Dragging the handles filters visible data elements by index.",
        fr = "Ajoute un curseur de plage a deux poignees sous le graphique. Les poignees filtrent les elements visibles par indice."
    )]
    pub fn range_slider(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</body>",
            &format!("<script>{}</script></body>", SP_RANGE_SLIDER_JS),
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("time_range_buttons", "date_buttons", "range_selector", "quick_range"),
        file = "charts/chart.md",
        en = "Appends 1M / 6M / YTD / 1Y / All preset range buttons below the chart. If data-point labels parse as dates (native Date.parse, no external dependency), buttons filter by real calendar range from the last point; otherwise they fall back to an equivalent proportional window over the index range.",
        fr = "Ajoute des boutons de plage predefinis 1M / 6M / YTD / 1A / Tout sous le graphique. Si les etiquettes des points se parsent comme des dates (Date.parse natif, sans dependance externe), les boutons filtrent par vraie plage calendaire depuis le dernier point ; sinon ils retombent sur une fenetre proportionnelle equivalente sur la plage d'index."
    )]
    pub fn range_buttons(&self) -> Chart {
        self.propagate(self.html.replacen(
            "</body>",
            &format!("<script>{}</script></body>", SP_RANGE_BUTTONS_JS),
            1,
        ))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("animate_frames", "play", "frame_animation", "slider_animation"),
        file = "charts/chart.md",
        en = "Adds data-frame animation controls with play, pause, step and timeline slider controls. Each frame is a list of values that replaces the current bar heights with a smooth transition.",
        fr = "Ajoute des controles d'animation par frames avec lecture, pause, pas et slider temporel. Chaque frame est une liste de valeurs qui remplace les hauteurs de barres avec une transition fluide.",
        param(
            name = "frames",
            ty = "list[list[float]]",
            en = "List of frames; each frame is a list of values with the same length as the chart data.",
            fr = "Liste de frames ; chaque frame contient des valeurs de meme longueur que les donnees du graphique."
        ),
        param(
            name = "speed_ms",
            ty = "int",
            en = "Duration between frames in milliseconds. Default: 600.",
            fr = "Duree entre frames en millisecondes. Defaut : 600."
        ),
        param(
            name = "labels",
            ty = "list[str] | None",
            en = "Optional label shown next to the slider for each frame.",
            fr = "Etiquette optionnelle affichee pres du slider pour chaque frame."
        )
    )]
    #[sera_sig(frames, speed_ms = 600, labels = None)]
    pub fn frames(
        &self,
        frames: Vec<Vec<f64>>,
        speed_ms: i64,
        labels: Option<Vec<String>>,
    ) -> Chart {
        let frames_json = {
            let inner: Vec<String> = frames
                .iter()
                .map(|fr| {
                    format!(
                        "[{}]",
                        fr.iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<_>>()
                            .join(",")
                    )
                })
                .collect();
            format!("[{}]", inner.join(","))
        };
        let labels_json = match &labels {
            Some(v) => format!(
                "[{}]",
                v.iter().map(|l| json_str(l)).collect::<Vec<_>>().join(",")
            ),
            None => "[]".into(),
        };
        let snippet = format!(
            "<script>window.__sp_frames__={{f:{},s:{},l:{}}};{}</script></body>",
            frames_json,
            speed_ms.max(50),
            labels_json,
            SP_FRAMES_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("y2", "right_axis", "dual_axis", "twin_axis", "second_axis"),
        file = "charts/chart.md",
        en = "Overlays a second line series mapped to an independent right-side Y axis. Useful for comparing series with different scales on the same chart.",
        fr = "Superpose une deuxieme serie en ligne sur un axe Y droit independant. Utile pour comparer des series avec des echelles differentes sur le meme graphique.",
        param(
            name = "values",
            ty = "list[float]",
            en = "Data values for the secondary series. Must match the primary series length.",
            fr = "Valeurs de la serie secondaire. Doit correspondre a la longueur de la serie principale."
        ),
        param(
            name = "color",
            ty = "str | None",
            en = "Line and axis color. Default: #f59e0b.",
            fr = "Couleur de la ligne et de l'axe. Defaut : #f59e0b."
        ),
        param(
            name = "label",
            ty = "str | None",
            en = "Optional right-axis label.",
            fr = "Etiquette optionnelle pour l'axe droit."
        )
    )]
    #[sera_sig(values, color = None, label = None)]
    pub fn secondary_y(&self, values: Vec<f64>, color: Option<&str>, label: Option<&str>) -> Chart {
        let vals_json = format!(
            "[{}]",
            values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        let label_json = match label {
            Some(l) => json_str(l),
            None => "null".into(),
        };
        let snippet = format!(
            "<script>window.__sp_sec_y__={{v:{},c:{},l:{}}};{}</script></body>",
            vals_json,
            json_str(color.unwrap_or("#f59e0b")),
            label_json,
            SP_SECONDARY_Y_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

    #[sera_doc(
        category = "chart_method",
        aliases("visual_scale", "bar_heights", "scale_bars"),
        file = "charts/chart.md",
        en = "Applies per-bar visual height scale factors without changing data values. A factor of 2.0 doubles the bar height; 0.5 halves it. Use dev() SCALE mode to find values interactively.",
        fr = "Applique des facteurs d'echelle visuels par barre sans modifier les valeurs de donnees. Un facteur de 2.0 double la hauteur; 0.5 la reduit de moitie. Utilisez le mode SCALE de dev() pour trouver les valeurs.",
        param(name = "factors", ty = "list[float]", en = "Scale factor per bar in data order. Length must match the number of bars.", fr = "Facteur d'echelle par barre dans l'ordre des donnees. La longueur doit correspondre au nombre de barres.")
    )]
    #[sera_sig(factors)]
    pub fn bar_scale(&self, factors: Vec<f64>) -> Chart {
        let json = format!(
            "[{}]",
            factors
                .iter()
                .map(|f| format!("{:.4}", f))
                .collect::<Vec<_>>()
                .join(",")
        );
        let snippet = format!(
            "<script>window.__sp_bar_scale__={};{}</script></body>",
            json, SP_BAR_SCALE_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }
}
