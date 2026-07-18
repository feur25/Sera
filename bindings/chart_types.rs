use crate::plot::default::_3d::register_default_3d_types;
use crate::plot::default::register_default_types;
use crate::plot::map::register_map_3d_types;
use crate::plot::map::register_map_types;
use crate::plot::seaborn::register_seaborn_3d_types;
use crate::plot::seaborn::register_seaborn_types;
use crate::plot::statistical::register_statistical_3d_types;
use crate::plot::statistical::register_statistical_types;

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

type GroupLoader = fn();

static LOADERS: OnceLock<Mutex<HashMap<String, GroupLoader>>> = OnceLock::new();
static INIT: OnceLock<()> = OnceLock::new();
static LOADERS_LOCK_WARNED: AtomicBool = AtomicBool::new(false);

fn warn_loaders_lock_once(caller: &str) {
    if !LOADERS_LOCK_WARNED.swap(true, Ordering::Relaxed) {
        eprintln!(
            "seraplot: chart-type group loader registry mutex poisoned in {caller} -- chart type \
             groups will silently fail to register/load from here on; this warning prints once"
        );
    }
}

static LIST_PLOT: &[(&str, GroupLoader)] = &[
    ("default", default_group_loader),
    ("map", map_group_loader),
    ("seaborn", seaborn_group_loader),
    ("statistical", statistical_group_loader),
];

fn get_loaders() -> &'static Mutex<HashMap<String, GroupLoader>> {
    LOADERS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn default_group_loader() {
    register_default_types();
    register_default_3d_types();
}

fn map_group_loader() {
    register_map_types();
    register_map_3d_types();
}

fn seaborn_group_loader() {
    register_seaborn_types();
    register_seaborn_3d_types();
}

fn statistical_group_loader() {
    register_statistical_types();
    register_statistical_3d_types();
}

fn register_chart_type(name: &str, loader: GroupLoader) {
    register_group_loader(name, loader);
}

pub fn register_group_loader(name: &str, loader: GroupLoader) {
    match get_loaders().lock() {
        Ok(mut loaders) => {
            loaders.insert(name.to_string(), loader);
        }
        Err(_) => warn_loaders_lock_once("register_group_loader"),
    }
}

pub fn load_group(name: &str) {
    match get_loaders().lock() {
        Ok(loaders) => {
            if let Some(loader) = loaders.get(name) {
                loader();
            }
        }
        Err(_) => warn_loaders_lock_once("load_group"),
    }
}

pub fn init_chart_types() {
    INIT.get_or_init(|| {
        for (name, loader) in LIST_PLOT {
            register_chart_type(name, *loader);
        }
    });
}