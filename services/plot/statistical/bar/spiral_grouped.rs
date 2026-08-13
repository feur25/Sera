use super::config::BarConfig;
use super::spiral_common::render as render_spiral;

#[crate::chart_demo(
    "labels=[\"Participation a des salons\",\"Visite de salons\",\"Prospection telephonique\",\"Voyages et conventions d'affaires\",\"Site web\",\"Mailing\",\"Emailing\",\"Reponses aux appels d'offres publics\",\"Recommandations des clients\",\"Reseaux de professionnels\",\"Club d'entreprises\",\"Action des CCI\"], series_names=[\"Designer independant\",\"Agence de design\"], series=[[7.3,4.3,3.2,6.7,11.7,2.9,9.3,4.2,36.6,33.3,18.6,20.3],[4.7,3.9,2.5,2.9,8.7,3.8,6.7,4.4,20.2,18.2,11.4,10.9]], palette=[2262390,7449553], variant=\"spiral_grouped\", width=680, height=370"
)]

pub fn render(cfg: &BarConfig) -> String {
    render_spiral(cfg, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg<'a>(labels: &'a [String], series: &'a [(String, Vec<f64>)]) -> BarConfig<'a> {
        BarConfig {
            title: "Test",
            labels,
            series,
            width: 700,
            height: 500,
            ..BarConfig::default()
        }
    }

    fn synth(n: usize, n_series: usize) -> (Vec<String>, Vec<(String, Vec<f64>)>) {
        let labels: Vec<String> = (0..n).map(|i| format!("Item {i}")).collect();
        let series: Vec<(String, Vec<f64>)> = (0..n_series)
            .map(|s| {
                let vals: Vec<f64> = (0..n).map(|i| 5.0 + ((i + s) as f64 * 0.6).sin().abs() * 20.0).collect();
                (format!("Series {s}"), vals)
            })
            .collect();
        (labels, series)
    }

    #[test]
    fn renders_one_sub_bar_per_item_per_series() {
        let (labels, series) = synth(12, 2);
        let html = render(&cfg(&labels, &series));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<path data-idx=").count(), 24);
        assert!(html.contains("class=\"sp-bg\""));
    }

    #[test]
    fn each_series_gets_its_own_legend_swatch() {
        let (labels, series) = synth(8, 3);
        let html = render(&cfg(&labels, &series));
        assert!(html.contains(">Series 0<"));
        assert!(html.contains(">Series 1<"));
        assert!(html.contains(">Series 2<"));
    }

    #[test]
    fn empty_input_returns_empty_string() {
        let labels: Vec<String> = vec![];
        let series: Vec<(String, Vec<f64>)> = vec![];
        assert!(render(&cfg(&labels, &series)).is_empty());
    }

    #[test]
    fn perf_rendering_many_grouped_spiral_items_stays_fast() {
        let (labels, series) = synth(400, 3);
        let start = std::time::Instant::now();
        let html = render(&cfg(&labels, &series));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
