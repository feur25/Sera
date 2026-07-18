pub fn fmt_val(v: f64) -> String {
    let a = v.abs();
    if a >= 1000.0     { format!("{:.0}", v) }
    else if a >= 10.0  { format!("{:.1}", v) }
    else if a >= 1.0   { format!("{:.2}", v) }
    else if a >= 0.001 { format!("{:.4}", v) }
    else               { format!("{:.2e}", v) }
}

pub fn mbar(label: &str, norm: f64, col: &str, val_str: &str) -> String {
    let w = (norm * 100.0).clamp(0.0, 100.0) as i32;
    format!(
        "<div class=\"pw-mr\"><div class=\"pw-ml\"><span>{}</span>\
         <span style=\"color:#94a3b8\">{}</span></div>\
         <div class=\"pw-mbg\"><div class=\"pw-mb\" style=\"width:{}%;background:{}\"></div></div></div>",
        label, val_str, w, col
    )
}

pub const BAR_CSS: &str = concat!(
    "<style>",
    ".pw-ap{display:inline-block;padding:2px 8px;border-radius:20px;",
    "font-size:9px;font-weight:700;letter-spacing:.08em;",
    "text-transform:uppercase;margin-bottom:8px}",
    ".pw-mr{margin-bottom:7px}",
    ".pw-ml{font-size:9px;color:#475569;margin-bottom:3px;display:flex;justify-content:space-between}",
    ".pw-mbg{background:#0f172a;border-radius:3px;height:6px;overflow:hidden}",
    ".pw-mb{height:6px;border-radius:3px}",
    ".pw-rt{margin-top:10px;padding:5px 8px;border-radius:6px;font-size:9px;",
    "font-weight:700;text-align:center;background:rgba(255,255,255,0.04);border:1px solid}",
    "</style>"
);
