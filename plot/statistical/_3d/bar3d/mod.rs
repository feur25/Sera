use crate::plot::statistical::bar::{layout_3d, BarConfig};

pub fn render_bar3d_blocks_html(
    title: &str,
    cfg: &BarConfig,
    axis_labels: (&str, &str, &str),
    color_labels: &[String],
    w: i32,
    h: i32,
    bg_color: Option<&str>,
    scene: &str,
) -> String {
    let blocks = layout_3d(cfg);
    let mut extra_js = String::with_capacity(blocks.len() * 48 + 32);
    extra_js.push_str("var BN=");
    extra_js.push_str(&blocks.len().to_string());
    extra_js.push_str(",BX=[");
    for (i, b) in blocks.iter().enumerate() {
        if i > 0 {
            extra_js.push(',');
        }
        extra_js.push_str(&format!("{:.4}", b.cx));
    }
    extra_js.push_str("],BY=[");
    for (i, b) in blocks.iter().enumerate() {
        if i > 0 {
            extra_js.push(',');
        }
        extra_js.push_str(&format!("{:.4}", b.cy));
    }
    extra_js.push_str("],BZ0=[");
    for (i, b) in blocks.iter().enumerate() {
        if i > 0 {
            extra_js.push(',');
        }
        extra_js.push_str(&format!("{:.4}", b.z0));
    }
    extra_js.push_str("],BZ1=[");
    for (i, b) in blocks.iter().enumerate() {
        if i > 0 {
            extra_js.push(',');
        }
        extra_js.push_str(&format!("{:.4}", b.z1));
    }
    extra_js.push_str("],BHW=[");
    for (i, b) in blocks.iter().enumerate() {
        if i > 0 {
            extra_js.push(',');
        }
        extra_js.push_str(&format!("{:.4}", b.hw));
    }
    extra_js.push_str("],BHD=[");
    for (i, b) in blocks.iter().enumerate() {
        if i > 0 {
            extra_js.push(',');
        }
        extra_js.push_str(&format!("{:.4}", b.hd));
    }
    extra_js.push_str("],BCI=[");
    for (i, b) in blocks.iter().enumerate() {
        if i > 0 {
            extra_js.push(',');
        }
        extra_js.push_str(&b.ci.to_string());
    }
    extra_js.push_str("];");

    let (x, y, z): (Vec<f64>, Vec<f64>, Vec<f64>) = blocks
        .iter()
        .map(|b| (b.cx, b.cy, b.z1))
        .fold(
            (Vec::new(), Vec::new(), Vec::new()),
            |(mut xs, mut ys, mut zs), (x, y, z)| {
                xs.push(x);
                ys.push(y);
                zs.push(z);
                (xs, ys, zs)
            },
        );
    let x = if x.is_empty() { vec![0.0] } else { x };
    let y = if y.is_empty() { vec![0.0] } else { y };
    let z = if z.is_empty() { vec![0.0] } else { z };

    crate::html::js_3d::render_3d_html_impl(
        1,
        title,
        &x,
        &y,
        &z,
        axis_labels,
        &[],
        color_labels,
        w,
        h,
        bg_color,
        scene,
        extra_js.as_bytes(),
    )
}
