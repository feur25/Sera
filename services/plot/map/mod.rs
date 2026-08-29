pub mod _3d;
pub mod bubble_map;
pub mod chart;
pub mod choropleth;
pub mod regions;
pub mod svg_parser;
pub mod world_data;

pub use _3d::register_map_3d_types;
pub use _3d::*;
pub use bubble_map::build_bubble_map;
pub use bubble_map::{
    render_bubble_map, render_bubble_map_fast, render_bubble_map_fast_for, render_bubble_map_html,
    render_bubble_map_html_for,
};
pub use chart::register_map_types;
pub use choropleth::build_choropleth;
pub use choropleth::{
    render_choropleth, render_choropleth_fast, render_choropleth_fast_for, render_choropleth_html,
    render_choropleth_html_for,
};
