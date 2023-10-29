extern crate tiny_keccak;
use tiny_keccak::Keccak;
use std::io;

// function takes a string and converts it to a keccak hash
// using the new_sha3_256 function. 

// Keccak256 is a cryptographic hash function. It takes an 
// input of any length and then produces a fixed-length hashed output 
// of 256 bits. The Keccak256 hash function is the function used to compute 
// the hashes of Ethereum addresses, transaction IDs, and other values.



fn main() {
    println!("Enter the string to hash:");

    // create a new mutable string var
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // create var sha3 to the hashed value
    let mut sha3 = Keccak::new_sha3_256();

    // trim any leading/trailing whitespace from the `input` string,
    // convert it to bytes, and store it in a variable named `data`.
    let data = input.trim().as_bytes();
    
    // update the hash object with the byte data.
    sha3.update(data);
    
    // create a mutable array named `res` 
    // of 32 zeroed bytes to hold the hash result.
    let mut res: [u8; 32] = [0; 32];
    
    // finalize the hash computation and store the 
    // result in the `res` array.
    sha3.finalize(&mut res);

    // prints the hashed value
    println!("Keccak hash: {:?}", res);
}
