use super::common::render_html_projected;
use super::config::ChoroplethConfig;
use crate::core::math::heat_color;
use crate::plot::map::projections::Projection;

#[crate::chart_demo(
    "labels=[\"US\",\"CN\",\"IN\",\"ID\",\"PK\",\"BR\",\"NG\",\"BD\",\"RU\",\"MX\",\"JP\",\"ET\",\"PH\",\"EG\",\"VN\",\"DE\",\"TR\",\"IR\",\"TH\",\"GB\",\"FR\",\"IT\",\"ZA\",\"TZ\",\"MM\",\"KR\",\"CO\",\"KE\",\"ES\",\"AR\",\"DZ\",\"UA\",\"UG\",\"IQ\",\"PL\",\"CA\",\"MA\",\"SA\",\"UZ\",\"PE\"], values=[331.9,1412.0,1380.0,273.5,220.9,213.3,206.1,164.7,144.1,128.9,125.7,114.9,109.6,104.3,97.3,83.2,84.3,84.0,69.8,67.9,67.4,60.4,59.9,59.7,54.4,51.8,50.9,53.8,47.4,45.4,43.9,44.1,45.7,40.2,38.4,38.2,36.9,34.8,33.5,33.0], title=\"Population by Country (millions)\", center_lat=15, center_lon=10"
)]

pub fn render(cfg: &ChoroplethConfig) -> String {
    let lat = cfg.center_lat.unwrap_or(15.0);
    let lon = cfg.center_lon.unwrap_or(10.0);
    render_html_projected(cfg, |value, _min, max| heat_color(value, max), Projection::Orthographic, lat, lon)
}
