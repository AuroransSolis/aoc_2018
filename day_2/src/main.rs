#![feature(test, proc_macro_hygiene)]
extern crate galaxy_brain;

use galaxy_brain::construct_galaxy_brain;

construct_galaxy_brain!();

fn main() {
    let input = include_str!("../fixed-big-input.txt");
    let input = input.lines().collect::<Vec<&str>>();
    part_1(&input);
    part_2_ff();
    part_2_fr();
    part_2_rf();
    part_2_rr();
    //original_zesterer(include_bytes!("../ameo-input.txt"));
    //println!("Completed original Zesterer function.");
    //improved_zesterer();
    //println!("Completed modified Zesterer function.");
}

#[no_mangle]
#[inline(never)]
pub fn part_1(input: &Vec<&str>) {
    let mut triples = 0usize;
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
use ugly_array_decl::big_ugly_array_decl;

const BYTES: [u8; 2700000] = *include_bytes!("../fixed-big-input.txt");
const LINES: usize = 100_000;
const ZEROS: u8x32 = u8x32::splat(0);
const ONES: u8x32 = u8x32::new(1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 0, 0, 0, 0, 0, 0);
const INPUTS: [u8x32; LINES] = big_ugly_array_decl!();

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

//use ugly_array_decl::ugly_sum_array_decl;

//const SUMS: [[u16; 2]; 250] = ugly_sum_array_decl!();

// Needs more work.
/*fn improved_zesterer() -> usize {
    for i in 0..250 {
        for j in i + 1..250 {
            if SUMS[i][0].wrapping_sub(SUMS[j][0]).min(1)
                + SUMS[i][1].wrapping_sub(SUMS[j][1]).min(1) == 1 {
                if (INPUTS[i] - INPUTS[j]).min(ONES).wrapping_sum() == 1 {
                    /*let n1: [u8; 32] = INPUTS[i].into();
                    let n2: [u8; 32] = INPUTS[j].into();
                    for n in 0..26 {
                        if n1[n] == n2[n] {
                            print!("{}", n1[n] as char)
                        }
                    }
                    println!();*/
                    return i;
                }
            }
        }
    }
    panic!();
    //unsafe { unreachable_unchecked() };
}*/

/*fn original_zesterer(l: &[u8]) -> usize {
    let mut hashes = [[0; 4]; 250];
    for (i, c) in l.chunks(27).enumerate() {
        hashes[i][0] = c[0..13].iter().map(|e| *e as u16).sum::<u16>();
        hashes[i][1] = c[13..26].iter().map(|e| *e as u16).sum::<u16>();
    }
    for i in 0..250 {
        if hashes[i][0] != SUMS[i][0] {
            println!("SUMS incorrect at: {} (0: {} vs {})", i, hashes[i][0], SUMS[i][0]);
        }
        if hashes[i][1] != SUMS[i][1] {
            println!("SUMS incorrect at: {} (1: {} vs {})", i, hashes[i][1], SUMS[i][1]);
        }
    }
    let mut rail = [unsafe { std::mem::uninitialized::<u8x32>() }; 250];
    for (i, c) in l.chunks(27).enumerate() {
        let mut ptr = unsafe { std::slice::from_raw_parts_mut(&mut rail[i] as *mut _ as *mut u8, 32) };
        ptr[0..26].copy_from_slice(&c[0..26]);
        ptr[26..32].copy_from_slice(&[0, 0, 0, 0, 0, 0])
    }
    for i in 0..250 {
        for j in i + 1..250 {
            if hashes[i][0].wrapping_sub(hashes[j][0]).min(1) +
                hashes[i][1].wrapping_sub(hashes[j][1]).min(1) == 1 {
                if (rail[i] - rail[j]).min(u8x32::splat(1)).wrapping_sum() == 1 {
                    return i;
                }
            }
        }
    }
    return 0;
}*/

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

#[bench]
fn galaxy_brain(b: &mut Bencher) {
    b.iter(|| black_box(galactic_thonk()));
}

/*#[bench]
fn zesterer(b: &mut Bencher) {
    b.iter(|| black_box(improved_zesterer()));
}*/

/*#[bench]
fn og_zesterer(b: &mut Bencher) {
    let i = include_bytes!("../ameo-input.txt");
    b.iter(|| black_box(original_zesterer(i)));
}*/