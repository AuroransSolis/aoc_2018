#![feature(test)]

extern crate packed_simd;

use packed_simd::{m8x64, u8x64, u8x16};

use std::mem::uninitialized;

fn main() {
    let input = include_str!("../input.txt");
    println!("p1: {}", part_1(input));
    println!("p2: {:?}", part_2(input));
}

const INPUTS: usize = 751;

const SHORT_OP_CHUNK_SIZE: usize = 51;
const LONG_OP_CHUNK_SIZE: usize = 52;
const OP_OV0_OFFSET: usize = 21;
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
    let mut o_v1 = [0; INPUTS];
    let mut o_v2 = [0; INPUTS];
    let mut ra = [0; INPUTS];
    let mut rb = [0; INPUTS];
    let mut rc = [0; INPUTS];
    let mut bytes_ind = 0;
    let input_bytes = input.as_bytes();
    for info_ind in 0..INPUTS {
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
            op_v1_slice[0..o_v1.len() - i].copy_from_slice(&o_v1[i..]);
            op_v2_slice[0..o_v1.len() - i].copy_from_slice(&o_v2[i..]);
            op_ra_slice[0..o_v1.len() - i].copy_from_slice(&ra[i..]);
            op_rb_slice[0..o_v1.len() - i].copy_from_slice(&rb[i..]);
            op_rc_slice[0..o_v1.len() - i].copy_from_slice(&rc[i..]);
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
        total += (simd_addr(op_ra_simd, op_rb_simd, op_rc_simd).select(ONES, ZEROS)
            + simd_addi(op_ra_simd, op_v2_simd, op_rc_simd).select(ONES, ZEROS)
            + simd_mulr(op_ra_simd, op_rb_simd, op_rc_simd).select(ONES, ZEROS)
            + simd_muli(op_ra_simd, op_v2_simd, op_rc_simd).select(ONES, ZEROS)
            + simd_banr(op_ra_simd, op_rb_simd, op_rc_simd).select(ONES, ZEROS)
            + simd_bani(op_ra_simd, op_v2_simd, op_rc_simd).select(ONES, ZEROS)
            + simd_borr(op_ra_simd, op_rb_simd, op_rc_simd).select(ONES, ZEROS)
            + simd_bori(op_ra_simd, op_v2_simd, op_rc_simd).select(ONES, ZEROS)
            + simd_setr(op_ra_simd, ZEROS, op_rc_simd).select(ONES, ZEROS)
            + simd_seti(op_v1_simd, ZEROS, op_rc_simd).select(ONES, ZEROS)
            + simd_gtir(op_v1_simd, op_rb_simd, op_rc_simd).select(ONES, ZEROS)
            + simd_gtri(op_ra_simd, op_v2_simd, op_rc_simd).select(ONES, ZEROS)
            + simd_gtrr(op_ra_simd, op_rb_simd, op_rc_simd).select(ONES, ZEROS)
            + simd_eqir(op_v1_simd, op_rb_simd, op_rc_simd).select(ONES, ZEROS)
            + simd_eqri(op_ra_simd, op_v2_simd, op_rc_simd).select(ONES, ZEROS)
            + simd_eqrr(op_ra_simd, op_rb_simd, op_rc_simd).select(ONES, ZEROS))
            .gt(TWOS).select(ONES, ZEROS).wrapping_sum() as usize;
    }
    total
}

macro_rules! simd_possibilities_to_array {
    ($($slice:ident, $simd_fn:ident($a:ident, $b:ident, $c:ident)),*) => {$(
        let mut $slice: [u8; 64] = unsafe { uninitialized() };
        unsafe {
            $simd_fn($a, $b, $c).select(ONES, ZEROS).write_to_slice_unaligned_unchecked(&mut $slice)
        };
    )*}
}

macro_rules! mark_zero {
    ({$($op:ident, $number:expr),*}, $mz:ident, $possibilities:ident) => {$(
        if $op.is_some() && !$mz[$number] {
            for i in 0..INPUTS {
                $possibilities[i][$number] = 0;
            }
            $mz[$number] = true;
        }
    )*}
}

const PROGRAM_LINES: usize = 857;

