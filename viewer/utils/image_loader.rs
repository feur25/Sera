use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct ImageLoader {
    cache: Arc<Mutex<HashMap<String, egui::ColorImage>>>,
    download_dir: String,
}

impl ImageLoader {
    pub fn new() -> Self {
        let download_dir = format!(
            "{}/.seraplot_cache",
            std::env::var("TEMP").unwrap_or_else(|_| ".".to_string())
        );
        std::fs::create_dir_all(&download_dir).ok();

        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            download_dir,
        }
    }

    pub fn load_image(&self, path: &str) -> Option<egui::ColorImage> {
        let cache_key = path.to_string();
        
        {
            let cache = self.cache.lock().unwrap();
            if let Some(img) = cache.get(&cache_key) {
                return Some(img.clone());
            }
        }

        let color_img = if path.starts_with("http://") || path.starts_with("https://") {
            self.load_web_image(path)?
        } else {
            self.load_local_image(path)?
        };

        {
            let mut cache = self.cache.lock().unwrap();
            cache.insert(cache_key, color_img.clone());
        }

        Some(color_img)
    }

    fn load_local_image(&self, path: &str) -> Option<egui::ColorImage> {
        let image = image::open(path).ok()?;
        let rgba = image.to_rgba8();
        let (w, h) = rgba.dimensions();
        let pixels = rgba.into_raw();

        Some(egui::ColorImage::from_rgba_unmultiplied(
            [w as usize, h as usize],
            &pixels,
        ))
    }

    fn load_web_image(&self, url: &str) -> Option<egui::ColorImage> {
        let filename = url.split('/').last().unwrap_or("image.png");
        let cache_path = format!("{}/{}", self.download_dir, filename);

        if Path::new(&cache_path).exists() {
            return self.load_local_image(&cache_path);
        }

        let rt = tokio::runtime::Runtime::new().ok()?;
        let bytes = rt.block_on(async {
            reqwest::get(url)
                .await
                .ok()?
                .bytes()
                .await
                .ok()
        })?;

        std::fs::write(&cache_path, &bytes).ok()?;
        self.load_local_image(&cache_path)
    }
}


