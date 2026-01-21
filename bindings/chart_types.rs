use crate::plot::default::register_default_types;
use crate::plot::default::_3d::register_default_3d_types;
use std::sync::OnceLock;

pub use crate::plot::default::chart::*;

static INIT: OnceLock<()> = OnceLock::new();

pub fn init_chart_types() {
    INIT.get_or_init(|| {
        register_default_types();
        register_default_3d_types();
    });
}
