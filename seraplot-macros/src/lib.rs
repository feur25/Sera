use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use quote::quote;

mod bind;
mod class;
mod doc;
mod model;
mod register;
mod util;

struct ChartDemoArgs {
    kwargs: syn::LitStr,
    media: Option<syn::LitStr>,
}

impl syn::parse::Parse for ChartDemoArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kwargs: syn::LitStr = input.parse()?;
        let mut media = None;
        if input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?;
            if !input.is_empty() {
                let ident: syn::Ident = input.parse()?;
                if ident != "media" {
                    return Err(syn::Error::new(ident.span(), "expected `media`"));
                }
                input.parse::<syn::Token![=]>()?;
                media = Some(input.parse()?);
            }
        }
        Ok(ChartDemoArgs { kwargs, media })
    }
}

#[proc_macro_attribute]
pub fn chart_demo(attr: TokenStream, item: TokenStream) -> TokenStream {
    let Ok(args) = syn::parse::<ChartDemoArgs>(attr) else {
        return item;
    };
    let Ok(func) = syn::parse::<syn::ItemFn>(item.clone()) else {
        return item;
    };
    let fn_name = func.sig.ident.to_string();
    let kwargs = &args.kwargs;
    let media = match &args.media {
        Some(lit) => quote!(#lit),
        None => quote!(""),
    };
    let item2: TS2 = item.into();
    let out = quote! {
        #item2
        inventory::submit! {
            crate::plot::chart_demo_registry::ChartDemoEntry {
                file: file!(),
                fn_name: #fn_name,
                kwargs: #kwargs,
                media: #media,
            }
        }
    };
    TokenStream::from(out)
}

#[proc_macro_attribute]
pub fn params(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn sera_alias(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn sera_builder(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn ml_doc(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn sera_sig(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn sera_bind(attr: TokenStream, item: TokenStream) -> TokenStream {
    bind::sera_bind_fn(attr, item)
}

#[proc_macro_attribute]
pub fn sera_binding(attr: TokenStream, item: TokenStream) -> TokenStream {
    bind::sera_bind_fn(attr, item)
}

#[proc_macro_attribute]
pub fn sera_doc(attr: TokenStream, item: TokenStream) -> TokenStream {
    doc::sera_doc_fn(attr, item)
}

#[proc_macro_attribute]
pub fn sera_doc_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    doc::sera_doc_impl_block(attr, item)
}

#[proc_macro_attribute]
pub fn sera_class(attr: TokenStream, item: TokenStream) -> TokenStream {
    class::sera_class_fn(attr, item)
}

#[proc_macro_attribute]
pub fn sera_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    class::sera_impl_fn(attr, item)
}

#[proc_macro_attribute]
pub fn model(attr: TokenStream, item: TokenStream) -> TokenStream {
    model::model_fn(attr, item)
}

#[proc_macro_attribute]
pub fn sera_register(attr: TokenStream, item: TokenStream) -> TokenStream {
    register::sera_register_fn(attr, item)
}

#[proc_macro_attribute]
pub fn sera_python_skip(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
