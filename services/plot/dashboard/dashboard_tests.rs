use super::canvas_core::Canvas;
use super::element::{El, Layer};
use super::geometry::polar_xy;
use super::render::render_el;

#[test]
fn polar_xy_places_cardinal_points_clockwise_from_top() {
    let (x, y) = polar_xy(100.0, 100.0, 50.0, 0.0);
    assert!((x - 100.0).abs() < 1e-9 && (y - 50.0).abs() < 1e-9);

    let (x, y) = polar_xy(100.0, 100.0, 50.0, 90.0);
    assert!((x - 150.0).abs() < 1e-9 && (y - 100.0).abs() < 1e-9);

    let (x, y) = polar_xy(100.0, 100.0, 50.0, 180.0);
    assert!((x - 100.0).abs() < 1e-9 && (y - 150.0).abs() < 1e-9);

    let (x, y) = polar_xy(100.0, 100.0, 50.0, 270.0);
    assert!((x - 50.0).abs() < 1e-9 && (y - 100.0).abs() < 1e-9);
}

#[test]
fn arc_renders_a_single_path_with_the_right_large_arc_flag() {
    let mut defs = String::new();
    let mut body = String::new();
    let short = El::Arc {
        cx: 0.0, cy: 0.0, r: 10.0, start_deg: 0.0, end_deg: 90.0,
        color: "#fff".into(), width: 1.0, opacity: 1.0, cap: "round".into(),
        layer: Layer::Fg, name: String::new(),
    };
    render_el(&short, &mut defs, &mut body);
    assert!(body.contains(" A 10.00,10.00 0 0,1 "));

    body.clear();
    let long = El::Arc {
        cx: 0.0, cy: 0.0, r: 10.0, start_deg: 0.0, end_deg: 270.0,
        color: "#fff".into(), width: 1.0, opacity: 1.0, cap: "round".into(),
        layer: Layer::Fg, name: String::new(),
    };
    render_el(&long, &mut defs, &mut body);
    assert!(body.contains(" A 10.00,10.00 0 1,1 "));
}

#[test]
fn wedge_with_zero_inner_radius_starts_from_center() {
    let mut defs = String::new();
    let mut body = String::new();
    let pie_slice = El::Wedge {
        cx: 0.0, cy: 0.0, r_inner: 0.0, r_outer: 10.0, start_deg: 0.0, end_deg: 90.0,
        fill: "#fff".into(), stroke: "none".into(), sw: 0.0, opacity: 1.0,
        layer: Layer::Fg, group: String::new(), name: String::new(),
    };
    render_el(&pie_slice, &mut defs, &mut body);
    assert!(body.starts_with("<path d=\"M 0.00,0.00 L "));
}

#[test]
fn wedge_with_nonzero_inner_radius_forms_a_closed_donut_segment() {
    let mut defs = String::new();
    let mut body = String::new();
    let segment = El::Wedge {
        cx: 0.0, cy: 0.0, r_inner: 5.0, r_outer: 10.0, start_deg: 0.0, end_deg: 90.0,
        fill: "#fff".into(), stroke: "none".into(), sw: 0.0, opacity: 1.0,
        layer: Layer::Fg, group: String::new(), name: String::new(),
    };
    render_el(&segment, &mut defs, &mut body);
    assert!(body.contains(" Z\""));
    assert!(!body.starts_with("<path d=\"M 0.00,0.00 L "));
}

#[test]
fn ribbon_connects_two_arc_spans_through_the_center() {
    let mut defs = String::new();
    let mut body = String::new();
    let rib = El::Ribbon {
        cx: 0.0, cy: 0.0, r: 10.0, a_start: 0.0, a_end: 30.0, b_start: 180.0, b_end: 210.0,
        fill: "#fff".into(), opacity: 0.7, layer: Layer::Fg, name: String::new(),
    };
    render_el(&rib, &mut defs, &mut body);
    assert_eq!(body.matches(" Q 0.00,0.00 ").count(), 2);
}

#[test]
fn preview_html_centers_via_flex_instead_of_shrinking_the_document_box() {
    let cv = Canvas::new(900, 540, "#0a0a0f");
    let html = cv.build().html;
    assert!(html.contains("display:flex;align-items:center;justify-content:center"));
    assert!(!html.contains("document.body.style.width"));
    assert!(!html.contains("document.documentElement.style.width"));
}

#[test]
fn radial_gradient_emits_a_radial_gradient_def_not_linear() {
    let mut defs = String::new();
    let mut body = String::new();
    let grad = El::RadialGradDef {
        id: "glow".into(), from_color: "#fff".into(), to_color: "#000".into(),
        cx: 0.5, cy: 0.5, r: 0.6,
    };
    render_el(&grad, &mut defs, &mut body);
    assert!(defs.contains("<radialGradient id=\"glow\""));
    assert!(body.is_empty());
}
