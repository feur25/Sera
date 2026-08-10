use crate::plot::{apply, parse_all};
pub mod basic;
pub mod common;
pub mod config;
pub mod heikin;
pub mod hollow;
pub mod indicators;
pub mod line;
pub mod milestone;
pub mod mountain;
pub mod ohlc;
pub mod outlined;
pub mod range;
pub mod variant;
pub mod volume;

pub use config::CandlestickConfig;
pub use variant::CandlestickVariant;

pub fn render_candlestick_html(cfg: &CandlestickConfig) -> String {
    use variant::CandlestickVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Hollow => hollow::render(cfg),
        Ohlc => ohlc::render(cfg),
        Heikin => heikin::render(cfg),
        Outlined => outlined::render(cfg),
        Line => line::render(cfg),
        Mountain => mountain::render(cfg),
        Range => range::render(cfg),
        Volume => volume::render(cfg),
        Milestone => milestone::render(cfg),
        Indicators => indicators::render(cfg),
    }
}

pub use build as build_candlestick;

pub fn render_candlestick_family_native(
    title: &str,
    _labels: &[String],
    open: &[f64],
    high: &[f64],
    low: &[f64],
    close: &[f64],
    opts: &crate::plot::canvas_points::NativeChartOpts,
) -> (String, u64) {
    use crate::html::hover::{html_id, html_prefix, html_suffix};
    use crate::plot::canvas_points::{pack_vector_i16, shared_minmax};
    use crate::plot::statistical::common::{escape_xml, push_b, push_f2, push_i};

    let n = open.len().min(high.len()).min(low.len()).min(close.len());
    let pad_l = 56i32;
    let pad_t = 36i32;
    let pad_b = 48i32;
    let pad_r = 20i32;
    let plot_w = (opts.width - pad_l - pad_r).max(10);
    let plot_h = (opts.height - pad_t - pad_b).max(10);
    let col_w = plot_w as f64 / n.max(1) as f64;

    let (min_val, range_val) = shared_minmax(&[open, high, low, close]);
    let values_px = pack_vector_i16(&[open, high, low, close], min_val, range_val, plot_h);

    let hid = html_id();
    let svg_id = format!("spohlcsvg{hid}");
    let mut buf = Vec::<u8>::with_capacity(n * 40 + 8192);
    html_prefix(&mut buf, title, hid);
    push_b(&mut buf, b"<svg id=\"");
    buf.extend_from_slice(svg_id.as_bytes());
    push_b(&mut buf, b"\" width=\"");
    push_i(&mut buf, opts.width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, opts.height);
    push_b(&mut buf, b"\" style=\"display:block\">");

    if !title.is_empty() {
        push_b(&mut buf, b"<text x=\"");
        push_i(&mut buf, opts.width / 2);
        push_b(&mut buf, b"\" y=\"22\" text-anchor=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-weight=\"700\" font-size=\"15\" fill=\"#1a202c\">");
        escape_xml(&mut buf, title);
        push_b(&mut buf, b"</text>");
    }

    push_b(&mut buf, b"<line x1=\"");
    push_i(&mut buf, pad_l);
    push_b(&mut buf, b"\" x2=\"");
    push_i(&mut buf, pad_l + plot_w);
    push_b(&mut buf, b"\" y1=\"");
    push_i(&mut buf, pad_t + plot_h);
    push_b(&mut buf, b"\" y2=\"");
    push_i(&mut buf, pad_t + plot_h);
    push_b(&mut buf, b"\" stroke=\"#64748b\"/>");

    push_b(&mut buf, b"<g>");
    for i in 0..n {
        let cx = pad_l as f64 + (i as f64 + 0.5) * col_w;
        push_b(&mut buf, b"<line class=\"wick\" x1=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" x2=\"");
        push_f2(&mut buf, cx);
        push_b(&mut buf, b"\" y1=\"0\" y2=\"0\" stroke=\"#999\"/>");
        push_b(&mut buf, b"<rect class=\"body\" x=\"");
        push_f2(&mut buf, cx - (col_w * 0.35));
        push_b(&mut buf, b"\" y=\"0\" width=\"");
        push_f2(&mut buf, (col_w * 0.7).max(1.0));
        push_b(&mut buf, b"\" height=\"0\" fill=\"#999\"/>");
    }
    push_b(&mut buf, b"</g></svg>");

    push_b(&mut buf, b"<script>(function(){");
    push_b(&mut buf, b"var svg=document.getElementById('");
    buf.extend_from_slice(svg_id.as_bytes());
    push_b(&mut buf, b"');");
    push_b(&mut buf, b"var WICKS=svg.querySelectorAll('line.wick');");
    push_b(&mut buf, b"var BODIES=svg.querySelectorAll('rect.body');");
    push_b(&mut buf, b"var padT=");
    push_i(&mut buf, pad_t);
    push_b(&mut buf, b",axisPx=");
    push_i(&mut buf, plot_h);
    push_b(&mut buf, b";");
    push_b(&mut buf, b"function toY(v){return padT+(axisPx-v);}");
    push_b(&mut buf, b"function paint(k,o,h,l,c){var w=WICKS[k],b=BODIES[k];if(!w||!b)return;");
    push_b(&mut buf, b"w.setAttribute('y1',toY(h));w.setAttribute('y2',toY(l));");
    push_b(&mut buf, b"var top=Math.min(toY(o),toY(c)),bot=Math.max(toY(o),toY(c));");
    push_b(&mut buf, b"b.setAttribute('y',top);b.setAttribute('height',Math.max(1,bot-top));");
    push_b(&mut buf, b"b.setAttribute('fill',c>=o?'#26a69a':'#ef5350');}");
    push_b(&mut buf, b"function b64(s){var b=atob(s),n=b.length,a=new Int16Array(n/2);for(var i=0;i<n;i+=2)a[i/2]=b.charCodeAt(i)|(b.charCodeAt(i+1)<<8);return a;}");
    push_b(&mut buf, b"var V=b64('");
    buf.extend_from_slice(values_px.as_bytes());
    push_b(&mut buf, b"');");
    push_b(&mut buf, b"for(var i=0;i<WICKS.length;i++){paint(i,V[i*4],V[i*4+1],V[i*4+2],V[i*4+3]);}");
    push_b(&mut buf, b"window['sp_apply_");
    buf.extend_from_slice(hid.to_string().as_bytes());
    push_b(&mut buf, b"']=function(idx,vals){for(var i=0;i<idx.length;i++){paint(idx[i],vals[i*4],vals[i*4+1],vals[i*4+2],vals[i*4+3]);}};");
    push_b(&mut buf, b"})();</script>");

    html_suffix(&mut buf, hid, "[]");
    (unsafe { String::from_utf8_unchecked(buf) }, hid)
}

