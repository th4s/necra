use k256::ecdsa::{signature::Signer, Signature, SigningKey};
use k256::CompressedPoint;
use std::convert::AsRef;

#[derive(Debug, Copy, Clone, Default)]
pub struct EthereumNodeRecord {
    pub seq: u64,
    pub id: IdentityScheme,
    pub secp256k1: CompressedPoint,
    pub ip: Option<[u8; 4]>,
    pub tcp: Option<u16>,
    pub udp: Option<u16>,
    pub ip6: Option<[u8; 16]>,
    pub tcp6: Option<u16>,
    pub udp6: Option<u16>,
}

impl EthereumNodeRecord {
    pub fn new(point: CompressedPoint) -> Self {
        let enr = Self::default();
        enr.seq(1).secp(point)
    }

    pub fn seq(mut self, seq: u64) -> Self {
        self.seq = seq;
        self
    }

    pub fn secp(mut self, secp256k1: CompressedPoint) -> Self {
        self.secp256k1 = secp256k1;
        self
    }

    pub fn ip(mut self, ip: [u8; 4]) -> Self {
        self.ip = Some(ip);
        self
    }

    pub fn tcp(mut self, tcp: u16) -> Self {
        self.tcp = Some(tcp);
        self
    }

    pub fn udp(mut self, udp: u16) -> Self {
        self.udp = Some(udp);
        self
    }

    pub fn ip6(mut self, ip6: [u8; 16]) -> Self {
        self.ip6 = Some(ip6);
        self
    }

    pub fn tcp6(mut self, tcp6: u16) -> Self {
        self.tcp6 = Some(tcp6);
        self
    }

    pub fn udp6(mut self, udp6: u16) -> Self {
        self.udp6 = Some(udp6);
        self
    }

    pub fn encode(&self) -> String {
        todo!()
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum IdentityScheme {
    #[default]
    V4,
}

impl std::fmt::Display for IdentityScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            IdentityScheme::V4 => write!(f, "v4"),
        }
    }
}

impl IdentityScheme {
    fn sign(&self, key: &SigningKey, bytes: impl AsRef<[u8]>) -> Signature {
        let signature: Signature = match self {
            IdentityScheme::V4 => key.sign(bytes.as_ref()),
        };
        signature
    }
}
