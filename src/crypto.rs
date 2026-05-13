use crate::config;
use crate::error::{Error, Result};
use sha2::{Digest, Sha256};
pub fn sha256_hex(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}
pub fn seal(payload_json: &str) -> String {
    let hash = sha256_hex(payload_json);
    format!("{}SHA={}\n{}", config::HEADER_MAGIC, hash, payload_json)
}
pub fn unseal(payload: &str) -> Result<String> {
    let newline_pos = payload
        .find('\n')
        .ok_or_else(|| Error::InvalidFormat("Missing header newline".into()))?;
    let header = &payload[..newline_pos];
    let json = &payload[newline_pos + 1..];
    if !header.starts_with(config::HEADER_MAGIC) {
        return Err(Error::InvalidFormat("Invalid magic header".into()));
    }
    let expected_hash = header
        .trim_start_matches(config::HEADER_MAGIC)
        .trim_start_matches(config::INTEGRITY_PREFIX);
    let actual_hash = sha256_hex(json);
    if expected_hash != actual_hash {
        return Err(Error::Integrity("Data corrupted or tampered!".into()));
    }
    Ok(json.to_string())
}
pub fn encrypt(plaintext: &str, passphrase: &str) -> Result<Vec<u8>> {
    let sealed = seal(plaintext);
    let ciphertext = age_crypto::encrypt_with_passphrase(sealed.as_bytes(), passphrase)
        .map_err(|e| Error::Crypto(e.to_string()))?;
    Ok(ciphertext.to_vec())
}
pub fn decrypt(ciphertext: &[u8], passphrase: &str) -> Result<String> {
    let plain =
        age_crypto::decrypt_with_passphrase(ciphertext, passphrase).map_err(|e| match e {
            age_crypto::Error::Decrypt(_) => Error::InvalidPassword,
            _ => Error::Crypto(e.to_string()),
        })?;
    let sealed = String::from_utf8(plain)
        .map_err(|_| Error::InvalidFormat("Payload is not UTF-8".into()))?;
    unseal(&sealed)
}
