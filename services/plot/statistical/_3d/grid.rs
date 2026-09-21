use super::super::bar::Bar3DBlock;
use std::f64::consts::{FRAC_PI_2, TAU};

pub struct CellField<'a> {
    pub n_rows: usize,
    pub n_cols: usize,
    pub heights: &'a [f64],
    pub tones: &'a [f64],
    pub classes: &'a [usize],
}

impl<'a> CellField<'a> {
    pub fn cells(&self) -> impl Iterator<Item = Cell> + '_ {
        (0..self.n_rows)
            .flat_map(move |r| (0..self.n_cols).map(move |c| (r, c)))
            .filter_map(move |(r, c)| {
                let idx = r * self.n_cols + c;
                self.heights.get(idx).map(|&height| Cell {
                    row: r,
                    col: c,
                    height,
                    tone: self.tones.get(idx).copied(),
                    class: self.classes.get(idx).copied().unwrap_or(idx),
                })
            })
    }
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
    pub height: f64,
    pub tone: Option<f64>,
    pub class: usize,
}

impl Cell {
    pub fn block(&self, cx: f64, cy: f64, hw: f64, hd: f64) -> Bar3DBlock {
        let block = Bar3DBlock::new(cx, cy, 0.0, self.height, hw, hd, self.class);
        match self.tone {
            Some(tone) => block.with_tone(tone),
            None => block,
        }
    }
}

pub fn spans(weights: &[f64], n: usize) -> Vec<(f64, f64)> {
    let usable = weights.len() == n && weights.iter().all(|w| w.is_finite() && *w > 0.0);
    let mut cursor = 0.0;
    (0..n)
        .map(|i| {
            let size = if usable { weights[i] } else { 1.0 };
            let centre = cursor + size / 2.0;
            cursor += size;
            (centre, size)
        })
        .collect()
}

pub fn rect_cells(
    field: &CellField,
    col_weights: &[f64],
    row_weights: &[f64],
    row_shift: &[f64],
    fill: f64,
) -> Vec<Bar3DBlock> {
    let cols = spans(col_weights, field.n_cols);
    let rows = spans(row_weights, field.n_rows);
    field
        .cells()
        .map(|cell| {
            let (cx, cw) = cols[cell.col];
            let (cy, rh) = rows[cell.row];
            let shift = row_shift.get(cell.row).copied().unwrap_or(0.0);
            cell.block(cx, cy + shift, cw * fill / 2.0, rh * fill / 2.0)
        })
        .collect()
}

pub fn bubble_cells(field: &CellField, fill: f64) -> Vec<Bar3DBlock> {
    field
        .cells()
        .map(|cell| {
            let scale = cell.tone.unwrap_or(1.0).abs().max(0.12).sqrt();
            let half = fill * 0.5 * scale;
            cell.block(cell.col as f64 + 0.5, cell.row as f64 + 0.5, half, half)
        })
        .collect()
}

pub fn radial_cells(field: &CellField, inner: f64, ring_step: f64, fill: f64) -> Vec<Bar3DBlock> {
    let sectors = field.n_cols.max(1) as f64;
    field
        .cells()
        .map(|cell| {
            let theta = -FRAC_PI_2 + TAU * cell.col as f64 / sectors;
            let radius = inner + ring_step * cell.row as f64;
            let arc = TAU * radius / sectors;
            let half = ring_step.min(arc) * 0.5 * fill;
            cell.block(radius * theta.cos(), radius * theta.sin(), half, half)
        })
        .collect()
}

pub fn hex_cells(field: &CellField, fill: f64) -> Vec<Bar3DBlock> {
    let row_step = 3.0_f64.sqrt() / 2.0;
    field
        .cells()
        .map(|cell| {
            let stagger = if cell.row % 2 == 1 { 0.5 } else { 0.0 };
            let half = 0.5 * fill;
            cell.block(cell.col as f64 + stagger, cell.row as f64 * row_step, half, half * row_step)
        })
        .collect()
}

pub fn ridge_cells(field: &CellField, row_gap: f64, fill: f64, thickness: f64) -> Vec<Bar3DBlock> {
    field
        .cells()
        .map(|cell| cell.block(cell.col as f64, cell.row as f64 * row_gap, 0.5 * fill, thickness))
        .collect()
}

