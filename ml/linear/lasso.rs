use crate::ml::linear::elastic_net::ElasticNet;

pub struct Lasso {
    inner: ElasticNet,
}

impl Lasso {
    pub fn new(alpha: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> Self {
        Self { inner: ElasticNet::new(alpha, 1.0, max_iter, tol, fit_intercept) }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) { self.inner.fit(x, n, p, y); }
    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> { self.inner.predict(x, n, p) }

    pub fn coef(&self) -> &[f64] { &self.inner.coef }
    pub fn intercept(&self) -> f64 { self.inner.intercept }
    pub fn alpha(&self) -> f64 { self.inner.alpha }
    pub fn n_iter(&self) -> usize { self.inner.n_iter }
}
