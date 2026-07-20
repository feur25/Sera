#[cfg(feature = "python")]
use pyo3::prelude::*;

static LIVE_STREAM_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(name = "LiveStream", module = "seraplot")
)]
pub struct LiveStream {
    kind: String,
    title: String,
    xs: Vec<f64>,
    ys: Vec<f64>,
    max_points: usize,
    color_hex: u32,
    width: i32,
    height: i32,
    display_id: String,
    started: bool,
}

impl LiveStream {
    fn render_html(&self) -> String {
        let base = serde_json::json!({
            "title": self.title,
            "x": self.xs,
            "y": self.ys,
            "color_hex": self.color_hex,
            "width": self.width,
            "height": self.height,
            "show_points": true
        });
        match self.kind.as_str() {
            "scatter" => crate::plot::statistical::scatter::build(&base.to_string()),
            _ => crate::plot::default::build_line_chart(
                &serde_json::json!({
                    "title": self.title,
                    "labels": self.xs.iter().map(|v| v.to_string()).collect::<Vec<_>>(),
                    "values": self.ys,
                    "color_hex": self.color_hex,
                    "width": self.width,
                    "height": self.height,
                    "show_points": true
                })
                .to_string(),
            ),
        }
    }

    fn iframe_html(&self, html: &str) -> String {
        let esc = html.replace('&', "&amp;").replace('"', "&quot;");
        format!(
            r#"<iframe id="{}" srcdoc="{}" style="width:100%;max-width:{}px;aspect-ratio:{}/{};border:none;display:block;border-radius:8px;overflow:hidden" frameborder="0"></iframe>"#,
            self.display_id, esc, self.width, self.width, self.height
        )
    }

    fn cap(&mut self) {
        if self.xs.len() > self.max_points {
            let cut = self.xs.len() - self.max_points;
            self.xs.drain(..cut);
            self.ys.drain(..cut);
        }
    }

