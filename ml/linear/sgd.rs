use crate::ml::linalg::{dot, sigmoid, col_means, col_std};

pub struct SGDClassifier {
    pub coef: Vec<f64>,
    pub intercept: f64,
    pub alpha: f64,
    pub max_iter: usize,
    pub tol: f64,
    pub loss: SGDLoss,
    pub learning_rate: f64,
    pub fit_intercept: bool,
    pub n_iter: usize,
    pub classes: Vec<i32>,
}

#[derive(Clone, Copy)]
pub enum SGDLoss {
    Hinge,
    Log,
    ModifiedHuber,
    SquaredHinge,
}

impl SGDClassifier {
    pub fn new(loss: SGDLoss, alpha: f64, max_iter: usize, tol: f64, learning_rate: f64, fit_intercept: bool) -> Self {
        Self {
            coef: Vec::new(), intercept: 0.0, alpha, max_iter, tol,
            loss, learning_rate, fit_intercept, n_iter: 0, classes: Vec::new(),
        }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) {
        let mut classes: Vec<i32> = y.to_vec();
        classes.sort_unstable();
        classes.dedup();
        self.classes = classes;

        let yf: Vec<f64> = y.iter().map(|&v| if v == self.classes[self.classes.len() - 1] { 1.0 } else { -1.0 }).collect();
        let mut w = vec![0.0; p];
        let mut b = 0.0;
        let mut rng = 0xDEADBEEFCAFEu64;

        for epoch in 0..self.max_iter {
            let mut indices: Vec<usize> = (0..n).collect();
            for i in (1..n).rev() {
                rng ^= rng << 13; rng ^= rng >> 7; rng ^= rng << 17;
                let j = rng as usize % (i + 1);
                indices.swap(i, j);
            }

            let eta = self.learning_rate / (1.0 + self.alpha * self.learning_rate * epoch as f64);
            let mut total_loss = 0.0;

            for &idx in &indices {
                let row = &x[idx * p..(idx + 1) * p];
                let z = dot(row, &w) + b;
                let yi = yf[idx];

                let (dloss, loss_val) = match self.loss {
                    SGDLoss::Hinge => {
                        let m = yi * z;
                        if m < 1.0 { (-yi, (1.0 - m).max(0.0)) } else { (0.0, 0.0) }
                    }
                    SGDLoss::Log => {
                        let p = sigmoid(yi * z);
                        (-yi * (1.0 - p), -(p.max(1e-15).ln()))
                    }
                    SGDLoss::ModifiedHuber => {
                        let m = yi * z;
                        if m < -1.0 { (-4.0 * yi, (1.0 - m) * (1.0 - m)) }
                        else if m < 1.0 { (-2.0 * yi * (1.0 - m), (1.0 - m) * (1.0 - m)) }
                        else { (0.0, 0.0) }
                    }
                    SGDLoss::SquaredHinge => {
                        let m = yi * z;
                        if m < 1.0 { (-2.0 * yi * (1.0 - m), (1.0 - m) * (1.0 - m)) }
                        else { (0.0, 0.0) }
                    }
                };

                total_loss += loss_val;
                for j in 0..p { w[j] -= eta * (dloss * row[j] + self.alpha * w[j]); }
                if self.fit_intercept { b -= eta * dloss; }
            }

            self.n_iter = epoch + 1;
            if total_loss / (n as f64) < self.tol { break; }
        }

        self.coef = w;
        self.intercept = b;
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        (0..n).map(|i| {
            let z = dot(&x[i * p..(i + 1) * p], &self.coef) + self.intercept;
            if z >= 0.0 { self.classes[self.classes.len() - 1] } else { self.classes[0] }
        }).collect()
    }

    pub fn decision_function(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        (0..n).map(|i| dot(&x[i * p..(i + 1) * p], &self.coef) + self.intercept).collect()
    }
}

pub struct SGDRegressor {
    pub coef: Vec<f64>,
    pub intercept: f64,
    pub alpha: f64,
    pub max_iter: usize,
    pub tol: f64,
    pub learning_rate: f64,
    pub fit_intercept: bool,
    pub n_iter: usize,
}

impl SGDRegressor {
    pub fn new(alpha: f64, max_iter: usize, tol: f64, learning_rate: f64, fit_intercept: bool) -> Self {
        Self { coef: Vec::new(), intercept: 0.0, alpha, max_iter, tol, learning_rate, fit_intercept, n_iter: 0 }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) {
        let mut w = vec![0.0; p];
        let mut b = 0.0;
        let mut rng = 0xDEADBEEFCAFEu64;

        for epoch in 0..self.max_iter {
            let mut indices: Vec<usize> = (0..n).collect();
            for i in (1..n).rev() {
                rng ^= rng << 13; rng ^= rng >> 7; rng ^= rng << 17;
                let j = rng as usize % (i + 1);
                indices.swap(i, j);
            }

            let eta = self.learning_rate / (1.0 + self.alpha * self.learning_rate * epoch as f64);
            let mut total_loss = 0.0;

            for &idx in &indices {
                let row = &x[idx * p..(idx + 1) * p];
                let pred = dot(row, &w) + b;
                let err = pred - y[idx];
                total_loss += err * err;
                for j in 0..p { w[j] -= eta * (err * row[j] + self.alpha * w[j]); }
                if self.fit_intercept { b -= eta * err; }
            }

            self.n_iter = epoch + 1;
            if total_loss / (n as f64) < self.tol { break; }
        }

        self.coef = w;
        self.intercept = b;
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        (0..n).map(|i| dot(&x[i * p..(i + 1) * p], &self.coef) + self.intercept).collect()
    }
}
