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