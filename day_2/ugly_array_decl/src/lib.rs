extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn ugly_array_decl(_item: TokenStream) -> TokenStream {
    let mut out = "[u8x32::from_slice_unaligned_unchecked([".to_string();
    out = format!("{}BYTES[0]", out);
    for i in 1..27 {
        out = format!("{}, BYTES[{}]", out, i);
    }
    for _ in 27..32 {
        out = format!("{}, 0", out);
    }
    out = format!("{}].as_ref())", out);
    for i in 1..250 {
        out = format!("{}, u8x32::from_slice_unaligned_unchecked([", out);
        out = format!("{}BYTES[{}]", out, i * 27);
        for j in 1..27 {
            out = format!("{}, BYTES[{}]", out, i * 27 + j);
        }
        for _ in 27..32 {
            out = format!("{}, 0", out);
        }
        out = format!("{}].as_ref())", out);
    }
    out = format!("{}]", out);
    out.parse().unwrap()
}