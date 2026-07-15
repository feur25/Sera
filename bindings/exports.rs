macro_rules! register_chart_entry {
    ($fn:ident, $_js:literal) => {
        inventory::submit! {
            crate::bindings::fn_registry::FnEntry {
                name: stringify!($fn),
                input: crate::bindings::fn_registry::InputKind::Json,
                output: crate::bindings::fn_registry::OutputKind::Html,
                invoke: crate::bindings::commands::charts::$fn,
            }
        }
    };
}
crate::for_each_json_chart_fn!(register_chart_entry);

macro_rules! register_html_util_entry {
    ($fn:ident, $_js:literal) => {
        inventory::submit! {
            crate::bindings::fn_registry::FnEntry {
                name: stringify!($fn),
                input: crate::bindings::fn_registry::InputKind::Json,
                output: crate::bindings::fn_registry::OutputKind::Html,
                invoke: crate::bindings::commands::charts::$fn,
            }
        }
    };
}
crate::for_each_html_util_fn!(register_html_util_entry);

#[allow(unused_macros)]
macro_rules! register_ml_entry {
    ($fn:ident, $_js:literal) => {
        inventory::submit! {
            crate::bindings::fn_registry::FnEntry {
                name: stringify!($fn),
                input: crate::bindings::fn_registry::InputKind::Json,
                output: crate::bindings::fn_registry::OutputKind::Json,
                invoke: crate::bindings::commands::ml::$fn,
            }
        }
    };
}
crate::for_each_ml_oneshot_fn!(register_ml_entry);
