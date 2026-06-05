pub struct DbscanState {
    pub eps: f64,
    pub min_samples: usize,
    pub labels: Vec<i32>,
    pub n_clusters: usize,
    pub n_noise: usize,
}

impl DbscanState {
    pub fn new(eps: f64, min_samples: usize) -> Self {
        Self {
            eps,
            min_samples,
            labels: Vec::new(),
            n_clusters: 0,
            n_noise: 0,
        }
    }
    pub fn fit(&mut self, x: &[Vec<f64>]) {
        let (labels, n_clusters) =
            crate::plot::default::scatter::dbscan_core_nd(x, self.eps, self.min_samples);
        self.n_noise = labels.iter().filter(|&&l| l < 0).count();
        self.labels = labels;
        self.n_clusters = n_clusters;
    }
    pub fn fit_flat(&mut self, x: &[f64], n: usize, p: usize) {
        let (labels, n_clusters) =
            crate::plot::default::scatter::dbscan_core_nd_flat(x, n, p, self.eps, self.min_samples);
        self.n_noise = labels.iter().filter(|&&l| l < 0).count();
        self.labels = labels;
        self.n_clusters = n_clusters;
    }
}

pub struct KMeansState {
    pub k: usize,
    pub max_iter: usize,
    pub tol: f64,
    pub mini_batch: bool,
    pub batch_size: usize,
    pub n_init: usize,
    pub labels: Vec<i32>,
    pub centroids: Vec<Vec<f64>>,
    pub inertia: f64,
    pub n_iter: usize,
}

