use super::common::{render as base_render, Glyph};
use super::config::CartogramConfig;

#[crate::chart_demo(
    "labels=[\"USA\",\"CHN\",\"IND\",\"BRA\",\"RUS\",\"DEU\",\"FRA\",\"GBR\",\"JPN\",\"NGA\",\"EGY\",\"AUS\",\"IDN\",\"PAK\",\"BGD\",\"MEX\",\"VNM\",\"TUR\",\"ITA\",\"ESP\",\"KOR\",\"ARG\",\"CAN\",\"ETH\"], values=[331,1412,1408,215,144,84,68,67,125,219,109,26,273,220,165,129,97,84,60,47,52,45,38,115], lats=[39,35,21,-10,61,51,47,54,36,9,27,-25,-2,30,24,23,16,39,43,40,36,-34,56,9], lons=[-98,105,78,-51,90,10,2,-2,138,8,30,133,118,70,90,-102,108,35,12,-4,128,-64,-106,40], title=\"Population (millions)\""
)]
pub fn render(cfg: &CartogramConfig) -> String {
    base_render(cfg, Glyph::Circle)
}