fn part_2(input: &str) {
    let mut addr_op = None;
    let mut addi_op = None;
    let mut mulr_op = None;
    let mut muli_op = None;
    let mut banr_op = None;
    let mut bani_op = None;
    let mut borr_op = None;
    let mut bori_op = None;
    let mut setr_op = None;
    let mut seti_op = None;
    let mut gtir_op = None;
    let mut gtri_op = None;
    let mut gtrr_op = None;
    let mut eqir_op = None;
    let mut eqri_op = None;
    let mut eqrr_op = None;
    let mut possibilities: [[u8; 16]; INPUTS] = unsafe { std::mem::uninitialized() };
    let mut o_v0 = [0; INPUTS];
    let mut o_v1 = [0; INPUTS];
    let mut o_v2 = [0; INPUTS];
    let mut ra = [0; INPUTS];
    let mut rb = [0; INPUTS];
    let mut rc = [0; INPUTS];
    let mut bytes_ind = 0;
    let input_bytes = input.as_bytes();
    for info_ind in 0..INPUTS {
        let mut long_op = false;
        o_v0[info_ind] = if input_bytes[bytes_ind + OP_OV0_OFFSET + 1] != b' ' {
            long_op = true;
            10 + input_bytes[bytes_ind + OP_OV0_OFFSET + 1] - b'0'
        } else {
            input_bytes[bytes_ind + OP_OV0_OFFSET] - b'0'
        };
        if long_op {
            o_v1[info_ind] = input_bytes[bytes_ind + LONG_OP_OV1_OFFSET] - b'0';
            o_v2[info_ind] = input_bytes[bytes_ind + LONG_OP_OV2_OFFSET] - b'0';
            let o_v3 = (input_bytes[bytes_ind + LONG_OP_OV3_OFFSET] - b'0') as usize;
            rc[info_ind] = if o_v3 < 4 {
                input_bytes[bytes_ind + LONG_OP_A_REGOFFSETS[o_v3]] - b'0'
            } else {
                253
            };
        } else {
            o_v1[info_ind] = input_bytes[bytes_ind + SHORT_OP_OV1_OFFSET] - b'0';
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
    bytes_ind += 3; // Start of example program
    for i in (0..o_v1.len()).step_by(64) {
        let (op_v1_simd, op_v2_simd, op_ra_simd, op_rb_simd, op_rc_simd, amt) = if i + 64 > o_v1.len() {
            let mut op_v1_slice = [255; 64];
            let mut op_v2_slice = [254; 64];
            let mut op_ra_slice = [252; 64];
            let mut op_rb_slice = [251; 64];
            let mut op_rc_slice = [250; 64];
            op_v1_slice[0..o_v1.len() - i].copy_from_slice(&o_v1[i..]);
            op_v2_slice[0..o_v1.len() - i].copy_from_slice(&o_v2[i..]);
            op_ra_slice[0..o_v1.len() - i].copy_from_slice(&ra[i..]);
            op_rb_slice[0..o_v1.len() - i].copy_from_slice(&rb[i..]);
            op_rc_slice[0..o_v1.len() - i].copy_from_slice(&rc[i..]);
            unsafe {(
                u8x64::from_slice_unaligned_unchecked(&op_v1_slice),
                u8x64::from_slice_unaligned_unchecked(&op_v2_slice),
                u8x64::from_slice_unaligned_unchecked(&op_ra_slice),
                u8x64::from_slice_unaligned_unchecked(&op_rb_slice),
                u8x64::from_slice_unaligned_unchecked(&op_rc_slice),
                o_v1.len() - i
            )}
        } else {
            unsafe {(
                u8x64::from_slice_unaligned_unchecked(&o_v1[i..i + 64]),
                u8x64::from_slice_unaligned_unchecked(&o_v2[i..i + 64]),
                u8x64::from_slice_unaligned_unchecked(&ra[i..i + 64]),
                u8x64::from_slice_unaligned_unchecked(&rb[i..i + 64]),
                u8x64::from_slice_unaligned_unchecked(&rc[i..i + 64]),
                64
            )}
        };
        simd_possibilities_to_array!{
            addr_possibilities, simd_addr(op_ra_simd, op_rb_simd, op_rc_simd),
            addi_possibilities,  simd_addi(op_ra_simd, op_v2_simd, op_rc_simd),
            mulr_possibilities,  simd_mulr(op_ra_simd, op_rb_simd, op_rc_simd),
            muli_possibilities,  simd_muli(op_ra_simd, op_v2_simd, op_rc_simd),
            banr_possibilities,  simd_banr(op_ra_simd, op_rb_simd, op_rc_simd),
            bani_possibilities,  simd_bani(op_ra_simd, op_v2_simd, op_rc_simd),
            borr_possibilities,  simd_borr(op_ra_simd, op_rb_simd, op_rc_simd),
            bori_possibilities,  simd_bori(op_ra_simd, op_v2_simd, op_rc_simd),
            setr_possibilities,  simd_setr(op_ra_simd, ZEROS, op_rc_simd),
            seti_possibilities,  simd_seti(op_v1_simd, ZEROS, op_rc_simd),
            gtir_possibilities,  simd_gtir(op_v1_simd, op_rb_simd, op_rc_simd),
            gtri_possibilities,  simd_gtri(op_ra_simd, op_v2_simd, op_rc_simd),
            gtrr_possibilities,  simd_gtrr(op_ra_simd, op_rb_simd, op_rc_simd),
            eqir_possibilities,  simd_eqir(op_v1_simd, op_rb_simd, op_rc_simd),
            eqri_possibilities,  simd_eqir(op_ra_simd, op_v2_simd, op_rc_simd),
            eqrr_possibilities,  simd_eqrr(op_ra_simd, op_rb_simd, op_rc_simd)
        };
        for j in 0..amt {
            possibilities[i + j][0] = addr_possibilities[j];
            possibilities[i + j][1] = addi_possibilities[j];
            possibilities[i + j][2] = mulr_possibilities[j];
            possibilities[i + j][3] = muli_possibilities[j];
            possibilities[i + j][4] = banr_possibilities[j];
            possibilities[i + j][5] = bani_possibilities[j];
            possibilities[i + j][6] = borr_possibilities[j];
            possibilities[i + j][7] = bori_possibilities[j];
            possibilities[i + j][8] = setr_possibilities[j];
            possibilities[i + j][9] = seti_possibilities[j];
            possibilities[i + j][10] = gtir_possibilities[j];
            possibilities[i + j][11] = gtri_possibilities[j];
            possibilities[i + j][12] = gtrr_possibilities[j];
            possibilities[i + j][13] = eqir_possibilities[j];
            possibilities[i + j][14] = eqri_possibilities[j];
            possibilities[i + j][15] = eqrr_possibilities[j];
        }
    }
    let mut skip = [false; INPUTS];
    let mut identified = [0; 16];
    let mut marked_zero = [false; 16];
    'identification: loop {
        for (i, pos) in possibilities.iter().enumerate() {
            if skip[i] {
                continue;
            }
            if unsafe { u8x16::from_slice_unaligned_unchecked(pos).wrapping_sum() } == 1 {
                let which = pos.iter().position(|&e| e == 1).unwrap();
                if identified[which] == 1 {
                    continue;
                }
                match which {
                    0 => addr_op = Some(o_v0[i]),
                    1 => addi_op = Some(o_v0[i]),
                    2 => mulr_op = Some(o_v0[i]),
                    3 => muli_op = Some(o_v0[i]),
                    4 => banr_op = Some(o_v0[i]),
                    5 => bani_op = Some(o_v0[i]),
                    6 => borr_op = Some(o_v0[i]),
                    7 => bori_op = Some(o_v0[i]),
                    8 => setr_op = Some(o_v0[i]),
                    9 => seti_op = Some(o_v0[i]),
                    10 => gtir_op = Some(o_v0[i]),
                    11 => gtri_op = Some(o_v0[i]),
                    12 => gtrr_op = Some(o_v0[i]),
                    13 => eqir_op = Some(o_v0[i]),
                    14 => eqri_op = Some(o_v0[i]),
                    15 => eqrr_op = Some(o_v0[i]),
                    _ => unsafe { std::hint::unreachable_unchecked() }
                }
                skip[i] = true;
                identified[which] = 1;
                unsafe {
                    if u8x16::from_slice_unaligned_unchecked(&identified).wrapping_sum() == 16 {
                        break 'identification;
                    }
                }
            }
        }
        mark_zero!{
            {
                addr_op, 0, addi_op, 1, mulr_op, 2, muli_op, 3, banr_op, 4, bani_op, 5, borr_op, 6,
                bori_op, 7, setr_op, 8, seti_op, 9, gtir_op, 10, gtri_op, 11, gtrr_op, 12,
                eqir_op, 13, eqri_op, 14, eqrr_op, 15
            }, marked_zero, possibilities
        };
    }
    let mut example_program_lines: [[u16; 4]; 857] = unsafe { uninitialized() };
    for i in 0..857
}

const TWOS: u8x64 = u8x64::splat(2);
const ONES: u8x64 = u8x64::splat(1);
const ZEROS: u8x64 = u8x64::splat(0);

// Yes, I'm aware some of the following functions are duplicates. They're there for completeness.

fn simd_addr(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a + b).eq(c)
}

