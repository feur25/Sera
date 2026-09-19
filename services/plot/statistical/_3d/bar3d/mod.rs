use crate::plot::statistical::bar::{height_ratio_3d, layout_3d, Bar3DBlock, BarConfig};

pub fn render_bar3d_blocks_html(
    title: &str,
    cfg: &BarConfig,
    axis_labels: (&str, &str, &str),
    color_labels: &[String],
    w: i32,
    h: i32,
    bg_color: Option<&str>,
    scene: &str,
) -> String {
    render_blocks3d_scaled_html(title, &layout_3d(cfg), height_ratio_3d(cfg.variant), axis_labels, color_labels, w, h, bg_color, scene)
}

pub fn render_blocks3d_html(
    title: &str,
    blocks: &[Bar3DBlock],
    axis_labels: (&str, &str, &str),
    color_labels: &[String],
    w: i32,
    h: i32,
    bg_color: Option<&str>,
    scene: &str,
) -> String {
    render_blocks3d_scaled_html(title, blocks, 1.0, axis_labels, color_labels, w, h, bg_color, scene)
}

pub fn render_blocks3d_scaled_html(
    title: &str,
    blocks: &[Bar3DBlock],
    height_ratio: f64,
    axis_labels: (&str, &str, &str),
    color_labels: &[String],
    w: i32,
    h: i32,
    bg_color: Option<&str>,
    scene: &str,
) -> String {
    let mut extra_js = String::with_capacity(blocks.len() * 48 + 32);
    extra_js.push_str("var BN=");
    extra_js.push_str(&blocks.len().to_string());
    extra_js.push_str(",BX=[");
    for (i, b) in blocks.iter().enumerate() {
        if i > 0 {
            extra_js.push(',');
        }
        extra_js.push_str(&format!("{:.4}", b.cx));
    }
    extra_js.push_str("],BY=[");
    for (i, b) in blocks.iter().enumerate() {
        if i > 0 {
            extra_js.push(',');
        }
        extra_js.push_str(&format!("{:.4}", b.cy));
    }
    extra_js.push_str("],BZ0=[");
    for (i, b) in blocks.iter().enumerate() {
        if i > 0 {
            extra_js.push(',');
        }
        extra_js.push_str(&format!("{:.4}", b.z0));
    }
    extra_js.push_str("],BZ1=[");
    for (i, b) in blocks.iter().enumerate() {
        if i > 0 {
            extra_js.push(',');
        }
        extra_js.push_str(&format!("{:.4}", b.z1));
    }
    extra_js.push_str("],BHW=[");
    for (i, b) in blocks.iter().enumerate() {
        if i > 0 {
            extra_js.push(',');
        }
        extra_js.push_str(&format!("{:.4}", b.hw));
    }
    extra_js.push_str("],BHD=[");
    for (i, b) in blocks.iter().enumerate() {
        if i > 0 {
            extra_js.push(',');
        }
        extra_js.push_str(&format!("{:.4}", b.hd));
    }
    extra_js.push_str("],BCI=[");
    for (i, b) in blocks.iter().enumerate() {
        if i > 0 {
            extra_js.push(',');
        }
        extra_js.push_str(&b.ci.to_string());
    }
    extra_js.push_str(&format!("];var BZK={:.3};", height_ratio));

    let (x, y, z): (Vec<f64>, Vec<f64>, Vec<f64>) = blocks
        .iter()
        .map(|b| (b.cx, b.cy, b.z1))
        .fold(
            (Vec::new(), Vec::new(), Vec::new()),
            |(mut xs, mut ys, mut zs), (x, y, z)| {
                xs.push(x);
                ys.push(y);
                zs.push(z);
                (xs, ys, zs)
            },
        );
    let x = if x.is_empty() { vec![0.0] } else { x };
    let y = if y.is_empty() { vec![0.0] } else { y };
    let z = if z.is_empty() { vec![0.0] } else { z };

    crate::html::js_3d::render_3d_html_impl(
        1,
        title,
        &x,
        &y,
        &z,
        axis_labels,
        &[],
        color_labels,
        w,
        h,
        bg_color,
        scene,
        extra_js.as_bytes(),
    )
}

