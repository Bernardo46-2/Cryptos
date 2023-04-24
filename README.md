# Cryptos
Some cryptography implementations in rust
This is mainly for fun and for learning how some cryptography methods works.

## Disclaimer
All the implementations are for educational or personal use only and should not be used for any critical or production systems. While I have taken reasonable steps to ensure the correctness of the implementation, there may still be bugs or vulnerabilities. Use at your own risk.

## How to use
- The test folder contains the files I used to test the encryption/decryption, but you can use your own
- Just run the program with the argumets below

### Arguments
- `-e` | `--encrypt` -> takes any number of arguments containing directories of folders to encrypt. (requires `key` or `ǹew-key` arguments)
- `-d` | `--decrypt` -> takes any number of arguments containing directories of folders to decrypt. (requires `key` argument)
- `-k` | `--key` -> takes a single argument containing the directory of the key to be used (required for decryption)
- `--new-key` -> takes a single argument containing the directory of the file to where to store a new key that will be created (can be used without any other arguments)

## Notes
- Arguments are kinda weird but I didn't find a better way to make them (I really don't wanna make a menu..).
- Since cargo has it's own arguments, you have to run `cargo run --` and then the actual arguments of the program.
- The program accepts glob patterns, so "test/*.txt" type stuff are allowed as arguments.

### TODO
- AES GCM next (?)
