pub mod linalg;
pub mod cache;
pub mod bindings;
pub mod linear;
pub mod tree;
pub mod neighbors;
pub mod naive_bayes;
pub mod svm;
pub mod preprocessing;
pub mod decomposition;
pub mod model_selection;
pub mod metrics;
pub mod anomaly;
pub mod registry;
pub mod export;
pub mod gpu;
pub mod distributed;

pub use linalg::*;
pub use linear::*;
pub use tree::*;
pub use neighbors::*;
pub use naive_bayes::*;
pub use svm::*;
pub use preprocessing::*;
pub use decomposition::*;
pub use model_selection::*;
pub use metrics::*;
pub use anomaly::*;

pub trait MlRegressor: Send + Sync {
    fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]);
    fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64>;
    fn score(&self, x: &[f64], n: usize, p: usize, y: &[f64]) -> f64 {
        let preds = self.predict(x, n, p);
        crate::ml::metrics::regression::r2_score(y, &preds)
    }
}

pub trait MlClassifier: Send + Sync {
    fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]);
    fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32>;
    fn score(&self, x: &[f64], n: usize, p: usize, y: &[i32]) -> f64 {
        let preds = self.predict(x, n, p);
        crate::ml::metrics::classification::accuracy_score(y, &preds)
    }
}


