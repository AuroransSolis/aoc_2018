#![feature(test, proc_macro_hygiene)]

use std::slice;

const R: usize = 250;

fn main() {
    let input = include_str!("../input.txt");
    let input = input.lines().collect::<Vec<&str>>();
    part_1(&input);
    part_2();
}

pub fn part_1(input: &Vec<&str>) {
    let mut triples = 0;
    let mut doubles = 0;
    'names: for name in input {
        let slice = name.as_bytes();
        let mut ct = false;
        let mut cd = false;
        for letter in 0..26 {
            let mut count = 0;
            for i in 0..slice.len() {
                if slice[i] == 'a' as u8 + letter {
                    count += 1;
                }
            }
            if count == 3 && !ct {
                triples += 1;
                ct = true;
                if cd {
                    continue 'names;
                }
            } else if count == 2 && !cd {
                doubles += 1;
                cd = true;
                if ct {
                    continue 'names;
                }
            }
        }
    }
    println!("{}", triples * doubles);
}

extern crate packed_simd;
extern crate ugly_array_decl;

use std::hint::unreachable_unchecked;

pub use packed_simd::{u8x32, m8x32, m8};

pub use ugly_array_decl::ugly_array_decl;

const BYTES: [u8; 6750] = *include_bytes!("../input.txt");

pub fn part_2() {
    let zeros = u8x32::splat(0);
    let ones = u8x32::splat(1);
    let mut inputs: [u8x32; R] = unsafe { std::mem::uninitialized() };
    let mask: m8x32 = [
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(true),
        m8::new(false),
        m8::new(false),
        m8::new(false),
        m8::new(false),
        m8::new(false),
        m8::new(false),
    ].into();
    for i in 0..R {
        unsafe {
            (&mut inputs[i] as *mut u8x32).write(mask.select(u8x32::from_slice_unaligned_unchecked(
                slice::from_raw_parts(&BYTES[i * 27] as *const u8, 26)
            ), zeros))
        }
    }
    for i in 0..R - 1 {
        for j in i + 1..R {
            if inputs[i].eq(inputs[j]).select(zeros, ones).wrapping_sum() == 1 {
                <[u8; 32]>::from(inputs[i]).iter()
                    .take(26).zip(<[u8; 32]>::from(inputs[i]).iter().take(26))
                    .filter(|&(c1, c2)| c1 == c2).map(|(c1, _)| print!("{}", *c1 as char))
                    .collect::<()>();
                println!();
                return;
            }
        }
    }
    unsafe { unreachable_unchecked() };
}

extern crate test;

#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};

    use crate::{part_1, part_2, BYTES, ugly_array_decl, u8x32};

    #[bench]
    fn part_1_bench(b: &mut Bencher) {
        let input = include_str!("../input.txt");
        let input = input.lines().collect::<Vec<&str>>();
        b.iter(|| black_box(part_1(&input)));
    }

    #[bench]
    fn part_2_bench(b: &mut Bencher) {
        b.iter(|| black_box(part_2()));
    }
}