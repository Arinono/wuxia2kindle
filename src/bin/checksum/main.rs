use std::fmt::Write;
use sha2::{Digest, Sha384};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }
    let filepath = &args[1];
    let file = std::fs::read(filepath).expect("Failed to read file");
    let checksum = Vec::from(Sha384::digest(file).as_slice());

    println!("{}", short_checksum(&checksum));
}

fn short_checksum(checksum: &[u8]) -> String {
    let mut s = String::with_capacity(checksum.len() * 2);
    for b in checksum {
        write!(&mut s, "{b:02x?}").expect("should not fail to write to str");
    }
    s
}
