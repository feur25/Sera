use crate::html::js_3d::render_3d_html;
use crate::plot::{apply_bg3d, parse_all};

pub fn render_heatmap3d_html(
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
        9,
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

#[crate::chart_demo("labels=[\"R1\",\"R2\"], categories=[\"C1\",\"C2\"], matrix=[[1,2],[3,4]]")]
#[crate::params(paramsList["title","labels","categories","matrix","x_labels","x_label","y_label","z_label","bg_color","scene","orientation3d","width","height"])]
#[crate::sera_alias("heatmap3d", "heatmap_3d", "heatmap3d_chart", "heatmaps3d")]
#[crate::sera_builder]
pub fn build_heatmap3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x_labels = a.x_labels.or_else(|| a.labels.clone()).unwrap_or_default();
    let y_labels = a.categories.or(a.labels).unwrap_or_default();
    let values = a.matrix.unwrap_or_default();
    let nr = y_labels.len().min(values.len());
    let nc = x_labels.len();
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    let mut cv = Vec::new();
    let mut cl = Vec::new();
    for r in 0..nr {
        let row = &values[r];
        for c2 in 0..nc.min(row.len()) {
            xv.push(c2 as f64);
            yv.push(r as f64);
            zv.push(row[c2]);
            cv.push(0.0);
            cl.push(format!("{}/{}", y_labels[r], x_labels[c2]));
        }
    }
    let bg_str = o.bg_str();
    apply_bg3d(
        crate::plot::statistical::_3d::render_heatmap3d_html(
            title,
            &xv,
            &yv,
            &zv,
            (&o.xl(), &o.yl(), &o.zl()),
            &cv,
            &cl,
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
        id: 76,
        name: "heatmap_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}
