use crate::plot::default::register_default_types;
use crate::plot::default::_3d::register_default_3d_types;
use std::sync::{OnceLock, Mutex};
use std::collections::HashMap;

pub use crate::plot::default::chart::*;

type GroupLoader = fn();

static LOADERS: OnceLock<Mutex<HashMap<String, GroupLoader>>> = OnceLock::new();
static INIT: OnceLock<()> = OnceLock::new();

fn get_loaders() -> &'static Mutex<HashMap<String, GroupLoader>> {
    LOADERS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn default_group_loader() {
    register_default_types();
    register_default_3d_types();
}

pub fn register_group_loader(name: &str, loader: GroupLoader) {
    if let Ok(mut loaders) = get_loaders().lock() {
        loaders.insert(name.to_string(), loader);
    }
}

pub fn load_group(name: &str) {
    if let Ok(loaders) = get_loaders().lock() {
        if let Some(loader) = loaders.get(name) {
            loader();
        }
    }
}

pub fn init_chart_types() {
    INIT.get_or_init(|| {
        register_group_loader("default", default_group_loader);
    });
}
