use std::{io, fs};

use crate::consts::*;
use crate::utils::rand_bytes;

pub struct Key {
    key: Vec<u8>,
    expanded: Vec<u8>
}

impl Key {
    pub fn new(size: usize) -> Self {
        let key = rand_bytes(size);
        let expanded = Self::expand(&key);
        Self { key, expanded }
    }

    pub fn from_file(path: &str) -> Result<Self, io::Error> {
        let key = fs::read(path)?;
        let expanded = Self::expand(&key);
        Ok(Self { key, expanded })
    }

    pub fn to_file(&self, path: &str) -> Result<(), io::Error> {
        fs::write(path, &self.key)
    }

    fn expand(key: &Vec<u8>) -> Vec<u8> {
        let mut key_schedule = key.clone();

        for i in 8..60 {
            let mut tmp = key_schedule[key_schedule.len() - 4..].to_vec();

            if i % 8 == 0 {
                tmp.rotate_left(1);

                for j in 0..tmp.len() {
                    tmp[j] = S_BOX[tmp[j] as usize];
                }

                tmp[0] ^= RCON[i / 8];
            } else if i % 8 == 4 {
                for j in 0..tmp.len() {
                    tmp[j] = S_BOX[tmp[j] as usize];
                }
            }

            let word = &key_schedule[(i - 8) * 4..(i - 7) * 4];
            for j in 0..4 {
                tmp[j] ^= word[j];
            }

            key_schedule.extend_from_slice(&tmp);
        }

        key_schedule
    }

    pub fn as_ref(&self) -> &Vec<u8> {
        &self.expanded
    }
}
