use proc_macro::TokenStream;
use syn::{Expr, Lit};
use quote::quote;
use proc_macro2::{Ident, Span};

#[proc_macro]
pub fn proc_use(input: TokenStream) -> TokenStream {
    if let Ok(items) = syn::parse::<Expr>(input) {
	if let Expr::Array(arr) = items {
	    for elem in arr.elems.into_iter() { // TODO concat the quotes
		if let Expr::Lit(lit) = elem {
		    if let Lit::Str(string) = lit.lit {
			let use_ident = Ident::new(&string.value(),
						   Span::call_site());
			return TokenStream::from(quote! {
			    use #use_ident::*;
			})
		    } // TODO: compiler error "must be a string"
		} // TODO: compiler error "need lit"
	    }
	} // TODO: env!()
    } // TODO: compiler error on else
    
    TokenStream::new()
}
