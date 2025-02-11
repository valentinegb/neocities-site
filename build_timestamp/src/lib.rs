extern crate proc_macro;

use std::time::{SystemTime, UNIX_EPOCH};

use proc_macro::TokenStream;

#[proc_macro]
pub fn build_timestamp(_item: TokenStream) -> TokenStream {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string()
        .parse()
        .unwrap()
}
