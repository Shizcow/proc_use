//! # proc_use_inline
//!
//! proc_use_inline is a library for semi-dynamically importing creates/modules.
//!
//! The proc_use_inline is a macro to generate use and mod statements.
//!
//! See the proc_use [website](https://docs.rs/proc_use_inline) for additional documentation and
//! usage examples.
//!
//! [https://docs.rs/proc_use_inline]: https://docs.rs/proc_use_inline

////////////////////////////////////////////////////////////////////////////////
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
			    syn::Expr::Lit(syn::ExprLit{lit: syn::Lit::Str(path), attrs: _}) => {
				Ok(Some(path.value()))
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
			let mod_name = tree_path(&item_use.tree); // only one mod with a path decl
			let mod_stmt = format!("mod {};", mod_name);
			match syn::parse_str::<syn::ItemMod>(&mod_stmt) {
			    Ok(mut item) => {
				item.attrs.push(
				    syn::parse2::<syn::ItemStruct>(quote!{
					#[path = #path]
					struct Dummy;
				    }).unwrap().attrs.pop().unwrap());
				mod_stmts.push(item);
			    },
			    Err(err) => {
				return TokenStream::from(err.to_compile_error());   
			    }
			}
		    },
		    Err(err) => return TokenStream::from(err.to_compile_error())
		}
		
		use_stmts.push(item_use);
	    },
	    syn::Item::Const(syn::ItemConst{attrs, vis, const_token: _, ident, colon_token: _, ty,
					    eq_token: _, expr, semi_token: _}) if
		attrs.len() == 0 && vis == syn::Visibility::Inherited && ident.to_string() == "r#mod"
		=> { // de-sugared mod()
		    match (*ty, *expr) {
			(syn::Type::Infer(syn::TypeInfer{underscore_token: syn::token::Underscore{spans: _}}),
			 syn::Expr::Lit(syn::ExprLit{attrs, lit: syn::Lit::Str(lit)}))
			    if attrs.len() == 0 => {
				let path_str = lit.value();
				let path_buf = std::path::PathBuf::from(path_str.clone());
				match (path_buf.file_stem(), path_buf.extension()) { // ensure file is valid
				    (Some(mod_name), Some(ext)) if ext == "rs" => {
					match mod_name.to_str() {
					    Some(mod_str) => {
						let mod_stmt = format!("mod {};", mod_str);
						match syn::parse_str::<syn::ItemMod>(&mod_stmt) {
						    Ok(mut item) => {
							item.attrs.push(
							    syn::parse2::<syn::ItemStruct>(quote!{
								#[path = #path_str]
								struct Dummy;
							    }).unwrap().attrs.pop().unwrap());
							mod_stmts.push(item);
						    },
						    Err(err) => {
							return TokenStream::from(err.to_compile_error());
						    }
						}
					    },
					    None => return TokenStream::from(mk_err(
						lit,
						"Invalid file. Only UTF8 file names are valie".to_string()
					    ).to_compile_error()),
					}
				    },
				    (Some(_), _) => return TokenStream::from(mk_err(
					lit,
					"Invalid file. Possible causes: file is not a Rust file.".to_string()
				    ).to_compile_error()),
				    (None, _) => return TokenStream::from(mk_err(
					lit,
					"Invalid file. Possible causes: does not exist, is not a regular file.".to_string()
				    ).to_compile_error()),
				}
			    },
			_ => return TokenStream::from(mk_err(
			    ident,
			    "Error: found const r#mod without proper syntax. Likely an internal error.".to_string()
			).to_compile_error()),
		    }
		},
	    err => {
		return TokenStream::from(mk_err(
		    err,
		    "Error: Expected syn::ItemUse. More info found at https://docs.rs/syn/1.0.30/syn/struct.ItemUse.html.".to_string()
		).to_compile_error());
	    }
	}
    }

    TokenStream::from(quote! {
	#(#mod_stmts)*
	#(#use_stmts)*
    })
}

// 1) replaces keyword `mod` with `__mod` in attribute contexts
// 2) replaces `mod($PATH)` with `const r#mod: _ = $PATH;`
fn desugar(input: TokenStream) -> TokenStream {
    let mut tokens: Vec<TokenTree> = input.into_iter().collect();
    for i in 0..tokens.len()-1 {
	match &tokens[i] {
	    TokenTree::Punct(p) if p.as_char() == '#' => {
		match &tokens[i+1] {
		    TokenTree::Group(g) if g.delimiter() == Delimiter::Bracket => {
			let mut stream: Vec<TokenTree> = g.stream().into_iter().collect();
			if stream.len() >= 1 && stream[0].to_string() == "mod" { // this optional arg
			    stream[0] = TokenTree::Ident(Ident::new("__mod", stream[0].span()));
			}
			tokens[i+1] = TokenTree::Group(proc_macro::Group::new(Delimiter::Bracket, stream.into_iter().collect()));
		    },
		    TokenTree::Punct(p) if p.as_char() == '!' => {
			match &tokens[i+2] {
			    TokenTree::Group(g) if g.delimiter() == Delimiter::Bracket => {
				let mut stream: Vec<TokenTree> = g.stream().into_iter().collect();
				if stream.len() == 1 && stream[0].to_string() == "mod" { // this does not
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
	    TokenTree::Ident(__mod) if __mod.to_string() == "mod" => {
		match &tokens[i+1] {
		    TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis => {
			let stream: Vec<TokenTree> = g.stream().into_iter().collect();
			if stream.len() == 1  {
			    let path_quoted = stream[0].to_string();
			    if path_quoted.as_bytes()[0] == '"' as u8 && path_quoted.chars().last() == Some('"') {
				let path = &path_quoted[1..path_quoted.len()-1];
				let new_tokens: Vec<TokenTree> = TokenStream::from(quote!{
				    const r#mod: _ = #path
				}).into_iter().collect();
				tokens.splice(i..i+2, new_tokens.into_iter());
			    }
			} else {
			    tokens[i+1] = TokenTree::Group(proc_macro::Group::new(Delimiter::Bracket, stream.into_iter().collect()));
			}
		    },
		    _ => {},
		}
	    }
	    _ => {},
	}
    }
    
    tokens.into_iter().collect()
}

/// proc_use! macro, takes mod and use syntax to generate mod and use statements.
///
/// # Example
/// ```
/// proc_use_inline::proc_use! {
///    #[mod]
///    use foo::*;
///    #[mod("../external/bar.rs")]
///    use bar::bar;
/// }
/// ```
#[proc_macro]
pub fn proc_use(input: TokenStream) -> TokenStream {
    let input = desugar(input);
    expand(parse_macro_input!(input as File).items)
}
