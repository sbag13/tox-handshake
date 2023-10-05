use crypto_box::{
    aead::{AeadCore, OsRng},
    Nonce, PublicKey, SalsaBox, SecretKey,
};

pub struct Session {
    secret_key: SecretKey,
    public_key: PublicKey,
    nonce: Nonce,
}

impl Session {
    pub fn new() -> Self {
        let secret_key = SecretKey::generate(&mut OsRng);
        let public_key = secret_key.public_key();

        Self {
            secret_key,
            public_key,
            nonce: SalsaBox::generate_nonce(&mut OsRng),
        }
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    pub fn secret_key(&self) -> &SecretKey {
        &self.secret_key
    }

    pub fn nonce(&self) -> &Nonce {
        &self.nonce
    }
}
