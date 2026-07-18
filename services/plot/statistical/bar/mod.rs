use crate::plot::{apply, apply_h, parse_all};
pub mod basic;
pub mod block3d;
pub mod config;
pub mod deluxe;
pub mod grouped;
pub mod grouped_stacked;
pub mod marimekko;
pub mod multicategory;
pub mod pictogram;
pub mod prism;
pub mod relative;
pub mod variant;

pub use block3d::Bar3DBlock;
pub use config::BarConfig;
pub use variant::BarVariant;

pub fn layout_3d(cfg: &BarConfig) -> Vec<Bar3DBlock> {
    use BarVariant::*;
    match cfg.variant {
        Basic => basic::layout_3d(cfg),
        Horizontal => basic::layout_3d_horizontal(cfg),
        Grouped => grouped::layout_3d(cfg, false),
        Stacked => grouped::layout_3d(cfg, true),
        Relative => relative::layout_3d(cfg),
        GroupedStacked => grouped_stacked::layout_3d(cfg),
        Marimekko => marimekko::layout_3d(cfg),
        Multicategory => multicategory::layout_3d(cfg),
        Pictogram => pictogram::layout_3d(cfg),
    }
}

pub fn render_bar_html(cfg: &BarConfig) -> String {
    use crate::plot::statistical::theme::ChartTheme;
    match cfg.theme {
        ChartTheme::Deluxe => return deluxe::render(cfg, cfg.orientation),
        ChartTheme::Prism => return prism::render(cfg),
        _ => {}
    }
    use BarVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg, cfg.orientation),
        Horizontal => basic::render(cfg, b'h'),
        Grouped => grouped::render(cfg, false),
        Stacked => grouped::render(cfg, true),
        Relative => relative::render(cfg),
        GroupedStacked => grouped_stacked::render(cfg),
        Marimekko => marimekko::render(cfg),
        Pictogram => pictogram::render(cfg),
        Multicategory => multicategory::render(cfg),
    }
}

pub use build as build_bar;

pub fn build_series(
    a: &crate::plot::chart_input::ChartArgs,
    o: &crate::plot::chart_input::ChartOpts,
    category_labels: &[String],
) -> Vec<(String, Vec<f64>)> {
    let sn = o.series_names.clone().unwrap_or_default();
    let n_cats = category_labels.len();
    if let Some(s) = a.series.as_ref() {
        s.iter()
            .enumerate()
            .map(|(si, vals)| {
                (
                    sn.get(si)
                        .cloned()
                        .unwrap_or_else(|| format!("S{}", si + 1)),
                    vals.clone(),
                )
            })
            .collect()
    } else if !sn.is_empty() && n_cats > 0 {
        let flat = a.values.clone().unwrap_or_default();
        sn.iter()
            .enumerate()
            .map(|(si, name)| {
                let vals: Vec<f64> = (0..n_cats)
                    .map(|ci| flat.get(si * n_cats + ci).copied().unwrap_or(0.0))
                    .collect();
                (name.clone(), vals)
            })
            .collect()
    } else {
        Vec::new()
    }
}

#[crate::sera_alias(
    "bar",
    "bar_chart",
    "bars",
    "bar_unified",
    "bars_unified",
    "bar_family"
)]
#[crate::sera_builder("build_bar")]
pub fn build(input: &str) -> String {
    use crate::plot::statistical::{render_bar_html, BarConfig, BarVariant, ChartTheme};
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let variant = BarVariant::from_str(o.variant.as_deref().unwrap_or("basic"));

    let labels = a.labels.clone().unwrap_or_default();
    let values = a.values.clone().unwrap_or_default();
    let category_labels = a.labels.clone().unwrap_or_default();
    let groups = o.color_groups.clone().unwrap_or_default();
    let hover = o.hj();
    let palette = o.pal();
    let offset_groups = o.offset_groups.clone().unwrap_or_default();
    let widths = o.widths.clone().unwrap_or_default();
    let super_categories = o.super_categories.clone().unwrap_or_default();
    let unit_desc = o.unit_description.clone().unwrap_or_default();
    let xl = o.xl();
    let yl = o.yl();
    let srt = o.srt();
    let lp = o.lp();

    let series: Vec<(String, Vec<f64>)> = build_series(&a, &o, &category_labels);

    let cfg = BarConfig {
        variant,
        title,
        x_label: &xl,
        y_label: &yl,
        width: o.w(900),
        height: o.h(480),
        gridlines: o.grid(),
        sort_order: &srt,
        legend_position: &lp,
        hover: &hover,
        palette: &palette,
        labels: &labels,
        values: &values,
        color_hex: o.color_hex.unwrap_or(0),
        color_groups: &groups,
        category_labels: &category_labels,
        series: &series,
        offset_groups: &offset_groups,
        widths: &widths,
        super_categories: &super_categories,
        icon_size: o.icon_size.unwrap_or(24),
        max_icons_per_column: o.max_icons_per_column.unwrap_or(10),
        units_per_icon: o.units_per_icon.unwrap_or(1.0),
        unit_description: &unit_desc,
        show_text: o.show_values.or(o.show_text).unwrap_or(false),
        corner_radius: o.corner_radius.unwrap_or(0),
        bar_gap: o.bar_gap.unwrap_or(0.2),
        bargroup_gap: o.bargroup_gap.unwrap_or(0.1),
        orientation: o.orient_byte(),
        theme: ChartTheme::from_str(o.theme.as_deref().unwrap_or("none")),
    };
    let html = render_bar_html(&cfg);
    use crate::plot::statistical::BarVariant::*;
    let native = matches!(variant, Basic | Horizontal | Grouped | Stacked);
    if native {
        apply_h(html, &o)
    } else {
        apply(html, &o)
    }
}
