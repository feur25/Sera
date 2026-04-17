use crate::ml::linalg::{mat_t_mat, mat_t_y, solve_spd, qr_solve, dot, col_means, mat_vec};
use rayon::prelude::*;

pub struct LinearRegression {
    pub coef: Vec<f64>,
    pub intercept: f64,
    pub fit_intercept: bool,
}

impl LinearRegression {
    pub fn new(fit_intercept: bool) -> Self {
        Self { coef: Vec::new(), intercept: 0.0, fit_intercept }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[f64]) {
        if self.fit_intercept {
            let means = col_means(x, n, p);
            let y_mean = y.iter().sum::<f64>() / n as f64;
            let mut xc = vec![0.0; n * p];
            let mut yc = vec![0.0; n];
            for i in 0..n {
                for j in 0..p { xc[i * p + j] = x[i * p + j] - means[j]; }
                yc[i] = y[i] - y_mean;
            }
            self.coef = solve_normal(&xc, n, p, &yc);
            self.intercept = y_mean - dot(&self.coef, &means);
        } else {
            self.coef = solve_normal(x, n, p, y);
            self.intercept = 0.0;
        }
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
        r2_score(y, &pred)
    }
}

fn solve_normal(x: &[f64], n: usize, p: usize, y: &[f64]) -> Vec<f64> {
    if n >= p * 4 {
        let mut ata = vec![0.0; p * p];
        mat_t_mat(x, n, p, &mut ata);
        let mut atb = vec![0.0; p];
        mat_t_y(x, n, p, y, &mut atb);
        if let Some(w) = solve_spd(&ata, p, &atb) { return w; }
    }
    qr_solve(x, n, p, y)
}

pub fn r2_score_pub(y: &[f64], pred: &[f64]) -> f64 {
    r2_score(y, pred)
}

fn r2_score(y: &[f64], pred: &[f64]) -> f64 {
    let n = y.len();
    let mean = y.iter().sum::<f64>() / n as f64;
    let ss_res: f64 = y.iter().zip(pred).map(|(a, b)| (a - b) * (a - b)).sum();
    let ss_tot: f64 = y.iter().map(|a| (a - mean) * (a - mean)).sum();
    if ss_tot < 1e-15 { return 0.0; }
    1.0 - ss_res / ss_tot
}
