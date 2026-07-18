mod annotate;
mod html;
mod parse;
mod types;

pub use annotate::apply_annotations;
pub use html::{apply, apply_bg3d, apply_h, build_html_chart};
pub use parse::{parse_all, parse_args, parse_opts, sanitize_non_finite_json};
pub use types::{Annotation, ChartArgs, ChartOpts};
