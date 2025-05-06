pub struct Encryptor {
    key: u8,
}
/// The `Encryptor` struct provides a simple XOR-based encryption and decryption mechanism.
/// It holds a single-byte key that is used to encrypt and decrypt integer data.
///
/// The encrypt method flips certain bits in the number using XOR with the key.
/// The decrypt method flips those same bits again using the same XOR key.
/// Exclusive OR (XOR)

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
