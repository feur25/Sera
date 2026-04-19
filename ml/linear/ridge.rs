use crate::ml::linalg::{solve_spd, dot, mat_vec};
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
        let inv = 1.0 / n as f64;
        let nf = n as f64;
        let mut ata = vec![0.0f64; p * p];
        let mut atb = vec![0.0f64; p];
        let (means, y_mean) = if self.fit_intercept {
            let mut xm = vec![0.0f64; p];
            let mut ym = 0.0f64;
            for i in 0..n {
                let row = &x[i * p..(i + 1) * p];
                for j in 0..p { xm[j] += row[j]; }
                ym += y[i];
            }
            for j in 0..p { xm[j] *= inv; }
            ym *= inv;
            (xm, ym)
        } else {
            (vec![0.0; p], 0.0)
        };
        if n >= 1024 {
            let chunk = 2048usize.min(n);
            let nc = (n + chunk - 1) / chunk;
            let partials: Vec<(Vec<f64>, Vec<f64>)> = (0..nc).into_par_iter().map(|ci| {
                let s = ci * chunk;
                let e = (s + chunk).min(n);
                let mut pa = vec![0.0f64; p * p];
                let mut pb = vec![0.0f64; p];
                let mut r = s;
                while r + 4 <= e {
                    let r0 = &x[r * p..(r + 1) * p];
                    let r1 = &x[(r+1) * p..(r+2) * p];
                    let r2 = &x[(r+2) * p..(r+3) * p];
                    let r3 = &x[(r+3) * p..(r+4) * p];
                    let y0 = y[r]; let y1 = y[r+1]; let y2 = y[r+2]; let y3 = y[r+3];
                    for i in 0..p {
                        let (a0, a1, a2, a3) = (r0[i], r1[i], r2[i], r3[i]);
                        let base = i * p;
                        for j in i..p { pa[base+j] += a0*r0[j]+a1*r1[j]+a2*r2[j]+a3*r3[j]; }
                        pb[i] += a0*y0 + a1*y1 + a2*y2 + a3*y3;
                    }
                    r += 4;
                }
                while r < e {
                    let row = &x[r * p..(r+1)*p];
                    let yr = y[r];
                    for i in 0..p {
                        let ai = row[i];
                        for j in i..p { pa[i*p+j] += ai*row[j]; }
                        pb[i] += ai * yr;
                    }
                    r += 1;
                }
                (pa, pb)
            }).collect();
            for (pa, pb) in &partials {
                for i in 0..p { for j in i..p { ata[i*p+j] += pa[i*p+j]; } }
                for j in 0..p { atb[j] += pb[j]; }
            }
        } else {
            for i in 0..n {
                let row = &x[i * p..(i+1)*p];
                let yr = y[i];
                for ii in 0..p {
                    let ai = row[ii];
                    for j in ii..p { ata[ii*p+j] += ai*row[j]; }
                    atb[ii] += ai * yr;
                }
            }
        }
        if self.fit_intercept {
            for i in 0..p {
                for j in i..p { ata[i*p+j] -= nf * means[i] * means[j]; }
                atb[i] -= nf * y_mean * means[i];
            }
        }
        for i in 0..p { for j in (i+1)..p { ata[j*p+i] = ata[i*p+j]; } }
        for j in 0..p { ata[j*p+j] += self.alpha; }
        self.coef = solve_spd(&ata, p, &atb).unwrap_or_else(|| vec![0.0; p]);
        self.intercept = if self.fit_intercept { y_mean - dot(&self.coef, &means) } else { 0.0 };
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let mut out = vec![0.0; n];
        mat_vec(x, n, p, &self.coef, &mut out);
        let b = self.intercept;
        if b != 0.0 { for v in out.iter_mut() { *v += b; } }
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
