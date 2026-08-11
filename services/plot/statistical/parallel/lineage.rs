use super::common::{open, prepare, Prepared};
use super::config::ParallelConfig;
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};

const SUB: usize = 10;

#[crate::chart_demo(
    "axes=[\"1880\",\"1890\",\"1900\",\"1910\",\"1920\",\"1930\",\"1940\",\"1950\",\"1960\",\"1970\",\"1980\",\"1990\"], series_names=[\"Art Nouveau\",\"Arts and Crafts\",\"Constructivism\",\"Art Deco\",\"Bauhaus\",\"Surrealism\",\"International Style\",\"Pop Art\",\"Psychedelic Art\"], series=[[8.2,55.0,23.5,10.0,4.3,0.0,0.0,0.0,0.0,0.0,0.0,0.0],[6.3,24.1,42.0,18.0,7.7,3.3,0.0,0.0,0.0,0.0,0.0,0.0],[13.2,38.1,63.1,88.0,37.6,16.1,6.9,0.0,0.0,0.0,0.0,0.0],[0.0,0.0,5.2,20.1,35.0,15.0,6.4,2.7,0.0,0.0,0.0,0.0],[0.0,0.0,10.8,31.2,51.6,72.0,30.8,13.2,5.6,0.0,0.0,0.0],[0.0,0.0,0.0,0.0,4.5,17.2,30.0,12.8,5.5,2.3,0.0,0.0],[0.0,0.0,0.0,14.2,34.4,54.6,74.8,95.0,40.6,17.4,7.4,0.0],[0.0,0.0,0.0,0.0,0.0,0.0,5.7,21.8,38.0,16.2,6.9,3.0],[0.0,0.0,0.0,0.0,0.0,0.0,0.0,7.5,28.7,50.0,21.4,9.1]], palette=[14037868,3120708,1087661,7358696,15764480,1667522,14692657,6727695,11419337], variant=\"lineage\", height=680"
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
    let mut peak_value = vec![0.0f64; n];
    for m in 0..n {
        let row = &cfg.series_values[m];
        let len = row.len().min(p.n_axes);
        let mut best = 0usize;
        let mut best_val = f64::NEG_INFINITY;
        for ai in 0..len {
            if row[ai] > best_val {
                best_val = row[ai];
                best = ai;
            }
        }
        own_axis[m] = best;
        peak_value[m] = best_val.max(0.0);
        let mut r = 0usize;
        let mut ai = best;
        while ai > 0 && row[ai - 1] > 0.5 {
            r += 1;
            ai -= 1;
        }
        reach[m] = r.max(1);
    }
    let max_peak = peak_value.iter().cloned().fold(0.0f64, f64::max).max(1.0);
    let depth: Vec<f64> = peak_value.iter().map(|&v| 16.0 + (v / max_peak) * 76.0).collect();

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
                .kv("Peak decade", cfg.axes.get(own_axis[m]).cloned().unwrap_or_default())
                .kv("Peak activity", format!("{:.0}", peak_value[m]))
                .kv("Buildup", format!("{} decades", reach[m])),
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
        let values: Vec<Vec<f64>> = vec![
            vec![0.0, 20.0, 40.0, 15.0, 6.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 30.0, 60.0, 90.0, 38.0, 16.0, 0.0, 0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 10.0, 30.0, 58.0, 25.0, 11.0, 0.0, 0.0],
        ];
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

    fn stem_y1(html: &str, idx: i32) -> f64 {
        let needle = format!("<line data-idx=\"{idx}\" x1=\"");
        let start = html.find(&needle).expect("stem not found");
        let after_y1 = &html[start..];
        let y1_key = "y1=\"";
        let y1_start = after_y1.find(y1_key).unwrap() + y1_key.len();
        let rest = &after_y1[y1_start..];
        let y1_end = rest.find('"').unwrap();
        rest[..y1_end].parse().unwrap()
    }

    #[test]
    fn the_convergence_knot_sits_deeper_for_a_higher_peak_activity_value() {
        let (axes, names, values) = demo();
        let html = render(&cfg(&axes, &names, &values));
        let y_art_nouveau = stem_y1(&html, 0);
        let y_constructivism = stem_y1(&html, 1);
        assert!(
            y_constructivism > y_art_nouveau,
            "Constructivism peaks at 90 vs Art Nouveau's 40, its knot should sit further down: {y_constructivism} vs {y_art_nouveau}"
        );
    }

    #[test]
    fn peak_decade_is_the_axis_with_the_highest_activity_value() {
        let axes: Vec<String> = (0..12).map(|i| format!("{}", 1880 + i * 10)).collect();
        let names: Vec<String> = vec!["Bauhaus".into()];
        let values: Vec<Vec<f64>> =
            vec![vec![0.0, 0.0, 0.0, 0.0, 5.0, 20.0, 50.0, 22.0, 6.0, 0.0, 0.0, 0.0]];
        let html = render(&cfg(&axes, &names, &values));
        let expected_x = 50.0 + (6.0 / 11.0) * (1000.0 - 50.0 - 150.0);
        let start = html.find("<line data-idx=\"0\" x1=\"").unwrap() + "<line data-idx=\"0\" x1=\"".len();
        let rest = &html[start..];
        let x1: f64 = rest[..rest.find('"').unwrap()].parse().unwrap();
        assert!((x1 - expected_x).abs() < 0.5, "expected knot near axis 6 (x={expected_x}), got x1={x1}");
    }

    #[test]
    fn unclaimed_years_stay_as_plain_straight_gridlines() {
        let axes: Vec<String> = (0..12).map(|i| format!("{}", 1880 + i * 10)).collect();
        let names: Vec<String> = vec!["Bauhaus".into()];
        let values: Vec<Vec<f64>> =
            vec![vec![0.0, 0.0, 0.0, 0.0, 0.0, 20.0, 50.0, 0.0, 0.0, 0.0, 0.0, 0.0]];
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
        let values: Vec<Vec<f64>> = (0..9)
            .map(|i| {
                let mut row = vec![0.0; 12];
                row[i] = 30.0 + i as f64 * 6.0;
                if i > 0 {
                    row[i - 1] = 10.0;
                }
                row
            })
            .collect();
        let start = std::time::Instant::now();
        let html = render(&cfg(&axes, &names, &values));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
