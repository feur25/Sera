use crate::util::{param_tokens, DocArgs, SeraImplArgs};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use quote::quote;
use std::collections::HashMap;
use syn::{parse::Parser, parse_macro_input, ItemImpl};

fn parse_sig_defaults(sig_tokens: &TS2) -> HashMap<String, TS2> {
    let mut map = HashMap::new();
    let parser = syn::punctuated::Punctuated::<syn::ExprAssign, syn::Token![,]>::parse_terminated;
    if let Ok(list) = parser.parse2(sig_tokens.clone()) {
        for ea in list {
            if let syn::Expr::Path(p) = *ea.left {
                if let Some(ident) = p.path.get_ident() {
                    let right = *ea.right;
                    map.insert(ident.to_string(), quote! { #right });
                }
            }
        }
    }
    map
}

fn js_method_blocks(
    method: &syn::ImplItemFn,
    doc_args: &DocArgs,
    reg_name: &str,
) -> Vec<TS2> {
    if doc_args.category != "chart_method" {
        return Vec::new();
    }
    let has_self = method
        .sig
        .inputs
        .iter()
        .any(|a| matches!(a, syn::FnArg::Receiver(_)));
    if !has_self || !returns_chart(&method.sig) {
        return Vec::new();
    }
    let method_ident = &method.sig.ident;
    let sig_tokens: Option<TS2> = method
        .attrs
        .iter()
        .find(|a| a.path().is_ident("sera_sig"))
        .and_then(|a| a.parse_args::<TS2>().ok());
    let defaults = sig_tokens
        .as_ref()
        .map(parse_sig_defaults)
        .unwrap_or_default();
    let typed_params: Vec<&syn::PatType> = method
        .sig
        .inputs
        .iter()
        .filter_map(|a| match a {
            syn::FnArg::Typed(pt) => Some(pt),
            _ => None,
        })
        .collect();
    let mut decls: Vec<TS2> = Vec::new();
    let mut call_args: Vec<TS2> = Vec::new();
    for pt in &typed_params {
        let name = match &*pt.pat {
            syn::Pat::Ident(pi) => pi.ident.to_string(),
            _ => return Vec::new(),
        };
        let ty_str = quote! { #pt }
            .to_string()
            .split(':')
            .nth(1)
            .unwrap_or_default()
            .replace(' ', "");
        let var = syn::Ident::new(&format!("__v_{}", name), method_ident.span());
        let default = defaults.get(&name).cloned();
        match ty_str.as_str() {
            "f64" => {
                let d = default.unwrap_or_else(|| quote! { 0.0 });
                decls.push(quote! {
                    let #var: f64 = __args.get(#name).and_then(|v| v.as_f64()).unwrap_or(#d);
                });
                call_args.push(quote! { #var });
            }
            "i32" => {
                let d = default.unwrap_or_else(|| quote! { 0 });
                decls.push(quote! {
                    let #var: i32 = __args.get(#name).and_then(|v| v.as_i64()).map(|x| x as i32).unwrap_or(#d);
                });
                call_args.push(quote! { #var });
            }
            "u32" => {
                let d = default.unwrap_or_else(|| quote! { 0 });
                decls.push(quote! {
                    let #var: u32 = __args.get(#name).and_then(|v| v.as_u64()).map(|x| x as u32).unwrap_or(#d);
                });
                call_args.push(quote! { #var });
            }
            "usize" => {
                let d = default.unwrap_or_else(|| quote! { 0 });
                decls.push(quote! {
                    let #var: usize = __args.get(#name).and_then(|v| v.as_u64()).map(|x| x as usize).unwrap_or(#d);
                });
                call_args.push(quote! { #var });
            }
            "Option<f64>" => {
                let d = default.unwrap_or_else(|| quote! { None });
                decls.push(quote! {
                    let #var: Option<f64> = __args.get(#name).and_then(|v| v.as_f64()).or(#d);
                });
                call_args.push(quote! { #var });
            }
            "Option<i32>" => {
                let d = default.unwrap_or_else(|| quote! { None });
                decls.push(quote! {
                    let #var: Option<i32> = __args.get(#name).and_then(|v| v.as_i64()).map(|x| x as i32).or(#d);
                });
                call_args.push(quote! { #var });
            }
            "Option<Vec<usize>>" => {
                decls.push(quote! {
                    let #var: Option<Vec<usize>> = __args.get(#name).and_then(|v| v.as_array()).map(|a| a.iter().filter_map(|x| x.as_u64().map(|y| y as usize)).collect());
                });
                call_args.push(quote! { #var });
            }
            "&str" => {
                let d = default.unwrap_or_else(|| quote! { "" });
                decls.push(quote! {
                    let #var: String = __args.get(#name).and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_else(|| #d.to_string());
                });
                call_args.push(quote! { #var.as_str() });
            }
            "Option<&str>" => {
                decls.push(quote! {
                    let #var: Option<String> = __args.get(#name).and_then(|v| v.as_str()).map(|s| s.to_string());
                });
                call_args.push(quote! { #var.as_deref() });
            }
            _ => return Vec::new(),
        }
    }
    let mut names = vec![reg_name.to_string()];
    names.extend(doc_args.aliases.iter().cloned());
    names
        .iter()
        .map(|name| {
            quote! {
                #[cfg(feature = "js")]
                const _: () = {
                    inventory::submit! {
                        crate::bindings::method_registry::MethodEntry {
                            name: #name,
                            apply: |c: &crate::Chart, args_json: &str| -> crate::Chart {
                                let __args: serde_json::Value = serde_json::from_str(args_json).unwrap_or(serde_json::Value::Null);
                                #(#decls)*
                                c.#method_ident(#(#call_args),*)
                            },
                        }
                    }
                };
            }
        })
        .collect()
}

pub fn sera_class_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_ts: TS2 = item.into();
    TokenStream::from(quote! {
        #[cfg_attr(all(feature = "js", target_arch = "wasm32"), wasm_bindgen::prelude::wasm_bindgen)]
        #item_ts
    })
}

#[allow(dead_code)]
pub fn gen_py_protocol(html_display: bool, pickle: bool, export: bool) -> (TS2, TS2) {
    let mut items = TS2::new();
    let mut docs = TS2::new();
    if html_display {
        items.extend(quote! {
            #[new]
            #[pyo3(signature = (html = String::new()))]
            fn py_new(html: String) -> Self { Self { html, doc_str: "" } }
            #[getter]
            fn html(&self) -> &str { &self.html }
            fn doc(&self) -> &'static str { self.doc_str }
            fn _repr_html_(&self) -> String { self.chart_iframe() }
            #[pyo3(signature = (**kwargs))]
            fn _ipython_display_(&self, py: pyo3::prelude::Python<'_>, kwargs: ::std::option::Option<&pyo3::Bound<'_, pyo3::types::PyDict>>) -> pyo3::prelude::PyResult<()> {
                let _ = kwargs;
                let ipython = py.import_bound("IPython.display")?;
                let html_cls = ipython.getattr("HTML")?;
                let display_fn = ipython.getattr("display")?;
                let html_obj = html_cls.call1((self.chart_iframe().as_str(),))?;
                display_fn.call1((html_obj,))?;
                Ok(())
            }
            fn show(&self, py: pyo3::prelude::Python<'_>) -> pyo3::prelude::PyResult<()> {
                let ipython = py.import_bound("IPython.display")?;
                let html_cls = ipython.getattr("HTML")?;
                let display_fn = ipython.getattr("display")?;
                let html_obj = html_cls.call1((self.chart_iframe().as_str(),))?;
                display_fn.call1((html_obj,))?;
                Ok(())
            }
            fn __str__(&self) -> &str { &self.html }
            fn __repr__(&self) -> String { format!("SeraPlot.Chart({} bytes)", self.html.len()) }
            fn __len__(&self) -> usize { self.html.len() }
            fn __bool__(&self) -> bool { !self.html.is_empty() }
        });
        docs.extend(quote! {
            inventory::submit! {
                crate::doc_registry::FnDoc {
                    name: "show",
                    category: "chart_method",
                    file: "charts/chart.md",
                    en: "Renders the chart inline in a Jupyter notebook using IPython display.",
                    fr: "Affiche le graphique en ligne dans un notebook Jupyter via IPython display.",
                    params: &[],
                    aliases: &[],
                }
            }
        });
    }
    if pickle {
        items.extend(quote! {
            fn __getstate__(&self) -> String { self.html.clone() }
            fn __setstate__(&mut self, state: String) -> pyo3::prelude::PyResult<()> {
                self.html = state;
                Ok(())
            }
        });
    }
    if export {
        items.extend(quote! {
            #[pyo3(signature = (path, scale = 2.0))]
            fn export_png(&self, py: pyo3::prelude::Python<'_>, path: &str, scale: f64) -> pyo3::prelude::PyResult<()> {
                let h = &self.html;
                let start = h.find("<svg").ok_or_else(|| pyo3::exceptions::PyValueError::new_err("No SVG in chart"))?;
                let end = h.rfind("</svg>").ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Malformed SVG"))? + 6;
                let svg = h[start..end].to_string();
                match py.import_bound("cairosvg") {
                    Ok(m) => {
                        let kwargs = pyo3::types::PyDict::new_bound(py);
                        kwargs.set_item("write_to", path)?;
                        kwargs.set_item("scale", scale)?;
                        m.call_method("svg2png", (svg.as_str(),), Some(&kwargs))?;
                        Ok(())
                    }
                    Err(_) => Err(pyo3::exceptions::PyImportError::new_err("PNG export requires cairosvg: pip install cairosvg"))
                }
            }
        });
        docs.extend(quote! {
            inventory::submit! {
                crate::doc_registry::FnDoc {
                    name: "export_png",
                    category: "chart_method",
                    file: "charts/export.md",
                    en: "Exports the chart as a PNG image via cairosvg. Requires: pip install cairosvg.",
                    fr: "Exporte le graphique en image PNG via cairosvg. Nécessite: pip install cairosvg.",
                    params: &[
                        crate::doc_registry::ParamDoc { name: "path", ty: "str", en: "Destination .png file path.", fr: "Chemin du fichier .png de destination." },
                        crate::doc_registry::ParamDoc { name: "scale", ty: "float", en: "Resolution scale factor (default 2.0 for 2x resolution).", fr: "Facteur d'échelle de résolution (défaut 2.0 pour une résolution 2x)." },
                    ],
                    aliases: &[],
                }
            }
        });
    }
    (items, docs)
}

fn returns_chart(sig: &syn::Signature) -> bool {
    match &sig.output {
        syn::ReturnType::Type(_, ty) => quote!(#ty).to_string().replace(' ', "").ends_with("Chart"),
        _ => false,
    }
}

fn build_pymethods_blocks(method: &syn::ImplItemFn, doc_args: &DocArgs, reg_name: &str) -> Vec<TS2> {
    if doc_args.category != "chart_method" {
        return Vec::new();
    }
    let has_self = method
        .sig
        .inputs
        .iter()
        .any(|a| matches!(a, syn::FnArg::Receiver(_)));
    if !has_self || !returns_chart(&method.sig) {
        return Vec::new();
    }
    let method_ident = &method.sig.ident;
    let sig_tokens: Option<TS2> = method
        .attrs
        .iter()
        .find(|a| a.path().is_ident("sera_sig"))
        .and_then(|a| a.parse_args::<TS2>().ok());
    let typed_params: Vec<&syn::PatType> = method
        .sig
        .inputs
        .iter()
        .filter_map(|a| match a {
            syn::FnArg::Typed(pt) => Some(pt),
            _ => None,
        })
        .collect();
    let param_names: Vec<&syn::Ident> = typed_params
        .iter()
        .filter_map(|pt| match &*pt.pat {
            syn::Pat::Ident(pi) => Some(&pi.ident),
            _ => None,
        })
        .collect();
    if param_names.len() != typed_params.len() {
        return Vec::new();
    }
    let mut names = vec![reg_name.to_string()];
    names.extend(doc_args.aliases.iter().cloned());
    let mut blocks: Vec<TS2> = Vec::new();
    for (i, name) in names.iter().enumerate() {
        let fn_ident = syn::Ident::new(
            &format!("__sera_py_{}_{}", method_ident, i),
            method_ident.span(),
        );
        let pyo3_attr = if let Some(ref st) = sig_tokens {
            quote! { #[pyo3(name = #name, signature = (#st))] }
        } else {
            quote! { #[pyo3(name = #name)] }
        };
        let decls = typed_params.iter().map(|pt| quote!(#pt));
        blocks.push(quote! {
            #pyo3_attr
            fn #fn_ident(&self, #(#decls),*) -> Chart {
                self.#method_ident(#(#param_names),*)
            }
        });
    }
    blocks
}

pub fn sera_impl_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as SeraImplArgs);
    let mut impl_block = parse_macro_input!(item as ItemImpl);
    let ty = impl_block.self_ty.clone();
    let (impl_generics, _, where_clause) = impl_block.generics.split_for_impl();
    let has_html_display = args.builders.iter().any(|b| b == "html_display");
    let has_pickle = args.builders.iter().any(|b| b == "pickle");
    let has_export = args.builders.iter().any(|b| b == "export");
    let mut doc_submits: Vec<TS2> = Vec::new();
    let mut py_blocks: Vec<TS2> = Vec::new();
    let mut js_blocks: Vec<TS2> = Vec::new();
    for impl_item in &mut impl_block.items {
        if let syn::ImplItem::Fn(ref mut method) = impl_item {
            let method_name = method.sig.ident.to_string();
            if let Some(idx) = method
                .attrs
                .iter()
                .position(|a| a.path().is_ident("sera_doc"))
            {
                let attr = method.attrs.remove(idx);
                if let Ok(doc_args) = attr.parse_args::<DocArgs>() {
                    let reg_name = if doc_args.name.is_empty() {
                        method_name
                    } else {
                        doc_args.name.clone()
                    };
                    let en = &doc_args.en;
                    let fr = &doc_args.fr;
                    let file = &doc_args.file;
                    let category = &doc_args.category;
                    let ptokens = param_tokens(&doc_args.params);
                    let atokens: Vec<&str> = doc_args.aliases.iter().map(|s| s.as_str()).collect();
                    doc_submits.push(quote! {
                        inventory::submit! {
                            crate::doc_registry::FnDoc {
                                name: #reg_name,
                                category: #category,
                                file: #file,
                                en: #en,
                                fr: #fr,
                                params: &[#(#ptokens),*],
                                aliases: &[#(#atokens),*],
                            }
                        }
                    });
                    py_blocks.extend(build_pymethods_blocks(method, &doc_args, &reg_name));
                    js_blocks.extend(js_method_blocks(method, &doc_args, &reg_name));
                }
            }
        }
    }
    let js_impl = quote! { #(#js_blocks)* };
    let py_impl = if py_blocks.is_empty() {
        quote! {}
    } else {
        quote! {
            #[cfg(feature = "python")]
            const _: () = {
                #[pyo3::pymethods]
                impl Chart {
                    #(#py_blocks)*
                }
            };
        }
    };
    let base_items: Vec<TS2> = impl_block
        .items
        .iter()
        .filter_map(|it| {
            if let syn::ImplItem::Fn(m) = it {
                let mut m2 = m.clone();
                m2.attrs.retain(|a| {
                    let p = a.path();
                    !p.is_ident("pyo3")
                        && !p.is_ident("sera_wasm_skip")
                        && !p.is_ident("sera_python_skip")
                        && !p.is_ident("sera_sig")
                });
                Some(quote! { #m2 })
            } else {
                Some(quote! { #it })
            }
        })
        .collect();
    let wasm_items: Vec<TS2> = impl_block
        .items
        .iter()
        .filter_map(|it| {
            if let syn::ImplItem::Fn(m) = it {
                if m.attrs.iter().any(|a| a.path().is_ident("sera_wasm_skip")) {
                    return None;
                }
                let mut m2 = m.clone();
                m2.attrs.retain(|a| {
                    let p = a.path();
                    !p.is_ident("pyo3")
                        && !p.is_ident("sera_wasm_skip")
                        && !p.is_ident("sera_python_skip")
                        && !p.is_ident("sera_sig")
                });
                Some(quote! { #m2 })
            } else {
                Some(quote! { #it })
            }
        })
        .collect();
    let _ = wasm_items;
    let _ = (has_html_display, has_pickle, has_export);
    TokenStream::from(quote! {
        impl #impl_generics #ty #where_clause {
            #(#base_items)*
        }
        #(#doc_submits)*
        #py_impl
        #js_impl
    })
}
