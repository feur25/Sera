use super::common::{open, prepare, Prepared};
use super::config::ParallelConfig;
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};

const SUB: usize = 10;

#[crate::chart_demo(
    "axes=[\"1880\",\"1890\",\"1900\",\"1910\",\"1920\",\"1930\",\"1940\",\"1950\",\"1960\",\"1970\",\"1980\",\"1990\"], series_names=[\"Art Nouveau\",\"Arts and Crafts\",\"Constructivism\",\"Art Deco\",\"Bauhaus\",\"Surrealism\",\"International Style\",\"Pop Art\",\"Psychedelic Art\"], series=[[1.0,1.0,45.0],[2.0,2.0,35.0],[3.0,3.0,70.0],[4.0,2.0,28.0],[5.0,3.0,58.0],[6.0,2.0,24.0],[7.0,4.0,74.0],[8.0,2.0,30.0],[9.0,2.0,42.0]], palette=[14037868,3120708,1087661,7358696,15764480,1667522,14692657,6727695,11419337], variant=\"lineage\", height=680"
)]

pub fn render(cfg: &ParallelConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let n = cfg.series_names.len().min(cfg.series_values.len());
    if n == 0 || p.n_axes < 2 {
        return String::new();
    }

    let mut own_axis = vec![0usize; n];
    let mut reach = vec![0usize; n];
    let mut depth = vec![0.0f64; n];
    for m in 0..n {
        let row = &cfg.series_values[m];
        own_axis[m] = row.first().copied().unwrap_or(0.0).round().clamp(0.0, (p.n_axes - 1) as f64) as usize;
        reach[m] = row.get(1).copied().unwrap_or(0.0).round().max(0.0) as usize;
        depth[m] = row.get(2).copied().unwrap_or(50.0).clamp(4.0, 96.0);
    }

    let n_fine = (p.n_axes - 1) * SUB + 1;
    let fine_x = |i: usize| -> f64 { p.pad_l as f64 + (i as f64 / (n_fine - 1) as f64) * p.plot_w as f64 };
    let axis_x = |ai: usize| -> f64 { fine_x(ai * SUB) };
    let top_y = p.pad_t as f64;
    let bottom_y = (p.pad_t + p.plot_h) as f64;
    let knot_y = |d: f64| -> f64 { top_y + (d / 100.0) * (bottom_y - top_y) };

    let mut owner: Vec<i32> = vec![-1; n_fine];
    for m in 0..n {
        let own_fine = own_axis[m] * SUB;
        let span = reach[m] * SUB;
        let start = own_fine.saturating_sub(span);
        for i in start..=own_fine {
            let take = match owner[i] {
                -1 => true,
                cur => {
                    let cur = cur as usize;
                    let cur_own_fine = own_axis[cur] * SUB;
                    (i as i64 - own_fine as i64).abs() < (i as i64 - cur_own_fine as i64).abs()
                }
            };
            if take {
                owner[i] = m as i32;
            }
        }
    }

    let mut b = open(cfg, &p);

    push_b(&mut b, b"<g stroke=\"#e8ecf3\" stroke-width=\"1\">");
    for i in 0..n_fine {
        if owner[i] >= 0 {
            continue;
        }
        let x = fine_x(i);
        push_b(&mut b, b"<line x1=\"");
        push_f2(&mut b, x);
        push_b(&mut b, b"\" y1=\"");
        push_f2(&mut b, top_y);
        push_b(&mut b, b"\" x2=\"");
        push_f2(&mut b, x);
        push_b(&mut b, b"\" y2=\"");
        push_f2(&mut b, bottom_y);
        push_b(&mut b, b"\"/>");
    }
    push_b(&mut b, b"</g>");

    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n);
    for m in 0..n {
        let col = palette_color(cfg.palette, m);
        let hx = hex6(col);
        let kx = axis_x(own_axis[m]);
        let ky = knot_y(depth[m]);

        push_b(&mut b, b"<g fill=\"none\" stroke=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-opacity=\"0.55\" stroke-width=\"1.1\">");
        for (i, &o) in owner.iter().enumerate() {
            if o != m as i32 {
                continue;
            }
            let x0 = fine_x(i);
            let c1y = top_y + (ky - top_y) * 0.62;
            let c2y = top_y + (ky - top_y) * 0.88;
            push_b(&mut b, b"<path d=\"M ");
            push_f2(&mut b, x0);
            push_b(&mut b, b" ");
            push_f2(&mut b, top_y);
            push_b(&mut b, b" C ");
            push_f2(&mut b, x0);
            push_b(&mut b, b" ");
            push_f2(&mut b, c1y);
            push_b(&mut b, b", ");
            push_f2(&mut b, kx);
            push_b(&mut b, b" ");
            push_f2(&mut b, c2y);
            push_b(&mut b, b", ");
            push_f2(&mut b, kx);
            push_b(&mut b, b" ");
            push_f2(&mut b, ky);
            push_b(&mut b, b"\"/>");
        }
        push_b(&mut b, b"</g>");

        push_b(&mut b, b"<line data-idx=\"");
        push_i(&mut b, m as i32);
        push_b(&mut b, b"\" x1=\"");
        push_f2(&mut b, kx);
        push_b(&mut b, b"\" y1=\"");
        push_f2(&mut b, ky);
        push_b(&mut b, b"\" x2=\"");
        push_f2(&mut b, kx);
        push_b(&mut b, b"\" y2=\"");
        push_f2(&mut b, bottom_y);
        push_b(&mut b, b"\" stroke=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"2.4\" stroke-opacity=\"0.95\"/>");

        slots.push(
            HoverSlot::new(cfg.series_names[m].clone())
                .kv("Peak", cfg.axes.get(own_axis[m]).cloned().unwrap_or_default())
                .kv("Reach", format!("{} decades", reach[m]))
                .kv("Depth", format!("{:.0}%", depth[m])),
        );
    }

    timeline_dots(&mut b, cfg, &p, top_y - 16.0, n_fine, &fine_x);
    timeline_dots(&mut b, cfg, &p, bottom_y + 16.0, n_fine, &fine_x);

    for m in 0..n {
        let col = palette_color(cfg.palette, m);
        let hx = hex6(col);
        let x = axis_x(own_axis[m]);
        let ly = (bottom_y + 20.0) as i32;
        push_b(&mut b, b"<circle cx=\"");
        push_f2(&mut b, x);
        push_b(&mut b, b"\" cy=\"");
        push_i(&mut b, ly);
        push_b(&mut b, b"\" r=\"3.5\" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\"/>");
        push_b(&mut b, b"<text x=\"");
        push_f2(&mut b, x + 6.0);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, ly + 3);
        push_b(&mut b, b"\" transform=\"rotate(-32 ");
        push_f2(&mut b, x + 6.0);
        push_b(&mut b, b" ");
        push_i(&mut b, ly + 3);
        push_b(&mut b, b")\" text-anchor=\"start\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9\" font-weight=\"600\" fill=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\">");
        escape_xml(&mut b, &cfg.series_names[m]);
        push_b(&mut b, b"</text>");
    }

    push_b(&mut b, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(b) };
    build_chart_html(cfg.title, &svg, &slots_to_json(&slots))
}

