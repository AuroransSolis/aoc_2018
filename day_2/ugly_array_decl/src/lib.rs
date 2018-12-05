extern crate proc_macro;

use proc_macro::TokenStream;

use std::fmt::Write;

#[proc_macro]
pub fn ugly_array_decl(_item: TokenStream) -> TokenStream {
    let mut out = "[u8x32::new(".to_string();
    out = format!("{}BYTES[0]", out);
    for i in 1..26 {
        out = format!("{}, BYTES[{}]", out, i);
    }
    for _ in 26..32 {
        out = format!("{}, 0", out);
    }
    out = format!("{})", out);
    for i in 1..250 {
        out = format!("{}, u8x32::new(", out);
        out = format!("{}BYTES[{}]", out, i * 27);
        for j in 1..26 {
            out = format!("{}, BYTES[{}]", out, i * 27 + j);
        }
        for _ in 26..32 {
            out = format!("{}, 0", out);
        }
        out = format!("{})", out);
    }
    out = format!("{}]", out);
    out.parse().unwrap()
}

#[proc_macro]
pub fn big_ugly_array_decl(_item: TokenStream) -> TokenStream {
    let mut out = "[u8x32::new(".to_string();
    out += "BYTES[0]";
    for i in 1..26 {
        write!(out, ", BYTES[{}]", i).unwrap();
    }
    for _ in 26..32 {
        out += ", 0";
    }
    out += ")";
    for i in 1..100_000 {
        out += ", u8x32::new(";
        write!(out, "BYTES[{}]", i * 27).unwrap();
        for j in 1..26 {
            write!(out, ", BYTES[{}]", i * 27 + j).unwrap();
        }
        for _ in 26..32 {
            out += ", 0";
        }
        out += ")";
    }
    out += "]";
    out.parse().unwrap()
}

#[proc_macro]
pub fn ugly_sum_array_decl(_item: TokenStream) -> TokenStream {
    let mut out = "[[BYTES[0] as u16".to_string();
    for i in 1..13 {
        out += &format!(" + BYTES[{}] as u16", i);
    }
    out += ", BYTES[13] as u16";
    for i in 14..26 {
        out += &format!(" + BYTES[{}] as u16", i);
    }
    out += "]";
    for r in 1..250 {
        out += &format!(", [BYTES[{}] as u16", r * 27);
        for i in r * 27 + 1..r * 27 + 13 {
            out += &format!(" + BYTES[{}] as u16", i);
        }
        out += &format!(", BYTES[{}] as u16", r * 26 + 13);
        for i in r * 27 + 14..(r + 1) * 27 - 1 {
            out += &format!(" + BYTES[{}] as u16", i);
        }
        out += "]";
    }
    out += "]";
    out.parse().unwrap()
}