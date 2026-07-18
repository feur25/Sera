use std::collections::HashMap;
use std::sync::Mutex;

static CUSTOM_ALIASES: Mutex<Option<HashMap<String, Vec<String>>>> = Mutex::new(None);

fn with_registry<F, R>(f: F) -> R
where
    F: FnOnce(&mut HashMap<String, Vec<String>>) -> R,
{
    let mut guard = CUSTOM_ALIASES.lock().unwrap_or_else(|e| e.into_inner());
    let map = guard.get_or_insert_with(HashMap::new);
    f(map)
}

pub fn add_alias(method: &str, alias: &str) -> bool {
    with_registry(|m| {
        let list = m.entry(method.to_string()).or_default();
        if list.iter().any(|a| a == alias) {
            false
        } else {
            list.push(alias.to_string());
            true
        }
    })
}

pub fn remove_alias(method: &str, alias: &str) -> bool {
    with_registry(|m| match m.get_mut(method) {
        Some(list) => {
            let before = list.len();
            list.retain(|a| a != alias);
            list.len() != before
        }
        None => false,
    })
}

pub fn reset() {
    with_registry(|m| m.clear());
}

pub fn aliases_map() -> HashMap<String, Vec<String>> {
    with_registry(|m| m.clone())
}

pub fn aliases_json() -> String {
    serde_json::to_string(&aliases_map()).unwrap_or_else(|_| "{}".to_string())
}

pub fn add_aliases_from_json(json: &str) -> bool {
    match serde_json::from_str::<HashMap<String, Vec<String>>>(json) {
        Ok(map) => {
            with_registry(|m| {
                for (method, list) in map {
                    let entry = m.entry(method).or_default();
                    for alias in list {
                        if !entry.iter().any(|a| a == &alias) {
                            entry.push(alias);
                        }
                    }
                }
            });
            true
        }
        Err(_) => false,
    }
}

pub fn resolve(name: &str) -> Option<String> {
    with_registry(|m| {
        m.iter()
            .find(|(_, aliases)| aliases.iter().any(|a| a == name))
            .map(|(method, _)| method.clone())
    })
}

pub fn default_config_path() -> std::path::PathBuf {
    let home = std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .unwrap_or_else(|_| ".".to_string());
    std::path::Path::new(&home).join(".seraplot").join("config.json")
}

pub fn save_to_disk(path: Option<&str>) -> Result<String, String> {
    let p = path
        .map(std::path::PathBuf::from)
        .unwrap_or_else(default_config_path);
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(&p, aliases_json()).map_err(|e| e.to_string())?;
    Ok(p.to_string_lossy().to_string())
}

pub fn load_from_disk(path: Option<&str>) -> Result<bool, String> {
    let p = path
        .map(std::path::PathBuf::from)
        .unwrap_or_else(default_config_path);
    match std::fs::read_to_string(&p) {
        Ok(content) => Ok(add_aliases_from_json(&content)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e.to_string()),
    }
}