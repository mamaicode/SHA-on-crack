use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use sha1::Digest;

const SHA1_HEX_STRING_LENGTH: usize =40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: ");
        println!("SHA1_cracker: <list.txt> <sha1_hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    let list_file = File::open(&args[1])?;
    let reader = BufReader::new(&list_file);

    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();
        if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password found: {}", &common_password);
            return Ok(());
        }
    }
    println!("Password not found");

    Ok(())
}