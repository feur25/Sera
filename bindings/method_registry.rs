pub struct MethodEntry {
    pub name: &'static str,
    pub apply: fn(&crate::Chart, &str) -> crate::Chart,
}

inventory::collect!(MethodEntry);

pub fn iter_entries() -> impl Iterator<Item = &'static MethodEntry> {
    inventory::iter::<MethodEntry>()
}

static INDEX: std::sync::OnceLock<std::collections::HashMap<&'static str, &'static MethodEntry>> =
    std::sync::OnceLock::new();

pub fn find(name: &str) -> Option<&'static MethodEntry> {
    INDEX
        .get_or_init(|| iter_entries().map(|e| (e.name, e)).collect())
        .get(name)
        .copied()
}

pub fn apply_by_name(html: &str, name: &str, args_json: &str) -> Option<String> {
    let entry = find(name)?;
    let chart = crate::Chart {
        html: html.to_string(),
        doc_str: "",
    };
    Some((entry.apply)(&chart, args_json).html)
}