pub mod builders;

pub use builders::*;
pub use crate::plot::{set_global_bg, set_global_pal, set_global_grid, get_global_bg, get_global_pal, get_global_grid, set_global_background, reset_global_background, set_theme, reset_theme, themes, build_slideshow, build_hover_json, chart_append, export_svg, export_data_url, export_html_file, chart_info, validate_input, downsample_lttb, chart_diff, drift_ks, scale_plan, system_profile, csv_count_rows, csv_chunk_read};
pub use crate::plot::{ChartOpts, ChartArgs, Annotation, parse_all, parse_opts, parse_args, apply, apply_h, apply_bg3d, apply_annotations, build_html_chart};

fn parse_json_value(input: &str) -> Option<serde_json::Value> {
    serde_json::from_str(input).ok()
}

pub fn build_grid(input: &str) -> String {
    #[derive(serde::Deserialize, Default)]
    struct In {
        html_parts: Option<Vec<String>>,
        cols: Option<usize>,
        gap: Option<i32>,
        bg_color: Option<String>,
        title: Option<String>,
        cell_height: Option<i32>,
    }
    let i: In = serde_json::from_str(input).unwrap_or_default();
    crate::plot::layout::build_grid_impl(
        i.html_parts.unwrap_or_default(),
        i.cols.unwrap_or(3),
        i.gap.unwrap_or(16),
        i.bg_color.as_deref().unwrap_or("#0a0f1c"),
        i.title.as_deref().unwrap_or(""),
        i.cell_height,
    )
}

pub fn grid(input: &str) -> String {
    build_grid(input)
}

pub fn build_sysmon(input: &str) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = input;
        return "<!DOCTYPE html><html><body style=\"margin:0;display:flex;align-items:center;justify-content:center;min-height:100vh;background:#0a0f1c;color:#e2e8f0;font-family:system-ui\">System monitor is unavailable on wasm targets.</body></html>".to_string();
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
    #[derive(serde::Deserialize, Default)]
    struct In {
        bg_color: Option<String>,
        update_interval_ms: Option<u32>,
    }
    let i: In = serde_json::from_str(input).unwrap_or_default();
    crate::plot::layout::build_sysmon_html(
        i.bg_color.as_deref().unwrap_or("#0a0f1c"),
        i.update_interval_ms.unwrap_or(2000),
    )
    }
}

pub fn sysmon(input: &str) -> String {
    build_sysmon(input)
}

pub fn plot(input: &str) -> String {
    crate::plot::plot_chart(input)
}

pub fn set_bg(input: &str) -> String {
    #[derive(serde::Deserialize, Default)]
    struct In {
        html: Option<String>,
        color: Option<String>,
    }
    if let Ok(payload) = serde_json::from_str::<In>(input) {
        return crate::html::hover::apply_bg(payload.html.unwrap_or_default(), payload.color.as_deref());
    }
    crate::html::hover::apply_bg(input.to_string(), None)
}

pub fn show_chart_value(input: &str) -> String {
    crate::plot::show_chart_value(input).to_string()
}

pub fn bench_chart_value(input: &str) -> String {
    crate::plot::bench_chart_value(input).to_string()
}

pub fn set_chart_kind(input: &str) -> String {
    let kind = parse_json_value(input)
        .and_then(|value| value.get("kind").and_then(|kind| kind.as_u64()).or_else(|| value.as_u64()))
        .unwrap_or(0) as u8;
    crate::plot::set_chart_kind(kind);
    "true".to_string()
}

pub fn set_chart_orientation(input: &str) -> String {
    let vertical = parse_json_value(input)
        .and_then(|value| value.get("vertical").and_then(|vertical| vertical.as_bool()).or_else(|| value.as_bool()))
        .unwrap_or(false);
    crate::plot::set_chart_orientation(vertical);
    "true".to_string()
}

pub fn bench_pure_rust(input: &str) -> String {
    let n = parse_json_value(input)
        .and_then(|value| value.get("n").and_then(|count| count.as_u64()).or_else(|| value.as_u64()))
        .unwrap_or(2000) as usize;
    let (hist_ms, bar_ms, scatter_ms, heatmap_ms) = crate::plot::bench_pure_rust(n);
    serde_json::json!({
        "histogram_ms": hist_ms,
        "bar_ms": bar_ms,
        "scatter_ms": scatter_ms,
        "heatmap_ms": heatmap_ms,
    })
    .to_string()
}

pub fn push_telemetry(input: &str) -> String {
    #[derive(serde::Deserialize, Default)]
    struct In {
        endpoint: Option<String>,
        token: Option<String>,
    }
    let payload: In = serde_json::from_str(input).unwrap_or_default();
    match crate::telemetry::push_pending_to_endpoint(
        payload.endpoint.as_deref().unwrap_or(""),
        payload.token.as_deref().unwrap_or(""),
    ) {
        Ok(count) => serde_json::json!({"ok": true, "count": count}).to_string(),
        Err(error) => serde_json::json!({"ok": false, "error": error}).to_string(),
    }
}
