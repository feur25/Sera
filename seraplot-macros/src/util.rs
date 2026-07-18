use proc_macro2::TokenStream as TS2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    FnArg, Ident, ItemFn, LitStr, Pat, Token,
};

pub struct BindTarget {
    pub name: String,
}

pub struct BindArgs(pub Vec<BindTarget>);

impl Parse for BindArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let idents = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(BindArgs(
            idents
                .into_iter()
                .map(|i| BindTarget {
                    name: i.to_string(),
                })
                .collect(),
        ))
    }
}

pub fn extract_arg_idents(func: &ItemFn) -> Vec<Ident> {
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

pub struct DocParam {
    pub name: String,
    pub ty: String,
    pub en: String,
    pub fr: String,
}

pub struct DocArgs {
    pub name: String,
    pub en: String,
    pub fr: String,
    pub file: String,
    pub category: String,
    pub params: Vec<DocParam>,
    pub aliases: Vec<String>,
}

impl Parse for DocArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = String::new();
        let mut en = String::new();
        let mut fr = String::new();
        let mut file = String::new();
        let mut category = String::new();
        let mut params: Vec<DocParam> = Vec::new();
        let mut aliases: Vec<String> = Vec::new();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let key_str = key.to_string();
            if key_str == "aliases" {
                let content;
                syn::parenthesized!(content in input);
                let lits = Punctuated::<LitStr, Token![,]>::parse_terminated(&content)?;
                aliases.extend(lits.into_iter().map(|l| l.value()));
            } else if key_str == "param" {
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
                params.push(DocParam {
                    name: pname,
                    ty: pty,
                    en: pen,
                    fr: pfr,
                });
            } else {
                input.parse::<Token![=]>()?;
                let val: LitStr = input.parse()?;
                match key_str.as_str() {
                    "name" => name = val.value(),
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

        Ok(DocArgs {
            name,
            en,
            fr,
            file,
            category,
            params,
            aliases,
        })
    }
}

pub fn param_tokens(params: &[DocParam]) -> Vec<TS2> {
    params
        .iter()
        .map(|p| {
            let pname = &p.name;
            let pty = &p.ty;
            let pen = &p.en;
            let pfr = &p.fr;
            quote! {
                crate::doc_registry::ParamDoc { name: #pname, ty: #pty, en: #pen, fr: #pfr }
            }
        })
        .collect()
}

pub struct SeraImplArgs {
    pub builders: Vec<String>,
}

impl Parse for SeraImplArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(SeraImplArgs {
                builders: Vec::new(),
            });
        }
        let idents = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(SeraImplArgs {
            builders: idents.into_iter().map(|i| i.to_string()).collect(),
        })
    }
}

pub struct ModelArgs {
    pub category: String,
    pub domain: String,
}

impl Parse for ModelArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut category = String::new();
        let mut domain = String::new();
        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let val: LitStr = input.parse()?;
            match key.to_string().as_str() {
                "category" => category = val.value(),
                "domain" => domain = val.value(),
                _ => {}
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(ModelArgs { category, domain })
    }
}
