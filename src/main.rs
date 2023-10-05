use anyhow::{anyhow, Context, Ok};
use arg::Args;
use clap::Parser;
use crypto_box::{
    aead::{Aead, AeadCore, OsRng},
    PublicKey, SalsaBox, SecretKey, KEY_SIZE,
};
use futures::{SinkExt, StreamExt};
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio_util::codec::Framed;

use crate::{
    codec::HandshakeCodec, packet::InitHandshakePacket, payload::Payload, session::Session,
};

mod arg;
mod codec;
mod packet;
mod payload;
mod session;

const NONCE_SIZE: usize = 24;
const PAYLOAD_SIZE: usize = KEY_SIZE + NONCE_SIZE;

async fn handshake(
    server_pk: &PublicKey,
    client_sk: &SecretKey,
    server_addr: impl ToSocketAddrs,
) -> anyhow::Result<SalsaBox> {
    println!("Connecting to socket...");
    let tcp_stream = TcpStream::connect(server_addr).await?;
    println!("Connected");

    let mut framed_tcp_stream = Framed::new(tcp_stream, HandshakeCodec::default());

    let session = Session::new();
    let combined_key = SalsaBox::new(server_pk, &client_sk);

    println!("Sending initial packet");
    let init_packet = init_handshake_packet(&session, &combined_key, client_sk)?;
    framed_tcp_stream
        .send(init_packet)
        .await
        .context("Error while sending initial handshake packet")?;

    println!("Waiting for the server's response");
    let server_response = framed_tcp_stream
        .next()
        .await
        .ok_or(anyhow!("No response from the server. Stream terminated. Check if the server's public key is correct"))?
        .map_err(|e| anyhow!("Decoder error: {e}"))?;

    let decrypted_payload_bytes: Vec<u8> = combined_key
        .decrypt(server_response.nonce(), server_response.encrypted_payload())
        .map_err(|e| anyhow!("Failed to decrypt message from the server: {e}"))?;
    let received_payload = Payload::try_from(decrypted_payload_bytes.as_slice())?;

    println!("Handshake completed");

    Ok(SalsaBox::new(
        received_payload.session_pk(),
        session.secret_key(),
    ))
}

fn init_handshake_packet(
    session: &Session,
    combined_key: &SalsaBox,
    client_sk: &SecretKey,
) -> anyhow::Result<InitHandshakePacket> {
    let send_payload = Payload::new(session.nonce().to_owned(), session.public_key().to_owned());
    let encryption_nonce = SalsaBox::generate_nonce(&mut OsRng);
    let encrypted_payload = combined_key
        .encrypt(&encryption_nonce, send_payload.bytes().as_slice())
        .map_err(|e| anyhow!("Error while encrypting payload: {e}"))?;

    Ok(InitHandshakePacket::new(
        client_sk.public_key(),
        encryption_nonce,
        encrypted_payload,
    ))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let client_sk = SecretKey::generate(&mut OsRng);

    let _crypto_box_for_further_usage =
        handshake(&args.server_pk.into(), &client_sk, args.server_addr).await?;

    Ok(())
}
