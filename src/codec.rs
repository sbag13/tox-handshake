use crypto_box::{Nonce, KEY_SIZE};
use tokio_util::codec::{Decoder, Encoder};

use crate::{
    packet::{HandshakeRespPacket, InitHandshakePacket},
    NONCE_SIZE,
};

// public key + nonce + mac (16)
pub const ENCRYPTED_PAYLOAD_SIZE: usize = KEY_SIZE + NONCE_SIZE + 16;

#[derive(Default)]
pub struct HandshakeCodec {}

impl Decoder for HandshakeCodec {
    type Item = HandshakeRespPacket;

    type Error = anyhow::Error;

    fn decode(
        &mut self,
        src: &mut tokio_util::bytes::BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() <= NONCE_SIZE + ENCRYPTED_PAYLOAD_SIZE {
            return Ok(None);
        }

        let nonce = *Nonce::from_slice(&src.split_to(NONCE_SIZE));
        let encrypted_payload = src.split_to(ENCRYPTED_PAYLOAD_SIZE).to_vec();

        Ok(Some(HandshakeRespPacket::new(nonce, encrypted_payload)))
    }
}

impl Encoder<InitHandshakePacket> for HandshakeCodec {
    type Error = anyhow::Error;

    fn encode(
        &mut self,
        item: InitHandshakePacket,
        dst: &mut tokio_util::bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        dst.extend_from_slice(&item.bytes());
        Ok(())
    }
}
