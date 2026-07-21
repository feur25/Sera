use crate::html::js_3d::render_3d_html;
use crate::plot::{apply_bg3d, parse_all};

pub fn render_sunburst3d_html(
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
        13,
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

#[crate::chart_demo("labels=[\"Root\",\"A\",\"B\"], parents=[\"\",\"Root\",\"Root\"], values=[0,40,60]")]
#[crate::params(paramsList["title","labels","parents","values","bg_color","scene","orientation3d","width","height"])]
#[crate::sera_alias("sunburst3d", "sunburst_3d", "sunburst3d_chart")]
#[crate::sera_builder]
pub fn build_sunburst3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let parents = a.parents.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let n = labels.len().min(parents.len()).min(values.len());
    let mut ring_map: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    ring_map.insert(String::new(), 0);
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    let mut cv = Vec::new();
    let mut cl = Vec::new();
    for i in 0..n {
        let parent_ring = ring_map.get(&parents[i]).copied().unwrap_or(0);
        let my_ring = parent_ring + 1;
        ring_map.insert(labels[i].clone(), my_ring);
        xv.push(i as f64);
        yv.push(my_ring as f64);
        zv.push(values[i]);
        cv.push(i as f64);
        cl.push(labels[i].clone());
    }
    let bg_str = o.bg_str();
    apply_bg3d(
        crate::plot::statistical::_3d::render_sunburst3d_html(
            title,
            &xv,
            &yv,
            &zv,
            ("", "Ring", "Value"),
            &cv,
            &cl,
            o.w(700),
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
        id: 80,
        name: "sunburst_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}
