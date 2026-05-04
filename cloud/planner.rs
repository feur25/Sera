use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct ScalePlan {
    pub n_rows: usize,
    pub n_cols: usize,
    pub bytes_total: u64,
    pub mem_budget_mb: u64,
    pub recommended_workers: usize,
    pub recommended_chunk_rows: usize,
    pub n_chunks: usize,
    pub estimated_seconds: f64,
    pub strategy: String,
}

pub fn plan(n_rows: usize, n_cols: usize, mem_budget_mb: u64) -> ScalePlan {
    let bytes_total = (n_rows as u64) * (n_cols as u64) * 8;
    let workers = rayon::current_num_threads().max(1);
    let mem_bytes = mem_budget_mb.saturating_mul(1024 * 1024);
    let chunk_rows = if n_cols == 0 { n_rows } else {
        let max_rows_in_mem = (mem_bytes / 8 / n_cols as u64) as usize;
        let target = (n_rows / workers.max(1)).max(1024);
        target.min(max_rows_in_mem.max(1024)).max(1024).min(n_rows.max(1))
    };
    let n_chunks = if chunk_rows == 0 { 1 } else { (n_rows + chunk_rows - 1) / chunk_rows };
    let cells = (n_rows as f64) * (n_cols as f64);
    let throughput = 1.0e8 * (workers as f64);
    let estimated_seconds = if throughput > 0.0 { cells / throughput } else { 0.0 };
    let strategy = if bytes_total < 64 * 1024 * 1024 { "in_memory".into() }
        else if bytes_total < 2u64 * 1024 * 1024 * 1024 { "chunked".into() }
        else { "streamed".into() };
    ScalePlan {
        n_rows, n_cols, bytes_total, mem_budget_mb,
        recommended_workers: workers,
        recommended_chunk_rows: chunk_rows,
        n_chunks,
        estimated_seconds,
        strategy,
    }
}

pub fn to_json(p: &ScalePlan) -> String {
    serde_json::to_string_pretty(p).unwrap_or_else(|_| "{}".to_string())
}
