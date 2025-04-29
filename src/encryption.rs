pub struct Encryptor {
    key: u8,
}
/// The `Encryptor` struct provides a simple XOR-based encryption and decryption mechanism.
/// It holds a single-byte key that is used to encrypt and decrypt integer data.
///
/// XOR encryption works by applying the bitwise exclusive OR (XOR) operation between the data and the key.
/// This operation is symmetric, meaning applying XOR with the same key twice returns the original data.
///
/// For example:
/// - Encryption: encrypted = data ^ key
/// - Decryption: decrypted = encrypted ^ key
///
/// Because XOR is its own inverse, the same function can be used for both encryption and decryption.
///
/// This method is very fast and lightweight, suitable for simple obfuscation tasks like protecting
/// player points in a game, but it is not secure against serious cryptographic attacks.
///
/// The key is a single byte (`u8`), which is cast to `i32` to match the data type before XOR-ing.

impl Encryptor {
    pub fn new(key: u8) -> Self {
        Encryptor { key }
    }

    // XOR encryption/decryption
    pub fn encrypt(&self, data: i32) -> i32 {
        data ^ self.key as i32
    }

    pub fn decrypt(&self, data: i32) -> i32 {
        data ^ self.key as i32
    }
}
