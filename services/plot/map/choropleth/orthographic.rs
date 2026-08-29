use super::common::render_html_projected;
use super::config::ChoroplethConfig;
use crate::core::math::heat_color;
use crate::plot::map::projections::Projection;

#[crate::chart_demo(
    "labels=[\"US\",\"BR\",\"CN\",\"IN\",\"RU\",\"AU\",\"CA\",\"ZA\",\"EG\",\"FR\",\"DE\",\"JP\"], values=[331.9,213.3,1412.0,1380.0,144.1,25.7,38.2,59.9,104.3,67.8,83.2,125.7], title=\"Population by Country (millions)\", center_lat=15, center_lon=10"
)]

pub fn render(cfg: &ChoroplethConfig) -> String {
    let lat = cfg.center_lat.unwrap_or(15.0);
    let lon = cfg.center_lon.unwrap_or(10.0);
    render_html_projected(cfg, |value, _min, max| heat_color(value, max), Projection::Orthographic, lat, lon)
}
