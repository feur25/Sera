#[macro_export]
macro_rules! impl_wasm_bindings {
    () => {
        use wasm_bindgen::prelude::*;
        macro_rules! impl_wasm_chart {
            ($fn:ident, $js:literal) => {
                #[wasm_bindgen(js_name = $js)]
                pub fn $fn(input: &str) -> String {
                    crate::bindings::commands::charts::$fn(input)
                }
            };
        }
        crate::for_each_json_chart_fn!(impl_wasm_chart);

        macro_rules! impl_wasm_ml {
            ($fn:ident, $js:literal) => {
                #[wasm_bindgen(js_name = $js)]
                pub fn $fn(input: &str) -> String {
                    crate::bindings::commands::ml::$fn(input)
                }
            };
        }
        crate::for_each_ml_oneshot_fn!(impl_wasm_ml);

        macro_rules! impl_wasm_util {
            ($fn:ident, $js:literal) => {
                #[wasm_bindgen(js_name = $js)]
                pub fn $fn(input: &str) -> String {
                    crate::bindings::commands::charts::$fn(input)
                }
            };
        }
        crate::for_each_util_fn!(impl_wasm_util);

        #[wasm_bindgen(js_name = "demo")]
        pub fn demo(input: &str) -> String {
            #[derive(serde::Deserialize, Default)]
            struct In { family: Option<String>, variant: Option<String> }
            let i: In = serde_json::from_str(input).unwrap_or_default();
            let f = i.family.unwrap_or_default();
            let v = i.variant.unwrap_or_else(|| "basic".to_string());
            crate::demo_snippet(&f, &v).unwrap_or_default()
        }

        #[wasm_bindgen(js_name = "chartAliases")]
        pub fn chart_aliases() -> String {
            let mut obj = serde_json::Map::new();
            for (alias, target) in crate::bindings::registry_macro::CHART_ALIASES {
                let camel: String = target.split('_').enumerate().map(|(i, s)| {
                    if i == 0 { s.to_string() }
                    else { let mut c = s.chars(); c.next().map(|ch| ch.to_uppercase().collect::<String>() + c.as_str()).unwrap_or_default() }
                }).collect();
                obj.insert(alias.to_string(), serde_json::Value::String(camel));
            }
            serde_json::to_string(&serde_json::Value::Object(obj)).unwrap_or_default()
        }
    };
}
