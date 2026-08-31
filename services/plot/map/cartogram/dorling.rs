use super::common::{render as base_render, Glyph};
use super::config::CartogramConfig;

#[crate::chart_demo(
    "labels=[\"USA\",\"CHN\",\"IND\",\"BRA\",\"RUS\",\"DEU\",\"FRA\",\"GBR\",\"JPN\",\"NGA\",\"EGY\",\"AUS\"], values=[331,1412,1408,215,144,84,68,67,125,219,109,26], lats=[39,35,21,-10,61,51,47,54,36,9,27,-25], lons=[-98,105,78,-51,90,10,2,-2,138,8,30,133], title=\"Population (millions)\""
)]
pub fn render(cfg: &CartogramConfig) -> String {
    base_render(cfg, Glyph::Circle)
}
