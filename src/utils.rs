use rand::RngCore;
use clap::{Command, Arg, ArgMatches};
use glob::glob;

use crate::key::Key;
use crate::consts::KEY_SIZE;

// =========================================== Argument stuff =========================================== //

pub fn get_args() -> ArgMatches {
    Command::new("Crypto")
        .arg(Arg::new("Encrypt")
            .value_name("paths")
            .short('e')
            .long("encrypt")
            .num_args(1..)
            .help("Path to the file to be encrypted")
            .required_unless_present_any(["Decrypt", "New Key"]))
        .arg(Arg::new("Decrypt")
            .value_name("paths")
            .short('d')
            .long("decrypt")
            .num_args(1..)
            .help("Path to the file to be decrypted")
            .conflicts_with_all(["Encrypt", "New Key"]))
        .arg(Arg::new("Key Path")
            .value_name("path")
            .default_value("key.txt")
            .short('k')
            .long("key")
            .num_args(1)
            .help("Path to the file with the key"))
        .arg(Arg::new("New Key")
            .value_name("path")
            .short('n')
            .long("new-key")
            .num_args(1)
            .help("Creates a new key and writes it to the given path")
            .conflicts_with("Key Path"))
        .get_matches()
}

pub fn get_action(args: &ArgMatches) -> String {
    String::from(
        if args.contains_id("Encrypt") {
            "Encrypt"
        } else if args.contains_id("Decrypt") {
            "Decrypt"
        } else {
            ""
        }
    )
}

pub fn get_paths(args: &ArgMatches, action: &str) -> Option<Vec<String>> {
    if let Some(paths) = args.get_many::<String>(action) {
        let paths = paths.map(|i| i.clone()).collect();
        Some(find_glob_paths(paths))
    } else {
        None
    }
}

fn find_glob_paths(paths: Vec<String>) -> Vec<String> {
    let mut vec = Vec::with_capacity(paths.capacity());

    for path in paths {
        for p in glob(&path).expect("Error pattern glob pattern") {
            vec.push(p.expect("Error parsing glob pattern").to_str().unwrap().to_string());
        }
    }

    vec
}

pub fn get_key(args: &ArgMatches) -> Result<Key, std::io::Error> {
    if let Some(key_path) = args.get_one::<String>("New Key") {
        let key = Key::new(KEY_SIZE);
        key.to_file(&key_path)?;
        Ok(key)
    } else {
        let key_path = args.get_one::<String>("Key Path").unwrap();
        Key::from_file(&key_path)
    }
}

// =========================================== Encryption stuff =========================================== //

pub enum EncryptMode<'a> {
    Encrypt(&'a Key, &'a str),
    Decrypt(&'a Key, &'a str)
}

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
