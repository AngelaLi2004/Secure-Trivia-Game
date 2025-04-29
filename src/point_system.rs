use crate::encryption::Encryptor;

pub struct SecurePointSystem {
    encrypted_points: i32,
    encryptor: Encryptor,
}

impl SecurePointSystem {
    pub fn new(key: u8) -> Self {
        let encryptor = Encryptor::new(key);
        SecurePointSystem {
            encrypted_points: encryptor.encrypt(0),
            encryptor,
        }
    }

    pub fn add_points(&mut self, pts: i32) {
        let mut points = self.encryptor.decrypt(self.encrypted_points);
        points += pts;
        self.encrypted_points = self.encryptor.encrypt(points);
    }

    pub fn use_points(&mut self, pts: i32) -> bool {
        let mut points = self.encryptor.decrypt(self.encrypted_points);
        if points >= pts {
            points -= pts;
            self.encrypted_points = self.encryptor.encrypt(points);
            true
        } else {
            false
        }
    }

    pub fn get_points(&self) -> i32 {
        self.encryptor.decrypt(self.encrypted_points)
    }
}