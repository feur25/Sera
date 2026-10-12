use crate::Chart;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use seraplot_macros::sera_doc_impl;
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

const CAPACITY: usize = 512;
const MAX_PER_CHART: usize = 2048;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Annotation {
    pub id: u64,
    pub index: u32,
    pub label: String,
    pub at_ms: Option<i64>,
    pub created_at: i64,
}

struct ChartAnnotations {
    next_id: u64,
    items: Vec<Annotation>,
    persist_path: Option<String>,
}

struct Registry {
    map: HashMap<u64, ChartAnnotations>,
    order: VecDeque<u64>,
}

static REGISTRY: Mutex<Option<Registry>> = Mutex::new(None);

fn persist(chart: &ChartAnnotations) {
    let Some(path) = &chart.persist_path else { return };
    let Ok(json) = serde_json::to_string_pretty(&chart.items) else { return };
    let _ = super::atomic_write(path, &json);
}

pub fn add(hid: u64, index: u32, label: String, at_ms: Option<i64>) -> PyResult<u64> {
    let Ok(mut guard) = REGISTRY.lock() else {
        return Err(pyo3::exceptions::PyRuntimeError::new_err("seraplot: annotation registry lock poisoned"));
    };
    let reg = guard.get_or_insert_with(|| Registry { map: HashMap::new(), order: VecDeque::new() });
    if !reg.map.contains_key(&hid) {
        reg.order.push_back(hid);
        while reg.order.len() > CAPACITY {
            if let Some(old) = reg.order.pop_front() {
                reg.map.remove(&old);
            }
        }
    }
    let chart = reg.map.entry(hid).or_insert_with(|| ChartAnnotations { next_id: 1, items: Vec::new(), persist_path: None });
    if chart.items.len() >= MAX_PER_CHART {
        return Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "seraplot: this chart already has {MAX_PER_CHART} bookmarks, the maximum"
        )));
    }
    let id = chart.next_id;
    chart.next_id += 1;
    chart.items.push(Annotation { id, index, label, at_ms, created_at: crate::plot::now_ms() });
    persist(chart);
    Ok(id)
}

pub fn list(hid: u64) -> Vec<Annotation> {
    let Ok(guard) = REGISTRY.lock() else { return Vec::new() };
    guard.as_ref().and_then(|r| r.map.get(&hid)).map(|c| c.items.clone()).unwrap_or_default()
}

pub fn remove(hid: u64, id: u64) -> bool {
    let Ok(mut guard) = REGISTRY.lock() else { return false };
    let Some(reg) = guard.as_mut() else { return false };
    let Some(chart) = reg.map.get_mut(&hid) else { return false };
    let before = chart.items.len();
    chart.items.retain(|a| a.id != id);
    let changed = chart.items.len() != before;
    if changed {
        persist(chart);
    }
    changed
}

pub fn clear(hid: u64) {
    let Ok(mut guard) = REGISTRY.lock() else { return };
    let Some(reg) = guard.as_mut() else { return };
    let has_persist = reg.map.get(&hid).map(|c| c.persist_path.is_some()).unwrap_or(false);
    if has_persist {
        if let Some(chart) = reg.map.get_mut(&hid) {
            chart.items.clear();
        }
        if let Some(chart) = reg.map.get(&hid) {
            persist(chart);
        }
    } else {
        reg.map.remove(&hid);
    }
}

pub fn enable_persistence(hid: u64, path: &str) -> PyResult<()> {
    let loaded: Vec<Annotation> =
        std::fs::read_to_string(path).ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default();
    let Ok(mut guard) = REGISTRY.lock() else {
        return Err(pyo3::exceptions::PyRuntimeError::new_err("seraplot: annotation registry lock poisoned"));
    };
    let reg = guard.get_or_insert_with(|| Registry { map: HashMap::new(), order: VecDeque::new() });
    if !reg.map.contains_key(&hid) {
        reg.order.push_back(hid);
        while reg.order.len() > CAPACITY {
            if let Some(old) = reg.order.pop_front() {
                reg.map.remove(&old);
            }
        }
    }
    let recovered_next_id = loaded.iter().map(|a| a.id).max().unwrap_or(0) + 1;
    let chart = reg.map.entry(hid).or_insert_with(|| ChartAnnotations { next_id: 1, items: Vec::new(), persist_path: None });
    if chart.items.is_empty() && !loaded.is_empty() {
        chart.next_id = chart.next_id.max(recovered_next_id);
        chart.items = loaded;
    }
    chart.persist_path = Some(path.to_string());
    Ok(())
}

