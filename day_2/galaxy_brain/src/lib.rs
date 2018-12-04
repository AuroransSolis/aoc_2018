#![feature(proc_macro_hygiene)]

extern crate ugly_array_decl;
extern crate proc_macro;
extern crate packed_simd;

use proc_macro::TokenStream;

#[proc_macro]
pub fn construct_galaxy_brain(_input: TokenStream) -> TokenStream {
    format!("const fn galactic_thonk() -> &'static str {{ \"{}\" }}", part_2_ff()).parse().unwrap()
}

use packed_simd::u8x32;
use ugly_array_decl::ugly_array_decl;

const BYTES: [u8; 6750] = *include_bytes!("../../zesterer-input.txt");
const LINES: usize = 250;
const ZEROS: u8x32 = u8x32::splat(0);
const ONES: u8x32 = u8x32::new(1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 0, 0, 0, 0, 0, 0);
const INPUTS: [u8x32; LINES] = ugly_array_decl!();

#[no_mangle]
#[inline(never)]
fn part_2_ff() -> String {
    for i in 0..LINES - 1 {
        for j in i + 1..LINES {
            if INPUTS[i].eq(INPUTS[j]).select(ONES, ZEROS).wrapping_sum() == 25 {
                let n1: [u8; 32] = INPUTS[i].into();
                let n2: [u8; 32] = INPUTS[j].into();
                let mut out = String::new();
                for n in 0..26 {
                    if n1[n] == n2[n] {
                        out.push(n1[n] as char);
                    }
                }
                return out;
            }
        }
    }
    unreachable!();
}