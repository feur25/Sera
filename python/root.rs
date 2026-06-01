use pyo3::prelude::*;
use pyo3::types::PyAny;

use crate::{
    __sera_py_chart_variants, __sera_py_demo, __sera_py_demos, __sera_py_doc, __sera_py_docs,
    __sera_py_get_metrics, __sera_py_params, __sera_py_required_params, __sera_py_reset_config,
    __sera_py_reset_global_background, __sera_py_reset_theme, __sera_py_set_auto_display,
    __sera_py_set_global_background, __sera_py_telemetry_consent, __sera_py_telemetry_path,
    __sera_py_themes,
};

struct ThemePreset {
    bg: Option<&'static str>,
    palette: &'static [u32],
    gridlines: bool,
}

const THEME_DARK: ThemePreset = ThemePreset {
    bg: Some("#0f172a"),
    palette: &[0x818CF8, 0xFB7185, 0x34D399, 0xFBBF24, 0xA78BFA, 0x22D3EE, 0xF472B6, 0xA3E635, 0xF87171, 0x2DD4BF],
    gridlines: true,
};

const THEME_LIGHT: ThemePreset = ThemePreset {
    bg: None,
    palette: &[0x6366F1, 0xF43F5E, 0x10B981, 0xF59E0B, 0x8B5CF6, 0x06B6D4, 0xEC4899, 0x84CC16, 0xEF4444, 0x14B8A6],
    gridlines: false,
};

const THEME_SCIENTIFIC: ThemePreset = ThemePreset {
    bg: Some("#fafafa"),
    palette: &[0x1F77B4, 0xFF7F0E, 0x2CA02C, 0xD62728, 0x9467BD, 0x8C564B, 0xE377C2, 0x7F7F7F, 0xBCBD22, 0x17BECF],
    gridlines: true,
};

const THEME_APPLE: ThemePreset = ThemePreset {
    bg: Some("#000000"),
    palette: &[0x0A84FF, 0x30D158, 0xFF453A, 0xFFD60A, 0xBF5AF2, 0x64D2FF, 0xFF9F0A, 0xFF375F, 0xAC8E68, 0x63E6E2],
    gridlines: false,
};

const THEME_NOTION: ThemePreset = ThemePreset {
    bg: Some("#191919"),
    palette: &[0x529CCA, 0xD08B65, 0x6C9B7D, 0xCB7C7A, 0x9A6DD7, 0x868686, 0xCCAA55, 0x75B5AA, 0xD477A8, 0x507AA6],
    gridlines: false,
};

const THEME_MINIMAL: ThemePreset = ThemePreset {
    bg: None,
    palette: &[0x374151, 0x6B7280, 0x9CA3AF, 0xD1D5DB, 0x111827, 0x4B5563, 0x1F2937, 0xE5E7EB, 0x030712, 0x6B7280],
    gridlines: false,
};

const THEME_NEON: ThemePreset = ThemePreset {
    bg: Some("#0a0a0a"),
    palette: &[0x00FF87, 0xFF006E, 0x00B4D8, 0xFFBE0B, 0xE500A4, 0x8338EC, 0x3A86FF, 0xFB5607, 0xFF006E, 0x06D6A0],
    gridlines: false,
};

fn resolve_theme(name: &str) -> Option<&'static ThemePreset> {
    match name {
        "dark" => Some(&THEME_DARK),
        "light" => Some(&THEME_LIGHT),
        "scientific" => Some(&THEME_SCIENTIFIC),
        "apple" => Some(&THEME_APPLE),
        "notion" => Some(&THEME_NOTION),
        "minimal" => Some(&THEME_MINIMAL),
        "neon" => Some(&THEME_NEON),
        _ => None,
    }
}

