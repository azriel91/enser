extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

use proc_macro2::Ident;
use proc_macro_roids::{FieldsExt, IdentExt};
use syn::{parse_macro_input, Fields, ItemEnum};

#[proc_macro_attribute]
pub fn enser(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as ItemEnum);

    let gen = impl_enser_derive(&mut item);

    gen.into()
}

fn impl_enser_derive(the_enum: &mut ItemEnum) -> proc_macro2::TokenStream {
    let mut enser = the_enum.clone();

    // generate separate enum, attach `(())` to every unit variant
    let enser_mod: Ident = parse_quote!(__enser);
    let enser_enum_name = enser.ident.append("Serde");
    let enser_enum_path_str = format!("{enser_mod}::{enser_enum_name}");

    // MyEnum -> MyEnumSerde
    enser.ident = enser_enum_name;

    // Variant -> Variant(())
    attach_tuple_to_unit_variants(&mut enser);

    let the_enum_from_enser = impl_the_enum_from_enser(the_enum, &enser);
    let enser_from_the_enum = impl_enser_from_the_enum(the_enum, &enser);

    // Tell serde to serialize and deserialize with `enser`
    the_enum
        .attrs
        .push(parse_quote!(#[serde(from = #enser_enum_path_str, into = #enser_enum_path_str)]));
    let the_enum_name = &the_enum.ident;

    quote! {
        #the_enum

        mod #enser_mod {
            use super::#the_enum_name;

            #enser

            #the_enum_from_enser

            #enser_from_the_enum
        }
    }
}

fn attach_tuple_to_unit_variants(enser: &mut ItemEnum) {
    enser
        .variants
        .iter_mut()
        .filter_map(|variant| {
            if let Fields::Unit = &variant.fields {
                Some(&mut variant.fields)
            } else {
                None
            }
        })
        .for_each(|fields| *fields = Fields::Unnamed(parse_quote!((()))));
}

fn impl_the_enum_from_enser(the_enum: &ItemEnum, enser: &ItemEnum) -> proc_macro2::TokenStream {
    let the_enum_name = &the_enum.ident;
    let enser_enum_name = &enser.ident;

    let variant_mappings = the_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Unit => {
                quote!(#enser_enum_name::#variant_name(()) => #the_enum_name::#variant_name)
            }
            fields @ Fields::Unnamed(_) | fields @ Fields::Named(_) => {
                let fields = fields.construction_form();
                quote! {
                    #enser_enum_name::#variant_name #fields =>
                    #the_enum_name::#variant_name #fields
                }
            }
        }
    });

    quote! {
        impl From<#enser_enum_name> for #the_enum_name {
            fn from(enser_enum: #enser_enum_name) -> Self {
                match enser_enum {
                    #(#variant_mappings),*
                }
            }
        }
    }
}

fn impl_enser_from_the_enum(the_enum: &ItemEnum, enser: &ItemEnum) -> proc_macro2::TokenStream {
    let the_enum_name = &the_enum.ident;
    let enser_enum_name = &enser.ident;

    let variant_mappings = the_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Unit => {
                quote!(#the_enum_name::#variant_name => #enser_enum_name::#variant_name(()))
            }
            fields @ Fields::Unnamed(_) | fields @ Fields::Named(_) => {
                let fields = fields.construction_form();
                quote! {
                    #the_enum_name::#variant_name #fields =>
                    #enser_enum_name::#variant_name #fields
                }
            }
        }
    });

    quote! {
        impl From<#the_enum_name> for #enser_enum_name {
            fn from(the_enum: #the_enum_name) -> Self {
                match the_enum {
                    #(#variant_mappings),*
                }
            }
        }
    }
}
