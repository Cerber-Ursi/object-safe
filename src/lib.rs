extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as pm2_ts;

use quote::quote;
use syn::{parse2, spanned::Spanned, FnArg, Ident, ItemTrait, TraitItem::*};

mod config;
mod tt_flatten;

use config::Config;
use darling::FromMeta;

#[proc_macro_attribute]
pub fn object_safe(attr: TokenStream, item: TokenStream) -> TokenStream {
    let stream = pm2_ts::from(item);
    let orig_trait: ItemTrait = match parse2(stream.clone()) {
        Ok(item) => item,
        Err(_) => {
            return syn::Error::new_spanned(stream, "Only traits can be made object-safe")
                .to_compile_error()
                .into()
        }
    };

    if let Some(bound) = orig_trait.supertraits.iter().find(|bound| {
        use syn::TypeParamBound::*;
        match bound {
            Trait(bound) => {
                bound
                    .path
                    .segments
                    .iter()
                    .any(|segment| match segment.arguments.clone() {
                        syn::PathArguments::AngleBracketed(args) => {
                            let mut tt: tt_flatten::TokenStreamFlatten =
                                quote::quote! { #args }.into();
                            tt.any(|item| match item {
                                proc_macro2::TokenTree::Ident(ident) => {
                                    ident.to_string() == "Self".to_string()
                                }
                                _ => false,
                            })
                        }
                        _ => false,
                    })
            }
            Lifetime(_) => false,
        }
    }) {
        return syn::Error::new_spanned(
            bound,
            "Trait with Self in supertrait generic parameters cannot be made object-safe",
        )
        .to_compile_error()
        .into();
    }

    let attr_args = syn::parse_macro_input!(attr as syn::AttributeArgs);
    let cfg = match Config::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    let mut new_trait = orig_trait.clone();
    let orig_trait_name = orig_trait.ident.clone();
    if let Some(where_clause) = orig_trait.generics.where_clause.clone() {
        if let (Some(pred), others) = splice_self_sized(&where_clause) {
            if cfg.allow_self_sized() {
                new_trait.generics.where_clause =
                    new_trait.generics.where_clause.and_then(|item| {
                        if others.len() == 0 {
                            None
                        } else {
                            Some(syn::WhereClause {
                                predicates: others.clone().into_iter().collect(),
                                ..item
                            })
                        }
                    });
            } else {
                // needed, since we don't require nightly (and cannot use `pred.span()` explicitly)
                return syn::Error::new_spanned(
                    pred,
                    "Trait with Self: Sized trait bound cannot be made object-safe",
                )
                .to_compile_error()
                .into();
            }
        }
    }

    let new_trait_name = match cfg.name() {
        Some(name) => quote::format_ident!("{}", name),
        None => quote::format_ident!("ObjectSafe{}", orig_trait_name),
    };
    new_trait.ident = new_trait_name.clone();
    new_trait.items.retain(|item| match item {
        Const(_) => false,
        Method(item) => check_obj_safe(item),
        Type(_) => false,
        Macro(_) => false,
        Verbatim(_) => false,
        _ => false,
    });

    let impl_items = new_trait
        .items
        .iter()
        .cloned()
        .map(|item| impl_trait_item(orig_trait_name.clone(), item));

    let output = quote! {
        #orig_trait
        #new_trait

        impl<T: #orig_trait_name> #new_trait_name for T {
            #(#impl_items)*
        }
    };
    output.into()
}

fn splice_self_sized(
    item: &syn::WhereClause,
) -> (Option<syn::WherePredicate>, Vec<syn::WherePredicate>) {
    let tmp: (Vec<_>, Vec<_>) = item
        .predicates
        .iter()
        .cloned()
        .partition(|pred| match pred {
            syn::WherePredicate::Type(syn::PredicateType {
                bounded_ty, bounds, ..
            }) => bound_self(bounded_ty) && bounds.iter().any(bound_sized),
            _ => false,
        });
    (tmp.0.get(0).cloned(), tmp.1)
}

fn bound_self(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(syn::TypePath { path, .. }) => {
            path.segments
                .last()
                .expect("Path in trait bound can't be empty")
                .ident
                .to_string()
                == "Self".to_string()
        }
        _ => false,
    }
}
fn bound_sized(bound: &syn::TypeParamBound) -> bool {
    match bound {
        syn::TypeParamBound::Trait(syn::TraitBound { path, .. }) => path
            .segments
            .iter()
            .any(|item| item.ident.to_string() == "Sized".to_string()),
        _ => false,
    }
}

fn check_obj_safe(item: &syn::TraitItemMethod) -> bool {
    let sig = &item.sig;
    if let Some(where_clause) = &sig.generics.where_clause {
        if splice_self_sized(where_clause).0.is_some() {
            return true;
        }
    }
    if !sig.generics.params.is_empty() {
        return false;
    }
    if let Some(FnArg::Receiver(_)) = sig.inputs.first() {
        let inputs = sig.inputs.clone();
        let inputs: tt_flatten::TokenStreamFlatten = quote! { #inputs }.into();
        let return_type = match &sig.output {
            syn::ReturnType::Type(_, ty) => Some(ty),
            _ => None,
        };
        let stream: tt_flatten::TokenStreamFlatten = quote! { #return_type }.into();
        // If all idents are not equal to "Self" - this will be object-safe
        // We don't try to traverse the type, we use raw token stream instead
        inputs.chain(stream).all(|item| match item {
            proc_macro2::TokenTree::Ident(ident) => ident.to_string() != "Self".to_string(),
            _ => true,
        })
    } else {
        // no receiver - not object-safe
        false
    }
}

fn impl_trait_item(orig_ident: Ident, trait_item: syn::TraitItem) -> syn::ImplItemMethod {
    let span = trait_item.span();
    match trait_item {
        Method(item) => {
            let syn::TraitItemMethod { attrs, sig, .. } = item;
            let name = sig.ident.clone();
            let inputs = sig.inputs.clone().into_iter().map(|item| match item {
                FnArg::Receiver(_) => quote! { self },
                FnArg::Typed(pat) => {
                    let pat = pat.pat;
                    quote! { #pat }
                }
            });
            parse2(quote::quote_spanned! {
                span =>
                #(#attrs)* #sig {
                    <Self as #orig_ident>::#name(#(#inputs),*)
                }
            })
            .expect("Internal error, macro generated wrong code for method impl")
        }
        _ => unreachable!(),
    }
}
