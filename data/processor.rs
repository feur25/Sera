use std::collections::HashMap;
use std::fmt;

pub trait Numeric: Clone + Copy + PartialOrd + fmt::Debug {
    fn to_f64(self) -> f64;
    fn from_f64(val: f64) -> Self;
}

impl Numeric for f64 {
    fn to_f64(self) -> f64 { self }
    fn from_f64(val: f64) -> Self { val }
}

impl Numeric for f32 {
    fn to_f64(self) -> f64 { self as f64 }
    fn from_f64(val: f64) -> Self { val as f32 }
}

impl Numeric for i32 {
    fn to_f64(self) -> f64 { self as f64 }
    fn from_f64(val: f64) -> Self { val as i32 }
}

impl Numeric for i64 {
    fn to_f64(self) -> f64 { self as f64 }
    fn from_f64(val: f64) -> Self { val as i64 }
}

impl Numeric for u32 {
    fn to_f64(self) -> f64 { self as f64 }
    fn from_f64(val: f64) -> Self { val as u32 }
}

impl Numeric for u64 {
    fn to_f64(self) -> f64 { self as f64 }
    fn from_f64(val: f64) -> Self { val as u64 }
}

pub trait Transform<T: Clone> {
    fn apply(&self, data: T) -> T;
}

pub struct FilterTransform<T: Clone, F>
where
    F: Fn(&T) -> bool,
{
    predicate: F,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone, F> FilterTransform<T, F>
where
    F: Fn(&T) -> bool,
{
    pub fn new(predicate: F) -> Self {
        Self {
            predicate,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Clone, F> Transform<Vec<T>> for FilterTransform<T, F>
where
    F: Fn(&T) -> bool,
{
    fn apply(&self, data: Vec<T>) -> Vec<T> {
        data.into_iter().filter(|item| (self.predicate)(item)).collect()
    }
}

pub struct MapTransformVec<T: Clone, U: Clone, F>
where
    F: Fn(&T) -> U,
{
    mapper: F,
    _phantom: std::marker::PhantomData<(T, U)>,
}

impl<T: Clone, U: Clone, F> MapTransformVec<T, U, F>
where
    F: Fn(&T) -> U,
{
    pub fn new(mapper: F) -> Self {
        Self {
            mapper,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn apply(&self, data: Vec<T>) -> Vec<U> {
        data.iter().map(|item| (self.mapper)(item)).collect()
    }
}

#[derive(Clone, Debug)]
pub struct DataPoint<T: Clone> {
    pub value: T,
    pub label: String,
    pub metadata: HashMap<String, String>,
}

impl<T: Clone> DataPoint<T> {
    pub fn new(value: T, label: String) -> Self {
        Self {
            value,
            label,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: String, val: String) -> Self {
        self.metadata.insert(key, val);
        self
    }
}

#[derive(Clone, Debug)]
pub struct Dataset<T: Clone> {
    pub points: Vec<DataPoint<T>>,
    pub name: String,
    pub metadata: HashMap<String, String>,
}

impl<T: Clone> Dataset<T> {
    pub fn new(name: String) -> Self {
        Self {
            points: Vec::new(),
            name,
            metadata: HashMap::new(),
        }
    }

    pub fn with_points(mut self, points: Vec<DataPoint<T>>) -> Self {
        self.points = points;
        self
    }

    pub fn add_point(mut self, point: DataPoint<T>) -> Self {
        self.points.push(point);
        self
    }

    pub fn with_metadata(mut self, key: String, val: String) -> Self {
        self.metadata.insert(key, val);
        self
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
}

pub struct DataProcessor<T: Clone> {
    dataset: Dataset<T>,
    transforms: Vec<Box<dyn Fn(Vec<DataPoint<T>>) -> Vec<DataPoint<T>>>>,
}

impl<T: Clone + 'static> DataProcessor<T> {
    pub fn new(dataset: Dataset<T>) -> Self {
        Self {
            dataset,
            transforms: Vec::new(),
        }
    }

    pub fn filter<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&DataPoint<T>) -> bool + 'static,
    {
        self.transforms.push(Box::new(move |points| {
            points.into_iter().filter(|p| predicate(p)).collect()
        }));
        self
    }

    pub fn map<U, F>(self, mapper: F) -> DataProcessor<U>
    where
        U: Clone + 'static,
        F: Fn(&DataPoint<T>) -> DataPoint<U> + 'static,
    {
        let mapped_points = self.dataset.points.iter().map(mapper).collect();
        DataProcessor {
            dataset: Dataset {
                points: mapped_points,
                name: self.dataset.name.clone(),
                metadata: self.dataset.metadata.clone(),
            },
            transforms: Vec::new(),
        }
    }

    pub fn limit(mut self, count: usize) -> Self {
        self.transforms.push(Box::new(move |points| {
            points.into_iter().take(count).collect()
        }));
        self
    }

    pub fn skip(mut self, count: usize) -> Self {
        self.transforms.push(Box::new(move |points| {
            points.into_iter().skip(count).collect()
        }));
        self
    }

    pub fn process(self) -> Dataset<T> {
        let points = self.transforms.into_iter().fold(
            self.dataset.points,
            |acc, transform| transform(acc),
        );
        
        Dataset {
            points,
            name: self.dataset.name,
            metadata: self.dataset.metadata,
        }
    }

    pub fn collect(self) -> Vec<DataPoint<T>> {
        self.process().points
    }
}

pub struct AggregationBuilder<T: Numeric> {
    data: Vec<T>,
    operations: Vec<String>,
}

impl<T: Numeric + 'static> AggregationBuilder<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self {
            data,
            operations: Vec::new(),
        }
    }

    pub fn sum(mut self) -> Self {
        self.operations.push("sum".to_string());
        self
    }

    pub fn mean(mut self) -> Self {
        self.operations.push("mean".to_string());
        self
    }

    pub fn min(mut self) -> Self {
        self.operations.push("min".to_string());
        self
    }

    pub fn max(mut self) -> Self {
        self.operations.push("max".to_string());
        self
    }

    pub fn count(mut self) -> Self {
        self.operations.push("count".to_string());
        self
    }

    pub fn median(mut self) -> Self {
        self.operations.push("median".to_string());
        self
    }

    pub fn stddev(mut self) -> Self {
        self.operations.push("stddev".to_string());
        self
    }

    pub fn build(self) -> AggregationResult {
        let mut result = AggregationResult::new();

        for op in self.operations {
            match op.as_str() {
                "sum" => {
                    let sum = self.data.iter().map(|v| v.to_f64()).sum::<f64>();
                    result.insert("sum".to_string(), sum);
                }
                "mean" => {
                    let mean = if self.data.is_empty() {
                        0.0
                    } else {
                        self.data.iter().map(|v| v.to_f64()).sum::<f64>() / self.data.len() as f64
                    };
                    result.insert("mean".to_string(), mean);
                }
                "min" => {
                    let min = self.data.iter()
                        .map(|v| v.to_f64())
                        .fold(f64::INFINITY, f64::min);
                    result.insert("min".to_string(), min);
                }
                "max" => {
                    let max = self.data.iter()
                        .map(|v| v.to_f64())
                        .fold(f64::NEG_INFINITY, f64::max);
                    result.insert("max".to_string(), max);
                }
                "count" => {
                    result.insert("count".to_string(), self.data.len() as f64);
                }
                "median" => {
                    let mut sorted: Vec<f64> = self.data.iter().map(|v| v.to_f64()).collect();
                    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                    let median = if sorted.is_empty() {
                        0.0
                    } else if sorted.len() % 2 == 0 {
                        (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
                    } else {
                        sorted[sorted.len() / 2]
                    };
                    result.insert("median".to_string(), median);
                }
                "stddev" => {
                    let mean = if self.data.is_empty() {
                        0.0
                    } else {
                        self.data.iter().map(|v| v.to_f64()).sum::<f64>() / self.data.len() as f64
                    };
                    let variance = if self.data.is_empty() {
                        0.0
                    } else {
                        self.data.iter()
                            .map(|v| (v.to_f64() - mean).powi(2))
                            .sum::<f64>() / self.data.len() as f64
                    };
                    let stddev = variance.sqrt();
                    result.insert("stddev".to_string(), stddev);
                }
                _ => {}
            }
        }

        result
    }
}

#[derive(Clone, Debug)]
pub struct AggregationResult {
    values: HashMap<String, f64>,
}

impl AggregationResult {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: f64) {
        self.values.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<f64> {
        self.values.get(key).copied()
    }

    pub fn all(&self) -> &HashMap<String, f64> {
        &self.values
    }
}

impl Default for AggregationResult {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GroupProcessor<T: Clone + 'static> {
    dataset: Dataset<T>,
    group_key: String,
}

impl<T: Clone + 'static> GroupProcessor<T> {
    pub fn new(dataset: Dataset<T>, group_key: String) -> Self {
        Self { dataset, group_key }
    }

    pub fn group(self) -> HashMap<String, Dataset<T>> {
        let mut groups: HashMap<String, Vec<DataPoint<T>>> = HashMap::new();

        for point in self.dataset.points {
            let group = point.metadata.get(&self.group_key)
                .cloned()
                .unwrap_or_else(|| "unknown".to_string());
            
            groups.entry(group).or_insert_with(Vec::new).push(point);
        }

        groups.into_iter().map(|(name, points)| {
            (name.clone(), Dataset {
                points,
                name,
                metadata: self.dataset.metadata.clone(),
            })
        }).collect()
    }
}

pub struct PipelineBuilder<T: Clone + 'static> {
    dataset: Dataset<T>,
    filters: Vec<Box<dyn Fn(&DataPoint<T>) -> bool>>,
    limit_val: Option<usize>,
    skip_val: Option<usize>,
}

impl<T: Clone + 'static> PipelineBuilder<T> {
    pub fn new(dataset: Dataset<T>) -> Self {
        Self {
            dataset,
            filters: Vec::new(),
            limit_val: None,
            skip_val: None,
        }
    }

    pub fn filter<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&DataPoint<T>) -> bool + 'static,
    {
        self.filters.push(Box::new(predicate));
        self
    }

    pub fn limit(mut self, n: usize) -> Self {
        self.limit_val = Some(n);
        self
    }

    pub fn skip(mut self, n: usize) -> Self {
        self.skip_val = Some(n);
        self
    }

    pub fn execute(self) -> Dataset<T> {
        let mut points = self.dataset.points;

        for filter in self.filters {
            points.retain(|p| filter(p));
        }

        if let Some(n) = self.skip_val {
            points = points.into_iter().skip(n).collect();
        }

        if let Some(n) = self.limit_val {
            points = points.into_iter().take(n).collect();
        }

        Dataset {
            points,
            name: self.dataset.name,
            metadata: self.dataset.metadata,
        }
    }
}

