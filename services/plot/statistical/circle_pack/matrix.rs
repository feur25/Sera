use super::common::pack_local;
use super::config::CirclePackConfig;
use crate::html::hover::{build_chart_html, slots_to_json, HoverSlot};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i};
use crate::plot::statistical::scatter::common::{cycle_symbol, draw_marker};
use std::collections::HashMap;

const R_MIN: f64 = 2.0;
const R_MAX: f64 = 15.0;
const PAD: f64 = 22.0;
const LEGEND_H: f64 = 108.0;
const COLHEAD_H: f64 = 40.0;
const ROWLABEL_W: f64 = 150.0;

struct Sat<'a> {
    name: &'a str,
    region: usize,
    orbit: usize,
    cat: usize,
    sym: &'a str,
    mass: f64,
    shade: Option<f64>,
}

fn ordered_by_count(values: &[String]) -> (Vec<String>, HashMap<String, usize>) {
    let mut counts: Vec<(String, usize)> = Vec::new();
    for v in values {
        if let Some(e) = counts.iter_mut().find(|(k, _)| k == v) {
            e.1 += 1;
        } else {
            counts.push((v.clone(), 1));
        }
    }
    counts.sort_by(|a, b| b.1.cmp(&a.1));
    let order: Vec<String> = counts.into_iter().map(|(k, _)| k).collect();
    let idx: HashMap<String, usize> = order.iter().enumerate().map(|(i, k)| (k.clone(), i)).collect();
    (order, idx)
}

fn proportional_extents(counts: &[usize], total: f64, gap: f64) -> Vec<f64> {
    let n = counts.len();
    if n == 0 {
        return Vec::new();
    }
    let sum = counts.iter().map(|&c| c as f64).sum::<f64>().max(1.0);
    let equal = 1.0 / n as f64;
    let floor = equal * 0.45;
    let mut frac: Vec<f64> = counts.iter().map(|&c| (c as f64 / sum).max(floor)).collect();
    let fsum: f64 = frac.iter().sum();
    for f in frac.iter_mut() {
        *f /= fsum;
    }
    let avail = (total - gap * (n as f64 - 1.0)).max(n as f64 * 6.0);
    frac.into_iter().map(|f| f * avail).collect()
}

