use super::common::render_html;
use super::config::ChoroplethConfig;
use crate::core::math::heat_color;

#[crate::chart_demo(
    "labels=[\"CA\",\"TX\",\"NY\",\"FL\",\"WA\",\"CO\",\"IL\",\"OH\",\"GA\",\"AZ\",\"NV\",\"UT\",\"OR\",\"NC\",\"MA\"], values=[38.9,30.5,19.6,22.6,7.8,5.9,12.6,11.8,11.0,7.4,3.2,3.4,4.2,10.8,7.0], title=\"Population by State (millions)\", map=\"usa_states\""
)]

pub fn render(cfg: &ChoroplethConfig) -> String {
    render_html(cfg, |value, _min, max| heat_color(value, max))
}
