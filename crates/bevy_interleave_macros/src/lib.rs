extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{
    DeriveInput,
    parse_macro_input,
};

mod planar;
use planar::generate_planar_struct;


#[proc_macro_derive(Planar)]
pub fn planar_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let output = match generate_planar_struct(&input) {
        Ok(quote) => quote,
        Err(e) => return e.to_compile_error().into(),
    };

    TokenStream::from(output)
}
