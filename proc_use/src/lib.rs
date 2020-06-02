extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse_macro_input, Result, Token};
use syn::parse::{Parse, ParseStream};
use quote::quote;

#[derive(Debug)]
struct ProcUseMacroInput {
    brace_token: syn::token::Brace,
    items: Vec<syn::Item>,
}

fn mk_err<T: quote::ToTokens>(t: T, msg: String) -> syn::Error {
    syn::Error::new_spanned(t, msg)
}

fn ident_match(term: &str, ident: syn::Ident) -> syn::Result<bool> {
    if ident.to_string().as_str() == term {
	return Ok(true);
    }
    
    Err(syn::Error::new(
	ident.span(),
	format!("Error expected ident to say {} and got {}.", term, ident.to_string())
    ))
}

fn has_attr(attr: &str, item: syn::ItemUse) -> syn::Result<bool> {
    let num_attrs = item.attrs.len();
    if num_attrs == 1 {
	let segments = item.attrs[0].path.segments.clone();
	let num_segments = segments.len();
	if num_segments < 1 || num_segments > 1 {
	    return Err(mk_err(
		segments,
		format!("Error: expected 1 segment but recieved {}.", num_segments)
	    ));
	}

	ident_match(attr, segments[0].ident.clone())?;
    } else if num_attrs > 1 {
	return Err(mk_err(
	    item,
	    format!("Error: expected 1 attribute but recieved {}.", num_attrs)
	));
    }
    
    Ok(false)
}

impl ProcUseMacroInput {
    fn expand(&self) -> proc_macro2::TokenStream  {
	
	for item_outer in self.items.clone() {
	    match item_outer {
		syn::Item::Use(item_use) => {
		    
		    println!("{:#?}", item_use);
		    let res = has_attr("mod_field", item_use);

		    match res {
			Ok(has_attr) => {
			    println!("I have attr and its valid");
			},
			Err(err) => return err.to_compile_error()
		    }
		},
		_ => {
		    println!("fail");
		}
	    }
	    
	}
	
	proc_macro2::TokenStream::new()
    }
}

impl Parse for ProcUseMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
	let content;

        Ok(ProcUseMacroInput {
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
