use super::common::{cx_at, draw_candles, draw_volume_pane, finalize, layout_with_volume, open_with_axes, prepare, val_to_y};
use super::config::CandlestickConfig;
use crate::plot::statistical::common::{catmull_rom_path, hex6, moving_average, push_b, push_i};

const MA_PERIODS: &[usize] = &[5, 10, 20, 40, 70];
const MA_COLORS: &[u32] = &[0x64748b, 0x3b82f6, 0x06b6d4, 0x22c55e, 0xf97316];

#[crate::chart_demo(
    "labels=[\"Jan 01\",\"Jan 02\",\"Jan 03\",\"Jan 04\",\"Jan 05\",\"Jan 06\",\"Jan 07\",\"Jan 08\",\"Jan 09\",\"Jan 10\",\"Jan 11\",\"Jan 12\",\"Jan 13\",\"Jan 14\",\"Jan 15\",\"Jan 16\",\"Jan 17\",\"Jan 18\",\"Jan 19\",\"Jan 20\",\"Jan 21\",\"Jan 22\",\"Jan 23\",\"Jan 24\",\"Jan 25\",\"Jan 26\",\"Jan 27\",\"Jan 28\",\"Jan 29\",\"Jan 30\",\"Jan 31\",\"Feb 01\",\"Feb 02\",\"Feb 03\",\"Feb 04\",\"Feb 05\",\"Feb 06\",\"Feb 07\",\"Feb 08\",\"Feb 09\",\"Feb 10\",\"Feb 11\",\"Feb 12\",\"Feb 13\",\"Feb 14\",\"Feb 15\",\"Feb 16\",\"Feb 17\",\"Feb 18\",\"Feb 19\",\"Feb 20\",\"Feb 21\",\"Feb 22\",\"Feb 23\",\"Feb 24\",\"Feb 25\",\"Feb 26\",\"Feb 27\",\"Feb 28\",\"Mar 01\",\"Mar 02\",\"Mar 03\",\"Mar 04\",\"Mar 05\",\"Mar 06\",\"Mar 07\",\"Mar 08\",\"Mar 09\",\"Mar 10\",\"Mar 11\",\"Mar 12\",\"Mar 13\",\"Mar 14\",\"Mar 15\",\"Mar 16\",\"Mar 17\",\"Mar 18\",\"Mar 19\",\"Mar 20\",\"Mar 21\",\"Mar 22\",\"Mar 23\",\"Mar 24\",\"Mar 25\",\"Mar 26\",\"Mar 27\",\"Mar 28\",\"Mar 29\",\"Mar 30\",\"Mar 31\"], open=[200.0,203.08,205.03,203.88,203.87,204.86,208.5,208.01,211.82,209.32,210.52,211.78,212.82,216.01,222.1,220.36,220.88,216.93,216.9,216.92,214.06,218.07,215.66,207.93,209.65,210.14,209.92,209.1,210.71,209.92,204.76,208.69,206.68,205.79,204.34,202.25,197.69,200.02,204.89,203.78,205.98,206.46,208.51,214.2,215.42,214.87,212.24,210.45,209.75,215.39,212.61,218.12,218.61,216.75,220.49,223.24,221.71,225.36,230.02,228.19,229.27,231.84,230.21,228.43,227.59,226.34,224.74,223.89,224.36,226.59,225.56,227.54,230.85,233.87,236.68,230.28,232.93,235.32,238.36,243.34,244.55,247.37,249.46,249.59,253.95,255.12,254.71,259.17,259.99,261.47], high=[205.24,207.52,206.12,204.83,206.19,208.82,209.6,214.14,213.21,211.82,212.34,213.08,217.74,222.42,223.43,222.04,222.0,219.1,218.46,218.9,219.75,218.07,218.08,210.13,210.9,210.33,211.68,212.11,210.89,210.82,208.8,210.26,207.65,206.51,204.47,204.48,201.32,207.23,205.82,207.52,207.48,208.81,215.82,217.75,216.96,215.95,212.77,212.71,217.06,218.79,219.87,219.6,219.62,221.85,223.31,224.95,226.92,230.23,230.3,232.16,232.58,233.58,231.2,228.64,228.16,226.88,227.38,226.49,227.36,228.17,230.53,231.46,235.24,237.82,238.26,233.27,237.17,238.72,244.56,244.8,247.47,249.87,251.76,255.44,256.5,255.17,259.85,260.12,262.8,262.32], low=[199.5,201.64,202.01,202.61,201.46,203.06,206.18,205.61,208.73,207.83,207.76,211.75,211.47,215.55,218.47,219.76,216.76,216.84,216.86,213.09,212.83,212.94,205.8,207.26,207.24,208.68,208.43,206.16,208.85,203.12,202.86,205.18,204.67,202.18,201.69,196.24,195.73,199.16,203.24,203.59,205.28,204.47,206.84,213.19,213.23,210.79,209.56,209.66,207.91,211.27,211.46,217.55,216.11,215.62,219.96,221.42,221.54,225.2,227.47,227.92,228.08,229.46,227.96,224.93,225.83,222.96,222.1,222.93,224.29,224.93,223.03,225.51,229.94,232.2,229.48,229.43,232.37,232.74,235.76,242.13,244.1,246.36,248.12,248.72,251.94,253.4,254.63,256.74,259.48,260.61], close=[203.08,205.03,203.88,203.87,204.86,208.5,208.01,211.82,209.32,210.52,211.78,212.82,216.01,222.1,220.36,220.88,216.93,216.9,216.92,214.06,218.07,215.66,207.93,209.65,210.14,209.92,209.1,210.71,209.92,204.76,208.69,206.68,205.79,204.34,202.25,197.69,200.02,204.89,203.78,205.98,206.46,208.51,214.2,215.42,214.87,212.24,210.45,209.75,215.39,212.61,218.12,218.61,216.75,220.49,223.24,221.71,225.36,230.02,228.19,229.27,231.84,230.21,228.43,227.59,226.34,224.74,223.89,224.36,226.59,225.56,227.54,230.85,233.87,236.68,230.28,232.93,235.32,238.36,243.34,244.55,247.37,249.46,249.59,253.95,255.12,254.71,259.17,259.99,261.47,261.57], volume=[1318199,1096044,1601918,1151082,1566597,1886037,1396631,2614918,2180034,1322711,1394429,1190374,1458429,2243213,1622879,81380,1557710,1259764,2434891,1947167,1342507,943619,1957428,1902757,1920629,1377639,1713384,780504,2825720,1974154,1354394,1312077,1500603,1714513,1471800,1715330,1772537,1673781,1602074,2222236,551624,1691559,2657826,1732461,1299498,1637948,342227,1452728,1882864,1846220,1396129,1113193,1856500,2580056,1819702,676090,1832567,1270128,927017,2040295,1701523,1807237,2775988,2353753,2215225,1495561,2188516,2018505,1945664,1846904,1854427,1835016,1388436,1965098,2207623,1682720,1576571,1273612,2417236,1759040,1338812,2130119,1388368,1556610,1729491,1729904,2273649,2008729,2112190,943250], variant=\"indicators\""
)]

