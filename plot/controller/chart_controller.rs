use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::ffi::CStr;
use std::os::raw::c_char;

pub type ChartRenderer = fn(crate::plot::types::PlotRenderContext);

struct ChartRegistry {
    entries: HashMap<u8, (String, ChartRenderer)>,
}

impl ChartRegistry {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    fn register(&mut self, id: u8, name: String, renderer: ChartRenderer) {
        self.entries.insert(id, (name, renderer));
    }

    fn get(&self, id: u8) -> Option<(String, ChartRenderer)> {
        self.entries.get(&id).map(|(n, r)| (n.clone(), *r))
    }

    fn list(&self) -> Vec<(u8, String)> {
        self.entries.iter()
            .map(|(&id, (name, _))| (id, name.clone()))
            .collect()
    }
}

pub struct ChartGroupRegistry {
    groups: HashMap<String, Vec<u8>>,
    current: String,
}

impl ChartGroupRegistry {
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

static REGISTRY: OnceLock<Mutex<ChartRegistry>> = OnceLock::new();
static GROUP_REGISTRY: OnceLock<Mutex<ChartGroupRegistry>> = OnceLock::new();

fn get_registry() -> &'static Mutex<ChartRegistry> {
    REGISTRY.get_or_init(|| Mutex::new(ChartRegistry::new()))
}

pub fn get_group_registry() -> &'static Mutex<ChartGroupRegistry> {
    GROUP_REGISTRY.get_or_init(|| Mutex::new(ChartGroupRegistry::new()))
}

pub struct ChartTypeBuilder {
    id: u8,
    name: String,
    renderer: Option<ChartRenderer>,
}

impl ChartTypeBuilder {
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

    pub fn with_renderer(mut self, renderer: ChartRenderer) -> Self {
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

pub struct ChartGroupBuilder {
    types: Vec<(u8, String, ChartRenderer)>,
}

impl ChartGroupBuilder {
    pub fn new() -> Self {
        Self { types: Vec::new() }
    }

    pub fn add(mut self, id: u8, name: String, renderer: ChartRenderer) -> Self {
        self.types.push((id, name, renderer));
        self
    }

    pub fn build(self) -> Vec<(u8, String, ChartRenderer)> {
        self.types
    }
}

#[no_mangle]
pub extern "C" fn sera_register_chart_group(group_name: *const c_char, type_ids: *const u8, count: u32) -> bool {
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
pub extern "C" fn sera_set_current_chart_group(group_name: *const c_char) -> bool {
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
pub extern "C" fn sera_get_current_chart_group() -> *const c_char {
    if let Ok(grp_reg) = get_group_registry().lock() {
        Box::leak(grp_reg.get_current().to_string().into_boxed_str()) as *const str as *const c_char
    } else {
        std::ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn sera_get_chart_group_types(group_name: *const c_char, count: *mut u32) -> *const u8 {
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
pub extern "C" fn sera_list_chart_groups(count: *mut u32) -> *const *const c_char {
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

#[no_mangle]
pub extern "C" fn sera_list_chart_types() -> u32 {
    if let Ok(reg) = get_registry().lock() {
        reg.list().len() as u32
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn sera_get_chart_type(id: u8) -> bool {
    if let Ok(reg) = get_registry().lock() {
        reg.get(id).is_some()
    } else {
        false
    }
}

pub fn render_by_type(id: u8, ctx: crate::plot::types::PlotRenderContext) {
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

pub fn set_current_chart_group(name: &str) -> bool {
    if let Ok(mut grp_reg) = get_group_registry().lock() {
        grp_reg.set_current(name.to_string())
    } else {
        false
    }
}