#[cfg(test)]
mod tests {
    use crate::plot::chart_demo_registry::{iter_entries, kwargs_to_json};
    use crate::plot::statistical::BarVariant;

    fn stem_of(file: &str) -> String {
        file.replace('\\', "/")
            .rsplit('/')
            .next()
            .unwrap_or("")
            .trim_end_matches(".rs")
            .to_string()
    }

    fn set_field(json: &str, field: &str, value: &str) -> String {
        let mut parsed: serde_json::Value = serde_json::from_str(json).expect("demo json");
        if let Some(obj) = parsed.as_object_mut() {
            obj.insert(field.to_string(), serde_json::Value::String(value.to_string()));
        }
        parsed.to_string()
    }

    fn with_variant(kwargs: &str, key: &str) -> String {
        set_field(&kwargs_to_json(kwargs), "variant", key)
    }

    fn variant_demos() -> Vec<(&'static str, String)> {
        let demos: Vec<(String, &'static str)> = iter_entries()
            .filter(|e| e.file.replace('\\', "/").contains("statistical/bar/"))
            .map(|e| (stem_of(e.file), e.kwargs))
            .collect();
        let fallback = demos
            .iter()
            .find(|(stem, _)| stem.as_str() == BarVariant::default_key())
            .map(|(_, kwargs)| *kwargs)
            .unwrap_or("");
        BarVariant::keys_and_aliases()
            .iter()
            .map(|(key, aliases)| {
                let kwargs = demos
                    .iter()
                    .find(|(stem, _)| stem.as_str() == *key || aliases.iter().any(|a| *a == stem.as_str()))
                    .map(|(_, kwargs)| *kwargs)
                    .unwrap_or(fallback);
                (*key, with_variant(kwargs, key))
            })
            .collect()
    }

    #[test]
    fn every_bar_variant_has_a_working_3d_counterpart() {
        let demos = variant_demos();
        assert_eq!(demos.len(), BarVariant::all().len());
        for (key, json) in demos {
            let html = crate::plot::build_bar3d_chart(&json);
            assert!(html.contains("class=\"c3w\""), "{key} must render a 3D canvas");
            assert!(html.contains("var BN="), "{key} must emit block data");
            assert!(!html.contains("var BN=0,"), "{key} must emit at least one 3D block");
        }
    }

    #[test]
    fn every_3d_plane_applies_to_every_bar_variant() {
        use crate::plot::scene3d::Orientation3D;
        for (variant_key, json) in variant_demos() {
            for (plane_key, _) in Orientation3D::keys_and_aliases() {
                let (yaw, pitch) = Orientation3D::from_str(plane_key).angles();
                let html = crate::plot::build_bar3d_chart(&set_field(&json, "orientation3d", plane_key));
                assert!(
                    html.contains(&format!("var yaw={:.4},pitch={:.4}", yaw, pitch)),
                    "{variant_key} must honour the {plane_key} plane"
                );
            }
        }
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        use crate::plot::scene3d::Orientation3D;
        let demos = variant_demos();
        for (key, json) in &demos {
            let html = crate::plot::build_bar3d_chart(json);
            std::fs::write(format!("docs/previews/bar3d-{key}.html"), &html).unwrap();
            if *key == BarVariant::default_key() {
                std::fs::write("docs/previews/bar3d.html", &html).unwrap();
            }
        }
        let default_json = demos
            .iter()
            .find(|(key, _)| *key == BarVariant::default_key())
            .map(|(_, json)| json.clone())
            .unwrap();
        for (plane_key, _) in Orientation3D::keys_and_aliases() {
            let html = crate::plot::build_bar3d_chart(&set_field(&default_json, "orientation3d", plane_key));
            std::fs::write(format!("docs/previews/bar3d-plane-{plane_key}.html"), &html).unwrap();
        }
    }
}
