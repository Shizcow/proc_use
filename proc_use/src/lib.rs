use proc_macro::TokenStream;
use syn::{Expr, Lit, ItemUse};
use quote::quote;

#[proc_macro]
pub fn proc_use(input: TokenStream) -> TokenStream {
    if let Ok(items) = syn::parse::<Expr>(input) {
	if let Expr::Array(arr) = items {
	    let mut uses = Vec::new();
	    for elem in arr.elems.into_iter() {
		if let Expr::Lit(lit) = elem {
		    if let Lit::Str(string) = lit.lit {
			match syn::parse_str::<ItemUse>(&format!("use {};", string.value())) {
			    Ok(item) => {
				uses.push(item);
				continue;
			    },
			    Err(_) => return syn::Error::new(
				string.span(),
				"Invalid format for use"
			    ).to_compile_error().into(),
			}
		    } // TODO: compiler error "must be a string"
		} // TODO: compiler error "need lit"
		panic!("I don't have intelligent error reporting yet");
	    }
	    return TokenStream::from(quote! {
		#(#uses)*
	    })
	} // TODO: env!()
    } // TODO: compiler error on else
    
    TokenStream::new()
}
