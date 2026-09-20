use super::common::{finite_minmax, hierarchical_leaf_order, map_value_to_t, quantize_t};
use super::config::HeatmapConfig;
use super::unequal::{default_x_widths, default_y_heights};
use super::variant::HeatmapVariant;
use crate::plot::statistical::_3d::grid::{
    bubble_cells, col_sums, decimated, group_offsets, hex_cells, leading_groups, margin_cells, radial_cells,
    rect_cells, reordered, ridge_cells, row_sums, upsampled, CellField,
};
use crate::plot::statistical::bar::Bar3DBlock;

pub const HEIGHT_RATIO: f64 = 0.6;
const HEIGHT_FLOOR: f64 = 0.06;
const SMOOTH_FACTOR: usize = 3;
const SMOOTH_MAX_CELLS: usize = 3600;
const MAX_CELLS: usize = 4096;
const GROUP_SEPARATOR: &str = "::";

#[derive(Clone, Copy)]
enum Shape {
    Rect,
    Bubble,
    Radial,
    Hex,
    Ridge,
}

#[derive(Clone, Copy)]
enum Margin {
    Off,
    Adjacent,
    Detached,
}

#[derive(Clone, Copy)]
struct Recipe {
    shape: Shape,
    log: bool,
    diverging: bool,
    steps: usize,
    smooth: bool,
    weighted: bool,
    cluster: bool,
    categorical: bool,
    diagonal: bool,
    grouped: bool,
    margin: Margin,
    cmap: &'static str,
}

impl Recipe {
    const fn on(shape: Shape) -> Self {
        Self {
            shape,
            log: false,
            diverging: false,
            steps: 0,
            smooth: false,
            weighted: false,
            cluster: false,
            categorical: false,
            diagonal: false,
            grouped: false,
            margin: Margin::Off,
            cmap: "",
        }
    }

    const fn logarithmic(mut self) -> Self {
        self.log = true;
        self
    }

    const fn signed(mut self) -> Self {
        self.diverging = true;
        self
    }

    const fn stepped(mut self, steps: usize) -> Self {
        self.steps = steps;
        self
    }

    const fn blended(mut self) -> Self {
        self.smooth = true;
        self
    }

    const fn weighted(mut self) -> Self {
        self.weighted = true;
        self
    }

    const fn clustered(mut self) -> Self {
        self.cluster = true;
        self
    }

    const fn categorical(mut self) -> Self {
        self.categorical = true;
        self
    }

    const fn diagonal(mut self) -> Self {
        self.diagonal = true;
        self
    }

    const fn grouped(mut self) -> Self {
        self.grouped = true;
        self
    }

    const fn margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        self
    }

    const fn colormap(mut self, cmap: &'static str) -> Self {
        self.cmap = cmap;
        self
    }
}

fn recipe(variant: HeatmapVariant) -> Recipe {
    use HeatmapVariant::*;
    let rect = Recipe::on(Shape::Rect);
    match variant {
        Basic | Annotated => rect,
        Categorical => rect.categorical(),
        Unequal => rect.weighted(),
        Log => rect.logarithmic(),
        Discrete => rect.stepped(5),
        Correlation => rect.signed(),
        Density | Temporal => rect.blended().colormap("viridis"),
        Contour => rect.blended().stepped(5),
        Cluster => rect.clustered(),
        Bubble => Recipe::on(Shape::Bubble),
        Marginal => rect.margin(Margin::Detached),
        Confusion => rect.diagonal(),
        Pivot => rect.margin(Margin::Adjacent),
        Polar => Recipe::on(Shape::Radial),
        RadialCluster => Recipe::on(Shape::Radial).clustered(),
        HexGrid => Recipe::on(Shape::Hex),
        Horizon => Recipe::on(Shape::Ridge),
        Moods => rect.grouped(),
    }
}

pub fn colormap(variant: HeatmapVariant) -> &'static str {
    recipe(variant).cmap
}

fn dims(cfg: &HeatmapConfig) -> (usize, usize) {
    let n_rows = cfg.row_labels.len();
    if n_rows == 0 || cfg.flat_matrix.is_empty() {
        return (0, 0);
    }
    let n_cols = if cfg.col_labels.is_empty() {
        (cfg.flat_matrix.len() / n_rows).max(1)
    } else {
        cfg.col_labels.len()
    };
    ((cfg.flat_matrix.len() / n_cols).clamp(1, n_rows), n_cols)
}

