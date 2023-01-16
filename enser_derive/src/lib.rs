extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

use proc_macro2::Ident;
use proc_macro_roids::{FieldsExt, IdentExt};
use syn::{
    parse_macro_input, Fields, GenericParam, Generics, ImplGenerics, ItemEnum, PathSegment,
    PredicateType, TraitBound, Type, TypeGenerics, TypeParam, TypeParamBound, TypePath,
    WhereClause, WherePredicate,
};

#[proc_macro_attribute]
pub fn enser(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as ItemEnum);

    let gen = impl_enser_derive(&mut item);

    gen.into()
}

fn impl_enser_derive(the_enum: &mut ItemEnum) -> proc_macro2::TokenStream {
    // The enum must implement `Clone`, as well as `Deserialize` and `Serialize`.
    let mut enser = the_enum.clone();

    // generate separate enum, attach `(())` to every unit variant
    let enser_mod: Ident = parse_quote!(_enser);
    let enser_mod = enser_mod.append(&the_enum.ident);
    let enser_enum_name = enser.ident.append("Serde");

    // MyEnum -> MyEnumSerde
    enser.vis = parse_quote!(pub(super));
    enser.ident = enser_enum_name;

    // Variant -> Variant(())
    attach_tuple_to_unit_variants(&mut enser);

    let Generics {
        params,
        where_clause,
        ..
    } = &mut the_enum.generics;
    params.iter_mut().for_each(|generic_param| {
        if let GenericParam::Type(TypeParam {
            ident: type_param_ident,
            bounds,
            ..
        }) = generic_param
        {
            // If neither the bounds in the param / in the where clause contains `Clone`,
            // then we add the `Clone` bound
            let where_bounds_for_type_contains_clone =
                if let Some(where_clause) = where_clause.as_ref() {
                    where_clause
                        .predicates
                        .iter()
                        .filter_map(|predicate| {
                            if let WherePredicate::Type(PredicateType {
                                bounded_ty: Type::Path(TypePath { path, .. }),
                                bounds,
                                ..
                            }) = predicate
                            {
                                if path.segments.len() == 1 {
                                    if let Some(PathSegment {
                                        ident: predicate_ident,
                                        ..
                                    }) = path.segments.last()
                                    {
                                        if type_param_ident == predicate_ident {
                                            // This where predicate is for the type param
                                            return Some(bounds);
                                        }
                                    }
                                }
                            }
                            None
                        })
                        .any(|bounds| bounds_contains_clone(bounds.iter()))
                } else {
                    false
                };
            if !bounds_contains_clone(bounds.iter()) && !where_bounds_for_type_contains_clone {
                // Add the `Clone` bound
                //
                // We can't add this to the type bounds, because something makes Rust error:
                //
                // ```text
                // error[E0658]: associated type bounds are unstable
                //   --> examples/generics.rs:13:1
                //    |
                // 13 | #[enser::enser]
                //    | ^^^^^^^^^^^^^^^
                //    |
                //    = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
                //    = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
                //    = note: this error originates in the attribute macro `enser::enser` (in Nightly builds, run with -Z macro-backtrace for more info)
                // For more information about this error, try `rustc --explain E0658`.
                // ```
                //
                // Possibly one of the serde generated impls.
                if let Some(where_clause) = where_clause.as_mut() {
                    let where_predicate_exists = where_clause
                        .predicates
                        .iter_mut()
                        .filter_map(|predicate| {
                            if let WherePredicate::Type(PredicateType {
                                bounded_ty: Type::Path(TypePath { path, .. }),
                                bounds,
                                ..
                            }) = predicate
                            {
                                if path.segments.len() == 1 {
                                    if let Some(PathSegment {
                                        ident: predicate_ident,
                                        ..
                                    }) = path.segments.last()
                                    {
                                        if type_param_ident == predicate_ident {
                                            // This where predicate is for the type param
                                            return Some(bounds);
                                        }
                                    }
                                }
                            }
                            None
                        })
                        // Only apply `Clone` to one predicate
                        .next()
                        .map(|bounds| {
                            bounds.push(parse_quote!(std::clone::Clone));
                            true
                        })
                        .unwrap_or(false);

                    if !where_predicate_exists {
                        // Need to add one for this type parameter
                        where_clause
                            .predicates
                            .push(parse_quote!(#type_param_ident: std::clone::Clone))
                    }
                } else {
                    // We need to attach the where clause
                    *where_clause = Some(parse_quote!(where #type_param_ident: std::clone::Clone,));
                }
            }
        }
    });

    let generics_split = the_enum.generics.split_for_impl();
    let the_enum_from_enser = impl_the_enum_from_enser(the_enum, &enser, &generics_split);
    let enser_from_the_enum = impl_enser_from_the_enum(the_enum, &enser, &generics_split);

    // Tell serde to serialize and deserialize with `enser`
    let ty_generics = &generics_split.0;
    let ty_generics = quote!(#ty_generics);
    let enser_enum_name = &enser.ident;
    let enser_enum_path_str = format!("{enser_mod}::{enser_enum_name}{ty_generics}");
    the_enum
        .attrs
        .push(parse_quote!(#[serde(from = #enser_enum_path_str, into = #enser_enum_path_str)]));

    quote! {
        #the_enum

        mod #enser_mod {
            // Imports `serde` or `{Deserialize, Serialize}`, depending on usage.
            use super::*;

            #enser

            #the_enum_from_enser

            #enser_from_the_enum
        }
    }
}

fn bounds_contains_clone<'bounds>(
    mut bounds: impl Iterator<Item = &'bounds TypeParamBound>,
) -> bool {
    bounds.any(|bound| {
        matches!(
                    bound,
                    TypeParamBound::Trait(TraitBound { path, .. }) if matches!(
                        path.segments.last(),
                        Some(PathSegment { ident, .. }) if ident == "Clone"))
    })
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

fn impl_the_enum_from_enser(
    the_enum: &ItemEnum,
    enser: &ItemEnum,
    generics_split: &(ImplGenerics<'_>, TypeGenerics<'_>, Option<&WhereClause>),
) -> proc_macro2::TokenStream {
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

    let (impl_generics, ty_generics, where_clause) = generics_split;

    quote! {
        impl #impl_generics From<#enser_enum_name #ty_generics> for #the_enum_name #ty_generics #where_clause {
            fn from(enser_enum: #enser_enum_name #ty_generics) -> Self {
                match enser_enum {
                    #(#variant_mappings),*
                }
            }
        }
    }
}

fn impl_enser_from_the_enum(
    the_enum: &ItemEnum,
    enser: &ItemEnum,
    generics_split: &(ImplGenerics<'_>, TypeGenerics<'_>, Option<&WhereClause>),
) -> proc_macro2::TokenStream {
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

    let (impl_generics, ty_generics, where_clause) = generics_split;

    quote! {
        impl #impl_generics From<#the_enum_name #ty_generics> for #enser_enum_name #ty_generics #where_clause {
            fn from(the_enum: #the_enum_name #ty_generics) -> Self {
                match the_enum {
                    #(#variant_mappings),*
                }
            }
        }
    }
}
