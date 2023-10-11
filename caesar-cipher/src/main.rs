/*
This code defines two functions: encrypt and decrypt.
The encrypt function takes a plaintext string and a shift value,
and returns the ciphertext string.
The decrypt function takes a ciphertext string and a shift value,
and returns the plaintext string.

*/

use std::env;
use caesar_cipher_tool::decrypt;
use caesar_cipher_tool::encrypt;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <plaintext> <shift>", args[0]);
        return;
    }

    let plaintext = &args[1];
    let shift: u8 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Error: Shift value should be a number.");
            return;
        }
    };

    let ciphertext = encrypt(plaintext, shift);
    let decrypted_text = decrypt(&ciphertext, shift);

    println!("Plaintext: {}", plaintext);
    println!("Ciphertext: {}", ciphertext);
    println!("Decrypted text: {}", decrypted_text);
}
