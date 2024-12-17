// This defines a procedural macro that can be used to derive a trait for a type.

use proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens};

#[proc_macro_derive(LitIntBug)]
pub fn lit_int_bug_derive(input: TokenStream) -> TokenStream {
    let int = -12;
    let integer = quote! { #int };
    //let integer: syn::LitInt = syn::parse_str("-12").unwrap();
    let debug_tokens: Vec<String> = integer
        .to_token_stream()
        .into_iter()
        .map(|t| format!("{:?}", t))
        .collect();
    let debug_tokens_str = debug_tokens.join(" + ");

    quote! {
        pub struct Zapp {
            pub numbers: Vec<i32>,
        }

        /// This contant contains the debug representation of syn::parse_str::<syn::LitInt>("-12")
        const INT_DEBUG_STR: &str = #debug_tokens_str;

        fn new_zapp() -> Zapp {
            Zapp {
                numbers: vec![#integer],
            }
        }
    }
    .into()
}
