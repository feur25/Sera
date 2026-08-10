use super::common::{draw_candles, finalize, open_with_axes, prepare};
use super::config::CandlestickConfig;

#[crate::chart_demo("labels=[\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\"], open=[100,102,105,103,108], high=[105,107,109,110,114], low=[99,101,103,102,107], close=[102,105,103,108,112]")]

pub fn render(cfg: &CandlestickConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let mut b = Vec::<u8>::with_capacity(p.n * 220 + 4096);
    open_with_axes(&mut b, cfg, &p);
    draw_candles(&mut b, &p, 1.0);
    finalize(b, cfg)
}
