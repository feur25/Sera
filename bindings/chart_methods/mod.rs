pub(crate) mod apply;
pub(crate) mod compose;
pub(crate) mod methods;
pub(crate) mod extras;
pub(crate) mod interactive;
pub(crate) mod layout;
#[allow(dead_code)]
pub(crate) mod js;
pub(crate) mod pages;

pub(crate) use apply::*;

#[cfg(feature = "python")]
pub(crate) use pages::{build_compare_page, cmp_score};
pub(crate) use pages::{build_grid_page, chart_iframe};