impl KMeansState {
    pub fn new(
        k: usize,
        max_iter: usize,
        tol: f64,
        mini_batch: bool,
        batch_size: usize,
        n_init: usize,
    ) -> Self {
        Self {
            k,
            max_iter,
            tol,
            mini_batch,
            batch_size,
            n_init,
            labels: Vec::new(),
            centroids: Vec::new(),
            inertia: 0.0,
            n_iter: 0,
        }
    }
    pub fn fit_flat(&mut self, flat: &[f64], n: usize, dims: usize) {
        if self.mini_batch {
            let (labels, flat_c, inertia) = crate::plot::default::minibatch_kmeans_core_flat(
                flat,
                n,
                dims,
                self.k,
                self.max_iter,
                self.batch_size,
            );
            self.labels = labels;
            self.centroids = (0..self.k.min(n))
                .map(|ki| flat_c[ki * dims..(ki + 1) * dims].to_vec())
                .collect();
            self.inertia = inertia;
        } else {
            let (labels, flat_c, inertia) = crate::plot::default::kmeans_core_flat_ninit(
                flat,
                n,
                dims,
                self.k,
                self.max_iter,
                self.tol,
                self.n_init,
            );
            self.labels = labels;
            self.centroids = (0..self.k.min(n))
                .map(|ki| flat_c[ki * dims..(ki + 1) * dims].to_vec())
                .collect();
            self.inertia = inertia;
        }
        self.n_iter = self.max_iter;
    }
    pub fn predict_flat(&self, flat: &[f64], n: usize, dims: usize) -> Vec<i32> {
        use rayon::prelude::*;
        let kk = self.centroids.len();
        if n == 0 || dims == 0 || kk == 0 {
            return vec![0i32; n];
        }
        let mut c_flat: Vec<f64> = Vec::with_capacity(kk * dims);
        for c in &self.centroids {
            if c.len() == dims {
                c_flat.extend_from_slice(c);
            } else {
                c_flat.extend(std::iter::repeat(0.0).take(dims));
            }
        }
        let mut c_norm = vec![0.0f64; kk];
        for ki in 0..kk {
            let row = &c_flat[ki * dims..(ki + 1) * dims];
            let mut s = 0.0;
            for &v in row {
                s += v * v;
            }
            c_norm[ki] = s;
        }
        let mut d = vec![0.0f64; n * kk];
        if n * dims * kk < 4096 {
            d.par_chunks_mut(kk).enumerate().for_each(|(i, drow)| {
                let xrow = &flat[i * dims..(i + 1) * dims];
                for ki in 0..kk {
                    let crow = &c_flat[ki * dims..(ki + 1) * dims];
                    let mut s = 0.0;
                    for j in 0..dims {
                        s += xrow[j] * crow[j];
                    }
                    drow[ki] = s;
                }
            });
        } else {
            unsafe {
                matrixmultiply::dgemm(
                    n,
                    dims,
                    kk,
                    1.0,
                    flat.as_ptr(),
                    dims as isize,
                    1,
                    c_flat.as_ptr(),
                    1,
                    dims as isize,
                    0.0,
                    d.as_mut_ptr(),
                    kk as isize,
                    1,
                );
            }
        }
        let mut labels = vec![0i32; n];
        labels.par_iter_mut().enumerate().for_each(|(i, lab)| {
            let row = &d[i * kk..(i + 1) * kk];
            let mut best = 0usize;
            let mut bestv = c_norm[0] - 2.0 * row[0];
            for ki in 1..kk {
                let v = c_norm[ki] - 2.0 * row[ki];
                if v < bestv {
                    bestv = v;
                    best = ki;
                }
            }
            *lab = best as i32;
        });
        labels
    }
    pub fn transform_flat(&self, flat: &[f64], n: usize, dims: usize) -> Vec<Vec<f64>> {
        use rayon::prelude::*;
        let kk = self.centroids.len();
        if n == 0 || dims == 0 || kk == 0 {
            return vec![vec![0.0; kk]; n];
        }
        let mut c_flat: Vec<f64> = Vec::with_capacity(kk * dims);
        for c in &self.centroids {
            if c.len() == dims {
                c_flat.extend_from_slice(c);
            } else {
                c_flat.extend(std::iter::repeat(0.0).take(dims));
            }
        }
        let mut c_norm = vec![0.0f64; kk];
        for ki in 0..kk {
            let row = &c_flat[ki * dims..(ki + 1) * dims];
            let mut s = 0.0;
            for &v in row {
                s += v * v;
            }
            c_norm[ki] = s;
        }
        let mut x_norm = vec![0.0f64; n];
        x_norm.par_iter_mut().enumerate().for_each(|(i, s)| {
            let row = &flat[i * dims..(i + 1) * dims];
            let mut acc = 0.0;
            for &v in row {
                acc += v * v;
            }
            *s = acc;
        });
        let mut d = vec![0.0f64; n * kk];
        if n * dims * kk < 4096 {
            d.par_chunks_mut(kk).enumerate().for_each(|(i, drow)| {
                let xrow = &flat[i * dims..(i + 1) * dims];
                for ki in 0..kk {
                    let crow = &c_flat[ki * dims..(ki + 1) * dims];
                    let mut s = 0.0;
                    for j in 0..dims {
                        s += xrow[j] * crow[j];
                    }
                    drow[ki] = s;
                }
            });
        } else {
            unsafe {
                matrixmultiply::dgemm(
                    n,
                    dims,
                    kk,
                    1.0,
                    flat.as_ptr(),
                    dims as isize,
                    1,
                    c_flat.as_ptr(),
                    1,
                    dims as isize,
                    0.0,
                    d.as_mut_ptr(),
                    kk as isize,
                    1,
                );
            }
        }
        (0..n)
            .into_par_iter()
            .map(|i| {
                let row = &d[i * kk..(i + 1) * kk];
                let xn = x_norm[i];
                (0..kk)
                    .map(|ki| {
                        let v = xn - 2.0 * row[ki] + c_norm[ki];
                        if v <= 0.0 {
                            0.0
                        } else {
                            v.sqrt()
                        }
                    })
                    .collect()
            })
            .collect()
    }
}
