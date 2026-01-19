use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use egui::{Color32, Pos2, Rect, Painter};

pub trait Renderable {
    fn render(&self, painter: &Painter, pos: Pos2, size: f32);
}

pub trait Colorizable {
    fn color(&self) -> Color32;
    fn hover_color(&self) -> Color32;
}

pub trait Hoverable {
    fn is_hovered(&self, mouse_pos: Pos2, threshold: f32) -> bool;
}

pub trait Labeled {
    fn label(&self) -> String;
}

#[derive(Clone)]
pub struct DataPoint<T: Clone> {
    pub x: f64,
    pub y: f64,
    pub value: T,
    pub metadata: HashMap<String, String>,
}

impl<T: Clone> DataPoint<T> {
    pub fn new(x: f64, y: f64, value: T) -> Self {
        Self {
            x,
            y,
            value,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), val.into());
        self
    }

    pub fn add_metadata(&mut self, key: impl Into<String>, val: impl Into<String>) {
        self.metadata.insert(key.into(), val.into());
    }
}

pub struct RenderPayload<T: Clone> {
    pub points: Vec<DataPoint<T>>,
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
    pub labels: Vec<String>,
}

impl<T: Clone> Default for RenderPayload<T> {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            min_x: 0.0,
            max_x: 1.0,
            min_y: 0.0,
            max_y: 1.0,
            labels: Vec::new(),
        }
    }
}

impl<T: Clone> RenderPayload<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_point(mut self, point: DataPoint<T>) -> Self {
        self.points.push(point);
        self
    }

    pub fn with_bounds(mut self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self {
        self.min_x = min_x;
        self.max_x = max_x;
        self.min_y = min_y;
        self.max_y = max_y;
        self
    }

    pub fn with_labels(mut self, labels: Vec<String>) -> Self {
        self.labels = labels;
        self
    }

    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        (self.min_x, self.max_x, self.min_y, self.max_y)
    }
}

pub fn normalize_to_rect<T: Clone>(
    payload: &RenderPayload<T>,
    rect: Rect,
) -> Vec<(Pos2, &DataPoint<T>)> {
    let (min_x, max_x, min_y, max_y) = payload.bounds();
    let range_x = max_x - min_x;
    let range_y = max_y - min_y;
    
    payload.points.iter().map(|pt| {
        let norm_x = if range_x > 0.0 { (pt.x - min_x) / range_x } else { 0.5 };
        let norm_y = if range_y > 0.0 { (pt.y - min_y) / range_y } else { 0.5 };
        
        let screen_x = rect.left() + rect.width() * norm_x as f32;
        let screen_y = rect.bottom() - rect.height() * norm_y as f32;
        
        (Pos2::new(screen_x, screen_y), pt)
    }).collect()
}

pub fn apply_to_collection<T: Clone, F>(collection: &[T], mut f: F) -> Vec<T>
where
    F: FnMut(&T) -> T,
{
    collection.iter().map(|item| f(item)).collect()
}

pub fn filter_by_property<T: Clone, F>(collection: &[T], predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    collection.iter().filter(|item| predicate(item)).cloned().collect()
}

pub fn group_by_key<K: Eq + std::hash::Hash + Clone, T: Clone, F>(
    collection: &[T],
    mut f: F,
) -> HashMap<K, Vec<T>>
where
    F: FnMut(&T) -> K,
{
    let mut result = HashMap::new();
    for item in collection {
        let key = f(item);
        result.entry(key).or_insert_with(Vec::new).push(item.clone());
    }
    result
}

pub struct CacheLayer<K: Eq + std::hash::Hash, V: Clone> {
    cache: Arc<Mutex<HashMap<K, V>>>,
    max_size: usize,
    access_order: Arc<Mutex<Vec<K>>>,
}

impl<K: Eq + std::hash::Hash + Clone, V: Clone> CacheLayer<K, V> {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            max_size,
            access_order: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        if let Ok(cache) = self.cache.lock() {
            cache.get(key).cloned()
        } else {
            None
        }
    }

    pub fn insert(&self, key: K, value: V) {
        if let Ok(mut cache) = self.cache.lock() {
            if cache.len() >= self.max_size && !cache.contains_key(&key) {
                if let Ok(mut order) = self.access_order.lock() {
                    if !order.is_empty() {
                        let oldest = order.remove(0);
                        cache.remove(&oldest);
                    }
                }
            }
            cache.insert(key.clone(), value);
            if let Ok(mut order) = self.access_order.lock() {
                order.retain(|k| k != &key);
                order.push(key);
            }
        }
    }

    pub fn clear(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
        if let Ok(mut order) = self.access_order.lock() {
            order.clear();
        }
    }

    pub fn size(&self) -> usize {
        if let Ok(cache) = self.cache.lock() {
            cache.len()
        } else {
            0
        }
    }
}

