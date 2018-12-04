#![feature(test)]

use std::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
struct CountFabric {
    grid: [u8; 1000 * 1000]
}

impl CountFabric {
    fn new() -> Self {
        CountFabric {
            grid: [0; 1000 * 1000]
        }
    }
}

impl Index<usize> for CountFabric {
    type Output = [u8];

    fn index(&self, ind: usize) -> &[u8] {
        &self.grid[ind * 1000..(ind + 1) * 1000]
    }
}

impl IndexMut<usize> for CountFabric {
    fn index_mut(&mut self, ind: usize) -> &mut [u8] {
        &mut self.grid[ind * 1000..(ind + 1) * 1000]
    }
}

#[derive(Copy, Clone)]
struct Claim {
    id: usize,
    fl: usize,
    ft: usize,
    w: usize,
    h: usize
}

impl Claim {
    fn new(id: usize, fl: usize, ft: usize, w: usize, h: usize) -> Self {
        Claim {
            id,
            fl,
            ft,
            w,
            h
        }
    }
}

fn main() {
    let input_str = include_str!("../input.txt");
    let input_lines = input_str.lines().collect::<Vec<&str>>();
    part_1(&input_lines);
    part_2(&input_lines);
}

fn part_1(input: &Vec<&str>) {
    let mut cf = CountFabric::new();
    for input_line in input {
        let mut parts_iter = input_line.split_whitespace();
        drop((parts_iter.next().unwrap(), parts_iter.next().unwrap()));
        let lt_str = parts_iter.next().unwrap();
        let (from_left_str, comma_from_top_colon) = lt_str.split_at(
            lt_str.chars().position(|c| c == ',').unwrap()
        );
        let (_, from_top_str_colon) = comma_from_top_colon.split_at(1);
        let (from_top_str, _) = from_top_str_colon.split_at(from_top_str_colon.len() - 1);
        let from_left = from_left_str.parse::<usize>().unwrap();
        let from_top = from_top_str.parse::<usize>().unwrap();
        let wh_str = parts_iter.next().unwrap();
        let (width_str, x_height_colon) = wh_str.split_at(
            wh_str.chars().position(|c| c == 'x').unwrap()
        );
        let (_, height_str) = x_height_colon.split_at(1);
        let width = width_str.parse::<usize>().unwrap();
        let height = height_str.parse::<usize>().unwrap();
        for i in from_top..from_top + height {
            for j in from_left..from_left + width {
                cf[i][j] += 1;
            }
        }
    }
    let mut more_than_two = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if cf[i][j] >= 2 {
                more_than_two += 1;
            }
        }
    }
    println!("{}", more_than_two);
}

extern crate packed_simd;
extern crate simd_masks;

use packed_simd::{u8x32, m8x32};
use simd_masks::m8x32_masks;

const MASKS: [m8x32; 32] = m8x32_masks!();
const ZEROS: u8x32 = u8x32::splat(0);

fn part_2(input: &Vec<&str>) {
    let mut cf = CountFabric::new();
    let mut claims: [Claim; 1293] = unsafe { std::mem::uninitialized() };
    for (i, input_line) in input.iter().enumerate() {
        let mut parts_iter = input_line.split_whitespace();
        drop((parts_iter.next().unwrap(), parts_iter.next().unwrap()));
        let lt_str = parts_iter.next().unwrap();
        let (from_left_str, comma_from_top_colon) = lt_str.split_at(
            lt_str.chars().position(|c| c == ',').unwrap()
        );
        let (_, from_top_str_colon) = comma_from_top_colon.split_at(1);
        let (from_top_str, _) = from_top_str_colon.split_at(from_top_str_colon.len() - 1);
        let from_left = from_left_str.parse::<usize>().unwrap();
        let from_top = from_top_str.parse::<usize>().unwrap();
        let wh_str = parts_iter.next().unwrap();
        let (width_str, x_height_colon) = wh_str.split_at(
            wh_str.chars().position(|c| c == 'x').unwrap()
        );
        let (_, height_str) = x_height_colon.split_at(1);
        let width = width_str.parse::<usize>().unwrap();
        let height = height_str.parse::<usize>().unwrap();
        for i in from_top..from_top + height {
            for j in from_left..from_left + width {
                cf[i][j] += 1;
            }
        }
        claims[i] = Claim::new(i, from_left, from_top, width, height);
    }
    let mut threads: [std::thread::JoinHandle<()>; 4] = unsafe { std::mem::uninitialized() };
    for i in 0..4 {
        threads[i] = std::thread::spawn(move || {
            for claim in claims.iter().step_by(4) {
                for row in claim.ft..claim.ft + claim.h {
                    let row_vec = unsafe {
                        MASKS[claim.w].select(u8x32::from_slice_unaligned_unchecked(
                            cf[row][claim.fl..claim.fl + claim.w]
                        ), ZEROS).min(2).
                    };
                }
            }
        })
    }
    for i in 0..4 {
        threads[i].join().unwrap();
    }
    /*'claim: for c in claims.iter() {
        for i in c.ft..c.ft + c.h {
            for j in c.fl..c.fl + c.w {
                if cf[i][j] > 1 {
                    continue 'claim;
                }
            }
        }
        println!("Good ID: {}", c.id + 1);
    }*/
}

extern crate test;

use test::{Bencher, black_box};

#[bench]
fn p1(b: &mut Bencher) {
    let i = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    b.iter(|| part_1(&i));
}

#[bench]
fn p2(b: &mut Bencher) {
    let i = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    b.iter(|| part_2(&i));
}