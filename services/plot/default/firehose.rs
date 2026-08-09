use crate::plot::{apply, parse_all};

fn push_js_str(buf: &mut Vec<u8>, s: &str) {
    for b in s.bytes() {
        match b {
            b'\'' => {
                buf.push(b'\\');
                buf.push(b'\'');
            }
            b'\\' => {
                buf.push(b'\\');
                buf.push(b'\\');
            }
            b'\n' => {
                buf.push(b'\\');
                buf.push(b'n');
            }
            _ => buf.push(b),
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn render_firehose_canvas_html_id(
    title: &str,
    capacity: usize,
    min_val: f64,
    max_val: f64,
    width: i32,
    height: i32,
    color_hex: u32,
    gridlines: bool,
) -> (String, u64, i32) {
    use crate::html::hover::{html_id, html_prefix, html_suffix};
    use crate::plot::statistical::common::{hex6, push_b, push_f2, push_i};
    let pad_l = 56i32;
    let pad_t = 36i32;
    let pad_b = 48i32;
    let pad_r = 20i32;
    let plot_w = (width - pad_l - pad_r).max(10);
    let plot_h = (height - pad_t - pad_b).max(10);
    let hid = html_id();
    let bg_id = format!("spfhbg{}", hid);
    let gl_id = format!("spfhgl{}", hid);
    let live_id = format!("spfhlive{}", hid);
    let hex = hex6(if color_hex != 0 { color_hex } else { 0x22D3EE });
    let mut buf = Vec::<u8>::with_capacity(4_000);
    html_prefix(&mut buf, title, hid);
    push_b(&mut buf, b"<div style=\"position:relative;display:inline-block\">");
    push_b(&mut buf, b"<canvas id=\"");
    buf.extend_from_slice(bg_id.as_bytes());
    push_b(&mut buf, b"\" width=\"");
    push_i(&mut buf, width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, height);
    push_b(&mut buf, b"\" style=\"display:block\"></canvas>");
    push_b(&mut buf, b"<canvas id=\"");
    buf.extend_from_slice(gl_id.as_bytes());
    push_b(&mut buf, b"\" width=\"");
    push_i(&mut buf, width);
    push_b(&mut buf, b"\" height=\"");
    push_i(&mut buf, height);
    push_b(&mut buf, b"\" style=\"display:block;position:absolute;left:0;top:0\"></canvas>");
    push_b(&mut buf, b"<div id=\"");
    buf.extend_from_slice(live_id.as_bytes());
    push_b(&mut buf, b"\" style=\"position:absolute;right:");
    push_i(&mut buf, pad_r + 4);
    push_b(&mut buf, b"px;top:");
    push_i(&mut buf, pad_t + 4);
    push_b(&mut buf, b"px;font:11px -apple-system,Arial,sans-serif;color:#64748b;text-align:right\"></div>");
    push_b(&mut buf, b"</div><script>(function(){");
    push_b(&mut buf, b"var bgc=document.getElementById('");
    buf.extend_from_slice(bg_id.as_bytes());
    push_b(&mut buf, b"'),bgx=bgc.getContext('2d');");
    push_b(&mut buf, b"var pL=");
    push_i(&mut buf, pad_l);
    push_b(&mut buf, b",pT=");
    push_i(&mut buf, pad_t);
    push_b(&mut buf, b",pW=");
    push_i(&mut buf, plot_w);
    push_b(&mut buf, b",pH=");
    push_i(&mut buf, plot_h);
    push_b(&mut buf, b",W=");
    push_i(&mut buf, width);
    push_b(&mut buf, b",H=");
    push_i(&mut buf, height);
    push_b(&mut buf, b",minV=");
    push_f2(&mut buf, min_val);
    push_b(&mut buf, b",maxV=");
    push_f2(&mut buf, max_val);
    push_b(&mut buf, b",AX=pH;");
    push_b(&mut buf, b"bgx.fillStyle='#fff';bgx.fillRect(0,0,W,H);");
    if gridlines {
        push_b(&mut buf, b"bgx.strokeStyle='#e2e8f0';bgx.lineWidth=0.5;for(var i=1;i<=4;i++){var gy=pT+Math.round((1-i/4)*pH);bgx.beginPath();bgx.moveTo(pL,gy);bgx.lineTo(pL+pW,gy);bgx.stroke();}");
    }
    push_b(&mut buf, b"bgx.strokeStyle='#cbd5e1';bgx.lineWidth=1;bgx.beginPath();bgx.moveTo(pL,pT);bgx.lineTo(pL,pT+pH);bgx.lineTo(pL+pW,pT+pH);bgx.stroke();");
    push_b(&mut buf, b"bgx.fillStyle='#6b7280';bgx.font='9px Arial';bgx.textAlign='end';");
    push_b(&mut buf, b"for(var i=0;i<=4;i++){var f=i/4,yp=pT+Math.round((1-f)*pH),yv=minV+f*(maxV-minV);bgx.fillText(yv>=1000?Math.round(yv)+'':yv.toFixed(2),pL-4,yp+3);}");
    if !title.is_empty() {
        push_b(&mut buf, b"bgx.font='700 14px -apple-system,Arial,sans-serif';bgx.fillStyle='#1a202c';bgx.textAlign='center';bgx.fillText('");
        push_js_str(&mut buf, title);
        push_b(&mut buf, b"',W/2,22);");
    }
    push_b(&mut buf, b"bgx.fillStyle='#6b7280';bgx.font='10px Arial';bgx.textAlign='left';bgx.fillText('cap=");
    buf.extend_from_slice(capacity.to_string().as_bytes());
    push_b(&mut buf, b"',pL+4,pT+pH-8);");
    push_b(&mut buf, b"var gl=document.getElementById('");
    buf.extend_from_slice(gl_id.as_bytes());
    push_b(&mut buf, b"').getContext('webgl2',{antialias:false,alpha:true,premultipliedAlpha:false});");
    push_b(&mut buf, b"var live=document.getElementById('");
    buf.extend_from_slice(live_id.as_bytes());
    push_b(&mut buf, b"');");
    push_b(&mut buf, b"var CAP=");
    buf.extend_from_slice(capacity.to_string().as_bytes());
    push_b(&mut buf, b";var VALS=new Float32Array(CAP);var CNT=0,CUR=0,DIRTY=0;");
    push_b(&mut buf, b"if(!gl){live.textContent='WebGL2 unavailable';return;}");
    push_b(&mut buf, b"var vsSrc='#version 300 es\\nin float aY;uniform float uCap;void main(){float i=float(gl_VertexID);float x=(i/max(uCap-1.0,1.0))*2.0-1.0;float y=aY*2.0-1.0;gl_Position=vec4(x,y,0.0,1.0);gl_PointSize=2.0;}';");
    push_b(&mut buf, b"var fsSrc='#version 300 es\\nprecision mediump float;uniform vec4 uColor;out vec4 o;void main(){o=uColor;}';");
    push_b(&mut buf, b"function sh(type,src){var s=gl.createShader(type);gl.shaderSource(s,src);gl.compileShader(s);return s;}");
    push_b(&mut buf, b"var prog=gl.createProgram();gl.attachShader(prog,sh(gl.VERTEX_SHADER,vsSrc));gl.attachShader(prog,sh(gl.FRAGMENT_SHADER,fsSrc));gl.linkProgram(prog);gl.useProgram(prog);");
    push_b(&mut buf, b"var vbo=gl.createBuffer();gl.bindBuffer(gl.ARRAY_BUFFER,vbo);gl.bufferData(gl.ARRAY_BUFFER,VALS,gl.DYNAMIC_DRAW);");
    push_b(&mut buf, b"gl.enableVertexAttribArray(0);gl.vertexAttribPointer(0,1,gl.FLOAT,false,0,0);");
    push_b(&mut buf, b"var uCap=gl.getUniformLocation(prog,'uCap');gl.uniform1f(uCap,CAP);");
    push_b(&mut buf, b"var uColor=gl.getUniformLocation(prog,'uColor');gl.uniform4f(uColor,");
    push_f2(&mut buf, hex[0] as f64 / 255.0);
    push_b(&mut buf, b",0.42,0.86,1.0);");
    push_b(&mut buf, b"function frame(){");
    push_b(&mut buf, b"if(DIRTY){gl.bindBuffer(gl.ARRAY_BUFFER,vbo);gl.bufferData(gl.ARRAY_BUFFER,VALS,gl.DYNAMIC_DRAW);DIRTY=0;live.textContent='n='+CNT;}");
    push_b(&mut buf, b"gl.viewport(pL,H-pT-pH,pW,pH);gl.clear(gl.COLOR_BUFFER_BIT);gl.useProgram(prog);gl.drawArrays(gl.LINE_STRIP,0,CAP);");
    push_b(&mut buf, b"requestAnimationFrame(frame);}");
    push_b(&mut buf, b"gl.clearColor(0,0,0,0);requestAnimationFrame(frame);");
    push_b(&mut buf, b"window['sp_apply_");
    buf.extend_from_slice(hid.to_string().as_bytes());
    push_b(&mut buf, b"']=function(idx,pv){for(var i=0;i<idx.length;i++){var s=idx[i]%CAP;VALS[s]=pv[i]/AX;CUR=s;}CNT+=idx.length;DIRTY=1;};");
    push_b(&mut buf, b"window['sp_push_");
    buf.extend_from_slice(hid.to_string().as_bytes());
    push_b(&mut buf, b"']=function(idxB64,ptsB64){var ib=atob(idxB64),ilen=ib.length,idx=new Uint32Array(ilen/4);for(var i=0;i<ilen;i+=4)idx[i/4]=ib.charCodeAt(i)|(ib.charCodeAt(i+1)<<8)|(ib.charCodeAt(i+2)<<16)|(ib.charCodeAt(i+3)<<24);");
    push_b(&mut buf, b"var pb=atob(ptsB64),plen=pb.length,pv=new Int16Array(plen/2);for(var i=0;i<plen;i+=2)pv[i/2]=pb.charCodeAt(i)|(pb.charCodeAt(i+1)<<8);");
    push_b(&mut buf, b"window['sp_apply_");
    buf.extend_from_slice(hid.to_string().as_bytes());
    push_b(&mut buf, b"'](idx,pv);};");
    push_b(&mut buf, b"})();</script>");
    html_suffix(&mut buf, hid, "[]");
    (unsafe { String::from_utf8_unchecked(buf) }, hid, plot_h)
}

#[crate::sera_alias("firehose", "firehose_chart", "firehose_stream")]
#[crate::sera_builder]
pub fn build_firehose_chart(input: &str) -> String {
    let (title_s, _a, o) = parse_all(input);
    let title = title_s.as_str();
    let capacity = o.capacity.unwrap_or(2_000).clamp(2, 200_000);
    let min_val = o.min_val.unwrap_or(0.0);
    let max_val = o.max_val.unwrap_or(100.0).max(min_val + 1e-9);
    let color_hex = o.color_hex.unwrap_or(0);
    let (html, hid, plot_h) = render_firehose_canvas_html_id(
        title,
        capacity,
        min_val,
        max_val,
        o.w(900),
        o.h(300),
        color_hex,
        o.grid(),
    );
    #[cfg(feature = "sera-pulse")]
    {
        let range_val = (max_val - min_val).max(1e-12);
        let meta = crate::plot::push_registry::PushMeta::Scalar(crate::plot::push_registry::ScalarMeta {
            min_val,
            range_val,
            axis_px: plot_h,
        });
        crate::plot::push_registry::register(hid, meta);
        crate::plot::firehose_registry::register_capacity(hid, capacity);
        crate::plot::chart_source_registry::register(hid, "firehose", input.to_string());
    }
    #[cfg(not(feature = "sera-pulse"))]
    let _ = hid;
    apply(html, &o)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embeds_the_hid_marker_extractable_by_the_push_registry() {
        let (html, hid, _) = render_firehose_canvas_html_id("t", 100, 0.0, 10.0, 900, 300, 0, true);
        assert!(html.contains(&format!("id=\"spp{hid}\"")));
    }

    #[test]
    fn embeds_matching_sp_apply_and_sp_push_functions_for_the_hid() {
        let (html, hid, _) = render_firehose_canvas_html_id("t", 100, 0.0, 10.0, 900, 300, 0, true);
        assert!(html.contains(&format!("sp_apply_{hid}")));
        assert!(html.contains(&format!("sp_push_{hid}")));
    }

    #[test]
    fn embeds_a_webgl2_context_request_not_a_2d_only_canvas() {
        let (html, _, _) = render_firehose_canvas_html_id("t", 100, 0.0, 10.0, 900, 300, 0, true);
        assert!(html.contains("webgl2"));
    }

    #[test]
    fn plot_height_matches_the_axis_pixel_span_used_for_quantization() {
        let (_, _, plot_h) = render_firehose_canvas_html_id("t", 100, 0.0, 10.0, 900, 300, 0, true);
        assert_eq!(plot_h, 300 - 36 - 48);
    }

    #[test]
    fn different_hids_do_not_collide_in_their_generated_function_names() {
        let (h1, hid1, _) = render_firehose_canvas_html_id("a", 10, 0.0, 1.0, 400, 200, 0, false);
        let (h2, hid2, _) = render_firehose_canvas_html_id("b", 10, 0.0, 1.0, 400, 200, 0, false);
        assert_ne!(hid1, hid2);
        assert!(!h1.contains(&format!("sp_apply_{hid2}")));
        assert!(!h2.contains(&format!("sp_apply_{hid1}")));
    }

    #[test]
    fn a_gridlines_flag_of_false_omits_gridline_strokes() {
        let (with_grid, ..) = render_firehose_canvas_html_id("t", 10, 0.0, 1.0, 400, 200, 0, true);
        let (without_grid, ..) = render_firehose_canvas_html_id("t", 10, 0.0, 1.0, 400, 200, 0, false);
        assert!(with_grid.len() > without_grid.len());
    }

    #[test]
    fn builder_produces_real_html_with_a_hid_marker_for_a_minimal_kwarg_json() {
        let html = build_firehose_chart("{\"title\":\"cpu\"}");
        assert!(html.contains("id=\"spp"));
        assert!(html.contains("cpu"));
    }
}
