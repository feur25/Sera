use super::common::render_impl;
use super::config::CorrelogramConfig;

#[crate::chart_demo(
    "labels=[\"mpg\",\"cyl\",\"disp\",\"hp\",\"drat\",\"wt\",\"qsec\",\"vs\"], matrix=[[1,-0.85,-0.85,-0.78,0.68,-0.87,0.42,0.66],[-0.85,1,0.9,0.83,-0.7,0.78,-0.59,-0.81],[-0.85,0.9,1,0.79,-0.71,0.89,-0.43,-0.71],[-0.78,0.83,0.79,1,-0.45,0.66,-0.71,-0.72],[0.68,-0.7,-0.71,-0.45,1,-0.71,0.09,0.44],[-0.87,0.78,0.89,0.66,-0.71,1,-0.17,-0.55],[0.42,-0.59,-0.43,-0.71,0.09,-0.17,1,0.74],[0.66,-0.81,-0.71,-0.72,0.44,-0.55,0.74,1]], variant=\"circle_legend\""
)]
pub fn render(cfg: &CorrelogramConfig) -> String {
    render_impl(cfg, "circle", "", "upper", true)
}
