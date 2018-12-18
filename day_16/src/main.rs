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
    // b_rn vectors hold the nth value in 'Before' lines
    // o_vn vectors hold the nth value in operation lines
    // a_rn vectors hold the nth value in 'After' lines
    // ra holds the values that operation lines point to using the first value as a reference to a
    //     register
    // rb holds the values that operation lines point to using the second value as a reference to a
    //     register
    let mut b_r0 = Vec::new();
    let mut o_v0 = Vec::new();
    let mut a_r0 = Vec::new();
    let mut b_r1 = Vec::new();
    let mut o_v1 = Vec::new();
    let mut a_r1 = Vec::new();
    let mut b_r2 = Vec::new();
    let mut o_v2 = Vec::new();
    let mut a_r2 = Vec::new();
    let mut b_r3 = Vec::new();
    let mut o_v3 = Vec::new();
    let mut a_r3 = Vec::new();
    let mut ra = Vec::new();
    let mut rb = Vec::new();
    while let Some(line) = input.next() {
        // Should only push to ra and rb if it's not a 'Before' or 'After' line
        let mut rarb = false;
        {
            let (which_v0, which_v1, which_v2, which_v3, useful) = match line.as_ref()
                .as_bytes()[0] {
                b'B' => {
                    (&mut b_r0, &mut b_r1, &mut b_r2, &mut b_r3,
                        line.as_ref().trim_start_matches("Before: [").trim_end_matches(']'))
                },
                b'A' => {
                    (&mut a_r0, &mut a_r1, &mut a_r2, &mut a_r3,
                        line.as_ref().trim_start_matches("After:  [").trim_end_matches(']'))
                },
                _ => {
                    rarb = true;
                    (&mut o_v0, &mut o_v1, &mut o_v2, &mut o_v3, line.as_ref())
                }
            };
            let mut words_iter = useful.split_whitespace();
            which_v0.push(words_iter.next().unwrap().trim_end_matches(',').parse::<u8>().unwrap());
            which_v1.push(words_iter.next().unwrap().as_bytes()[0] - b'0');
            which_v2.push(words_iter.next().unwrap().as_bytes()[0] - b'0');
            which_v3.push(words_iter.next().unwrap().as_bytes()[0] - b'0');
        }
        if rarb {
            ra.push(match o_v1[o_v1.len() - 1] {
                0 => b_r0[b_r0.len() - 1],
                1 => b_r1[b_r1.len() - 1],
                2 => b_r2[b_r2.len() - 1],
                3 => b_r3[b_r3.len() - 1],
                _ => 255
            });
            rb.push(match o_v2[o_v2.len() - 1] {
                0 => b_r0[b_r0.len() - 1],
                1 => b_r1[b_r1.len() - 1],
                2 => b_r2[b_r2.len() - 1],
                3 => b_r3[b_r3.len() - 1],
                _ => 255
            });
        }
    }
    let mut total = 0;
    for i in (0..b_r0.len()).step_by(64) {
        // Y'all mind if I  S I M D ?
        let (op_v1_simd, op_v2_simd, af_r3_simd, op_ra_simd, op_rb_simd) = if i + 64 > b_r0.len() {
            let mut op_v1_slice = [255; 64];
            let mut op_v2_slice = [254; 64];
            let mut af_r3_slice = [253; 64];
            let mut op_ra_slice = [252; 64];
            let mut op_rb_slice = [251; 64];
            for j in i..b_r0.len() {
                op_v1_slice[j - i] = o_v1[i];
                op_v2_slice[j - i] = o_v2[i];
                af_r3_slice[j - i] = a_r3[i];
                op_ra_slice[j - i] = ra[i];
                op_rb_slice[j - i] = rb[i];
            }
            unsafe {(
                u8x64::from_slice_unaligned_unchecked(&op_v1_slice),
                u8x64::from_slice_unaligned_unchecked(&op_v2_slice),
                u8x64::from_slice_unaligned_unchecked(&af_r3_slice),
                u8x64::from_slice_unaligned_unchecked(&op_ra_slice),
                u8x64::from_slice_unaligned_unchecked(&op_rb_slice)
            )}
        } else {
            unsafe {(
                u8x64::from_slice_unaligned_unchecked(&o_v1[i..i + 64]),
                u8x64::from_slice_unaligned_unchecked(&o_v2[i..i + 64]),
                u8x64::from_slice_unaligned_unchecked(&a_r3[i..i + 64]),
                u8x64::from_slice_unaligned_unchecked(&ra[i..i + 64]),
                u8x64::from_slice_unaligned_unchecked(&rb[i..i + 64])
            )}
        };
        // All of the functions matching operation names do the operation and check if it's valid.
        // This results in m8x64 SIMD vecs, which I call `.select()` on to get to be u8x64 SIMD vecs
        // of ones and zeros. I then call '.gt(TWOS)' to get a m8x64 of which ones had a count
        // greater than two, then finally call `.select()` and `.wrapping_sum()` on that to get how
        // many of the 64 inputs had more than three possible operations.
        total += (addr(op_ra_simd, op_rb_simd, af_r3_simd).select(ONES, ZEROS)
            + addi(op_v1_simd, op_rb_simd, af_r3_simd).select(ONES, ZEROS)
            + mulr(op_ra_simd, op_rb_simd, af_r3_simd).select(ONES, ZEROS)
            + muli(op_v1_simd, op_rb_simd, af_r3_simd).select(ONES, ZEROS)
            + banr(op_ra_simd, op_rb_simd, af_r3_simd).select(ONES, ZEROS)
            + bani(op_v1_simd, op_rb_simd, af_r3_simd).select(ONES, ZEROS)
            + borr(op_ra_simd, op_rb_simd, af_r3_simd).select(ONES, ZEROS)
            + bori(op_v1_simd, op_rb_simd, af_r3_simd).select(ONES, ZEROS)
            + setr(op_ra_simd, ZEROS, af_r3_simd).select(ONES, ZEROS)
            + seti(op_v1_simd, ZEROS, af_r3_simd).select(ONES, ZEROS)
            + gtir(op_v1_simd, op_rb_simd, af_r3_simd).select(ONES, ZEROS)
            + gtri(op_ra_simd, op_v2_simd, af_r3_simd).select(ONES, ZEROS)
            + gtrr(op_ra_simd, op_rb_simd, af_r3_simd).select(ONES, ZEROS)
            + eqir(op_v1_simd, op_rb_simd, af_r3_simd).select(ONES, ZEROS)
            + eqri(op_ra_simd, op_v2_simd, af_r3_simd).select(ONES, ZEROS)
            + eqrr(op_ra_simd, op_rb_simd, af_r3_simd).select(ONES, ZEROS))
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

// Yes, I'm aware that the gtxx and eqxx functions are duplicates. They're just there for the sake
// of completeness.

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