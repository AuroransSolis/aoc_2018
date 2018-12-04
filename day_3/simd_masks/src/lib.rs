extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn m8x32_masks(_input: TokenStream) -> TokenStream {
    let mut out = "[m8x32::new(false".to_string();
    for _ in 0..32 {
        out += ", false";
    }
    for i in 1..32 {
        out += "), m8x32::new(true";
        for _ in 1..i {
            out += ", true";
        }
        for _ in i..32 {
            out += ", false"
        }
    }
    out += ")]";
    out.parse().unwrap()
}