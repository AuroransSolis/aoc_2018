#![feature(test)]

use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let input = input.to_string();
    let input_vals = input.lines().map(|line| line.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    part_1(&input_vals);
    part_2(&input_vals);
}

pub fn part_1(input: &Vec<i64>) {
    println!("{}", input.iter().sum::<i64>());
}

pub fn part_2(input: &Vec<i64>) {
    let mut total = 0;
    let mut starting_freqs = Vec::new();
    for val in input {
        total += val;
        starting_freqs.push(total);
    }
    let mut min_cycle = i64::max_value();
    let mut val_plus_ind = Vec::new();
    'a: for a in 0..starting_freqs.len() - 1 {
        for b in a + 1..starting_freqs.len() {
            let ba_diff = (starting_freqs[b] - starting_freqs[a]).abs();
            let cycle = ba_diff / total;
            if ba_diff % total == 0 && cycle <= min_cycle {
                if cycle < min_cycle {
                    val_plus_ind.clear();
                }
                min_cycle = cycle;
                val_plus_ind.push((starting_freqs[a], b, cycle));
            }
        }
    }
    let mut res_val = 0;
    let mut min_b = usize::max_value();
    for (val, b_ind, cycle) in val_plus_ind {
        if cycle != min_cycle {
            continue;
        } else {
            if b_ind < min_b {
                min_b = b_ind;
                res_val = val;
            }
        }
    }
    println!("{}", res_val);
}

extern crate test;

#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};

    use crate::{part_1, part_2};

    #[bench]
    fn part_1_bench(b: &mut Bencher) {
        let input = include_str!("../input.txt");
        let input = input.to_string();
        let input_vals = input.lines().map(|line| line.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        b.iter(|| black_box(part_1(&input_vals)));
    }

    #[bench]
    fn part_2_bench(b: &mut Bencher) {
        let input = include_str!("../input.txt");
        let input = input.to_string();
        let input_vals = input.lines().map(|line| line.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        b.iter(|| black_box(part_2(&input_vals)));
    }
}