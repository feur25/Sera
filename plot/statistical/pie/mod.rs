pub mod variant;
pub mod config;
pub mod basic;
pub mod subplots;

pub use variant::PieVariant;
pub use config::{Pie, PieConfig};

use crate::html::hover::{slots_to_json, build_chart_html};

pub fn render_pie_html(cfg: &PieConfig) -> String {
    use PieVariant::*;
    let svg = match cfg.variant {
        Basic => basic::render_single(cfg, 0.0),
        Donut => {
            let d = if cfg.donut > 0.0 { cfg.donut } else { 0.55 };
            basic::render_single(cfg, d)
        }
        Exploded => {
            if !cfg.pull.is_empty() {
                basic::render_single(cfg, cfg.donut)
            } else {
                let n = cfg.values.len();
                let mut auto = vec![0.0_f64; n];
                if let Some((idx, _)) = cfg.values.iter().enumerate()
                    .fold(None::<(usize, f64)>, |acc, (i, &v)| match acc {
                        None => Some((i, v)),
                        Some((_, mv)) if v > mv => Some((i, v)),
                        x => x,
                    }) {
                    auto[idx] = 0.18;
                }
                let saved_pull = cfg.pull;
                let temp = PieConfig { pull: &auto, ..clone_cfg(cfg, saved_pull) };
                basic::render_single(&temp, cfg.donut)
            }
        }
        Subplots => subplots::render_subplots(cfg),
        Proportional => {
            let temp = PieConfig { proportional: true, ..clone_cfg(cfg, cfg.pull) };
            subplots::render_subplots(&temp)
        }
    };
    build_chart_html(cfg.title, &svg, &slots_to_json(cfg.hover))
}

fn clone_cfg<'a>(cfg: &'a PieConfig<'a>, pull: &'a [f64]) -> PieConfig<'a> {
    PieConfig {
        variant: cfg.variant,
        title: cfg.title,
        x_label: cfg.x_label,
        y_label: cfg.y_label,
        gridlines: cfg.gridlines,
        sort_order: cfg.sort_order,
        hover: cfg.hover,
        legend_position: cfg.legend_position,
        width: cfg.width,
        height: cfg.height,
        labels: cfg.labels,
        values: cfg.values,
        donut: cfg.donut,
        show_pct: cfg.show_pct,
        min_label_frac: cfg.min_label_frac,
        palette: cfg.palette,
        pull,
        series: cfg.series,
        subplot_titles: cfg.subplot_titles,
        subplot_cols: cfg.subplot_cols,
        proportional: cfg.proportional,
    }
}
