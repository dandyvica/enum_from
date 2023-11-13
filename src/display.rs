// gather here all helper functions for create the EnumDisplay proc macro
// all these functions return the proc_macro2::TokenStream type which is later
// converted to a TokenStream
use quote::quote;
use syn::{DataEnum, DeriveInput, Fields, Ident, Variant};

pub struct EnumDisplay;

impl EnumDisplay {
    pub fn impl_display(ast: &DeriveInput, de: &DataEnum) -> proc_macro2::TokenStream {
        let enum_name = &ast.ident;
        let arms = de
            .variants
            .iter()
            .map(|v| EnumDisplay::build_display_arm(enum_name, v));

        // this is probably not useful but left here to see how to handle lifetimes and generics
        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        quote! {
            impl #impl_generics std::fmt::Display for #enum_name #ty_generics #where_clause {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        #( #arms)*
                    }
                }
            }
        }
    }

    fn build_display_arm(enum_name: &Ident, variant: &Variant) -> proc_macro2::TokenStream {
        let variant_ident = &variant.ident;
        let variant_ident_as_string = &variant.ident.to_string();

        // this only works for C-like enums
        if matches!(&variant.fields, Fields::Unit) {
            quote! {
                #enum_name::#variant_ident => write!(f, #variant_ident_as_string),
            }
        } else {
            unimplemented!("only C-like enums are implemented")
        }
    }
}
