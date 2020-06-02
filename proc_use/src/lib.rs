use proc_macro::TokenStream;
use syn::{Expr, Lit};
use quote::quote;
use proc_macro2::{Ident, Span};

#[proc_macro]
pub fn proc_use(input: TokenStream) -> TokenStream {
    if let Ok(items) = syn::parse::<Expr>(input) {
	if let Expr::Array(arr) = items {
	    let idents = arr.elems.into_iter().map(|elem| {
		if let Expr::Lit(lit) = elem {
		    if let Lit::Str(string) = lit.lit {
			return Ident::new(&string.value(),
						   Span::call_site());
		    } // TODO: compiler error "must be a string"
		} // TODO: compiler error "need lit"
		panic!("I don't have error reporting yet");
	    });
	    return TokenStream::from(quote! {
		#(use #idents::*;)*
	    })
	} // TODO: env!()
    } // TODO: compiler error on else
    
    TokenStream::new()
}