pub trait RenderStrategy<T: Clone> {
    fn render_point(&self, painter: &Painter, pos: Pos2, point: &DataPoint<T>, is_hovered: bool);
}

pub struct DefaultPointStrategy {
    pub radius: f32,
    pub hover_radius: f32,
    pub color: Color32,
    pub hover_color: Color32,
}

impl DefaultPointStrategy {
    pub fn new() -> Self {
        Self {
            radius: 5.5,
            hover_radius: 7.0,
            color: Color32::from_rgb(100, 150, 200),
            hover_color: Color32::from_rgb(200, 220, 255),
        }
    }
}

impl<T: Clone> RenderStrategy<T> for DefaultPointStrategy {
    fn render_point(&self, painter: &Painter, pos: Pos2, _point: &DataPoint<T>, is_hovered: bool) {
        let radius = if is_hovered { self.hover_radius } else { self.radius };
        let color = if is_hovered { self.hover_color } else { self.color };
        
        painter.circle_filled(pos, radius, color);
        
        if is_hovered {
            painter.circle_stroke(pos, radius + 2.0, egui::Stroke::new(1.5, Color32::WHITE));
        }
    }
}

pub fn filter_points<T: Clone>(
    points: &[DataPoint<T>],
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
) -> Vec<DataPoint<T>> {
    points.iter()
        .filter(|pt| pt.x >= x_min && pt.x <= x_max && pt.y >= y_min && pt.y <= y_max)
        .cloned()
        .collect()
}

pub fn merge_payloads<T: Clone>(p1: &RenderPayload<T>, p2: &RenderPayload<T>) -> RenderPayload<T> {
    let mut merged = RenderPayload::new();
    merged.points.extend(p1.points.clone());
    merged.points.extend(p2.points.clone());
    
    merged.min_x = p1.min_x.min(p2.min_x);
    merged.max_x = p1.max_x.max(p2.max_x);
    merged.min_y = p1.min_y.min(p2.min_y);
    merged.max_y = p1.max_y.max(p2.max_y);
    
    merged.labels.extend(p1.labels.clone());
    merged.labels.extend(p2.labels.clone());
    
    merged
}

pub struct LayerComposer<T: Clone> {
    layers: Vec<RenderPayload<T>>,
    opacity: Vec<f32>,
}

impl<T: Clone> LayerComposer<T> {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            opacity: Vec::new(),
        }
    }

    pub fn add_layer(mut self, payload: RenderPayload<T>, opacity: f32) -> Self {
        self.layers.push(payload);
        self.opacity.push(opacity.clamp(0.0, 1.0));
        self
    }

    pub fn composite(self) -> RenderPayload<T> {
        if self.layers.is_empty() {
            return RenderPayload::new();
        }

        let mut result = RenderPayload::new();
        
        for layer in self.layers.iter() {
            result.points.extend(layer.points.clone());
        }

        result.min_x = self.layers.iter().map(|l| l.min_x).fold(f64::INFINITY, f64::min);
        result.max_x = self.layers.iter().map(|l| l.max_x).fold(f64::NEG_INFINITY, f64::max);
        result.min_y = self.layers.iter().map(|l| l.min_y).fold(f64::INFINITY, f64::min);
        result.max_y = self.layers.iter().map(|l| l.max_y).fold(f64::NEG_INFINITY, f64::max);

        result
    }
}

pub fn interpolate_points<T: Clone>(points: &[DataPoint<T>], sample_count: usize) -> Vec<(f64, f64)> {
    if points.len() < 2 {
        return points.iter().map(|p| (p.x, p.y)).collect();
    }

    let mut result = Vec::new();
    for i in 0..points.len() - 1 {
        let p1 = &points[i];
        let p2 = &points[i + 1];

        for j in 0..sample_count {
            let t = j as f64 / sample_count as f64;
            let x = p1.x + (p2.x - p1.x) * t;
            let y = p1.y + (p2.y - p1.y) * t;
            result.push((x, y));
        }
    }
    result.push((points[points.len() - 1].x, points[points.len() - 1].y));
    result
}

pub fn aggregate_by_range<T: Clone + Default>(
    points: &[DataPoint<T>],
    range_count: usize,
    x_min: f64,
    x_max: f64,
) -> HashMap<usize, Vec<DataPoint<T>>> {
    let mut buckets: HashMap<usize, Vec<DataPoint<T>>> = HashMap::new();
    let range = x_max - x_min;

    for point in points {
        if point.x >= x_min && point.x <= x_max {
            let bucket = ((point.x - x_min) / range * range_count as f64).floor() as usize;
            let bucket = bucket.min(range_count - 1);
            buckets.entry(bucket).or_insert_with(Vec::new).push(point.clone());
        }
    }

    buckets
}
