#![feature(test)]

extern crate bytecount;

fn main() {
    let input = include_str!("../input.txt");
    let input = input.lines().collect::<Vec<&str>>();
    part_1(&input);
    part_2(&input);
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

pub fn part_2(input: &Vec<&str>) {
    'i: for i in 0..input.len() - 1 {
        let a_bytes = input[i].as_bytes();
        'j: for j in i + 1..input.len() {
            let b_bytes = input[j].as_bytes();
            let mut different = 0;
            for i in 0..a_bytes.len().min(b_bytes.len()) {
                if a_bytes[i] != b_bytes[i] {
                    different += 1;
                }
                if different > 1 {
                    continue 'j;
                }
            }
            if different == 1 {
                print_same(input[i], input[j]);
                break 'i;
            }
        }
    }
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

    use crate::{part_1, part_2, cryze_part1};

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