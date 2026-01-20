pub trait SvgChart {
    fn render_svg(
        svg: &mut String,
        values: &[f64],
        colors: &[&'static str],
        pad: i32,
        plot_width: i32,
        plot_height: i32,
        max_val: f64,
        vertical: bool,
    );
}

impl SvgChart for super::bar::Bar {
    fn render_svg(
        svg: &mut String,
        values: &[f64],
        colors: &[&'static str],
        pad: i32,
        plot_width: i32,
        plot_height: i32,
        max_val: f64,
        vertical: bool,
    ) {
        super::bar::render_svg_bars(svg, values, colors, pad, plot_width, plot_height, max_val, vertical);
    }
}

impl SvgChart for super::line::Line {
    fn render_svg(
        svg: &mut String,
        values: &[f64],
        colors: &[&'static str],
        pad: i32,
        plot_width: i32,
        plot_height: i32,
        max_val: f64,
        _vertical: bool,
    ) {
        super::line::render_svg_lines(svg, values, colors, pad, plot_width, plot_height, max_val);
    }
}

impl SvgChart for super::scatter::Scatter {
    fn render_svg(
        svg: &mut String,
        values: &[f64],
        colors: &[&'static str],
        pad: i32,
        plot_width: i32,
        plot_height: i32,
        max_val: f64,
        _vertical: bool,
    ) {
        super::scatter::render_svg_scatter(svg, values, colors, pad, plot_width, plot_height, max_val);
    }
}
