extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree::{self, Punct, Group}, Delimiter::Bracket, Ident};
use syn::{File, parse_macro_input};

// replaces keyword `mod` with `__mod` in attribute contexts
fn sanitize(input: TokenStream) -> TokenStream {
    let mut tokens: Vec<TokenTree> = input.into_iter().collect();
    for i in 0..tokens.len()-1 {
	match &tokens[i] {
	    Punct(p) if p.as_char() == '#' => {
		match &tokens[i+1] {
		    Group(g) if g.delimiter() == Bracket => {
			let mut stream: Vec<TokenTree> = g.stream().into_iter().collect();
			if stream.len() == 1 && stream[0].to_string() == "mod" {
			    stream[0] = TokenTree::Ident(Ident::new("__mod", stream[0].span()));
			}
			tokens[i+1] = Group(proc_macro::Group::new(Bracket, stream.into_iter().collect()));
		    },
		    Punct(p) if p.as_char() == '!' => {
			match &tokens[i+2] {
			    Group(g) if g.delimiter() == Bracket => {
				let mut stream: Vec<TokenTree> = g.stream().into_iter().collect();
				if stream.len() == 1 && stream[0].to_string() == "mod" {
				    stream[0] = TokenTree::Ident(Ident::new("__mod", stream[0].span()));
				}
				tokens[i+2] = Group(proc_macro::Group::new(Bracket, stream.into_iter().collect()));
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
pub fn proc_use(input: TokenStream) -> TokenStream {
    let input = sanitize(input);
    // because `mod` is a keyword, replace it in attributes with `__mod`
    // This is done without regex in order to preserve span info
    
    let parsed = parse_macro_input!(input as File);

    panic!("{:#?}", parsed);

    //TokenStream::new()
}
