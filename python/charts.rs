use pyo3::prelude::*;
use pyo3::types::PyAny;

use crate::bindings::registry_macro::{for_each_auto_util_fn, for_each_json_chart_fn};

use super::{extract_f64_vec, merge_global_opts, python_chart_from_payload, python_chart_refs_to_html};

macro_rules! impl_python_json_chart_binding {
    ($fn:ident, $_js:literal) => {
        #[crate::sera_binding(python, py_json, chart)]
        fn $fn(input: &str) -> String {
            crate::bindings::commands::charts::$fn(input)
        }
    };
}
for_each_json_chart_fn!(impl_python_json_chart_binding);

macro_rules! impl_python_json_util {
    ($fn:ident, $_js:literal) => {
        #[pyfunction]
        pub fn $fn(input: &str) -> String {
            crate::bindings::commands::charts::$fn(input)
        }
    };
}
for_each_auto_util_fn!(impl_python_json_util);

#[crate::sera_sig(html, color=None)]
#[crate::sera_binding(python, py_chart, chart)]
fn set_bg(html: &str, color: Option<&str>) -> String {
    crate::html::hover::apply_bg(html.to_string(), color)
}

#[crate::sera_binding(python, py_chart)]
fn show_chart_value(chart_json: &str) -> bool {
    crate::plot::show_chart_value(chart_json)
}

#[crate::sera_binding(python, py_chart)]
fn bench_chart_value(chart_json: &str) -> bool {
    crate::plot::bench_chart_value(chart_json)
}

#[crate::sera_binding(python, py_true)]
fn set_chart_kind(kind: u8) {
    crate::plot::set_chart_kind(kind)
}

#[crate::sera_binding(python, py_true)]
fn set_chart_orientation(vertical: bool) {
    crate::plot::set_chart_orientation(vertical)
}

#[crate::sera_sig(n=2000)]
#[crate::sera_binding(python)]
fn bench_pure_rust(n: usize) -> (f64, f64, f64, f64) {
    crate::plot::bench_pure_rust(n)
}

#[crate::sera_doc(category = "layout", file = "charts/layout.md", en = "Arranges multiple Chart objects in a responsive grid layout and returns a single Chart.", fr = "Organise plusieurs objets Chart dans une mise en page grille responsive et retourne un seul Chart.", param(name = "charts", ty = "list[Chart]", en = "List of Chart objects to arrange in the grid.", fr = "Liste d'objets Chart a organiser dans la grille."), param(name = "cols", ty = "int", en = "Number of columns in the grid. Default: 3.", fr = "Nombre de colonnes dans la grille. Defaut: 3."), param(name = "gap", ty = "int", en = "Gap between cells in pixels. Default: 16.", fr = "Espacement entre les cellules en pixels. Defaut: 16."), param(name = "bg_color", ty = "str", en = "Background color for the grid container. Default: '#0a0f1c'.", fr = "Couleur d'arriere-plan du conteneur de grille. Defaut: '#0a0f1c'."), param(name = "title", ty = "str", en = "Optional title displayed above the grid.", fr = "Titre optionnel affiche au-dessus de la grille."), param(name = "cell_height", ty = "int | None", en = "Fixed height for each cell in pixels. Auto-detected if None.", fr = "Hauteur fixe de chaque cellule en pixels. Detection automatique si None."))]
#[pyfunction]
#[pyo3(signature = (charts, cols=3, gap=16, bg_color="#0a0f1c", title="", cell_height=None))]
pub fn build_grid(
    py: Python<'_>,
    charts: Vec<PyObject>,
    cols: usize,
    gap: i32,
    bg_color: &str,
    title: &str,
    cell_height: Option<i32>,
) -> PyResult<crate::Chart> {
    let html_parts: Vec<String> = charts
        .iter()
        .map(|object| object.extract::<PyRef<crate::Chart>>(py).map(|chart| chart.html.clone()))
        .collect::<PyResult<_>>()?;
    Ok(crate::Chart::new(crate::plot::layout::build_grid_impl(
        html_parts,
        cols,
        gap,
        bg_color,
        title,
        cell_height,
    )))
}

