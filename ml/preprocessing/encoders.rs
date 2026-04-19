use std::collections::HashMap;

pub struct LabelEncoder {
    pub classes: Vec<String>,
    map: HashMap<String, i32>,
}

impl LabelEncoder {
    pub fn new() -> Self {
        Self { classes: Vec::new(), map: HashMap::new() }
    }

    pub fn fit(&mut self, y: &[String]) {
        let mut seen = HashMap::with_capacity(64);
        for s in y {
            if !seen.contains_key(s.as_str()) {
                let idx = seen.len();
                seen.insert(s.as_str(), idx);
            }
        }
        let mut cls: Vec<&str> = seen.keys().copied().collect();
        cls.sort_unstable();
        self.classes = cls.iter().map(|&s| s.to_string()).collect();
        self.map.clear();
        self.map.reserve(self.classes.len());
        for (i, c) in self.classes.iter().enumerate() {
            self.map.insert(c.clone(), i as i32);
        }
    }

    pub fn transform(&self, y: &[String]) -> Vec<i32> {
        y.iter().map(|s| *self.map.get(s.as_str()).unwrap_or(&-1)).collect()
    }

    pub fn fit_transform(&mut self, y: &[String]) -> Vec<i32> {
        self.fit(y);
        self.transform(y)
    }

    pub fn inverse_transform(&self, y: &[i32]) -> Vec<String> {
        y.iter().map(|&i| {
            if i >= 0 && (i as usize) < self.classes.len() { self.classes[i as usize].clone() }
            else { String::new() }
        }).collect()
    }
}

pub struct OrdinalEncoder {
    pub categories: Vec<Vec<String>>,
    maps: Vec<HashMap<String, f64>>,
}

impl OrdinalEncoder {
    pub fn new() -> Self {
        Self { categories: Vec::new(), maps: Vec::new() }
    }

    pub fn fit(&mut self, x: &[String], n: usize, p: usize) {
        self.categories.clear();
        self.maps.clear();
        for j in 0..p {
            let mut cats: Vec<String> = (0..n).map(|i| x[i * p + j].clone()).collect();
            cats.sort();
            cats.dedup();
            let mut m = HashMap::new();
            for (k, c) in cats.iter().enumerate() { m.insert(c.clone(), k as f64); }
            self.categories.push(cats);
            self.maps.push(m);
        }
    }

    pub fn transform(&self, x: &[String], n: usize, p: usize) -> Vec<f64> {
        let mut out = vec![0.0; n * p];
        for i in 0..n {
            for j in 0..p {
                out[i * p + j] = *self.maps[j].get(&x[i * p + j]).unwrap_or(&-1.0);
            }
        }
        out
    }

    pub fn fit_transform(&mut self, x: &[String], n: usize, p: usize) -> Vec<f64> {
        self.fit(x, n, p);
        self.transform(x, n, p)
    }
}

pub struct OneHotEncoder {
    pub categories: Vec<Vec<String>>,
    maps: Vec<HashMap<String, usize>>,
    total_cols: usize,
}

impl OneHotEncoder {
    pub fn new() -> Self {
        Self { categories: Vec::new(), maps: Vec::new(), total_cols: 0 }
    }

    pub fn fit(&mut self, x: &[String], n: usize, p: usize) {
        self.categories.clear();
        self.maps.clear();
        self.total_cols = 0;
        for j in 0..p {
            let mut cats: Vec<String> = (0..n).map(|i| x[i * p + j].clone()).collect();
            cats.sort();
            cats.dedup();
            let mut m = HashMap::new();
            for (k, c) in cats.iter().enumerate() { m.insert(c.clone(), self.total_cols + k); }
            self.total_cols += cats.len();
            self.categories.push(cats);
            self.maps.push(m);
        }
    }

    pub fn transform(&self, x: &[String], n: usize, p: usize) -> Vec<f64> {
        let mut out = vec![0.0; n * self.total_cols];
        for i in 0..n {
            for j in 0..p {
                if let Some(&col) = self.maps[j].get(&x[i * p + j]) {
                    out[i * self.total_cols + col] = 1.0;
                }
            }
        }
        out
    }

    pub fn fit_transform(&mut self, x: &[String], n: usize, p: usize) -> Vec<f64> {
        self.fit(x, n, p);
        self.transform(x, n, p)
    }

    pub fn n_output_features(&self) -> usize { self.total_cols }
}
