use crate::util::{extract_arg_idents, BindArgs, BindTarget};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::{FnArg, Pat, Token};

pub fn sera_bind_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(attr as BindArgs);
    let mut func = syn::parse_macro_input!(item as syn::ItemFn);

    let mut sig_tokens: Option<TS2> = None;
    func.attrs.retain(|a| {
        if a.path().is_ident("sera_sig") {
            if let syn::Meta::List(list) = &a.meta {
                sig_tokens = Some(list.tokens.clone());
            }
            false
        } else {
            true
        }
    });
    let mut out: TS2 = quote! { #func };
    let fn_name = &func.sig.ident;
    let fn_inputs = &func.sig.inputs;
    let fn_output = &func.sig.output;
    let arg_idents = extract_arg_idents(&func);
    let mut serde_mode = false;
    let mut chart_output = false;
    let mut json_input = false;
    let mut explicit_targets: Vec<BindTarget> = Vec::new();
    for t in &args.0 {
        match t.name.as_str() {
            "serde" => serde_mode = true,
            "chart" => chart_output = true,
            "json" => json_input = true,
            "chart_input" | "ok" => {}
            _ => explicit_targets.push(BindTarget {
                name: t.name.clone(),
            }),
        }
    }
    let all_targets = vec![
        BindTarget {
            name: "ffi".to_string(),
        },
        BindTarget {
            name: "wasm".to_string(),
        },
    ];
    let targets: &Vec<BindTarget> = if explicit_targets.is_empty() {
        &all_targets
    } else {
        &explicit_targets
    };

    for target in targets {
        match target.name.as_str() {
            "python" => {}
            "ffi" => {
                let wrapper = format_ident!("sera_{}", fn_name);
                if serde_mode {
                    out.extend(quote! {
                        #[cfg(feature = "ffi")]
                        #[allow(improper_ctypes_definitions)]
                        #[no_mangle]
                        pub extern "C" fn #wrapper(#fn_inputs) -> *mut std::os::raw::c_char {
                            let __v = #fn_name(#(#arg_idents),*);
                            match serde_json::to_string(&__v) {
                                Ok(s) => std::ffi::CString::new(s)
                                    .map(|c| c.into_raw())
                                    .unwrap_or(std::ptr::null_mut()),
                                Err(_) => std::ptr::null_mut(),
                            }
                        }
                    });
                } else {
                    out.extend(quote! {
                        #[cfg(feature = "ffi")]
                        #[allow(improper_ctypes_definitions)]
                        #[no_mangle]
                        pub extern "C" fn #wrapper(#fn_inputs) #fn_output {
                            #fn_name(#(#arg_idents),*)
                        }
                    });
                }
            }
            "wasm" => {
                let wrapper = format_ident!("wasm_{}", fn_name);
                let mut wasm_inputs: Punctuated<FnArg, Token![,]> = Punctuated::new();
                let mut call_args: Vec<TS2> = Vec::new();
                for arg in fn_inputs.iter() {
                    if let FnArg::Typed(pt) = arg {
                        let ident = if let Pat::Ident(pi) = &*pt.pat {
                            pi.ident.clone()
                        } else {
                            wasm_inputs.push(arg.clone());
                            continue;
                        };
                        let ty = &*pt.ty;
                        let ty_string = quote!(#ty).to_string().replace(' ', "");
                        if ty_string == "&str" {
                            let mut new_pt = pt.clone();
                            new_pt.ty = Box::new(syn::parse_quote!(String));
                            wasm_inputs.push(FnArg::Typed(new_pt));
                            call_args.push(quote! { #ident.as_str() });
                        } else if ty_string == "Option<&str>" {
                            let mut new_pt = pt.clone();
                            new_pt.ty = Box::new(syn::parse_quote!(Option<String>));
                            wasm_inputs.push(FnArg::Typed(new_pt));
                            call_args.push(quote! { #ident.as_deref() });
                        } else {
                            wasm_inputs.push(arg.clone());
                            call_args.push(quote! { #ident });
                        }
                    } else {
                        wasm_inputs.push(arg.clone());
                    }
                }
                out.extend(quote! {
                    #[cfg(all(feature = "js", target_arch = "wasm32"))]
                    #[wasm_bindgen::prelude::wasm_bindgen]
                    pub fn #wrapper(#wasm_inputs) -> wasm_bindgen::JsValue {
                        let __v = #fn_name(#(#call_args),*);
                        serde_wasm_bindgen::to_value(&__v).unwrap_or(wasm_bindgen::JsValue::NULL)
                    }
                });
            }
            _ => {}
        }
    }

    if json_input {
        let output_kind = if chart_output {
            quote! { crate::bindings::fn_registry::OutputKind::Html }
        } else {
            quote! { crate::bindings::fn_registry::OutputKind::Json }
        };
        out.extend(quote! {
            inventory::submit! {
                crate::bindings::fn_registry::FnEntry {
                    name: stringify!(#fn_name),
                    input: crate::bindings::fn_registry::InputKind::Json,
                    output: #output_kind,
                    invoke: #fn_name,
                }
            }
        });
    }

    TokenStream::from(out)
}
