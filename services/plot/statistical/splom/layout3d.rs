use super::common::prepare;
use super::config::SplomConfig;
use super::variant::SplomVariant;
use crate::plot::statistical::_3d::budget::{even_indices, pick, Budget};
use crate::plot::statistical::_3d::curves::marker_blocks;
use crate::plot::statistical::bar::Bar3DBlock;

pub const HEIGHT_RATIO: f64 = 0.4;
pub const COLORMAP: &str = "updown";
const POINT_SIZE: f64 = 0.045;
const POINT_HEIGHT: f64 = 0.06;
const MARGIN: f64 = 0.12;
const LINE_TONE: f64 = 0.5;

#[derive(Clone, Copy, PartialEq)]
enum Glyph {
    Points,
    Correlation,
    Regression,
}

fn recipe(variant: SplomVariant) -> Glyph {
    use SplomVariant::*;
    match variant {
        Basic | Density => Glyph::Points,
        Correlation => Glyph::Correlation,
        Regression => Glyph::Regression,
    }
}

fn normalized(values: &[f64], lo: f64, hi: f64) -> Vec<f64> {
    let range = (hi - lo).max(1e-9);
    values.iter().map(|&v| ((v - lo) / range).clamp(0.0, 1.0)).collect()
}

fn correlation(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len().min(ys.len());
    if n < 2 {
        return 0.0;
    }
    let (mx, my) = (xs[..n].iter().sum::<f64>() / n as f64, ys[..n].iter().sum::<f64>() / n as f64);
    let (mut sxy, mut sxx, mut syy) = (0.0, 0.0, 0.0);
    for i in 0..n {
        let (dx, dy) = (xs[i] - mx, ys[i] - my);
        sxy += dx * dy;
        sxx += dx * dx;
        syy += dy * dy;
    }
    let denom = (sxx * syy).sqrt();
    if denom > 1e-12 { (sxy / denom).clamp(-1.0, 1.0) } else { 0.0 }
}

fn linear_fit(xs: &[f64], ys: &[f64]) -> Option<(f64, f64)> {
    let n = xs.len().min(ys.len());
    if n < 2 {
        return None;
    }
    let (mx, my) = (xs[..n].iter().sum::<f64>() / n as f64, ys[..n].iter().sum::<f64>() / n as f64);
    let (mut num, mut den) = (0.0, 0.0);
    for i in 0..n {
        num += (xs[i] - mx) * (ys[i] - my);
        den += (xs[i] - mx).powi(2);
    }
    if den < 1e-12 {
        return None;
    }
    let slope = num / den;
    Some((slope, my - slope * mx))
}

pub fn layout_3d(cfg: &SplomConfig, budget: &Budget) -> Vec<Bar3DBlock> {
    layout_named(cfg, budget).0
}

