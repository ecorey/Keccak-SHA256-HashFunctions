extern crate tiny_keccak;
use tiny_keccak::Keccak;
use std::io;

// function takes a string and converts it to a keccak hash
// using the new_sha3_256 function. 

// Keccak256 is a cryptographic hash function that takes an 
// input of an arbitrary length and produces a fixed-length output 
// of 256 bits. It is the function used to compute the hashes 
// of Ethereum addresses, transaction IDs, and other important values 
// in the Ethereum ecosystem.


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
