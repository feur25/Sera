use super::common::{prepare, open_svg, draw_axes, dot, label_left, label_right, finalize};
use super::config::SlopeConfig;
use crate::plot::statistical::common::{palette_color, push_b, push_i, escape_xml, hex6};

pub const DEMO_KWARGS: &str = "labels=[\"A\",\"B\",\"C\",\"D\",\"E\"], left=[20,35,15,42,28], right=[35,28,40,55,22]";
fn ranks_desc(vals: &[f64]) -> Vec<usize> {
    let n = vals.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|a, b| vals[*b].partial_cmp(&vals[*a]).unwrap_or(std::cmp::Ordering::Equal));
    let mut rank = vec![0usize; n];
    for (r, i) in idx.iter().enumerate() { rank[*i] = r + 1; }
    rank
}

pub fn render(cfg: &SlopeConfig) -> String {
    let p = match prepare(cfg) { Some(v) => v, None => return String::new() };
    let rl = ranks_desc(&p.values_left);
    let rr = ranks_desc(&p.values_right);
    let n = p.n as i32;
    let l = &p.layout;
    let rank_to_y = |r: usize| -> i32 {
        if n <= 1 { return l.pad_t + l.plot_h / 2; }
        l.pad_t + ((r as f64 - 1.0) * l.plot_h as f64 / (n as f64 - 1.0)) as i32
    };

    let mut b = Vec::<u8>::with_capacity(p.n * 220 + 1024);
    open_svg(&mut b, cfg);
    draw_axes(&mut b, cfg, &p);

    for i in 0..p.n {
        let y1 = rank_to_y(rl[i]);
        let y2 = rank_to_y(rr[i]);
        let color = palette_color(cfg.palette, i);
        let hx = hex6(color);
        let mid_x = (l.x_left + l.x_right) / 2;
        let dx = (l.x_right - l.x_left) as f64 * 0.35;
        push_b(&mut b, b"<path data-idx=\""); push_i(&mut b, i as i32);
        push_b(&mut b, b"\" data-lbl=\""); escape_xml(&mut b, &p.labels[i]);
        push_b(&mut b, b"\" d=\"M ");
        push_i(&mut b, l.x_left); b.push(b' '); push_i(&mut b, y1);
        push_b(&mut b, b" C ");
        push_i(&mut b, l.x_left + dx as i32); b.push(b' '); push_i(&mut b, y1);
        b.push(b' ');
        push_i(&mut b, l.x_right - dx as i32); b.push(b' '); push_i(&mut b, y2);
        b.push(b' ');
        push_i(&mut b, l.x_right); b.push(b' '); push_i(&mut b, y2);
        push_b(&mut b, b"\" fill=\"none\" stroke=\"#"); b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"2.6\" stroke-linecap=\"round\" opacity=\"0.85\"/>");
        let mut prefixed = Vec::with_capacity(7);
        prefixed.push(b'#');
        prefixed.extend_from_slice(&hx);
        dot(&mut b, l.x_left, y1, &prefixed, 5.0);
        dot(&mut b, l.x_right, y2, &prefixed, 5.0);
        let _ = mid_x;
        if cfg.show_text {
            label_left(&mut b, l.x_left, y1, &p.labels[i], rl[i] as f64);
            label_right(&mut b, l.x_right, y2, &p.labels[i], rr[i] as f64);
        }
    }
    finalize(b, cfg)
}


