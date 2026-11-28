use super::common::{open, prepare};
use super::config::ParallelConfig;
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};

#[crate::chart_demo(
    "axes=[\"2000\",\"2001\",\"2002\",\"2003\",\"2004\",\"2005\",\"2006\",\"2007\",\"2008\",\"2009\"], series_names=[\"Paradigm\",\"Society\",\"Finance\",\"Politics\",\"Ecology\",\"Technology\",\"Media\",\"Web\",\"Industrial Design\",\"Architecture\",\"Art\",\"Fashion\",\"Personae\",\"Brands\",\"Tragedy\"], series=[[2,0,0,0,2,0,0,0,0,0],[0,4,0,0,0,0,0,0,1,1],[0,0,0,1,0,0,0,0,0,2],[2,0,0,4,3,0,3,2,0,0],[0,4,0,0,0,1,0,0,3,0],[4,0,0,0,1,4,0,0,0,1],[0,0,0,1,0,1,0,0,2,3],[0,4,0,3,0,0,0,0,0,0],[0,3,1,0,0,2,0,0,2,2],[0,0,1,2,1,3,4,0,0,0],[0,4,0,4,0,4,0,0,0,2],[0,0,4,4,3,2,0,0,0,0],[0,1,0,0,0,0,0,1,2,0],[0,4,0,0,0,0,4,0,0,3],[0,3,0,0,0,0,0,0,3,0]], palette=[14037868,3120708,1087661,7358696,15764480,1667522,14692657,6727695,11419337,15227148,829048,15764480,3559367,12723548,4804695], variant=\"chronicle\", height=640"
)]

