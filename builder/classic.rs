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

impl Default for Vault {
    fn default() -> Self {
        Self {
            key: Default::default(),
            cipher: Cipher::Chacha20Poly1305,
            kdf: KeyDerivationFunction::Pbkdf2,
        }
    }
}

impl Vault {
    pub fn encrypt(&self, data: String) {
        let _ = self.key;

        println!(
            "KDF {:?}, Cipher {:?}, Encrypting '{}'...",
            self.kdf, self.cipher, data
        );
    }
}

struct VaultBuilder {
    container: Vault,
}

impl VaultBuilder {
    pub fn new_vault(key: [u32; 32]) -> Self {
        let mut container = Vault::default();
        container.key = key;
        Self { container }
    }

    pub fn with_cipher(mut self, cipher: Cipher) -> Self {
        self.container.cipher = cipher;
        self
    }

    pub fn with_kdf(mut self, kdf: KeyDerivationFunction) -> Self {
        self.container.kdf = kdf;
        self
    }

    pub fn build(self) -> Vault {
        self.container
    }
}

pub fn example() {
    let key = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32,
    ];

    let vault = VaultBuilder::new_vault(key)
        .with_cipher(Cipher::Aes256Gcm)
        .with_kdf(KeyDerivationFunction::Argon2)
        .build();

    vault.encrypt("data".into());
}
