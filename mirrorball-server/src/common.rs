use crate::utils;

pub struct ChunkDigest([u8; 32]);

impl TryFrom<&str> for ChunkDigest {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let bytes = utils::digest::decode_sha256_hex_string(input)?;

        Ok(Self(bytes))
    }
}

impl ChunkDigest {
    pub fn bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
