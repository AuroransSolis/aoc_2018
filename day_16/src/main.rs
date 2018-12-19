#![feature(test)]

extern crate packed_simd;

use packed_simd::{m8x64, u8x64};

fn main() {
    let input = include_str!("../input.txt");
    println!("p1: {}", part_1(input));
    //println!("p2: {}", part_2(part_2_lines.iter()));
}

const SHORT_OP_CHUNK_SIZE: usize = 51;
const LONG_OP_CHUNK_SIZE: usize = 52;
const SHORT_OP_OV1_OFFSET: usize = 23;
const LONG_OP_OV1_OFFSET: usize = 24;
const SHORT_OP_OV2_OFFSET: usize = 25;
const LONG_OP_OV2_OFFSET: usize = 26;
const SHORT_OP_OV3_OFFSET: usize = 27;
const LONG_OP_OV3_OFFSET: usize = 28;
const B_REG0_OFFSET: usize = 9;
const B_REG1_OFFSET: usize = 12;
const B_REG2_OFFSET: usize = 15;
const B_REG3_OFFSET: usize = 18;
const B_REGOFFSETS: [usize; 4] = [B_REG0_OFFSET, B_REG1_OFFSET, B_REG2_OFFSET, B_REG3_OFFSET];
const SHORT_OP_A_REG0_OFFSET: usize = 38;
const SHORT_OP_A_REG1_OFFSET: usize = 41;
const SHORT_OP_A_REG2_OFFSET: usize = 44;
const SHORT_OP_A_REG3_OFFSET: usize = 47;
const SHORT_OP_A_REGOFFSETS: [usize; 4] = [SHORT_OP_A_REG0_OFFSET, SHORT_OP_A_REG1_OFFSET,
    SHORT_OP_A_REG2_OFFSET, SHORT_OP_A_REG3_OFFSET];
const LONG_OP_A_REG0_OFFSET: usize = 39;
const LONG_OP_A_REG1_OFFSET: usize = 42;
const LONG_OP_A_REG2_OFFSET: usize = 45;
const LONG_OP_A_REG3_OFFSET: usize = 48;
const LONG_OP_A_REGOFFSETS: [usize; 4] = [LONG_OP_A_REG0_OFFSET, LONG_OP_A_REG1_OFFSET,
    LONG_OP_A_REG2_OFFSET, LONG_OP_A_REG3_OFFSET];

