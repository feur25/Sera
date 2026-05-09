use std::collections::HashMap;
use egui::Color32;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CacheKey {
    pub chart_type: u8,
    pub is_3d: bool,
    pub vertical: bool,
    pub sort_mode: i32,
}

pub struct ProjectionCache {
    positions: Vec<egui::Pos2>,
    key: CacheKey,
}

pub struct RenderCache {
    projections: HashMap<CacheKey, ProjectionCache>,
    active_key: Option<CacheKey>,
}

impl RenderCache {
    pub fn new() -> Self {
        Self {
            projections: HashMap::new(),
            active_key: None,
        }
    }

    pub fn get_or_create_positions(
        &mut self,
        key: CacheKey,
        compute_fn: impl FnOnce() -> Vec<egui::Pos2>,
    ) -> Vec<egui::Pos2> {
        self.active_key = Some(key);
        
        if let Some(cache) = self.projections.get(&key) {
            return cache.positions.clone();
        }

        let positions = compute_fn();
        self.projections.insert(
            key,
            ProjectionCache {
                positions: positions.clone(),
                key,
            },
        );
        positions
    }

    pub fn update_active(&mut self, key: CacheKey) {
        if self.active_key != Some(key) {
            self.active_key = Some(key);
        }
    }

    pub fn invalidate_except(&mut self, key: CacheKey) {
        self.projections.retain(|k, _| *k == key);
        self.active_key = Some(key);
    }

    pub fn clear(&mut self) {
        self.projections.clear();
        self.active_key = None;
    }

    pub fn memory_usage(&self) -> usize {
        self.projections.values().map(|c| c.positions.len() * 16).sum()
    }
}

impl Default for RenderCache {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ColorCache {
    colors: Vec<Color32>,
}

impl ColorCache {
    pub fn new() -> Self {
        Self {
            colors: vec![
                Color32::from_rgb(31, 119, 180),
                Color32::from_rgb(255, 127, 14),
                Color32::from_rgb(44, 160, 44),
                Color32::from_rgb(214, 39, 40),
                Color32::from_rgb(148, 103, 189),
                Color32::from_rgb(140, 86, 75),
                Color32::from_rgb(227, 119, 194),
                Color32::from_rgb(127, 127, 127),
                Color32::from_rgb(188, 143, 143),
                Color32::from_rgb(23, 190, 207),
            ],
        }
    }

    pub fn colors(&self) -> &[Color32] {
        &self.colors
    }
}

impl Default for ColorCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_key_equality() {
        let key1 = CacheKey { chart_type: 0, is_3d: false, vertical: true, sort_mode: 0 };
        let key2 = CacheKey { chart_type: 0, is_3d: false, vertical: true, sort_mode: 0 };
        let key3 = CacheKey { chart_type: 0, is_3d: true, vertical: true, sort_mode: 0 };
        
        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_render_cache_basic() {
        let mut cache = RenderCache::new();
        let key = CacheKey { chart_type: 0, is_3d: false, vertical: true, sort_mode: 0 };
        
        let positions = cache.get_or_create_positions(key, || {
            vec![egui::pos2(0.0, 0.0), egui::pos2(100.0, 50.0)]
        });
        
        assert_eq!(positions.len(), 2);
        assert_eq!(cache.active_key, Some(key));
    }

    #[test]
    fn test_render_cache_hit() {
        let mut cache = RenderCache::new();
        let key = CacheKey { chart_type: 0, is_3d: false, vertical: true, sort_mode: 0 };
        
        let pos1 = cache.get_or_create_positions(key, || {
            vec![egui::pos2(0.0, 0.0)]
        });
        
        let pos2 = cache.get_or_create_positions(key, || {
            vec![egui::pos2(99.0, 99.0)]
        });
        
        assert_eq!(pos1, pos2);
    }

    #[test]
    fn test_render_cache_update_active() {
        let mut cache = RenderCache::new();
        let key1 = CacheKey { chart_type: 0, is_3d: false, vertical: true, sort_mode: 0 };
        let key2 = CacheKey { chart_type: 1, is_3d: false, vertical: true, sort_mode: 0 };
        
        cache.get_or_create_positions(key1, || vec![egui::pos2(0.0, 0.0)]);
        cache.update_active(key2);
        
        assert_eq!(cache.active_key, Some(key2));
    }

    #[test]
    fn test_render_cache_invalidate_except() {
        let mut cache = RenderCache::new();
        let key1 = CacheKey { chart_type: 0, is_3d: false, vertical: true, sort_mode: 0 };
        let key2 = CacheKey { chart_type: 1, is_3d: false, vertical: true, sort_mode: 0 };
        let key3 = CacheKey { chart_type: 2, is_3d: false, vertical: true, sort_mode: 0 };
        
        cache.get_or_create_positions(key1, || vec![egui::pos2(0.0, 0.0)]);
        cache.get_or_create_positions(key2, || vec![egui::pos2(1.0, 1.0)]);
        cache.get_or_create_positions(key3, || vec![egui::pos2(2.0, 2.0)]);
        
        cache.invalidate_except(key2);
        
        assert_eq!(cache.projections.len(), 1);
        assert!(cache.projections.contains_key(&key2));
    }

    #[test]
    fn test_color_cache() {
        let cache = ColorCache::new();
        let colors = cache.colors();
        
        assert_eq!(colors.len(), 10);
    }
}


