pub mod builders;

pub use builders::*;
pub use crate::plot::{set_global_bg, set_global_pal, set_global_grid, get_global_bg, get_global_pal, get_global_grid, set_global_background, reset_global_background, set_theme, reset_theme, themes, build_slideshow, build_hover_json, chart_append, export_svg, export_data_url, export_html_file, chart_info, validate_input, downsample_lttb, chart_diff, drift_ks, bench_chart_value, bench_pure_rust, scale_plan, system_profile, csv_count_rows, csv_chunk_read, show_chart_value, set_chart_kind, set_chart_orientation};
pub use crate::plot::{ChartOpts, ChartArgs, Annotation, parse_all, parse_opts, parse_args, apply, apply_h, apply_bg3d, apply_annotations, build_html_chart};
