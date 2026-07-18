use crate::plot::statistical::bar::build_series;
use crate::plot::statistical::{render_bar3d_blocks_html, BarConfig, BarVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("labels=[\"Q1\",\"Q2\",\"Q3\",\"Q4\"], values=[24,38,17,42], series=[[24,38,17,42],[18,29,33,21],[12,15,28,30],[20,25,18,22]], series_names=[\"A\",\"B\",\"C\",\"D\"], offset_groups=[\"G1\",\"G1\",\"G2\",\"G2\"], widths=[2.0,1.5,1.0,1.2], super_categories=[\"H1\",\"H1\",\"H2\",\"H2\"]")]
#[crate::params(paramsList["title","x","y","z","labels","values","series","series_names","color_hex","palette","bg_color","variant","offset_groups","widths","super_categories","units_per_icon","max_icons_per_column","scene","orientation3d","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("bar3d", "bar_3d", "bar3d_chart", "bar3d_family", "bars3d")]
#[crate::sera_builder]
pub fn build_bar3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let cl = o.color_labels.clone().unwrap_or_default();
    let bg_str = o.bg_str();
    let variant = BarVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let env = o.scene.as_deref().unwrap_or("default");

    let legacy_z = a.z.clone().unwrap_or_default();
    let labels = a
        .labels
        .clone()
        .unwrap_or_else(|| (0..legacy_z.len()).map(|i| format!("C{}", i + 1)).collect());
    let values = a.values.clone().unwrap_or(legacy_z);
    let category_labels = labels.clone();
    let series = build_series(&a, &o, &category_labels);
    let palette = o.pal();
    let offset_groups = o.offset_groups.clone().unwrap_or_default();
    let widths = o.widths.clone().unwrap_or_default();
    let super_categories = o.super_categories.clone().unwrap_or_default();

    let cfg = BarConfig {
        variant,
        labels: &labels,
        values: &values,
        category_labels: &category_labels,
        series: &series,
        palette: &palette,
        offset_groups: &offset_groups,
        widths: &widths,
        super_categories: &super_categories,
        units_per_icon: o.units_per_icon.unwrap_or(1.0),
        max_icons_per_column: o.max_icons_per_column.unwrap_or(10),
        ..BarConfig::default()
    };

    let bg_default = if env == "default" && bg_str.is_none() {
        Some("#090d18")
    } else {
        bg_str.as_deref()
    };

    let html = render_bar3d_blocks_html(
        title,
        &cfg,
        (&o.xl(), &o.yl(), &o.zl()),
        &cl,
        o.w(900),
        o.h(560),
        bg_default,
        env,
    );
    let html = apply_bg3d(html, &o);
    let wants_colorbar = env == "terrain";
    if wants_colorbar && o.colorbar_position.is_none() {
        crate::apply_colorbar(html, "right")
    } else {
        html
    }
}
