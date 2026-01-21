use crate::plot::types::register_default_types;
use crate::plot::types::_3d::register_default_3d_types;
use std::sync::OnceLock;

pub use crate::plot::types::chart::*;

static INIT: OnceLock<()> = OnceLock::new();

pub fn init_chart_types() {
    INIT.get_or_init(|| {
        register_default_types();
        register_default_3d_types();
    });
}
