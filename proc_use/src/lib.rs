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

fn ident_match(term: String, ident: syn::Ident) -> String {
    if ident.to_string() == term {
	return term;
    }
    "".to_string()
}

fn has_attr(attr: String, item: syn::ItemUse) -> bool {
    if item.attrs.len() == 1 {
	let segments = item.attrs[0].path.segments.clone();
	if segments.len() < 1 || segments.len() > 1 {
	    return false;
	}
	
	println!("{:#?}", item.attrs[0].path.segments);
	return true;
    }

    false
}

fn expand_mod_all(items: Vec<syn::Item>,) -> proc_macro2::TokenStream {
    for item_outer in items {
	match item_outer {
	    syn::Item::Use(item_use) => {
		
		// println!("{:#?}", item_use);
		has_attr("disabled".to_string(), item_use);
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
