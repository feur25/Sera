use std::sync::atomic::AtomicUsize;

mod anchors;

static MARKER_ID: AtomicUsize = AtomicUsize::new(0);

mod geometry;
mod html_util;
mod element;
mod render;
mod canvas_core;
mod canvas_placement;
mod canvas_shapes;
mod canvas_links;
mod canvas_pins;
mod canvas_persistence;
mod canvas_build;
mod canvas_index;

#[cfg(test)]
mod dashboard_tests;

pub use canvas_core::Canvas;
pub use canvas_index::{canvas, canvas_default_dir, canvas_load, canvas_load_named, canvas_save_named};
