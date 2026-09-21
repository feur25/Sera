use crate::html::js_3d::render_3d_html;
use crate::plot::statistical::_3d::budget::{even_indices, pick, Budget};
use crate::plot::{apply_bg3d, parse_all};

pub fn render_dumbbell3d_html(
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
        11,
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

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\"], start=[10,20,15], end=[30,25,40]")]
#[crate::params(paramsList["title","labels","start","end","y_label","bg_color","scene","orientation3d","max_points","width","height"])]
#[crate::sera_alias("dumbbell3d", "dumbbell_3d", "dumbbell3d_chart")]
#[crate::sera_builder]
pub fn build_dumbbell3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values_start = a.start.unwrap_or_default();
    let values_end = a.end.unwrap_or_default();
    let s_name = o
        .series_name_start
        .as_deref()
        .unwrap_or("Start")
        .to_string();
    let e_name = o.series_name_end.as_deref().unwrap_or("End").to_string();
    let yl = o.yl();
    let y_lbl = if yl.is_empty() { "Item" } else { &yl };
    let n = labels.len().min(values_start.len()).min(values_end.len());
    let keep = even_indices(n, Budget::new(o.max_points).elements());
    let labels = pick(&labels, &keep);
    let xv = pick(&values_start, &keep);
    let zv = pick(&values_end, &keep);
    let yv: Vec<f64> = (0..keep.len()).map(|i| i as f64).collect();
    let cv = yv.clone();
    let bg_str = o.bg_str();
    apply_bg3d(
        crate::plot::statistical::_3d::render_dumbbell3d_html(
            title,
            &xv,
            &yv,
            &zv,
            (&s_name, y_lbl, &e_name),
            &cv,
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
        id: 78,
        name: "dumbbell_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_dumbbell3d_chart;
    use crate::plot::statistical::_3d::budget::Budget;
    use crate::plot::statistical::_3d::twin;

    fn columns(n: usize) -> serde_json::Value {
        serde_json::json!({"title": "t", "labels": twin::names(n), "start": twin::wave(n, 0), "end": twin::wave(n, 1)})
    }

    #[test]
    fn a_big_dumbbell_chart_is_capped_by_the_budget_and_small_ones_are_untouched() {
        twin::check_capped(build_dumbbell3d_chart, columns, Budget::elements);
    }

    #[test]
    fn labels_follow_the_kept_rows() {
        let html = build_dumbbell3d_chart(&columns(100_000).to_string());
        let labels = html.split("var CL=[").nth(1).and_then(|s| s.split("];").next()).map(|s| s.split("','").count());
        assert_eq!(labels, Some(Budget::default().elements()));
    }
}
