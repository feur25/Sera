use proc_macro::TokenStream;
use syn::ItemFn;

pub fn sera_register_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = match syn::parse::<ItemFn>(item.clone()) {
        Ok(f) => f,
        Err(_) => return item,
    };
    let _ = func;
    item
}
