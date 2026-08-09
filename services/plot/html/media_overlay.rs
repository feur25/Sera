use super::hover::{extract_svg_dims, html_id, inject_before_svg_close};
use crate::plot::statistical::common::escape_xml_s;

const VIDEO_MIME_TABLE: &[(&str, &str)] = &[
    (".mp4", "video/mp4"),
    (".webm", "video/webm"),
    (".ogv", "video/ogg"),
    (".ogg", "video/ogg"),
    (".mov", "video/quicktime"),
];

fn guess_video_mime(path: &str) -> &'static str {
    let lower = path.to_lowercase();
    VIDEO_MIME_TABLE
        .iter()
        .find(|(ext, _)| lower.ends_with(ext))
        .map(|(_, mime)| *mime)
        .unwrap_or("video/mp4")
}

fn resolve_src(kind: &str, src: &str) -> String {
    if src.starts_with("data:") || src.starts_with("http://") || src.starts_with("https://") {
        return src.to_string();
    }
    let Ok(bytes) = std::fs::read(src) else {
        return src.to_string();
    };
    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    let mime = if kind == "video" {
        guess_video_mime(src)
    } else {
        crate::core::dispatch::guess_mime(src, "image/png")
    };
    format!("data:{mime};base64,{b64}")
}

pub(crate) fn add_media(
    html: &str,
    kind: &str,
    src: &str,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    shape: &str,
    opacity: f64,
) -> String {
    let (sw, sh) = extract_svg_dims(html);
    let px = x.clamp(0.0, 1.0) * sw as f64;
    let py = y.clamp(0.0, 1.0) * sh as f64;
    let pw = width.clamp(0.0, 1.0) * sw as f64;
    let ph = height.clamp(0.0, 1.0) * sh as f64;
    let op = opacity.clamp(0.0, 1.0);
    let circle = shape.eq_ignore_ascii_case("circle");
    let clip_id = format!("sp-media-clip-{}", html_id());

    let mut frag = String::with_capacity(256);
    if circle {
        frag.push_str(&format!(
            "<clipPath id=\"{clip_id}\"><ellipse cx=\"{:.2}\" cy=\"{:.2}\" rx=\"{:.2}\" ry=\"{:.2}\"/></clipPath>",
            px + pw / 2.0,
            py + ph / 2.0,
            pw / 2.0,
            ph / 2.0
        ));
    }
    let clip_attr = if circle {
        format!(" clip-path=\"url(#{clip_id})\"")
    } else {
        String::new()
    };

    match kind {
        "video" => {
            let resolved = escape_xml_s(&resolve_src("video", src));
            let radius = if circle { "border-radius:50%;" } else { "" };
            frag.push_str(&format!(
                "<foreignObject x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" opacity=\"{op:.3}\"{clip_attr}><video xmlns=\"http://www.w3.org/1999/xhtml\" src=\"{resolved}\" width=\"100%\" height=\"100%\" style=\"object-fit:cover;{radius}\" autoplay loop muted playsinline></video></foreignObject>",
                px, py, pw, ph
            ));
        }
        "text" => {
            let escaped = escape_xml_s(src);
            let font_size = (ph * 0.5).max(8.0);
            frag.push_str(&format!(
                "<g{clip_attr}><text x=\"{:.2}\" y=\"{:.2}\" text-anchor=\"middle\" dominant-baseline=\"middle\" font-family=\"-apple-system,Arial,sans-serif\" font-size=\"{font_size:.1}\" fill=\"#1e293b\" opacity=\"{op:.3}\">{escaped}</text></g>",
                px + pw / 2.0,
                py + ph / 2.0
            ));
        }
        _ => {
            let resolved = escape_xml_s(&resolve_src("image", src));
            frag.push_str(&format!(
                "<image x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" opacity=\"{op:.3}\" preserveAspectRatio=\"xMidYMid slice\" href=\"{resolved}\"{clip_attr}/>",
                px, py, pw, ph
            ));
        }
    }
    frag.push_str("</svg>");
    inject_before_svg_close(html, &frag)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_html() -> String {
        "<html><body><svg id=\"s1\" width=\"900\" height=\"480\"><rect/></svg></body></html>".to_string()
    }

    #[test]
    fn image_element_is_injected_with_pixel_coords_from_fractional_input() {
        let out = add_media(&sample_html(), "image", "https://example.com/pic.png", 0.5, 0.25, 0.2, 0.1, "rect", 1.0);
        assert!(out.contains("<image "));
        assert!(out.contains("x=\"450.00\""));
        assert!(out.contains("y=\"120.00\""));
        assert!(out.contains("width=\"180.00\""));
        assert!(out.contains("height=\"48.00\""));
        assert!(out.contains("href=\"https://example.com/pic.png\""));
    }

    #[test]
    fn video_kind_produces_a_foreign_object_with_a_video_tag() {
        let out = add_media(&sample_html(), "video", "https://example.com/clip.mp4", 0.0, 0.0, 0.3, 0.3, "rect", 1.0);
        assert!(out.contains("<foreignObject"));
        assert!(out.contains("<video "));
        assert!(out.contains("src=\"https://example.com/clip.mp4\""));
    }

    #[test]
    fn text_kind_escapes_and_centers_the_string() {
        let out = add_media(&sample_html(), "text", "<beta> & co", 0.5, 0.5, 0.2, 0.1, "rect", 0.9);
        assert!(out.contains("&lt;beta&gt; &amp; co"));
        assert!(out.contains("text-anchor=\"middle\""));
    }

    #[test]
    fn circle_shape_wraps_with_a_clip_path_ellipse() {
        let out = add_media(&sample_html(), "image", "https://example.com/pic.png", 0.5, 0.5, 0.2, 0.2, "circle", 1.0);
        assert!(out.contains("<clipPath"));
        assert!(out.contains("<ellipse"));
        assert!(out.contains("clip-path=\"url(#sp-media-clip-"));
    }

    #[test]
    fn data_uri_and_http_sources_pass_through_unresolved() {
        let out_data = add_media(&sample_html(), "image", "data:image/png;base64,AAAA", 0.1, 0.1, 0.1, 0.1, "rect", 1.0);
        assert!(out_data.contains("href=\"data:image/png;base64,AAAA\""));
        let out_http = add_media(&sample_html(), "image", "http://x.test/a.png", 0.1, 0.1, 0.1, 0.1, "rect", 1.0);
        assert!(out_http.contains("href=\"http://x.test/a.png\""));
    }

    #[test]
    fn nonexistent_local_file_falls_back_to_literal_src_instead_of_panicking() {
        let out = add_media(&sample_html(), "image", "no/such/file.png", 0.1, 0.1, 0.1, 0.1, "rect", 1.0);
        assert!(out.contains("href=\"no/such/file.png\""));
    }

    #[test]
    fn opacity_and_fraction_inputs_are_clamped_into_range() {
        let out = add_media(&sample_html(), "image", "http://x.test/a.png", 1.5, -0.5, 2.0, 2.0, "rect", 3.0);
        assert!(out.contains("x=\"900.00\""));
        assert!(out.contains("y=\"0.00\""));
        assert!(out.contains("width=\"900.00\""));
        assert!(out.contains("height=\"480.00\""));
        assert!(out.contains("opacity=\"1.000\""));
    }
}