#[crate::sera_doc(category = "layout", file = "charts/layout.md", en = "Alias for build_grid(). Arranges multiple Chart objects in a responsive grid.", fr = "Alias de build_grid(). Organise plusieurs objets Chart dans une grille responsive.", param(name = "charts", ty = "list[Chart]", en = "List of Chart objects to arrange.", fr = "Liste d'objets Chart a organiser."), param(name = "cols", ty = "int", en = "Number of grid columns. Default: 3.", fr = "Nombre de colonnes de la grille. Defaut: 3."), param(name = "gap", ty = "int", en = "Gap in pixels. Default: 16.", fr = "Espacement en pixels. Defaut: 16."), param(name = "bg_color", ty = "str", en = "Grid background color. Default: '#0a0f1c'.", fr = "Couleur d'arriere-plan de la grille. Defaut: '#0a0f1c'."), param(name = "title", ty = "str", en = "Optional grid title.", fr = "Titre optionnel de la grille."), param(name = "cell_height", ty = "int | None", en = "Fixed cell height in pixels or None for auto.", fr = "Hauteur fixe des cellules en pixels ou None pour automatique."))]
#[pyfunction]
#[pyo3(signature = (charts, cols=3, gap=16, bg_color="#0a0f1c", title="", cell_height=None))]
pub fn grid(
    py: Python<'_>,
    charts: Vec<PyObject>,
    cols: usize,
    gap: i32,
    bg_color: &str,
    title: &str,
    cell_height: Option<i32>,
) -> PyResult<crate::Chart> {
    build_grid(py, charts, cols, gap, bg_color, title, cell_height)
}

#[pyfunction]
#[pyo3(signature = (charts, interval_ms=2500, title="", width=900, height=520))]
pub fn build_slideshow(
    charts: Vec<pyo3::PyRef<crate::Chart>>,
    interval_ms: u32,
    title: &str,
    width: i32,
    height: i32,
) -> crate::Chart {
    let html_strings = python_chart_refs_to_html(&charts);
    python_chart_from_payload(
        crate::plot::build_slideshow,
        serde_json::json!({"charts": html_strings, "interval_ms": interval_ms, "title": title, "width": width, "height": height}).to_string(),
        crate::bindings::commands::docs::DOC_BUILD_SLIDESHOW,
    )
}

#[crate::sera_doc(category = "layout", file = "charts/layout.md", en = "Builds a live system monitor dashboard showing CPU, memory, disk usage, and system info.", fr = "Construit un tableau de bord de surveillance systeme en direct montrant l'utilisation du CPU, de la memoire, des disques et les informations systeme.", param(name = "bg_color", ty = "str", en = "Dashboard background color. Default: '#0a0f1c'.", fr = "Couleur d'arriere-plan du tableau de bord. Defaut: '#0a0f1c'."), param(name = "update_interval_ms", ty = "int", en = "Simulated refresh interval in milliseconds. Default: 2000.", fr = "Intervalle de rafraichissement simule en millisecondes. Defaut: 2000."))]
#[pyfunction]
#[pyo3(signature = (bg_color="#0a0f1c", update_interval_ms=2000u32))]
pub fn build_sysmon(bg_color: &str, update_interval_ms: u32) -> crate::Chart {
    crate::Chart::new(crate::plot::layout::build_sysmon_html(bg_color, update_interval_ms))
}

