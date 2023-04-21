mod consts;
mod utils;
mod key;
mod aes256cbc;

use std::io;

use utils::EncryptMode;

fn main() -> Result<(), io::Error> {
    let args = utils::get_args();
    let action = utils::get_action(&args);
    let paths = utils::get_paths(&args, &action);
    let key = utils::get_key(&args)?;

    match action.as_str() {
        "Encrypt" => {
            for path in paths.unwrap() {
                aes256cbc::run(EncryptMode::Encrypt(&key, &path))?;
            }
        },
        "Decrypt" => {
            for path in paths.unwrap() {
                aes256cbc::run(EncryptMode::Decrypt(&key, &path))?;
            }
        },
        _ => ()
    }

    Ok(())
}
