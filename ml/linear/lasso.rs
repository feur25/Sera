use crate::ml::linear::elastic_net::ElasticNet;

pub struct Lasso {
    inner: ElasticNet,
}

impl Lasso {
    pub fn new(alpha: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> Self {
        Self { inner: ElasticNet::new(alpha, 1.0, max_iter, tol, fit_intercept) }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) { self.inner.fit(x, n, p, y); }
    pub fn fit_resumable(&mut self, x: &[f64], n: usize, p: usize, y: &[f64], checkpoint_id: Option<u64>) { self.inner.fit_resumable(x, n, p, y, checkpoint_id); }
    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> { self.inner.predict(x, n, p) }

    pub fn coef(&self) -> &[f64] { &self.inner.coef }
    pub fn intercept(&self) -> f64 { self.inner.intercept }
    pub fn alpha(&self) -> f64 { self.inner.alpha }
    pub fn set_alpha(&mut self, v: f64) { self.inner.alpha = v; }
    pub fn max_iter(&self) -> usize { self.inner.max_iter }
    pub fn set_max_iter(&mut self, v: usize) { self.inner.max_iter = v; }
    pub fn tol(&self) -> f64 { self.inner.tol }
    pub fn set_tol(&mut self, v: f64) { self.inner.tol = v; }
    pub fn fit_intercept(&self) -> bool { self.inner.fit_intercept }
    pub fn set_fit_intercept(&mut self, v: bool) { self.inner.fit_intercept = v; }
    pub fn n_iter(&self) -> usize { self.inner.n_iter }
}

impl crate::ml::MlRegressor for Lasso {
    fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) { self.fit(x, n, p, y); }
    fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> { self.predict(x, n, p) }
}