fn timeline_dots(
    buf: &mut Vec<u8>,
    cfg: &ParallelConfig,
    p: &Prepared,
    y: f64,
    n_fine: usize,
    fine_x: &dyn Fn(usize) -> f64,
) {
    for i in 0..n_fine {
        let x = fine_x(i);
        let r = if i % SUB == 0 { 3.0 } else { 1.4 };
        push_b(buf, b"<circle cx=\"");
        push_f2(buf, x);
        push_b(buf, b"\" cy=\"");
        push_f2(buf, y);
        push_b(buf, b"\" r=\"");
        push_f2(buf, r);
        push_b(buf, b"\" fill=\"#fff\" stroke=\"#334155\" stroke-width=\"1\"/>");
    }
    if y < p.pad_t as f64 {
        for ai in 0..p.n_axes {
            let x = fine_x(ai * SUB);
            push_b(buf, b"<text x=\"");
            push_f2(buf, x);
            push_b(buf, b"\" y=\"");
            push_f2(buf, y - 10.0);
            push_b(buf, b"\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"9\" fill=\"#64748b\">");
            escape_xml(buf, &cfg.axes[ai]);
            push_b(buf, b"</text>");
        }
    }
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
        let axes: Vec<String> = (0..12).map(|i| format!("{}", 1880 + i * 10)).collect();
        let names: Vec<String> =
            vec!["Art Nouveau".into(), "Constructivism".into(), "Bauhaus".into()];
        let values: Vec<Vec<f64>> = vec![vec![1.0, 1.0, 45.0], vec![3.0, 3.0, 70.0], vec![5.0, 3.0, 58.0]];
        (axes, names, values)
    }

    #[test]
    fn renders_a_stem_and_a_labeled_dot_per_movement() {
        let (axes, names, values) = demo();
        let html = render(&cfg(&axes, &names, &values));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<line data-idx=").count(), 3);
        assert!(html.contains(">Art Nouveau<"));
        assert!(html.contains(">Constructivism<"));
        assert!(html.contains(">Bauhaus<"));
        assert!(html.contains("class=\"sp-bg\""));
    }

    #[test]
    fn unclaimed_years_stay_as_plain_straight_gridlines() {
        let axes: Vec<String> = (0..12).map(|i| format!("{}", 1880 + i * 10)).collect();
        let names: Vec<String> = vec!["Bauhaus".into()];
        let values: Vec<Vec<f64>> = vec![vec![5.0, 1.0, 50.0]];
        let html = render(&cfg(&axes, &names, &values));
        assert!(html.contains("stroke=\"#e8ecf3\""));
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let axes: Vec<String> = vec![];
        let names: Vec<String> = vec![];
        let values: Vec<Vec<f64>> = vec![];
        assert!(render(&cfg(&axes, &names, &values)).is_empty());
    }

    #[test]
    fn perf_rendering_stays_fast() {
        let axes: Vec<String> = (0..12).map(|i| format!("{}", 1880 + i * 10)).collect();
        let names: Vec<String> = (0..9).map(|i| format!("Movement {i}")).collect();
        let values: Vec<Vec<f64>> = (0..9).map(|i| vec![i as f64, 3.0, 30.0 + i as f64 * 6.0]).collect();
        let start = std::time::Instant::now();
        let html = render(&cfg(&axes, &names, &values));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
