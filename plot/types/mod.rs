pub mod bar;
pub mod line;
pub mod scatter;
pub mod _3d;

use std::collections::HashMap;
use std::sync::LazyLock;

pub use bar::render_bars;
pub use line::render_lines;
pub use scatter::render_points;
pub use _3d::{render_plot_3d_by_type, Bar3DRenderContext, Line3DRenderContext, Scatter3DRenderContext};
pub use _3d::*;

pub struct PlotRenderContext<'a> {
    pub painter: &'a egui::Painter,
    pub plot_rect: egui::Rect,
    pub colors: &'a [egui::Color32],
    pub hovered_idx: Option<usize>,
    pub values: &'a [f64],
    pub max_val: f64,
    pub visible_indices: &'a [usize],
    pub vertical: bool,
    pub labels: &'a [String],
}

pub struct PlotRegistry {
    renderers: HashMap<u8, &'static str>,
}

impl PlotRegistry {
    fn new() -> Self {
        let mut renderers = HashMap::new();
        renderers.insert(0, "line");
        renderers.insert(1, "scatter");
        renderers.insert(2, "bar");
        renderers.insert(3, "line_3d");
        renderers.insert(4, "scatter_3d");
        renderers.insert(5, "bar_3d");
        
        Self { renderers }
    }
    
    pub fn get(&self, kind: u8) -> Option<&'static str> {
        self.renderers.get(&kind).copied()
    }
    
    pub fn list(&self) -> Vec<(u8, &'static str)> {
        let mut items: Vec<_> = self.renderers.iter()
            .map(|(&k, &v)| (k, v))
            .collect();
        items.sort_by_key(|&(k, _)| k);
        items
    }
}

pub static PLOT_REGISTRY: LazyLock<PlotRegistry> = LazyLock::new(PlotRegistry::new);

pub fn render_plot_by_type(chart_type: u8, ctx: PlotRenderContext) {
    match chart_type {
        0 => render_lines(ctx),
        2 => render_bars(ctx),
        _ => render_points(ctx),
    }
}
