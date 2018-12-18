extern crate packed_simd;

use packed_simd::{m8x64, u8x64};

fn main() {
    let input = include_str!("../input.txt");
    let mut part_1_lines = Vec::new();
    let mut part_2_lines = Vec::new();
    let mut on_part_1 = true;
    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }
        if on_part_1 {
            if line.as_bytes()[0] == b'B' {
                part_1_lines.push(line);
            } else if line.as_bytes()[0] == b'A' {
                part_1_lines.push(line);
            } else {
                if part_1_lines[part_1_lines.len() - 1].as_bytes()[0] != b'B' {
                    part_2_lines.push(line);
                    on_part_1 = false;
                } else {
                    part_1_lines.push(line);
                }
            }
        } else {
            part_2_lines.push(line);
        }
    }
    println!("p1: {}", part_1(part_1_lines.iter()));
    //println!("p2: {}", part_2(part_2_lines.iter()));
}

fn part_1<S: AsRef<str>, T: IntoIterator<Item = S>>(input: T) -> usize {
    let mut input = input.into_iter();
    let mut opcodes = Vec::new();
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    let mut v3 = Vec::new();
    let mut ra = Vec::new();
    let mut rb = Vec::new();
    while let Some(line) = input.next() {
        let mut print = false;
        let useful = match line.as_ref().as_bytes()[0] {
            b'B' => line.as_ref().trim_start_matches("Before: [").trim_end_matches(']'),
            b'A' => line.as_ref().trim_start_matches("After:  [").trim_end_matches(']'),
            _ => line.as_ref()
        };
        let mut words_iter = useful.split_whitespace();
        opcodes.push(words_iter.next().unwrap().trim_end_matches(',').parse::<u8>().unwrap());
        v1.push(words_iter.next().unwrap().as_bytes()[0] - b'0');
        v2.push(words_iter.next().unwrap().as_bytes()[0] - b'0');
        v3.push(words_iter.next().unwrap().as_bytes()[0] - b'0');
        ra.push(match v1[v1.len() - 1] {
            0 => opcodes[opcodes.len() - 1],
            1 => v1[v1.len() - 1],
            2 => v2[v2.len() - 1],
            3 => v3[v3.len() - 1],
            _ => 255
        });
        rb.push(match v2[v2.len() - 1] {
            0 => opcodes[opcodes.len() - 1],
            1 => v1[v1.len() - 1],
            2 => v2[v2.len() - 1],
            3 => v3[v3.len() - 1],
            _ => 255
        });
    }
    let mut total = 0;
    for i in (0..v1.len()).step_by(64) {
        let (v1_simd, v2_simd, v3_simd, ra_simd, rb_simd) = if i + 64 > v1.len() {
            let mut v1_slice = [255; 64];
            let mut v2_slice = [255; 64];
            let mut v3_slice = [255; 64];
            let mut ra_slice = [255; 64];
            let mut rb_slice = [255; 64];
            for j in i..opcodes.len() {
                v1_slice[j - i] = v1[i];
                v2_slice[j - i] = v2[i];
                v3_slice[j - i] = v3[i];
                ra_slice[j - i] = ra[i];
                rb_slice[j - i] = rb[i];
            }
            unsafe {(
                u8x64::from_slice_unaligned_unchecked(&v1_slice),
                u8x64::from_slice_unaligned_unchecked(&v2_slice),
                u8x64::from_slice_unaligned_unchecked(&v3_slice),
                u8x64::from_slice_unaligned_unchecked(&ra_slice),
                u8x64::from_slice_unaligned_unchecked(&rb_slice)
            )}
        } else {
            unsafe {(
                u8x64::from_slice_unaligned_unchecked(&v1[i..i + 64]),
                u8x64::from_slice_unaligned_unchecked(&v2[i..i + 64]),
                u8x64::from_slice_unaligned_unchecked(&v3[i..i + 64]),
                u8x64::from_slice_unaligned_unchecked(&ra[i..i + 64]),
                u8x64::from_slice_unaligned_unchecked(&rb[i..i + 64])
            )}
        };
        total += (addr(ra_simd, v2_simd, v3_simd).select(ONES, ZEROS)
            + addi(v1_simd, v2_simd, v3_simd).select(ONES, ZEROS)
            + mulr(ra_simd, v2_simd, v3_simd).select(ONES, ZEROS)
            + muli(v1_simd, v2_simd, v3_simd).select(ONES, ZEROS)
            + banr(ra_simd, v2_simd, v3_simd).select(ONES, ZEROS)
            + bani(v1_simd, v2_simd, v3_simd).select(ONES, ZEROS)
            + borr(ra_simd, v2_simd, v3_simd).select(ONES, ZEROS)
            + bori(v1_simd, v2_simd, v3_simd).select(ONES, ZEROS)
            + setr(ra_simd, ZEROS, v3_simd).select(ONES, ZEROS)
            + seti(v1_simd, ZEROS, v3_simd).select(ONES, ZEROS)
            + gtir(v1_simd, rb_simd, v3_simd).select(ONES, ZEROS)
            + gtri(ra_simd, v2_simd, v3_simd).select(ONES, ZEROS)
            + gtrr(ra_simd, rb_simd, v3_simd).select(ONES, ZEROS)
            + eqir(v1_simd, rb_simd, v3_simd).select(ONES, ZEROS)
            + eqri(ra_simd, v2_simd, v3_simd).select(ONES, ZEROS)
            + eqrr(ra_simd, rb_simd, v3_simd).select(ONES, ZEROS))
            .gt(TWOS).select(ONES, ZEROS).wrapping_sum() as usize;
    }
    total
}

/*fn part_2(mut input: impl Iterator<Item = &str>) -> usize {

}*/

const TWOS: u8x64 = u8x64::splat(2);
const ONES: u8x64 = u8x64::splat(1);
const ZEROS: u8x64 = u8x64::splat(0);

fn addr(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a + b).eq(c)
}

fn addi(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a + b).eq(c)
}

fn mulr(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a * b).eq(c)
}

fn muli(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a * b).eq(c)
}

fn banr(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a & b).eq(c)
}

fn bani(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a & b).eq(c)
}

fn borr(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a | b).eq(c)
}

fn bori(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a | b).eq(c)
}

fn setr(a: u8x64, _b: u8x64, c: u8x64) -> m8x64 {
    a.eq(c)
}

fn seti(a: u8x64, _b: u8x64, c: u8x64) -> m8x64 {
    a.eq(c)
}

fn gtir(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    a.gt(b).select(ONES, ZEROS).eq(c)
}

fn gtri(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    a.gt(b).select(ONES, ZEROS).eq(c)
}

fn gtrr(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    a.gt(b).select(ONES, ZEROS).eq(c)
}

fn eqir(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    a.eq(b).select(ONES, ZEROS).eq(c)
}

fn eqri(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    a.eq(b).select(ONES, ZEROS).eq(c)
}

fn eqrr(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    a.eq(b).select(ONES, ZEROS).eq(c)
}