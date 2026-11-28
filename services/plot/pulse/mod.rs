pub mod board_sessions;
pub mod push_registry;
pub mod record_registry;
pub mod frame_codec;
pub mod token_bucket;
pub mod studio;
pub mod ffi_bridge;
#[cfg(feature = "python")]
pub mod anomaly_registry;
pub mod chart_source_registry;
pub mod firehose_registry;
pub mod live_marks;
pub mod live_registry;
pub mod ring_buffer;
#[cfg(feature = "python")]
pub mod chart_methods;
#[cfg(feature = "python")]
pub mod features;

pub fn now_ms() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0)
}
