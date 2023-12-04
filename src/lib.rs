use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

mod from_str;
use from_str::EnumFromStr;

mod try_from;
use try_from::EnumTryFrom;

mod display;
use display::EnumDisplay;

//--------------------------------------------------------------------------------
// Implement the FromStr trait for C-like enums only
//--------------------------------------------------------------------------------
#[proc_macro_derive(EnumFromStr)]
// The string inside proc_macro_derive is the name of the proc macro to generate
// the code when the macro is used
// The name off the function (here: from_str()) doesn't matter
pub fn from_str(input: TokenStream) -> TokenStream {
    // this is the usual way to get the ast from the input
    let ast = parse_macro_input!(input as DeriveInput);

    // if we can't get DataEnum type, this is not an enum and compilation fails
    let code: proc_macro2::TokenStream = match &ast.data {
        Data::Enum(de) => EnumFromStr::impl_from_str(&ast, de),
        _ => unimplemented!("{} is not an enum", ast.ident.to_string()),
    };

    // uncomment this to view generated code during compilation
    println!("{}", code);

    // this convert proc_macro2::TokenStream into a TokenStream
    code.into()
}

//--------------------------------------------------------------------------------
// Implement the TryFrom trait for C-like enums only
//--------------------------------------------------------------------------------
#[proc_macro_derive(EnumTryFrom, attributes(fallback))]
pub fn try_from(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let code: proc_macro2::TokenStream = match &ast.data {
        Data::Enum(de) => EnumTryFrom::impl_tryfrom(&ast, de),
        _ => unimplemented!("{} is not an enum", ast.ident.to_string()),
    };

    println!("{}", code);
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

    println!("{}", code);
    code.into()
}
