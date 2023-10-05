use crypto_box::{Nonce, PublicKey};

pub struct InitHandshakePacket {
    client_pk: PublicKey,
    encryption_nonce: Nonce,
    encrypted_payload: Vec<u8>,
}

impl From<&InitHandshakePacket> for Vec<u8> {
    fn from(value: &InitHandshakePacket) -> Self {
        [
            value.client_pk.as_bytes(),
            value.encryption_nonce.as_slice(),
            value.encrypted_payload.as_slice(),
        ]
        .concat()
    }
}

impl InitHandshakePacket {
    pub fn new(client_pk: PublicKey, encryption_nonce: Nonce, encrypted_payload: Vec<u8>) -> Self {
        Self {
            client_pk,
            encryption_nonce,
            encrypted_payload,
        }
    }
    pub fn bytes(&self) -> Vec<u8> {
        Vec::<u8>::from(self)
    }
}

#[derive(Debug)]
pub struct HandshakeRespPacket {
    nonce: Nonce,
    encrypted_payload: Vec<u8>,
}

impl HandshakeRespPacket {
    pub fn new(nonce: Nonce, encrypted_payload: Vec<u8>) -> Self {
        Self {
            nonce,
            encrypted_payload,
        }
    }
    pub fn nonce(&self) -> &Nonce {
        &self.nonce
    }
    pub fn encrypted_payload(&self) -> &[u8] {
        self.encrypted_payload.as_slice()
    }
}
