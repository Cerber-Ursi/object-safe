extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as pm2_ts;

use syn::{parse2, ItemTrait, Ident, TraitItem::*, spanned::Spanned, ReturnType, FnArg};
use quote::quote;

#[proc_macro_attribute]
pub fn object_safe(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let orig_trait: ItemTrait = match parse2(pm2_ts::from(item)) {
    Ok(item) => item,
    Err(err) => return err.to_compile_error().into(),
  };

  if let Some(where_clause) = orig_trait.generics.where_clause.clone() {
    if check_self_sized(&where_clause) {
      return quote::quote_spanned! {
        where_clause.span() =>
        compile_error!("Trait with Self: Sized trait bound cannot be made object-safe");
      }.into();
    }
  }

  let mut new_trait = orig_trait.clone();
  let orig_trait_name = orig_trait.ident.clone();

  let new_trait_name = Ident::new(&format!("ObjectSafe{}", orig_trait_name), orig_trait_name.span());
  new_trait.ident = new_trait_name.clone();
  new_trait.items.retain(|item| match item {
    Const(_) => false,
    Method(item) => check_obj_safe(item),
    Type(_) => false,
    Macro(_) => false,
    Verbatim(_) => false,
    _ => false,
  });

  let output = quote! {
    #orig_trait
    #new_trait

    impl<T: #orig_trait_name> #new_trait_name for T {

    }
  };
  output.into()
}

fn check_self_sized(item: &syn::WhereClause) -> bool {
  unimplemented!();
}

fn check_obj_safe(item: &syn::TraitItemMethod) -> bool {
  let sig = &item.sig;
  if let Some(where_clause) = &sig.generics.where_clause {
    if check_self_sized(where_clause) {
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
