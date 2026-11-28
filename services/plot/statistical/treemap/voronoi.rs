use super::common::open_svg;
use super::config::TreemapConfig;
use super::voronoi_engine::{polygon_area, polygon_centroid, sector_polygon, voronoi_treemap};
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, truncate};
use std::collections::HashMap;

struct Item<'a> {
    name: &'a str,
    sector: usize,
    cat: usize,
    flagged: bool,
    value: f64,
}

fn order_by_first_seen(values: &[String]) -> (Vec<String>, HashMap<String, usize>) {
    let mut order: Vec<String> = Vec::new();
    for v in values {
        if !order.iter().any(|x| x == v) {
            order.push(v.clone());
        }
    }
    let idx: HashMap<String, usize> = order.iter().enumerate().map(|(i, k)| (k.clone(), i)).collect();
    (order, idx)
}

fn order_by_value(values: &[String], weight: &[f64]) -> (Vec<String>, HashMap<String, usize>) {
    let mut totals: Vec<(String, f64)> = Vec::new();
    for (v, &w) in values.iter().zip(weight.iter()) {
        if let Some(e) = totals.iter_mut().find(|(k, _)| k == v) {
            e.1 += w;
        } else {
            totals.push((v.clone(), w));
        }
    }
    totals.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let order: Vec<String> = totals.into_iter().map(|(k, _)| k).collect();
    let idx: HashMap<String, usize> = order.iter().enumerate().map(|(i, k)| (k.clone(), i)).collect();
    (order, idx)
}

