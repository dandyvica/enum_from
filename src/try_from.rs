// gather here all helper functions for create the EnumTryFrom proc macro
// all these functions return the proc_macro2::TokenStream type which is later
// converted to a TokenStream
use quote::quote;
use syn::{DataEnum, DeriveInput};

pub struct EnumTryFrom;

impl EnumTryFrom {
    pub fn impl_try_from(ast: &DeriveInput, de: &DataEnum) -> proc_macro2::TokenStream {
        let enum_name = &ast.ident;

        let arms = de.variants.iter().map(|v| {
            let variant_ident = &v.ident;
            quote! {
                x if x == #enum_name::#variant_ident as u64 => Ok(#enum_name::#variant_ident),
            }
        });

        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        quote! {
            impl #impl_generics std::convert::TryFrom<u64> for #enum_name #ty_generics #where_clause {
                type Error = &'static str;

                fn try_from(value: u64) -> std::result::Result<Self, Self::Error> {
                    match value {
                        #( #arms)*
                        _ => Err("no variant corresponding to value"),
                    }
                }
            }
        }
    }
}
