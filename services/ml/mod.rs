pub mod anomaly;
pub mod bindings;
pub mod cache;
pub mod decomposition;
pub mod distributed;
pub mod export;
pub mod gpu;
pub mod handle;
pub mod linalg;
pub mod linear;
pub mod metrics;
pub mod model_selection;
pub mod models;
pub mod naive_bayes;
pub mod neighbors;
pub mod preprocessing;
pub mod registry;
pub mod svm;
pub mod tree;

pub use anomaly::*;
pub use decomposition::*;
pub use linalg::*;
pub use linear::*;
pub use metrics::*;
pub use model_selection::*;
pub use naive_bayes::*;
pub use neighbors::*;
pub use preprocessing::*;
pub use svm::*;
pub use tree::*;

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
