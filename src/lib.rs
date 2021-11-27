extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as pm2_ts;

use syn::{parse2, ItemTrait, Ident, TraitItem::*};
use quote::quote;

#[proc_macro_attribute]
pub fn object_safe(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let old_trait: ItemTrait = parse2(pm2_ts::from(item)).expect("Only traits can be made into object-safe variant");
  let mut new_trait = old_trait.clone();

  new_trait.ident = Ident::new(&format!("ObjectSafe_{}", old_trait.ident), old_trait.ident.span());
  new_trait.items.retain(|item| match item {
    Const(_) => false,
    Method(item) => check_obj_safe(item),
    Type(_) => false,
    Macro(_) => false,
    Verbatim(_) => false,
    _ => false,
  });

  let output = quote! {
    #old_trait
    #new_trait
  };
  output.into()
}

fn check_obj_safe(item: syn::TraitItemMethod) -> bool {
  let sig = item.sig;
  if let Some(where_clause) = sig.generics.where_clause {
    unimplemented!();
  } else if let Some(Typed(_)) = sig.inputs.first() {
    false
  } else if let Type(_, return_type) = sig.output {
    unimplemented!()
  } else {
    true
  }
}
