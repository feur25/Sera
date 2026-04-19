use crate::ml::linalg::{dot, sigmoid, mat_vec, mat_t_vec};
use rayon::prelude::*;

fn lbfgs_direction(grad: &[f64], s_vecs: &[Vec<f64>], y_vecs: &[Vec<f64>], rhos: &[f64]) -> Vec<f64> {
    let m = s_vecs.len();
    let dim = grad.len();
    let mut q = grad.to_vec();
    let mut alphas = vec![0.0f64; m];
    for i in (0..m).rev() {
        let a = rhos[i] * s_vecs[i].iter().zip(q.iter()).map(|(s, qi)| s * qi).sum::<f64>();
        alphas[i] = a;
        for j in 0..dim { q[j] -= a * y_vecs[i][j]; }
    }
    let gamma = if m > 0 {
        let sy: f64 = s_vecs[m-1].iter().zip(y_vecs[m-1].iter()).map(|(s, y)| s * y).sum();
        let yy: f64 = y_vecs[m-1].iter().map(|y| y * y).sum();
        if yy > 1e-15 { sy / yy } else { 1.0 }
    } else { 1.0 };
    let mut r: Vec<f64> = q.iter().map(|&qi| gamma * qi).collect();
    for i in 0..m {
        let beta = rhos[i] * y_vecs[i].iter().zip(r.iter()).map(|(y, ri)| y * ri).sum::<f64>();
        for j in 0..dim { r[j] += s_vecs[i][j] * (alphas[i] - beta); }
    }
    r
}


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
        self.fit_resumable(x, n, p, y, None);
    }

    pub fn fit_resumable(&mut self, x: &[f64], n: usize, p: usize, y: &[i32], checkpoint_id: Option<u64>) {
        let classes = crate::ml::linalg::discover_classes(y);
        self.classes = classes.clone();

        if self.classes.len() == 2 {
            let yb: Vec<f64> = y.iter().map(|&v| if v == self.classes[1] { 1.0 } else { 0.0 }).collect();
            let (w, b, iters) = Self::fit_binary_impl(x, n, p, &yb, self.c, self.max_iter, self.tol, self.fit_intercept, checkpoint_id);
            self.coef = w;
            self.intercept = b;
            self.n_iter = iters;
        } else {
            let k = self.classes.len();
            let y_idx: Vec<usize> = y.iter().map(|&v| self.classes.iter().position(|&c| c == v).unwrap()).collect();
            let (multi_coef, multi_intercept, iters) = Self::fit_multinomial_impl(x, n, p, &y_idx, k, self.c, self.max_iter, self.tol, checkpoint_id);
            self.multi_coef = multi_coef;
            self.multi_intercept = multi_intercept;
            self.n_iter = iters;
        }
    }

    fn fit_multinomial_impl(x: &[f64], n: usize, p: usize, y_idx: &[usize], k: usize, c: f64, max_iter: usize, tol: f64, checkpoint_id: Option<u64>) -> (Vec<Vec<f64>>, Vec<f64>, usize) {
        let inv_n = 1.0 / n as f64;
        let reg = inv_n / c;
        let dim = k * (p + 1);
        let pp = p + 1;
        let mut w = vec![0.0; dim];

        let start_iter = if let Some(id) = checkpoint_id {
            if let Some(entry) = crate::ml::cache::checkpoint_load(id) {
                if entry.weights.len() == dim { w.copy_from_slice(&entry.weights); }
                entry.iteration.min(max_iter)
            } else { 0 }
        } else { 0 };

        let mut probs = vec![0.0; n * k];

        let compute_loss = |wv: &[f64], pr: &mut Vec<f64>| -> f64 {
            const CHUNK: usize = 2048;
            pr.par_chunks_mut(k * CHUNK).enumerate().for_each(|(ci, blk)| {
                let base = ci * CHUNK;
                let rows = blk.len() / k;
                for r in 0..rows {
                    let i = base + r;
                    let xi = &x[i * p..(i + 1) * p];
                    let row = &mut blk[r * k..(r + 1) * k];
                    let mut mx = f64::NEG_INFINITY;
                    for cj in 0..k {
                        let off = cj * pp;
                        let mut s = wv[off + p];
                        for j in 0..p { s += wv[off + j] * xi[j]; }
                        row[cj] = s;
                        if s > mx { mx = s; }
                    }
                    let mut sum = 0.0;
                    for v in row.iter_mut() { *v = (*v - mx).exp(); sum += *v; }
                    let inv = 1.0 / sum;
                    for v in row.iter_mut() { *v *= inv; }
                }
            });
            let nll: f64 = (0..n).map(|i| -pr[i * k + y_idx[i]].max(1e-300).ln()).sum();
            let mut loss = nll * inv_n;
            for ci in 0..k {
                let off = ci * pp;
                for j in 0..p { loss += 0.5 * reg * wv[off + j] * wv[off + j]; }
            }
            loss
        };

        let compute_grad = |wv: &[f64], pr: &mut Vec<f64>| -> (f64, Vec<f64>) {
            const CHUNK: usize = 2048;
            pr.par_chunks_mut(k * CHUNK).enumerate().for_each(|(ci, blk)| {
                let base = ci * CHUNK;
                let rows = blk.len() / k;
                for r in 0..rows {
                    let i = base + r;
                    let xi = &x[i * p..(i + 1) * p];
                    let row = &mut blk[r * k..(r + 1) * k];
                    let mut mx = f64::NEG_INFINITY;
                    for cj in 0..k {
                        let off = cj * pp;
                        let mut s = wv[off + p];
                        for j in 0..p { s += wv[off + j] * xi[j]; }
                        row[cj] = s;
                        if s > mx { mx = s; }
                    }
                    let mut sum = 0.0;
                    for v in row.iter_mut() { *v = (*v - mx).exp(); sum += *v; }
                    let inv = 1.0 / sum;
                    for v in row.iter_mut() { *v *= inv; }
                }
            });
            let nll: f64 = pr.par_chunks(k * CHUNK).enumerate().map(|(ci, blk)| {
                let base = ci * CHUNK;
                let rows = blk.len() / k;
                let mut l = 0.0f64;
                for r in 0..rows { l -= blk[r * k + y_idx[base + r]].max(1e-300).ln(); }
                l
            }).sum();
            let mut loss = nll * inv_n;
            for ci in 0..k {
                let off = ci * pp;
                for j in 0..p { loss += 0.5 * reg * wv[off + j] * wv[off + j]; }
            }
            let mut grad = if n >= 4096 {
                let chunk = 2048usize;
                let nc2 = (n + chunk - 1) / chunk;
                let partials: Vec<Vec<f64>> = (0..nc2).into_par_iter().map(|c2| {
                    let s = c2 * chunk;
                    let e = (s + chunk).min(n);
                    let mut g = vec![0.0; dim];
                    for i in s..e {
                        for ci in 0..k {
                            let diff = pr[i * k + ci] - if y_idx[i] == ci { 1.0 } else { 0.0 };
                            let off = ci * pp;
                            for j in 0..p { g[off + j] += diff * x[i * p + j]; }
                            g[off + p] += diff;
                        }
                    }
                    g
                }).collect();
                let mut g = vec![0.0; dim];
                for part in &partials { for j in 0..dim { g[j] += part[j]; } }
                g
            } else {
                let mut g = vec![0.0; dim];
                for i in 0..n {
                    for ci in 0..k {
                        let diff = pr[i * k + ci] - if y_idx[i] == ci { 1.0 } else { 0.0 };
                        let off = ci * pp;
                        for j in 0..p { g[off + j] += diff * x[i * p + j]; }
                        g[off + p] += diff;
                    }
                }
                g
            };
            for ci in 0..k {
                let off = ci * pp;
                for j in 0..p { grad[off + j] = grad[off + j] * inv_n + reg * wv[off + j]; }
                grad[off + p] *= inv_n;
            }
            (loss, grad)
        };

        let compute_grad_only = |wv: &[f64], pr: &Vec<f64>| -> Vec<f64> {
            let mut grad = if n >= 4096 {
                let chunk = 2048usize;
                let nc2 = (n + chunk - 1) / chunk;
                let partials: Vec<Vec<f64>> = (0..nc2).into_par_iter().map(|c2| {
                    let s = c2 * chunk;
                    let e = (s + chunk).min(n);
                    let mut g = vec![0.0; dim];
                    for i in s..e {
                        for ci in 0..k {
                            let diff = pr[i * k + ci] - if y_idx[i] == ci { 1.0 } else { 0.0 };
                            let off = ci * pp;
                            for j in 0..p { g[off + j] += diff * x[i * p + j]; }
                            g[off + p] += diff;
                        }
                    }
                    g
                }).collect();
                let mut g = vec![0.0; dim];
                for part in &partials { for j in 0..dim { g[j] += part[j]; } }
                g
            } else {
                let mut g = vec![0.0; dim];
                for i in 0..n {
                    for ci in 0..k {
                        let diff = pr[i * k + ci] - if y_idx[i] == ci { 1.0 } else { 0.0 };
                        let off = ci * pp;
                        for j in 0..p { g[off + j] += diff * x[i * p + j]; }
                        g[off + p] += diff;
                    }
                }
                g
            };
            for ci in 0..k {
                let off = ci * pp;
                for j in 0..p { grad[off + j] = grad[off + j] * inv_n + reg * wv[off + j]; }
                grad[off + p] *= inv_n;
            }
            grad
        };

        const M: usize = 10;
        let mut s_vecs: Vec<Vec<f64>> = Vec::with_capacity(M);
        let mut y_vecs: Vec<Vec<f64>> = Vec::with_capacity(M);
        let mut rhos: Vec<f64> = Vec::with_capacity(M);

        let (mut loss, mut grad) = compute_grad(&w, &mut probs);

        for iter in start_iter..max_iter {
            let gnorm: f64 = grad.iter().map(|&g| g * g).sum::<f64>().sqrt();
            if gnorm < tol {
                if let Some(id) = checkpoint_id { crate::ml::cache::checkpoint_clear(id); }
                let (mc, mi, _) = Self::unpack_multinomial(&w, k, p);
                return (mc, mi, iter + 1);
            }
            if let Some(id) = checkpoint_id {
                if (iter + 1) % 5 == 0 { crate::ml::cache::checkpoint_save(id, &w, iter + 1); }
            }

            let dir = lbfgs_direction(&grad, &s_vecs, &y_vecs, &rhos);

            let mut alpha = 1.0f64;
            let slope = grad.iter().zip(dir.iter()).map(|(g, d)| g * d).sum::<f64>();
            let mut w_new = vec![0.0; dim];
            let mut new_loss = loss;
            let mut new_grad = grad.clone();
            let mut accepted = false;
            for _ in 0..20 {
                for j in 0..dim { w_new[j] = w[j] - alpha * dir[j]; }
                let nl = compute_loss(&w_new, &mut probs);
                if nl <= loss - 1e-4 * alpha * slope {
                    new_loss = nl;
                    new_grad = compute_grad_only(&w_new, &probs);
                    accepted = true;
                    break;
                }
                alpha *= 0.5;
            }
            if !accepted {
                for j in 0..dim { w_new[j] = w[j] - alpha * dir[j]; }
                let (nl, ng) = compute_grad(&w_new, &mut probs);
                new_loss = nl;
                new_grad = ng;
            }

            let sk: Vec<f64> = (0..dim).map(|j| w_new[j] - w[j]).collect();
            let yk: Vec<f64> = (0..dim).map(|j| new_grad[j] - grad[j]).collect();
            let rho_denom: f64 = sk.iter().zip(yk.iter()).map(|(s, y)| s * y).sum();
            if rho_denom > 1e-15 {
                if s_vecs.len() == M { s_vecs.remove(0); y_vecs.remove(0); rhos.remove(0); }
                s_vecs.push(sk);
                y_vecs.push(yk);
                rhos.push(1.0 / rho_denom);
            }

            w.copy_from_slice(&w_new);
            grad = new_grad;
            loss = new_loss;
        }
        if let Some(id) = checkpoint_id { crate::ml::cache::checkpoint_clear(id); }
        let (mc, mi, _) = Self::unpack_multinomial(&w, k, p);
        (mc, mi, max_iter)
    }

    fn unpack_multinomial(w: &[f64], k: usize, p: usize) -> (Vec<Vec<f64>>, Vec<f64>, usize) {
        let pp = p + 1;
        let coefs: Vec<Vec<f64>> = (0..k).map(|c| w[c * pp..c * pp + p].to_vec()).collect();
        let intercepts: Vec<f64> = (0..k).map(|c| w[c * pp + p]).collect();
        (coefs, intercepts, 0)
    }

    fn fit_binary_impl(x: &[f64], n: usize, p: usize, y: &[f64], c: f64, max_iter: usize, tol: f64, fit_intercept: bool, checkpoint_id: Option<u64>) -> (Vec<f64>, f64, usize) {
        let reg = 1.0 / c;
        let inv_n = 1.0 / n as f64;
        let dim = p + if fit_intercept { 1 } else { 0 };
        let mut wb = vec![0.0; dim];

        let start_iter = if let Some(id) = checkpoint_id {
            if let Some(entry) = crate::ml::cache::checkpoint_load(id) {
                if entry.weights.len() == dim { wb.copy_from_slice(&entry.weights[..dim]); }
                entry.iteration.min(max_iter)
            } else { 0 }
        } else { 0 };

        let mut z_buf = vec![0.0; n];
        let mut s_buf = vec![0.0; n];

        let compute_loss_bin = |wb: &[f64], z_buf: &mut Vec<f64>, s_buf: &mut Vec<f64>| -> f64 {
            for i in 0..n {
                let mut z = 0.0;
                let row = &x[i * p..(i + 1) * p];
                for j in 0..p { z += wb[j] * row[j]; }
                if fit_intercept { z += wb[p]; }
                s_buf[i] = sigmoid(z);
            }
            let mut loss = 0.0;
            for i in 0..n {
                let si = s_buf[i].max(1e-300).min(1.0 - 1e-300);
                loss -= y[i] * si.ln() + (1.0 - y[i]) * (1.0 - si).ln();
            }
            loss *= inv_n;
            for j in 0..p { loss += 0.5 * reg * wb[j] * wb[j]; }
            loss
        };

        let compute_grad = |wb: &[f64], z_buf: &mut Vec<f64>, s_buf: &mut Vec<f64>| -> (f64, Vec<f64>) {
            for i in 0..n {
                let mut z = 0.0;
                let row = &x[i * p..(i + 1) * p];
                for j in 0..p { z += wb[j] * row[j]; }
                if fit_intercept { z += wb[p]; }
                z_buf[i] = z;
                s_buf[i] = sigmoid(z);
            }
            let mut loss = 0.0;
            for i in 0..n {
                let si = s_buf[i].max(1e-300).min(1.0 - 1e-300);
                loss -= y[i] * si.ln() + (1.0 - y[i]) * (1.0 - si).ln();
                z_buf[i] = s_buf[i] - y[i];
            }
            loss *= inv_n;
            for j in 0..p { loss += 0.5 * reg * wb[j] * wb[j]; }
            let mut grad = vec![0.0; dim];
            let mut gb = 0.0;
            for i in 0..n {
                let diff = z_buf[i];
                let row = &x[i * p..(i + 1) * p];
                for j in 0..p { grad[j] += diff * row[j]; }
                if fit_intercept { gb += diff; }
            }
            for j in 0..p { grad[j] = grad[j] * inv_n + reg * wb[j]; }
            if fit_intercept { grad[p] = gb * inv_n; }
            (loss, grad)
        };

        let compute_grad_only_bin = |wb: &[f64], s_buf: &Vec<f64>| -> Vec<f64> {
            let mut grad = vec![0.0; dim];
            let mut gb = 0.0;
            for i in 0..n {
                let diff = s_buf[i] - y[i];
                let row = &x[i * p..(i + 1) * p];
                for j in 0..p { grad[j] += diff * row[j]; }
                if fit_intercept { gb += diff; }
            }
            for j in 0..p { grad[j] = grad[j] * inv_n + reg * wb[j]; }
            if fit_intercept { grad[p] = gb * inv_n; }
            grad
        };

        const M: usize = 10;
        let mut s_vecs: Vec<Vec<f64>> = Vec::with_capacity(M);
        let mut y_vecs: Vec<Vec<f64>> = Vec::with_capacity(M);
        let mut rhos: Vec<f64> = Vec::with_capacity(M);

        let (mut loss, mut grad) = compute_grad(&wb, &mut z_buf, &mut s_buf);

        for iter in start_iter..max_iter {
            let gnorm: f64 = grad.iter().map(|&g| g * g).sum::<f64>().sqrt();
            if gnorm < tol {
                if let Some(id) = checkpoint_id { crate::ml::cache::checkpoint_clear(id); }
                let w = wb[..p].to_vec();
                let b = if fit_intercept { wb[p] } else { 0.0 };
                return (w, b, iter + 1);
            }
            if let Some(id) = checkpoint_id {
                if (iter + 1) % 5 == 0 { crate::ml::cache::checkpoint_save(id, &wb, iter + 1); }
            }

            let dir = lbfgs_direction(&grad, &s_vecs, &y_vecs, &rhos);
            let slope = grad.iter().zip(dir.iter()).map(|(g, d)| g * d).sum::<f64>();
            let mut alpha = 1.0f64;
            let mut wb_new = wb.clone();
            let mut new_loss = loss;
            let mut new_grad = grad.clone();
            let mut accepted = false;
            for _ in 0..20 {
                for j in 0..dim { wb_new[j] = wb[j] - alpha * dir[j]; }
                let nl = compute_loss_bin(&wb_new, &mut z_buf, &mut s_buf);
                if nl <= loss - 1e-4 * alpha * slope {
                    new_loss = nl;
                    new_grad = compute_grad_only_bin(&wb_new, &s_buf);
                    accepted = true;
                    break;
                }
                alpha *= 0.5;
            }
            if !accepted {
                for j in 0..dim { wb_new[j] = wb[j] - alpha * dir[j]; }
                let (nl, ng) = compute_grad(&wb_new, &mut z_buf, &mut s_buf);
                new_loss = nl;
                new_grad = ng;
            }

            let sk: Vec<f64> = (0..dim).map(|j| wb_new[j] - wb[j]).collect();
            let yk: Vec<f64> = (0..dim).map(|j| new_grad[j] - grad[j]).collect();
            let rho_denom: f64 = sk.iter().zip(yk.iter()).map(|(s, y)| s * y).sum();
            if rho_denom > 1e-15 {
                if s_vecs.len() == M { s_vecs.remove(0); y_vecs.remove(0); rhos.remove(0); }
                s_vecs.push(sk);
                y_vecs.push(yk);
                rhos.push(1.0 / rho_denom);
            }

            wb.copy_from_slice(&wb_new);
            grad = new_grad;
            loss = new_loss;
        }
        if let Some(id) = checkpoint_id { crate::ml::cache::checkpoint_clear(id); }
        let w = wb[..p].to_vec();
        let b = if fit_intercept { wb[p] } else { 0.0 };
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
