mod consts;
pub mod key;
pub mod aes_cbc;
pub mod aes_ctr;
pub mod aes_gcm;
mod aad;

use self::consts::{S_BOX, INV_S_BOX};

pub(crate) fn gmul(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0;

    while a != 0 && b != 0 {
        if b & 1 != 0 {
            p ^= a;
        }

        let c = a & 0x80 != 0;
        a <<= 1;
        if c {
            a ^= 0x1b;
        }
        b >>= 1;
    }

    p
}

pub(crate) fn xor_block(input: &[u8], output: &mut Vec<u8>) {
    for i in 0..usize::min(input.len(), output.len()) {
        output[i] ^= input[i];
    }
}

pub(crate) fn split_blocks(bytes: Vec<u8>, block_size: usize) -> Vec<Vec<u8>> {
    let chunks = bytes.chunks(block_size);
    let mut blocks = Vec::with_capacity(chunks.len());

    for chunk in chunks {
        blocks.push(Vec::from(chunk));
    }

    blocks
}

pub(crate) fn add_padding(blocks: &mut Vec<Vec<u8>>) {
    let mut last = blocks.len() - 1;
    let mut last_len = blocks[last].len();
    let mut byte = 16 - last_len as u8;

    if byte == 0 {
        byte = 16;
        last_len = 0;
        last += 1;
        blocks.push(Vec::with_capacity(16));
    }

    for _ in last_len..16 {
        blocks[last].push(byte);
    }
}

pub(crate) fn remove_padding(state: &mut Vec<u8>) {
    let padding_len = *state.last().unwrap();
    state.truncate(state.len() - padding_len as usize);
}

pub(crate) fn rotate_bytes_left(vec: &mut Vec<u8>, start: usize, end: usize) {
    let tmp = vec[start];
    for i in start..end - 1 {
        vec[i] = vec[i + 1];
    }
    vec[end - 1] = tmp;
}

pub(crate) fn rotate_bytes_right(vec: &mut Vec<u8>, start: usize, end: usize) {
    let tmp = vec[end - 1];
    for i in (start + 1..end).rev() {
        vec[i] = vec[i - 1];
    }
    vec[start] = tmp;
}

pub(crate) fn sub_bytes(vec: &mut Vec<u8>) {
    for i in 0..vec.len() {
        vec[i] = S_BOX[vec[i] as usize];
    }
}

pub(crate) fn inv_sub_bytes(vec: &mut Vec<u8>) {
    for i in 0..vec.len() {
        vec[i] = INV_S_BOX[vec[i] as usize];
    }
}

pub(crate) fn shift_rows(vec: &mut Vec<u8>) {
    rotate_bytes_left(vec, 4, 8);
    rotate_bytes_left(vec, 8, 12);
    rotate_bytes_left(vec, 8, 12);
    rotate_bytes_left(vec, 12, 16);
    rotate_bytes_left(vec, 12, 16);
    rotate_bytes_left(vec, 12, 16);
}

pub(crate) fn inv_shift_rows(vec: &mut Vec<u8>) {
    rotate_bytes_right(vec, 4, 8);
    rotate_bytes_right(vec, 8, 12);
    rotate_bytes_right(vec, 8, 12);
    rotate_bytes_right(vec, 12, 16);
    rotate_bytes_right(vec, 12, 16);
    rotate_bytes_right(vec, 12, 16);
}

pub(crate) fn mix_columns(vec: &mut Vec<u8>) {
    for i in 0..4 {
        let (i0, i1, i2, i3) = (i, i + 4, i + 8, i + 12);
        let (s0, s1, s2, s3) = (vec[i0], vec[i1], vec[i2], vec[i3]);

        vec[i0] = gmul(2, s0) ^ gmul(3, s1) ^ s2 ^ s3;
        vec[i1] = s0 ^ gmul(2, s1) ^ gmul(3, s2) ^ s3;
        vec[i2] = s0 ^ s1 ^ gmul(2, s2) ^ gmul(3, s3);
        vec[i3] = gmul(3, s0) ^ s1 ^ s2 ^ gmul(2, s3);
    }
}

pub(crate) fn inv_mix_columns(vec: &mut Vec<u8>) {
    for i in 0..4 {
        let (i0, i1, i2, i3) = (i, i + 4, i + 8, i + 12);
        let (s0, s1, s2, s3) = (vec[i0], vec[i1], vec[i2], vec[i3]);

        vec[i0] = gmul(0x0e, s0) ^ gmul(0x0b, s1) ^ gmul(0x0d, s2) ^ gmul(0x09, s3);
        vec[i1] = gmul(0x09, s0) ^ gmul(0x0e, s1) ^ gmul(0x0b, s2) ^ gmul(0x0d, s3);
        vec[i2] = gmul(0x0d, s0) ^ gmul(0x09, s1) ^ gmul(0x0e, s2) ^ gmul(0x0b, s3);
        vec[i3] = gmul(0x0b, s0) ^ gmul(0x0d, s1) ^ gmul(0x09, s2) ^ gmul(0x0e, s3);
    }
}

pub(crate) fn add_round_key(key: &[u8], vec: &mut Vec<u8>) {
    xor_block(key, vec);
}
