extern crate tiny_keccak;
use tiny_keccak::Keccak;
use std::io;

fn main() {
    println!("Enter the string to hash:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut sha3 = Keccak::new_sha3_256();
    let data = input.trim().as_bytes();
    sha3.update(data);
    let mut res: [u8; 32] = [0; 32];
    sha3.finalize(&mut res);
    println!("Keccak hash: {:?}", res);
}