pub fn group_offsets(groups: &[usize], gap: f64) -> Vec<f64> {
    let mut shift = 0.0;
    groups
        .iter()
        .enumerate()
        .map(|(i, &g)| {
            if i > 0 && groups[i - 1] != g {
                shift += gap;
            }
            shift
        })
        .collect()
}

pub fn leading_groups<T: AsRef<str>>(labels: &[T], separator: &str) -> Vec<usize> {
    let mut seen: Vec<&str> = Vec::new();
    labels
        .iter()
        .map(|label| {
            let head = label.as_ref().split(separator).next().unwrap_or("");
            match seen.iter().position(|s| *s == head) {
                Some(p) => p,
                None => {
                    seen.push(head);
                    seen.len() - 1
                }
            }
        })
        .collect()
}

pub fn margin_cells(
    row_totals: &[f64],
    col_totals: &[f64],
    n_rows: usize,
    n_cols: usize,
    gap: f64,
    fill: f64,
    tone: f64,
) -> Vec<Bar3DBlock> {
    let half = 0.5 * fill;
    let along_rows = row_totals.iter().take(n_rows).enumerate().map(move |(r, &v)| {
        Bar3DBlock::new(n_cols as f64 + gap, r as f64 + 0.5, 0.0, v, half, half, n_rows * n_cols + r)
            .with_tone(tone)
    });
    let along_cols = col_totals.iter().take(n_cols).enumerate().map(move |(c, &v)| {
        Bar3DBlock::new(c as f64 + 0.5, n_rows as f64 + gap, 0.0, v, half, half, n_rows * n_cols + n_rows + c)
            .with_tone(tone)
    });
    along_rows.chain(along_cols).collect()
}

pub fn row_sums(n_rows: usize, n_cols: usize, cells: &[f64]) -> Vec<f64> {
    (0..n_rows)
        .map(|r| (0..n_cols).filter_map(|c| cells.get(r * n_cols + c)).filter(|v| v.is_finite()).sum())
        .collect()
}

pub fn col_sums(n_rows: usize, n_cols: usize, cells: &[f64]) -> Vec<f64> {
    (0..n_cols)
        .map(|c| (0..n_rows).filter_map(|r| cells.get(r * n_cols + c)).filter(|v| v.is_finite()).sum())
        .collect()
}

pub fn reordered(n_rows: usize, n_cols: usize, cells: &[f64], row_order: &[usize], col_order: &[usize]) -> Vec<f64> {
    let rows: Vec<usize> = if row_order.len() == n_rows { row_order.to_vec() } else { (0..n_rows).collect() };
    let cols: Vec<usize> = if col_order.len() == n_cols { col_order.to_vec() } else { (0..n_cols).collect() };
    rows.iter()
        .flat_map(|&r| cols.iter().map(move |&c| (r, c)))
        .map(|(r, c)| cells.get(r * n_cols + c).copied().unwrap_or(0.0))
        .collect()
}

