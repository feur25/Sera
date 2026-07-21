use crate::plot::{apply, parse_all};
pub mod common;
pub mod config;
pub mod heat_scatter;
pub mod hexbin_marginal;
pub mod layered_bivariate;
pub mod variant;

pub use config::JointConfig;
pub use variant::JointVariant;

pub fn render_joint_html(cfg: &JointConfig) -> String {
    use variant::JointVariant::*;
    match cfg.variant {
        HexbinMarginal => hexbin_marginal::render(cfg),
        HeatScatter => heat_scatter::render(cfg),
        LayeredBivariate => layered_bivariate::render(cfg),
    }
}

pub use build as build_joint;

#[crate::sera_alias("joint", "jointplot", "joint_plot", "bivariate")]
#[crate::sera_builder("build_joint")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x_values = a.x.unwrap_or_default();
    let y_values = a.y.unwrap_or_default();
    use crate::plot::statistical::{render_joint_html, JointConfig, JointVariant};
    let hover = o.hj();
    let variant = JointVariant::from_str(o.variant.as_deref().unwrap_or("hexbin_marginal"));
    let colorscale = o.colorscale.clone().unwrap_or_default();
    let xl = o.xl();
    let yl = o.yl();
    let html = render_joint_html(&JointConfig {
        title,
        variant,
        x_label: &xl,
        y_label: &yl,
        gridlines: o.grid(),
        x_values: &x_values,
        y_values: &y_values,
        bins: o.bins.map(|b| b.max(2) as usize).unwrap_or(24),
        colorscale: &colorscale,
        point_hex: o.color_hex.unwrap_or(0x6366f1),
        palette: &o.pal(),
        width: o.w(760),
        height: o.h(760),
        hover: &hover,
        ..JointConfig::default()
    });
    apply(html, &o)
}
