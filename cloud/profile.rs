use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Resources {
    pub cpu_threads: usize,
    pub backend: String,
    pub os: String,
    pub arch: String,
    pub registry_dir: String,
    pub tasks_dir: String,
}

pub fn current() -> Resources {
    let registry_dir = crate::ml::registry::registry_dir().to_string_lossy().to_string();
    let tasks_dir = std::env::var("SERAPLOT_TASKS_DIR")
        .unwrap_or_else(|_| std::env::temp_dir().join("seraplot_tasks").to_string_lossy().to_string());
    Resources {
        cpu_threads: rayon::current_num_threads(),
        backend: crate::ml::gpu::active().name().to_string(),
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        registry_dir,
        tasks_dir,
    }
}

pub fn to_json(r: &Resources) -> String {
    serde_json::to_string_pretty(r).unwrap_or_else(|_| "{}".to_string())
}
