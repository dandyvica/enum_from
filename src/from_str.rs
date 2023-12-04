// gather here all helper functions for create the EnumFromStr proc macro
// all these functions return the proc_macro2::TokenStream type which is later
// converted to a TokenStream
use quote::quote;
use syn::{DataEnum, DeriveInput, Fields, Ident, Variant};

pub struct EnumFromStr;

impl EnumFromStr {
    pub fn impl_from_str(ast: &DeriveInput, de: &DataEnum) -> proc_macro2::TokenStream {
        let enum_name = &ast.ident;
        let arms = de
            .variants
            .iter()
            .map(|v| EnumFromStr::build_fromstr_arm(enum_name, v));

        // this is probably not useful but left here to see how to handle lifetimes and generics
        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        quote! {
            impl #impl_generics std::str::FromStr for #enum_name #ty_generics #where_clause {
                type Err = String;

                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    match s {
                        #( #arms)*
                        _ => Err(format!("no variant corresponding to value {}", s)),
                    }
                }
            }
        }
    }

    fn build_fromstr_arm(enum_name: &Ident, variant: &Variant) -> proc_macro2::TokenStream {
        let variant_ident = &variant.ident;
        let variant_ident_as_string = &variant.ident.to_string();

        // this only works for C-like enums
        if matches!(&variant.fields, Fields::Unit) {
            quote! {
                #variant_ident_as_string => Ok(#enum_name::#variant_ident),
            }
        } else {
            quote!()
        }
    }
}
