use super::common::render_html_projected;
use super::config::ChoroplethConfig;
use crate::core::math::heat_color;
use crate::plot::map::projections::Projection;

#[crate::chart_demo(
    "labels=[\"USA\",\"CAN\",\"RUS\",\"NOR\",\"SWE\",\"FIN\",\"ISL\",\"GRL\",\"CHN\",\"DEU\",\"FRA\",\"JPN\"], values=[12.4,9.1,5.2,3.8,2.9,2.1,0.7,0.1,1.0,4.0,3.5,1.5], title=\"Arctic Research Stations (count)\", center_lat=90, center_lon=0"
)]

pub fn render(cfg: &ChoroplethConfig) -> String {
    let lat = cfg.center_lat.unwrap_or(90.0);
    let lon = cfg.center_lon.unwrap_or(0.0);
    render_html_projected(cfg, |value, _min, max| heat_color(value, max), Projection::Polar, lat, lon)
}
