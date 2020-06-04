extern crate proc_macro;

use syn::{File, parse_macro_input};
use proc_macro::{Delimiter, TokenStream, TokenTree, Ident};
use quote::quote;

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
    match tree {
	syn::UseTree::Path(path) => {
	    path.ident.to_string()
	},
	_ => "err".to_string()
    }
}

fn expand(items: Vec<syn::Item>) -> TokenStream  {
    let mut mod_stmts = Vec::new();
    let mut use_stmts = Vec::new();
    
    for item_outer in items.clone() {
	match item_outer {
	    syn::Item::Use(mut item_use) => {
		let res = has_attr("__mod", item_use.clone());
		let mod_name = tree_path(item_use.clone().tree);
		item_use.attrs.clear();
		use_stmts.push(item_use);

		match res {
		    Ok(has_attr) => {
			if has_attr {
			    let mod_stmt = format!("mod {};", mod_name);
			    match syn::parse_str::<syn::ItemMod>(&mod_stmt) {
				Ok(item) => {
				    mod_stmts.push(item);
				},
				Err(err) => {
				    return TokenStream::from(err.to_compile_error());   
				}
			    }
			} else {
			    println!("no attr!");
			}
		    },
		    Err(err) => return TokenStream::from(err.to_compile_error())
		}
	    },
	    _ => {
		return TokenStream::from(mk_err(
		    item_outer,
		    "Error: expected syn::ItemUse. More info found at https://docs.rs/syn/1.0.30/syn/struct.ItemUse.html.".to_string()
		).to_compile_error());
	    }
	}
    }

    TokenStream::from(quote! {
	#(#mod_stmts)*
	#(#use_stmts)*
    })
}

// replaces keyword `mod` with `__mod` in attribute contexts
fn sanitize(input: TokenStream) -> TokenStream {
    let mut tokens: Vec<TokenTree> = input.into_iter().collect();
    for i in 0..tokens.len()-1 {
	match &tokens[i] {
	    TokenTree::Punct(p) if p.as_char() == '#' => {
		match &tokens[i+1] {
		    TokenTree::Group(g) if g.delimiter() == Delimiter::Bracket => {
			let mut stream: Vec<TokenTree> = g.stream().into_iter().collect();
			if stream.len() == 1 && stream[0].to_string() == "mod" {
			    stream[0] = TokenTree::Ident(Ident::new("__mod", stream[0].span()));
			}
			tokens[i+1] = TokenTree::Group(proc_macro::Group::new(Delimiter::Bracket, stream.into_iter().collect()));
		    },
		    TokenTree::Punct(p) if p.as_char() == '!' => {
			match &tokens[i+2] {
			    TokenTree::Group(g) if g.delimiter() == Delimiter::Bracket => {
				let mut stream: Vec<TokenTree> = g.stream().into_iter().collect();
				if stream.len() == 1 && stream[0].to_string() == "mod" {
				    stream[0] = TokenTree::Ident(Ident::new("__mod", stream[0].span()));
				}
				tokens[i+2] = TokenTree::Group(proc_macro::Group::new(Delimiter::Bracket, stream.into_iter().collect()));
			    },
			    _ => {},
			}
		    },
		    _ => {},
		}
	    },
	    _ => {},
	}
    }
    
    tokens.into_iter().collect()
}

#[proc_macro]
pub fn proc_use_inline(input: TokenStream) -> TokenStream {
    let input = sanitize(input);
    expand(parse_macro_input!(input as File).items)
}
