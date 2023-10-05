use std::{
    fmt::Display,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
};

use clap::Parser;
use crypto_box::{PublicKey, KEY_SIZE};
use hex::FromHex;

/// Tox simple handshake
#[derive(Parser, Debug)]
#[command(about)]
pub struct Args {
    /// Server's public key (hex-formatted 32 bytes)
    #[arg(
        long,
        value_parser = clap::value_parser!(HexPublicKey),
        default_value_t = HexPublicKey::from_str("7235A7AC54CCC946192F809E716B8A4BDCD4E407F874D0522A1D7D523EC9E57C").unwrap()
    )]
    pub server_pk: HexPublicKey,
    // Server address (in addr:port format)
    #[arg(long, default_value_t = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 33445))]
    pub server_addr: SocketAddr,
}

#[derive(Debug, Clone)]
pub struct HexPublicKey(PublicKey);

impl FromStr for HexPublicKey {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(HexPublicKey(PublicKey::from_bytes(
            <[u8; KEY_SIZE]>::from_hex(value).map_err(|e| e.to_string())?,
        )))
    }
}

impl From<HexPublicKey> for PublicKey {
    fn from(value: HexPublicKey) -> Self {
        value.0
    }
}

impl Display for HexPublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}
