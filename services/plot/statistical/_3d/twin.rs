use crate::plot::chart_demo_registry::{iter_entries, kwargs_to_json};
use crate::plot::scene3d::{Orientation3D, Scene3DVariant};
use crate::plot::statistical::ChartTheme;

pub type Keys = &'static [(&'static str, &'static [&'static str])];
pub type Demos = Vec<(&'static str, String)>;

fn stem_of(file: &str) -> String {
    file.replace('\\', "/")
        .rsplit('/')
        .next()
        .unwrap_or("")
        .trim_end_matches(".rs")
        .to_string()
}

pub fn set_field(json: &str, field: &str, value: &str) -> String {
    let mut parsed: serde_json::Value = serde_json::from_str(json).expect("demo json");
    if let Some(obj) = parsed.as_object_mut() {
        obj.insert(field.to_string(), serde_json::Value::String(value.to_string()));
    }
    parsed.to_string()
}

pub fn variant_demos(dir: &str, keys: Keys, default_key: &str) -> Demos {
    let demos: Vec<(String, &'static str)> = iter_entries()
        .filter(|e| e.file.replace('\\', "/").contains(dir))
        .map(|e| (stem_of(e.file), e.kwargs))
        .collect();
    let fallback = demos
        .iter()
        .find(|(stem, _)| stem.as_str() == default_key)
        .map(|(_, kwargs)| *kwargs)
        .unwrap_or("");
    keys.iter()
        .map(|(key, aliases)| {
            let kwargs = demos
                .iter()
                .find(|(stem, _)| stem.as_str() == *key || aliases.iter().any(|a| *a == stem.as_str()))
                .map(|(_, kwargs)| *kwargs)
                .unwrap_or(fallback);
            (*key, set_field(&kwargs_to_json(kwargs), "variant", key))
        })
        .collect()
}

pub fn assert_blocks(html: &str, what: &str) {
    assert!(html.contains("class=\"c3w\""), "{what} must render a 3D canvas");
    assert!(html.contains("var BN="), "{what} must emit block data");
    assert!(!html.contains("var BN=0,"), "{what} must emit at least one 3D block");
}

pub fn check_variants(build: fn(&str) -> String, demos: &Demos, expected: usize) {
    assert_eq!(demos.len(), expected);
    for (key, json) in demos {
        assert_blocks(&build(json), key);
    }
}

pub fn check_planes(build: fn(&str) -> String, demos: &Demos) {
    for (variant_key, json) in demos {
        for (plane_key, _) in Orientation3D::keys_and_aliases() {
            let (yaw, pitch) = Orientation3D::from_str(plane_key).angles();
            let html = build(&set_field(json, "orientation3d", plane_key));
            assert!(
                html.contains(&format!("var yaw={:.4},pitch={:.4}", yaw, pitch)),
                "{variant_key} must honour the {plane_key} plane"
            );
        }
    }
}

pub fn check_scenes(build: fn(&str) -> String, demos: &Demos) {
    for (variant_key, json) in demos {
        for (scene_key, _) in Scene3DVariant::keys_and_aliases() {
            assert_blocks(&build(&set_field(json, "scene", scene_key)), &format!("{variant_key} under {scene_key}"));
        }
    }
}

pub fn check_themes(build: fn(&str) -> String, demos: &Demos) {
    for (variant_key, json) in demos {
        let plain = build(json);
        for (theme_key, _) in ChartTheme::keys_and_aliases() {
            if *theme_key == ChartTheme::default_key() {
                continue;
            }
            let themed = build(&set_field(json, "theme", theme_key));
            assert!(themed.contains(".c3w canvas{filter:"), "{variant_key} must be styled by the {theme_key} theme");
            assert_ne!(themed, plain, "{variant_key} must differ under {theme_key}");
        }
    }
}

pub fn check_zone(build: fn(&str) -> String, demos: &Demos) {
    for (variant_key, json) in demos {
        assert!(build(json).contains("var BFIT="), "{variant_key} must carry its fitted zone");
        let mut forced: serde_json::Value = serde_json::from_str(json).expect("demo json");
        forced["zone"] = serde_json::json!([2.0, 1.0, 1.0]);
        let html = build(&forced.to_string());
        assert!(
            html.contains("\"lx\":1.000000,\"ly\":0.500000,\"lz\":0.500000"),
            "{variant_key} must honour an explicit zone"
        );
    }
}

fn reshape(value: &mut serde_json::Value, len: &dyn Fn(usize) -> usize, keep_outer: bool) {
    match value {
        serde_json::Value::Array(items) => {
            let nested = items.iter().any(|v| v.is_array());
            if !(nested && keep_outer) {
                let base = std::mem::take(items);
                if !base.is_empty() {
                    *items = (0..len(base.len())).map(|i| base[i % base.len()].clone()).collect();
                }
            }
            if nested {
                items.iter_mut().for_each(|v| reshape(v, len, keep_outer));
            }
        }
        serde_json::Value::Object(map) => {
            map.iter_mut().filter(|(k, _)| k.as_str() != "zone").for_each(|(_, v)| reshape(v, len, keep_outer));
        }
        _ => {}
    }
}

fn reshaped(json: &str, len: &dyn Fn(usize) -> usize, keep_outer: bool) -> String {
    let mut parsed: serde_json::Value = serde_json::from_str(json).expect("demo json");
    reshape(&mut parsed, len, keep_outer);
    parsed.to_string()
}

pub fn block_count(html: &str) -> usize {
    html.split("var BN=").nth(1).and_then(|s| s.split(',').next()).and_then(|s| s.parse().ok()).unwrap_or(0)
}

fn scene_script(html: &str) -> &str {
    let start = html.find("var BN=").unwrap_or(0);
    let end = html.find("var N=X.length").unwrap_or(html.len());
    &html[start..end.max(start)]
}

pub fn check_robust(build: fn(&str) -> String, demos: &Demos) {
    for (variant_key, json) in demos {
        let cases: Vec<(&str, String)> = vec![
            ("empty", reshaped(json, &|_| 0, false)),
            ("single", reshaped(json, &|_| 1, false)),
            ("pair", reshaped(json, &|_| 2, false)),
            ("doubled", reshaped(json, &|n| n * 2, true)),
        ];
        for (case, input) in cases {
            let html = build(&input);
            let script = scene_script(&html);
            assert!(!script.contains("NaN") && !script.contains("inf"), "{variant_key} with {case} input must not emit non-finite numbers");
            assert!(block_count(&html) <= super::budget::HARD_BLOCKS, "{variant_key} with {case} input must respect the hard cap");
        }
        let mut extreme: serde_json::Value = serde_json::from_str(json).expect("demo json");
        fn blow_up(v: &mut serde_json::Value) {
            match v {
                serde_json::Value::Number(n) => {
                    if let Some(f) = n.as_f64() {
                        *v = serde_json::json!(f * 1e300);
                    }
                }
                serde_json::Value::Array(a) => a.iter_mut().for_each(blow_up),
                _ => {}
            }
        }
        blow_up(&mut extreme);
        let html = build(&extreme.to_string());
        let script = scene_script(&html);
        assert!(!script.contains("NaN") && !script.contains("inf"), "{variant_key} with extreme values must not emit non-finite numbers");
    }
}

pub fn check_big(build: fn(&str) -> String, demos: &Demos, factor: usize) {
    for (variant_key, json) in demos {
        let input = reshaped(json, &|n| n * factor, true);
        let started = std::time::Instant::now();
        let html = build(&input);
        let elapsed = started.elapsed();
        let blocks = block_count(&html);
        assert!(blocks <= super::budget::HARD_BLOCKS, "{variant_key} drew {blocks} blocks for big data");
        assert!(html.len() < 8 * 1024 * 1024, "{variant_key} emitted {} bytes for big data", html.len());
        assert!(elapsed.as_secs() < 30, "{variant_key} took {elapsed:?} on big data");
    }
}

pub fn check_axes(family: &str, variants: usize) {
    let listing = crate::chart_variants();
    let entry = &listing[family];
    assert_eq!(entry["variants"].as_array().map(|v| v.len()), Some(variants));
    for (axis, expected) in [
        ("scene", Scene3DVariant::keys_and_aliases().len()),
        ("orientation3d", Orientation3D::keys_and_aliases().len()),
        ("theme", ChartTheme::keys_and_aliases().len()),
    ] {
        assert_eq!(entry["axes"][axis]["keys"].as_array().map(|k| k.len()), Some(expected), "{axis} axis");
        assert!(entry["axes"][axis]["default"].is_string(), "{axis} default");
    }
}

pub fn write_previews(prefix: &str, build: fn(&str) -> String, demos: &Demos, default_key: &str) {
    let write = |name: String, html: String| {
        std::fs::write(format!("docs/previews/{prefix}-{name}.html"), html).unwrap();
    };
    for (key, json) in demos {
        let html = build(json);
        if *key == default_key {
            std::fs::write(format!("docs/previews/{prefix}.html"), &html).unwrap();
        }
        write(key.to_string(), html);
    }
    let default_json = demos.iter().find(|(key, _)| *key == default_key).map(|(_, json)| json.clone()).unwrap();
    for (plane_key, _) in Orientation3D::keys_and_aliases() {
        write(format!("plane-{plane_key}"), build(&set_field(&default_json, "orientation3d", plane_key)));
    }
    for (scene_key, _) in Scene3DVariant::keys_and_aliases() {
        write(format!("scene-{scene_key}"), build(&set_field(&default_json, "scene", scene_key)));
    }
    for (theme_key, _) in ChartTheme::keys_and_aliases() {
        if *theme_key != ChartTheme::default_key() {
            write(format!("theme-{theme_key}"), build(&set_field(&default_json, "theme", theme_key)));
        }
    }
}
