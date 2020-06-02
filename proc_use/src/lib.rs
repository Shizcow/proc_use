use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{Expr, Lit, ItemUse, spanned::Spanned};
use quote::quote;

#[proc_macro]
pub fn proc_use(input: TokenStream) -> TokenStream {
    if let Ok(items) = syn::parse::<Expr>(input) {
	match items {
	    Expr::Array(arr) => {
		let mut uses = Vec::new();
		for elem in arr.elems.into_iter() {
		    let error = match elem {
			Expr::Lit(lit) =>
			    match lit.lit {
				Lit::Str(string) =>
				    match syn::parse_str::<ItemUse>
				    (&format!("use {};", string.value())) {
					Ok(item) => {
					    uses.push(item);
					    continue;
					},
					Err(error) => error.span(),
				    },
				error => error.span(),
			    },
			error => error.span(),
		    };
		    return syn::Error::new(
			error,
			"Invalid format for use. You just want the import path, eg: \
			 \"foo::{bar, baz}\""
		    ).to_compile_error().into();
		}
		return TokenStream::from(quote! {
		    #(#uses)*
		})
	    },
	    other => return syn::Error::new(
		other.span(),
		"I can't yet parse this item"
	    ).to_compile_error().into(),
	}
    }
    syn::Error::new(
	Span::call_site(),
	"Syntax Error"
    ).to_compile_error().into()
}
