extern crate proc_macro;

use syn::{File, parse_macro_input, spanned::Spanned};
use proc_macro::{Delimiter, TokenStream, TokenTree, Ident};
use quote::quote;

fn mk_err<T: quote::ToTokens>(t: T, msg: String) -> syn::Error {
    syn::Error::new_spanned(t, msg)
}

fn ident_match(term: &str, ident: &syn::Ident) -> syn::Result<bool> {
    if ident.to_string().as_str() == term {
	return Ok(true);
    }
    
    Err(syn::Error::new(
	ident.span(),
	format!("Error expected ident to say {} and got {}.", term, ident.to_string())
    ))
}

fn extract_path(attr_str: &str, item: &mut syn::ItemUse) -> syn::Result<Option<String>> {
    let num_attrs = item.attrs.len();
    if num_attrs == 1 {
	let attr = item.attrs.pop().unwrap();
	let segments = attr.path.segments;
	let num_segments = segments.len();
	if num_segments < 1 || num_segments > 1 {
	    return Err(mk_err(
		segments,
		format!("Error: expected 1 segment but recieved {}.", num_segments)
	    ));
	}
	return match ident_match(attr_str, &segments[0].ident) {
	    Ok(true) if !attr.tokens.is_empty() => {
		match syn::parse2::<syn::ExprParen>(attr.tokens) {
		    Ok(paren) => {
			match *paren.expr {
			    syn::Expr::Lit(lit) => {
				match lit.lit {
				    syn::Lit::Str(path) => Ok(Some(path.value())),
				    err => Err(syn::Error::new(err.span(),
							       "Expected string literal for path"
				    )),
				}
			    }
			    err => Err(syn::Error::new(err.span(),
						       "Expected string literal for path"
			    )),
			}
		    },
		    Err(err) => Err(err),
		}
	    },
	    Err(err) => Err(err),
	    _ => Ok(None),
	};
    } else if num_attrs > 1 {
	return Err(mk_err(
	    item,
	    format!("Error: expected 1 attribute but recieved {}.", num_attrs)
	));
    }
    Ok(None)
}

fn tree_path(tree: &syn::UseTree) -> String {
    match tree {
	syn::UseTree::Path(path) => {
	    path.ident.to_string()
	},
	_ => "err".to_string()
    }
}

fn tree_paths(tree: &syn::UseTree) -> Vec<String> {
    match tree {
	syn::UseTree::Group(group) => {
	    group.items.iter().map(|item| tree_path(item)).collect()
	},
	item => vec![tree_path(item)]
    }
}

fn expand(items: Vec<syn::Item>) -> TokenStream  {
    let mut mod_stmts = Vec::new();
    let mut use_stmts = Vec::new();
    
    for item_outer in items.into_iter() {
	match item_outer {
	    syn::Item::Use(mut item_use) => {
		let res = extract_path("__mod", &mut item_use);
		item_use.attrs.clear();

		match res {
		    Ok(None) => {
			let mod_names = tree_paths(&item_use.tree);
			for mod_name in mod_names {
			    let mod_stmt = format!("mod {};", mod_name);
			    match syn::parse_str::<syn::ItemMod>(&mod_stmt) {
				Ok(item) => {
				    mod_stmts.push(item);
				},
				Err(err) => {
				    return TokenStream::from(err.to_compile_error());   
				}
			    }
			}
		    },
		    Ok(Some(path)) => {
			panic!("found path {}", path);
		    },
		    Err(err) => return TokenStream::from(err.to_compile_error())
		}
		
		use_stmts.push(item_use);
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
