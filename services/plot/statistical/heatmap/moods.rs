use super::config::HeatmapConfig;
use crate::html::hover::{html_id, html_prefix, html_suffix, slots_to_json};
use crate::plot::statistical::common::{escape_xml, hex6, palette_color, push_b, push_f2, push_i, truncate};

#[crate::chart_demo(
    "title=\"My Music Moods\", labels=[\"Rock::Ironclad Sky::Silver Gravity\",\"Rock::Ironclad Sky::Hollow Bloom\",\"Rock::Ironclad Sky::Distant Ember\",\"Rock::Voltage Parade::Slow Gravity\",\"Rock::Voltage Parade::Wild Gravity\",\"Rock::Rust Horizon::Distant Bloom\",\"Rock::Rust Horizon::Static Season\",\"Rock::Rust Horizon::Slow Rooms\",\"Alternative::Paper Static::Wild Ember\",\"Alternative::Paper Static::Slow Signals\",\"Alternative::Paper Static::Velvet Ember\",\"Alternative::Faded Compass::Velvet Orbit\",\"Alternative::Faded Compass::Slow Ember\",\"Electronic::Neon Cartograph::Static Rooms\",\"Electronic::Neon Cartograph::Velvet Gravity\",\"Electronic::Neon Cartograph::Distant Tides\",\"Electronic::Glass Circuit::Quiet Gravity\",\"Electronic::Glass Circuit::Midnight Season\",\"Jazz::Blue Meridian Trio::Slow Echoes\",\"Jazz::Blue Meridian Trio::Velvet Ember\",\"Jazz::Blue Meridian Trio::Slow Signals\",\"Jazz::Slow Copper::Hollow Season\",\"Jazz::Slow Copper::Broken Ember\",\"Classical::Amara Voss::Amber Bloom\",\"Classical::Amara Voss::Slow Orbit\",\"Classical::Kestrel Quartet::Slow Coast\",\"Classical::Kestrel Quartet::Quiet Coast\",\"Classical::Kestrel Quartet::Velvet Gravity\"], col_labels=[\"Angry\",\"Passionate\",\"Excited\",\"Happy\",\"Curious\",\"Content\",\"Determined\",\"Nostalgic\",\"Melancholy\",\"Anxious\",\"Sleepy\",\"Restless\"], values=[0.891,0.941,0.615,0.129,0.218,0.119,0.8,0.053,0.16,0.158,0.1,0.601,0.771,0.86,0.585,0.043,0.225,0.182,0.71,0.102,0.225,0.042,0.176,0.667,0.947,0.943,0.72,0.239,0.048,0.042,0.747,0.167,0.164,0.206,0.165,0.592,0.759,0.954,0.528,0.181,0.159,0.184,0.782,0.096,0.043,0.052,0.154,0.648,0.858,0.774,0.713,0.136,0.24,0.187,0.811,0.11,0.214,0.213,0.116,0.486,0.786,0.862,0.624,0.058,0.063,0.137,0.699,0.191,0.14,0.09,0.191,0.514,0.924,0.783,0.543,0.206,0.174,0.165,0.726,0.096,0.153,0.078,0.196,0.493,0.912,0.892,0.614,0.094,0.144,0.119,0.687,0.066,0.222,0.216,0.133,0.614,0.083,0.615,0.104,0.11,0.53,0.167,0.2,0.219,0.573,0.708,0.23,0.725,0.145,0.482,0.22,0.234,0.46,0.161,0.143,0.153,0.483,0.761,0.135,0.783,0.078,0.563,0.079,0.104,0.551,0.142,0.146,0.123,0.511,0.676,0.211,0.782,0.084,0.447,0.092,0.243,0.603,0.08,0.234,0.192,0.518,0.577,0.154,0.661,0.156,0.436,0.125,0.21,0.46,0.114,0.059,0.057,0.652,0.608,0.234,0.769,0.216,0.193,0.784,0.322,0.595,0.173,0.154,0.24,0.339,0.543,0.12,0.695,0.148,0.209,0.899,0.511,0.547,0.148,0.139,0.102,0.094,0.57,0.157,0.757,0.156,0.2,0.934,0.34,0.499,0.181,0.064,0.125,0.123,0.556,0.203,0.646,0.165,0.177,0.897,0.37,0.652,0.094,0.142,0.057,0.166,0.462,0.219,0.782,0.125,0.114,0.835,0.504,0.621,0.057,0.177,0.055,0.056,0.601,0.112,0.643,0.128,0.104,0.219,0.113,0.748,0.779,0.153,0.788,0.473,0.233,0.233,0.057,0.143,0.203,0.138,0.132,0.737,0.781,0.049,0.64,0.388,0.099,0.242,0.089,0.104,0.095,0.186,0.224,0.673,0.683,0.158,0.672,0.45,0.165,0.391,0.231,0.213,0.067,0.199,0.052,0.845,0.669,0.239,0.873,0.518,0.051,0.398,0.066,0.322,0.19,0.044,0.176,0.677,0.809,0.121,0.713,0.398,0.307,0.309,0.179,0.081,0.115,0.107,0.091,0.106,0.615,0.36,0.806,0.7,0.058,0.556,0.224,0.193,0.168,0.12,0.198,0.044,0.6,0.422,0.792,0.801,0.14,0.667,0.207,0.08,0.155,0.226,0.211,0.221,0.582,0.518,0.695,0.799,0.206,0.485,0.237,0.209,0.168,0.122,0.064,0.066,0.481,0.478,0.691,0.803,0.096,0.651,0.063,0.207,0.221,0.127,0.198,0.089,0.645,0.341,0.801,0.82,0.114,0.503,0.065], width=1100, height=760, variant=\"moods\""
)]
pub fn render(cfg: &HeatmapConfig) -> String {
    let n_rows = cfg.row_labels.len();
    let n_cols = cfg.col_labels.len();
    if n_rows == 0 || n_cols == 0 || cfg.flat_matrix.len() < n_rows * n_cols {
        return String::new();
    }
    let data = &cfg.flat_matrix[..n_rows * n_cols];
    let max_v = data.iter().cloned().fold(0.0_f64, f64::max).max(1e-9);

    let parts_of = |row: usize| -> (&str, &str, &str) {
        let label = cfg.row_labels[row].as_str();
        let mut it = label.splitn(3, "::");
        let a = it.next().unwrap_or("");
        let b = it.next().unwrap_or("");
        let c = it.next().unwrap_or("");
        if c.is_empty() && b.is_empty() {
            ("", "", a)
        } else {
            (a, b, c)
        }
    };

    let mut genres: Vec<&str> = Vec::new();
    let mut artists: Vec<&str> = Vec::new();
    for row in 0..n_rows {
        let (g, a, _) = parts_of(row);
        if genres.last() != Some(&g) {
            genres.push(g);
        }
        if artists.last() != Some(&a) {
            artists.push(a);
        }
    }

    let ink: u32 = 0x1a202c;
    let sub: u32 = 0x6b7280;
    let ring_col: u32 = 0xe2e8f0;

    let pad_left = 232i32;
    let pad_top = 92i32;
    let pad_right = 128i32;
    let pad_bottom = 44i32;
    let plot_w = (cfg.width - pad_left - pad_right).max(40);
    let plot_h = (cfg.height - pad_top - pad_bottom).max(40);
    let col_w = plot_w as f64 / n_cols as f64;
    let row_h = plot_h as f64 / n_rows as f64;
    let max_r = (col_w.min(row_h) * 0.42).max(2.0);
    let min_r = 1.2;

    let hid = html_id();
    let mut buf = Vec::<u8>::with_capacity(n_rows * n_cols * 180 + 8192);
    html_prefix(&mut buf, cfg.title, hid);

    push_b(&mut buf, b"<svg xmlns=\"http://www.w3.org/2000/svg\" role=\"group\" width=\"");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\" viewBox=\"0 0 ");
    push_i(&mut buf, cfg.width);
    push_b(&mut buf, b" ");
    push_i(&mut buf, cfg.height);
    push_b(&mut buf, b"\"><rect class=\"sp-bg\" width=\"100%\" height=\"100%\"/>");

    if !cfg.title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, pad_left);
        push_b(&mut buf, b"\" y=\"26\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"17\" font-weight=\"700\" fill=\"#");
        buf.extend_from_slice(&hex6(ink));
        push_b(&mut buf, b"\">");
        escape_xml(&mut buf, cfg.title);
        push_b(&mut buf, b"</text>");
    }

    for (col, lbl) in cfg.col_labels.iter().enumerate() {
        let cx = pad_left as f64 + col as f64 * col_w + col_w / 2.0;
        let cy = pad_top as f64 - 12.0;
        let col_hx = hex6(palette_color(cfg.palette, col));
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy);
        push_b(&mut buf, b"\" text-anchor=\"start\" font-family=\"Arial,sans-serif\" font-size=\"10\" font-weight=\"700\" fill=\"#");
        buf.extend_from_slice(&col_hx);
        push_b(&mut buf, b"\" transform=\"rotate(-42,");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b",");
        push_f2(&mut buf, cy);
        push_b(&mut buf, b")\">");
        escape_xml(&mut buf, lbl);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"<g stroke=\"#");
    buf.extend_from_slice(&hex6(ring_col));
    push_b(&mut buf, b"\" stroke-width=\"0.6\" stroke-dasharray=\"1,3\">");
    for col in 0..=n_cols {
        let x = pad_left as f64 + col as f64 * col_w;
        push_b(&mut buf, b"<line x1=\"");
        push_f2(&mut buf, x);
        push_b(&mut buf, b"\" y1=\"");
        push_i(&mut buf, pad_top);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, x);
        push_b(&mut buf, b"\" y2=\"");
        push_i(&mut buf, pad_top + plot_h);
        push_b(&mut buf, b"\"/>");
    }
    push_b(&mut buf, b"</g>");

    let mut genre_start = 0usize;
    let mut prev_genre = parts_of(0).0;
    let mut prev_artist = parts_of(0).1;
    let mut idx = 0i32;
    for row in 0..n_rows {
        let (genre, artist, album) = parts_of(row);
        let is_new_genre = genre != prev_genre;
        if is_new_genre {
            let band_top = pad_top as f64 + genre_start as f64 * row_h;
            push_b(&mut buf, b"<line x1=\"18\" y1=\"");
            push_f2(&mut buf, band_top);
            push_b(&mut buf, b"\" x2=\"");
            push_i(&mut buf, pad_left + plot_w);
            push_b(&mut buf, b"\" y2=\"");
            push_f2(&mut buf, band_top);
            push_b(&mut buf, b"\" stroke=\"#");
            buf.extend_from_slice(&hex6(ink));
            push_b(&mut buf, b"\" stroke-width=\"1\" stroke-opacity=\"0.15\"/>");
            genre_start = row;
            prev_genre = genre;
        }
        let is_new_artist = artist != prev_artist || is_new_genre;
        if is_new_artist {
            prev_artist = artist;
        }

        let cy = pad_top as f64 + row as f64 * row_h + row_h / 2.0;
        let cb_x = pad_left as f64 - 16.0;
        push_b(&mut buf, b"<rect x=\"");
        push_f2(&mut buf, cb_x);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy - 4.0);
        push_b(&mut buf, b"\" width=\"8\" height=\"8\" rx=\"1.6\" fill=\"none\" stroke=\"#");
        buf.extend_from_slice(&hex6(ring_col));
        push_b(&mut buf, b"\" stroke-width=\"1\"/>");

        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, cb_x - 6.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, cy + 3.0);
        push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#");
        buf.extend_from_slice(&hex6(sub));
        push_b(&mut buf, b"\">");
        escape_xml(&mut buf, truncate(album, 20));
        push_b(&mut buf, b"</text>");

        if is_new_artist {
            push_b(&mut buf, b"<text x=\"");
            push_f2(&mut buf, cb_x - 6.0);
            push_b(&mut buf, b"\" y=\"");
            push_f2(&mut buf, cy - 7.0);
            push_b(&mut buf, b"\" text-anchor=\"end\" font-family=\"Arial,sans-serif\" font-size=\"8.5\" font-weight=\"700\" fill=\"#");
            buf.extend_from_slice(&hex6(ink));
            push_b(&mut buf, b"\">");
            escape_xml(&mut buf, truncate(artist, 22));
            push_b(&mut buf, b"</text>");
        }

        for col in 0..n_cols {
            let v = data[row * n_cols + col];
            let t = (v / max_v).clamp(0.0, 1.0);
            let r = min_r + t.sqrt() * (max_r - min_r);
            let cx = pad_left as f64 + col as f64 * col_w + col_w / 2.0;
            let col_hx = hex6(palette_color(cfg.palette, col));
            push_b(&mut buf, b"<circle data-idx=\"");
            push_i(&mut buf, idx);
            push_b(&mut buf, b"\" data-v=\"");
            push_f2(&mut buf, v);
            push_b(&mut buf, b"\" data-r=\"");
            escape_xml(&mut buf, &format!("{artist} - {album}"));
            push_b(&mut buf, b"\" data-c=\"");
            escape_xml(&mut buf, &cfg.col_labels[col]);
            push_b(&mut buf, b"\" cx=\"");
            push_f2(&mut buf, cx);
            push_b(&mut buf, b"\" cy=\"");
            push_f2(&mut buf, cy);
            push_b(&mut buf, b"\" r=\"");
            push_f2(&mut buf, r);
            push_b(&mut buf, b"\" fill=\"#");
            buf.extend_from_slice(&col_hx);
            push_b(&mut buf, b"\" fill-opacity=\"");
            push_f2(&mut buf, 0.28 + t * 0.64);
            push_b(&mut buf, b"\"/>");
            idx += 1;
        }
    }
    push_b(&mut buf, b"<line x1=\"18\" y1=\"");
    push_i(&mut buf, pad_top + plot_h);
    push_b(&mut buf, b"\" x2=\"");
    push_i(&mut buf, pad_left + plot_w);
    push_b(&mut buf, b"\" y2=\"");
    push_i(&mut buf, pad_top + plot_h);
    push_b(&mut buf, b"\" stroke=\"#");
    buf.extend_from_slice(&hex6(ink));
    push_b(&mut buf, b"\" stroke-width=\"1\" stroke-opacity=\"0.15\"/>");

    let mut gy = pad_top as f64;
    prev_genre = parts_of(0).0;
    genre_start = 0;
    for row in 0..=n_rows {
        let genre = if row < n_rows { parts_of(row).0 } else { "" };
        if genre != prev_genre {
            let band_h = (row - genre_start) as f64 * row_h;
            let mid = gy + band_h / 2.0;
            push_b(&mut buf, b"<text x=\"14\" y=\"");
            push_f2(&mut buf, mid);
            push_b(&mut buf, b"\" text-anchor=\"middle\" font-family=\"Arial,sans-serif\" font-size=\"9.5\" font-weight=\"700\" letter-spacing=\"1\" fill=\"#");
            buf.extend_from_slice(&hex6(sub));
            push_b(&mut buf, b"\" transform=\"rotate(-90,14,");
            push_f2(&mut buf, mid);
            push_b(&mut buf, b")\">");
            escape_xml(&mut buf, prev_genre);
            push_b(&mut buf, b"</text>");
            gy += band_h;
            genre_start = row;
            prev_genre = genre;
        }
    }

    let leg_x = (pad_left + plot_w + 30) as f64;
    let leg_y = pad_top as f64;
    push_b(&mut buf, b"<text x=\"");
    push_f2(&mut buf, leg_x);
    push_b(&mut buf, b"\" y=\"");
    push_f2(&mut buf, leg_y);
    push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" font-weight=\"700\" letter-spacing=\"0.5\" fill=\"#");
    buf.extend_from_slice(&hex6(sub));
    push_b(&mut buf, b"\">INTENSITY</text>");
    let steps: [f64; 3] = [0.25, 0.6, 1.0];
    let mut sy = leg_y + 22.0;
    for &s in steps.iter() {
        let r = min_r + s.sqrt() * (max_r - min_r);
        push_b(&mut buf, b"<circle cx=\"");
        push_f2(&mut buf, leg_x + max_r);
        push_b(&mut buf, b"\" cy=\"");
        push_f2(&mut buf, sy);
        push_b(&mut buf, b"\" r=\"");
        push_f2(&mut buf, r);
        push_b(&mut buf, b"\" fill=\"none\" stroke=\"#");
        buf.extend_from_slice(&hex6(sub));
        push_b(&mut buf, b"\" stroke-width=\"1\"/>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, leg_x + max_r * 2.0 + 8.0);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, sy + 3.0);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" fill=\"#");
        buf.extend_from_slice(&hex6(ink));
        push_b(&mut buf, b"\">");
        escape_xml(&mut buf, &format!("{:.0}%", s * 100.0));
        push_b(&mut buf, b"</text>");
        sy += max_r * 2.0 + 14.0;
    }

    let stats: [(&str, usize); 3] = [("ALBUMS", n_rows), ("ARTISTS", artists.len()), ("GENRES", genres.len())];
    let mut sty = sy + 14.0;
    for (label, val) in stats.iter() {
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, leg_x);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, sty);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"9\" font-weight=\"700\" letter-spacing=\"0.5\" fill=\"#");
        buf.extend_from_slice(&hex6(sub));
        push_b(&mut buf, b"\">");
        push_b(&mut buf, label.as_bytes());
        push_b(&mut buf, b"</text>");
        push_b(&mut buf, b"<text x=\"");
        push_f2(&mut buf, leg_x);
        push_b(&mut buf, b"\" y=\"");
        push_f2(&mut buf, sty + 16.0);
        push_b(&mut buf, b"\" font-family=\"Arial,sans-serif\" font-size=\"14\" font-weight=\"700\" fill=\"#");
        buf.extend_from_slice(&hex6(ink));
        push_b(&mut buf, b"\">");
        push_i(&mut buf, *val as i32);
        push_b(&mut buf, b"</text>");
        sty += 34.0;
    }

    push_b(&mut buf, b"</svg>");
    html_suffix(&mut buf, hid, &slots_to_json(cfg.hover));
    unsafe { String::from_utf8_unchecked(buf) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg<'a>(labels: &'a [String], cols: &'a [String], values: &'a [f64]) -> HeatmapConfig<'a> {
        HeatmapConfig {
            title: "Test",
            row_labels: labels,
            col_labels: cols,
            flat_matrix: values,
            width: 1100,
            height: 760,
            ..HeatmapConfig::default()
        }
    }

    fn synth(n_rows: usize, n_cols: usize) -> (Vec<String>, Vec<String>, Vec<f64>) {
        let genres = ["Rock", "Jazz", "Classical"];
        let labels: Vec<String> = (0..n_rows)
            .map(|i| {
                let genre = genres[i * genres.len() / n_rows.max(1)];
                format!("{genre}::Artist{}::Album{i}", i / 3)
            })
            .collect();
        let cols: Vec<String> = (0..n_cols).map(|i| format!("Mood{i}")).collect();
        let values: Vec<f64> = (0..n_rows * n_cols).map(|i| ((i * 37) % 100) as f64 / 100.0).collect();
        (labels, cols, values)
    }

    #[test]
    #[ignore]
    fn write_preview_asset() {
        use crate::plot::chart_demo_registry::{iter_entries, render_demo_html};
        for entry in iter_entries() {
            if !entry.file.replace('\\', "/").ends_with("heatmap/moods.rs") {
                continue;
            }
            let html = render_demo_html(entry).expect("demo html");
            std::fs::write("docs/previews/heatmap-moods.html", html).unwrap();
        }
    }

    #[test]
    fn renders_one_hoverable_bubble_per_cell_and_a_checkbox_per_row() {
        let (labels, cols, values) = synth(9, 5);
        let html = render(&cfg(&labels, &cols, &values));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<circle data-idx=\"").count(), 45);
        assert_eq!(html.matches("rx=\"1.6\" fill=\"none\"").count(), 9);
    }

    #[test]
    fn draws_one_genre_band_label_per_distinct_genre() {
        let (labels, cols, values) = synth(9, 4);
        let html = render(&cfg(&labels, &cols, &values));
        assert_eq!(html.matches("rotate(-90,14,").count(), 3);
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let empty_s: Vec<String> = vec![];
        let empty_f: Vec<f64> = vec![];
        assert!(render(&cfg(&empty_s, &empty_s, &empty_f)).is_empty());
    }

    #[test]
    fn perf_rendering_a_large_library_stays_fast() {
        let (labels, cols, values) = synth(150, 16);
        let start = std::time::Instant::now();
        let html = render(&cfg(&labels, &cols, &values));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 500, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
