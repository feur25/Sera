pub const BOOTSTRAP_JS: &str = r#"(function(){
var ws = new WebSocket("ws://" + location.host + "/ws");
window.__sp_ws = ws;
window.spSendEvent = function(id, value){
  if (ws.readyState === 1) ws.send(JSON.stringify({type:"event", id: id, value: value}));
};
ws.onmessage = function(ev){
  var msg = JSON.parse(ev.data);
  if (msg.type === "update") {
    var el = document.getElementById(msg.id);
    if (el) el.innerHTML = msg.html;
  }
};
})();"#;

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('"', "&quot;").replace('<', "&lt;")
}

pub fn dropdown_html(id: &str, options: &[String], selected: &str) -> String {
    let opts: String = options
        .iter()
        .map(|o| {
            let sel = if o == selected { " selected" } else { "" };
            format!("<option value=\"{}\"{}>{}</option>", esc(o), sel, esc(o))
        })
        .collect();
    format!(
        "<select id=\"{}\" onchange=\"spSendEvent('{}', this.value)\">{}</select>",
        esc(id),
        esc(id),
        opts
    )
}

pub fn slider_html(id: &str, min: f64, max: f64, step: f64, value: f64) -> String {
    format!(
        "<input type=\"range\" id=\"{}\" min=\"{}\" max=\"{}\" step=\"{}\" value=\"{}\" oninput=\"spSendEvent('{}', this.value)\">",
        esc(id),
        min,
        max,
        step,
        value,
        esc(id)
    )
}

pub fn button_html(id: &str, label: &str) -> String {
    format!(
        "<button id=\"{}\" onclick=\"spSendEvent('{}', 'click')\">{}</button>",
        esc(id),
        esc(id),
        esc(label)
    )
}

pub fn text_input_html(id: &str, value: &str, placeholder: &str) -> String {
    format!(
        "<input type=\"text\" id=\"{}\" value=\"{}\" placeholder=\"{}\" oninput=\"spSendEvent('{}', this.value)\">",
        esc(id),
        esc(value),
        esc(placeholder),
        esc(id)
    )
}

pub fn checkbox_html(id: &str, label: &str, checked: bool) -> String {
    let attr = if checked { " checked" } else { "" };
    format!(
        "<label for=\"{}\"><input type=\"checkbox\" id=\"{}\"{} onchange=\"spSendEvent('{}', this.checked ? 'true' : 'false')\"> {}</label>",
        esc(id),
        esc(id),
        attr,
        esc(id),
        esc(label)
    )
}
