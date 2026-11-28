use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn render_line_wasm(
    title: &str,
    x: Vec<f64>,
    y: Vec<f64>,
    width: i32,
    height: i32,
    color_hex: u32,
    gridlines: bool,
    x_label: &str,
    y_label: &str,
) -> String {
    let opts = crate::plot::canvas_points::NativeChartOpts {
        width,
        height,
        color_hex,
        gridlines,
        x_label,
        y_label,
        show_regression: false,
        regression_type: "linear",
        cols: 0,
        categories: &[],
        variant: "",
    };
    crate::plot::canvas_points::render_line_family_native(title, &x, &y, &opts)
}

#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn render_scatter_wasm(
    title: &str,
    x: Vec<f64>,
    y: Vec<f64>,
    width: i32,
    height: i32,
    color_hex: u32,
    gridlines: bool,
    x_label: &str,
    y_label: &str,
) -> String {
    let opts = crate::plot::canvas_points::NativeChartOpts {
        width,
        height,
        color_hex,
        gridlines,
        x_label,
        y_label,
        show_regression: false,
        regression_type: "linear",
        cols: 0,
        categories: &[],
        variant: "",
    };
    crate::plot::default::scatter::render_scatter_family_native(title, &x, &y, &opts)
}