fn cap_label(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

fn legend_dot(buf: &mut Vec<u8>, x: f64, y: f64, r: f64, hex: &[u8; 6], opacity: f64) {
    push_b(buf, b"<circle cx=\"");
    push_f2(buf, x);
    push_b(buf, b"\" cy=\"");
    push_f2(buf, y);
    push_b(buf, b"\" r=\"");
    push_f2(buf, r.max(1.0));
    push_b(buf, b"\" fill=\"#");
    buf.extend_from_slice(hex);
    push_b(buf, b"\" fill-opacity=\"");
    push_f2(buf, opacity);
    push_b(buf, b"\"/>");
}

fn legend_title(buf: &mut Vec<u8>, x: f64, y: f64, text: &str) {
    push_b(buf, b"<text x=\"");
    push_f2(buf, x);
    push_b(buf, b"\" y=\"");
    push_f2(buf, y);
    push_b(buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" letter-spacing=\"1\" fill=\"#8291b3\">");
    push_b(buf, text.as_bytes());
    push_b(buf, b"</text>");
}

#[crate::chart_demo("labels=[\"Nova-8\",\"Zenith-8\",\"Nova-51\",\"Orbis-74\",\"Orbis-71\",\"Kepler-60\",\"Nimbus-74\",\"Orbis-66\",\"Nimbus-98\",\"Nova-59\",\"Vega-40\",\"Meridian-3\",\"Vela-95\",\"Solis-18\",\"Draco-20\",\"Solis-37\",\"Orion-66\",\"Sentinel-72\",\"Lyra-57\",\"Meridian-79\",\"Terra-47\",\"Nimbus-19\",\"Lyra-68\",\"Orion-12\",\"Atlas-65\",\"Cygnus-52\",\"Sentinel-36\",\"Meridian-47\",\"Cygnus-1\",\"Sentinel-92\",\"Pulsar-60\",\"Nova-60\",\"Vela-71\",\"Echo-18\",\"Rigel-31\",\"Meridian-59\",\"Atlas-20\",\"Corvus-19\",\"Halo-14\",\"Atlas-4\",\"Lyra-89\",\"Titan-34\",\"Argus-41\",\"Vela-92\",\"Orbis-51\",\"Pulsar-44\",\"Argus-91\",\"Draco-14\",\"Orion-87\",\"Kepler-12\",\"Ionos-12\",\"Aster-44\",\"Draco-15\",\"Beacon-27\",\"Helio-2\",\"Orbis-85\",\"Astra-89\",\"Pulsar-45\",\"Corvus-8\",\"Vega-38\",\"Atlas-42\",\"Nimbus-61\",\"Vela-52\",\"Orion-97\",\"Halo-20\",\"Ionos-55\",\"Cygnus-88\",\"Helio-18\",\"Ionos-69\",\"Titan-69\",\"Draco-94\",\"Halo-88\",\"Solis-84\",\"Quasar-13\",\"Beacon-16\",\"Argus-10\",\"Nimbus-75\",\"Rigel-36\",\"Corvus-1\",\"Kepler-16\",\"Vega-2\",\"Nimbus-47\",\"Astra-82\",\"Sentinel-55\",\"Lyra-93\",\"Halo-7\",\"Comet-95\",\"Orbis-22\",\"Echo-43\",\"Atlas-12\",\"Pulsar-53\",\"Meridian-17\"], parents=[\"U.S.\",\"Other\",\"India\",\"U.S.\",\"U.S.\",\"W. Europe\",\"Russia\",\"U.S.\",\"China\",\"Russia\",\"U.S.\",\"W. Europe\",\"Other\",\"U.S.\",\"Japan\",\"U.S.\",\"U.S.\",\"Other\",\"China\",\"U.S.\",\"U.S.\",\"China\",\"U.S.\",\"China\",\"W. Europe\",\"U.S.\",\"W. Europe\",\"China\",\"U.S.\",\"China\",\"Japan\",\"China\",\"Japan\",\"Russia\",\"China\",\"Japan\",\"W. Europe\",\"Russia\",\"China\",\"India\",\"Japan\",\"U.S.\",\"Other\",\"U.S.\",\"W. Europe\",\"India\",\"China\",\"U.S.\",\"U.S.\",\"Japan\",\"U.S.\",\"Japan\",\"Other\",\"Other\",\"U.S.\",\"U.S.\",\"Japan\",\"U.S.\",\"Other\",\"U.S.\",\"U.S.\",\"U.S.\",\"U.S.\",\"Russia\",\"U.S.\",\"U.S.\",\"W. Europe\",\"Russia\",\"W. Europe\",\"W. Europe\",\"U.S.\",\"Japan\",\"U.S.\",\"Japan\",\"W. Europe\",\"Other\",\"Japan\",\"U.S.\",\"India\",\"Other\",\"India\",\"Other\",\"Other\",\"Other\",\"India\",\"U.S.\",\"Other\",\"Other\",\"W. Europe\",\"Other\",\"U.S.\",\"Japan\"], categories=[\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Medium Earth Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Other Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Other Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Low Earth Orbit\",\"Geosynchronous Orbit\",\"Other Orbit\",\"Medium Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\",\"Low Earth Orbit\"], categories2=[\"Technology\",\"Communications\",\"Communications\",\"Navigation\",\"Communications\",\"Navigation\",\"Communications\",\"Earth Observation\",\"Communications\",\"Research\",\"Communications\",\"Research\",\"Navigation\",\"Research\",\"Communications\",\"Technology\",\"Earth Observation\",\"Technology\",\"Communications\",\"Communications\",\"Navigation\",\"Earth Observation\",\"Communications\",\"Research\",\"Earth Observation\",\"Research\",\"Earth Observation\",\"Navigation\",\"Earth Observation\",\"Research\",\"Communications\",\"Technology\",\"Research\",\"Research\",\"Research\",\"Navigation\",\"Earth Observation\",\"Earth Observation\",\"Navigation\",\"Communications\",\"Earth Observation\",\"Research\",\"Communications\",\"Communications\",\"Communications\",\"Technology\",\"Communications\",\"Navigation\",\"Research\",\"Communications\",\"Communications\",\"Research\",\"Research\",\"Communications\",\"Communications\",\"Research\",\"Earth Observation\",\"Communications\",\"Communications\",\"Research\",\"Communications\",\"Earth Observation\",\"Communications\",\"Communications\",\"Research\",\"Communications\",\"Communications\",\"Technology\",\"Earth Observation\",\"Communications\",\"Technology\",\"Technology\",\"Navigation\",\"Navigation\",\"Communications\",\"Earth Observation\",\"Earth Observation\",\"Communications\",\"Communications\",\"Earth Observation\",\"Technology\",\"Earth Observation\",\"Research\",\"Communications\",\"Navigation\",\"Earth Observation\",\"Communications\",\"Technology\",\"Earth Observation\",\"Communications\",\"Research\",\"Communications\"], symbols=[\"circle\",\"circle\",\"star\",\"circle\",\"star\",\"star\",\"diamond\",\"circle\",\"star\",\"circle\",\"star\",\"circle\",\"star\",\"circle\",\"circle\",\"circle\",\"star\",\"star\",\"star\",\"circle\",\"circle\",\"triangle\",\"diamond\",\"diamond\",\"circle\",\"triangle\",\"circle\",\"circle\",\"circle\",\"circle\",\"diamond\",\"circle\",\"triangle\",\"star\",\"circle\",\"diamond\",\"diamond\",\"circle\",\"circle\",\"circle\",\"star\",\"star\",\"circle\",\"star\",\"circle\",\"circle\",\"circle\",\"star\",\"circle\",\"triangle\",\"diamond\",\"circle\",\"star\",\"triangle\",\"circle\",\"star\",\"diamond\",\"diamond\",\"star\",\"star\",\"circle\",\"circle\",\"diamond\",\"star\",\"star\",\"diamond\",\"star\",\"star\",\"star\",\"diamond\",\"star\",\"star\",\"circle\",\"circle\",\"star\",\"circle\",\"circle\",\"circle\",\"diamond\",\"star\",\"diamond\",\"circle\",\"circle\",\"star\",\"triangle\",\"star\",\"circle\",\"circle\",\"diamond\",\"circle\",\"circle\",\"diamond\"], values=[500.9,168.1,309.5,400.5,4394.6,497.3,3875.6,563.7,1396.0,2950.9,133.0,371.8,227.7,426.3,2993.9,754.7,859.7,5554.9,132.4,150.3,256.9,29.3,1510.3,2690.4,703.9,51.3,64.9,735.4,455.1,4502.9,2263.1,149.3,39.8,128.4,256.6,1068.0,5590.8,43.4,3791.7,139.5,474.8,253.0,398.3,276.8,213.5,647.4,187.6,95.3,690.0,34.7,2544.6,782.0,77.2,38.1,123.6,251.3,2529.6,4006.9,5599.3,2609.7,192.8,40.9,1654.3,136.9,697.3,3946.0,3721.9,237.7,83.7,4092.4,735.9,767.2,555.7,92.2,3523.3,446.7,220.5,4216.8,3630.7,3280.9,1328.2,899.0,1458.1,158.0,5903.5,159.2,282.3,519.3,3221.7,138.1,57.3,2991.8], color_values=[1998,2010,2011,2009,1998,2024,1990,1979,2017,2006,2021,2017,1993,2010,1999,1986,2019,2018,1979,1981,1997,1994,1976,2016,2009,1990,1976,2021,2014,1999,2021,1984,1997,2022,1993,2022,1983,1984,2010,2003,2007,2008,2000,2024,2022,2007,2004,2025,2002,2019,1976,2004,2020,2008,1991,2003,2007,1983,2002,1990,1996,1999,1980,2008,2021,2007,1976,1976,1976,2007,1991,1979,1996,1992,2004,1993,1988,2015,1976,1999,1987,2012,2017,1998,2010,1993,1991,2000,2003,1996,2001,2011], variant=\"matrix\", width=1220, height=800")]
pub fn render(cfg: &CirclePackConfig) -> String {
    let n = cfg
        .labels
        .len()
        .min(cfg.parents.len())
        .min(cfg.categories.len())
        .min(cfg.values.len());
    if n == 0 {
        return String::new();
    }

    let (region_order, region_idx) = ordered_by_count(&cfg.parents[..n]);
    let (orbit_order, orbit_idx) = ordered_by_count(&cfg.categories[..n]);

    let cat2_vals: Vec<String> = (0..n)
        .map(|i| {
            let s = cfg.categories2.get(i).map(|s| s.as_str()).unwrap_or("");
            if s.is_empty() { "Other".to_string() } else { s.to_string() }
        })
        .collect();
    let (cat_order, cat_idx) = ordered_by_count(&cat2_vals);

    let sats: Vec<Sat> = (0..n)
        .map(|i| {
            let ci = *cat_idx.get(&cat2_vals[i]).unwrap_or(&0);
            let raw_sym = cfg.symbols.get(i).map(|s| s.as_str()).unwrap_or("");
            let sym = if raw_sym.is_empty() { cycle_symbol(ci) } else { raw_sym };
            Sat {
                name: cfg.labels[i].as_str(),
                region: *region_idx.get(&cfg.parents[i]).unwrap_or(&0),
                orbit: *orbit_idx.get(&cfg.categories[i]).unwrap_or(&0),
                cat: ci,
                sym,
                mass: cfg.values[i].max(0.0),
                shade: cfg.color_values.get(i).copied(),
            }
        })
        .collect();

    let n_cols = region_order.len();
    let n_rows = orbit_order.len();
    if n_cols == 0 || n_rows == 0 {
        return String::new();
    }

    let mass_max = sats.iter().map(|s| s.mass).fold(0.0_f64, f64::max).max(1.0);
    let radii: Vec<f64> = sats
        .iter()
        .map(|s| R_MIN + (s.mass / mass_max).sqrt() * (R_MAX - R_MIN))
        .collect();

    let shades: Vec<f64> = sats.iter().filter_map(|s| s.shade).collect();
    let has_shade = !shades.is_empty();
    let shade_min = shades.iter().cloned().fold(f64::INFINITY, f64::min);
    let shade_max = shades.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let shade_span = (shade_max - shade_min).max(1e-9);

    let mut cells: Vec<Vec<usize>> = vec![Vec::new(); n_cols * n_rows];
    for (i, s) in sats.iter().enumerate() {
        cells[s.orbit * n_cols + s.region].push(i);
    }
    let col_counts: Vec<usize> = (0..n_cols)
        .map(|c| sats.iter().filter(|s| s.region == c).count())
        .collect();
    let row_counts: Vec<usize> = (0..n_rows)
        .map(|r| sats.iter().filter(|s| s.orbit == r).count())
        .collect();

    let title_h = if cfg.title.is_empty() { 0.0 } else { 30.0 };
    let plot_l = PAD + ROWLABEL_W;
    let plot_t = PAD + title_h + LEGEND_H + COLHEAD_H;
    let plot_w = (cfg.width as f64 - plot_l - PAD).max(160.0);
    let plot_h = (cfg.height as f64 - plot_t - PAD).max(160.0);

    let col_w = proportional_extents(&col_counts, plot_w, 3.0);
    let row_h = proportional_extents(&row_counts, plot_h, 3.0);

    let mut col_x = vec![0.0_f64; n_cols];
    let mut acc = plot_l;
    for i in 0..n_cols {
        col_x[i] = acc;
        acc += col_w[i] + 3.0;
    }
    let mut row_y = vec![0.0_f64; n_rows];
    let mut acc = plot_t;
    for j in 0..n_rows {
        row_y[j] = acc;
        acc += row_h[j] + 3.0;
    }

    let mut buf = Vec::<u8>::with_capacity(n * 220 + 8192);
    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\" viewBox=\"0 0 ");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b" ");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect width=\"100%\" height=\"100%\" fill=\"#0b1220\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cfg.width as f64 / 2.0);
        push_b(&mut buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"14\" font-weight=\"700\" fill=\"#e2e8f0\" letter-spacing=\"1\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    let leg_y0 = PAD + title_h;
    let leg_x0 = plot_l;
    let n_seg = if has_shade { 4 } else { 3 };
    let seg_w = plot_w / n_seg as f64;

    push_b(&mut buf, b"<g data-legend=\"matrix\">");

    {
        legend_title(&mut buf, leg_x0, leg_y0 + 12.0, "SIZE (KG)");
        let refs: [f64; 3] = [100.0, 1000.0, 5000.0];
        let mut dx = leg_x0 + 10.0;
        let dy = leg_y0 + 48.0;
        for &m in refs.iter() {
            let t = (m / mass_max).clamp(0.0, 1.0).sqrt();
            let r = R_MIN + t * (R_MAX - R_MIN);
            legend_dot(&mut buf, dx, dy, r, b"9fb0d0", 0.88);
            let label = if m >= 1000.0 {
                format!("{:.0}k", m / 1000.0)
            } else {
                format!("{:.0}", m)
            };
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, dx);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, dy + R_MAX + 13.0);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#7d8ba8\">");
            push_b(&mut buf, label.as_bytes());
            push_b(&mut buf, b"</text>");
            dx += 36.0;
        }
    }

    {
        let sx = leg_x0 + seg_w;
        legend_title(&mut buf, sx, leg_y0 + 12.0, "CATEGORY");
        let mut fx = sx;
        let mut fy = leg_y0 + 30.0;
        let max_x = sx + seg_w - 10.0;
        for (ci, name) in cat_order.iter().enumerate() {
            let w_est = 16.0 + name.len() as f64 * 5.6;
            if fx + w_est > max_x && fx > sx {
                fx = sx;
                fy += 17.0;
            }
            let col = palette_color(cfg.palette, ci);
            legend_dot(&mut buf, fx + 4.0, fy, 4.0, &hex6(col), 0.92);
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, fx + 12.0);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, fy + 3.0);
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#c3cee3\">");
            escape_xml(&mut buf, name);
            push_b(&mut buf, b"</text>");
            fx += w_est;
        }
    }

    {
        let sx = leg_x0 + seg_w * 2.0;
        legend_title(&mut buf, sx, leg_y0 + 12.0, "CLASS");
        let sym_names: Vec<String> = sats.iter().map(|s| s.sym.to_string()).collect();
        let (sym_order, _) = ordered_by_count(&sym_names);
        let mut fx = sx;
        let mut fy = leg_y0 + 30.0;
        let max_x = sx + seg_w - 10.0;
        for sym in sym_order.iter() {
            let label = cap_label(sym);
            let w_est = 18.0 + label.len() as f64 * 5.6;
            if fx + w_est > max_x && fx > sx {
                fx = sx;
                fy += 18.0;
            }
            draw_marker(&mut buf, sym, (fx + 5.0) as i32, fy as i32, 5.0, b"c3cee3", b"c3cee3", 1.0, 0.9);
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, fx + 15.0);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, fy + 3.0);
            push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"10\" fill=\"#c3cee3\">");
            escape_xml(&mut buf, &label);
            push_b(&mut buf, b"</text>");
            fx += w_est;
        }
    }

    if has_shade {
        let sx = leg_x0 + seg_w * 3.0;
        legend_title(&mut buf, sx, leg_y0 + 12.0, "LAUNCH DATE");
        let dy = leg_y0 + 44.0;
        legend_dot(&mut buf, sx + 8.0, dy, 6.0, b"9fb0d0", 0.40);
        legend_dot(&mut buf, sx + 40.0, dy, 6.0, b"9fb0d0", 0.92);
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, sx + 8.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, dy + 20.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#7d8ba8\">");
        push_b(&mut buf, format!("{:.0}", shade_min).as_bytes());
        push_b(&mut buf, b"</text>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, sx + 40.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, dy + 20.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#7d8ba8\">");
        push_b(&mut buf, format!("{:.0}", shade_max).as_bytes());
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"<line x1=\"");
    push_f2(&mut buf, leg_x0);
    push_b(&mut buf, b"\" y1=\"");
    push_f2(&mut buf, leg_y0 + LEGEND_H - 6.0);
    push_b(&mut buf, b"\" x2=\"");
    push_f2(&mut buf, leg_x0 + plot_w);
    push_b(&mut buf, b"\" y2=\"");
    push_f2(&mut buf, leg_y0 + LEGEND_H - 6.0);
    push_b(&mut buf, b"\" stroke=\"#1c2942\" stroke-width=\"1\"/>");

    push_b(&mut buf, b"</g>");

    for (i, name) in region_order.iter().enumerate() {
        let cx = col_x[i] + col_w[i] / 2.0;
        let cy = plot_t - COLHEAD_H;
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy + 16.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"700\" fill=\"#e2e8f0\">");
        escape_xml(&mut buf, name);
        push_b(&mut buf, b"</text>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy + 30.0);
        push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9.5\" fill=\"#7d8ba8\">");
        push_b(&mut buf, col_counts[i].to_string().as_bytes());
        push_b(&mut buf, if col_counts[i] == 1 { b" sat" } else { b" sats" });
        push_b(&mut buf, b"</text>");
    }

    for (j, name) in orbit_order.iter().enumerate() {
        let ry = row_y[j] + row_h[j] / 2.0 + 4.0;
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, plot_l - 12.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, ry);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"11.5\" font-weight=\"600\" fill=\"#c3cee3\">");
        escape_xml(&mut buf, name);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"<rect x=\"");
    push_f2(&mut buf, plot_l);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, plot_t);
    push_b(&mut buf, b"\" width=\"");
    push_f2(&mut buf, plot_w);
    push_b(&mut buf, b"\" height=\"");
    push_f2(&mut buf, plot_h);
    push_b(&mut buf, b"\" fill=\"none\" stroke=\"#243254\" stroke-width=\"1\"/>");

    for i in 1..n_cols {
        let x = col_x[i] - 1.5;
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, x);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, plot_t);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, x);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, plot_t + plot_h);
        push_b(&mut buf, b"\" stroke=\"#1c2942\" stroke-width=\"1\"/>");
    }
    for j in 1..n_rows {
        let y = row_y[j] - 1.5;
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, plot_l);
        push_b(&mut buf, b"\" y1=\"");
        push_f2(&mut buf, y);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, plot_l + plot_w);
        push_b(&mut buf, b"\" y2=\"");
        push_f2(&mut buf, y);
        push_b(&mut buf, b"\" stroke=\"#1c2942\" stroke-width=\"1\"/>");
    }

    let bg_hex = hex6(0x0b1220);
    for r in 0..n_rows {
        for c in 0..n_cols {
            let members = &cells[r * n_cols + c];
            if members.is_empty() {
                continue;
            }
            let mut order = members.clone();
            order.sort_by(|&a, &b| radii[b].partial_cmp(&radii[a]).unwrap_or(std::cmp::Ordering::Equal));
            let local_radii: Vec<f64> = order.iter().map(|&i| radii[i]).collect();
            let raw_pos = pack_local(&local_radii, 1.0);

            let mut min_x = f64::MAX;
            let mut max_x = f64::MIN;
            let mut min_y = f64::MAX;
            let mut max_y = f64::MIN;
            for (k, &(px, py)) in raw_pos.iter().enumerate() {
                let rr = local_radii[k];
                min_x = min_x.min(px - rr);
                max_x = max_x.max(px + rr);
                min_y = min_y.min(py - rr);
                max_y = max_y.max(py + rr);
            }
            let bbox_w = (max_x - min_x).max(1e-6);
            let bbox_h = (max_y - min_y).max(1e-6);
            let mid_x = (min_x + max_x) / 2.0;
            let mid_y = (min_y + max_y) / 2.0;

            let cell_cx = col_x[c] + col_w[c] / 2.0;
            let cell_cy = row_y[r] + row_h[r] / 2.0;
            let avail_w = col_w[c] * 0.90;
            let avail_h = row_h[r] * 0.82;
            let scale = (avail_w / bbox_w).min(avail_h / bbox_h).min(2.4);

            for (k, &ci) in order.iter().enumerate() {
                let (lx, ly) = raw_pos[k];
                let px = cell_cx + (lx - mid_x) * scale;
                let py = cell_cy + (ly - mid_y) * scale;
                let rr = (local_radii[k] * scale).max(1.0);
                let s = &sats[ci];
                let col = palette_color(cfg.palette, s.cat);
                let hx = hex6(col);
                let opacity = match s.shade {
                    Some(v) if has_shade => 0.40 + ((v - shade_min) / shade_span).clamp(0.0, 1.0) * 0.52,
                    _ => 0.85,
                };
                push_b(&mut buf, b"<g data-idx=\"");
                push_i(&mut buf, ci as i32);
                push_b(&mut buf, b"\">");
                draw_marker(&mut buf, s.sym, px.round() as i32, py.round() as i32, rr, &hx, &bg_hex, 1.1, opacity);
                push_b(&mut buf, b"</g>");
            }
        }
    }

    push_b(&mut buf, b"</svg>");
    let svg = unsafe { String::from_utf8_unchecked(buf) };

    let mut slots: Vec<HoverSlot> = Vec::with_capacity(n);
    for s in sats.iter() {
        let mut slot = HoverSlot::new(s.name.to_string())
            .kv("Region", region_order[s.region].clone())
            .kv("Orbit", orbit_order[s.orbit].clone())
            .kv("Category", cat_order[s.cat].clone())
            .kv("Class", cap_label(s.sym))
            .kv("Mass", format!("{:.0} kg", s.mass));
        if let Some(v) = s.shade {
            slot = slot.kv("Launch", format!("{:.0}", v));
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
        categories: &'a [String],
        categories2: &'a [String],
        symbols: &'a [String],
        values: &'a [f64],
        color_values: &'a [f64],
    ) -> CirclePackConfig<'a> {
        CirclePackConfig {
            title: "Test",
            labels,
            parents,
            categories,
            categories2,
            symbols,
            values,
            color_values,
            width: 1200,
            height: 800,
            ..CirclePackConfig::default()
        }
    }

    fn synth(n: usize) -> (Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<f64>, Vec<f64>) {
        let regions = ["U.S.", "China", "Russia"];
        let orbits = ["Low Earth Orbit", "Geosynchronous Orbit"];
        let cats = ["Communications", "Research", "Navigation"];
        let syms = ["circle", "star", "diamond"];
        let mut labels = Vec::with_capacity(n);
        let mut parents = Vec::with_capacity(n);
        let mut categories = Vec::with_capacity(n);
        let mut categories2 = Vec::with_capacity(n);
        let mut symbols = Vec::with_capacity(n);
        let mut values = Vec::with_capacity(n);
        let mut color_values = Vec::with_capacity(n);
        for i in 0..n {
            labels.push(format!("Sat-{i}"));
            parents.push(regions[i % regions.len()].to_string());
            categories.push(orbits[i % orbits.len()].to_string());
            categories2.push(cats[i % cats.len()].to_string());
            symbols.push(syms[i % syms.len()].to_string());
            values.push(((i % 50) + 1) as f64 * 40.0);
            color_values.push(1980.0 + (i % 45) as f64);
        }
        (labels, parents, categories, categories2, symbols, values, color_values)
    }

    #[test]
    #[ignore]
    fn write_preview_asset() {
        use crate::plot::chart_demo_registry::{iter_entries, render_demo_html};
        for entry in iter_entries() {
            if !entry.file.replace('\\', "/").ends_with("circle_pack/matrix.rs") {
                continue;
            }
            let html = render_demo_html(entry).expect("demo html");
            std::fs::write("docs/previews/circle_pack-matrix.html", html).unwrap();
        }
    }

    #[test]
    fn renders_one_hoverable_mark_per_point() {
        let (labels, parents, categories, categories2, symbols, values, color_values) = synth(90);
        let html = render(&cfg(&labels, &parents, &categories, &categories2, &symbols, &values, &color_values));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<g data-idx=\"").count(), 90);
    }

    #[test]
    fn draws_both_axes_as_real_category_labels() {
        let (labels, parents, categories, categories2, symbols, values, color_values) = synth(30);
        let html = render(&cfg(&labels, &parents, &categories, &categories2, &symbols, &values, &color_values));
        assert!(html.contains("U.S."));
        assert!(html.contains("China"));
        assert!(html.contains("Russia"));
        assert!(html.contains("Low Earth Orbit"));
        assert!(html.contains("Geosynchronous Orbit"));
    }

    #[test]
    fn renders_multiple_marker_shapes_not_just_circles() {
        let (labels, parents, categories, categories2, symbols, values, color_values) = synth(60);
        let html = render(&cfg(&labels, &parents, &categories, &categories2, &symbols, &values, &color_values));
        assert!(html.contains("<polygon"));
    }

    #[test]
    fn never_defaults_to_a_white_page_background() {
        let (labels, parents, categories, categories2, symbols, values, color_values) = synth(20);
        let html = render(&cfg(&labels, &parents, &categories, &categories2, &symbols, &values, &color_values));
        assert!(html.contains("fill=\"#0b1220\""));
        assert!(!html.contains("fill=\"#ffffff\""));
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let empty_s: Vec<String> = vec![];
        let empty_v: Vec<f64> = vec![];
        let html = render(&cfg(&empty_s, &empty_s, &empty_s, &empty_s, &empty_s, &empty_v, &empty_v));
        assert!(html.is_empty());
    }

    #[test]
    fn perf_rendering_a_few_hundred_satellites_stays_fast() {
        let (labels, parents, categories, categories2, symbols, values, color_values) = synth(600);
        let c = cfg(&labels, &parents, &categories, &categories2, &symbols, &values, &color_values);
        let start = std::time::Instant::now();
        let html = render(&c);
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 1500, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
