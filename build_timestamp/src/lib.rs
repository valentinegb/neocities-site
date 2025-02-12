extern crate proc_macro;

use std::time::UNIX_EPOCH;

use proc_macro::TokenStream;

#[proc_macro]
pub fn build_timestamp(_item: TokenStream) -> TokenStream {
    UNIX_EPOCH
        .elapsed()
        .unwrap()
        .as_secs()
        .to_string()
        .parse()
        .unwrap()
}
