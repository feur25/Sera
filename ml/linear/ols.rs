use crate::ml::linalg::{mat_t_mat, mat_t_y, solve_spd, qr_solve, dot, mat_vec};
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
            let inv = 1.0 / n as f64;
            let mut xmean = vec![0.0f64; p];
            let mut ymean = 0.0f64;
            for i in 0..n {
                let row = &x[i * p..(i + 1) * p];
                for j in 0..p { xmean[j] += row[j]; }
                ymean += y[i];
            }
            for j in 0..p { xmean[j] *= inv; }
            ymean *= inv;
            let nf = n as f64;
            let mut ata = vec![0.0f64; p * p];
            let mut atb = vec![0.0f64; p];
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
                for i2 in 0..n {
                    let row = &x[i2 * p..(i2+1)*p];
                    let yr = y[i2];
                    for i in 0..p {
                        let ai = row[i];
                        for j in i..p { ata[i*p+j] += ai*row[j]; }
                        atb[i] += ai * yr;
                    }
                }
            }
            for i in 0..p {
                for j in i..p {
                    ata[i*p+j] -= nf * xmean[i] * xmean[j];
                    ata[j*p+i] = ata[i*p+j];
                }
                atb[i] -= nf * ymean * xmean[i];
            }
            self.coef = match solve_spd(&ata, p, &atb) {
                Some(w) => w,
                None => {
                    let mut xc = vec![0.0f64; n * p];
                    let mut yc = vec![0.0f64; n];
                    for i2 in 0..n {
                        for j in 0..p { xc[i2*p+j] = x[i2*p+j] - xmean[j]; }
                        yc[i2] = y[i2] - ymean;
                    }
                    qr_solve(&xc, n, p, &yc)
                }
            };
            self.intercept = ymean - dot(&self.coef, &xmean);
        } else {
            self.coef = solve_normal(x, n, p, y);
            self.intercept = 0.0;
        }
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