pub struct BatchProcessor<T: Clone + 'static> {
    dataset: Dataset<T>,
    batch_size: usize,
}

impl<T: Clone + 'static> BatchProcessor<T> {
    pub fn new(dataset: Dataset<T>, batch_size: usize) -> Self {
        Self { dataset, batch_size }
    }

    pub fn process<F, U>(self, processor: F) -> Vec<U>
    where
        F: Fn(Vec<DataPoint<T>>) -> U,
    {
        let mut result = Vec::new();
        let batches: Vec<Vec<DataPoint<T>>> = self.dataset.points
            .into_iter()
            .collect::<Vec<_>>()
            .chunks(self.batch_size)
            .map(|chunk| chunk.to_vec())
            .collect();

        for batch in batches {
            result.push(processor(batch));
        }

        result
    }
}

pub struct CachedProcessor<T: Clone + 'static> {
    dataset: Dataset<T>,
    cache: HashMap<String, Vec<DataPoint<T>>>,
}

impl<T: Clone + 'static> CachedProcessor<T> {
    pub fn new(dataset: Dataset<T>) -> Self {
        Self {
            dataset,
            cache: HashMap::new(),
        }
    }

    pub fn cache_filtered<F>(mut self, key: String, predicate: F) -> Self
    where
        F: Fn(&DataPoint<T>) -> bool,
    {
        let filtered = self.dataset.points.iter()
            .filter(|p| predicate(p))
            .cloned()
            .collect();
        self.cache.insert(key, filtered);
        self
    }

    pub fn get_cached(&self, key: &str) -> Option<Vec<DataPoint<T>>> {
        self.cache.get(key).cloned()
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}


