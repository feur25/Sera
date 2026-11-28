use super::common::{draw_candles, draw_volume_pane, finalize, layout_with_volume, open_with_axes, prepare};
use super::config::CandlestickConfig;

#[crate::chart_demo(
    "labels=[\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\",\"Sat\",\"Sun\"], open=[100,102,105,103,108,111,109], high=[105,107,109,110,114,116,113], low=[99,101,103,102,107,109,106], close=[102,105,103,108,112,110,107], volume=[1200,900,1500,700,2100,1800,1000], variant=\"volume\""
)]

pub fn render(cfg: &CandlestickConfig) -> String {
    let mut p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let n = p.n;
    let has_vol = cfg.volume.len() >= n;
    let vol_h = layout_with_volume(&mut p, has_vol, 0.22);

    let mut b = Vec::<u8>::with_capacity(n * 300 + 4096);
    open_with_axes(&mut b, cfg, &p);
    draw_candles(&mut b, &p, 1.0);
    draw_volume_pane(&mut b, cfg, &p, vol_h);
    finalize(b, cfg)
}
