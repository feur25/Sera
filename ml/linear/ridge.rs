use crate::ml::linalg::{mat_t_mat, mat_t_y, solve_spd, dot, col_means, mat_vec};
use rayon::prelude::*;

pub struct Ridge {
    pub coef: Vec<f64>,
    pub intercept: f64,
    pub alpha: f64,
    pub fit_intercept: bool,
}

impl Ridge {
    pub fn new(alpha: f64, fit_intercept: bool) -> Self {
        Self { coef: Vec::new(), intercept: 0.0, alpha, fit_intercept }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) {
        let (means, y_mean) = if self.fit_intercept {
            (col_means(x, n, p), y.iter().sum::<f64>() / n as f64)
        } else {
            (vec![0.0; p], 0.0)
        };
        let mut ata = vec![0.0; p * p];
        mat_t_mat(x, n, p, &mut ata);
        if self.fit_intercept {
            let nf = n as f64;
            for i in 0..p { for j in 0..p { ata[i * p + j] -= nf * means[i] * means[j]; } }
        }
        for j in 0..p { ata[j * p + j] += self.alpha; }
        let mut atb = vec![0.0; p];
        mat_t_y(x, n, p, y, &mut atb);
        if self.fit_intercept {
            let nf = n as f64;
            for j in 0..p { atb[j] -= nf * y_mean * means[j]; }
        }
        self.coef = solve_spd(&ata, p, &atb).unwrap_or_else(|| vec![0.0; p]);
        self.intercept = if self.fit_intercept { y_mean - dot(&self.coef, &means) } else { 0.0 };
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let mut out = vec![0.0; n];
        mat_vec(x, n, p, &self.coef, &mut out);
        let b = self.intercept;
        if b != 0.0 { out.par_iter_mut().for_each(|v| *v += b); }
        out
    }

    pub fn score(&self, x: &[f64], n: usize, p: usize, y: &[f64]) -> f64 {
        let pred = self.predict(x, n, p);
        super::ols::r2_score_pub(y, &pred)
    }
}

pub struct RidgeClassifier {
    pub ridge: Ridge,
    pub classes: Vec<i32>,
}

impl RidgeClassifier {
    pub fn new(alpha: f64) -> Self {
        Self { ridge: Ridge::new(alpha, true), classes: Vec::new() }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) {
        let mut classes: Vec<i32> = y.to_vec();
        classes.sort_unstable();
        classes.dedup();
        self.classes = classes;
        let yf: Vec<f64> = y.iter().map(|&v| v as f64).collect();
        self.ridge.fit(x, n, p, &yf);
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        let scores = self.ridge.predict(x, n, p);
        scores.iter().map(|&s| {
            let mut best = self.classes[0];
            let mut bd = f64::INFINITY;
            for &c in &self.classes {
                let d = (s - c as f64).abs();
                if d < bd { bd = d; best = c; }
            }
            best
        }).collect()
    }
}
