use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};
mod try_from;
use try_from::EnumTryFrom;

mod display;
use display::EnumDisplay;

//--------------------------------------------------------------------------------
// Implement the TryFrom trait for C-like enums only
//--------------------------------------------------------------------------------
#[proc_macro_derive(EnumTryFrom)]
pub fn try_from(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let code: proc_macro2::TokenStream = match &ast.data {
        Data::Enum(de) => EnumTryFrom::impl_tryfrom(&ast, de),
        _ => unimplemented!("{} is not an enum", ast.ident.to_string()),
    };

    //println!("{}", code);
    code.into()
}

//--------------------------------------------------------------------------------
// Implement the Display trait for C-like enums only
//--------------------------------------------------------------------------------
#[proc_macro_derive(EnumDisplay)]
pub fn display(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let code: proc_macro2::TokenStream = match &ast.data {
        Data::Enum(de) => EnumDisplay::impl_display(&ast, de),
        _ => unimplemented!("{} is not an enum", ast.ident.to_string()),
    };

    //println!("{}", code);
    code.into()
}
