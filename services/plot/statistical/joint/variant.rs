crate::plot_family! {
    pub enum JointVariant default Hexbin family "joint" {
        Hexbin     => "hexbin" | "hexbin_marginal" | "hexbin_marginals",
        HeatGrid   => "heat_grid" | "heat_scatter" | "heatscatter" | "density_heat" | "joint_histogram" | "histogram2d" | "hist2d",
        KdeHeat    => "kde_heat" | "layered_bivariate" | "kde_scatter" | "layered" | "joint_kde" | "kde_joint" | "bivariate_kde" | "kde_smooth" | "smooth_bivariate_kde" | "smooth_kde",
        KdeContour => "kde_contour" | "multiple_bivariate_kde" | "kde_multi" | "overlapping_kde",
        Scatter    => "scatter" | "marginal_ticks" | "rug_marginal" | "rug" | "regression_marginals" | "regression_joint" | "joint_regression",
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum JointMarginal {
    Histogram,
    Kde,
    Rug,
    None,
}

impl JointMarginal {
    pub fn from_str(s: &str) -> Self {
        match s {
            "kde" | "density" => JointMarginal::Kde,
            "rug" | "ticks" | "rugplot" => JointMarginal::Rug,
            "none" | "off" | "hidden" => JointMarginal::None,
            _ => JointMarginal::Histogram,
        }
    }
}

pub struct Preset {
    pub panel: &'static str,
    pub marginal: JointMarginal,
    pub show_points: bool,
    pub show_regression: bool,
}

pub fn resolve_preset(name: &str) -> Option<Preset> {
    Some(match name {
        "hexbin_marginal" | "hexbin_marginals" => Preset {
            panel: "hexbin",
            marginal: JointMarginal::Histogram,
            show_points: false,
            show_regression: false,
        },
        "heat_scatter" | "heatscatter" | "density_heat" | "joint_histogram" | "histogram2d" | "hist2d" => Preset {
            panel: "heat_grid",
            marginal: JointMarginal::Histogram,
            show_points: false,
            show_regression: false,
        },
        "layered_bivariate" | "kde_scatter" | "layered" => Preset {
            panel: "kde_heat",
            marginal: JointMarginal::Kde,
            show_points: true,
            show_regression: false,
        },
        "joint_kde" | "kde_joint" | "bivariate_kde" => Preset {
            panel: "kde_heat",
            marginal: JointMarginal::Kde,
            show_points: false,
            show_regression: false,
        },
        "kde_smooth" | "smooth_bivariate_kde" | "smooth_kde" => Preset {
            panel: "kde_heat",
            marginal: JointMarginal::Histogram,
            show_points: false,
            show_regression: false,
        },
        "multiple_bivariate_kde" | "kde_multi" | "overlapping_kde" => Preset {
            panel: "kde_contour",
            marginal: JointMarginal::None,
            show_points: false,
            show_regression: false,
        },
        "marginal_ticks" | "rug_marginal" | "rug" => Preset {
            panel: "scatter",
            marginal: JointMarginal::Rug,
            show_points: false,
            show_regression: false,
        },
        "regression_marginals" | "regression_joint" | "joint_regression" => Preset {
            panel: "scatter",
            marginal: JointMarginal::Histogram,
            show_points: false,
            show_regression: true,
        },
        _ => return None,
    })
}