pub fn layout_named(cfg: &SplomConfig, budget: &Budget) -> (Vec<Bar3DBlock>, Vec<String>) {
    let Some(p) = prepare(cfg) else {
        return (Vec::new(), Vec::new());
    };
    let glyph = recipe(cfg.variant);
    let cells = p.m * p.m.saturating_sub(1).max(1);
    let cap = (budget.elements() / cells.max(1)).max(8);
    let keep = even_indices(p.n, cap);
    let normed: Vec<Vec<f64>> = (0..p.m).map(|j| pick(&normalized(&p.cols[j], p.mins[j], p.maxs[j]), &keep)).collect();
    let n = keep.len();
    let mut blocks = Vec::new();
    let mut names = Vec::new();
    for r in 0..p.m {
        for c in 0..p.m {
            if r == c {
                continue;
            }
            let class = r * p.m + c;
            let points: Vec<(f64, f64)> = (0..n)
                .map(|i| (c as f64 + MARGIN + normed[c][i] * (1.0 - 2.0 * MARGIN), r as f64 + MARGIN + normed[r][i] * (1.0 - 2.0 * MARGIN)))
                .collect();
            let corr = correlation(&p.cols[c], &p.cols[r]);
            let tone = if glyph == Glyph::Correlation { Some((corr + 1.0) / 2.0) } else { None };
            blocks.extend(marker_blocks(&points, POINT_SIZE, POINT_HEIGHT, 0.0, class, tone));
            if glyph == Glyph::Regression {
                if let Some((slope, intercept)) = linear_fit(&p.cols[c], &p.cols[r]) {
                    let x_lo = p.mins[c];
                    let x_hi = p.maxs[c];
                    let y_at = |x: f64| ((slope * x + intercept - p.mins[r]) / (p.maxs[r] - p.mins[r]).max(1e-9)).clamp(0.0, 1.0);
                    let line: Vec<(f64, f64)> = [x_lo, x_hi]
                        .iter()
                        .map(|&x| {
                            let nx = (x - x_lo) / (x_hi - x_lo).max(1e-9);
                            (c as f64 + MARGIN + nx * (1.0 - 2.0 * MARGIN), r as f64 + MARGIN + y_at(x) * (1.0 - 2.0 * MARGIN))
                        })
                        .collect();
                    blocks.extend(marker_blocks(&line, POINT_SIZE * 0.7, POINT_HEIGHT * 1.6, 0.0, class, Some(LINE_TONE)));
                }
            }
            names.push(format!("{} vs {}", cfg.axes[c], cfg.axes[r]));
        }
    }
    let full_names: Vec<String> = (0..p.m * p.m)
        .map(|idx| {
            let (r, c) = (idx / p.m, idx % p.m);
            if r == c { cfg.axes[r].clone() } else { format!("{} vs {}", cfg.axes[c], cfg.axes[r]) }
        })
        .collect();
    (blocks, full_names)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn axes() -> Vec<String> {
        ["Speed", "Power", "Range"].iter().map(|s| s.to_string()).collect()
    }

    fn series() -> Vec<Vec<f64>> {
        vec![vec![80.0, 65.0, 70.0], vec![60.0, 80.0, 55.0], vec![40.0, 70.0, 90.0], vec![90.0, 40.0, 60.0], vec![55.0, 85.0, 45.0], vec![70.0, 55.0, 80.0]]
    }

    fn draw(variant: SplomVariant) -> (Vec<Bar3DBlock>, Vec<String>) {
        let ax = axes();
        let s = series();
        let cfg = SplomConfig { variant, axes: &ax, series_values: &s, ..SplomConfig::default() };
        layout_named(&cfg, &Budget::default())
    }

    #[test]
    fn every_splom_variant_draws_every_off_diagonal_cell() {
        for &variant in SplomVariant::all() {
            let (blocks, names) = draw(variant);
            assert!(!blocks.is_empty(), "{variant:?}");
            assert_eq!(names.len(), 9, "{variant:?}");
            assert!(names.contains(&"Speed".to_string()));
        }
    }

    #[test]
    fn correlation_tones_cells_by_their_pearson_coefficient() {
        let (blocks, _) = draw(SplomVariant::Correlation);
        assert!(blocks.iter().any(|b| b.tone.is_some()));
    }

    #[test]
    fn regression_adds_a_fit_line_on_top_of_the_points() {
        let (basic, _) = draw(SplomVariant::Basic);
        let (regression, _) = draw(SplomVariant::Regression);
        assert!(regression.len() > basic.len());
    }

    #[test]
    fn empty_input_draws_nothing() {
        assert!(layout_named(&SplomConfig::default(), &Budget::default()).0.is_empty());
    }

    #[test]
    fn a_wide_axis_count_stays_within_a_bounded_block_count() {
        let axes: Vec<String> = (0..10).map(|i| format!("A{i}")).collect();
        let series: Vec<Vec<f64>> = (0..5000).map(|i| (0..10).map(|j| ((i * (j + 1)) % 97) as f64).collect()).collect();
        let cfg = SplomConfig { axes: &axes, series_values: &series, ..SplomConfig::default() };
        let (blocks, _) = layout_named(&cfg, &Budget::default());
        assert!(blocks.len() < 12_000);
    }
}
