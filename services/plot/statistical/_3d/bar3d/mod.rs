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
    extra_js.push_str(&format!("];var BZK={:.3},BZM=1.6;", height_ratio));

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
    fn every_scene_applies_to_every_bar_variant() {
        use crate::plot::scene3d::Scene3DVariant;
        for (variant_key, json) in variant_demos() {
            for (scene_key, _) in Scene3DVariant::keys_and_aliases() {
                let html = crate::plot::build_bar3d_chart(&set_field(&json, "scene", scene_key));
                assert!(html.contains("class=\"c3w\""), "{variant_key} must render under the {scene_key} scene");
                assert!(html.contains("var BN="), "{variant_key} must emit block data under the {scene_key} scene");
            }
        }
    }

    #[test]
    fn the_public_builder_forwards_sort_order_to_single_series_variants() {
        let json = r#"{"title":"t","labels":["A","B","C","D"],"values":[10,40,20,30],"sort_order":"desc"}"#;
        let html = crate::plot::build_bar3d_chart(json);
        assert!(
            html.contains("BZ1=[40.0000,30.0000,20.0000,10.0000]"),
            "desc order must reach the block heights"
        );
    }

    #[test]
    fn every_chart_theme_styles_every_bar_variant() {
        use crate::plot::statistical::ChartTheme;
        for (variant_key, json) in variant_demos() {
            let plain = crate::plot::build_bar3d_chart(&json);
            for (theme_key, _) in ChartTheme::keys_and_aliases() {
                if *theme_key == ChartTheme::default_key() {
                    continue;
                }
                let themed = crate::plot::build_bar3d_chart(&set_field(&json, "theme", theme_key));
                assert!(themed.contains("class=\"c3w\""), "{variant_key} must stay a 3D canvas under {theme_key}");
                assert!(themed.contains(".c3w canvas{filter:"), "{variant_key} must be styled by the {theme_key} theme");
                assert_ne!(themed, plain, "{variant_key} must differ under {theme_key}");
            }
        }
    }

    #[test]
    fn the_registry_exposes_scene_plane_and_theme_axes_on_the_twin_family() {
        let variants = crate::chart_variants();
        let axes = &variants["bar_3d"]["axes"];
        for (axis, expected) in [
            ("scene", crate::plot::scene3d::Scene3DVariant::keys_and_aliases().len()),
            ("orientation3d", crate::plot::scene3d::Orientation3D::keys_and_aliases().len()),
            ("theme", crate::plot::statistical::ChartTheme::keys_and_aliases().len()),
        ] {
            assert_eq!(axes[axis]["keys"].as_array().map(|k| k.len()), Some(expected), "{axis} axis must list every key");
            assert!(axes[axis]["default"].is_string(), "{axis} axis must name its default");
        }
        assert_eq!(variants["bar_3d"]["variants"].as_array().map(|v| v.len()), Some(BarVariant::all().len()));
    }

    #[test]
    fn sort_order_reorders_the_columns_like_the_2d_chart() {
        use crate::plot::statistical::bar::{layout_3d, BarConfig};
        let labels: Vec<String> = ["A", "B", "C", "D"].iter().map(|s| s.to_string()).collect();
        let values = [10.0, 40.0, 20.0, 30.0];
        for (order, expected) in [
            ("none", vec![10.0, 40.0, 20.0, 30.0]),
            ("desc", vec![40.0, 30.0, 20.0, 10.0]),
            ("asc", vec![10.0, 20.0, 30.0, 40.0]),
        ] {
            let cfg = BarConfig {
                labels: &labels,
                values: &values,
                sort_order: order,
                ..BarConfig::default()
            };
            let tops: Vec<f64> = layout_3d(&cfg).iter().map(|b| b.z1).collect();
            assert_eq!(tops, expected, "sort_order={order}");
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
        for (theme_key, _) in crate::plot::statistical::ChartTheme::keys_and_aliases() {
            if *theme_key == crate::plot::statistical::ChartTheme::default_key() {
                continue;
            }
            let html = crate::plot::build_bar3d_chart(&set_field(&default_json, "theme", theme_key));
            std::fs::write(format!("docs/previews/bar3d-theme-{theme_key}.html"), &html).unwrap();
        }
        for (scene_key, _) in crate::plot::scene3d::Scene3DVariant::keys_and_aliases() {
            let html = crate::plot::build_bar3d_chart(&set_field(&default_json, "scene", scene_key));
            std::fs::write(format!("docs/previews/bar3d-scene-{scene_key}.html"), &html).unwrap();
        }
    }
}
