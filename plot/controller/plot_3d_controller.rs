use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::ffi::CStr;
use std::os::raw::c_char;

pub struct Plot3DRenderContext<'a> {
    pub painter: &'a egui::Painter,
    pub plot_rect: egui::Rect,
    pub colors: &'a [egui::Color32],
    pub hovered_idx: Option<usize>,
    pub values: &'a [f64],
    pub max_val: f64,
    pub visible_indices: &'a [usize],
    pub camera_controller: &'a crate::plot::containers_3d::CameraController,
}

pub type Plot3DRenderer = fn(Plot3DRenderContext);

struct Plot3DRegistry {
    entries: HashMap<u8, (String, Plot3DRenderer)>,
}

impl Plot3DRegistry {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    fn register(&mut self, id: u8, name: String, renderer: Plot3DRenderer) {
        self.entries.insert(id, (name, renderer));
    }

    fn get(&self, id: u8) -> Option<(String, Plot3DRenderer)> {
        self.entries.get(&id).map(|(n, r)| (n.clone(), *r))
    }

    fn list(&self) -> Vec<(u8, String)> {
        self.entries.iter()
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

    fn get_current(&self) -> &str {
        &self.current
    }

    fn list_groups(&self) -> Vec<String> {
        self.groups.keys().cloned().collect()
    }
}

static REGISTRY: OnceLock<Mutex<Plot3DRegistry>> = OnceLock::new();
static GROUP_REGISTRY: OnceLock<Mutex<Plot3DGroupRegistry>> = OnceLock::new();

fn get_registry() -> &'static Mutex<Plot3DRegistry> {
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
pub extern "C" fn sera_register_plot_3d_type(id: u8, name: *const c_char, renderer_id: u32) -> bool {
    if name.is_null() {
        return false;
    }

    let name_str = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };

    let renderer = match get_renderer(renderer_id) {
        Some(r) => r,
        None => return false,
    };

    Plot3DTypeBuilder::new(id)
        .with_name(&name_str)
        .with_renderer(renderer)
        .build()
        .is_ok()
}

#[no_mangle]
pub extern "C" fn sera_register_plot_3d_group(group_name: *const c_char, type_ids: *const u8, count: u32) -> bool {
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
pub extern "C" fn sera_get_plot_3d_group_types(group_name: *const c_char, count: *mut u32) -> *const u8 {
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

pub fn get_renderer(id: u32) -> Option<Plot3DRenderer> {
    crate::plot::default::_3d::plot_3d_types::get_renderers()
        .iter()
        .find(|(rid, _)| *rid == id)
        .map(|(_, renderer)| *renderer)
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
                return type_ids.into_iter()
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
