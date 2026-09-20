use crate::plot::statistical::bar::{height_ratio_3d, layout_3d, Bar3DBlock, BarConfig};

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
    let view = BlockView { height_ratio: height_ratio_3d(cfg.variant), ..BlockView::default() };
    render_blocks3d_view_html(title, &layout_3d(cfg), &view, axis_labels, color_labels, w, h, bg_color, scene)
}

pub struct BlockView<'a> {
    pub height_ratio: f64,
    pub cmap: &'a str,
    pub uniform: bool,
}

impl Default for BlockView<'_> {
    fn default() -> Self {
        Self { height_ratio: 1.0, cmap: "", uniform: false }
    }
}

pub fn render_blocks3d_html(
    title: &str,
    blocks: &[Bar3DBlock],
    axis_labels: (&str, &str, &str),
    color_labels: &[String],
    w: i32,
    h: i32,
    bg_color: Option<&str>,
    scene: &str,
) -> String {
    render_blocks3d_view_html(title, blocks, &BlockView::default(), axis_labels, color_labels, w, h, bg_color, scene)
}

pub fn render_blocks3d_view_html(
    title: &str,
    blocks: &[Bar3DBlock],
    view: &BlockView,
    axis_labels: (&str, &str, &str),
    color_labels: &[String],
    w: i32,
    h: i32,
    bg_color: Option<&str>,
    scene: &str,
) -> String {
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
    extra_js.push(']');
    if blocks.iter().any(|b| b.tone.is_some()) {
        extra_js.push_str(",BCT=[");
        for (i, b) in blocks.iter().enumerate() {
            if i > 0 {
                extra_js.push(',');
            }
            extra_js.push_str(&format!("{:.4}", b.tone.unwrap_or(-1.0)));
        }
        extra_js.push(']');
    }
    extra_js.push_str(&format!(";var BZK={:.3},BZM=1.6;", view.height_ratio));
    if !view.cmap.is_empty() {
        extra_js.push_str(&format!("CMAP='{}';", view.cmap));
    }
    if view.uniform {
        extra_js.push_str("var BUF=1;");
    }

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

#[cfg(test)]
mod tests {
    use crate::plot::build_bar3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::BarVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/bar/", BarVariant::keys_and_aliases(), BarVariant::default_key())
    }

    #[test]
    fn every_bar_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_bar3d_chart, &demos(), BarVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_bar_variant() {
        twin::check_planes(build_bar3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_bar_variant() {
        twin::check_scenes(build_bar3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_bar_variant() {
        twin::check_themes(build_bar3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_scene_plane_and_theme_axes_on_the_twin_family() {
        twin::check_axes("bar_3d", BarVariant::all().len());
    }

    #[test]
    fn sort_order_reorders_the_columns_like_the_2d_chart() {
        use crate::plot::statistical::bar::{layout_3d, BarConfig};
        let labels: Vec<String> = ["A", "B", "C", "D"].iter().map(|s| s.to_string()).collect();
        let values = [10.0, 40.0, 20.0, 30.0];
        for (order, expected) in [
            ("none", vec![10.0, 40.0, 20.0, 30.0]),
            ("desc", vec![40.0, 30.0, 20.0, 10.0]),
            ("asc", vec![10.0, 20.0, 30.0, 40.0]),
        ] {
            let cfg = BarConfig {
                labels: &labels,
                values: &values,
                sort_order: order,
                ..BarConfig::default()
            };
            let tops: Vec<f64> = layout_3d(&cfg).iter().map(|b| b.z1).collect();
            assert_eq!(tops, expected, "sort_order={order}");
        }
    }

    #[test]
    fn the_public_builder_forwards_sort_order_to_single_series_variants() {
        let json = r#"{"title":"t","labels":["A","B","C","D"],"values":[10,40,20,30],"sort_order":"desc"}"#;
        let html = build_bar3d_chart(json);
        assert!(
            html.contains("BZ1=[40.0000,30.0000,20.0000,10.0000]"),
            "desc order must reach the block heights"
        );
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("bar3d", build_bar3d_chart, &demos(), BarVariant::default_key());
    }
}