pub fn upsampled(n_rows: usize, n_cols: usize, cells: &[f64], factor: usize) -> (usize, usize, Vec<f64>) {
    if factor <= 1 || n_rows < 2 || n_cols < 2 || cells.len() < n_rows * n_cols {
        return (n_rows, n_cols, cells.to_vec());
    }
    let out_rows = (n_rows - 1) * factor + 1;
    let out_cols = (n_cols - 1) * factor + 1;
    let at = |r: usize, c: usize| cells[r * n_cols + c];
    let out = (0..out_rows)
        .flat_map(|orow| (0..out_cols).map(move |ocol| (orow, ocol)))
        .map(|(orow, ocol)| {
            let fr = orow as f64 / factor as f64;
            let fc = ocol as f64 / factor as f64;
            let (r0, c0) = (fr.floor() as usize, fc.floor() as usize);
            let (r1, c1) = ((r0 + 1).min(n_rows - 1), (c0 + 1).min(n_cols - 1));
            let (tr, tc) = (fr - r0 as f64, fc - c0 as f64);
            let top = at(r0, c0) * (1.0 - tc) + at(r0, c1) * tc;
            let bottom = at(r1, c0) * (1.0 - tc) + at(r1, c1) * tc;
            top * (1.0 - tr) + bottom * tr
        })
        .collect();
    (out_rows, out_cols, out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn field<'a>(n_rows: usize, n_cols: usize, heights: &'a [f64], tones: &'a [f64]) -> CellField<'a> {
        CellField { n_rows, n_cols, heights, tones, classes: &[] }
    }

    #[test]
    fn rect_cells_place_one_block_per_cell_with_weighted_spans() {
        let heights = [1.0, 2.0, 3.0, 4.0];
        let f = field(2, 2, &heights, &[]);
        let blocks = rect_cells(&f, &[1.0, 3.0], &[], &[], 1.0);
        assert_eq!(blocks.len(), 4);
        assert_eq!((blocks[0].cx, blocks[1].cx), (0.5, 2.5));
        assert_eq!((blocks[1].hw, blocks[0].hw), (1.5, 0.5));
        assert_eq!(blocks[3].z1, 4.0);
    }

    #[test]
    fn tones_travel_with_their_cells() {
        let heights = [1.0, 2.0];
        let tones = [0.25, 0.75];
        let f = field(1, 2, &heights, &tones);
        let blocks = bubble_cells(&f, 1.0);
        assert_eq!(blocks[0].tone, Some(0.25));
        assert_eq!(blocks[1].tone, Some(0.75));
        assert!(blocks[1].hw > blocks[0].hw);
    }

    #[test]
    fn radial_cells_wrap_the_columns_around_a_circle() {
        let heights = vec![1.0; 8];
        let f = field(2, 4, &heights, &[]);
        let blocks = radial_cells(&f, 2.0, 1.0, 0.8);
        assert_eq!(blocks.len(), 8);
        let radii: Vec<f64> = blocks.iter().map(|b| (b.cx * b.cx + b.cy * b.cy).sqrt()).collect();
        assert!(radii[..4].iter().all(|r| (r - 2.0).abs() < 1e-9));
        assert!(radii[4..].iter().all(|r| (r - 3.0).abs() < 1e-9));
    }

    #[test]
    fn hex_cells_stagger_every_other_row() {
        let heights = vec![1.0; 4];
        let f = field(2, 2, &heights, &[]);
        let blocks = hex_cells(&f, 0.9);
        assert_eq!(blocks[0].cx, 0.0);
        assert_eq!(blocks[2].cx, 0.5);
        assert!(blocks[2].cy > blocks[0].cy);
    }

    #[test]
    fn leading_groups_number_labels_by_first_token() {
        let labels = ["a::x", "a::y", "b::x", "a::z"];
        assert_eq!(leading_groups(&labels, "::"), vec![0, 0, 1, 0]);
        assert_eq!(group_offsets(&[0, 0, 1, 1, 2], 0.5), vec![0.0, 0.0, 0.5, 0.5, 1.0]);
    }

    #[test]
    fn margins_sit_beyond_the_grid_on_both_axes() {
        let blocks = margin_cells(&[3.0, 7.0], &[4.0, 6.0, 0.0], 2, 3, 1.0, 0.8, 1.0);
        assert_eq!(blocks.len(), 5);
        assert!(blocks[..2].iter().all(|b| b.cx == 4.0));
        assert!(blocks[2..].iter().all(|b| b.cy == 3.0));
    }

    #[test]
    fn sums_reorder_and_upsample_keep_the_grid_consistent() {
        let cells = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        assert_eq!(row_sums(2, 3, &cells), vec![6.0, 15.0]);
        assert_eq!(col_sums(2, 3, &cells), vec![5.0, 7.0, 9.0]);
        assert_eq!(reordered(2, 3, &cells, &[1, 0], &[2, 1, 0]), vec![6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
        let (r, c, up) = upsampled(2, 3, &cells, 2);
        assert_eq!((r, c, up.len()), (3, 5, 15));
        assert_eq!(up[0], 1.0);
        assert_eq!(up[14], 6.0);
        assert_eq!(up[1], 1.5);
    }
}
