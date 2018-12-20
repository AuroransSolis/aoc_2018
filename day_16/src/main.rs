#![feature(test)]

extern crate packed_simd;

use packed_simd::{m8x64, u8x64, u8x16};

use std::mem::uninitialized;

fn main() {
    let input = include_str!("../input.txt");
    println!("p1: {}", part_1(input));
    println!("p2: {}", part_2(input));
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
        } & 15;
        if long_op {
            o_v2[info_ind] = input_bytes[bytes_ind + LONG_OP_OV2_OFFSET] & 15;
            let o_v3 = (input_bytes[bytes_ind + LONG_OP_OV3_OFFSET] & 15) as usize;
           rc[info_ind] = if o_v3 < 4 {
                input_bytes[bytes_ind + LONG_OP_A_REGOFFSETS[o_v3]] & 15
            } else {
                253
            };
        } else {
            o_v2[info_ind] = input_bytes[bytes_ind + SHORT_OP_OV2_OFFSET] & 15;
            let o_v3 = (input_bytes[bytes_ind + SHORT_OP_OV3_OFFSET] & 15) as usize;
           rc[info_ind] = if o_v3 < 4 {
                input_bytes[bytes_ind + SHORT_OP_A_REGOFFSETS[o_v3]] & 15
            } else {
                253
            };
        }
       ra[info_ind] = if o_v1[info_ind] < 4 {
            input_bytes[bytes_ind + B_REGOFFSETS[o_v1[info_ind] as usize]] & 15
        } else {
            255
        };
       rb[info_ind] = if o_v2[info_ind] < 4 {
            input_bytes[bytes_ind + B_REGOFFSETS[o_v2[info_ind] as usize]] & 15
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

const PROGRAM_LINES: usize = 857;

const SHORT_V0_V1_OFFSET: usize = 2;
const LONG_V0_V1_OFFSET: usize = 3;
const SHORT_V0_V2_OFFSET: usize = 4;
const LONG_V0_V2_OFFSET: usize = 5;
const SHORT_V0_V3_OFFSET: usize = 6;
const LONG_V0_V3_OFFSET: usize = 7;
const SHORT_LINE_LENGTH: usize = 8;
const LONG_LINE_LENGTH: usize = 9;

const FUNCTIONS: [fn(&mut [u16; 4], u8, u8, u8); 16] = [addr, addi, mulr, muli, banr, bani, borr,
    bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr];

fn part_2(input: &str) -> u16 {
    let mut ops = [None; 16];
    let mut possibilities: [[u8; 16]; INPUTS] = [[1; 16]; INPUTS];
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
            10 + input_bytes[bytes_ind + OP_OV0_OFFSET + 1] & 15
        } else {
            input_bytes[bytes_ind + OP_OV0_OFFSET] & 15
        };
        if long_op {
            o_v1[info_ind] = input_bytes[bytes_ind + LONG_OP_OV1_OFFSET] & 15;
            o_v2[info_ind] = input_bytes[bytes_ind + LONG_OP_OV2_OFFSET] & 15;
            let o_v3 = (input_bytes[bytes_ind + LONG_OP_OV3_OFFSET] & 15) as usize;
            rc[info_ind] = if o_v3 < 4 {
                input_bytes[bytes_ind + LONG_OP_A_REGOFFSETS[o_v3]] & 15
            } else {
                253
            };
        } else {
            o_v1[info_ind] = input_bytes[bytes_ind + SHORT_OP_OV1_OFFSET] & 15;
            o_v2[info_ind] = input_bytes[bytes_ind + SHORT_OP_OV2_OFFSET] & 15;
            let o_v3 = (input_bytes[bytes_ind + SHORT_OP_OV3_OFFSET] & 15) as usize;
            rc[info_ind] = if o_v3 < 4 {
                input_bytes[bytes_ind + SHORT_OP_A_REGOFFSETS[o_v3]] & 15
            } else {
                253
            };
        }
        ra[info_ind] = if o_v1[info_ind] < 4 {
            input_bytes[bytes_ind + B_REGOFFSETS[o_v1[info_ind] as usize]] & 15
        } else {
            255
        };
        rb[info_ind] = if o_v2[info_ind] < 4 {
            input_bytes[bytes_ind + B_REGOFFSETS[o_v2[info_ind] as usize]] & 15
        } else {
            255
        };
        if long_op {
            bytes_ind += LONG_OP_CHUNK_SIZE;
        } else {
            bytes_ind += SHORT_OP_CHUNK_SIZE;
        }
    }
    bytes_ind += 2; // Start of example program
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
            addi_possibilities, simd_addi(op_ra_simd, op_v2_simd, op_rc_simd),
            mulr_possibilities, simd_mulr(op_ra_simd, op_rb_simd, op_rc_simd),
            muli_possibilities, simd_muli(op_ra_simd, op_v2_simd, op_rc_simd),
            banr_possibilities, simd_banr(op_ra_simd, op_rb_simd, op_rc_simd),
            bani_possibilities, simd_bani(op_ra_simd, op_v2_simd, op_rc_simd),
            borr_possibilities, simd_borr(op_ra_simd, op_rb_simd, op_rc_simd),
            bori_possibilities, simd_bori(op_ra_simd, op_v2_simd, op_rc_simd),
            setr_possibilities, simd_setr(op_ra_simd, ZEROS, op_rc_simd),
            seti_possibilities, simd_seti(op_v1_simd, ZEROS, op_rc_simd),
            gtir_possibilities, simd_gtir(op_v1_simd, op_rb_simd, op_rc_simd),
            gtri_possibilities, simd_gtri(op_ra_simd, op_v2_simd, op_rc_simd),
            gtrr_possibilities, simd_gtrr(op_ra_simd, op_rb_simd, op_rc_simd),
            eqir_possibilities, simd_eqir(op_v1_simd, op_rb_simd, op_rc_simd),
            eqri_possibilities, simd_eqir(op_ra_simd, op_v2_simd, op_rc_simd),
            eqrr_possibilities, simd_eqrr(op_ra_simd, op_rb_simd, op_rc_simd)
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
    'identification: for _ in 0..2 {
        for i in 0..possibilities.len() {
            if skip[i] {
                continue;
            } else if unsafe {
                u8x16::from_slice_unaligned_unchecked(&possibilities[i]).wrapping_sum()
            } == 1 {
                let which = possibilities[i].iter().position(|&e| e == 1).unwrap();
                if identified[which] == 1 {
                    continue;
                } else {
                    ops[o_v0[i] as usize] = Some(which);
                    skip[i] = true;
                    identified[which] = 1;
                    unsafe {
                        if u8x16::from_slice_unaligned_unchecked(&identified).wrapping_sum() == 16 {
                            break 'identification;
                        }
                    }
                    for j in (0..INPUTS).filter(|&j| j != i) {
                        if skip[j] {
                            continue;
                        } else {
                            possibilities[j][which] = 0;
                        }
                    }
                }
            }
        }
    }
    let mut organized_functions: [fn(&mut [u16; 4], u8, u8, u8); 16] = unsafe { uninitialized() };
    for i in 0..16 {
        organized_functions[i] = FUNCTIONS[ops[i].unwrap() as usize];
    }
    let mut registers = [0; 4];
    for _ in 0..PROGRAM_LINES {
        let mut long_v0 = false;
        let v0 = if input_bytes[bytes_ind + 1] != b' ' {
            long_v0 = true;
            10 + input_bytes[bytes_ind + 1] & 15
        } else {
            input_bytes[bytes_ind] & 15
        } as usize;
        let (v1, v2, v3) = if long_v0 {
            (input_bytes[bytes_ind + LONG_V0_V1_OFFSET] & 15,
            input_bytes[bytes_ind + LONG_V0_V2_OFFSET] & 15,
            input_bytes[bytes_ind + LONG_V0_V3_OFFSET] & 15)
        } else {
            (input_bytes[bytes_ind + SHORT_V0_V1_OFFSET] & 15,
            input_bytes[bytes_ind + SHORT_V0_V2_OFFSET] & 15,
            input_bytes[bytes_ind + SHORT_V0_V3_OFFSET] & 15)
        };
        organized_functions[v0](&mut registers, v1, v2, v3);
        if long_v0 {
            bytes_ind += LONG_LINE_LENGTH;
        } else {
            bytes_ind += SHORT_LINE_LENGTH;
        }
    }
    registers[0]
}