fn cells_of(cfg: &HeatmapConfig, n_rows: usize, n_cols: usize) -> Vec<f64> {
    (0..n_rows * n_cols)
        .map(|i| cfg.flat_matrix.get(i).copied().filter(|v| v.is_finite()).unwrap_or(0.0))
        .collect()
}

fn line_vectors(n_rows: usize, n_cols: usize, cells: &[f64]) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let rows = (0..n_rows).map(|r| (0..n_cols).map(|c| cells[r * n_cols + c]).collect()).collect();
    let cols = (0..n_cols).map(|c| (0..n_rows).map(|r| cells[r * n_cols + c]).collect()).collect();
    (rows, cols)
}

fn smoothing_factor(n_rows: usize, n_cols: usize) -> usize {
    (1..=SMOOTH_FACTOR)
        .rev()
        .find(|f| ((n_rows.saturating_sub(1)) * f + 1) * ((n_cols.saturating_sub(1)) * f + 1) <= SMOOTH_MAX_CELLS)
        .unwrap_or(1)
}

fn height_of(tone: f64, diverging: bool) -> f64 {
    if diverging {
        (tone - 0.5) * 2.0
    } else {
        HEIGHT_FLOOR + (1.0 - HEIGHT_FLOOR) * tone
    }
}

fn tone_of(plan: Recipe, cfg: &HeatmapConfig, v: f64, bounds: (f64, f64)) -> f64 {
    let steps = match (plan.steps, cfg.discrete_steps) {
        (0, _) => 0,
        (_, 0) => plan.steps,
        (_, chosen) => chosen,
    };
    quantize_t(map_value_to_t(v, bounds.0, bounds.1, plan.log, plan.diverging), steps)
}

