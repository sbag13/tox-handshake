use anyhow::anyhow;
use crypto_box::{Nonce, PublicKey, KEY_SIZE};

use crate::PAYLOAD_SIZE;

#[derive(Debug)]
pub struct Payload {
    session_pk: PublicKey,
    session_nonce: Nonce,
}

impl From<&Payload> for Vec<u8> {
    fn from(value: &Payload) -> Self {
        [value.session_pk.as_bytes(), value.session_nonce.as_slice()].concat()
    }
}

impl TryFrom<&[u8]> for Payload {
    type Error = anyhow::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < PAYLOAD_SIZE {
            return Err(anyhow!("Too short payload"));
        }

        let session_pk = PublicKey::from_slice(&value[0..KEY_SIZE]).unwrap();
        let session_nonce = *Nonce::from_slice(&value[KEY_SIZE..PAYLOAD_SIZE]);
        Ok(Payload {
            session_pk,
            session_nonce,
        })
    }
}

impl Payload {
    pub fn new(session_nonce: Nonce, session_pk: PublicKey) -> Self {
        Self {
            session_pk,
            session_nonce,
        }
    }
    pub fn bytes(&self) -> Vec<u8> {
        Vec::<u8>::from(self)
    }
    pub fn session_pk(&self) -> &PublicKey {
        &self.session_pk
    }
}
