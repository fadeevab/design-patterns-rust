#[derive(Debug)]
enum Cipher {
    Chacha20Poly1305,
    Aes256Gcm,
}

#[derive(Debug)]
enum KeyDerivationFunction {
    Pbkdf2,
    Argon2,
}

struct Vault {
    key: [u32; 32],
    cipher: Cipher,
    kdf: KeyDerivationFunction,
}

impl Vault {
    pub fn new(key: [u32; 32]) -> Self {
        Self {
            key,
            cipher: Cipher::Chacha20Poly1305,
            kdf: KeyDerivationFunction::Pbkdf2,
        }
    }

    pub fn with_cipher(mut self, cipher: Cipher) -> Self {
        self.cipher = cipher;
        self
    }

    pub fn with_kdf(mut self, kdf: KeyDerivationFunction) -> Self {
        self.kdf = kdf;
        self
    }

    pub fn encrypt(&self, data: String) {
        let _ = self.key;

        println!(
            "KDF {:?}, Cipher {:?}, encrypting '{}'...",
            self.kdf, self.cipher, data
        );
    }
}

pub fn example() {
    let key = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32,
    ];

    let vault = Vault::new(key)
        .with_cipher(Cipher::Aes256Gcm)
        .with_kdf(KeyDerivationFunction::Argon2);

    vault.encrypt("data".into());
}
