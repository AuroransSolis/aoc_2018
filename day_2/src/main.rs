#![feature(test, proc_macro_hygiene)]

fn main() {
    let input = include_str!("../input.txt");
    let input = input.lines().collect::<Vec<&str>>();
    part_1(&input);
    //part_2();
}

#[no_mangle]
#[inline(never)]
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

use packed_simd::u8x32;

use ugly_array_decl::ugly_array_decl;

const BYTES: [u8; 6750] = *include_bytes!("../input.txt");
//const CHARS_PER_LINE: usize = 27;
const LINES: usize = 250;
/*const MASK: m8x32 = m8x32::new(true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false,
    false, false, false, false, false);*/
const ZEROS: u8x32 = u8x32::splat(0);
const ONES: u8x32 = u8x32::new(1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 0, 0, 0, 0, 0, 0);
const INPUTS: [u8x32; LINES] = ugly_array_decl!();

#[no_mangle]
#[inline(never)]
pub fn part_2_rr() -> usize {
    for i in (0..LINES - 1).rev() {
        for j in (i + 1..LINES).rev() {
            if INPUTS[i].eq(INPUTS[j]).select(ONES, ZEROS).wrapping_sum() == 25 {
                let n1: [u8; 32] = INPUTS[i].into();
                let n2: [u8; 32] = INPUTS[j].into();
                for n in 0..26 {
                    if n1[n] != n2[n] {
                        return n;
                    }
                }
            }
        }
    }
    unsafe { unreachable_unchecked() };
}

#[no_mangle]
#[inline(never)]
pub fn part_2_rf() -> usize {
    for i in (0..LINES - 1).rev() {
        for j in i + 1..LINES {
            if INPUTS[i].eq(INPUTS[j]).select(ONES, ZEROS).wrapping_sum() == 25 {
                let n1: [u8; 32] = INPUTS[i].into();
                let n2: [u8; 32] = INPUTS[j].into();
                for n in 0..26 {
                    if n1[n] != n2[n] {
                        return n;
                    }
                }
            }
        }
    }
    unsafe { unreachable_unchecked() };
}

#[no_mangle]
#[inline(never)]
pub fn part_2_fr() -> usize {
    for i in 0..LINES - 1 {
        for j in (i + 1..LINES).rev() {
            if INPUTS[i].eq(INPUTS[j]).select(ONES, ZEROS).wrapping_sum() == 25 {
                let n1: [u8; 32] = INPUTS[i].into();
                let n2: [u8; 32] = INPUTS[j].into();
                for n in 0..26 {
                    if n1[n] != n2[n] {
                        return n;
                    }
                }
            }
        }
    }
    unsafe { unreachable_unchecked() };
}

#[no_mangle]
#[inline(never)]
pub fn part_2_ff() -> usize {
    for i in 0..LINES - 1 {
        for j in i + 1..LINES {
            if INPUTS[i].eq(INPUTS[j]).select(ONES, ZEROS).wrapping_sum() == 25 {
                let n1: [u8; 32] = INPUTS[i].into();
                let n2: [u8; 32] = INPUTS[j].into();
                for n in 0..26 {
                    if n1[n] != n2[n] {
                        return n;
                    }
                }
            }
        }
    }
    unsafe { unreachable_unchecked() };
}

extern crate test;

use test::{Bencher, black_box};

#[bench]
fn ff(b: &mut Bencher) {
    b.iter(|| black_box(part_2_ff()));
}

#[bench]
fn fr(b: &mut Bencher) {
    b.iter(|| black_box(part_2_fr()));
}

#[bench]
fn rf(b: &mut Bencher) {
    b.iter(|| black_box(part_2_rf()));
}

#[bench]
fn rr(b: &mut Bencher) {
    b.iter(|| black_box(part_2_rr()));
}