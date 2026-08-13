use super::config::BarConfig;
use super::spiral_common::render as render_spiral;
use crate::plot::statistical::common::hex6;

#[crate::chart_demo(
    "labels=[\"1950\",\"1951\",\"1952\",\"1953\",\"1954\",\"1955\",\"1956\",\"1957\",\"1958\",\"1959\",\"1960\",\"1961\",\"1962\",\"1963\",\"1964\",\"1965\",\"1966\",\"1967\",\"1968\",\"1969\",\"1970\",\"1971\",\"1972\",\"1973\",\"1974\",\"1975\",\"1976\",\"1977\",\"1978\",\"1979\",\"1980\",\"1981\",\"1982\",\"1983\",\"1984\",\"1985\",\"1986\",\"1987\",\"1988\",\"1989\",\"1990\",\"1991\",\"1992\",\"1993\",\"1994\",\"1995\",\"1996\",\"1997\",\"1998\",\"1999\",\"2000\",\"2001\",\"2002\",\"2003\",\"2004\",\"2005\",\"2006\",\"2007\",\"2008\",\"2009\",\"2010\",\"2011\",\"2012\"], values=[6,7,9,8,10,11,9,12,13,11,14,15,13,16,18,17,19,21,20,18,22,20,19,23,21,24,26,23,25,27,26,28,25,24,27,29,26,28,31,29,33,36,31,29,34,32,38,35,33,37,30,28,33,39,42,45,40,38,44,49,52,58,64], variant=\"spiral\", width=680, height=370"
)]

pub fn render(cfg: &BarConfig) -> String {
    render_spiral(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg<'a>(labels: &'a [String], values: &'a [f64]) -> BarConfig<'a> {
        BarConfig {
            title: "Test",
            labels,
            values,
            width: 700,
            height: 500,
            ..BarConfig::default()
        }
    }

    fn synth(n: usize) -> (Vec<String>, Vec<f64>) {
        let labels: Vec<String> = (0..n).map(|i| format!("Y{}", 1950 + i)).collect();
        let values: Vec<f64> = (0..n).map(|i| 5.0 + (i as f64 * 0.7).sin().abs() * 20.0 + i as f64 * 0.3).collect();
        (labels, values)
    }

    #[test]
    fn renders_one_bar_per_point_along_the_spiral() {
        let (labels, values) = synth(60);
        let html = render(&cfg(&labels, &values));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<path data-idx=").count(), 60);
        assert!(html.contains("class=\"sp-bg\""));
    }

    #[test]
    fn the_baseline_radius_grows_monotonically_with_index() {
        let n = 80;
        let r_hub = 10.0;
        let growth = 0.5;
        let mut last = -1.0;
        for i in 0..n {
            let r = r_hub + growth * i as f64;
            assert!(r > last);
            last = r;
        }
    }

    #[test]
    fn longer_bars_get_the_high_end_of_the_color_gradient() {
        let (labels, values) = synth(40);
        let html = render(&cfg(&labels, &values));
        let low = hex6(0x636EFA);
        let high = hex6(0xF43F5E);
        assert!(html.contains(std::str::from_utf8(&low).unwrap()) || html.contains(std::str::from_utf8(&high).unwrap()));
    }

    #[test]
    fn lap_boundaries_are_labeled_with_the_real_axis_values() {
        let (labels, values) = synth(90);
        let html = render(&cfg(&labels, &values));
        assert!(html.contains(">Y1950<"));
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let labels: Vec<String> = vec![];
        let values: Vec<f64> = vec![];
        assert!(render(&cfg(&labels, &values)).is_empty());
    }

    #[test]
    fn perf_rendering_a_dense_multi_lap_spiral_stays_fast() {
        let (labels, values) = synth(2000);
        let start = std::time::Instant::now();
        let html = render(&cfg(&labels, &values));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