pub fn disable_persistence(hid: u64) {
    let Ok(mut guard) = REGISTRY.lock() else { return };
    let Some(reg) = guard.as_mut() else { return };
    if let Some(chart) = reg.map.get_mut(&hid) {
        chart.persist_path = None;
    }
}

#[cfg(feature = "python")]
#[sera_doc_impl]
#[pyo3::pymethods]
impl Chart {
    #[pyo3(signature = (index, label, at_ms = None))]
    #[sera_doc(
        category = "chart_method",
        module = "bookmarks",
        file = "seraplot/pulse-pricing.md",
        en = "Attaches a bookmark note to a specific index in this chart's stream (e.g. 'deploy happened here') — for your own reference, not rendered on the chart yet. Returns the bookmark's id. Note: annotate() is already taken by the free-text speech-bubble callout method (a different, static drawing feature) — use bookmark()/bookmarks() for the streaming version.",
        fr = "Attache une note-repère à un indice précis du flux de ce chart (ex : 'déploiement effectué ici') — pour ta propre référence, pas encore rendue sur le chart. Retourne l'id du repère. Remarque : annotate() désigne déjà la bulle de texte libre dessinée sur le chart (une autre fonctionnalité, statique) — utilise bookmark()/bookmarks() pour la version streaming.",
        param(name = "index", ty = "int", en = "The index this bookmark refers to.", fr = "L'indice auquel ce repère se rapporte."),
        param(name = "label", ty = "str", en = "Free-text note for this bookmark.", fr = "Note libre pour ce repère."),
        param(name = "at_ms", ty = "int | None", en = "Optional epoch-millisecond timestamp this bookmark refers to, if different from creation time.", fr = "Timestamp epoch en millisecondes auquel ce repère se rapporte, si différent de l'heure de création.")
    )]
    fn bookmark(&self, index: u32, label: String, at_ms: Option<i64>) -> PyResult<u64> {
        let hid = super::require_hid(&self.html)?;
        add(hid, index, label, at_ms)
    }

    #[sera_doc(
        category = "chart_method",
        module = "bookmarks",
        file = "seraplot/pulse-pricing.md",
        aliases("annotations"),
        en = "Returns every bookmark attached to this chart, most-recently-created last.",
        fr = "Retourne tous les repères attachés à ce chart, du plus ancien au plus récent."
    )]
    fn bookmarks(&self, py: Python<'_>) -> PyResult<Py<PyList>> {
        let hid = super::require_hid(&self.html)?;
        let out = PyList::empty_bound(py);
        for a in list(hid) {
            let d = PyDict::new_bound(py);
            d.set_item("id", a.id)?;
            d.set_item("index", a.index)?;
            d.set_item("label", a.label)?;
            d.set_item("at_ms", a.at_ms)?;
            d.set_item("created_at", a.created_at)?;
            out.append(d)?;
        }
        Ok(out.unbind())
    }

    #[sera_doc(
        category = "chart_method",
        module = "bookmarks",
        file = "seraplot/pulse-pricing.md",
        aliases("remove_annotation"),
        en = "Removes one bookmark by id. Returns whether a bookmark was actually removed.",
        fr = "Retire un repère par son id. Retourne si un repère a bien été retiré.",
        param(name = "id", ty = "int", en = "The bookmark id returned by bookmark().", fr = "L'id du repère retourné par bookmark().")
    )]
    fn remove_bookmark(&self, id: u64) -> PyResult<bool> {
        let hid = super::require_hid(&self.html)?;
        Ok(remove(hid, id))
    }

    #[sera_doc(
        category = "chart_method",
        module = "bookmarks",
        file = "seraplot/pulse-pricing.md",
        aliases("clear_annotations"),
        en = "Removes every bookmark attached to this chart.",
        fr = "Retire tous les repères attachés à ce chart."
    )]
    fn clear_bookmarks(&self) -> PyResult<()> {
        let hid = super::require_hid(&self.html)?;
        clear(hid);
        Ok(())
    }

    #[sera_doc(
        category = "chart_method",
        module = "bookmarks",
        file = "seraplot/pulse-pricing.md",
        aliases("persist_bookmarks"),
        en = "Saves this chart's bookmarks to a JSON file on every change from now on, and immediately loads any bookmarks already saved there (e.g. from a previous run) if this chart doesn't already have any in memory. Without calling this, bookmarks are memory-only and lost on restart.",
        fr = "Sauvegarde les repères de ce chart dans un fichier JSON à chaque changement à partir de maintenant, et charge immédiatement les repères déjà sauvegardés là (ex : d'une exécution précédente) si ce chart n'en a pas encore en mémoire. Sans appeler ceci, les repères sont en mémoire seulement et perdus au redémarrage.",
        param(name = "path", ty = "str", en = "JSON file path to save bookmarks to and load them from.", fr = "Chemin du fichier JSON où sauvegarder et charger les repères.")
    )]
    fn enable_bookmark_persistence(&self, path: &str) -> PyResult<()> {
        let hid = super::require_hid(&self.html)?;
        enable_persistence(hid, path)
    }

    #[sera_doc(
        category = "chart_method",
        module = "bookmarks",
        file = "seraplot/pulse-pricing.md",
        en = "Stops saving this chart's bookmarks to disk on every change (undoes enable_bookmark_persistence()). Bookmarks already in memory are kept - only the auto-save-on-change behavior is turned off - and the file already on disk is left untouched.",
        fr = "Arrête de sauvegarder les repères de ce chart sur disque à chaque changement (annule enable_bookmark_persistence()). Les repères déjà en mémoire sont conservés — seul le comportement de sauvegarde automatique est désactivé — et le fichier déjà présent sur disque n'est pas modifié."
    )]
    fn disable_bookmark_persistence(&self) -> PyResult<()> {
        let hid = super::require_hid(&self.html)?;
        disable_persistence(hid);
        Ok(())
    }

    fn annotations(&self, py: Python<'_>) -> PyResult<Py<PyList>> {
        self.bookmarks(py)
    }

    fn remove_annotation(&self, id: u64) -> PyResult<bool> {
        self.remove_bookmark(id)
    }

    fn clear_annotations(&self) -> PyResult<()> {
        self.clear_bookmarks()
    }

    fn persist_bookmarks(&self, path: &str) -> PyResult<()> {
        self.enable_bookmark_persistence(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_list_remove_clear_round_trip() {
        let hid = 900_001u64;
        clear(hid);
        let id_a = add(hid, 5, "first".to_string(), None).expect("add a");
        let id_b = add(hid, 9, "second".to_string(), Some(123)).expect("add b");
        assert_ne!(id_a, id_b);

        let items = list(hid);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].label, "first");
        assert_eq!(items[1].at_ms, Some(123));

        assert!(remove(hid, id_a));
        assert!(!remove(hid, id_a));
        assert_eq!(list(hid).len(), 1);

        clear(hid);
        assert_eq!(list(hid).len(), 0);
    }

    #[test]
    fn unknown_chart_returns_empty_not_panic() {
        assert_eq!(list(999_999_999u64).len(), 0);
        assert!(!remove(999_999_999u64, 1));
    }

    #[test]
    fn per_chart_cap_is_enforced() {
        pyo3::prepare_freethreaded_python();
        let hid = 900_002u64;
        clear(hid);
        for i in 0..MAX_PER_CHART {
            add(hid, i as u32, "x".to_string(), None).expect("add within cap");
        }
        let err = add(hid, 0, "overflow".to_string(), None).unwrap_err();
        assert!(Python::with_gil(|py| err.value_bound(py).to_string().contains("maximum")));
        clear(hid);
    }

    #[test]
    fn ids_are_unique_and_increasing_per_chart() {
        let hid = 900_003u64;
        clear(hid);
        let mut last = 0u64;
        for i in 0..50 {
            let id = add(hid, i, "n".to_string(), None).expect("add");
            assert!(id > last);
            last = id;
        }
        clear(hid);
    }

    #[test]
    fn persistence_writes_to_disk_on_every_mutation() {
        let hid = 900_004u64;
        clear(hid);
        let path = std::env::temp_dir().join(format!("sp_bookmarks_test_{}.json", std::process::id()));
        let _ = std::fs::remove_file(&path);

        enable_persistence(hid, path.to_str().unwrap()).expect("enable persistence");
        let id = add(hid, 3, "incident start".to_string(), None).expect("add");

        let on_disk = std::fs::read_to_string(&path).expect("file should exist after add");
        assert!(on_disk.contains("incident start"));

        remove(hid, id);
        let on_disk_after_remove = std::fs::read_to_string(&path).unwrap();
        assert!(!on_disk_after_remove.contains("incident start"));

        let _ = std::fs::remove_file(&path);
        clear(hid);
    }

    #[test]
    fn a_fresh_chart_recovers_bookmarks_saved_by_a_previous_session() {
        let old_hid = 900_005u64;
        let new_hid = 900_006u64;
        clear(old_hid);
        clear(new_hid);
        let path = std::env::temp_dir().join(format!("sp_bookmarks_recover_{}.json", std::process::id()));
        let _ = std::fs::remove_file(&path);

        enable_persistence(old_hid, path.to_str().unwrap()).expect("enable on old chart");
        add(old_hid, 1, "before restart".to_string(), None).expect("add");

        enable_persistence(new_hid, path.to_str().unwrap()).expect("enable on new chart, simulating a restart (the old process just exits here, never calling clear)");
        let recovered = list(new_hid);
        assert_eq!(recovered.len(), 1, "a fresh chart pointed at the same file should recover the saved bookmark");
        assert_eq!(recovered[0].label, "before restart");

        let _ = std::fs::remove_file(&path);
        clear(new_hid);
    }

    #[test]
    fn disabling_persistence_stops_further_writes_but_keeps_the_file_and_memory_intact() {
        let hid = 900_008u64;
        clear(hid);
        let path = std::env::temp_dir().join(format!("sp_bookmarks_disable_{}.json", std::process::id()));
        let _ = std::fs::remove_file(&path);

        enable_persistence(hid, path.to_str().unwrap()).expect("enable persistence");
        add(hid, 1, "first".to_string(), None).expect("add");
        let on_disk_before = std::fs::read_to_string(&path).expect("file exists after add");
        assert!(on_disk_before.contains("first"));

        disable_persistence(hid);
        add(hid, 2, "second, should not reach disk".to_string(), None).expect("add after disabling");
        let on_disk_after = std::fs::read_to_string(&path).expect("file still exists, untouched");
        assert_eq!(on_disk_before, on_disk_after, "disabling persistence should stop further writes, leaving the on-disk file exactly as it was");

        assert_eq!(list(hid).len(), 2, "in-memory bookmarks should be unaffected by disabling persistence");

        let _ = std::fs::remove_file(&path);
        clear(hid);
    }

    #[test]
    fn disabling_persistence_on_a_chart_that_never_enabled_it_is_a_harmless_noop() {
        disable_persistence(999_999_997u64);
    }

    #[test]
    fn enabling_persistence_never_clobbers_bookmarks_already_in_memory() {
        let hid = 900_007u64;
        clear(hid);
        let path = std::env::temp_dir().join(format!("sp_bookmarks_no_clobber_{}.json", std::process::id()));
        std::fs::write(&path, "[]").unwrap();

        add(hid, 1, "already here before persistence was turned on".to_string(), None).expect("add");
        enable_persistence(hid, path.to_str().unwrap()).expect("enable persistence with an empty file on disk");

        assert_eq!(list(hid).len(), 1, "the in-memory bookmark should survive, not get overwritten by the empty file");

        let _ = std::fs::remove_file(&path);
        clear(hid);
    }
}
