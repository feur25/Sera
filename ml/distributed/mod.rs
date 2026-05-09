use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Shard {
    pub id: u64,
    pub start: usize,
    pub end: usize,
}

pub struct WorkerPool {
    pub n_workers: usize,
    shards: Mutex<HashMap<u64, Vec<Shard>>>,
    counter: Mutex<u64>,
}

impl WorkerPool {
    pub fn new(n_workers: usize) -> Arc<Self> {
        let n = if n_workers == 0 { rayon::current_num_threads() } else { n_workers };
        Arc::new(Self { n_workers: n, shards: Mutex::new(HashMap::new()), counter: Mutex::new(1) })
    }

    pub fn scatter(&self, n_rows: usize) -> u64 {
        let mut id = self.counter.lock().unwrap();
        let key = *id; *id += 1; drop(id);
        let chunk = (n_rows + self.n_workers - 1) / self.n_workers.max(1);
        let mut out = Vec::with_capacity(self.n_workers);
        let mut s = 0;
        let mut sid = 0u64;
        while s < n_rows {
            let e = (s + chunk).min(n_rows);
            out.push(Shard { id: sid, start: s, end: e });
            sid += 1; s = e;
        }
        self.shards.lock().unwrap().insert(key, out);
        key
    }

    pub fn shards_for(&self, handle: u64) -> Vec<Shard> {
        self.shards.lock().unwrap().get(&handle).cloned().unwrap_or_default()
    }

    pub fn release(&self, handle: u64) { self.shards.lock().unwrap().remove(&handle); }

    pub fn parallel_map<T, F>(&self, items: Vec<T>, f: F) -> Vec<T>
    where T: Send, F: Fn(T) -> T + Send + Sync {
        items.into_par_iter().map(f).collect()
    }

    pub fn parallel_train<M, F>(&self, n_models: usize, factory: F) -> Vec<M>
    where M: Send, F: Fn(usize) -> M + Send + Sync {
        (0..n_models).into_par_iter().map(|i| factory(i)).collect()
    }

    pub fn allreduce_sum(&self, vecs: Vec<Vec<f64>>) -> Vec<f64> {
        if vecs.is_empty() { return Vec::new(); }
        let len = vecs[0].len();
        let mut out = vec![0.0; len];
        for v in vecs {
            for (o, x) in out.iter_mut().zip(v.iter()) { *o += *x; }
        }
        out
    }

    pub fn allreduce_mean(&self, vecs: Vec<Vec<f64>>) -> Vec<f64> {
        let n = vecs.len() as f64;
        if n == 0.0 { return Vec::new(); }
        let mut s = self.allreduce_sum(vecs);
        for v in s.iter_mut() { *v /= n; }
        s
    }
}

pub struct DataParallelTrainer<M> {
    pub pool: Arc<WorkerPool>,
    pub _phantom: std::marker::PhantomData<M>,
}

impl<M: Send + Sync> DataParallelTrainer<M> {
    pub fn new(n_workers: usize) -> Self {
        Self { pool: WorkerPool::new(n_workers), _phantom: std::marker::PhantomData }
    }

    pub fn fit_chunks<F>(&self, n_rows: usize, factory: F) -> Vec<M>
    where F: Fn(usize, usize, usize) -> M + Send + Sync {
        let h = self.pool.scatter(n_rows);
        let shards = self.pool.shards_for(h);
        let out: Vec<M> = shards.into_par_iter().enumerate().map(|(i, s)| factory(i, s.start, s.end)).collect();
        self.pool.release(h);
        out
    }
}


