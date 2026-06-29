#[derive(serde::Serialize)]
pub struct ParamDoc {
    pub name: &'static str,
    pub ty: &'static str,
    pub en: &'static str,
    pub fr: &'static str,
}

#[derive(serde::Serialize)]
pub struct FnDoc {
    pub name: &'static str,
    pub category: &'static str,
    pub file: &'static str,
    pub en: &'static str,
    pub fr: &'static str,
    pub params: &'static [ParamDoc],
    pub aliases: &'static [&'static str],
}

inventory::collect!(FnDoc);

pub fn all_docs() -> Vec<&'static FnDoc> {
    let mut v: Vec<&'static FnDoc> = inventory::iter::<FnDoc>().collect();
    v.sort_by_key(|d| d.name);
    v
}

pub fn doc_for(name: &str) -> Option<&'static FnDoc> {
    inventory::iter::<FnDoc>().find(|d| d.name == name)
}
