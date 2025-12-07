use crate::error::Error;

const ED25519_PREFIX: &str = "ed25519:";

pub fn encode_key(key: &[u8]) -> String {
    format!("{}{}", ED25519_PREFIX, bs58::encode(key).into_string())
}

pub fn decode_key(key: &str) -> Result<Vec<u8>, Error> {
    let key = key.strip_prefix(ED25519_PREFIX).unwrap_or(key);
    Ok(bs58::decode(key).into_vec()?)
}