const TWOS: u8x64 = u8x64::splat(2);
const ONES: u8x64 = u8x64::splat(1);
const ZEROS: u8x64 = u8x64::splat(0);

// Yes, I'm aware some of the following functions are duplicates. They're there for completeness.

fn simd_addr(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a + b).eq(c)
}

fn addr(registers: &mut [u16; 4], a: u8, b: u8, c: u8) {
    let result = registers[a as usize] + registers[b as usize];
    registers[c as usize] = result;
}

fn simd_addi(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a + b).eq(c)
}

fn addi(registers: &mut [u16; 4], a: u8, b: u8, c: u8) {
    let result = registers[a as usize] + b as u16;
    registers[c as usize] = result;
}

fn simd_mulr(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a * b).eq(c)
}

fn mulr(registers: &mut [u16; 4], a: u8, b: u8, c: u8) {
    let result = registers[a as usize] * registers[b as usize];
    registers[c as usize] = result;
}

fn simd_muli(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a * b).eq(c)
}

fn muli(registers: &mut [u16; 4], a: u8, b: u8, c: u8) {
    let result = registers[a as usize] * b as u16;
    registers[c as usize] = result;
}

fn simd_banr(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a & b).eq(c)
}

fn banr(registers: &mut [u16; 4], a: u8, b: u8, c: u8) {
    let result = registers[a as usize] & registers[b as usize];
    registers[c as usize] = result;
}

