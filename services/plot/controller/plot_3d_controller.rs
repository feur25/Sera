use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::{Mutex, OnceLock};

pub struct Plot3DRenderContext<'a> {
    pub painter: &'a egui::Painter,
    pub plot_rect: egui::Rect,
    pub colors: &'a [egui::Color32],
    pub hovered_idx: Option<usize>,
    pub values: &'a [f64],
    pub max_val: f64,
    pub visible_indices: &'a [usize],
    pub camera_controller: &'a crate::plot::containers_3d::CameraController,
    pub labels: &'a [String],
}

pub type Plot3DRenderer = fn(Plot3DRenderContext);
pub type Plot3DPositioner = fn(
    &[f64],
    f64,
    &[usize],
    &crate::plot::containers_3d::CameraController,
    egui::Rect,
) -> Vec<(egui::Pos2, usize)>;

pub fn noop_3d_renderer(_ctx: Plot3DRenderContext) {}

pub fn noop_3d_positioner(
    _values: &[f64],
    _max_val: f64,
    _visible_indices: &[usize],
    _camera_controller: &crate::plot::containers_3d::CameraController,
    _plot_rect: egui::Rect,
) -> Vec<(egui::Pos2, usize)> {
    Vec::new()
}

pub struct Plot3DTypeEntry {
    pub group: &'static str,
    pub id: u8,
    pub name: &'static str,
    pub renderer: Plot3DRenderer,
    pub positioner: Plot3DPositioner,
}

inventory::collect!(Plot3DTypeEntry);

pub fn register_group_from_inventory(group: &'static str) {
    let mut ids: Vec<u8> = inventory::iter::<Plot3DTypeEntry>()
        .filter(|entry| entry.group == group)
        .map(|entry| {
            let _ = Plot3DTypeBuilder::new(entry.id)
                .with_name(entry.name)
                .with_renderer(entry.renderer)
                .build();
            register_positioner_for_type(entry.id, entry.positioner);
            entry.id
        })
        .collect();
    ids.sort_unstable();

    if let Ok(mut grp_reg) = get_group_registry().lock() {
        grp_reg.register_group(group.to_string(), ids);
    }
}

pub(crate) struct Plot3DRegistry {
    entries: HashMap<u8, (String, Plot3DRenderer)>,
    positioners: HashMap<u8, Plot3DPositioner>,
}

impl Plot3DRegistry {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
            positioners: HashMap::new(),
        }
    }

    fn register(&mut self, id: u8, name: String, renderer: Plot3DRenderer) {
        self.entries.insert(id, (name, renderer));
    }

    pub fn register_positioner(&mut self, id: u8, positioner: Plot3DPositioner) {
        self.positioners.insert(id, positioner);
    }

    fn get(&self, id: u8) -> Option<(String, Plot3DRenderer)> {
        self.entries.get(&id).map(|(n, r)| (n.clone(), *r))
    }

    fn get_positioner(&self, id: u8) -> Option<Plot3DPositioner> {
        self.positioners.get(&id).copied()
    }

    fn list(&self) -> Vec<(u8, String)> {
        self.entries
            .iter()
            .map(|(&id, (name, _))| (id, name.clone()))
            .collect()
    }
}

pub struct Plot3DGroupRegistry {
    groups: HashMap<String, Vec<u8>>,
    current: String,
}

impl Plot3DGroupRegistry {
    fn new() -> Self {
        Self {
            groups: HashMap::new(),
            current: String::from("default"),
        }
    }

    pub fn register_group(&mut self, name: String, types: Vec<u8>) {
        self.groups.insert(name, types);
    }

    fn get_group(&self, name: &str) -> Option<Vec<u8>> {
        self.groups.get(name).cloned()
    }

    fn set_current(&mut self, name: String) -> bool {
        if self.groups.contains_key(&name) {
            self.current = name;
            true
        } else {
            false
        }
    }

    pub fn get_current(&self) -> &str {
        &self.current
    }

    fn list_groups(&self) -> Vec<String> {
        self.groups.keys().cloned().collect()
    }
}

static REGISTRY: OnceLock<Mutex<Plot3DRegistry>> = OnceLock::new();
static GROUP_REGISTRY: OnceLock<Mutex<Plot3DGroupRegistry>> = OnceLock::new();

