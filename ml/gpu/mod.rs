use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Backend {
    Cpu,
    Cuda,
    Metal,
    Rocm,
    Auto,
}

impl Backend {
    pub fn from_str(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "cuda" | "gpu" | "nvidia" => Self::Cuda,
            "metal" | "mps" | "apple" => Self::Metal,
            "rocm" | "amd" => Self::Rocm,
            "auto" => Self::Auto,
            _ => Self::Cpu,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Self::Cpu => "cpu",
            Self::Cuda => "cuda",
            Self::Metal => "metal",
            Self::Rocm => "rocm",
            Self::Auto => "auto",
        }
    }
}

#[derive(Clone, Debug)]
pub struct DeviceInfo {
    pub backend: Backend,
    pub name: String,
    pub mem_mb: u64,
    pub available: bool,
}

pub fn detect_devices() -> Vec<DeviceInfo> {
    let mut v = vec![DeviceInfo {
        backend: Backend::Cpu,
        name: cpu_name(),
        mem_mb: total_mem_mb(),
        available: true,
    }];
    if let Some(d) = detect_cuda() { v.push(d); }
    if let Some(d) = detect_metal() { v.push(d); }
    if let Some(d) = detect_rocm() { v.push(d); }
    v
}

pub fn select(backend: Backend) -> DeviceInfo {
    let devices = detect_devices();
    if matches!(backend, Backend::Auto) {
        devices.iter().rev().find(|d| d.available).cloned().unwrap_or_else(|| devices[0].clone())
    } else {
        devices.into_iter().find(|d| d.backend == backend && d.available).unwrap_or(DeviceInfo {
            backend: Backend::Cpu, name: cpu_name(), mem_mb: total_mem_mb(), available: true,
        })
    }
}

static ACTIVE: OnceLock<std::sync::Mutex<Backend>> = OnceLock::new();
fn active_lock() -> &'static std::sync::Mutex<Backend> { ACTIVE.get_or_init(|| std::sync::Mutex::new(Backend::Cpu)) }

pub fn set_active(b: Backend) { *active_lock().lock().unwrap() = b; }
pub fn active() -> Backend { *active_lock().lock().unwrap() }

fn cpu_name() -> String {
    std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "cpu".to_string())
}

fn total_mem_mb() -> u64 {
    let n = num_cpus();
    (n as u64) * 2048
}

pub fn num_cpus() -> usize { rayon::current_num_threads().max(1) }

fn detect_cuda() -> Option<DeviceInfo> {
    #[cfg(feature = "cuda")]
    {
        if let Ok(d) = candle_core::Device::new_cuda(0) {
            let _ = d;
            return Some(DeviceInfo {
                backend: Backend::Cuda,
                name: "cuda:0 (candle)".into(),
                mem_mb: 0,
                available: true,
            });
        }
    }
    let p = std::env::var("CUDA_PATH").ok().or_else(|| std::env::var("CUDA_HOME").ok());
    if p.is_some() || std::path::Path::new("/usr/local/cuda").exists() {
        Some(DeviceInfo { backend: Backend::Cuda, name: "cuda-detected".into(), mem_mb: 0, available: false })
    } else { None }
}

fn detect_metal() -> Option<DeviceInfo> {
    #[cfg(feature = "metal")]
    {
        if let Ok(d) = candle_core::Device::new_metal(0) {
            let _ = d;
            return Some(DeviceInfo {
                backend: Backend::Metal,
                name: "metal:0 (candle)".into(),
                mem_mb: 0,
                available: true,
            });
        }
    }
    if cfg!(target_os = "macos") {
        Some(DeviceInfo { backend: Backend::Metal, name: "apple-gpu".into(), mem_mb: 0, available: false })
    } else { None }
}

fn detect_rocm() -> Option<DeviceInfo> {
    if std::path::Path::new("/opt/rocm").exists() {
        Some(DeviceInfo { backend: Backend::Rocm, name: "rocm".into(), mem_mb: 0, available: false })
    } else { None }
}


