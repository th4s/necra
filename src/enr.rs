use k256::ecdsa::{signature::Signer, Signature, SigningKey};
use k256::CompressedPoint;
use std::convert::AsRef;

#[derive(Debug, Copy, Clone, Default)]
pub struct EthereumNodeRecord {
    pub id: IdentityScheme,
    pub secp256k1: CompressedPoint,
    pub ip: Option<[u8; 4]>,
    pub tcp: Option<u16>,
    pub udp: Option<u16>,
    pub ip6: Option<[u8; 16]>,
    pub tcp6: Option<u16>,
    pub udp6: Option<u16>,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum IdentityScheme {
    #[default]
    V4,
}

impl std::fmt::Display for IdentityScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v4")
    }
}

impl IdentityScheme {
    pub fn sign(&self, key: &SigningKey, bytes: impl AsRef<[u8]>) -> Signature {
        let signature: Signature = match self {
            IdentityScheme::V4 => key.sign(bytes.as_ref()),
        };
        signature
    }
}