#[crate::sera_doc(category = "theme", file = "theme/theme.md", en = "Applies a named color theme to all subsequent chart renders.", fr = "Applique un thème de couleurs nommé à tous les rendus de graphiques suivants.", param(name = "name", ty = "str", en = "Theme name (e.g. 'dark', 'light', 'ocean'). Use sp.themes() to list all.", fr = "Nom du thème (ex: 'dark', 'light', 'ocean'). Utilisez sp.themes() pour lister tous les thèmes."))]
#[pyfunction]
#[pyo3(signature = (name))]
pub fn theme(name: &str) -> PyResult<()> {
    let preset = resolve_theme(name).ok_or_else(|| {
        pyo3::exceptions::PyValueError::new_err(format!(
            "Unknown theme '{}'. Available: dark, light, scientific, apple, notion, minimal, neon",
            name
        ))
    })?;
    if let Ok(mut bg) = crate::GLOBAL_BACKGROUND.lock() {
        *bg = preset.bg.map(|value| value.to_string());
    }
    if let Ok(mut palette) = crate::GLOBAL_PALETTE.lock() {
        *palette = Some(preset.palette.to_vec());
    }
    crate::GLOBAL_GRIDLINES.store(preset.gridlines, std::sync::atomic::Ordering::Relaxed);
    if let Ok(mut theme_name) = crate::GLOBAL_THEME_NAME.lock() {
        *theme_name = Some(name.to_string());
    }
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (*, font=None, font_size=None, title_size=None, border_radius=None, opacity=None, responsive=None, animation=None, animation_duration=None, crosshair=None, zoom=None, tooltip=None, locale=None, thousands_sep=None, margin=None, export_button=None, palette=None, background=None, gridlines=None, text_auto=None, text_position=None, text_angle=None, text_font_size=None, text_font_color=None, uniform_text_min_size=None, uniform_text_mode=None, bar_corner_radius=None))]
pub fn config(font: Option<&str>, font_size: Option<i32>, title_size: Option<i32>, border_radius: Option<i32>, opacity: Option<f64>, responsive: Option<bool>, animation: Option<bool>, animation_duration: Option<i32>, crosshair: Option<bool>, zoom: Option<bool>, tooltip: Option<&str>, locale: Option<&str>, thousands_sep: Option<&str>, margin: Option<i32>, export_button: Option<bool>, palette: Option<Vec<u32>>, background: Option<&str>, gridlines: Option<bool>, text_auto: Option<&Bound<'_, PyAny>>, text_position: Option<&str>, text_angle: Option<i32>, text_font_size: Option<i32>, text_font_color: Option<&str>, uniform_text_min_size: Option<i32>, uniform_text_mode: Option<&str>, bar_corner_radius: Option<&Bound<'_, PyAny>>) {
    use std::sync::atomic::Ordering::Relaxed;

    if let Some(value) = font { if let Ok(mut field) = crate::GLOBAL_FONT.lock() { *field = Some(value.to_string()); } }
    if let Some(value) = font_size { crate::GLOBAL_FONT_SIZE.store(value, Relaxed); }
    if let Some(value) = title_size { crate::GLOBAL_TITLE_SIZE.store(value, Relaxed); }
    if let Some(value) = border_radius { crate::GLOBAL_BORDER_RADIUS.store(value, Relaxed); }
    if let Some(value) = opacity { if let Ok(mut field) = crate::GLOBAL_OPACITY.lock() { *field = if value < 1.0 { Some(value) } else { None }; } }
    if let Some(value) = responsive { crate::GLOBAL_RESPONSIVE.store(value, Relaxed); }
    if let Some(value) = animation { crate::GLOBAL_ANIMATION.store(value, Relaxed); }
    if let Some(value) = animation_duration { crate::GLOBAL_ANIMATION_DURATION.store(value, Relaxed); }
    if let Some(value) = crosshair { crate::GLOBAL_CROSSHAIR.store(value, Relaxed); }
    if let Some(value) = zoom { crate::GLOBAL_ZOOM.store(value, Relaxed); }
    if let Some(value) = tooltip { if let Ok(mut field) = crate::GLOBAL_TOOLTIP.lock() { *field = Some(value.to_string()); } }
    if let Some(value) = locale { if let Ok(mut field) = crate::GLOBAL_LOCALE.lock() { *field = Some(value.to_string()); } }
    if let Some(value) = thousands_sep { if let Ok(mut field) = crate::GLOBAL_THOUSANDS_SEP.lock() { *field = Some(value.to_string()); } }
    if let Some(value) = margin { crate::GLOBAL_MARGIN.store(value, Relaxed); }
    if let Some(value) = export_button { crate::GLOBAL_EXPORT_BTN.store(value, Relaxed); }
    if let Some(value) = background { if let Ok(mut field) = crate::GLOBAL_BACKGROUND.lock() { *field = Some(value.to_string()); } }
    if let Some(value) = palette { if let Ok(mut field) = crate::GLOBAL_PALETTE.lock() { *field = Some(value); } }
    if let Some(value) = gridlines { crate::GLOBAL_GRIDLINES.store(value, Relaxed); }
    if let Some(value) = text_auto {
        let text = if let Ok(boolean) = value.extract::<bool>() {
            if boolean {
                String::new()
            } else {
                reset_text_auto();
                return;
            }
        } else if let Ok(text) = value.extract::<String>() {
            text
        } else {
            return;
        };
        if let Ok(mut field) = crate::GLOBAL_TEXT_AUTO.lock() { *field = Some(text); }
    }
    if let Some(value) = text_position { if let Ok(mut field) = crate::GLOBAL_TEXT_POSITION.lock() { *field = Some(value.to_string()); } }
    if let Some(value) = text_angle { crate::GLOBAL_TEXT_ANGLE.store(value, Relaxed); }
    if let Some(value) = text_font_size { crate::GLOBAL_TEXT_FONT_SIZE.store(value, Relaxed); }
    if let Some(value) = text_font_color { if let Ok(mut field) = crate::GLOBAL_TEXT_FONT_COLOR.lock() { *field = Some(value.to_string()); } }
    if let Some(value) = uniform_text_min_size { crate::GLOBAL_UNIFORM_TEXT_MIN.store(value, Relaxed); }
    if let Some(value) = uniform_text_mode { if let Ok(mut field) = crate::GLOBAL_UNIFORM_TEXT_MODE.lock() { *field = Some(value.to_string()); } }
    if let Some(value) = bar_corner_radius {
        let radius = if let Ok(integer) = value.extract::<i32>() {
            integer.to_string()
        } else if let Ok(number) = value.extract::<f64>() {
            number.to_string()
        } else if let Ok(text) = value.extract::<String>() {
            text
        } else {
            return;
        };
        if let Ok(mut field) = crate::GLOBAL_BAR_CORNER_RADIUS.lock() { *field = Some(radius); }
    }
}

fn reset_text_auto() {
    if let Ok(mut field) = crate::GLOBAL_TEXT_AUTO.lock() {
        *field = None;
    }
}

pub fn register_root_functions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(__sera_py_set_global_background, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_reset_global_background, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_set_auto_display, m)?)?;
    m.add_function(wrap_pyfunction!(theme, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_reset_theme, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_themes, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_demos, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_demo, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_chart_variants, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_params, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_required_params, m)?)?;
    m.add_function(wrap_pyfunction!(config, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_reset_config, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_telemetry_consent, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_telemetry_path, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_get_metrics, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_docs, m)?)?;
    m.add_function(wrap_pyfunction!(__sera_py_doc, m)?)?;
    Ok(())
}