pub fn render(cfg: &ParallelConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let n_cat = cfg.series_names.len().min(cfg.series_values.len());
    let n_years = cfg.axes.len();
    if n_cat == 0 || n_years == 0 {
        return String::new();
    }

    let top_y = (p.pad_t + 30) as f64;
    let bottom_y = (p.pad_t + p.plot_h) as f64;
    let cat_x = |ci: usize| -> f64 { p.pad_l as f64 + (ci as f64 + 0.5) / n_cat as f64 * p.plot_w as f64 };
    let col_w = p.plot_w as f64 / n_years as f64;
    let col_left = |yi: usize| -> f64 { p.pad_l as f64 + yi as f64 * col_w };

    let mut year_total = vec![0usize; n_years];
    for yi in 0..n_years {
        for ci in 0..n_cat {
            year_total[yi] += cfg.series_values[ci].get(yi).copied().unwrap_or(0.0).max(0.0) as usize;
        }
    }

    let mut b = open(cfg, &p);

    push_b(&mut b, b"<g stroke=\"#eef1f6\" stroke-width=\"1\">");
    for yi in 1..n_years {
        let x = col_left(yi);
        push_b(&mut b, b"<line x1=\"");
        push_f2(&mut b, x);
        push_b(&mut b, b"\" y1=\"");
        push_f2(&mut b, top_y - 10.0);
        push_b(&mut b, b"\" x2=\"");
        push_f2(&mut b, x);
        push_b(&mut b, b"\" y2=\"");
        push_f2(&mut b, bottom_y + 24.0);
        push_b(&mut b, b"\"/>");
    }
    push_b(&mut b, b"</g>");

    let mut slots: Vec<HoverSlot> = Vec::new();
    let mut idx = 0i32;
    for yi in 0..n_years {
        let total = year_total[yi].max(1);
        let left = col_left(yi) + col_w * 0.08;
        let width = col_w * 0.84;
        let mut placed = 0usize;
        for ci in 0..n_cat {
            let count = cfg.series_values[ci].get(yi).copied().unwrap_or(0.0).max(0.0) as usize;
            if count == 0 {
                continue;
            }
            let col = palette_color(cfg.palette, ci);
            let hx = hex6(col);
            let sx = cat_x(ci);
            for _ in 0..count {
                let leaf_x = left + (placed as f64 + 0.5) / total as f64 * width;
                let c1y = top_y + (bottom_y - top_y) * 0.42;
                let c2y = top_y + (bottom_y - top_y) * 0.82;
                push_b(&mut b, b"<path data-idx=\"");
                push_i(&mut b, idx);
                push_b(&mut b, b"\" fill=\"none\" stroke=\"#");
                b.extend_from_slice(&hx);
                push_b(&mut b, b"\" stroke-opacity=\"0.4\" stroke-width=\"1\" d=\"M ");
                push_f2(&mut b, sx);
                push_b(&mut b, b" ");
                push_f2(&mut b, top_y);
                push_b(&mut b, b" C ");
                push_f2(&mut b, sx);
                push_b(&mut b, b" ");
                push_f2(&mut b, c1y);
                push_b(&mut b, b", ");
                push_f2(&mut b, leaf_x);
                push_b(&mut b, b" ");
                push_f2(&mut b, c2y);
                push_b(&mut b, b", ");
                push_f2(&mut b, leaf_x);
                push_b(&mut b, b" ");
                push_f2(&mut b, bottom_y);
                push_b(&mut b, b"\"/>");

                push_b(&mut b, b"<circle cx=\"");
                push_f2(&mut b, leaf_x);
                push_b(&mut b, b"\" cy=\"");
                push_f2(&mut b, bottom_y);
                push_b(&mut b, b"\" r=\"2.6\" fill=\"#");
                b.extend_from_slice(&hx);
                push_b(&mut b, b"\"/>");

                slots.push(
                    HoverSlot::new(cfg.series_names[ci].clone())
                        .kv("Year", cfg.axes[yi].clone())
                        .kv("Category", cfg.series_names[ci].clone()),
                );
                placed += 1;
                idx += 1;
            }
        }
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, col_left(yi) + col_w / 2.0);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, bottom_y + 18.0);
        push_b(&mut b, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"10\" font-weight=\"600\" fill=\"#334155\">");
        escape_xml(&mut b, &cfg.axes[yi]);
        push_b(&mut b, b"</text>");
    }

    for ci in 0..n_cat {
        let col = palette_color(cfg.palette, ci);
        let hx = hex6(col);
        let x = cat_x(ci);
        push_b(&mut b, b"<circle cx=\"");
        push_f2(&mut b, x);
        push_b(&mut b, b"\" cy=\"");
        push_f2(&mut b, top_y);
        push_b(&mut b, b"\" r=\"3\" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\"/>");
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, x + 5.0);
        push_b(&mut b, b"\" y=\"");
        push_f2(&mut b, top_y - 8.0);
        push_b(&mut b, b"\" transform=\"rotate(-38 ");
        push_f2(&mut b, x + 5.0);
        push_b(&mut b, b" ");
        push_f2(&mut b, top_y - 8.0);
        push_b(&mut b, b")\" text-anchor=\"start\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9.5\" font-weight=\"600\" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\">");
        escape_xml(&mut b, &cfg.series_names[ci]);
        push_b(&mut b, b"</text>");
    }

    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    build_chart_html(cfg.title, &svg, &slots_to_json(&slots))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg<'a>(axes: &'a [String], names: &'a [String], values: &'a [Vec<f64>]) -> ParallelConfig<'a> {
        ParallelConfig {
            title: "Test",
            axes,
            series_names: names,
            series_values: values,
            width: 1000,
            height: 500,
            ..ParallelConfig::default()
        }
    }

    fn demo() -> (Vec<String>, Vec<String>, Vec<Vec<f64>>) {
        let axes: Vec<String> = vec!["2000".into(), "2001".into(), "2002".into()];
        let names: Vec<String> = vec!["Web".into(), "Art".into(), "Brands".into()];
        let values: Vec<Vec<f64>> = vec![vec![2.0, 0.0, 1.0], vec![0.0, 3.0, 0.0], vec![1.0, 0.0, 2.0]];
        (axes, names, values)
    }

    #[test]
    fn renders_one_curve_and_one_dot_per_event() {
        let (axes, names, values) = demo();
        let html = render(&cfg(&axes, &names, &values));
        assert!(!html.is_empty());
        let total: usize = values.iter().map(|row| row.iter().sum::<f64>() as usize).sum();
        assert_eq!(html.matches("<path data-idx=").count(), total);
        assert!(html.contains("class=\"sp-bg\""));
    }

    #[test]
    fn category_labels_and_year_labels_are_both_present() {
        let (axes, names, values) = demo();
        let html = render(&cfg(&axes, &names, &values));
        assert!(html.contains(">Web<"));
        assert!(html.contains(">Art<"));
        assert!(html.contains(">2000<"));
        assert!(html.contains(">2002<"));
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let axes: Vec<String> = vec![];
        let names: Vec<String> = vec![];
        let values: Vec<Vec<f64>> = vec![];
        assert!(render(&cfg(&axes, &names, &values)).is_empty());
    }

    #[test]
    fn zero_only_data_still_renders_the_frame_without_events() {
        let axes: Vec<String> = vec!["2000".into(), "2001".into()];
        let names: Vec<String> = vec!["Web".into()];
        let values: Vec<Vec<f64>> = vec![vec![0.0, 0.0]];
        let html = render(&cfg(&axes, &names, &values));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<path data-idx=").count(), 0);
        assert!(html.contains(">Web<"));
    }

    #[test]
    fn perf_rendering_a_dense_chronicle_stays_fast() {
        let axes: Vec<String> = (0..10).map(|i| format!("{}", 2000 + i)).collect();
        let names: Vec<String> = (0..15).map(|i| format!("Category {i}")).collect();
        let values: Vec<Vec<f64>> = (0..15).map(|i| (0..10).map(|y| ((i + y) % 5) as f64).collect()).collect();
        let start = std::time::Instant::now();
        let html = render(&cfg(&axes, &names, &values));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
