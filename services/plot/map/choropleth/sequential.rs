use super::common::render_html;
use super::config::ChoroplethConfig;
use crate::core::math::heat_color;

#[crate::chart_demo(
    "labels=[\"AL\",\"AK\",\"AZ\",\"AR\",\"CA\",\"CO\",\"CT\",\"DE\",\"FL\",\"GA\",\"HI\",\"ID\",\"IL\",\"IN\",\"IA\",\"KS\",\"KY\",\"LA\",\"ME\",\"MD\",\"MA\",\"MI\",\"MN\",\"MS\",\"MO\",\"MT\",\"NE\",\"NV\",\"NH\",\"NJ\",\"NM\",\"NY\",\"NC\",\"ND\",\"OH\",\"OK\",\"OR\",\"PA\",\"RI\",\"SC\",\"SD\",\"TN\",\"TX\",\"UT\",\"VT\",\"VA\",\"WA\",\"WV\",\"WI\",\"WY\",\"DC\"], values=[5.1,0.73,7.4,3.0,38.9,5.9,3.6,1.0,22.6,11.0,1.4,2.0,12.6,6.8,3.2,2.9,4.5,4.6,1.4,6.2,7.0,10.0,5.7,2.9,6.2,1.1,2.0,3.2,1.4,9.3,2.1,19.6,10.8,0.78,11.8,4.0,4.2,12.9,1.1,5.4,0.92,7.1,30.5,3.4,0.65,8.7,7.8,1.8,5.9,0.58,0.68], title=\"Population by State (millions)\", map=\"usa_states\""
)]

pub fn render(cfg: &ChoroplethConfig) -> String {
    render_html(cfg, |value, _min, max| heat_color(value, max))
}
