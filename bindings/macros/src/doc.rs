use crate::util::{param_tokens, DocArgs};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use quote::quote;
use syn::ItemFn;

fn returns_chart(func: &ItemFn) -> bool {
    match &func.sig.output {
        syn::ReturnType::Type(_, ty) => quote!(#ty).to_string().replace(' ', "").ends_with("Chart"),
        _ => false,
    }
}

fn build_pymethods_binding(func: &ItemFn, args: &DocArgs) -> TS2 {
    if args.category != "chart_method" {
        return quote! {};
    }
    let has_self = func
        .sig
        .inputs
        .iter()
        .any(|a| matches!(a, syn::FnArg::Receiver(_)));
    if !has_self || !returns_chart(func) {
        return quote! {};
    }
    let method_ident = &func.sig.ident;
    let sig_tokens: Option<TS2> = func
        .attrs
        .iter()
        .find(|a| a.path().is_ident("sera_sig"))
        .and_then(|a| a.parse_args::<TS2>().ok());
    let typed_params: Vec<&syn::PatType> = func
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
        return quote! {};
    }
    let mut names = vec![method_ident.to_string()];
    names.extend(args.aliases.iter().cloned());
    let mut blocks: Vec<TS2> = Vec::new();
    for (i, name) in names.iter().enumerate() {
        let fn_ident = syn::Ident::new(&format!("__sera_py_{}_{}", method_ident, i), method_ident.span());
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
    quote! {
        #[cfg(feature = "python")]
        const _: () = {
            #[pyo3::pymethods]
            impl Chart {
                #(#blocks)*
            }
        };
    }
}

pub fn sera_doc_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    match syn::parse::<ItemFn>(item.clone()) {
        Ok(func) => {
            let args = match syn::parse::<DocArgs>(attr) {
                Ok(a) => a,
                Err(e) => return TokenStream::from(e.to_compile_error()),
            };
            let fn_name = if args.name.is_empty() {
                func.sig.ident.to_string()
            } else {
                args.name.clone()
            };
            let en = &args.en;
            let fr = &args.fr;
            let file = &args.file;
            let category = &args.category;
            let ptokens = param_tokens(&args.params);
            let atokens: Vec<&str> = args.aliases.iter().map(|s| s.as_str()).collect();
            let pybind = build_pymethods_binding(&func, &args);
            let out = quote! {
                #func
                inventory::submit! {
                    crate::doc_registry::FnDoc {
                        name: #fn_name,
                        category: #category,
                        file: #file,
                        en: #en,
                        fr: #fr,
                        params: &[#(#ptokens),*],
                        aliases: &[#(#atokens),*],
                    }
                }
                #pybind
            };
            TokenStream::from(out)
        }
        Err(_) => item,
    }
}

fn returns_self_ty(sig: &syn::Signature, self_ty_str: &str, result_ty_str: &str) -> bool {
    match &sig.output {
        syn::ReturnType::Type(_, ty) => {
            let s = quote!(#ty).to_string().replace(' ', "");
            s == self_ty_str || s == result_ty_str
        }
        _ => false,
    }
}

pub fn sera_doc_impl_block(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut impl_block = match syn::parse::<syn::ItemImpl>(item) {
        Ok(i) => i,
        Err(e) => return TokenStream::from(e.to_compile_error()),
    };
    let self_ty = impl_block.self_ty.clone();
    let self_ty_str = quote!(#self_ty).to_string().replace(' ', "");
    let result_ty_str = format!("PyResult<{}>", self_ty_str);
    let mut doc_submits: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut alias_blocks: Vec<proc_macro2::TokenStream> = Vec::new();
    for impl_item in &mut impl_block.items {
        if let syn::ImplItem::Fn(ref mut method) = impl_item {
            let method_name = method.sig.ident.to_string();
            if let Some(idx) = method
                .attrs
                .iter()
                .position(|a| a.path().is_ident("sera_doc"))
            {
                let attr = method.attrs.remove(idx);
                if let Ok(args) = attr.parse_args::<DocArgs>() {
                    let reg_name = if args.name.is_empty() {
                        method_name
                    } else {
                        args.name.clone()
                    };
                    let en = &args.en;
                    let fr = &args.fr;
                    let file = &args.file;
                    let category = &args.category;
                    let ptokens = param_tokens(&args.params);
                    let atokens: Vec<&str> = args.aliases.iter().map(|s| s.as_str()).collect();
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
                    let has_self = method
                        .sig
                        .inputs
                        .iter()
                        .any(|a| matches!(a, syn::FnArg::Receiver(_)));
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
                    let eligible = !args.aliases.is_empty()
                        && has_self
                        && param_names.len() == typed_params.len()
                        && returns_self_ty(&method.sig, &self_ty_str, &result_ty_str);
                    if eligible {
                        let method_ident = method.sig.ident.clone();
                        let ret_ty = method.sig.output.clone();
                        let decls: Vec<TS2> = typed_params.iter().map(|pt| quote!(#pt)).collect();
                        let sig_attr = method
                            .attrs
                            .iter()
                            .find(|a| a.path().is_ident("pyo3"))
                            .map(|a| quote!(#a));
                        for (i, alias) in args.aliases.iter().enumerate() {
                            let fn_ident = syn::Ident::new(
                                &format!("__sera_alias_{}_{}", method_ident, i),
                                method_ident.span(),
                            );
                            alias_blocks.push(quote! {
                                #sig_attr
                                #[pyo3(name = #alias)]
                                fn #fn_ident(&self, #(#decls),*) #ret_ty {
                                    self.#method_ident(#(#param_names),*)
                                }
                            });
                        }
                    }
                }
            }
        }
    }
    let alias_impl = if alias_blocks.is_empty() {
        quote! {}
    } else {
        quote! {
            #[cfg(feature = "python")]
            const _: () = {
                #[pyo3::pymethods]
                impl #self_ty {
                    #(#alias_blocks)*
                }
            };
        }
    };
    let out = quote! {
        #impl_block
        #(#doc_submits)*
        #alias_impl
    };
    TokenStream::from(out)
}
