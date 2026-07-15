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

}
