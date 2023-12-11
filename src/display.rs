// gather here all helper functions for create the EnumDisplay proc macro
// all these functions return the proc_macro2::TokenStream type which is later
// converted to a TokenStream
use proc_macro2::Span;
use quote::quote;
use syn::{DataEnum, DeriveInput, Fields, Ident, Variant};

use syn_utils::*;

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
                    Ok(())
                }
            }
        }
    }

    fn build_display_arm(enum_name: &Ident, variant: &Variant) -> proc_macro2::TokenStream {
        let variant_ident = &variant.ident;
        let variant_ident_as_string = &variant.ident.to_string();

        match &variant.fields {
            // unnamed variant like: ChangeColor(i32, i32, i32)
            Fields::Unnamed(_) => {
                // check for the fallback attribute. If so, use the variant name to print out value
                let has_attr = variant.has_attribute("fallback");

                let fields = (0..variant.fields.len())
                    .map(|i| Ident::new(&format!("f{}", i), Span::call_site()));

                let method_calls = fields.clone().map(|f| {
                    if has_attr.is_none() {
                        quote! {
                            write!(f, "{}", #f)?;
                        }
                    } else {
                        quote! {
                            write!(f, "{}{}", #variant_ident_as_string, #f)?;
                        }
                    }
                });

                quote! {
                    #enum_name::#variant_ident(#(#fields),*) => {
                        #( #method_calls)*
                    },
                }
            }

            // named variant like: Move { x: i32, y: i32 }
            Fields::Named(_) => {
                let members = variant.fields.iter().map(|f| &f.ident);

                let method_calls = members.clone().map(|f| {
                    quote! {
                        write!(f, "{}", #f)?;
                    }
                });

                quote! {
                    #enum_name::#variant_ident{#(#members),*} => {
                        #( #method_calls)*
                    },
                }
            }

            // unit variant like: Quit = 1
            Fields::Unit => quote! {
                #enum_name::#variant_ident => write!(f, #variant_ident_as_string)?,
            },
        }
    }
}
