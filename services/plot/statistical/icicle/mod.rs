use crate::plot::{apply, parse_all};
pub mod basic;
pub mod common;
pub mod config;
pub mod gapped;
pub mod horizontal;
pub mod radial;
pub mod rank;
pub mod variant;

pub use config::IcicleConfig;
pub use variant::IcicleVariant;

pub fn render_icicle_html(cfg: &IcicleConfig) -> String {
    use variant::IcicleVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Gapped => gapped::render(cfg),
        Horizontal => horizontal::render(cfg),
        Radial => radial::render(cfg),
        Rank => rank::render(cfg),
    }
}

pub use build as build_icicle;

const ICICLE_COLOR_LOW: u32 = 0x313695;
const ICICLE_COLOR_HIGH: u32 = 0xA50026;
const ICICLE_COLOR_SCALE: i32 = 1000;

pub fn render_icicle_family_native(
    title: &str,
    labels: &[String],
    parents: &[String],
    values: &[f64],
    opts: &crate::plot::canvas_points::NativeChartOpts,
) -> (String, u64, f64, f64) {
    use crate::html::hover::{html_id, html_prefix, html_suffix};
    use crate::plot::canvas_points::{pack_scalar_i16, push_color_patch_js};
    use crate::plot::statistical::common::push_b;
    use common::{node_rect, prepare, rect_attrs};

    let hid = html_id();
    let cfg = IcicleConfig {
        title,
        variant: IcicleVariant::Basic,
        labels,
        parents,
        values,
        palette: &[],
        width: opts.width,
        height: opts.height,
        ..IcicleConfig::default()
    };

    let Some(p) = prepare(&cfg) else {
        let mut buf = Vec::<u8>::new();
        html_prefix(&mut buf, title, hid);
        html_suffix(&mut buf, hid, "[]");
        return (unsafe { String::from_utf8_unchecked(buf) }, hid, 0.0, 1.0);
    };

    let (min_v, max_v) = crate::bindings::utils::simd_ops::find_minmax(&p.values_eff);
    let range_v = (max_v - min_v).max(1e-12);
    let values_px = pack_scalar_i16(&p.values_eff, min_v, range_v, ICICLE_COLOR_SCALE);

    let svg_id = format!("spicsvg{hid}");
    let mut buf = Vec::<u8>::with_capacity(p.n * 60 + 8192);
    html_prefix(&mut buf, title, hid);
    push_b(&mut buf, b"<svg id=\"");
    buf.extend_from_slice(svg_id.as_bytes());
    push_b(&mut buf, b"\" width=\"");
    buf.extend_from_slice(opts.width.to_string().as_bytes());
    push_b(&mut buf, b"\" height=\"");
    buf.extend_from_slice(opts.height.to_string().as_bytes());
    push_b(&mut buf, b"\" style=\"display:block\">");

    if !title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        buf.extend_from_slice((opts.width / 2).to_string().as_bytes());
        push_b(&mut buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-weight=\"700\" font-size=\"15\" fill=\"#1a202c\">");
        crate::plot::statistical::common::escape_xml(&mut buf, title);
        push_b(&mut buf, b"</text>");
    }

    for &i in &p.bfs_order {
        let r = node_rect(&p, i);
        if r.w < 0.5 {
            continue;
        }
        push_b(&mut buf, b"<rect class=\"cell\" data-idx=\"");
        buf.extend_from_slice(i.to_string().as_bytes());
        push_b(&mut buf, b"\"");
        rect_attrs(&mut buf, r);
        push_b(&mut buf, b" fill=\"#eeeeee\" stroke=\"#fff\" stroke-width=\"1\"/>");
    }
    push_b(&mut buf, b"</svg>");

    push_b(&mut buf, b"<script>(function(){");
    push_b(&mut buf, b"var svg=document.getElementById('");
    buf.extend_from_slice(svg_id.as_bytes());
    push_b(&mut buf, b"');");
    push_b(&mut buf, b"var CELLS=[];var els=svg.querySelectorAll('rect.cell');");
    push_b(&mut buf, b"for(var k=0;k<els.length;k++){CELLS[parseInt(els[k].getAttribute('data-idx'))]=els[k];}");
    push_b(&mut buf, b"function b64(s){var b=atob(s),n=b.length,a=new Int16Array(n/2);for(var i=0;i<n;i+=2)a[i/2]=b.charCodeAt(i)|(b.charCodeAt(i+1)<<8);return a;}");
    push_b(&mut buf, b"var V=b64('");
    buf.extend_from_slice(values_px.as_bytes());
    push_b(&mut buf, b"');");
    push_color_patch_js(&mut buf, hid, b"CELLS", ICICLE_COLOR_SCALE, ICICLE_COLOR_LOW, ICICLE_COLOR_HIGH);
    push_b(&mut buf, b"window['sp_apply_");
    buf.extend_from_slice(hid.to_string().as_bytes());
    push_b(&mut buf, b"'](Array.from({length:V.length},function(_,i){return i;}),V);");
    push_b(&mut buf, b"})();</script>");

    html_suffix(&mut buf, hid, "[]");
    (unsafe { String::from_utf8_unchecked(buf) }, hid, min_v, range_v)
}

#[crate::sera_alias("icicle", "icicles", "icicle_chart", "icicle_family")]
#[crate::sera_builder("build_icicle")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let parents = a.parents.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{render_icicle_html, IcicleConfig, IcicleVariant};
    let hover = o.hj();
    let variant = IcicleVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_icicle_html(&IcicleConfig {
        title,
        variant,
        labels: &labels,
        parents: &parents,
        values: &values,
        palette: &o.pal(),
        width: o.w(760),
        height: o.h(520),
        hover: &hover,
        ..IcicleConfig::default()
    });
    apply(html, &o)
}
