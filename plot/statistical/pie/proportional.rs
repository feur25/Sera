use super::config::PieConfig;

#[crate::chart_demo("labels=[\"Apple\",\"Banana\",\"Cherry\",\"Date\",\"Fig\"], values=[40,25,20,10,5]")]

pub fn render(cfg: &PieConfig) -> String {
    let forced = PieConfig { proportional: true, ..clone_cfg(cfg) };
    super::subplots::render(&forced)
}

fn clone_cfg<'a>(cfg: &'a PieConfig<'a>) -> PieConfig<'a> {
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
        pull: cfg.pull,
        series: cfg.series,
        subplot_titles: cfg.subplot_titles,
        subplot_cols: cfg.subplot_cols,
        proportional: cfg.proportional,
        center_text: cfg.center_text,
        center_subtext: cfg.center_subtext,
        secondary_values: cfg.secondary_values,
        secondary_labels: cfg.secondary_labels,
        pattern: cfg.pattern,
    }
}

