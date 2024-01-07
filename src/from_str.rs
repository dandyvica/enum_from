// gather here all helper functions for create the EnumFromStr proc macro
// all these functions return the proc_macro2::TokenStream type which is later
// converted to a TokenStream
use quote::{quote, ToTokens};
use syn::{DataEnum, DeriveInput, Fields, Ident, Variant};
use syn_utils::VariantHelper;

pub struct EnumFromStr;

impl EnumFromStr {
    pub fn impl_from_str(ast: &DeriveInput, de: &DataEnum) -> proc_macro2::TokenStream {
        let enum_name = &ast.ident;
        let arms = de
            .variants
            .iter()
            .map(|v| EnumFromStr::build_fromstr_arm(enum_name, v));

        // check if the enum owns a fallback attribute because the _ arm in that case
        // already include the _ arm
        let found_fallback = de
            .variants
            .iter()
            .any(|a| a.has_attribute("fallback").is_some());

        let underscore_arm = if found_fallback {
            quote!()
        } else {
            quote!(_ => Err(format!("no variant corresponding to value '{}'", s)))
        };

        // this is probably not useful but left here to see how to handle lifetimes and generics
        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

        quote! {
            impl #impl_generics std::str::FromStr for #enum_name #ty_generics #where_clause {
                type Err = String;

                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    match s {
                        #( #arms)*
                        #underscore_arm
                    }
                }
            }
        }
    }

    fn build_fromstr_arm(enum_name: &Ident, variant: &Variant) -> proc_macro2::TokenStream {
        let variant_ident = &variant.ident;
        let variant_ident_as_string = &variant.ident.to_string();

        // arm if depending on variant type
        match &variant.fields {
            // unit variant like: Quit = 1
            Fields::Unit => quote! {
                #variant_ident_as_string => Ok(#enum_name::#variant_ident),
            },

            // named variant like: Move { x: i32, y: i32 }
            Fields::Named(_) => panic!("FromStr trait not supported for named variant"),

            // unnamed variant like: ChangeColor(i32, i32, i32)
            // only those having just on field are supported.
            // e.g.: CLASS1234
            Fields::Unnamed(fields) => {
                // safeguard: only unnamed variant with one field are supported
                if fields.unnamed.len() != 1 {
                    panic!("FromStr: only unnamed variant with one field are supported");
                }

                // we can safely unwrap
                let unique = fields.unnamed.first().unwrap();

                // get the type of this unique field
                let ts = unique.ty.to_token_stream();
                quote! {
                    // extract value from string: 1234 in CLASS1234
                    // and convert it to value
                    _ => if let Some(n) = s.strip_prefix(#variant_ident_as_string) {
                        // parse could fail
                        if let Ok(x) = n.parse::<#ts>() {
                            Ok(#enum_name::#variant_ident(x))
                        } else {
                            Err(format!("no variant corresponding to value '{}'", s))
                        }
                    } else {
                        Err(format!("no variant corresponding to value '{}'", s))
                    }
                }
            }
        }
    }
}
