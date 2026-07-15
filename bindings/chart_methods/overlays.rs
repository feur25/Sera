use seraplot_macros::{sera_impl, sera_doc, sera_sig, sera_python_skip};
#[allow(unused_imports)]
use crate::{Chart, json_str};
#[allow(unused_imports)]
use super::apply::*;
#[allow(unused_imports)]
use super::js::*;
#[sera_impl]
impl Chart {
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
            en = "Column labels to include in the cluster frame (must match the heatmap col_labels).",
            fr = "Étiquettes de colonnes à inclure dans le cadrage (doivent correspondre aux col_labels de la heatmap)."
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
        aliases("y2", "right_axis", "dual_axis", "twin_axis", "second_axis"),
        file = "charts/chart.md",
        en = "Overlays a second line series mapped to an independent right-side Y axis. Useful for comparing datasets with very different scales on the same chart.",
        fr = "Superpose une deuxième série de type ligne avec un axe Y droit indépendant. Utile pour comparer des jeux de données d'échelles très différentes sur le même graphique.",
        param(name = "values", ty = "list[float]", en = "Data values for the secondary series. Must match the primary series length.", fr = "Valeurs de la série secondaire. Doit correspondre à la longueur de la série principale."),
        param(name = "color", ty = "str | None", en = "Line and axis color. Default: #f59e0b.", fr = "Couleur de la ligne et de l'axe. Défaut : #f59e0b."),
        param(name = "label", ty = "str | None", en = "Optional right-axis label (rotated 90°).", fr = "Étiquette optionnelle pour l'axe droit (pivotée 90°).")
    )]
    #[sera_sig(values, color = None, label = None)]
    pub fn secondary_y(&self, values: Vec<f64>, color: Option<&str>, label: Option<&str>) -> Chart {
        let c = color.unwrap_or("#f59e0b");
        let vals_json = format!("[{}]", values.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(","));
        let label_json = match label {
            Some(l) => json_str(l),
            None => "null".into(),
        };
        let snippet = format!(
            "<script>window.__sp_sec_y__={{v:{},c:{},l:{}}};{}</script></body>",
            vals_json,
            json_str(c),
            label_json,
            SP_SECONDARY_Y_JS
        );
        self.propagate(self.html.replacen("</body>", &snippet, 1))
    }

}