pub(crate) fn get_registry() -> &'static Mutex<Plot3DRegistry> {
    REGISTRY.get_or_init(|| Mutex::new(Plot3DRegistry::new()))
}

pub fn get_group_registry() -> &'static Mutex<Plot3DGroupRegistry> {
    GROUP_REGISTRY.get_or_init(|| Mutex::new(Plot3DGroupRegistry::new()))
}

pub struct Plot3DTypeBuilder {
    id: u8,
    name: String,
    renderer: Option<Plot3DRenderer>,
}

impl Plot3DTypeBuilder {
    pub fn new(id: u8) -> Self {
        Self {
            id,
            name: String::new(),
            renderer: None,
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_renderer(mut self, renderer: Plot3DRenderer) -> Self {
        self.renderer = Some(renderer);
        self
    }

    pub fn build(self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Name required".to_string());
        }

        let renderer = self.renderer.ok_or("Renderer required")?;

        if let Ok(mut reg) = get_registry().lock() {
            reg.register(self.id, self.name.clone(), renderer);
        }

        Ok(())
    }
}

pub struct Plot3DGroupBuilder {
    types: Vec<(u8, String, Plot3DRenderer)>,
}

impl Plot3DGroupBuilder {
    pub fn new() -> Self {
        Self { types: Vec::new() }
    }

    pub fn add(mut self, id: u8, name: String, renderer: Plot3DRenderer) -> Self {
        self.types.push((id, name, renderer));
        self
    }

