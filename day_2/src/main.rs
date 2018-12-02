#![feature(test, proc_macro_hygiene)]

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

use packed_simd::u8x32;

use ugly_array_decl::ugly_array_decl;

const BYTES: &[u8] = include_bytes!("../input.txt");

pub fn part_2() {
    let inputs = unsafe { ugly_array_decl!() };
    let zeros = u8x32::splat(0);
    let ones = u8x32::splat(1);
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
}

fn print_same(a: &str, b: &str) {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    for i in 0..a.len().min(b.len()) {
        if a_bytes[i] == 0 {
            continue;
        }
        if a_bytes[i] == b_bytes[i] {
            print!("{}", a_bytes[i] as char);
        }
    }
    println!();
}

extern crate test;

#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};

    use crate::{part_1, part_2};

    #[bench]
    fn part_1_bench(b: &mut Bencher) {
        let input = include_str!("../input.txt");
        let input = input.lines().collect::<Vec<&str>>();
        b.iter(|| black_box(part_1(&input)));
    }

    #[bench]
    fn part_2_bench(b: &mut Bencher) {
        let input = include_str!("../input.txt");
        let input = input.lines().collect::<Vec<&str>>();
        b.iter(|| black_box(part_2()));
    }
}