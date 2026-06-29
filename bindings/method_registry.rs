pub struct MethodEntry {
    pub name: &'static str,
    pub apply: fn(&crate::Chart, &str) -> crate::Chart,
}

inventory::collect!(MethodEntry);

pub fn iter_entries() -> impl Iterator<Item = &'static MethodEntry> {
    inventory::iter::<MethodEntry>()
}

pub fn apply_by_name(html: &str, name: &str, args_json: &str) -> Option<String> {
    let chart = crate::Chart {
        html: html.to_string(),
        doc_str: "",
    };
    for entry in iter_entries() {
        if entry.name == name {
            return Some((entry.apply)(&chart, args_json).html);
        }
    }
    None
}
