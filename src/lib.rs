extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as pm2_ts;

use syn::{parse2, ItemTrait, Ident};
use quote::quote;

#[proc_macro_attribute]
pub fn object_safe(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let old_trait: ItemTrait = parse2(pm2_ts::from(item)).expect("Only traits can be made into object-safe variant");
  let mut new_trait = old_trait.clone();
  new_trait.ident = Ident::new(&format!("ObjectSafe_{}", old_trait.ident), old_trait.ident.span());
  let output = quote! { #new_trait };
  output.into()
}
