use super::budget;
use super::zone::{self, Fit};
use crate::plot::statistical::bar::{fit_3d, height_ratio_3d, layout_3d, Bar3DBlock, BarConfig};

pub fn render_bar3d_blocks_html(
    title: &str,
    cfg: &BarConfig,
    axis_labels: (&str, &str, &str),
    color_labels: &[String],
    w: i32,
    h: i32,
    bg_color: Option<&str>,
    scene: &str,
    zone: Option<&[f64]>,
) -> String {
    let view = BlockView::new(height_ratio_3d(cfg.variant), "").with_zone(zone).with_fit(fit_3d(cfg.variant));
    render_blocks3d_view_html(title, &layout_3d(cfg), &view, axis_labels, color_labels, w, h, bg_color, scene)
}

pub struct BlockView<'a> {
    pub height_ratio: f64,
    pub cmap: &'a str,
    pub zone: Option<[f64; 3]>,
    pub fit: Fit,
}

impl<'a> BlockView<'a> {
    pub fn new(height_ratio: f64, cmap: &'a str) -> Self {
        Self { height_ratio, cmap, zone: None, fit: Fit::Uniform }
    }

    pub fn with_zone(mut self, proportions: Option<&[f64]>) -> Self {
        self.zone = proportions.and_then(|p| <[f64; 3]>::try_from(p).ok());
        self
    }

    pub fn with_fit(mut self, fit: Fit) -> Self {
        self.fit = fit;
        self
    }
}

impl Default for BlockView<'_> {
    fn default() -> Self {
        Self::new(1.0, "")
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

fn column(blocks: &[Bar3DBlock], value: impl Fn(&Bar3DBlock) -> String) -> String {
    blocks.iter().map(value).collect::<Vec<_>>().join(",")
}

fn block_script(blocks: &[Bar3DBlock], view: &BlockView) -> String {
    let coords = |pick: fn(&Bar3DBlock) -> f64| column(blocks, move |b| format!("{:.4}", pick(b)));
    let mut js = format!(
        "var BN={},BX=[{}],BY=[{}],BZ0=[{}],BZ1=[{}],BHW=[{}],BHD=[{}],BCI=[{}]",
        blocks.len(),
        coords(|b| b.cx),
        coords(|b| b.cy),
        coords(|b| b.z0),
        coords(|b| b.z1),
        coords(|b| b.hw),
        coords(|b| b.hd),
        column(blocks, |b| b.ci.to_string()),
    );
    if blocks.iter().any(|b| b.tone.is_some()) {
        js.push_str(&format!(",BCT=[{}]", column(blocks, |b| format!("{:.4}", b.tone.unwrap_or(-1.0)))));
    }
    if blocks.iter().any(|b| b.end.is_some()) {
        let ends = |pick: fn((f64, f64)) -> f64, own: fn(&Bar3DBlock) -> f64| {
            column(blocks, move |b| format!("{:.4}", b.end.map(pick).unwrap_or_else(|| own(b))))
        };
        js.push_str(&format!(",BEZ0=[{}],BEZ1=[{}]", ends(|e| e.0, |b| b.z0), ends(|e| e.1, |b| b.z1)));
    }
    js.push_str(&format!(";var BFIT={};", zone::fit(blocks, view.height_ratio, view.zone, view.fit).to_js()));
    if !view.cmap.is_empty() {
        js.push_str(&format!("CMAP='{}';", view.cmap));
    }
    js
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
    let kept = budget::thin(budget::sound(blocks), budget::HARD_BLOCKS);
    crate::html::js_3d::render_3d_html_impl(
        1,
        title,
        &[0.0],
        &[0.0],
        &[0.0],
        axis_labels,
        &[],
        color_labels,
        w,
        h,
        bg_color,
        scene,
        block_script(&kept, view).as_bytes(),
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
    fn every_bar_variant_honours_an_explicit_zone() {
        twin::check_zone(build_bar3d_chart, &demos());
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("bar3d", build_bar3d_chart, &demos(), BarVariant::default_key());
    }
}