pub fn layout_3d(cfg: &HeatmapConfig) -> Vec<Bar3DBlock> {
    let plan = recipe(cfg.variant);
    let (mut n_rows, mut n_cols) = dims(cfg);
    if n_rows == 0 || n_cols == 0 {
        return Vec::new();
    }
    let mut cells = cells_of(cfg, n_rows, n_cols);
    (n_rows, n_cols, cells) = decimated(n_rows, n_cols, &cells, MAX_CELLS);
    if plan.cluster {
        let (row_vectors, col_vectors) = line_vectors(n_rows, n_cols, &cells);
        cells = reordered(
            n_rows,
            n_cols,
            &cells,
            &hierarchical_leaf_order(&row_vectors),
            &hierarchical_leaf_order(&col_vectors),
        );
    }
    if plan.smooth {
        let factor = smoothing_factor(n_rows, n_cols);
        (n_rows, n_cols, cells) = upsampled(n_rows, n_cols, &cells, factor);
    }
    let bounds = finite_minmax(&cells);
    let tones: Vec<f64> = cells.iter().map(|&v| tone_of(plan, cfg, v, bounds)).collect();
    let heights: Vec<f64> = tones.iter().map(|&t| height_of(t, plan.diverging)).collect();
    let classes: Vec<usize> = if plan.categorical {
        cells.iter().map(|v| v.round().max(0.0) as usize).collect()
    } else {
        Vec::new()
    };
    let shown_tones: Vec<f64> = match (plan.categorical, plan.diagonal) {
        (true, _) => Vec::new(),
        (false, true) => tones
            .iter()
            .enumerate()
            .map(|(i, &t)| if i / n_cols == i % n_cols { 1.0 } else { t * 0.55 })
            .collect(),
        (false, false) => tones.clone(),
    };
    let field = CellField {
        n_rows,
        n_cols,
        heights: &heights,
        tones: &shown_tones,
        classes: &classes,
    };
    let mut blocks = match plan.shape {
        Shape::Rect => {
            let (col_w, row_h) = if plan.weighted {
                (
                    if cfg.x_widths.len() == n_cols { cfg.x_widths.to_vec() } else { default_x_widths(n_cols) },
                    if cfg.y_heights.len() == n_rows { cfg.y_heights.to_vec() } else { default_y_heights(n_rows) },
                )
            } else {
                (Vec::new(), Vec::new())
            };
            let shift = if plan.grouped {
                group_offsets(&leading_groups(cfg.row_labels, GROUP_SEPARATOR), 0.8)
            } else {
                Vec::new()
            };
            rect_cells(&field, &col_w, &row_h, &shift, if plan.smooth { 1.0 } else { 0.9 })
        }
        Shape::Bubble => bubble_cells(&field, 0.9),
        Shape::Radial => radial_cells(&field, 1.6, 1.0, 0.85),
        Shape::Hex => hex_cells(&field, 0.92),
        Shape::Ridge => ridge_cells(&field, 1.6, 1.0, 0.22),
    };
    let (gap, tone) = match plan.margin {
        Margin::Off => return blocks,
        Margin::Adjacent => (0.5, 1.0),
        Margin::Detached => (1.5, 0.5),
    };
    let ceiling = heights.iter().cloned().fold(f64::MIN, f64::max).max(HEIGHT_FLOOR);
    let totals = (row_sums(n_rows, n_cols, &cells), col_sums(n_rows, n_cols, &cells));
    let peak = totals.0.iter().chain(totals.1.iter()).cloned().fold(f64::MIN, f64::max).max(1e-12);
    let scale = ceiling / peak;
    let scaled = |sums: &[f64]| sums.iter().map(|v| v * scale).collect::<Vec<f64>>();
    blocks.extend(margin_cells(&scaled(&totals.0), &scaled(&totals.1), n_rows, n_cols, gap, 0.9, tone));
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labels(n: usize, prefix: &str) -> Vec<String> {
        (0..n).map(|i| format!("{prefix}{i}")).collect()
    }

    fn blocks_for(variant: HeatmapVariant, n_rows: usize, n_cols: usize) -> Vec<Bar3DBlock> {
        let rows = labels(n_rows, "r");
        let cols = labels(n_cols, "c");
        let values: Vec<f64> = (0..n_rows * n_cols).map(|i| ((i * 7) % 11) as f64 + 1.0).collect();
        layout_3d(&HeatmapConfig {
            variant,
            row_labels: &rows,
            col_labels: &cols,
            flat_matrix: &values,
            ..HeatmapConfig::default()
        })
    }

    #[test]
    fn every_heatmap_variant_lays_out_a_column_per_cell() {
        for variant in HeatmapVariant::all() {
            let blocks = blocks_for(*variant, 4, 5);
            assert!(blocks.len() >= 20, "{} must draw at least one column per cell", variant.name());
        }
    }

    #[test]
    fn oversized_grids_are_decimated_before_they_become_blocks() {
        let blocks = blocks_for(HeatmapVariant::Basic, 200, 200);
        assert!(blocks.len() <= MAX_CELLS);
        assert!(blocks.len() > MAX_CELLS / 4);
    }

    #[test]
    fn smoothing_variants_densify_the_grid() {
        assert!(blocks_for(HeatmapVariant::Density, 4, 5).len() > 20);
        assert_eq!(blocks_for(HeatmapVariant::Basic, 4, 5).len(), 20);
    }

    #[test]
    fn marginal_and_pivot_add_a_total_per_row_and_column() {
        assert_eq!(blocks_for(HeatmapVariant::Marginal, 4, 5).len(), 20 + 4 + 5);
        assert_eq!(blocks_for(HeatmapVariant::Pivot, 4, 5).len(), 20 + 4 + 5);
    }

    #[test]
    fn correlation_columns_go_below_zero_for_negative_values() {
        let rows = labels(3, "r");
        let values = [-1.0, -0.5, 0.2, 0.4, 1.0, -0.8, 0.0, 0.6, -0.3];
        let blocks = layout_3d(&HeatmapConfig {
            variant: HeatmapVariant::Correlation,
            row_labels: &rows,
            col_labels: &rows,
            flat_matrix: &values,
            ..HeatmapConfig::default()
        });
        assert!(blocks.iter().any(|b| b.z1 < 0.0));
        assert!(blocks.iter().any(|b| b.z1 > 0.0));
    }

    #[test]
    fn discrete_variant_quantises_tones_to_the_step_count() {
        let mut tones: Vec<u32> = blocks_for(HeatmapVariant::Discrete, 5, 5)
            .iter()
            .filter_map(|b| b.tone.map(|t| (t * 1000.0) as u32))
            .collect();
        tones.sort();
        tones.dedup();
        assert!(tones.len() <= 5);
    }

    #[test]
    fn only_the_categorical_variant_drops_tones_for_palette_classes() {
        assert!(blocks_for(HeatmapVariant::Categorical, 3, 3).iter().all(|b| b.tone.is_none()));
        assert!(blocks_for(HeatmapVariant::Basic, 3, 3).iter().all(|b| b.tone.is_some()));
    }
}