    pub fn build(self) -> Vec<(u8, String, Plot3DRenderer)> {
        self.types
    }
}

#[no_mangle]
pub extern "C" fn sera_register_plot_3d_type(
    _id: u8,
    _name: *const c_char,
    _renderer_id: u32,
) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn sera_register_plot_3d_group(
    group_name: *const c_char,
    type_ids: *const u8,
    count: u32,
) -> bool {
    if group_name.is_null() || type_ids.is_null() || count == 0 {
        return false;
    }

    let name = unsafe { CStr::from_ptr(group_name).to_string_lossy().into_owned() };
    let types = unsafe { std::slice::from_raw_parts(type_ids, count as usize).to_vec() };

    if let Ok(mut grp_reg) = get_group_registry().lock() {
        grp_reg.register_group(name, types);
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn sera_set_current_plot_3d_group(group_name: *const c_char) -> bool {
    if group_name.is_null() {
        return false;
    }

    let name = unsafe { CStr::from_ptr(group_name).to_string_lossy().into_owned() };

    if let Ok(mut grp_reg) = get_group_registry().lock() {
        grp_reg.set_current(name)
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn sera_get_current_plot_3d_group() -> *const c_char {
    if let Ok(grp_reg) = get_group_registry().lock() {
        Box::leak(grp_reg.get_current().to_string().into_boxed_str()) as *const str as *const c_char
    } else {
        std::ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn sera_get_plot_3d_group_types(
    group_name: *const c_char,
    count: *mut u32,
) -> *const u8 {
    if group_name.is_null() || count.is_null() {
        return std::ptr::null();
    }

    let name = unsafe { CStr::from_ptr(group_name).to_string_lossy().into_owned() };

    if let Ok(grp_reg) = get_group_registry().lock() {
        if let Some(types) = grp_reg.get_group(&name) {
            unsafe { *count = types.len() as u32 };
            Box::leak(types.into_boxed_slice()).as_ptr()
        } else {
            unsafe { *count = 0 };
            std::ptr::null()
        }
    } else {
        unsafe { *count = 0 };
        std::ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn sera_list_plot_3d_groups(count: *mut u32) -> *const *const c_char {
    if count.is_null() {
        return std::ptr::null();
    }

    if let Ok(grp_reg) = get_group_registry().lock() {
        let groups: Vec<String> = grp_reg.list_groups();
        let result: Vec<*const c_char> = groups
            .into_iter()
            .map(|g| Box::leak(g.into_boxed_str()) as *const str as *const c_char)
            .collect();

        unsafe { *count = result.len() as u32 };
        Box::leak(result.into_boxed_slice()).as_ptr()
    } else {
        unsafe { *count = 0 };
        std::ptr::null()
    }
}

pub fn get_3d_positions(
    chart_type: u8,
    values: &[f64],
    max_val: f64,
    visible_indices: &[usize],
    camera_controller: &crate::plot::containers_3d::CameraController,
    plot_rect: egui::Rect,
) -> Vec<(egui::Pos2, usize)> {
    if let Ok(reg) = get_registry().lock() {
        if let Some(positioner) = reg.get_positioner(chart_type) {
            return positioner(
                values,
                max_val,
                visible_indices,
                camera_controller,
                plot_rect,
            );
        }
    }
    Vec::new()
}

pub fn register_positioner_for_type(chart_type: u8, positioner: Plot3DPositioner) {
    if let Ok(mut reg) = get_registry().lock() {
        reg.register_positioner(chart_type, positioner);
    }
}

#[no_mangle]
pub extern "C" fn sera_list_plot_3d_types() -> u32 {
    if let Ok(reg) = get_registry().lock() {
        reg.list().len() as u32
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn sera_get_plot_3d_type(id: u8) -> bool {
    if let Ok(reg) = get_registry().lock() {
        reg.get(id).is_some()
    } else {
        false
    }
}

pub fn render_by_type(id: u8, ctx: Plot3DRenderContext) {
    if let Ok(reg) = get_registry().lock() {
        if let Some((_, renderer)) = reg.get(id) {
            renderer(ctx);
        }
    }
}

pub fn get_current_group_types() -> Vec<(u8, String)> {
    if let Ok(grp_reg) = get_group_registry().lock() {
        let group_name = grp_reg.get_current().to_string();
        if let Some(type_ids) = grp_reg.get_group(&group_name) {
            if let Ok(reg) = get_registry().lock() {
                return type_ids
                    .into_iter()
                    .filter_map(|id| reg.get(id).map(|(name, _)| (id, name)))
                    .collect();
            }
        }
    }
    Vec::new()
}

pub fn set_current_group(name: &str) -> bool {
    if let Ok(mut grp_reg) = get_group_registry().lock() {
        grp_reg.set_current(name.to_string())
    } else {
        false
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::Plot3DTypeEntry;
    use std::collections::HashSet;

    pub fn group_entries(group: &'static str) -> Vec<&'static Plot3DTypeEntry> {
        inventory::iter::<Plot3DTypeEntry>().filter(|e| e.group == group).collect()
    }

    pub fn assert_group_well_formed(group: &'static str) {
        let entries = group_entries(group);
        assert!(!entries.is_empty(), "group '{group}' has no registered entries");

        let ids: Vec<u8> = entries.iter().map(|e| e.id).collect();
        let unique_ids: HashSet<u8> = ids.iter().copied().collect();
        assert_eq!(ids.len(), unique_ids.len(), "duplicate ids in group '{group}': {ids:?}");

        let names: Vec<&str> = entries.iter().map(|e| e.name).collect();
        let unique_names: HashSet<&str> = names.iter().copied().collect();
        assert_eq!(names.len(), unique_names.len(), "duplicate names in group '{group}': {names:?}");

        for entry in &entries {
            assert!(!entry.name.is_empty(), "entry id {} in group '{group}' has an empty name", entry.id);
        }
    }

    pub fn assert_registered_group_matches_inventory(group: &'static str, register_fn: fn()) {
        let inventory_ids: HashSet<u8> = group_entries(group).into_iter().map(|e| e.id).collect();

        register_fn();
        super::set_current_group(group);
        let registered_ids: HashSet<u8> = super::get_current_group_types()
            .into_iter()
            .map(|(id, _)| id)
            .collect();

        assert_eq!(registered_ids, inventory_ids, "registered group '{group}' drifted from its inventory entries");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn all_plot_3d_type_entries_share_a_globally_unique_id_space() {
        let ids: Vec<u8> = inventory::iter::<Plot3DTypeEntry>().map(|e| e.id).collect();
        let unique: HashSet<u8> = ids.iter().copied().collect();
        assert_eq!(
            ids.len(),
            unique.len(),
            "duplicate ids across Plot3DTypeEntry groups (they share one flat u8 registry): {ids:?}"
        );
    }
}
