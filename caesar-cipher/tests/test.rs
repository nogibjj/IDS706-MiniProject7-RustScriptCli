#[cfg(test)]
mod tests {

    #[test]
    fn test_encrypt() {
        let plaintext = "Hello, World!";
        let shift = 3;
        let encrypted = caesar_cipher_tool::encrypt(plaintext, shift);
        assert_eq!(encrypted, "Khoor, Zruog!");
    }

    #[test]
    fn test_decrypt() {
        let ciphertext = "Khoor, Zruog!";
        let shift = 3;
        let decrypted = caesar_cipher_tool::decrypt(ciphertext, shift);
        assert_eq!(decrypted, "Hello, World!");
    }

    #[test]
    fn test_encrypt_decrypt() {
        let plaintext = "Rust is fun!";
        let shift = 7;
        let encrypted = caesar_cipher_tool::encrypt(plaintext, shift);
        let decrypted = caesar_cipher_tool::decrypt(&encrypted, shift);
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_non_ascii() {
        let plaintext = "こんにちは, World!";
        let encrypted = caesar_cipher_tool::encrypt(plaintext, 5);
        let decrypted = caesar_cipher_tool::decrypt(&encrypted, 5);
        assert_eq!(decrypted, plaintext); // Non-ASCII characters should remain unchanged
    }
}