fn simd_bani(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a & b).eq(c)
}
fn bani(registers: &mut [u16; 4], a: u8, b: u8, c: u8) {
    let result = registers[a as usize] & b as u16;
    registers[c as usize] = result;
}

fn simd_borr(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a | b).eq(c)
}

fn borr(registers: &mut [u16; 4], a: u8, b: u8, c: u8) {
    let result = registers[a as usize] | registers[b as usize];
    registers[c as usize] = result;
}

fn simd_bori(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    (a | b).eq(c)
}

fn bori(registers: &mut [u16; 4], a: u8, b: u8, c: u8) {
    let result = registers[a as usize] | b as u16;
    registers[c as usize] = result;
}

fn simd_setr(a: u8x64, _b: u8x64, c: u8x64) -> m8x64 {
    a.eq(c)
}

fn setr(registers: &mut [u16; 4], a: u8, _b: u8, c: u8) {
    let tmp = registers[a as usize];
    registers[c as usize] = tmp;
}

fn simd_seti(a: u8x64, _b: u8x64, c: u8x64) -> m8x64 {
    a.eq(c)
}

fn seti(registers: &mut [u16; 4], a: u8, _b: u8, c: u8) {
    registers[c as usize] = a as u16;
}

fn simd_gtir(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    a.gt(b).select(ONES, ZEROS).eq(c)
}

fn gtir(registers: &mut [u16; 4], a: u8, b: u8, c: u8) {
    if a as u16 > registers[b as usize] {
       registers[c as usize] = 1;
    } else {
       registers[c as usize] = 0;
    }
}

fn simd_gtri(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    a.gt(b).select(ONES, ZEROS).eq(c)
}

fn gtri(registers: &mut [u16; 4], a: u8, b: u8, c: u8) {
    if registers[a as usize] > b as u16 {
       registers[c as usize] = 1;
    } else {
       registers[c as usize] = 0;
    }
}

fn simd_gtrr(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    a.gt(b).select(ONES, ZEROS).eq(c)
}

fn gtrr(registers: &mut [u16; 4], a: u8, b: u8, c: u8) {
    if registers[a as usize] > registers[b as usize] {
       registers[c as usize] = 1;
    } else {
       registers[c as usize] = 0;
    }
}

fn simd_eqir(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    a.eq(b).select(ONES, ZEROS).eq(c)
}

fn eqir(registers: &mut [u16; 4], a: u8, b: u8, c: u8) {
    if a as u16 == registers[b as usize] {
       registers[c as usize] = 1;
    } else {
       registers[c as usize] = 0;
    }
}

fn simd_eqri(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    a.eq(b).select(ONES, ZEROS).eq(c)
}

fn eqri(registers: &mut [u16; 4], a: u8, b: u8, c: u8) {
    if registers[a as usize] == b as u16 {
       registers[c as usize] = 1;
    } else {
       registers[c as usize] = 0;
    }
}

fn simd_eqrr(a: u8x64, b: u8x64, c: u8x64) -> m8x64 {
    a.eq(b).select(ONES, ZEROS).eq(c)
}

fn eqrr(registers: &mut [u16; 4], a: u8, b: u8, c: u8) {
    if registers[a as usize] == registers[b as usize] {
       registers[c as usize] = 1;
    } else {
       registers[c as usize] = 0;
    }
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