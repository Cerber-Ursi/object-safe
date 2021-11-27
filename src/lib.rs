extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as pm2_ts;

#[proc_macro_attribute]
pub fn object_safe(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let input = pm2_ts::from(item);
  let output = input;
  output.into()
}