fn part_1(input: &str) -> usize {
    let mut o_v1 = [0; 751];
    let mut o_v2 = [0; 751];
    let mut ra = [0; 751];
    let mut rb = [0; 751];
    let mut rc = [0; 751];
    let mut bytes_ind = 0;
    let input_bytes = input.as_bytes();
    for info_ind in 0..751 {
        let mut long_op = false;
        o_v1[info_ind] = if input_bytes[bytes_ind + SHORT_OP_OV1_OFFSET] == b' ' {
            long_op = true;
            input_bytes[bytes_ind + LONG_OP_OV1_OFFSET]
        } else {
            input_bytes[bytes_ind + SHORT_OP_OV1_OFFSET]
        } - b'0';
        if long_op {
            o_v2[info_ind] = input_bytes[bytes_ind + LONG_OP_OV2_OFFSET] - b'0';
            let o_v3 = (input_bytes[bytes_ind + LONG_OP_OV3_OFFSET] - b'0') as usize;
            rc[info_ind] = if o_v3 < 4 {
                input_bytes[bytes_ind + LONG_OP_A_REGOFFSETS[o_v3]] - b'0'
            } else {
                253
            };
        } else {
            o_v2[info_ind] = input_bytes[bytes_ind + SHORT_OP_OV2_OFFSET] - b'0';
            let o_v3 = (input_bytes[bytes_ind + SHORT_OP_OV3_OFFSET] - b'0') as usize;
            rc[info_ind] = if o_v3 < 4 {
                input_bytes[bytes_ind + SHORT_OP_A_REGOFFSETS[o_v3]] - b'0'
            } else {
                253
            };
        }
        ra[info_ind] = if o_v1[info_ind] < 4 {
            input_bytes[bytes_ind + B_REGOFFSETS[o_v1[info_ind] as usize]] - b'0'
        } else {
            255
        };
        rb[info_ind] = if o_v2[info_ind] < 4 {
            input_bytes[bytes_ind + B_REGOFFSETS[o_v2[info_ind] as usize]] - b'0'
        } else {
            255
        };
        if long_op {
            bytes_ind += LONG_OP_CHUNK_SIZE;
        } else {
            bytes_ind += SHORT_OP_CHUNK_SIZE;
        }
    }
    let mut total = 0;
    for i in (0..o_v1.len()).step_by(64) {
        // Y'all mind if I  S I M D ?
        let (op_v1_simd, op_v2_simd, op_ra_simd, op_rb_simd, op_rc_simd) = if i + 64 > o_v1.len() {
            let mut op_v1_slice = [255; 64];
            let mut op_v2_slice = [254; 64];
            let mut op_ra_slice = [252; 64];
            let mut op_rb_slice = [251; 64];
            let mut op_rc_slice = [250; 64];
            for j in i..o_v1.len() {
                op_v1_slice[j - i] = o_v1[j];
                op_v2_slice[j - i] = o_v2[j];
                op_ra_slice[j - i] = ra[j];
                op_rb_slice[j - i] = rb[j];
                op_rc_slice[j - i] = rc[j];
            }
            unsafe {(
                u8x64::from_slice_unaligned_unchecked(&op_v1_slice),
                u8x64::from_slice_unaligned_unchecked(&op_v2_slice),
                u8x64::from_slice_unaligned_unchecked(&op_ra_slice),
                u8x64::from_slice_unaligned_unchecked(&op_rb_slice),
                u8x64::from_slice_unaligned_unchecked(&op_rc_slice)
            )}
        } else {
            unsafe {(
                u8x64::from_slice_unaligned_unchecked(&o_v1[i..i + 64]),
                u8x64::from_slice_unaligned_unchecked(&o_v2[i..i + 64]),
                u8x64::from_slice_unaligned_unchecked(&ra[i..i + 64]),
                u8x64::from_slice_unaligned_unchecked(&rb[i..i + 64]),
                u8x64::from_slice_unaligned_unchecked(&rc[i..i + 64])
            )}
        };
        // All of the functions matching operation names do the operation and check if it's valid.
        // This results in m8x64 SIMD vecs, which I call `.select()` on to get to be u8x64 SIMD vecs
        // of ones and zeros. I then call '.gt(TWOS)' to get a m8x64 of which ones had a count
        // greater than two, then finally call `.select()` and `.wrapping_sum()` on that to get how
        // many of the 64 inputs had more than three possible operations.
        total += (addr(op_ra_simd, op_rb_simd, op_rc_simd).select(ONES, ZEROS)
            + addi(op_ra_simd, op_v2_simd, op_rc_simd).select(ONES, ZEROS)
            + mulr(op_ra_simd, op_rb_simd, op_rc_simd).select(ONES, ZEROS)
            + muli(op_ra_simd, op_v2_simd, op_rc_simd).select(ONES, ZEROS)
            + banr(op_ra_simd, op_rb_simd, op_rc_simd).select(ONES, ZEROS)
            + bani(op_ra_simd, op_v2_simd, op_rc_simd).select(ONES, ZEROS)
            + borr(op_ra_simd, op_rb_simd, op_rc_simd).select(ONES, ZEROS)
            + bori(op_ra_simd, op_v2_simd, op_rc_simd).select(ONES, ZEROS)
            + setr(op_ra_simd, ZEROS, op_rc_simd).select(ONES, ZEROS)
            + seti(op_v1_simd, ZEROS, op_rc_simd).select(ONES, ZEROS)
            + gtir(op_v1_simd, op_rb_simd, op_rc_simd).select(ONES, ZEROS)
            + gtri(op_ra_simd, op_v2_simd, op_rc_simd).select(ONES, ZEROS)
            + gtrr(op_ra_simd, op_rb_simd, op_rc_simd).select(ONES, ZEROS)
            + eqir(op_v1_simd, op_rb_simd, op_rc_simd).select(ONES, ZEROS)
            + eqri(op_ra_simd, op_v2_simd, op_rc_simd).select(ONES, ZEROS)
            + eqrr(op_ra_simd, op_rb_simd, op_rc_simd).select(ONES, ZEROS))
            .gt(TWOS).select(ONES, ZEROS).wrapping_sum() as usize;
    }
    total
}

/*fn part_2(mut input: impl Iterator<Item = &str>) -> usize {

}*/

const TWOS: u8x64 = u8x64::splat(2);
const ONES: u8x64 = u8x64::splat(1);
const ZEROS: u8x64 = u8x64::splat(0);

// Yes, I'm aware some of the following functions are duplicates. They're there for completeness.

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

extern crate test;

use test::{Bencher, black_box};

#[bench]
fn p1(b: &mut Bencher) {
    let input = include_str!("../input.txt");
    b.iter(|| black_box(part_1(input)));
}