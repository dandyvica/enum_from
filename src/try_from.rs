// gather here all helper functions for create the EnumTryFrom proc macro
// all these functions return the proc_macro2::TokenStream type which is later
// converted to a TokenStream
use quote::quote;
use syn::{Attribute, DataEnum, DeriveInput, Fields, Ident, Variant};

pub struct EnumTryFrom;

impl EnumTryFrom {
    pub fn impl_tryfrom(ast: &DeriveInput, de: &DataEnum) -> proc_macro2::TokenStream {
        let enum_name = &ast.ident;

        // get the type inside #[repr()]
        let ty = process_attr(&enum_name, &ast.attrs);

        let arms_ints = de.variants.iter().map(|v| {
            let variant_ident = &v.ident;
            quote! {
                x if x == #enum_name::#variant_ident as #ty => Ok(#enum_name::#variant_ident),
            }
        });

        let arms_str = de
            .variants
            .iter()
            .map(|v| Self::build_tryfrom_str_arm(enum_name, v));

        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        quote! {
            impl #impl_generics std::convert::TryFrom<#ty> for #enum_name #ty_generics #where_clause {
                type Error = #ty;

                fn try_from(value: #ty) -> std::result::Result<Self, Self::Error> {
                    match value {
                        #( #arms_ints)*
                        _ => Err(value),
                    }
                }
            }

            impl<'str> #impl_generics std::convert::TryFrom<&'str str> for #enum_name #ty_generics #where_clause {
                type Error = &'str str;

                fn try_from(s: &'str str) -> std::result::Result<Self, Self::Error> {
                    match s {
                        #( #arms_str)*
                        _ => Err(s),
                    }
                }
            }
        }
    }

    fn build_tryfrom_str_arm(enum_name: &Ident, variant: &Variant) -> proc_macro2::TokenStream {
        let variant_ident = &variant.ident;
        let variant_ident_as_string = &variant.ident.to_string();

        // this only works for C-like enums
        if matches!(&variant.fields, Fields::Unit) {
            quote! {
                #variant_ident_as_string => Ok(#enum_name::#variant_ident),
            }
        } else {
            unimplemented!("only C-like enums are implemented")
        }
    }
}

//process the #[deser] attribute for all different cases
fn process_attr(ident: &Ident, attrs: &[Attribute]) -> proc_macro2::TokenStream {
    let mut ty = proc_macro2::TokenStream::default();

    for attr in attrs {
        if attr.path().is_ident("repr") {
            let _ = attr.parse_nested_meta(|meta| {
                // #[repr(u8)]
                if meta.path.is_ident("u8") {
                    ty = quote!(u8);
                    return Ok(());
                }
                // #[repr(u16)]
                if meta.path.is_ident("u16") {
                    ty = quote!(u16);
                    return Ok(());
                }
                // #[repr(u32)]
                if meta.path.is_ident("u32") {
                    ty = quote!(u32);
                    return Ok(());
                }
                // #[repr(u64)]
                if meta.path.is_ident("u64") {
                    ty = quote!(u64);
                    return Ok(());
                }
                // #[repr(i8)]
                if meta.path.is_ident("i8") {
                    ty = quote!(i8);
                    return Ok(());
                }
                // #[repr(i16)]
                if meta.path.is_ident("i16") {
                    ty = quote!(i16);
                    return Ok(());
                }
                // #[repr(i32)]
                if meta.path.is_ident("i32") {
                    ty = quote!(i32);
                    return Ok(());
                }
                // #[repr(i64)]
                if meta.path.is_ident("64") {
                    ty = quote!(i64);
                    return Ok(());
                }

                unimplemented!("unsupported repr() in enum {}", ident.to_string());
            });
        }
    }

    ty
}
