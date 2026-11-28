#[derive(serde::Serialize)]
pub struct ModelFieldInfo {
    pub name: &'static str,
    pub ty: &'static str,
}

#[derive(serde::Serialize)]
pub struct ModelInfo {
    pub name: &'static str,
    pub category: &'static str,
    pub domain: &'static str,
    pub fields: &'static [ModelFieldInfo],
}

inventory::collect!(ModelInfo);

pub fn all_models() -> Vec<&'static ModelInfo> {
    let mut v: Vec<&'static ModelInfo> = inventory::iter::<ModelInfo>().collect();
    v.sort_by_key(|m| m.name);
    v
}

pub fn models_by_category(category: &str) -> Vec<&'static ModelInfo> {
    let mut v: Vec<&'static ModelInfo> = inventory::iter::<ModelInfo>()
        .filter(|m| m.category == category)
        .collect();
    v.sort_by_key(|m| m.name);
    v
}

pub fn models_by_domain(domain: &str) -> Vec<&'static ModelInfo> {
    let mut v: Vec<&'static ModelInfo> = inventory::iter::<ModelInfo>()
        .filter(|m| m.domain == domain)
        .collect();
    v.sort_by_key(|m| m.name);
    v
}

pub fn model_info(name: &str) -> Option<&'static ModelInfo> {
    inventory::iter::<ModelInfo>().find(|m| m.name == name)
}
