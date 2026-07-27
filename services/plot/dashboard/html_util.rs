pub(super) fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub(super) fn clip_path_css(clip: &str) -> String {
    match clip {
        "circle" => "circle(50%)".into(),
        "diamond" => "polygon(50% 0%, 100% 50%, 50% 100%, 0% 50%)".into(),
        "hex" | "hexagon" => "polygon(25% 0%, 75% 0%, 100% 50%, 75% 100%, 25% 100%, 0% 50%)".into(),
        "tri" | "triangle" => "polygon(50% 0%, 100% 100%, 0% 100%)".into(),
        "pent" | "pentagon" => "polygon(50% 0%, 100% 38%, 82% 100%, 18% 100%, 0% 38%)".into(),
        _ => String::new(),
    }
}

pub(super) fn name_attr(name: &str) -> String {
    if name.is_empty() {
        String::new()
    } else {
        format!(" data-sp-name=\"{}\"", escape_html(name))
    }
}

pub(super) fn grp_attr(group: &str) -> String {
    if group.is_empty() {
        String::new()
    } else {
        format!(" data-sp-grp=\"{}\"", escape_html(group))
    }
}

pub(super) fn guess_mime(path: &str) -> &'static str {
    crate::core::dispatch::guess_mime(path, "application/octet-stream")
}
