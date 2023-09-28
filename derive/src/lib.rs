use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Parser, attributes(command, arg))]
pub fn parser_derive(input: TokenStream) -> TokenStream {
    let _ast = parse_macro_input!(input as DeriveInput);
    // Your implementation here

    TokenStream::from(quote! {
        // Generated code
    })
}
