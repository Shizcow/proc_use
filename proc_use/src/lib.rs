use proc_macro::TokenStream;
use syn::{Expr, Lit, ItemUse};
use quote::quote;

#[proc_macro]
pub fn proc_use(input: TokenStream) -> TokenStream {
    if let Ok(items) = syn::parse::<Expr>(input) {
	if let Expr::Array(arr) = items {
	    let uses = arr.elems.into_iter().map(|elem| {
		if let Expr::Lit(lit) = elem {
		    if let Lit::Str(string) = lit.lit {
			return syn::parse_str::<ItemUse>(&format!("use {};", string.value()))
			    .unwrap();
		    } // TODO: compiler error "must be a string"
		} // TODO: compiler error "need lit"
		panic!("I don't have error reporting yet");
	    });
	    return TokenStream::from(quote! {
		#(#uses)*
	    })
	} // TODO: env!()
    } // TODO: compiler error on else
    
    TokenStream::new()
}
