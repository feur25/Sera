use super::common::{cx_at, draw_candles, finalize, open_with_axes, prepare, val_to_y};
use super::config::CandlestickConfig;
use crate::plot::statistical::common::{catmull_rom_path, draw_point_callout, format_axis_label, hex6, local_maxima_indices, moving_average, push_b};

#[crate::chart_demo(
    "labels=[\"Jan 01\",\"Jan 02\",\"Jan 03\",\"Jan 04\",\"Jan 05\",\"Jan 06\",\"Jan 07\",\"Jan 08\",\"Jan 09\",\"Jan 10\",\"Jan 11\",\"Jan 12\",\"Jan 13\",\"Jan 14\",\"Jan 15\",\"Jan 16\",\"Jan 17\",\"Jan 18\",\"Jan 19\",\"Jan 20\",\"Jan 21\",\"Jan 22\",\"Jan 23\",\"Jan 24\",\"Jan 25\",\"Jan 26\",\"Jan 27\",\"Jan 28\",\"Jan 29\",\"Jan 30\",\"Jan 31\",\"Feb 01\",\"Feb 02\",\"Feb 03\",\"Feb 04\",\"Feb 05\",\"Feb 06\",\"Feb 07\",\"Feb 08\",\"Feb 09\",\"Feb 10\",\"Feb 11\",\"Feb 12\",\"Feb 13\",\"Feb 14\",\"Feb 15\",\"Feb 16\",\"Feb 17\",\"Feb 18\",\"Feb 19\",\"Feb 20\",\"Feb 21\",\"Feb 22\",\"Feb 23\",\"Feb 24\",\"Feb 25\",\"Feb 26\",\"Feb 27\",\"Feb 28\",\"Mar 01\"], open=[100.0,99.94,99.79,101.92,102.66,100.35,101.5,100.42,100.7,101.54,104.64,104.0,104.18,103.81,106.12,107.15,109.59,109.77,110.02,111.44,112.37,113.7,112.03,114.44,115.18,112.49,111.06,112.57,113.02,113.6,115.4,111.99,108.84,106.49,106.0,105.94,105.03,103.37,102.41,101.92,100.03,99.27,101.14,101.89,102.83,104.55,105.57,106.05,107.17,105.87,106.75,107.71,108.77,108.91,108.63,109.03,111.91,111.87,108.07,109.67], high=[100.86,100.08,102.73,103.46,103.69,101.74,101.86,101.56,102.34,105.52,104.87,105.09,104.3,106.31,107.29,110.0,109.96,110.15,112.51,113.03,114.0,113.82,114.86,116.5,115.83,112.53,113.25,114.41,114.47,116.47,115.68,112.5,110.25,107.25,107.17,107.07,106.1,103.46,102.86,103.15,100.94,101.92,102.78,103.72,105.44,105.96,107.12,108.69,107.89,107.14,108.04,109.1,109.48,110.02,110.1,112.16,112.82,113.01,109.81,112.6], low=[99.45,99.29,98.67,101.22,99.49,100.08,99.67,100.14,100.43,100.34,103.57,103.27,103.47,103.09,105.49,106.71,108.74,108.75,108.7,111.39,112.0,111.7,111.9,113.55,111.71,109.97,110.34,111.66,112.83,112.73,110.97,107.73,105.62,105.08,105.68,104.42,102.04,101.88,100.62,100.0,98.11,98.6,100.63,101.29,101.23,104.14,105.14,105.37,105.07,104.94,104.94,107.16,108.01,108.62,107.61,108.6,110.72,107.95,107.39,108.97], close=[99.94,99.79,101.92,102.66,100.35,101.5,100.42,100.7,101.54,104.64,104.0,104.18,103.81,106.12,107.15,109.59,109.77,110.02,111.44,112.37,113.7,112.03,114.44,115.18,112.49,111.06,112.57,113.02,113.6,115.4,111.99,108.84,106.49,106.0,105.94,105.03,103.37,102.41,101.92,100.03,99.27,101.14,101.89,102.83,104.55,105.57,106.05,107.17,105.87,106.75,107.71,108.77,108.91,108.63,109.03,111.91,111.87,108.07,109.67,112.08]"
)]