#[crate::sera_alias(
    "candlestick",
    "candlesticks",
    "candlestick_chart",
    "candlestick_family",
    "ohlc",
    "ohlc_chart"
)]
#[crate::sera_builder("build_candlestick")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let open = a.open.unwrap_or_default();
    let high = a.high.unwrap_or_default();
    let low = a.low.unwrap_or_default();
    let close = a.close.unwrap_or_default();
    let volume = a.volume.unwrap_or_default();
    let dec = crate::plot::decimate::Decimator::new(o.max_points, &close);
    let labels = dec.apply(labels);
    let open = dec.apply(open);
    let high = dec.apply(high);
    let low = dec.apply(low);
    let close = dec.apply(close);
    let volume = dec.apply(volume);
    use crate::plot::statistical::{
        render_candlestick_html, CandlestickConfig, CandlestickVariant,
    };
    let hover = o.hj();
    let variant = CandlestickVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_candlestick_html(&CandlestickConfig {
        title,
        labels: &labels,
        open: &open,
        high: &high,
        low: &low,
        close: &close,
        volume: &volume,
        palette: &o.pal(),
        width: o.w(1100),
        height: o.h(500),
        x_label: &o.xl(),
        y_label: &o.yl(),
        gridlines: o.grid(),
        sort_order: &o.srt(),
        hover: &hover,
        variant,
        ..CandlestickConfig::default()
    });
    apply(html, &o)
}
