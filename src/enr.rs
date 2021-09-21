use k256::CompressedPoint;

pub struct EthereumNodeRecord {
    pub id: IdentityScheme,
    pub secp256k1: CompressedPoint,
    pub ip: [u8; 4],
    pub tcp: u16,
    pub udp: u16,
    pub ip6: [u8; 16],
    pub tcp6: u16,
    pub udp6: u16,
}

impl EthereumNodeRecord {
    fn builder(id: IdentityScheme) {}
}

pub enum IdentityScheme {
    V4,
}
