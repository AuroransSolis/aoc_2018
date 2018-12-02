#![feature(test)]

fn main() {
    let input = include_str!("../input.txt");
    let input = input.lines().collect::<Vec<&str>>();
    part_1(&input);
    part_2(&input);
}

pub fn part_1(input: &Vec<&str>) {
    let mut triples = 0;
    let mut doubles = 0;
    for name in input {
        'triple: for letter in 0..26 {
            let mut sum = 0;
            let mut add = true;
            for c in name.chars() {
                if c == ('a' as u8 + letter) as char {
                    sum += 1;
                }
                if sum > 3 {
                    add = false;
                    break;
                }
            }
            if add && sum == 3 {
                triples += 1;
                break 'triple;
            }
        }
        'double: for letter in 0..26 {
            let mut sum = 0;
            let mut add = true;
            for c in name.chars() {
                if c == ('a' as u8 + letter) as char {
                    sum += 1;
                }
                if sum > 2 {
                    add = false;
                    break;
                }
            }
            if add && sum == 2 {
                doubles += 1;
                break 'double;
            }
        }
    }
    println!("{}", triples * doubles);
}

pub fn part_2(input: &Vec<&str>) {
    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            if num_different(input[i], input[j]) == 1 {
                print_same(input[i], input[j]);
            }
        }
    }
}

fn num_different(a: &str, b: &str) -> usize {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let mut different = 0;
    for i in 0..a_bytes.len().min(b_bytes.len()) {
        if a_bytes[i] != b_bytes[i] {
            different += 1;
        }
    }
    different
}

fn print_same(a: &str, b: &str) {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    for i in 0..a.len().min(b.len()) {
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
        b.iter(|| black_box(part_2(&input)));
    }
}