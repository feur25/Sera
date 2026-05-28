use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    FnArg, Ident, ItemFn, LitStr, Pat, Token,
};

#[proc_macro_attribute]
pub fn chart_demo(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn params(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn sera_alias(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

struct BindTarget {
    name: String,
}

struct BindArgs(Vec<BindTarget>);

impl Parse for BindArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let idents = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(BindArgs(
            idents
                .into_iter()
                .map(|i| BindTarget { name: i.to_string() })
                .collect(),
        ))
    }
}

fn extract_arg_idents(func: &ItemFn) -> Vec<Ident> {
    func.sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pt) = arg {
                if let Pat::Ident(pi) = &*pt.pat {
                    return Some(pi.ident.clone());
                }
            }
            None
        })
        .collect()
}

#[proc_macro_attribute]
pub fn sera_bind(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as BindArgs);
    let func = parse_macro_input!(item as ItemFn);

    let mut out: TS2 = quote! { #func };

    let fn_name = &func.sig.ident;
    let fn_inputs = &func.sig.inputs;
    let fn_output = &func.sig.output;
    let arg_idents = extract_arg_idents(&func);

    for target in &args.0 {
        match target.name.as_str() {
            "python" => {
                let wrapper = format_ident!("__sera_py_{}", fn_name);
                let py_name = fn_name.to_string();
                out.extend(quote! {
                    #[cfg(feature = "python")]
                    #[pyo3::pyfunction(name = #py_name)]
                    pub fn #wrapper(#fn_inputs) #fn_output {
                        #fn_name(#(#arg_idents),*)
                    }
                });
            }
            "ffi" => {
                let wrapper = format_ident!("sera_{}", fn_name);
                out.extend(quote! {
                    #[cfg(feature = "ffi")]
                    #[no_mangle]
                    pub extern "C" fn #wrapper(#fn_inputs) #fn_output {
                        #fn_name(#(#arg_idents),*)
                    }
                });
            }
            "wasm" => {
                let wrapper = format_ident!("wasm_{}", fn_name);
                out.extend(quote! {
                    #[cfg(all(feature = "js", target_arch = "wasm32"))]
                    #[wasm_bindgen::prelude::wasm_bindgen]
                    pub fn #wrapper(#fn_inputs) #fn_output {
                        #fn_name(#(#arg_idents),*)
                    }
                });
            }
            _ => {}
        }
    }

    TokenStream::from(out)
}

struct DocParam {
    name: String,
    ty: String,
    en: String,
    fr: String,
}

struct DocArgs {
    en: String,
    fr: String,
    file: String,
    category: String,
    params: Vec<DocParam>,
}

impl Parse for DocArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut en = String::new();
        let mut fr = String::new();
        let mut file = String::new();
        let mut category = String::new();
        let mut params: Vec<DocParam> = Vec::new();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let key_str = key.to_string();
            if key_str == "param" {
                let content;
                syn::parenthesized!(content in input);
                let mut pname = String::new();
                let mut pty = String::new();
                let mut pen = String::new();
                let mut pfr = String::new();
                while !content.is_empty() {
                    let pk: Ident = content.parse()?;
                    content.parse::<Token![=]>()?;
                    let pv: LitStr = content.parse()?;
                    match pk.to_string().as_str() {
                        "name" => pname = pv.value(),
                        "ty" => pty = pv.value(),
                        "en" => pen = pv.value(),
                        "fr" => pfr = pv.value(),
                        _ => {}
                    }
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                }
                params.push(DocParam { name: pname, ty: pty, en: pen, fr: pfr });
            } else {
                input.parse::<Token![=]>()?;
                let val: LitStr = input.parse()?;
                match key_str.as_str() {
                    "en" => en = val.value(),
                    "fr" => fr = val.value(),
                    "file" => file = val.value(),
                    "category" => category = val.value(),
                    _ => {}
                }
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(DocArgs { en, fr, file, category, params })
    }
}

#[proc_macro_attribute]
pub fn sera_doc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as DocArgs);
    let func = parse_macro_input!(item as ItemFn);

    let fn_name = func.sig.ident.to_string();
    let en = &args.en;
    let fr = &args.fr;
    let file = &args.file;
    let category = &args.category;

    let param_tokens: Vec<TS2> = args.params.iter().map(|p| {
        let pname = &p.name;
        let pty = &p.ty;
        let pen = &p.en;
        let pfr = &p.fr;
        quote! {
            crate::doc_registry::ParamDoc { name: #pname, ty: #pty, en: #pen, fr: #pfr }
        }
    }).collect();

    let out = quote! {
        #func

        inventory::submit! {
            crate::doc_registry::FnDoc {
                name: #fn_name,
                category: #category,
                file: #file,
                en: #en,
                fr: #fr,
                params: &[#(#param_tokens),*],
            }
        }
    };

    TokenStream::from(out)
}
