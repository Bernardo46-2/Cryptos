use super::consts::*;
use super::super::utils::rand_bytes;

#[derive(Clone, Debug)]
pub struct Key {
    key: Vec<u8>,
    expanded: Vec<u8>,
    size_bits: usize,
    size_bytes: usize
}

impl Key {
    pub fn new(size_bits: usize) -> Self {
        Self::validate_size_bits(size_bits);
        let size_bytes = size_bits / 8;
        let key = rand_bytes(size_bytes);
        let expanded = Self::expand(&key);
        Self { key, expanded, size_bits, size_bytes }
    }
    
    pub fn from(key: &[u8]) -> Self {
        let size_bytes = key.len();
        let size_bits = size_bytes * 8;

        Self::validate_size_bits(size_bits);
        
        let key = key.to_owned();
        let expanded = Self::expand(&key);

        Self { key, expanded, size_bits, size_bytes }
    }

    fn validate_size_bits(size: usize) {
        if !ALLOWED_KEY_SIZES.contains(&size) {
            panic!("Invalid key size `{}`. Key size must be one of: {:?}", size, ALLOWED_KEY_SIZES);
        }
    }

    fn expand(key: &Vec<u8>) -> Vec<u8> {
        let key_len = key.len();
        let (nr, nk) = match key_len {
            16 => (10, 4),
            24 => (12, 6),
            32 => (14, 8),
            _ => unreachable!()
        };
        let key_schedule_len = 16 * (nr + 1);
        let mut key_schedule = Vec::with_capacity(key_schedule_len);
        key_schedule.extend(key);

        for i in 0..4*nr+4-nk {
            let mut word = key_schedule[i*4+key_len-4..i*4+key_len].to_owned();

            if i % nk == 0 {
                word.rotate_left(1);
                super::sub_bytes(&mut word);
                word[0] ^= RCON[i / nk];
            } else if nk > 6 && i % nk == 4 {
                super::sub_bytes(&mut word);
            }

            let prev_word = &key_schedule[i*4..i*4+4];
            super::xor_block(prev_word, &mut word);
            key_schedule.extend(&word);
        }

        key_schedule
    }

    pub fn vec(&self) -> &Vec<u8> {
        &self.key
    }

    pub fn expanded(&self) -> &Vec<u8> {
        &self.expanded
    }

    pub fn size_bits(&self) -> usize {
        self.size_bits
    }

    pub fn size_bytes(&self) -> usize {
        self.size_bytes
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn key_expansion_test() {
        let key = super::Key::from(&[0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c]);

        let expected_output = vec![0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c, 
                                   0xa0, 0xfa, 0xfe, 0x17, 0x88, 0x54, 0x2c, 0xb1, 0x23, 0xa3, 0x39, 0x39, 0x2a, 0x6c, 0x76, 0x05, 
                                   0xf2, 0xc2, 0x95, 0xf2, 0x7a, 0x96, 0xb9, 0x43, 0x59, 0x35, 0x80, 0x7a, 0x73, 0x59, 0xf6, 0x7f, 
                                   0x3d, 0x80, 0x47, 0x7d, 0x47, 0x16, 0xfe, 0x3e, 0x1e, 0x23, 0x7e, 0x44, 0x6d, 0x7a, 0x88, 0x3b, 
                                   0xef, 0x44, 0xa5, 0x41, 0xa8, 0x52, 0x5b, 0x7f, 0xb6, 0x71, 0x25, 0x3b, 0xdb, 0x0b, 0xad, 0x00, 
                                   0xd4, 0xd1, 0xc6, 0xf8, 0x7c, 0x83, 0x9d, 0x87, 0xca, 0xf2, 0xb8, 0xbc, 0x11, 0xf9, 0x15, 0xbc, 
                                   0x6d, 0x88, 0xa3, 0x7a, 0x11, 0x0b, 0x3e, 0xfd, 0xdb, 0xf9, 0x86, 0x41, 0xca, 0x00, 0x93, 0xfd, 
                                   0x4e, 0x54, 0xf7, 0x0e, 0x5f, 0x5f, 0xc9, 0xf3, 0x84, 0xa6, 0x4f, 0xb2, 0x4e, 0xa6, 0xdc, 0x4f, 
                                   0xea, 0xd2, 0x73, 0x21, 0xb5, 0x8d, 0xba, 0xd2, 0x31, 0x2b, 0xf5, 0x60, 0x7f, 0x8d, 0x29, 0x2f, 
                                   0xac, 0x77, 0x66, 0xf3, 0x19, 0xfa, 0xdc, 0x21, 0x28, 0xd1, 0x29, 0x41, 0x57, 0x5c, 0x00, 0x6e, 
                                   0xd0, 0x14, 0xf9, 0xa8, 0xc9, 0xee, 0x25, 0x89, 0xe1, 0x3f, 0x0c, 0xc8, 0xb6, 0x63, 0x0c, 0xa6];
        
        assert_eq!(key.expanded(), &expected_output);
    }
}