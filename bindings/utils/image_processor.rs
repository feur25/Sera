use std::path::Path;

pub struct ImageProcessor;

impl ImageProcessor {
    pub fn load_as_base64(path: &str) -> Option<String> {
        if path.is_empty() {
            return None;
        }

        if path.starts_with("http://") || path.starts_with("https://") {
            return None;
        }

        let bytes = std::fs::read(path).ok()?;
        Some(Self::to_base64(&bytes))
    }

    fn to_base64(data: &[u8]) -> String {
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode(data)
    }

    pub fn get_mime_type(path: &str) -> &'static str {
        if path.ends_with(".png") {
            "image/png"
        } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
            "image/jpeg"
        } else if path.ends_with(".gif") {
            "image/gif"
        } else if path.ends_with(".webp") {
            "image/webp"
        } else {
            "image/png"
        }
    }

    pub fn to_data_url(path: &str) -> Option<String> {
        let base64 = Self::load_as_base64(path)?;
        let mime = Self::get_mime_type(path);
        Some(format!("data:{};base64,{}", mime, base64))
    }
}
