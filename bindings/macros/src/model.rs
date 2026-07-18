use crate::util::ModelArgs;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use quote::quote;
use syn::Fields;

pub fn model_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = match syn::parse::<ModelArgs>(attr) {
        Ok(a) => a,
        Err(e) => return TokenStream::from(e.to_compile_error()),
    };
    let struct_item = match syn::parse::<syn::ItemStruct>(item.clone()) {
        Ok(s) => s,
        Err(_) => return item,
    };
    let struct_name = struct_item.ident.to_string();
    let category = &args.category;
    let domain = &args.domain;
    let field_tokens: Vec<TS2> = match &struct_item.fields {
        Fields::Named(named) => named
            .named
            .iter()
            .filter_map(|f| {
                if !matches!(f.vis, syn::Visibility::Public(_)) {
                    return None;
                }
                let name = f.ident.as_ref().map(|i| i.to_string()).unwrap_or_default();
                let ty = &f.ty;
                let ty_str = quote!(#ty).to_string().replace(' ', "");
                Some(quote! {
                    crate::model_registry::ModelFieldInfo { name: #name, ty: #ty_str }
                })
            })
            .collect(),
        _ => vec![],
    };
    let item_ts: TS2 = proc_macro2::TokenStream::from(item);
    let out = quote! {
        #item_ts
        inventory::submit! {
            crate::model_registry::ModelInfo {
                name: #struct_name,
                category: #category,
                domain: #domain,
                fields: &[#(#field_tokens),*],
            }
        }
    };
    TokenStream::from(out)
}
