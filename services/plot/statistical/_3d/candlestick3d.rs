use crate::html::js_3d::render_3d_html;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::candlestick::layout3d;
use crate::plot::statistical::{CandlestickConfig, CandlestickVariant};
use crate::plot::{apply_bg3d, parse_all};

pub fn render_candlestick3d_html(
    title: &str,
    x: &[f64],
    y: &[f64],
    z: &[f64],
    axis_labels: (&str, &str, &str),
    colors: &[f64],
    color_labels: &[String],
    w: i32,
    h: i32,
    bg_color: Option<&str>,
    scene: &str,
) -> String {
    render_3d_html(
        10,
        title,
        x,
        y,
        z,
        axis_labels,
        colors,
        color_labels,
        w,
        h,
        bg_color,
        scene,
    )
}

#[crate::chart_demo("labels=[\"D1\",\"D2\",\"D3\"], open=[10,12,11], high=[14,15,13], low=[9,10,9], close=[12,13,12]")]
#[crate::params(paramsList["title","labels","open","high","low","close","volume","variant","x_label","y_label","z_label","bg_color","scene","orientation3d","theme","zone","max_points","width","height"])]
#[crate::sera_alias("candlestick3d", "candlestick_3d", "candlestick3d_chart", "ohlc3d")]
#[crate::sera_builder]
pub fn build_candlestick3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let open = a.open.unwrap_or_default();
    let high = a.high.unwrap_or_default();
    let low = a.low.unwrap_or_default();
    let close = a.close.unwrap_or_default();
    let volume = a.volume.unwrap_or_default();
    let variant = CandlestickVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = CandlestickConfig {
        title,
        variant,
        labels: &labels,
        open: &open,
        high: &high,
        low: &low,
        close: &close,
        volume: &volume,
        ..CandlestickConfig::default()
    };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() {
        Some("#090d18")
    } else {
        bg_str.as_deref()
    };
    let view = BlockView::new(layout3d::HEIGHT_RATIO, layout3d::COLORMAP).with_zone(o.zone.as_deref());
    let (xl, yl) = (o.xl(), o.yl());
    let html = render_blocks3d_view_html(
        title,
        &layout3d::layout_3d(&cfg, &Budget::new(o.max_points)),
        &view,
        (if xl.is_empty() { "Bar" } else { &xl }, &yl, &o.zl()),
        &[],
        o.w(900),
        o.h(560),
        bg_default,
        env,
    );
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 77,
        name: "candlestick_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use crate::plot::build_candlestick3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::CandlestickVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos(
            "statistical/candlestick/",
            CandlestickVariant::keys_and_aliases(),
            CandlestickVariant::default_key(),
        )
    }

    #[test]
    fn every_candlestick_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_candlestick3d_chart, &demos(), CandlestickVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_candlestick_variant() {
        twin::check_planes(build_candlestick3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_candlestick_variant() {
        twin::check_scenes(build_candlestick3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_candlestick_variant() {
        twin::check_themes(build_candlestick3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_candlestick_variants_and_the_view_axes() {
        twin::check_axes("candlestick3d", CandlestickVariant::all().len());
    }

    #[test]
    fn the_legacy_three_bar_call_still_renders() {
        let json = r#"{"title":"t","labels":["D1","D2","D3"],"open":[10,12,11],"high":[14,15,13],"low":[9,10,9],"close":[12,13,12]}"#;
        twin::assert_blocks(&build_candlestick3d_chart(json), "legacy candlestick3d call");
    }

    #[test]
    fn every_candlestick_variant_honours_an_explicit_zone() {
        twin::check_zone(build_candlestick3d_chart, &demos());
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("candlestick3d", build_candlestick3d_chart, &demos(), CandlestickVariant::default_key());
    }
}