#[crate::sera_doc(category = "layout", file = "charts/layout.md", en = "Alias for build_sysmon(). Returns a live system monitor dashboard as a Chart.", fr = "Alias de build_sysmon(). Retourne un tableau de bord de surveillance systeme en direct sous forme de Chart.", param(name = "bg_color", ty = "str", en = "Background color. Default: '#0a0f1c'.", fr = "Couleur d'arriere-plan. Defaut: '#0a0f1c'."), param(name = "update_interval_ms", ty = "int", en = "Refresh interval in milliseconds. Default: 2000.", fr = "Intervalle de rafraichissement en millisecondes. Defaut: 2000."))]
#[pyfunction]
#[pyo3(signature = (bg_color="#0a0f1c", update_interval_ms=2000u32))]
pub fn sysmon(bg_color: &str, update_interval_ms: u32) -> crate::Chart {
    build_sysmon(bg_color, update_interval_ms)
}

#[pyfunction]
#[pyo3(signature = (labels, images=None, descriptions=None))]
pub fn build_hover_json(
    labels: Vec<String>,
    images: Option<Vec<Option<String>>>,
    descriptions: Option<Vec<Vec<(String, String)>>>,
) -> String {
    let descriptions_json: Option<Vec<Vec<Vec<String>>>> = descriptions.map(|outer| {
        outer
            .into_iter()
            .map(|row| row.into_iter().map(|(key, value)| vec![key, value]).collect())
            .collect()
    });
    crate::plot::build_hover_json(
        &serde_json::json!({"labels": labels, "images": images, "descriptions": descriptions_json}).to_string(),
    )
}

#[pyfunction]
#[pyo3(name = "plot", signature = (x, y=None, *, title="", kind="line", color_hex=0x6366F1_u32, width=900_i32, height=480_i32, x_label="", y_label="", gridlines=false, palette=None, background=None, show_points=true))]
pub fn plot_chart(
    py: Python<'_>,
    x: &Bound<'_, PyAny>,
    y: Option<&Bound<'_, PyAny>>,
    title: &str,
    kind: &str,
    color_hex: u32,
    width: i32,
    height: i32,
    x_label: &str,
    y_label: &str,
    gridlines: bool,
    palette: Option<Vec<u32>>,
    background: Option<&str>,
    show_points: bool,
) -> PyResult<crate::Chart> {
    let started_at = std::time::Instant::now();
    let (background, palette, gridlines) = merge_global_opts(background, palette, gridlines);
    let palette = if palette.is_empty() { None } else { Some(palette) };

    if let Ok(columns) = x.getattr("columns") {
        let columns: Vec<String> = columns.extract()?;
        if columns.len() >= 2 {
            let x_column = x.get_item(columns[0].as_str())?;
            let y_column = x.get_item(columns[1].as_str())?;
            let x_values = extract_f64_vec(py, &x_column)?;
            let y_values = extract_f64_vec(py, &y_column)?;
            let title = if title.is_empty() {
                format!("{} vs {}", columns[0], columns[1])
            } else {
                title.to_string()
            };
            let result = Ok(python_chart_from_payload(
                crate::plot::plot_chart,
                serde_json::json!({"x": x_values, "y": y_values, "kind": "scatter", "title": title, "color_hex": color_hex, "width": width, "height": height, "x_label": x_label, "y_label": y_label, "gridlines": gridlines, "palette": palette, "background": background}).to_string(),
                "",
            ));
            crate::telemetry::record(crate::telemetry::TelemetryEvent::new(
                "plot",
                started_at.elapsed().as_secs_f64() * 1000.0,
            ));
            return result;
        }
    }

    let x_values = extract_f64_vec(py, x)?;
    let y_values = if let Some(y_values) = y {
        Some(extract_f64_vec(py, y_values)?)
    } else {
        None
    };

    let result = Ok(python_chart_from_payload(
        crate::plot::plot_chart,
        serde_json::json!({"x": x_values, "y": y_values, "kind": kind, "title": title, "color_hex": color_hex, "show_points": show_points, "width": width, "height": height, "x_label": x_label, "y_label": y_label, "gridlines": gridlines, "palette": palette, "background": background}).to_string(),
        "",
    ));
    crate::telemetry::record(
        crate::telemetry::TelemetryEvent::new("plot", started_at.elapsed().as_secs_f64() * 1000.0)
            .with_data(x_values.len() as u64, 0.0),
    );
    result
}