use super::super::bar::Bar3DBlock;

pub const PARTS: usize = 2;
const THICK_RATIO: f64 = 0.06;
const ROW_HD: f64 = 0.32;
const FILL_HD: f64 = 0.26;
const MARK_SPREAD: f64 = 0.32;
const MARK_LIFT: f64 = 0.9;
const MIN_WIDTH: f64 = 0.004;
const FILL_LIFT: f64 = 0.12;
const EPS: f64 = 1e-9;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Part {
    Span,
    Fill,
}

impl Part {
    pub fn class(self, row: usize) -> usize {
        row * PARTS + self as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plan {
    pub progress: bool,
    pub milestones: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Task {
    pub start: f64,
    pub end: f64,
    pub progress: f64,
    pub tone: f64,
}

impl Task {
    fn low(&self) -> f64 {
        self.start.min(self.end)
    }

    fn high(&self) -> f64 {
        self.start.max(self.end)
    }

    fn is_milestone(&self) -> bool {
        (self.end - self.start).abs() < EPS
    }
}

fn extent(tasks: &[Task]) -> f64 {
    let lo = tasks.iter().map(Task::low).fold(f64::INFINITY, f64::min);
    let hi = tasks.iter().map(Task::high).fold(f64::NEG_INFINITY, f64::max);
    (hi - lo).max(EPS)
}

fn diamond(cx: f64, row: usize, z: f64, hw: f64, tall: f64, tone: f64) -> Vec<Bar3DBlock> {
    let (tip, belly) = ((z, z), (z - tall, z + tall));
    vec![
        Bar3DBlock::sloped(cx - hw / 2.0, row as f64, tip, belly, hw / 2.0, ROW_HD, Part::Span.class(row)).with_tone(tone),
        Bar3DBlock::sloped(cx + hw / 2.0, row as f64, belly, tip, hw / 2.0, ROW_HD, Part::Span.class(row)).with_tone(tone),
    ]
}

pub fn timeline(tasks: &[Task], plan: Plan) -> Vec<Bar3DBlock> {
    if tasks.is_empty() {
        return Vec::new();
    }
    let span = extent(tasks);
    let thick = THICK_RATIO * span.min(tasks.len() as f64);
    let mut blocks = Vec::with_capacity(tasks.len() * 2);
    for (row, task) in tasks.iter().enumerate() {
        if plan.milestones && task.is_milestone() {
            let width = MARK_SPREAD * span / tasks.len() as f64;
            blocks.extend(diamond(task.start, row, thick, width.max(span * MIN_WIDTH), thick * MARK_LIFT, task.tone));
            continue;
        }
        let half = ((task.high() - task.low()) / 2.0).max(span * MIN_WIDTH / 2.0);
        let centre = (task.low() + task.high()) / 2.0;
        blocks.push(Bar3DBlock::new(centre, row as f64, 0.0, thick, half, ROW_HD, Part::Span.class(row)).with_tone(task.tone));
        let done = task.progress.clamp(0.0, 1.0);
        if plan.progress && done > 0.0 {
            let reach = (task.high() - task.low()) * done;
            let fill_half = (reach / 2.0).max(span * MIN_WIDTH / 2.0);
            blocks.push(
                Bar3DBlock::new(task.low() + reach / 2.0, row as f64, thick, thick * 2.0, fill_half, FILL_HD, Part::Fill.class(row))
                    .with_tone((task.tone + FILL_LIFT).min(1.0)),
            );
        }
    }
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task(start: f64, end: f64, progress: f64) -> Task {
        Task { start, end, progress, tone: 0.4 }
    }

    const FLAT: Plan = Plan { progress: false, milestones: false };

    #[test]
    fn every_task_is_a_bar_along_the_time_axis_on_its_own_row() {
        let bars = timeline(&[task(0.0, 6.0, 0.0), task(5.0, 14.0, 0.0)], FLAT);
        assert_eq!(bars.len(), 2);
        assert_eq!((bars[0].cx, bars[0].hw, bars[0].cy), (3.0, 3.0, 0.0));
        assert_eq!((bars[1].cx, bars[1].hw, bars[1].cy), (9.5, 4.5, 1.0));
    }

    #[test]
    fn plain_bars_share_one_thickness_so_the_scene_stays_flat() {
        let bars = timeline(&[task(0.0, 6.0, 0.0), task(5.0, 14.0, 0.0)], FLAT);
        assert!(bars.windows(2).all(|w| w[0].z0 == w[1].z0 && w[0].z1 == w[1].z1));
        assert!(bars[0].z1 > 0.0);
    }

    #[test]
    fn reversed_tasks_span_the_same_interval_as_forward_ones() {
        let bars = timeline(&[task(6.0, 0.0, 0.0)], FLAT);
        assert_eq!((bars[0].cx, bars[0].hw), (3.0, 3.0));
    }

    #[test]
    fn progress_raises_a_fill_over_the_completed_part_only() {
        let plan = Plan { progress: true, milestones: false };
        let blocks = timeline(&[task(0.0, 10.0, 0.6), task(0.0, 10.0, 0.0)], plan);
        assert_eq!(blocks.len(), 3);
        let fill = blocks[1];
        assert_eq!((fill.cx, fill.hw), (3.0, 3.0));
        assert_eq!(fill.z0, blocks[0].z1);
        assert!(fill.tone > blocks[0].tone);
        assert_eq!(fill.ci, Part::Fill.class(0));
    }

    #[test]
    fn zero_length_tasks_become_diamonds_only_when_milestones_are_on() {
        let tasks = [task(0.0, 6.0, 0.0), task(18.0, 18.0, 0.0)];
        let plan = Plan { progress: false, milestones: true };
        let blocks = timeline(&tasks, plan);
        assert_eq!(blocks.len(), 3);
        assert!(blocks[1].end.is_some() && blocks[2].end.is_some());
        assert_eq!(timeline(&tasks, FLAT).len(), 2);
    }

    #[test]
    fn instant_tasks_keep_a_visible_width_without_milestones() {
        let bars = timeline(&[task(0.0, 20.0, 0.0), task(5.0, 5.0, 0.0)], FLAT);
        assert!(bars[1].hw > 0.0);
    }

    #[test]
    fn thickness_follows_the_smaller_of_the_time_span_and_the_row_count() {
        let few = timeline(&[task(0.0, 100.0, 0.0), task(0.0, 100.0, 0.0)], FLAT);
        assert!((few[0].z1 - THICK_RATIO * 2.0).abs() < 1e-9);
        let many: Vec<Task> = (0..500).map(|i| task(0.0, 10.0 + (i % 3) as f64, 0.0)).collect();
        assert!((timeline(&many, FLAT)[0].z1 - THICK_RATIO * 12.0).abs() < 1e-9);
    }

    #[test]
    fn classes_number_two_parts_per_row() {
        assert_eq!((Part::Span.class(3), Part::Fill.class(3)), (6, 7));
    }

    #[test]
    fn no_tasks_draw_nothing() {
        assert!(timeline(&[], FLAT).is_empty());
    }
}
