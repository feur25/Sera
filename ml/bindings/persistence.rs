use super::helpers::*;

#[crate::sera_doc(category = "Registry", en = "Save model to the in-memory registry with name, version, and metadata.", fr = "Sauvegarder le modèle dans le registre en mémoire avec nom, version et métadonnées.", file = "registry.md")]
#[crate::sera_alias("save_model")]
pub fn ml_save_model(input: &str) -> String {
    let v: serde_json::Value = serde_json::from_str(input).unwrap_or(serde_json::Value::Null);
    let name = js(&v, "name", "model");
    let kind = js(&v, "kind", "unknown");
    let payload = v.get("payload").and_then(|x| x.as_str()).unwrap_or("{}");
    let params: std::collections::HashMap<String, String> = v.get("params")
        .and_then(|x| serde_json::from_value(x.clone()).ok())
        .unwrap_or_default();
    let metrics: std::collections::HashMap<String, f64> = v.get("metrics")
        .and_then(|x| serde_json::from_value(x.clone()).ok())
        .unwrap_or_default();
    let tags: Vec<String> = v.get("tags")
        .and_then(|x| serde_json::from_value(x.clone()).ok())
        .unwrap_or_default();
    let record = crate::ml::registry::register(name, kind, payload, params, metrics, tags);
    format!(r#"{{"name":"{}","version":{}}}"#, record.name, record.version)
}

#[crate::sera_alias("load_model")]
pub fn ml_load_model(input: &str) -> String {
    let v: serde_json::Value = serde_json::from_str(input).unwrap_or(serde_json::Value::Null);
    let name = js(&v, "name", "");
    let version: Option<u32> = v.get("version").and_then(|x| x.as_u64()).map(|x| x as u32);
    match crate::ml::registry::get(name, version) {
        Some(r) => format!(r#"{{"found":true,"name":"{}","version":{},"kind":"{}","payload":{}}}"#,
            r.name, r.version, r.kind,
            serde_json::to_string(&r.payload).unwrap_or_default()),
        None => r#"{"found":false}"#.to_string(),
    }
}
