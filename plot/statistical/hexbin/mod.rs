use crate::plot::{apply, parse_all};
pub mod basic;
pub mod common;
pub mod config;
pub mod outlined;
pub mod variant;

pub use config::HexbinConfig;
pub use variant::HexbinVariant;

pub fn render_hexbin_html(cfg: &HexbinConfig) -> String {
    use variant::HexbinVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Outlined => outlined::render(cfg),
    }
}

pub use build as build_hexbin;

#[crate::sera_alias("hexbin", "hexbins", "hexbin_chart", "hexagonal_binning")]
#[crate::sera_builder("build_hexbin")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x_values = a.x.unwrap_or_default();
    let y_values = a.y.unwrap_or_default();
    use crate::plot::statistical::{render_hexbin_html, HexbinConfig, HexbinVariant};
    let hover = o.hj();
    let variant = HexbinVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let colorscale = o.colorscale.clone().unwrap_or_default();
    let xl = o.xl();
    let yl = o.yl();
    let html = render_hexbin_html(&HexbinConfig {
        title,
        variant,
        x_label: &xl,
        y_label: &yl,
        gridlines: o.grid(),
        x_values: &x_values,
        y_values: &y_values,
        gridsize: o.bins.map(|b| b.max(2) as usize).unwrap_or(20),
        colorscale: &colorscale,
        palette: &o.pal(),
        width: o.w(900),
        height: o.h(520),
        hover: &hover,
        ..HexbinConfig::default()
    });
    apply(html, &o)
}
