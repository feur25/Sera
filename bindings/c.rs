use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_uint};
use std::sync::Mutex;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref SVG_CACHE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref META_CACHE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

fn filter_data(values: &[f64], names: &[String], hidden_ptr: *const c_uint, hidden_count: c_uint) -> (Vec<f64>, Vec<String>) {
    let hidden_set: std::collections::HashSet<usize> = if !hidden_ptr.is_null() && hidden_count > 0 {
        unsafe {
            std::slice::from_raw_parts(hidden_ptr, hidden_count as usize)
                .iter()
                .map(|&idx| idx as usize)
                .collect()
        }
    } else {
        std::collections::HashSet::new()
    };
    
    let mut filtered_vals = Vec::new();
    let mut filtered_names = Vec::new();
    
    for (i, (val, name)) in values.iter().zip(names.iter()).enumerate() {
        if !hidden_set.contains(&i) {
            filtered_vals.push(*val);
            filtered_names.push(name.clone());
        }
    }
    
    (filtered_vals, filtered_names)
}

#[no_mangle]
pub extern "C" fn sera_render_interactive_svg(
    values_ptr: *const c_double,
    names_ptr: *const *const c_char,
    count: c_uint,
    damage_type: *const c_char,
    chart_type: *const c_char,
) -> *mut c_char {
    use crate::render::InteractiveSvgRenderer;
    
    if values_ptr.is_null() || names_ptr.is_null() {
        return CString::new("").unwrap().into_raw();
    }
    
    unsafe {
        let values: Vec<f64> = std::slice::from_raw_parts(values_ptr, count as usize).to_vec();
        let names_slice = std::slice::from_raw_parts(names_ptr, count as usize);
        let names: Vec<String> = names_slice
            .iter()
            .map(|&ptr| CStr::from_ptr(ptr).to_string_lossy().into_owned())
            .collect();
        
        let dmg_type = CStr::from_ptr(damage_type)
            .to_string_lossy()
            .into_owned();
        let chart_t = CStr::from_ptr(chart_type)
            .to_string_lossy()
            .into_owned();
        
        let cache_key = format!("{}_{}", dmg_type, chart_t);
        
        if let Ok(cache) = SVG_CACHE.lock() {
            if let Some(cached) = cache.get(&cache_key) {
                return CString::new(cached.clone()).unwrap().into_raw();
            }
        }
        
        let mut renderer = InteractiveSvgRenderer::new(1000.0, 500.0, &dmg_type);
        
        let colors = vec![
            "#667eea", "#764ba2", "#f093fb", "#4facfe", "#00f2fe",
            "#43e97b", "#fa709a", "#fee140", "#30cfd0", "#330867"
        ];
        
        for (i, (val, name)) in values.iter().zip(names.iter()).enumerate() {
            let color = colors[i % colors.len()].to_string();
            renderer.add_bar(
                format!("item_{}", i),
                name.clone(),
                *val,
                color,
            );
        }
        
        let svg = match chart_t.as_str() {
            "bar" => renderer.render_bar_chart(),
            "scatter" => renderer.render_scatter_chart(),
            "line" => renderer.render_line_chart(),
            _ => renderer.render_bar_chart(),
        };
        
        if let Ok(mut cache) = SVG_CACHE.lock() {
            if cache.len() > 100 {
                cache.clear();
            }
            cache.insert(cache_key, svg.clone());
        }
        
        CString::new(svg).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn sera_render_interactive_svg_filtered(
    values_ptr: *const c_double,
    names_ptr: *const *const c_char,
    count: c_uint,
    damage_type: *const c_char,
    chart_type: *const c_char,
    hidden_indices: *const c_uint,
    hidden_count: c_uint,
) -> *mut c_char {
    use crate::render::InteractiveSvgRenderer;
    
    if values_ptr.is_null() || names_ptr.is_null() {
        return CString::new("").unwrap().into_raw();
    }
    
    unsafe {
        let values: Vec<f64> = std::slice::from_raw_parts(values_ptr, count as usize).to_vec();
        let names_slice = std::slice::from_raw_parts(names_ptr, count as usize);
        let names: Vec<String> = names_slice
            .iter()
            .map(|&ptr| CStr::from_ptr(ptr).to_string_lossy().into_owned())
            .collect();
        
        let (filtered_vals, filtered_names) = filter_data(&values, &names, hidden_indices, hidden_count);
        
        let dmg_type = CStr::from_ptr(damage_type)
            .to_string_lossy()
            .into_owned();
        let chart_t = CStr::from_ptr(chart_type)
            .to_string_lossy()
            .into_owned();
        
        let mut renderer = InteractiveSvgRenderer::new(1000.0, 500.0, &dmg_type);
        
        let colors = vec![
            "#667eea", "#764ba2", "#f093fb", "#4facfe", "#00f2fe",
            "#43e97b", "#fa709a", "#fee140", "#30cfd0", "#330867"
        ];
        
        for (i, (val, name)) in filtered_vals.iter().zip(filtered_names.iter()).enumerate() {
            let color = colors[i % colors.len()].to_string();
            renderer.add_bar(
                format!("item_{}", i),
                name.clone(),
                *val,
                color,
            );
        }
        
        let svg = match chart_t.as_str() {
            "bar" => renderer.render_bar_chart(),
            "scatter" => renderer.render_scatter_chart(),
            "line" => renderer.render_line_chart(),
            _ => renderer.render_bar_chart(),
        };
        
        CString::new(svg).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn sera_render_3d_chart(
    values_ptr: *const c_double,
    names_ptr: *const *const c_char,
    count: c_uint,
    damage_type: *const c_char,
) -> *mut c_char {
    if values_ptr.is_null() || names_ptr.is_null() {
        return CString::new("").unwrap().into_raw();
    }
    
    unsafe {
        let values: Vec<f64> = std::slice::from_raw_parts(values_ptr, count as usize).to_vec();
        let names_slice = std::slice::from_raw_parts(names_ptr, count as usize);
        let names: Vec<String> = names_slice
            .iter()
            .map(|&ptr| CStr::from_ptr(ptr).to_string_lossy().into_owned())
            .collect();
        
        let dmg_type = CStr::from_ptr(damage_type)
            .to_string_lossy()
            .into_owned();
        
        let cache_key = format!("3d_{}", dmg_type);
        
        if let Ok(cache) = SVG_CACHE.lock() {
            if let Some(cached) = cache.get(&cache_key) {
                return CString::new(cached.clone()).unwrap().into_raw();
            }
        }
        
        let html = format!(
            "<svg viewBox='0 0 800 600' xmlns='http://www.w3.org/2000/svg'>\
             <rect width='800' height='600' fill='#f8f9fa'/>\
             <text x='400' y='50' text-anchor='middle' font-size='24' fill='#333'>3D Chart: {}</text>\
             <g transform='translate(400, 300)'>{}</g>\
             </svg>",
            dmg_type,
            names.iter().enumerate()
                .map(|(i, name)| {
                    let angle = (i as f32 / values.len() as f32) * std::f32::consts::TAU;
                    let r = 150.0f32;
                    let x = angle.cos() * r;
                    let y = angle.sin() * r;
                    let size = 10.0f32 + (values[i] as f32 / 100.0).min(50.0);
                    format!(
                        "<circle cx='{}' cy='{}' r='{}' fill='hsl({}, 70%, 50%)' data-idx='{}'/>\
                         <text x='{}' y='{}' font-size='10' fill='#333'>{}</text>",
                        x, y, size, (i * 45) % 360, i, x, y - size - 5.0f32, name
                    )
                })
                .collect::<Vec<_>>()
                .join("")
        );
        
        if let Ok(mut cache) = SVG_CACHE.lock() {
            cache.insert(cache_key, html.clone());
        }
        
        CString::new(html).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn sera_render_3d_chart_filtered(
    values_ptr: *const c_double,
    names_ptr: *const *const c_char,
    count: c_uint,
    damage_type: *const c_char,
    hidden_indices: *const c_uint,
    hidden_count: c_uint,
) -> *mut c_char {
    if values_ptr.is_null() || names_ptr.is_null() {
        return CString::new("").unwrap().into_raw();
    }
    
    unsafe {
        let values: Vec<f64> = std::slice::from_raw_parts(values_ptr, count as usize).to_vec();
        let names_slice = std::slice::from_raw_parts(names_ptr, count as usize);
        let names: Vec<String> = names_slice
            .iter()
            .map(|&ptr| CStr::from_ptr(ptr).to_string_lossy().into_owned())
            .collect();
        
        let (filtered_vals, filtered_names) = filter_data(&values, &names, hidden_indices, hidden_count);
        
        let dmg_type = CStr::from_ptr(damage_type)
            .to_string_lossy()
            .into_owned();
        
        let html = format!(
            "<svg viewBox='0 0 800 600' xmlns='http://www.w3.org/2000/svg'>\
             <rect width='800' height='600' fill='#f8f9fa'/>\
             <text x='400' y='50' text-anchor='middle' font-size='24' fill='#333'>3D Chart: {}</text>\
             <g transform='translate(400, 300)'>{}</g>\
             </svg>",
            dmg_type,
            filtered_names.iter().enumerate()
                .map(|(i, name)| {
                    let angle = (i as f32 / filtered_vals.len() as f32) * std::f32::consts::TAU;
                    let r = 150.0f32;
                    let x = angle.cos() * r;
                    let y = angle.sin() * r;
                    let size = 10.0f32 + (filtered_vals[i] as f32 / 100.0).min(50.0);
                    format!(
                        "<circle cx='{}' cy='{}' r='{}' fill='hsl({}, 70%, 50%)' data-idx='{}'/>\
                         <text x='{}' y='{}' font-size='10' fill='#333'>{}</text>",
                        x, y, size, (i * 45) % 360, i, x, y - size - 5.0f32, name
                    )
                })
                .collect::<Vec<_>>()
                .join("")
        );
        
        CString::new(html).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn sera_cache_clear() {
    if let Ok(mut cache) = SVG_CACHE.lock() {
        cache.clear();
    }
    if let Ok(mut meta) = META_CACHE.lock() {
        meta.clear();
    }
}

#[no_mangle]
pub extern "C" fn sera_render_zoomed_chart(
    values_ptr: *const c_double,
    names_ptr: *const *const c_char,
    count: c_uint,
    damage_type: *const c_char,
    chart_type: *const c_char,
    zoom_level: c_double,
    zoom_x: c_double,
    zoom_y: c_double,
    zoom_width: c_double,
    zoom_height: c_double,
) -> *mut c_char {
    use crate::render::InteractiveSvgRenderer;
    
    if values_ptr.is_null() || names_ptr.is_null() {
        return CString::new("").unwrap().into_raw();
    }
    
    unsafe {
        let values: Vec<f64> = std::slice::from_raw_parts(values_ptr, count as usize).to_vec();
        let names_slice = std::slice::from_raw_parts(names_ptr, count as usize);
        let names: Vec<String> = names_slice
            .iter()
            .map(|&ptr| CStr::from_ptr(ptr).to_string_lossy().into_owned())
            .collect();
        
        let dmg_type = CStr::from_ptr(damage_type)
            .to_string_lossy()
            .into_owned();
        let chart_t = CStr::from_ptr(chart_type)
            .to_string_lossy()
            .into_owned();
        
        let cache_key = format!("zoom_{}_{}_{}_{}", dmg_type, chart_t, zoom_level as i32, zoom_x as i32);
        
        if let Ok(cache) = SVG_CACHE.lock() {
            if let Some(cached) = cache.get(&cache_key) {
                return CString::new(cached.clone()).unwrap().into_raw();
            }
        }
        
        let mut renderer = InteractiveSvgRenderer::new(1000.0, 500.0, &dmg_type);
        renderer.set_zoom(zoom_level as f32, zoom_x as f32, zoom_y as f32, zoom_width as f32, zoom_height as f32);
        
        let colors = vec![
            "#667eea", "#764ba2", "#f093fb", "#4facfe", "#00f2fe",
            "#43e97b", "#fa709a", "#fee140", "#30cfd0", "#330867"
        ];
        
        for (i, (val, name)) in values.iter().zip(names.iter()).enumerate() {
            let color = colors[i % colors.len()].to_string();
            renderer.add_bar(
                format!("item_{}", i),
                name.clone(),
                *val,
                color,
            );
        }
        
        let svg = match chart_t.as_str() {
            "bar" => renderer.render_bar_chart(),
            "scatter" => renderer.render_scatter_chart(),
            "line" => renderer.render_line_chart(),
            _ => renderer.render_bar_chart(),
        };
        
        if let Ok(mut cache) = SVG_CACHE.lock() {
            if cache.len() > 100 {
                cache.clear();
            }
            cache.insert(cache_key, svg.clone());
        }
        
        CString::new(svg).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn sera_render_clustered_chart(
    values_ptr: *const c_double,
    names_ptr: *const *const c_char,
    count: c_uint,
    damage_type: *const c_char,
    chart_type: *const c_char,
    cluster_radius: c_double,
) -> *mut c_char {
    use crate::render::InteractiveSvgRenderer;
    
    if values_ptr.is_null() || names_ptr.is_null() {
        return CString::new("").unwrap().into_raw();
    }
    
    unsafe {
        let values: Vec<f64> = std::slice::from_raw_parts(values_ptr, count as usize).to_vec();
        let names_slice = std::slice::from_raw_parts(names_ptr, count as usize);
        let names: Vec<String> = names_slice
            .iter()
            .map(|&ptr| CStr::from_ptr(ptr).to_string_lossy().into_owned())
            .collect();
        
        let dmg_type = CStr::from_ptr(damage_type)
            .to_string_lossy()
            .into_owned();
        let chart_t = CStr::from_ptr(chart_type)
            .to_string_lossy()
            .into_owned();
        
        let cache_key = format!("cluster_{}_{}_r{}", dmg_type, chart_t, cluster_radius as i32);
        
        if let Ok(cache) = SVG_CACHE.lock() {
            if let Some(cached) = cache.get(&cache_key) {
                return CString::new(cached.clone()).unwrap().into_raw();
            }
        }
        
        let mut renderer = InteractiveSvgRenderer::new(1000.0, 500.0, &dmg_type);
        
        let clusters = cluster_points(&values, cluster_radius as f32);
        let colors = vec![
            "#667eea", "#764ba2", "#f093fb", "#4facfe", "#00f2fe",
            "#43e97b", "#fa709a", "#fee140", "#30cfd0", "#330867"
        ];
        
        for (cluster_idx, cluster) in clusters.iter().enumerate() {
            let cluster_value: f64 = cluster.iter().map(|&i| values[i]).sum::<f64>() / cluster.len() as f64;
            let cluster_label = if cluster.len() > 1 {
                format!("{} items", cluster.len())
            } else {
                names[cluster[0]].clone()
            };
            
            let color = colors[cluster_idx % colors.len()].to_string();
            renderer.add_bar(
                format!("cluster_{}", cluster_idx),
                cluster_label,
                cluster_value,
                color,
            );
        }
        
        let svg = match chart_t.as_str() {
            "bar" => renderer.render_bar_chart(),
            "scatter" => renderer.render_scatter_chart(),
            "line" => renderer.render_line_chart(),
            _ => renderer.render_bar_chart(),
        };
        
        if let Ok(mut cache) = SVG_CACHE.lock() {
            if cache.len() > 100 {
                cache.clear();
            }
            cache.insert(cache_key, svg.clone());
        }
        
        CString::new(svg).unwrap().into_raw()
    }
}

#[no_mangle]
pub extern "C" fn sera_render_clustered_chart_filtered(
    values_ptr: *const c_double,
    names_ptr: *const *const c_char,
    count: c_uint,
    damage_type: *const c_char,
    chart_type: *const c_char,
    cluster_radius: c_double,
    hidden_indices: *const c_uint,
    hidden_count: c_uint,
) -> *mut c_char {
    use crate::render::InteractiveSvgRenderer;
    
    if values_ptr.is_null() || names_ptr.is_null() {
        return CString::new("").unwrap().into_raw();
    }
    
    unsafe {
        let values: Vec<f64> = std::slice::from_raw_parts(values_ptr, count as usize).to_vec();
        let names_slice = std::slice::from_raw_parts(names_ptr, count as usize);
        let names: Vec<String> = names_slice
            .iter()
            .map(|&ptr| CStr::from_ptr(ptr).to_string_lossy().into_owned())
            .collect();
        
        let (filtered_vals, filtered_names) = filter_data(&values, &names, hidden_indices, hidden_count);
        
        let dmg_type = CStr::from_ptr(damage_type)
            .to_string_lossy()
            .into_owned();
        let chart_t = CStr::from_ptr(chart_type)
            .to_string_lossy()
            .into_owned();
        
        let mut renderer = InteractiveSvgRenderer::new(1000.0, 500.0, &dmg_type);
        
        let clusters = cluster_points(&filtered_vals, cluster_radius as f32);
        let colors = vec![
            "#667eea", "#764ba2", "#f093fb", "#4facfe", "#00f2fe",
            "#43e97b", "#fa709a", "#fee140", "#30cfd0", "#330867"
        ];
        
        for (cluster_idx, cluster) in clusters.iter().enumerate() {
            let cluster_value: f64 = cluster.iter().map(|&i| filtered_vals[i]).sum::<f64>() / cluster.len() as f64;
            let cluster_label = if cluster.len() > 1 {
                format!("{} items", cluster.len())
            } else {
                filtered_names[cluster[0]].clone()
            };
            
            let color = colors[cluster_idx % colors.len()].to_string();
            renderer.add_bar(
                format!("cluster_{}", cluster_idx),
                cluster_label,
                cluster_value,
                color,
            );
        }
        
        let svg = match chart_t.as_str() {
            "bar" => renderer.render_bar_chart(),
            "scatter" => renderer.render_scatter_chart(),
            "line" => renderer.render_line_chart(),
            _ => renderer.render_bar_chart(),
        };
        
        CString::new(svg).unwrap().into_raw()
    }
}

fn cluster_points(values: &[f64], radius: f32) -> Vec<Vec<usize>> {
    let mut clusters: Vec<Vec<usize>> = Vec::new();
    let mut visited = vec![false; values.len()];
    
    if values.is_empty() {
        return clusters;
    }
    
    let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let threshold = (max_val * radius as f64) / 100.0;
    
    for i in 0..values.len() {
        if visited[i] {
            continue;
        }
        
        let mut cluster = vec![i];
        visited[i] = true;
        
        for j in (i + 1)..values.len() {
            if !visited[j] && (values[i] - values[j]).abs() <= threshold {
                cluster.push(j);
                visited[j] = true;
            }
        }
        
        clusters.push(cluster);
    }
    
    clusters
}

#[no_mangle]
pub extern "C" fn sera_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}
