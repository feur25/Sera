use crate::html::js_3d::render_3d_html;
use crate::plot::{apply_bg3d, parse_all};

pub fn render_kde3d_html(
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
        5,
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

#[crate::chart_demo("categories=["A","B"], values=[[1,2,3],[4,5,6]]")]
#[crate::params(paramsList["title","categories","values","x_label","y_label","z_label","bg_color","scene","orientation3d","width","height"])]
#[crate::sera_alias("kde3d", "kde_3d", "kde3d_chart", "density3d")]
#[crate::sera_builder]
pub fn build_kde3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::kde::{kde_eval, scott_bw};
    let cats = a.categories.unwrap_or_default();
    let series: Vec<(String, Vec<f64>)> = if cats.is_empty() {
        vec![("Series".to_string(), values)]
    } else {
        let mut group_order: Vec<String> = Vec::new();
        let mut group_vals: std::collections::HashMap<String, Vec<f64>> =
            std::collections::HashMap::new();
        for (v, c) in values.iter().zip(cats.iter()) {
            group_vals.entry(c.clone()).or_default().push(*v);
            if !group_order.contains(c) {
                group_order.push(c.clone());
            }
        }
        group_order
            .into_iter()
            .map(|k| {
                let v = group_vals.remove(&k).unwrap_or_default();
                (k, v)
            })
            .collect()
    };
    let all_vals: Vec<f64> = series.iter().flat_map(|(_, v)| v.iter().copied()).collect();
    if all_vals.is_empty() {
        return String::new();
    }
    let xmin = all_vals.iter().cloned().fold(f64::INFINITY, f64::min);
    let xmax = all_vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let pad = (xmax - xmin).max(1.0) * 0.12;
    let lo = xmin - pad;
    let hi = xmax + pad;
    let n_pts: usize = 120;
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    let mut cv = Vec::new();
    let names: Vec<String> = series.iter().map(|(n, _)| n.clone()).collect();
    for (si, (_, vals)) in series.iter().enumerate() {
        let bw = scott_bw(vals);
        for k in 0..n_pts {
            let t = lo + (hi - lo) * k as f64 / (n_pts - 1).max(1) as f64;
            let d = kde_eval(vals, t, bw);
            xv.push(t);
            yv.push(si as f64);
            zv.push(d);
            cv.push(si as f64);
        }
    }
    let bg_str = o.bg_str();
    apply_bg3d(
        crate::plot::statistical::_3d::render_kde3d_html(
            title,
            &xv,
            &yv,
            &zv,
            (&o.xl(), &o.yl(), &o.zl()),
            &cv,
            &names,
            o.w(900),
            o.h(560),
            bg_str.as_deref(),
            &o.scene3d(),
        ),
        &o,
    )
}
