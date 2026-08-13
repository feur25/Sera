use super::config::BarConfig;
use super::spiral_common::render_dual;

#[crate::chart_demo(
    "labels=[\"Participation a des salons\",\"Visite de salons\",\"Prospection telephonique\",\"Voyages et conventions d'affaires\",\"Site web\",\"Mailing\",\"Emailing\",\"Reponses aux appels d'offres publics\",\"Recommandations des clients\",\"Reseaux de professionnels\",\"Club d'entreprises\",\"Action des CCI\",\"Action des organismes de promotion du design\"], series_names=[\"Designer independant\",\"Agence de design\"], series=[[7.3,4.3,3.2,6.7,11.7,2.9,9.3,4.2,36.6,33.3,18.6,20.3,15.8],[4.7,3.9,2.5,2.9,8.7,3.8,6.7,4.4,20.2,18.2,11.4,10.9,9.6]], series2=[[2.6,2.4,1.8,2.7,3.3,1.5,1.9,2.5,3.8,3.6,2.9,3.4,3.5],[2.3,2.1,1.6,2.5,3.6,1.4,1.7,2.8,3.7,3.5,2.6,3.2,3.3]], palette=[3039066,6271907], color_low=1976888, color_high=14832700, variant=\"spiral_grouped\", width=920, height=430"
)]

pub fn render(cfg: &BarConfig) -> String {
    render_dual(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg<'a>(labels: &'a [String], series: &'a [(String, Vec<f64>)], series2: &'a [(String, Vec<f64>)]) -> BarConfig<'a> {
        BarConfig {
            title: "Test",
            labels,
            series,
            series2,
            width: 920,
            height: 430,
            ..BarConfig::default()
        }
    }

    fn synth(n: usize, n_series: usize) -> (Vec<String>, Vec<(String, Vec<f64>)>, Vec<(String, Vec<f64>)>) {
        let labels: Vec<String> = (0..n).map(|i| format!("Item {i}")).collect();
        let series: Vec<(String, Vec<f64>)> = (0..n_series)
            .map(|s| {
                let vals: Vec<f64> = (0..n).map(|i| 5.0 + ((i + s) as f64 * 0.6).sin().abs() * 20.0).collect();
                (format!("Series {s}"), vals)
            })
            .collect();
        let series2: Vec<(String, Vec<f64>)> = (0..n_series)
            .map(|s| {
                let vals: Vec<f64> = (0..n).map(|i| 1.0 + ((i + s) as f64 * 0.5).sin().abs() * 3.0).collect();
                (format!("Series {s}"), vals)
            })
            .collect();
        (labels, series, series2)
    }

    #[test]
    fn renders_both_panels_one_wedge_per_item_per_series() {
        let (labels, series, series2) = synth(13, 2);
        let html = render(&cfg(&labels, &series, &series2));
        assert!(!html.is_empty());
        assert_eq!(html.matches("<path data-idx=").count(), 13 * 2 * 2);
        assert!(html.contains("class=\"sp-bg\""));
    }

    #[test]
    fn draws_the_four_named_efficacy_reference_levels() {
        let (labels, series, series2) = synth(13, 2);
        let html = render(&cfg(&labels, &series, &series2));
        assert!(html.contains("Pas du tout efficace"));
        assert!(html.contains("Tres efficace"));
    }

    #[test]
    fn both_legend_blocks_name_their_series() {
        let (labels, series, series2) = synth(13, 2);
        let html = render(&cfg(&labels, &series, &series2));
        assert!(html.contains(">MOYENS<"));
        assert!(html.contains(">EFFICACITE<"));
        assert!(html.contains(">Series 0<"));
        assert!(html.contains(">Series 1<"));
    }

    #[test]
    fn the_numbered_index_legend_lists_every_label() {
        let (labels, series, series2) = synth(13, 2);
        let html = render(&cfg(&labels, &series, &series2));
        for l in &labels {
            assert!(html.contains(l.as_str()));
        }
        assert_eq!(html.matches("<tspan font-weight=\"700\">").count(), 13);
    }

    #[test]
    fn the_shared_spine_numbers_every_category() {
        let (labels, series, series2) = synth(13, 2);
        let html = render(&cfg(&labels, &series, &series2));
        for i in 1..=13 {
            assert!(html.contains(&format!(">{i}<")));
        }
    }

    #[test]
    fn fewer_than_two_categories_returns_empty_string() {
        let (labels, series, series2) = synth(1, 2);
        assert!(render(&cfg(&labels, &series, &series2)).is_empty());
    }

    #[test]
    fn missing_series2_returns_empty_string() {
        let (labels, series, _series2) = synth(13, 2);
        let empty: Vec<(String, Vec<f64>)> = vec![];
        assert!(render(&cfg(&labels, &series, &empty)).is_empty());
    }

    #[test]
    fn perf_rendering_a_large_dual_panel_spiral_stays_fast() {
        let (labels, series, series2) = synth(300, 3);
        let start = std::time::Instant::now();
        let html = render(&cfg(&labels, &series, &series2));
        let elapsed = start.elapsed();
        assert!(elapsed.as_millis() < 200, "rendering took too long: {elapsed:?}");
        assert!(!html.is_empty());
    }
}