pub fn render(cfg: &CandlestickConfig) -> String {
    let mut p = match prepare(cfg) {
        Some(v) => v,
        None => return String::new(),
    };
    let n = p.n;
    let has_vol = cfg.volume.len() >= n;
    let vol_h = layout_with_volume(&mut p, has_vol, 0.16);
    let l = &p.layout;

    let mut b = Vec::<u8>::with_capacity(n * 320 + 8192);
    open_with_axes(&mut b, cfg, &p);
    draw_candles(&mut b, &p, 1.0);
    draw_volume_pane(&mut b, cfg, &p, vol_h);

    push_b(&mut b, b"<g class=\"sp-legend\">");
    for (k, &period) in MA_PERIODS.iter().enumerate() {
        let color = MA_COLORS[k % MA_COLORS.len()];
        let ma = moving_average(&p.close, period);
        let pts: Vec<(i32, i32)> = (0..n).map(|i| (cx_at(l, i), val_to_y(l, ma[i]))).collect();
        let hx = hex6(color);
        push_b(&mut b, b"<path fill=\"none\" stroke=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"1.6\" stroke-linecap=\"round\" opacity=\"0.92\" d=\"");
        catmull_rom_path(&mut b, &pts, 0.8);
        push_b(&mut b, b"\"/>");

        let ly = l.pad_t + 8 + k as i32 * 13;
        push_b(&mut b, b"<line x1=\"");
        push_i(&mut b, l.pad_l + l.plot_w - 90);
        push_b(&mut b, b"\" y1=\"");
        push_i(&mut b, ly);
        push_b(&mut b, b"\" x2=\"");
        push_i(&mut b, l.pad_l + l.plot_w - 74);
        push_b(&mut b, b"\" y2=\"");
        push_i(&mut b, ly);
        push_b(&mut b, b"\" stroke=\"#");
        b.extend_from_slice(&hx);
        push_b(&mut b, b"\" stroke-width=\"2.4\"/>");
        push_b(&mut b, b"<text x=\"");
        push_i(&mut b, l.pad_l + l.plot_w - 70);
        push_b(&mut b, b"\" y=\"");
        push_i(&mut b, ly + 3);
        push_b(&mut b, b"\" font-family=\"Arial,sans-serif\" font-size=\"8\" fill=\"#334155\">MA");
        push_i(&mut b, period as i32);
        push_b(&mut b, b"</text>");
    }
    push_b(&mut b, b"</g>");

    finalize(b, cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg<'a>(open: &'a [f64], high: &'a [f64], low: &'a [f64], close: &'a [f64], volume: &'a [f64], labels: &'a [String]) -> CandlestickConfig<'a> {
        CandlestickConfig {
            title: "Test",
            labels,
            open,
            high,
            low,
            close,
            volume,
            width: 1200,
            height: 560,
            ..CandlestickConfig::default()
        }
    }

    fn synth(n: usize) -> (Vec<String>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
        let labels: Vec<String> = (0..n).map(|i| format!("d{i}")).collect();
        let mut close = Vec::with_capacity(n);
        let mut v = 100.0;
        for i in 0..n {
            v += ((i as f64 * 0.5).cos()) * 2.5 + 0.3;
            close.push(v);
        }
        let open: Vec<f64> = std::iter::once(close[0]).chain(close[..n - 1].iter().cloned()).collect();
        let high: Vec<f64> = (0..n).map(|i| open[i].max(close[i]) + 1.2).collect();
        let low: Vec<f64> = (0..n).map(|i| open[i].min(close[i]) - 1.2).collect();
        let volume: Vec<f64> = (0..n).map(|i| 1000.0 + (i as f64 * 37.0) % 500.0).collect();
        (labels, open, high, low, close, volume)
    }

    #[test]
    fn renders_candles_a_volume_pane_and_every_moving_average() {
        let (labels, open, high, low, close, volume) = synth(80);
        let html = render(&cfg(&open, &high, &low, &close, &volume, &labels));
        assert!(!html.is_empty());
        assert!(html.contains("data-kv-Volume="));
        assert_eq!(html.matches("<path fill=\"none\"").count(), MA_PERIODS.len());
        for &period in MA_PERIODS {
            assert!(html.contains(&format!("MA{period}")));
        }
    }

    #[test]
    fn without_volume_data_still_renders_candles_and_moving_averages() {
        let (labels, open, high, low, close, _) = synth(50);
        let no_vol: Vec<f64> = vec![];
        let html = render(&cfg(&open, &high, &low, &close, &no_vol, &labels));
        assert!(!html.is_empty());
        assert!(!html.contains("data-kv-Volume="));
        assert_eq!(html.matches("<path fill=\"none\"").count(), MA_PERIODS.len());
    }

    #[test]
    fn empty_input_returns_empty_string_instead_of_a_broken_chart() {
        let labels: Vec<String> = vec![];
        let empty: Vec<f64> = vec![];
        assert!(render(&cfg(&empty, &empty, &empty, &empty, &empty, &labels)).is_empty());
    }

    #[test]
    fn perf_rendering_a_realistic_trading_history_with_volume_stays_fast() {
        let (labels, open, high, low, close, volume) = synth(500);
        let start = std::time::Instant::now();
        let html = render(&cfg(&open, &high, &low, &close, &volume, &labels));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 250, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
