use std::fs;
use std::io;

use crate::consts::*;
use crate::utils::{EncryptMode, rand_bytes, prepend_vec};
use crate::key::Key;

struct AES256CBC;

impl AES256CBC {
    fn add_padding(blocks: &mut Vec<Vec<u8>>) {
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

    fn remove_padding(state: &mut Vec<u8>) {
        let padding_len = *state.last().unwrap();
        state.truncate(state.len() - padding_len as usize);
    }
    
    fn split_blocks(bytes: Vec<u8>, block_size: usize) -> Vec<Vec<u8>> {
        let chunks = bytes.chunks(block_size);
        let mut blocks = Vec::with_capacity(chunks.len());
    
        for chunk in chunks {
            blocks.push(Vec::from(chunk));
        }
    
        blocks
    }

    fn xor_block(input: &[u8], output: &mut Vec<u8>) {
        for i in 0..usize::min(input.len(), output.len()) {
            output[i] ^= input[i];
        }
    }

    fn add_round_key(key: &[u8], vec: &mut Vec<u8>) {
        Self::xor_block(key, vec);
    }

    fn rotate_bytes_left(vec: &mut Vec<u8>, start: usize, end: usize) {
        let tmp = vec[start];
        for i in start..end - 1 {
            vec[i] = vec[i + 1];
        }
        vec[end - 1] = tmp;
    }

    fn rotate_bytes_right(vec: &mut Vec<u8>, start: usize, end: usize) {
        let tmp = vec[end - 1];
        for i in (start + 1..end).rev() {
            vec[i] = vec[i - 1];
        }
        vec[start] = tmp;
    }

    fn gmul(mut a:u8, mut b:u8) -> u8 {
        let mut p = 0;

        for _ in 0..8 {
            if b & 1 != 0 {
                p ^= a;
            }

            let carry_bit = a & 0x80 != 0;
            a <<= 1;
            if carry_bit {
                a ^= 0x1b;
            }
            b >>= 1;
        }

        p
    }

    fn sub_bytes(vec: &mut Vec<u8>) {
        for i in 0..vec.len() {
            vec[i] = S_BOX[vec[i] as usize];
        }
    }

    fn inv_sub_bytes(vec: &mut Vec<u8>) {
        for i in 0..vec.len() {
            vec[i] = INV_S_BOX[vec[i] as usize];
        }
    }

    fn shift_rows(vec: &mut Vec<u8>) {
        Self::rotate_bytes_left(vec, 4, 8);
        Self::rotate_bytes_left(vec, 8, 12);
        Self::rotate_bytes_left(vec, 8, 12);
        Self::rotate_bytes_left(vec, 12, 16);
        Self::rotate_bytes_left(vec, 12, 16);
        Self::rotate_bytes_left(vec, 12, 16);
    }

    fn inv_shift_rows(vec: &mut Vec<u8>) {
        Self::rotate_bytes_right(vec, 4, 8);
        Self::rotate_bytes_right(vec, 8, 12);
        Self::rotate_bytes_right(vec, 8, 12);
        Self::rotate_bytes_right(vec, 12, 16);
        Self::rotate_bytes_right(vec, 12, 16);
        Self::rotate_bytes_right(vec, 12, 16);
    }

    fn mix_columns(vec: &mut Vec<u8>) {
        for i in 0..4 {
            let (i0, i1, i2, i3) = (i, i + 4, i + 8, i + 12);
            let (s0, s1, s2, s3) = (vec[i0], vec[i1], vec[i2], vec[i3]);

            vec[i0] = Self::gmul(2, s0) ^ Self::gmul(3, s1) ^ s2 ^ s3;
            vec[i1] = s0 ^ Self::gmul(2, s1) ^ Self::gmul(3, s2) ^ s3;
            vec[i2] = s0 ^ s1 ^ Self::gmul(2, s2) ^ Self::gmul(3, s3);
            vec[i3] = Self::gmul(3, s0) ^ s1 ^ s2 ^ Self::gmul(2, s3);
        }
    }

    fn inv_mix_columns(vec: &mut Vec<u8>) {
        for i in 0..4 {
            let (i0, i1, i2, i3) = (i, i + 4, i + 8, i + 12);
            let (s0, s1, s2, s3) = (vec[i0], vec[i1], vec[i2], vec[i3]);

            vec[i0] = Self::gmul(0x0e, s0) ^ Self::gmul(0x0b, s1) ^ Self::gmul(0x0d, s2) ^ Self::gmul(0x09, s3);
            vec[i1] = Self::gmul(0x09, s0) ^ Self::gmul(0x0e, s1) ^ Self::gmul(0x0b, s2) ^ Self::gmul(0x0d, s3);
            vec[i2] = Self::gmul(0x0d, s0) ^ Self::gmul(0x09, s1) ^ Self::gmul(0x0e, s2) ^ Self::gmul(0x0b, s3);
            vec[i3] = Self::gmul(0x0b, s0) ^ Self::gmul(0x0d, s1) ^ Self::gmul(0x09, s2) ^ Self::gmul(0x0e, s3);
        }
    }
    
    pub fn encrypt(k: &Key, iv: Vec<u8>, data: Vec<u8>) -> Vec<u8> {
        let key = k.as_ref();
        let num_rounds = key.len() / 16;
        let mut state = Self::split_blocks(data, BLOCK_SIZE);
        Self::add_padding(&mut state);
        let num_blocks = state.len();
        let mut cipher_text = Vec::with_capacity(num_blocks * 17);
        let mut previous_block = iv;
        cipher_text.extend(&previous_block);

        for i in 0..num_blocks {
            Self::xor_block(&previous_block, &mut state[i]);
            Self::add_round_key(&key[0..16], &mut state[i]);

            for j in 1..num_rounds - 1 {
                Self::sub_bytes(&mut state[i]);
                Self::shift_rows(&mut state[i]);
                Self::mix_columns(&mut state[i]);
                Self::add_round_key(&key[j * 16..(j + 1) * 16], &mut state[i]);
            }

            Self::sub_bytes(&mut state[i]);
            Self::shift_rows(&mut state[i]);
            Self::add_round_key(&key[(num_rounds - 1) * 16..], &mut state[i]);

            previous_block = state[i].to_vec();
            cipher_text.extend(&state[i]);
        }
        
        cipher_text
    }

    pub fn decrypt(k: &Key, data: Vec<u8>) -> Vec<u8> {
        let key = k.as_ref();
        let key_len = key.len();
        let num_rounds = key_len / 16;
        let mut state = Self::split_blocks(data, BLOCK_SIZE);
        let num_blocks = state.len();
        let mut decrypted_text = Vec::with_capacity((num_blocks - 1) * 16);

        for i in (1..num_blocks).rev() {
            Self::add_round_key(&key[(num_rounds - 1) * 16..], &mut state[i]);
            Self::inv_shift_rows(&mut state[i]);
            Self::inv_sub_bytes(&mut state[i]);

            for j in 1..num_rounds - 1 {
                Self::add_round_key(&key[key_len - (j + 1) * 16..key_len - j * 16], &mut state[i]);
                Self::inv_mix_columns(&mut state[i]);
                Self::inv_shift_rows(&mut state[i]);
                Self::inv_sub_bytes(&mut state[i]);
            }

            let previous_block = state[i - 1].clone();
            Self::add_round_key(&key[0..16], &mut state[i]);
            Self::xor_block(&previous_block, &mut state[i]);
            prepend_vec(&mut decrypted_text, state[i].clone());
        }

        Self::remove_padding(&mut decrypted_text);
        decrypted_text
    }
}

pub fn run(op: EncryptMode) -> Result<(), io::Error> {
    match op {
        EncryptMode::Encrypt(key, path) => {
            let file = fs::read(path)?;
            let iv = rand_bytes(BLOCK_SIZE);
            let data = AES256CBC::encrypt(&key, iv, file);
            fs::write(path, data)
        },
        EncryptMode::Decrypt(key, path) => {
            let file = fs::read(path)?;
            let data = AES256CBC::decrypt(key, file);
            fs::write(path, data)
        }
    }
}
