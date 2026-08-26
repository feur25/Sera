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

fn build_index() -> std::collections::HashMap<&'static str, &'static FnEntry> {
    let mut map: std::collections::HashMap<&'static str, &'static FnEntry> =
        iter_entries().map(|e| (e.name, e)).collect();
    for doc in crate::doc_registry::all_docs() {
        if let Some(entry) = map.get(doc.name).copied() {
            for alias in doc.aliases {
                map.entry(alias).or_insert(entry);
            }
        }
    }
    map
}

pub fn find(name: &str) -> Option<&'static FnEntry> {
    let idx = INDEX.get_or_init(build_index);
    if let Some(entry) = idx.get(name) {
        return Some(*entry);
    }
    let snake = crate::bindings::name_norm::to_snake_case(name);
    if snake != name {
        idx.get(snake.as_str()).copied()
    } else {
        None
    }
}

#[cfg(test)]
mod alias_dispatch_tests {
    use super::find;

    #[test]
    fn every_declared_fn_alias_resolves_to_its_canonical_entry() {
        let mut checked = 0;
        for doc in crate::doc_registry::all_docs() {
            if doc.aliases.is_empty() {
                continue;
            }
            let canonical = match find(doc.name) {
                Some(e) => e,
                None => continue,
            };
            for alias in doc.aliases {
                let via_alias = find(alias).unwrap_or_else(|| {
                    panic!("alias '{alias}' of fn '{}' does not resolve -- declared in #[sera_doc(aliases(...))] but never dispatchable", doc.name)
                });
                assert!(
                    std::ptr::eq(canonical, via_alias),
                    "alias '{alias}' resolved to a different FnEntry than '{}' itself",
                    doc.name
                );
                checked += 1;
            }
        }
        assert!(checked > 0, "no fn_registry aliases were found to check -- doc_registry wiring may be broken");
    }

    #[test]
    fn pascal_case_fn_names_resolve_the_same_entry_as_their_snake_case_form() {
        let snake = find("build_bar").expect("build_bar is a registered fn");
        let pascal = find("BuildBar").expect("'BuildBar' (PascalCase, as a C# caller would naturally write it) must resolve");
        assert!(std::ptr::eq(snake, pascal));
    }
}