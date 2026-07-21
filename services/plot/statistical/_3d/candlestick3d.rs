use crate::html::js_3d::render_3d_html;
use crate::plot::{apply_bg3d, parse_all};

pub fn render_candlestick3d_html(
    title: &str,
    x: &[f64],
    y: &[f64],
    z: &[f64],
    axis_labels: (&str, &str, &str),
    colors: &[f64],
    color_labels: &[String],
    w: i32,
    h: i32,
    bg_color: Option<&str>,
    scene: &str,
) -> String {
    render_3d_html(
        10,
        title,
        x,
        y,
        z,
        axis_labels,
        colors,
        color_labels,
        w,
        h,
        bg_color,
        scene,
    )
}

#[crate::chart_demo("labels=[\"D1\",\"D2\",\"D3\"], open=[10,12,11], high=[14,15,13], low=[9,10,9], close=[12,13,12]")]
#[crate::params(paramsList["title","labels","open","high","low","close","x_label","y_label","z_label","bg_color","scene","orientation3d","width","height"])]
#[crate::sera_alias("candlestick3d", "candlestick_3d", "candlestick3d_chart", "ohlc3d")]
#[crate::sera_builder]
pub fn build_candlestick3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let open = a.open.unwrap_or_default();
    let high = a.high.unwrap_or_default();
    let low = a.low.unwrap_or_default();
    let close = a.close.unwrap_or_default();
    let n = labels
        .len()
        .min(open.len())
        .min(high.len())
        .min(low.len())
        .min(close.len());
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    for i in 0..n {
        xv.extend([open[i], high[i], low[i], close[i]]);
        yv.extend([i as f64; 4]);
        zv.extend([0.0f64; 4]);
    }
    let xl = o.xl();
    let yl = o.yl();
    let zl = o.zl();
    let x_lbl = if xl.is_empty() { "Price" } else { &xl };
    let y_lbl = if yl.is_empty() { "Bar" } else { &yl };
    let bg_str = o.bg_str();
    apply_bg3d(
        crate::plot::statistical::_3d::render_candlestick3d_html(
            title,
            &xv,
            &yv,
            &zv,
            (x_lbl, y_lbl, &zl),
            &[],
            &labels,
            o.w(900),
            o.h(560),
            bg_str.as_deref(),
            &o.scene3d(),
        ),
        &o,
    )
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 77,
        name: "candlestick_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}
