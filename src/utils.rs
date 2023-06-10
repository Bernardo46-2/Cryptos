use rand::RngCore;

// =========================================== Encryption stuff =========================================== //

pub fn rand_bytes(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut vec = vec![0; size];
    rng.fill_bytes(&mut vec);
    vec
}

pub fn prepend_vec<T> (vec: &mut Vec<T>, mut prefix: Vec<T>) {
    prefix.append(vec);
    *vec = prefix;
}

pub fn transpose_matrix<T> (vec: &mut [T], lines: usize, cols: usize) {
    for l in 0..lines {
        for c in l+1..cols {
            let i = l * cols + c;
            let j = c * cols + l;
            vec.swap(i, j);
        }
    }
}

// =========================================== Debug stuff =========================================== //

#[allow(dead_code)]
pub fn print_vec_hex(vec: &[u8]) {
    for j in 0..vec.len() {
        print!("{:02x} ", vec[j]);
    }
    println!("\n");
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! print_vec {
    ($(($x:expr, $y:expr)),*) => {{
        $(
            println!("{}", $x);
            print_vec_hex($y);
        )*
    }}
}


// =========================================== Test =========================================== //

#[cfg(test)]
mod test {
    #[test]
    fn transpose_text() {
        let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let expected = vec![0, 3, 6, 1, 4, 7, 2, 5, 8];
        super::transpose_matrix(&mut vec, 3, 3);
        assert_eq!(vec, expected);
    }
}
