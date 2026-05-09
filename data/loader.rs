use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use super::processor::{Dataset, DataPoint};

#[derive(Clone, Debug)]
pub struct CsvData {
    pub headers: Vec<String>,
    pub rows: Vec<HashMap<String, String>>,
    pub numeric_columns: Vec<String>,
}

impl CsvData {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, csv::Error> {
        let file = File::open(path)?;
        let mut reader = csv::Reader::from_reader(file);
        let headers = reader.headers()?.iter().map(|h| h.to_string()).collect::<Vec<_>>();

        let mut rows = Vec::new();
        for result in reader.records() {
            let record = result?;
            let mut row = HashMap::new();
            for (idx, header) in headers.iter().enumerate() {
                if let Some(value) = record.get(idx) {
                    row.insert(header.clone(), value.to_string());
                }
            }
            rows.push(row);
        }

        let numeric_columns = Self::detect_numeric(&headers, &rows);
        Ok(CsvData { headers, rows, numeric_columns })
    }

    fn detect_numeric(headers: &[String], rows: &[HashMap<String, String>]) -> Vec<String> {
        headers.iter()
            .filter(|col| {
                rows.iter().all(|row| {
                    row.get(*col)
                        .map(|v| v.parse::<f64>().is_ok())
                        .unwrap_or(true)
                })
            })
            .cloned()
            .collect()
    }

    pub fn get_numeric_column(&self, col: &str) -> Vec<f64> {
        self.rows.iter()
            .filter_map(|row| {
                row.get(col)
                    .and_then(|v| v.parse::<f64>().ok())
            })
            .collect()
    }

    pub fn get_string_column(&self, col: &str) -> Vec<String> {
        self.rows.iter()
            .filter_map(|row| row.get(col).cloned())
            .collect()
    }

    pub fn to_dataset<T>(&self, col: &str, converter: impl Fn(&str) -> Option<T>) -> Dataset<T>
    where
        T: Clone + 'static,
    {
        let mut dataset = Dataset::new(col.to_string());
        
        for (idx, row) in self.rows.iter().enumerate() {
            if let Some(val_str) = row.get(col) {
                if let Some(value) = converter(val_str) {
                    let label = row.get("id")
                        .or_else(|| row.get("label"))
                        .cloned()
                        .unwrap_or_else(|| idx.to_string());
                    
                    let point = DataPoint::new(value, label);
                    dataset = dataset.add_point(point);
                }
            }
        }
        
        dataset
    }

    pub fn to_numeric_dataset(&self, col: &str) -> Dataset<f64> {
        self.to_dataset(col, |s| s.parse::<f64>().ok())
    }

    pub fn to_string_dataset(&self, col: &str) -> Dataset<String> {
        self.to_dataset(col, |s| Some(s.to_string()))
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_detection() {
        let headers = vec!["id".to_string(), "value".to_string(), "name".to_string()];
        let mut rows = vec![HashMap::new()];
        rows[0].insert("id".to_string(), "1".to_string());
        rows[0].insert("value".to_string(), "42.5".to_string());
        rows[0].insert("name".to_string(), "test".to_string());

        let numeric = CsvData::detect_numeric(&headers, &rows);
        assert!(numeric.contains(&"id".to_string()));
        assert!(numeric.contains(&"value".to_string()));
        assert!(!numeric.contains(&"name".to_string()));
    }
}


