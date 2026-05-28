use std::sync::OnceLock;

#[derive(Clone, Copy, serde::Serialize)]
pub enum MemTier {
    Constrained,
    Normal,
    Generous,
}

#[derive(Clone, Copy, serde::Serialize)]
pub struct HwProfile {
    pub cpu_threads: usize,
    pub par_threshold: usize,
    pub l2_chunk_elems: usize,
    pub mem_tier: MemTier,
}

static HW: OnceLock<HwProfile> = OnceLock::new();

#[inline(always)]
pub fn hw() -> &'static HwProfile {
    HW.get_or_init(detect)
}

fn detect() -> HwProfile {
    let threads = rayon::current_num_threads().max(1);
    let par_threshold = if threads == 1 {
        usize::MAX
    } else if threads <= 2 {
        65_536
    } else if threads <= 4 {
        32_768
    } else if threads <= 8 {
        8_192
    } else {
        4_096
    };
    let l2_chunk_elems = 32_768;
    let mem_tier = match std::env::var("SERA_MEM_TIER").as_deref() {
        Ok("low") => MemTier::Constrained,
        Ok("high") => MemTier::Generous,
        _ => MemTier::Normal,
    };
    HwProfile { cpu_threads: threads, par_threshold, l2_chunk_elems, mem_tier }
}