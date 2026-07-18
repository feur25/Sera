use proc_macro::TokenStream;

mod bind;
mod class;
mod doc;
mod model;
mod register;
mod util;

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
