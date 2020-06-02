extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse_macro_input, Result, Token};
use syn::parse::{Parse, ParseStream};
use quote::quote;

#[derive(Debug)]
struct ProcUseMacroInput {
    ident: syn::Ident,
    brace_token: syn::token::Brace,
    items: Vec<syn::Item>,
}

fn mk_err<T: quote::ToTokens>(t: T) -> Option<(bool, proc_macro2::TokenStream)> {
    Some((
        false,
        syn::Error::new_spanned(t, "expected `builder(each = \"...\")`").to_compile_error(),
    ))
}

fn ident_match(term: &str, ident: syn::Ident) -> syn::Result<()> {
    if ident.to_string().as_str() == term {
	return Ok(());
    }
    
    Err(syn::Error::new(
	ident.span(),
	format!("Error expected ident to say {} and got {}.", term, ident.to_string())
    ))
}

fn has_attr(attr: &str, item: syn::ItemUse) -> syn::Result<()> {
    if item.attrs.len() == 1 {
	let segments = item.attrs[0].path.segments.clone();
	if segments.len() < 1 || segments.len() > 1 {
	    return Ok(());
	}

	ident_match(attr, segments[0].ident.clone())?;
    }

    Ok(())
}

fn expand_mod_all(items: Vec<syn::Item>,) -> proc_macro2::TokenStream {
    for item_outer in items {
	match item_outer {
	    syn::Item::Use(item_use) => {
		
		println!("{:#?}", item_use);
		has_attr("disabled", item_use);
	    },
	    _ => {
		println!("fail");
	    }
	}
	
    }
    proc_macro2::TokenStream::new()
}

fn expand_mod_none(items: Vec<syn::Item>,) -> proc_macro2::TokenStream {
    proc_macro2::TokenStream::new()
}

impl ProcUseMacroInput {
    fn expand(&self) -> proc_macro2::TokenStream  {
	match self.ident.to_string().as_str() {
	    "mod_all" => {
		expand_mod_all(self.items.clone())
	    },
	    "mod_none" => {
		expand_mod_none(self.items.clone())
	    },
	    _ => {
		syn::Error::new(
		    self.ident.span(),
		    "Invalid identifier. Use either mod_all or mod_none."
		).to_compile_error().into()	
	    }
	}
    }
}

impl Parse for ProcUseMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
	let content;

        Ok(ProcUseMacroInput {
	    ident: syn::Ident::parse(input)?,
	    brace_token: syn::braced!(content in input),
	    items: {
		let mut items = Vec::new();
                while !content.is_empty() {
                    items.push(content.parse()?);
                }
                items
	    },
        })
    }
}

// impl Into<proc_macro2::TokenStream> for ProcUseMacroInput {
//     fn into(self) -> proc_macro2::TokenStream {
//         self.expand(self.tt.clone())
//     }
// }

#[proc_macro]
pub fn proc_use_inline(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ProcUseMacroInput);
    let output = input.expand();
    // println!("{:#?}", input);
    // println!("{:#?}", output);
    
    TokenStream::new()
}