fn simd_addi(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a + b).eq(c)
}

fn simd_mulr(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a * b).eq(c)
}

fn simd_muli(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a * b).eq(c)
}

fn simd_banr(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a & b).eq(c)
}

fn simd_bani(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a & b).eq(c)
}

fn simd_borr(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a | b).eq(c)
}

fn simd_bori(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a | b).eq(c)
}

fn simd_setr(a: u8x64, _b: u8x64, c: u8x64) -> m8x64 {
    a.eq(c)
}

fn simd_seti(a: u8x64, _b: u8x64, c: u8x64) -> m8x64 {
    a.eq(c)
}

fn simd_gtir(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    a.gt(b).select(ONES, ZEROS).eq(c)
}

fn simd_gtri(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    a.gt(b).select(ONES, ZEROS).eq(c)
}

fn simd_gtrr(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    a.gt(b).select(ONES, ZEROS).eq(c)
}

fn simd_eqir(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    a.eq(b).select(ONES, ZEROS).eq(c)
}

fn simd_eqri(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    a.eq(b).select(ONES, ZEROS).eq(c)
}

fn simd_eqrr(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    a.eq(b).select(ONES, ZEROS).eq(c)
}

extern crate test;

use test::{Bencher, black_box};

#[bench]
fn p1(b: &mut Bencher) {
    let input = include_str!("../input.txt");
    b.iter(|| black_box(part_1(input)));
}

#[bench]
fn p2(b: &mut Bencher) {
    let input = include_str!("../input.txt");
    b.iter(|| black_box(part_2(input)));
}