    #[cfg(feature = "python")]
    fn live_update(&mut self, py: Python<'_>) -> PyResult<()> {
        let wrapped = self.iframe_html(&self.render_html());
        let display_mod = py.import_bound("IPython.display")?;
        let html_obj = display_mod.getattr("HTML")?.call1((wrapped,))?;
        let kwargs = pyo3::types::PyDict::new_bound(py);
        kwargs.set_item("display_id", &self.display_id)?;
        if !self.started {
            display_mod
                .getattr("display")?
                .call((html_obj,), Some(&kwargs))?;
            self.started = true;
        } else {
            display_mod
                .getattr("update_display")?
                .call((html_obj,), Some(&kwargs))?;
        }
        Ok(())
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl LiveStream {
    #[new]
    #[pyo3(signature = (kind = "line", title = "", max_points = 500, color_hex = 0x636EFA, width = 900, height = 420))]
    fn new(kind: &str, title: &str, max_points: usize, color_hex: u32, width: i32, height: i32) -> Self {
        let n = LIVE_STREAM_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        LiveStream {
            kind: kind.to_string(),
            title: title.to_string(),
            xs: Vec::new(),
            ys: Vec::new(),
            max_points: max_points.max(2),
            color_hex,
            width,
            height,
            display_id: format!("sp-live-{}", n),
            started: false,
        }
    }

    fn push(&mut self, py: Python<'_>, x: f64, y: f64) -> PyResult<()> {
        self.xs.push(x);
        self.ys.push(y);
        self.cap();
        self.live_update(py)
    }

    #[pyo3(name = "append")]
    fn push_alias_append(&mut self, py: Python<'_>, x: f64, y: f64) -> PyResult<()> {
        self.push(py, x, y)
    }

    #[pyo3(name = "add_point")]
    fn push_alias_add_point(&mut self, py: Python<'_>, x: f64, y: f64) -> PyResult<()> {
        self.push(py, x, y)
    }

    fn extend(&mut self, py: Python<'_>, xs: Vec<f64>, ys: Vec<f64>) -> PyResult<()> {
        self.xs.extend(xs);
        self.ys.extend(ys);
        self.cap();
        self.live_update(py)
    }

    #[pyo3(name = "add_points")]
    fn extend_alias_add_points(&mut self, py: Python<'_>, xs: Vec<f64>, ys: Vec<f64>) -> PyResult<()> {
        self.extend(py, xs, ys)
    }

    #[pyo3(name = "append_many")]
    fn extend_alias_append_many(&mut self, py: Python<'_>, xs: Vec<f64>, ys: Vec<f64>) -> PyResult<()> {
        self.extend(py, xs, ys)
    }

    fn clear(&mut self, py: Python<'_>) -> PyResult<()> {
        self.xs.clear();
        self.ys.clear();
        self.live_update(py)
    }

    #[pyo3(name = "reset")]
    fn clear_alias_reset(&mut self, py: Python<'_>) -> PyResult<()> {
        self.clear(py)
    }

    fn render(&self) -> crate::Chart {
        crate::Chart {
            html: self.render_html(),
            doc_str: "",
        }
    }

    #[pyo3(name = "snapshot")]
    fn render_alias_snapshot(&self) -> crate::Chart {
        self.render()
    }

    #[pyo3(name = "to_chart")]
    fn render_alias_to_chart(&self) -> crate::Chart {
        self.render()
    }

    #[getter]
    fn n(&self) -> usize {
        self.xs.len()
    }

    #[getter]
    fn html(&self) -> String {
        self.render_html()
    }
}

inventory::submit! {
    crate::doc_registry::FnDoc {
        name: "LiveStream",
        category: "chart_method",
        file: "config/streaming.md",
        en: "A bounded ring-buffer accumulator that turns a series of (x, y) samples arriving over time into a continuously redrawn chart. push()/extend()/clear() repaint the same Jupyter output cell in place via IPython's display_id, so the chart updates smoothly with no flicker or new cells.",
        fr: "Un accumulateur à buffer circulaire borné qui transforme une série d'échantillons (x, y) arrivant dans le temps en un graphique redessiné en continu. push()/extend()/clear() repeignent la même cellule de sortie Jupyter sur place via le display_id d'IPython, donc le graphique se met à jour sans scintillement ni nouvelle cellule.",
        params: &[
            crate::doc_registry::ParamDoc { name: "kind", ty: "str", en: "Chart kind: 'line' or 'scatter'. Default: 'line'.", fr: "Type de graphique : 'line' ou 'scatter'. Défaut : 'line'." },
            crate::doc_registry::ParamDoc { name: "title", ty: "str", en: "Chart title.", fr: "Titre du graphique." },
            crate::doc_registry::ParamDoc { name: "max_points", ty: "int", en: "Ring buffer size; oldest samples are dropped past this. Default: 500.", fr: "Taille du buffer circulaire ; les échantillons les plus anciens sont supprimés au-delà. Défaut : 500." },
            crate::doc_registry::ParamDoc { name: "color_hex", ty: "int", en: "Series color as a 24-bit RGB integer.", fr: "Couleur de la série en entier RGB 24 bits." },
            crate::doc_registry::ParamDoc { name: "width", ty: "int", en: "Canvas width in pixels.", fr: "Largeur du canvas en pixels." },
            crate::doc_registry::ParamDoc { name: "height", ty: "int", en: "Canvas height in pixels.", fr: "Hauteur du canvas en pixels." },
        ],
        aliases: &[],
    }
}
inventory::submit! {
    crate::doc_registry::FnDoc {
        name: "LiveStream.push",
        category: "chart_method",
        file: "config/streaming.md",
        en: "Appends one (x, y) sample, enforces max_points, and redraws the live chart in place.",
        fr: "Ajoute un échantillon (x, y), applique max_points, et redessine le graphique live sur place.",
        params: &[
            crate::doc_registry::ParamDoc { name: "x", ty: "float", en: "X value.", fr: "Valeur X." },
            crate::doc_registry::ParamDoc { name: "y", ty: "float", en: "Y value.", fr: "Valeur Y." },
        ],
        aliases: &["append", "add_point"],
    }
}
inventory::submit! {
    crate::doc_registry::FnDoc {
        name: "LiveStream.extend",
        category: "chart_method",
        file: "config/streaming.md",
        en: "Appends two lists of samples in lock-step, enforces max_points, and redraws the live chart in place.",
        fr: "Ajoute deux listes d'échantillons en parallèle, applique max_points, et redessine le graphique live sur place.",
        params: &[
            crate::doc_registry::ParamDoc { name: "xs", ty: "list[float]", en: "X values.", fr: "Valeurs X." },
            crate::doc_registry::ParamDoc { name: "ys", ty: "list[float]", en: "Y values.", fr: "Valeurs Y." },
        ],
        aliases: &["add_points", "append_many"],
    }
}
inventory::submit! {
    crate::doc_registry::FnDoc {
        name: "LiveStream.clear",
        category: "chart_method",
        file: "config/streaming.md",
        en: "Empties the ring buffer and redraws the (now empty) live chart in place.",
        fr: "Vide le buffer circulaire et redessine le graphique live (désormais vide) sur place.",
        params: &[],
        aliases: &["reset"],
    }
}
inventory::submit! {
    crate::doc_registry::FnDoc {
        name: "LiveStream.render",
        category: "chart_method",
        file: "config/streaming.md",
        en: "Renders the current buffer to a standalone Chart object, without touching the live display.",
        fr: "Rend le buffer courant sous forme d'objet Chart autonome, sans toucher à l'affichage live.",
        params: &[],
        aliases: &["snapshot", "to_chart"],
    }
}

