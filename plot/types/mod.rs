pub mod bar;
pub mod line;
pub mod scatter;

use std::collections::HashMap;
use std::sync::LazyLock;

pub use bar::{render_bars, BarRenderContext};
pub use line::{render_lines, LineRenderContext};
pub use scatter::{render_points, ScatterRenderContext};

pub struct PlotRegistry {
    renderers: HashMap<u8, &'static str>,
}

impl PlotRegistry {
    fn new() -> Self {
        let mut renderers = HashMap::new();
        renderers.insert(0, "line");
        renderers.insert(1, "scatter");
        renderers.insert(2, "bar");
        
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

pub fn render_plot_by_type(
    chart_type: u8,
    visible_count: usize,
    ctx: BarRenderContext,
) {
    match chart_type {
        0 if visible_count > 1 => {
            render_lines(LineRenderContext {
                painter: ctx.painter,
                plot_rect: ctx.plot_rect,
                colors: ctx.colors,
                hovered_idx: ctx.hovered_idx,
                values: ctx.values,
                max_val: ctx.max_val,
                visible_indices: ctx.visible_indices,
                vertical: ctx.vertical,
            });
        }
        2 => {
            render_bars(ctx);
        }
        _ => {
            render_points(ScatterRenderContext {
                painter: ctx.painter,
                plot_rect: ctx.plot_rect,
                colors: ctx.colors,
                hovered_idx: ctx.hovered_idx,
                values: ctx.values,
                max_val: ctx.max_val,
                visible_indices: ctx.visible_indices,
                vertical: ctx.vertical,
            });
        }
    }
}