pub fn render(cfg: &CandlestickConfig) -> String {
    let p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let l = &p.layout;
    let mut b = Vec::<u8>::with_capacity(p.n * 260 + 4096);
    open_with_axes(&mut b, cfg, &p);
    draw_candles(&mut b, &p, 0.28);

    let short_ma = moving_average(&p.close, 4);
    let long_ma = moving_average(&p.close, 12);
    let short_pts: Vec<(i32, i32)> = (0..p.n).map(|i| (cx_at(l, i), val_to_y(l, short_ma[i]))).collect();
    let long_pts: Vec<(i32, i32)> = (0..p.n).map(|i| (cx_at(l, i), val_to_y(l, long_ma[i]))).collect();

    let long_color = 0x1e3a8a;
    let short_color = 0x60a5fa;
    push_b(&mut b, b"<path fill=\"none\" stroke=\"#");
    b.extend_from_slice(&hex6(long_color));
    push_b(&mut b, b"\" stroke-width=\"2.2\" stroke-linecap=\"round\" d=\"");
    catmull_rom_path(&mut b, &long_pts, 0.85);
    push_b(&mut b, b"\"/>");
    push_b(&mut b, b"<path fill=\"none\" stroke=\"#");
    b.extend_from_slice(&hex6(short_color));
    push_b(&mut b, b"\" stroke-width=\"2.6\" stroke-linecap=\"round\" d=\"");
    catmull_rom_path(&mut b, &short_pts, 0.85);
    push_b(&mut b, b"\"/>");

    let peaks = local_maxima_indices(&short_ma, 4, p.n / 8);
    for &idx in &peaks {
        let x = cx_at(l, idx);
        let y = val_to_y(l, short_ma[idx]);
        draw_point_callout(&mut b, x, y, &format_axis_label(p.close[idx]), short_color, true);
    }

    finalize(b, cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg<'a>(open: &'a [f64], high: &'a [f64], low: &'a [f64], close: &'a [f64], labels: &'a [String]) -> CandlestickConfig<'a> {
        CandlestickConfig {
            title: "Test",
            labels,
            open,
            high,
            low,
            close,
            width: 1100,
            height: 500,
            ..CandlestickConfig::default()
        }
    }

    fn synth(n: usize) -> (Vec<String>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
        let labels: Vec<String> = (0..n).map(|i| format!("d{i}")).collect();
        let mut close = Vec::with_capacity(n);
        let mut v = 100.0;
        for i in 0..n {
            v += ((i as f64 * 0.7).sin()) * 3.0 + 0.4;
            close.push(v);
        }
        let open: Vec<f64> = std::iter::once(close[0]).chain(close[..n - 1].iter().cloned()).collect();
        let high: Vec<f64> = (0..n).map(|i| open[i].max(close[i]) + 1.0).collect();
        let low: Vec<f64> = (0..n).map(|i| open[i].min(close[i]) - 1.0).collect();
        (labels, open, high, low, close)
    }

    #[test]
    fn renders_ghosted_candles_with_reduced_opacity() {
        let (labels, open, high, low, close) = synth(30);
        let html = render(&cfg(&open, &high, &low, &close, &labels));
        assert!(!html.is_empty());
        assert!(html.contains("fill-opacity=\"0.28\""));
    }

    #[test]
    fn embeds_two_smooth_moving_average_trend_lines() {
        let (labels, open, high, low, close) = synth(30);
        let html = render(&cfg(&open, &high, &low, &close, &labels));
        assert_eq!(html.matches("<path fill=\"none\"").count(), 2);
    }

    #[test]
    fn marks_local_peaks_with_a_circle_leader_line_and_value_label() {
        let (labels, open, high, low, close) = synth(40);
        let html = render(&cfg(&open, &high, &low, &close, &labels));
        assert!(html.matches("<circle cx=").count() >= 1);
        assert!(html.matches("<circle cx=").count() <= 4);
    }

    #[test]
    fn empty_input_returns_empty_string_instead_of_a_broken_chart() {
        let labels: Vec<String> = vec![];
        let empty: Vec<f64> = vec![];
        assert!(render(&cfg(&empty, &empty, &empty, &empty, &labels)).is_empty());
    }

    #[test]
    fn perf_rendering_a_realistic_trading_history_stays_fast() {
        let (labels, open, high, low, close) = synth(400);
        let start = std::time::Instant::now();
        let html = render(&cfg(&open, &high, &low, &close, &labels));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
