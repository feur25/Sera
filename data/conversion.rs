use super::loader::CsvData;
use super::processor::{Dataset, DataPoint, DataProcessor, AggregationBuilder};
use std::collections::HashMap;

pub trait ToCsvDataset {
    fn to_dataset(&self, col: &str) -> Dataset<f64>;
    fn process_column<F>(&self, col: &str, processor: F) -> Dataset<f64>
    where
        F: Fn(DataProcessor<f64>) -> DataProcessor<f64>;
    fn aggregate_column(&self, col: &str) -> CsvAggregationResult;
}

impl ToCsvDataset for CsvData {
    fn to_dataset(&self, col: &str) -> Dataset<f64> {
        self.to_numeric_dataset(col)
    }

    fn process_column<F>(&self, col: &str, processor: F) -> Dataset<f64>
    where
        F: Fn(DataProcessor<f64>) -> DataProcessor<f64>,
    {
        let dataset = self.to_numeric_dataset(col);
        let data_processor = DataProcessor::new(dataset);
        processor(data_processor).process()
    }

    fn aggregate_column(&self, col: &str) -> CsvAggregationResult {
        let data = self.get_numeric_column(col);
        let result = AggregationBuilder::new(data)
            .sum()
            .mean()
            .min()
            .max()
            .count()
            .median()
            .stddev()
            .build();

        CsvAggregationResult {
            sum: result.get("sum").unwrap_or(0.0),
            mean: result.get("mean").unwrap_or(0.0),
            min: result.get("min").unwrap_or(0.0),
            max: result.get("max").unwrap_or(0.0),
            count: result.get("count").unwrap_or(0.0),
            median: result.get("median").unwrap_or(0.0),
            stddev: result.get("stddev").unwrap_or(0.0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CsvAggregationResult {
    pub sum: f64,
    pub mean: f64,
    pub min: f64,
    pub max: f64,
    pub count: f64,
    pub median: f64,
    pub stddev: f64,
}

pub struct DatasetBuilder<T: Clone + 'static> {
    dataset: Dataset<T>,
}

impl<T: Clone + 'static> DatasetBuilder<T> {
    pub fn new(name: String) -> Self {
        Self {
            dataset: Dataset::new(name),
        }
    }

    pub fn with_point(mut self, value: T, label: String) -> Self {
        self.dataset = self.dataset.add_point(DataPoint::new(value, label));
        self
    }

    pub fn with_points(mut self, points: Vec<DataPoint<T>>) -> Self {
        self.dataset = self.dataset.with_points(points);
        self
    }

    pub fn with_metadata(mut self, key: String, val: String) -> Self {
        self.dataset = self.dataset.with_metadata(key, val);
        self
    }

    pub fn build(self) -> Dataset<T> {
        self.dataset
    }

    pub fn process<F>(self, processor: F) -> Dataset<T>
    where
        F: Fn(DataProcessor<T>) -> DataProcessor<T>,
    {
        let data_processor = DataProcessor::new(self.dataset);
        processor(data_processor).process()
    }
}

pub struct ChartDataProcessor {
    labels: Vec<String>,
    values: Vec<f64>,
    metadata: HashMap<String, Vec<String>>,
}

impl ChartDataProcessor {
    pub fn new(labels: Vec<String>, values: Vec<f64>) -> Self {
        Self {
            labels,
            values,
            metadata: HashMap::new(),
        }
    }

    pub fn from_csv(csv: &CsvData, label_col: &str, value_col: &str) -> Result<Self, String> {
        let labels = csv.get_string_column(label_col);
        let values = csv.get_numeric_column(value_col);

        if labels.len() != values.len() {
            return Err("Column lengths don't match".to_string());
        }

        Ok(Self::new(labels, values))
    }

    pub fn with_metadata(mut self, key: String, column: Vec<String>) -> Self {
        self.metadata.insert(key, column);
        self
    }

    pub fn filter_by_value<F>(self, predicate: F) -> Self
    where
        F: Fn(f64) -> bool,
    {
        let mut new_labels = Vec::new();
        let mut new_values = Vec::new();
        let mut new_metadata: HashMap<String, Vec<String>> = HashMap::new();

        for (i, &val) in self.values.iter().enumerate() {
            if predicate(val) {
                if i < self.labels.len() {
                    new_labels.push(self.labels[i].clone());
                }
                new_values.push(val);

                for (key, col) in &self.metadata {
                    let entry = new_metadata.entry(key.clone()).or_insert_with(Vec::new);
                    if i < col.len() {
                        entry.push(col[i].clone());
                    }
                }
            }
        }

        Self {
            labels: new_labels,
            values: new_values,
            metadata: new_metadata,
        }
    }

    pub fn sort_by_values(mut self, descending: bool) -> Self {
        let mut indices: Vec<usize> = (0..self.values.len()).collect();

        if descending {
            indices.sort_by(|&a, &b| {
                self.values[b].partial_cmp(&self.values[a])
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        } else {
            indices.sort_by(|&a, &b| {
                self.values[a].partial_cmp(&self.values[b])
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }

        self.labels = indices.iter().map(|&i| self.labels[i].clone()).collect();
        self.values = indices.iter().map(|&i| self.values[i]).collect();

        for (_key, col) in &mut self.metadata {
            *col = indices.iter().map(|&i| col[i].clone()).collect();
        }

        self
    }

    pub fn limit(mut self, n: usize) -> Self {
        self.labels.truncate(n);
        self.values.truncate(n);
        for col in self.metadata.values_mut() {
            col.truncate(n);
        }
        self
    }

    pub fn get_labels(&self) -> Vec<String> {
        self.labels.clone()
    }

    pub fn get_values(&self) -> Vec<f64> {
        self.values.clone()
    }

    pub fn get_metadata(&self, key: &str) -> Option<Vec<String>> {
        self.metadata.get(key).cloned()
    }

    pub fn len(&self) -> usize {
        self.labels.len()
    }

    pub fn is_empty(&self) -> bool {
        self.labels.is_empty()
    }
}


