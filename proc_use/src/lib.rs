extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse_macro_input, Result, Token};
use syn::parse::{Parse, ParseStream};
use quote::quote;

fn mk_err<T: quote::ToTokens>(t: T, msg: String) -> syn::Error {
    syn::Error::new_spanned(t, msg)
}

fn ident_match(term: &str, ident: syn::Ident) -> syn::Result<bool> {
    println!("term: {}, ident: {}", term, ident.to_string());
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

	return ident_match(attr, segments[0].ident.clone());
    } else if num_attrs > 1 {
	return Err(mk_err(
	    item,
	    format!("Error: expected 1 attribute but recieved {}.", num_attrs)
	));
    }
    
    Ok(false)
}

fn tree_path(tree: syn::UseTree) -> String {
    "".to_string()
}

fn expand(items: Vec<syn::Item>) -> proc_macro2::TokenStream  {
    for item_outer in items.clone() {
	match item_outer {
	    syn::Item::Use(item_use) => {
		
		println!("{:#?}", item_use);
		let res = has_attr("__mod", item_use);

		match res {
		    Ok(has_attr) => {
			if has_attr {
			    println!("I have attr and its valid");
			} else {
			    println!("no attr!");
			}
		    },
		    Err(err) => return err.to_compile_error()
		}
	    },
	    _ => {
		return mk_err(
		    item_outer,
		    "Error: expected syn::ItemUse. More info found at https://docs.rs/syn/1.0.30/syn/struct.ItemUse.html.".to_string()
		).to_compile_error();
	    }
	}
	
    }
	
	proc_macro2::TokenStream::new()
}

#[proc_macro]
pub fn proc_use_inline(input: TokenStream) -> TokenStream {
    let input = syn::parse::<syn::File>(input);
    
    println!("{:?}", input);
    match input {
	Ok(tree) => {
	    let output = expand(tree.items);
	},
	Err(err) => return TokenStream::from(err.to_compile_error())
    }
    
    
    TokenStream::new()
}
