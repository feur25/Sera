#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputKind {
    Json,
    ChartHtml,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OutputKind {
    Html,
    Json,
    Bool,
}

pub struct FnEntry {
    pub name: &'static str,
    pub input: InputKind,
    pub output: OutputKind,
    pub invoke: fn(&str) -> String,
}

inventory::collect!(FnEntry);

pub fn iter_entries() -> impl Iterator<Item = &'static FnEntry> {
    inventory::iter::<FnEntry>()
}

static INDEX: std::sync::OnceLock<std::collections::HashMap<&'static str, &'static FnEntry>> =
    std::sync::OnceLock::new();

pub fn find(name: &str) -> Option<&'static FnEntry> {
    INDEX
        .get_or_init(|| iter_entries().map(|e| (e.name, e)).collect())
        .get(name)
        .copied()
}