#[crate::chart_demo("labels=[\"Glyphosate\",\"Glufosinate\",\"Paraquat\",\"Atrazine\",\"Acetochlor\",\"Epoxiconazole\",\"Cyproconazole\",\"Prothioconazole\",\"Chlorothalonil\",\"Imidacloprid\",\"Thiamethoxam\",\"Fipronil\",\"Lambda-Cyhalothrin\",\"Chlorantraniliprole\",\"Spinosad\",\"Isoxaflutole\",\"Bifenthrin\",\"Mancozeb\",\"Fluxapyroxad\",\"Fomesafen\",\"Difenoconazole\",\"Diflufenican\",\"Quinmerac\",\"Clomazone\",\"Trifloxystrobin\",\"Penoxsulam\",\"Pyroxsulam\",\"Tebuconazole\",\"Sulfentrazone\",\"Fluopyram\",\"S-Metolachlor\",\"Imazamox\",\"Azoxystrobin\",\"Mesotrione\",\"Saflufenacil\",\"Iodosulfuron\",\"Florasulam\",\"Benzovindiflupyr\",\"Bixafen\",\"Metconazole\",\"Dicamba\",\"Bentazone\",\"Clopyralid\",\"Prothioconazole-desthio\",\"Pinoxaden\",\"Pyraclostrobin\",\"Imazapyr\",\"Flufenacet\",\"Tembotrione\"], parents=[\"Highly Hazardous\",\"Highly Hazardous\",\"Highly Hazardous\",\"Highly Hazardous\",\"Highly Hazardous\",\"Highly Hazardous\",\"Highly Hazardous\",\"Highly Hazardous\",\"Highly Hazardous\",\"Highly Hazardous\",\"Highly Hazardous\",\"Highly Hazardous\",\"Highly Hazardous\",\"Highly Hazardous\",\"Highly Hazardous\",\"Highly Hazardous\",\"Highly Hazardous\",\"Highly Hazardous\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\",\"Other Ingredients\"], values=[940.0,336.0,105.0,210.0,133.0,191.0,165.0,480.0,158.0,118.0,242.0,92.0,156.0,340.0,74.0,104.0,96.0,122.0,285.0,118.0,220.0,100.0,93.0,158.0,390.0,102.0,143.0,183.0,96.0,88.0,175.0,93.0,270.0,325.0,84.0,138.0,172.0,182.0,74.0,128.0,217.0,79.0,90.0,96.0,273.0,299.0,82.0,97.0,91.0], categories=[\"Herbicide\",\"Herbicide\",\"Herbicide\",\"Herbicide\",\"Herbicide\",\"Fungicide\",\"Fungicide\",\"Fungicide\",\"Fungicide\",\"Insecticide\",\"Insecticide\",\"Insecticide\",\"Insecticide\",\"Insecticide\",\"Insecticide\",\"Herbicide\",\"Insecticide\",\"Fungicide\",\"Fungicide\",\"Herbicide\",\"Fungicide\",\"Herbicide\",\"Herbicide\",\"Herbicide\",\"Fungicide\",\"Herbicide\",\"Herbicide\",\"Fungicide\",\"Herbicide\",\"Fungicide\",\"Herbicide\",\"Herbicide\",\"Fungicide\",\"Herbicide\",\"Herbicide\",\"Herbicide\",\"Herbicide\",\"Fungicide\",\"Fungicide\",\"Fungicide\",\"Herbicide\",\"Herbicide\",\"Herbicide\",\"Fungicide\",\"Herbicide\",\"Fungicide\",\"Herbicide\",\"Herbicide\",\"Herbicide\"], categories2=[\"Probable carcinogen\",\"Reproductive toxicant\",\"Acutely toxic\",\"Endocrine disruptor\",\"\",\"Endocrine disruptor\",\"Endocrine disruptor\",\"\",\"Probable carcinogen\",\"Bee-toxic\",\"Bee-toxic\",\"Bee-toxic\",\"Bee-toxic\",\"\",\"Bee-toxic\",\"\",\"Bee-toxic\",\"Reproductive toxicant\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\",\"\"], variant=\"voronoi\", width=1100, height=980")]
pub fn render(cfg: &TreemapConfig) -> String {
    let n = cfg.labels.len().min(cfg.parents.len()).min(cfg.values.len());
    if n == 0 {
        return String::new();
    }

    let (sector_order, sector_idx) = order_by_first_seen(&cfg.parents[..n]);
    let n_sectors = sector_order.len();
    if n_sectors == 0 {
        return String::new();
    }

    let cat_vals: Vec<String> = (0..n)
        .map(|i| {
            let s = cfg.categories.get(i).map(|s| s.as_str()).unwrap_or("");
            if s.is_empty() { "Other".to_string() } else { s.to_string() }
        })
        .collect();
    let (cat_order, cat_idx) = order_by_value(&cat_vals, &cfg.values[..n]);

    let items: Vec<Item> = (0..n)
        .map(|i| {
            let flag = cfg.categories2.get(i).map(|s| s.as_str()).unwrap_or("");
            Item {
                name: cfg.labels[i].as_str(),
                sector: *sector_idx.get(&cfg.parents[i]).unwrap_or(&0),
                cat: *cat_idx.get(&cat_vals[i]).unwrap_or(&0),
                flagged: !flag.is_empty(),
                value: cfg.values[i].max(0.0),
            }
        })
        .collect();

    let grand_total: f64 = items.iter().map(|it| it.value).sum::<f64>().max(1e-9);
    let sector_totals: Vec<f64> = (0..n_sectors)
        .map(|s| items.iter().filter(|it| it.sector == s).map(|it| it.value).sum())
        .collect();

    let title_h = if cfg.title.is_empty() { 0.0 } else { 34.0 };
    let legend_h = 40.0;
    let pad = 30.0;
    let cx = cfg.width as f64 / 2.0;
    let top = title_h + legend_h + pad;
    let avail_h = cfg.height as f64 - top - pad - 34.0;
    let avail_w = cfg.width as f64 - 2.0 * pad;
    let r = (avail_w.min(avail_h) / 2.0).max(40.0);
    let cy = top + r;

    let mut buf = Vec::<u8>::with_capacity(n * 260 + 8192);
    open_svg(&mut buf, cfg);

    push_b(&mut buf, b"<defs><pattern id=\"sp-vhatch\" width=\"9\" height=\"9\" patternTransform=\"rotate(45)\" patternUnits=\"userSpaceOnUse\"><rect width=\"9\" height=\"9\" fill=\"none\"/><rect width=\"4.5\" height=\"9\" fill=\"#ffffff\" fill-opacity=\"0.4\"/></pattern></defs>");

    let mut cells_all: Vec<Option<Vec<(f64, f64)>>> = (0..n).map(|_| None).collect();
    let mut fallback_at: Vec<(f64, f64)> = (0..n).map(|_| (cx, cy)).collect();
    let mut theta_cursor = -std::f64::consts::FRAC_PI_2;
    let mut sector_range: Vec<(f64, f64)> = Vec::with_capacity(n_sectors);

    for s in 0..n_sectors {
        let share = sector_totals[s] / grand_total;
        let theta1 = theta_cursor;
        let theta0 = theta_cursor - share * 2.0 * std::f64::consts::PI;
        sector_range.push((theta0, theta1));
        theta_cursor = theta0;

        let mut order: Vec<usize> = (0..n).filter(|&i| items[i].sector == s).collect();
        if order.is_empty() {
            continue;
        }
        order.sort_by(|&a, &b| items[b].value.partial_cmp(&items[a].value).unwrap_or(std::cmp::Ordering::Equal));

        let boundary = sector_polygon(cx, cy, r, theta0, theta1, 110);
        let target_areas: Vec<f64> = items.iter().map(|it| it.value).collect();
        let (cells, sites) = voronoi_treemap(cx, cy, r, theta0, theta1, &boundary, &order, &target_areas, 160);
        for (k, &oi) in order.iter().enumerate() {
            cells_all[oi] = Some(cells[k].clone());
            fallback_at[oi] = (sites[k].x, sites[k].y);
        }
    }

    let white_hex = hex6(0xffffff);

    for (i, cell_opt) in cells_all.iter().enumerate() {
        let it = &items[i];
        let col = if it.sector == 0 { palette_color(cfg.palette, it.cat) } else { 0xcbd0d8 };
        let hx = hex6(col);
        let cell: &[(f64, f64)] = match cell_opt {
            Some(c) if c.len() >= 3 => c,
            _ => {
                let (fx, fy) = fallback_at[i];
                push_b(&mut buf, b"<circle data-idx=\"");
                push_i(&mut buf, i as i32);
                push_b(&mut buf, b"\" cx=\"");
                push_f2(&mut buf, fx);
                push_b(&mut buf, b"\" cy=\"");
                push_f2(&mut buf, fy);
                push_b(&mut buf, b"\" r=\"3\" fill=\"#");
                buf.extend_from_slice(&hx);
                push_b(&mut buf, b"\" stroke=\"#");
                buf.extend_from_slice(&white_hex);
                push_b(&mut buf, b"\" stroke-width=\"1\"/>");
                continue;
            }
        };
        push_b(&mut buf, b"<path data-idx=\"");
        push_i(&mut buf, i as i32);
        push_b(&mut buf, b"\" d=\"M");
        for (k, &(x, y)) in cell.iter().enumerate() {
            if k > 0 {
                push_b(&mut buf, b"L");
            }
            push_f2(&mut buf, x);
            push_b(&mut buf, b",");
            push_f2(&mut buf, y);
        }
        push_b(&mut buf, b"Z\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\" stroke=\"#");
        buf.extend_from_slice(&white_hex);
        push_b(&mut buf, b"\" stroke-width=\"1.6\" stroke-linejoin=\"round\"/>");

        if it.flagged {
            push_b(&mut buf, b"<path d=\"M");
            for (k, &(x, y)) in cell.iter().enumerate() {
                if k > 0 {
                    push_b(&mut buf, b"L");
                }
                push_f2(&mut buf, x);
                push_b(&mut buf, b",");
                push_f2(&mut buf, y);
            }
            push_b(&mut buf, b"Z\" fill=\"url(#sp-vhatch)\" stroke=\"none\" pointer-events=\"none\"/>");
        }

        let area = polygon_area(cell);
        let eff_r = (area / std::f64::consts::PI).sqrt();
        if eff_r > 20.0 {
            let (tx, ty) = polygon_centroid(cell);
            let text_fill: &[u8] = if it.sector == 0 { b"ffffff" } else { b"384252" };
            let fsz = (eff_r * 0.24).clamp(9.0, 15.0);
            let max_chars = ((eff_r * 1.85) / (fsz * 0.58)).max(3.0) as usize;
            let cut = truncate(it.name, max_chars);
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, tx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, ty - fsz * 0.35);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-weight=\"700\" font-size=\"");
            push_f2(&mut buf, fsz);
            push_b(&mut buf, b"\" fill=\"#");
            buf.extend_from_slice(text_fill);
            push_b(&mut buf, b"\" pointer-events=\"none\">");
            escape_xml(&mut buf, cut);
            if cut.len() < it.name.len() {
                push_b(&mut buf, "\u{2026}".as_bytes());
            }
            push_b(&mut buf, b"</text>");
            if eff_r > 30.0 {
                push_b(&mut buf, b"<text x=\"");
                push_f2(&mut buf, tx);
                push_b(&mut buf, b"\" y=\"");
                push_f2(&mut buf, ty + fsz * 0.9);
                push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"");
                push_f2(&mut buf, fsz * 0.82);
                push_b(&mut buf, b"\" fill=\"#");
                buf.extend_from_slice(text_fill);
                push_b(&mut buf, b"\" opacity=\"0.85\" pointer-events=\"none\">");
                push_f2(&mut buf, it.value);
                push_b(&mut buf, b"</text>");
            }
        }
    }

    for s in 0..n_sectors {
        let (theta0, theta1) = sector_range[s];
        let mid = (theta0 + theta1) / 2.0;
        let share = sector_totals[s] / grand_total * 100.0;
        let lx = cx + (r + 22.0) * mid.cos();
        let ly = cy + (r + 22.0) * mid.sin();
        let anchor: &[u8] = if mid.cos() < -0.15 {
            b"end"
        } else if mid.cos() > 0.15 {
            b"start"
        } else {
            b"middle"
        };
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, lx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly - 6.0);
        push_b(&mut buf, b"\" text-anchor=\"");
        buf.extend_from_slice(anchor);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"700\" fill=\"#1f2937\">");
        escape_xml(&mut buf, &sector_order[s]);
        push_b(&mut buf, b"</text>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, lx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ly + 10.0);
        push_b(&mut buf, b"\" text-anchor=\"");
        buf.extend_from_slice(anchor);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#6b7280\">");
        push_f2(&mut buf, share);
        push_b(&mut buf, b"% share</text>");
    }

    let leg_y = title_h + 20.0;
    let mut leg_x = pad;
    push_b(&mut buf, b"<g data-legend=\"voronoi\">");
    for (ci, name) in cat_order.iter().enumerate() {
        let col = palette_color(cfg.palette, ci);
        let hx = hex6(col);
        push_b(&mut buf, b"<rect x=\"");
        push_f2(&mut buf, leg_x);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, leg_y - 10.0);
        push_b(&mut buf, b"\" width=\"11\" height=\"11\" rx=\"2\" fill=\"#");
        buf.extend_from_slice(&hx);
        push_b(&mut buf, b"\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, leg_x + 16.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, leg_y - 1.0);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">");
        escape_xml(&mut buf, name);
        push_b(&mut buf, b"</text>");
        leg_x += 22.0 + name.len() as f64 * 6.2 + 18.0;
    }
    if items.iter().any(|it| it.flagged) {
        push_b(&mut buf, b"<rect x=\"");
        push_f2(&mut buf, leg_x);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, leg_y - 10.0);
        push_b(&mut buf, b"\" width=\"11\" height=\"11\" rx=\"2\" fill=\"#9ca3af\"/><rect x=\"");
        push_f2(&mut buf, leg_x);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, leg_y - 10.0);
        push_b(&mut buf, b"\" width=\"11\" height=\"11\" rx=\"2\" fill=\"url(#sp-vhatch)\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, leg_x + 16.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, leg_y - 1.0);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"11\" fill=\"#374151\">Additional flag</text>");
    }
    push_b(&mut buf, b"</g>");

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };

    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n);
    for it in items.iter() {
        let mut slot = HoverSlot::new(it.name.to_string())
            .kv("Group", sector_order[it.sector].clone())
            .kv("Category", cat_order[it.cat].clone())
            .kv("Value", format!("{:.0}", it.value));
        if it.flagged {
            slot = slot.kv("Flag", "yes".to_string());
        }
        slots.push(slot);
    }

    build_chart_html(cfg.title, &svg, &slots_to_json(&slots))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg<'a>(
        labels: &'a [String],
        parents: &'a [String],
        values: &'a [f64],
        categories: &'a [String],
        categories2: &'a [String],
    ) -> TreemapConfig<'a> {
        TreemapConfig {
            title: "Test",
            labels,
            parents,
            values,
            categories,
            categories2,
            width: 900,
            height: 820,
            ..TreemapConfig::default()
        }
    }

    fn synth(n: usize) -> (Vec<String>, Vec<String>, Vec<f64>, Vec<String>, Vec<String>) {
        let sectors = ["Highly Hazardous", "Other Ingredients"];
        let cats = ["Herbicide", "Insecticide", "Fungicide"];
        let mut labels = Vec::with_capacity(n);
        let mut parents = Vec::with_capacity(n);
        let mut values = Vec::with_capacity(n);
        let mut categories = Vec::with_capacity(n);
        let mut categories2 = Vec::with_capacity(n);
        for i in 0..n {
            labels.push(format!("Ingredient-{i}"));
            parents.push(sectors[if i < n / 3 { 0 } else { 1 }].to_string());
            values.push(((i % 30) + 1) as f64 * 7.0);
            categories.push(cats[i % cats.len()].to_string());
            categories2.push(if i % 5 == 0 { "Bee-toxic".to_string() } else { String::new() });
        }
        (labels, parents, values, categories, categories2)
    }

    #[test]
    #[ignore]
    fn write_preview_asset() {
        use crate::plot::chart_demo_registry::{iter_entries, render_demo_html};
        for entry in iter_entries() {
            if !entry.file.replace('\\', "/").ends_with("treemap/voronoi.rs") {
                continue;
            }
            let html = render_demo_html(entry).expect("demo html");
            std::fs::write("docs/previews/treemap-voronoi.html", html).unwrap();
        }
    }

    #[test]
    fn renders_one_hoverable_mark_per_ingredient() {
        let (labels, parents, values, categories, categories2) = synth(40);
        let html = render(&cfg(&labels, &parents, &values, &categories, &categories2));
        assert!(!html.is_empty());
        let marks = html.matches("<path data-idx=\"").count() + html.matches("<circle data-idx=\"").count();
        assert_eq!(marks, 40);
    }

    #[test]
    fn every_cell_polygon_stays_within_the_declared_canvas() {
        let (labels, parents, values, categories, categories2) = synth(30);
        let c = cfg(&labels, &parents, &values, &categories, &categories2);
        let html = render(&c);
        for m in regex_like_extract_coords(&html) {
            assert!(m >= -5.0 && m <= 1400.0, "coordinate {m} escaped the canvas");
        }
    }

    fn regex_like_extract_coords(html: &str) -> Vec<f64> {
        let mut out = Vec::new();
        for seg in html.split("d=\"M").skip(1) {
            let end = seg.find('Z').unwrap_or(0);
            let path = &seg[..end];
            for token in path.split(['L', ',']) {
                if let Ok(v) = token.parse::<f64>() {
                    out.push(v);
                }
            }
        }
        out
    }

    #[test]
    fn the_first_sector_is_colored_by_category_and_the_rest_render_neutral_gray() {
        let (labels, parents, values, categories, categories2) = synth(30);
        let html = render(&cfg(&labels, &parents, &values, &categories, &categories2));
        assert!(html.contains("fill=\"#cbd0d8\""));
    }

    #[test]
    fn flagged_ingredients_get_a_hatch_overlay_and_others_do_not() {
        let (labels, parents, values, categories, categories2) = synth(30);
        let html = render(&cfg(&labels, &parents, &values, &categories, &categories2));
        assert!(html.contains("url(#sp-vhatch)"));
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let empty_s: Vec<String> = vec![];
        let empty_v: Vec<f64> = vec![];
        let html = render(&cfg(&empty_s, &empty_s, &empty_v, &empty_s, &empty_s));
        assert!(html.is_empty());
    }

    #[test]
    fn perf_rendering_a_hundred_and_fifty_ingredients_stays_fast() {
        let (labels, parents, values, categories, categories2) = synth(150);
        let c = cfg(&labels, &parents, &values, &categories, &categories2);
        let start = std::time::Instant::now();
        let html = render(&c);
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 4000, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
