// gather here all helper functions for create the EnumTryFrom proc macro
// all these functions return the proc_macro2::TokenStream type which is later
// converted to a TokenStream
use quote::quote;
use syn::{DataEnum, DeriveInput};

use syn_utils::*;

pub struct EnumTryFrom;

impl EnumTryFrom {
    pub fn impl_tryfrom(ast: &DeriveInput, de: &DataEnum) -> proc_macro2::TokenStream {
        let enum_name = &ast.ident;

        // enums with named fields are not supported
        if de.variants.iter().any(|v| v.is_named().is_some()) {
            unimplemented!("enum {} contains named fields", enum_name)
        }

        // if enum contains unnamed fields, there should be only one and have only one field
        let unnamed: Vec<_> = de
            .variants
            .iter()
            .filter(|v| v.is_named().is_some())
            .collect();
        match unnamed.len() {
            0 => (),
            1 => {
                if unnamed[0].fields.len() != 1 {
                    unimplemented!(
                        "enum {} contains one unnamed field with more than one field",
                        enum_name
                    )
                }
            }
            _ => unimplemented!("enum {} contains more than one unnamed field", enum_name),
        }

        // check if the enum owns a fallback attribute
        let found_fallback = de
            .variants
            .iter()
            .any(|a| a.has_attribute("fallback").is_some());

        // get the type inside #[repr()]
        let ty = SynUtils::repr_size(&ast.attrs)
            .unwrap_or_else(|| unimplemented!("repr size is mandatory on enum {}", enum_name));

        // loop through fields to build code
        let arms = de.variants.iter().map(|v| {
            let variant_ident = &v.ident;

            // if we find the #[fallback] attribute for that variant
            if v.has_attribute("fallback").is_some() && !v.is_unit() {
                quote!(_ =>  Ok(#enum_name::#variant_ident(value)))
            } else {
                // extract the litteral value of the variant. Ex: Ok = 0
                let lit = v.literal();
                quote! {
                    #lit => Ok(Self::#variant_ident),
                }
            }
        });

        // search for the ident "Reserved"
        let err_case = if !found_fallback {
            quote!(_ => Err(value))
        } else {
            quote!()
        };

        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        quote! {
            impl #impl_generics std::convert::TryFrom<#ty> for #enum_name #ty_generics #where_clause {
                type Error = #ty;

                fn try_from(value: #ty) -> std::result::Result<Self, Self::Error> {
                    match value {
                        #( #arms)*
                        #err_case
                    }
                }
            }
        }
    }
}
