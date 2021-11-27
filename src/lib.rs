extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as pm2_ts;

use quote::quote;
use syn::{parse2, spanned::Spanned, FnArg, Ident, ItemTrait, ReturnType, TraitItem::*};

#[proc_macro_attribute]
pub fn object_safe(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let stream = pm2_ts::from(item);
    let orig_trait: ItemTrait = match parse2(stream.clone()) {
        Ok(item) => item,
        Err(_) => {
            return syn::Error::new_spanned(stream, "Only traits can be made object-safe")
                .to_compile_error()
                .into()
        }
    };

    if let Some(where_clause) = orig_trait.generics.where_clause.clone() {
        if let Some((_, pred)) = find_self_sized(&where_clause) {
            // needed, since we don't require nightly (and cannot use `pred.span()` explicitly)
            return syn::Error::new_spanned(
                pred,
                "Trait with Self: Sized trait bound cannot be made object-safe",
            )
            .to_compile_error()
            .into();
        }
    }

    let mut new_trait = orig_trait.clone();
    let orig_trait_name = orig_trait.ident.clone();

    let new_trait_name = Ident::new(
        &format!("ObjectSafe{}", orig_trait_name),
        orig_trait_name.span(),
    );
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

fn find_self_sized(item: &syn::WhereClause) -> Option<(usize, &syn::WherePredicate)> {
    item.predicates
        .iter()
        .enumerate()
        .find(|(_, pred)| match pred {
            syn::WherePredicate::Type(syn::PredicateType {
                bounded_ty, bounds, ..
            }) => bound_self(bounded_ty) && bounds.iter().any(bound_sized),
            _ => false,
        })
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
        if find_self_sized(where_clause).is_some() {
            return true;
        }
    }
    if let Some(FnArg::Typed(_)) = sig.inputs.first() {
        false
    } else if let ReturnType::Type(_, return_type) = &sig.output {
        unimplemented!()
    } else {
        true
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
                    <Self as #orig_ident>::#name(#(#inputs)*)
                }
            })
            .expect("Internal error, macro generated wrong code for method impl")
        }
        _ => unreachable!(),
    }
}
