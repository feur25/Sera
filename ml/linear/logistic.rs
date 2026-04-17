use crate::ml::linalg::{dot, sigmoid, mat_vec, mat_t_vec, solve_spd};
use rayon::prelude::*;


pub struct LogisticRegression {
    pub coef: Vec<f64>,
    pub intercept: f64,
    pub c: f64,
    pub max_iter: usize,
    pub tol: f64,
    pub fit_intercept: bool,
    pub n_iter: usize,
    pub classes: Vec<i32>,
    pub multi_coef: Vec<Vec<f64>>,
    pub multi_intercept: Vec<f64>,
}

impl LogisticRegression {
    pub fn new(c: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> Self {
        Self {
            coef: Vec::new(), intercept: 0.0, c, max_iter, tol,
            fit_intercept, n_iter: 0, classes: Vec::new(),
            multi_coef: Vec::new(), multi_intercept: Vec::new(),
        }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize, y: &[i32]) {
        let classes = crate::ml::linalg::discover_classes(y);
        self.classes = classes.clone();

        if self.classes.len() == 2 {
            let yb: Vec<f64> = y.iter().map(|&v| if v == self.classes[1] { 1.0 } else { 0.0 }).collect();
            let (w, b, iters) = Self::fit_binary_impl(x, n, p, &yb, self.c, self.max_iter, self.tol, self.fit_intercept);
            self.coef = w;
            self.intercept = b;
            self.n_iter = iters;
        } else {
            let k = self.classes.len();
            let y_idx: Vec<usize> = y.iter().map(|&v| self.classes.iter().position(|&c| c == v).unwrap()).collect();
            let (multi_coef, multi_intercept, iters) = Self::fit_multinomial_impl(x, n, p, &y_idx, k, self.c, self.max_iter, self.tol);
            self.multi_coef = multi_coef;
            self.multi_intercept = multi_intercept;
            self.n_iter = iters;
        }
    }

    fn fit_multinomial_impl(x: &[f64], n: usize, p: usize, y_idx: &[usize], k: usize, c: f64, max_iter: usize, tol: f64) -> (Vec<Vec<f64>>, Vec<f64>, usize) {
        let inv_n = 1.0 / n as f64;
        let reg = inv_n / c;
        let dim = k * (p + 1);
        let pp = p + 1;
        let mut w = vec![0.0; dim];
        let mut probs = vec![0.0; n * k];
        let mut tmp_w = vec![0.0; dim];
        let mut tmp_probs = vec![0.0; n * k];

        let compute_probs_loss = |wv: &[f64], pr: &mut [f64], xd: &[f64], n: usize, p: usize, k: usize, pp: usize, y_idx: &[usize], reg: f64, inv_n: f64| -> f64 {
            let mut loss = 0.0f64;
            for i in 0..n {
                let mut max_s = f64::NEG_INFINITY;
                for ci in 0..k {
                    let off = ci * pp;
                    let mut s = wv[off + p];
                    for j in 0..p { s += wv[off + j] * xd[i * p + j]; }
                    pr[i * k + ci] = s;
                    if s > max_s { max_s = s; }
                }
                let mut sum = 0.0;
                for ci in 0..k {
                    let v = (pr[i * k + ci] - max_s).exp();
                    pr[i * k + ci] = v;
                    sum += v;
                }
                let inv_sum = 1.0 / sum;
                for ci in 0..k { pr[i * k + ci] *= inv_sum; }
                let pc = pr[i * k + y_idx[i]].max(1e-300);
                loss -= pc.ln();
            }
            loss *= inv_n;
            for ci in 0..k {
                let off = ci * pp;
                for j in 0..p { loss += 0.5 * reg * wv[off + j] * wv[off + j]; }
            }
            loss
        };

        let mut loss = compute_probs_loss(&w, &mut probs, x, n, p, k, pp, y_idx, reg, inv_n);

        for iter in 0..max_iter {
            let mut grad = if n >= 4096 {
                let chunk = 2048usize;
                let nc = (n + chunk - 1) / chunk;
                let partials: Vec<Vec<f64>> = (0..nc).into_par_iter().map(|c| {
                    let s = c * chunk;
                    let e = (s + chunk).min(n);
                    let mut g = vec![0.0; dim];
                    for i in s..e {
                        for ci in 0..k {
                            let diff = probs[i * k + ci] - if y_idx[i] == ci { 1.0 } else { 0.0 };
                            let off = ci * pp;
                            for j in 0..p { g[off + j] += diff * x[i * p + j]; }
                            g[off + p] += diff;
                        }
                    }
                    g
                }).collect();
                let mut grad = vec![0.0; dim];
                for part in &partials { for j in 0..dim { grad[j] += part[j]; } }
                grad
            } else {
                let mut grad = vec![0.0; dim];
                for i in 0..n {
                    for ci in 0..k {
                        let diff = probs[i * k + ci] - if y_idx[i] == ci { 1.0 } else { 0.0 };
                        let off = ci * pp;
                        for j in 0..p { grad[off + j] += diff * x[i * p + j]; }
                        grad[off + p] += diff;
                    }
                }
                grad
            };
            let mut gnorm = 0.0f64;
            for ci in 0..k {
                let off = ci * pp;
                for j in 0..p {
                    grad[off + j] = grad[off + j] * inv_n + reg * w[off + j];
                    gnorm += grad[off + j] * grad[off + j];
                }
                grad[off + p] *= inv_n;
                gnorm += grad[off + p] * grad[off + p];
            }
            if gnorm < tol * tol {
                let (mc, mi, _) = Self::unpack_multinomial(&w, k, p);
                return (mc, mi, iter + 1);
            }

            if dim <= 512 {
                let hess_inner = |start: usize, end: usize| -> Vec<f64> {
                    let mut h = vec![0.0; dim * dim];
                    for i in start..end {
                        for ci in 0..k {
                            let pc = probs[i * k + ci];
                            let ro = ci * pp;
                            for di in ci..k {
                                let wt = if ci == di { pc * (1.0 - pc) } else { -pc * probs[i * k + di] };
                                let co = di * pp;
                                if ci == di {
                                    for a in 0..p {
                                        let xa = x[i * p + a];
                                        let va = wt * xa;
                                        for bb in a..p { h[(ro + a) * dim + co + bb] += va * x[i * p + bb]; }
                                        h[(ro + a) * dim + co + p] += va;
                                    }
                                    h[(ro + p) * dim + co + p] += wt;
                                } else {
                                    for a in 0..p {
                                        let va = wt * x[i * p + a];
                                        for bb in 0..p { h[(ro + a) * dim + co + bb] += va * x[i * p + bb]; }
                                        h[(ro + a) * dim + co + p] += va;
                                    }
                                    for bb in 0..p { h[(ro + p) * dim + co + bb] += wt * x[i * p + bb]; }
                                    h[(ro + p) * dim + co + p] += wt;
                                }
                            }
                        }
                    }
                    h
                };
                let mut hess = if n >= 4096 {
                    let chunk = 2048usize;
                    let nc = (n + chunk - 1) / chunk;
                    let partials: Vec<Vec<f64>> = (0..nc).into_par_iter().map(|c| {
                        hess_inner(c * chunk, (c * chunk + chunk).min(n))
                    }).collect();
                    let mut h = vec![0.0; dim * dim];
                    for ph in &partials { for j in 0..dim * dim { h[j] += ph[j]; } }
                    h
                } else {
                    hess_inner(0, n)
                };
                for i in 0..dim {
                    hess[i * dim + i] = hess[i * dim + i] * inv_n + reg.max(1e-6);
                    for j in (i + 1)..dim {
                        hess[i * dim + j] *= inv_n;
                        hess[j * dim + i] = hess[i * dim + j];
                    }
                }
                if let Some(delta) = solve_spd(&hess, dim, &grad) {
                    let mut alpha = 1.0f64;
                    for _ in 0..12 {
                        for j in 0..dim { tmp_w[j] = w[j] - alpha * delta[j]; }
                        let new_loss = compute_probs_loss(&tmp_w, &mut tmp_probs, x, n, p, k, pp, y_idx, reg, inv_n);
                        if new_loss < loss {
                            w.copy_from_slice(&tmp_w);
                            probs.copy_from_slice(&tmp_probs);
                            loss = new_loss;
                            break;
                        }
                        alpha *= 0.5;
                    }
                    continue;
                }
            }

            let lr = 4.0 / (n as f64 + 4.0 / c);
            for j in 0..dim { tmp_w[j] = w[j] - lr * grad[j]; }
            let new_loss = compute_probs_loss(&tmp_w, &mut tmp_probs, x, n, p, k, pp, y_idx, reg, inv_n);
            if new_loss < loss {
                w.copy_from_slice(&tmp_w);
                probs.copy_from_slice(&tmp_probs);
                loss = new_loss;
            }
        }
        let (mc, mi, _) = Self::unpack_multinomial(&w, k, p);
        (mc, mi, max_iter)
    }

    fn unpack_multinomial(w: &[f64], k: usize, p: usize) -> (Vec<Vec<f64>>, Vec<f64>, usize) {
        let pp = p + 1;
        let coefs: Vec<Vec<f64>> = (0..k).map(|c| w[c * pp..c * pp + p].to_vec()).collect();
        let intercepts: Vec<f64> = (0..k).map(|c| w[c * pp + p]).collect();
        (coefs, intercepts, 0)
    }

    fn fit_binary_impl(x: &[f64], n: usize, p: usize, y: &[f64], c: f64, max_iter: usize, tol: f64, fit_intercept: bool) -> (Vec<f64>, f64, usize) {
        let reg = 1.0 / c;
        let inv_n = 1.0 / n as f64;
        let mut w = vec![0.0; p];
        let mut b = 0.0;
        let mut z_buf = vec![0.0; n];
        let mut s_buf = vec![0.0; n];
        let mut grad = vec![0.0; p];

        for iter in 0..max_iter {
            mat_vec(x, n, p, &w, &mut z_buf);
            let mut grad_b_sum = 0.0;
            for i in 0..n {
                s_buf[i] = sigmoid(z_buf[i] + b);
                z_buf[i] = s_buf[i] - y[i];
                grad_b_sum += z_buf[i];
            }
            let grad_b = grad_b_sum * inv_n;

            mat_t_vec(x, n, p, &z_buf, &mut grad);
            for j in 0..p { grad[j] = grad[j] * inv_n + reg * w[j]; }

            let gnorm: f64 = grad.iter().map(|&g| g * g).sum::<f64>() + grad_b * grad_b;
            if gnorm < tol * tol { return (w, b, iter + 1); }

            if p <= 512 {
                let hess = if n >= 4096 {
                    let chunk = 2048usize;
                    let nc = (n + chunk - 1) / chunk;
                    let partials: Vec<Vec<f64>> = (0..nc).into_par_iter().map(|c| {
                        let s = c * chunk;
                        let e = (s + chunk).min(n);
                        let mut h = vec![0.0; p * p];
                        for i in s..e {
                            let sw = s_buf[i] * (1.0 - s_buf[i]);
                            let row = &x[i * p..(i + 1) * p];
                            for a in 0..p {
                                let va = sw * row[a];
                                for bb in a..p { h[a * p + bb] += va * row[bb]; }
                            }
                        }
                        h
                    }).collect();
                    let mut hess = vec![0.0; p * p];
                    for part in &partials {
                        for k in 0..p * p { hess[k] += part[k]; }
                    }
                    hess
                } else {
                    let mut hess = vec![0.0; p * p];
                    for i in 0..n {
                        let sw = s_buf[i] * (1.0 - s_buf[i]);
                        let row = &x[i * p..(i + 1) * p];
                        for a in 0..p {
                            let va = sw * row[a];
                            for bb in a..p { hess[a * p + bb] += va * row[bb]; }
                        }
                    }
                    hess
                };
                let mut hess = hess;
                for a in 0..p {
                    for bb in a..p {
                        hess[a * p + bb] *= inv_n;
                        if a == bb { hess[a * p + bb] += reg; }
                        else { hess[bb * p + a] = hess[a * p + bb]; }
                    }
                }
                if let Some(delta) = solve_spd(&hess, p, &grad) {
                    for j in 0..p { w[j] -= delta[j]; }
                    if fit_intercept {
                        let hb = s_buf.iter().map(|&si| si * (1.0 - si)).sum::<f64>() * inv_n;
                        b -= grad_b / hb.max(1e-12);
                    }
                    continue;
                }
            }
            let lr = 4.0 / (n as f64 + 4.0 / c);
            for j in 0..p { w[j] -= lr * grad[j]; }
            if fit_intercept { b -= lr * grad_b; }
        }
        (w, b, max_iter)
    }

    pub fn predict_proba(&self, x: &[f64], n: usize, p: usize) -> Vec<Vec<f64>> {
        if self.classes.len() == 2 {
            (0..n).map(|i| {
                let z = dot(&x[i * p..(i + 1) * p], &self.coef) + self.intercept;
                let p1 = sigmoid(z);
                vec![1.0 - p1, p1]
            }).collect()
        } else {
            (0..n).map(|i| {
                let scores: Vec<f64> = self.multi_coef.iter().zip(self.multi_intercept.iter()).map(|(w, &b)| {
                    dot(&x[i * p..(i + 1) * p], w) + b
                }).collect();
                let max_s = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let exps: Vec<f64> = scores.iter().map(|&s| (s - max_s).exp()).collect();
                let sum: f64 = exps.iter().sum();
                exps.iter().map(|&e| e / sum).collect()
            }).collect()
        }
    }

    pub fn predict(&self, x: &[f64], n: usize, p: usize) -> Vec<i32> {
        let predict_one = |i: usize| -> i32 {
            let xi = &x[i * p..(i + 1) * p];
            if self.classes.len() == 2 {
                let z = dot(xi, &self.coef) + self.intercept;
                if z >= 0.0 { self.classes[1] } else { self.classes[0] }
            } else {
                let mut best_score = f64::NEG_INFINITY;
                let mut best_cls = 0;
                for (k, (w, &b)) in self.multi_coef.iter().zip(self.multi_intercept.iter()).enumerate() {
                    let s = dot(xi, w) + b;
                    if s > best_score { best_score = s; best_cls = k; }
                }
                self.classes[best_cls]
            }
        };
        if n >= 512 {
            (0..n).into_par_iter().map(predict_one).collect()
        } else {
            (0..n).map(predict_one).collect()
        }
    }
}
