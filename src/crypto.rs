use crate::config;
use crate::error::{Error, Result};
use hmac::KeyInit;
use hmac::{Hmac, Mac};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use zeroize::Zeroizing;
type HmacSha256 = Hmac<Sha256>;
fn derive_key(passphrase: &str, version: u8) -> Result<Vec<u8>> {
    let mut key = vec![0u8; 32];
    pbkdf2_hmac::<Sha256>(
        passphrase.as_bytes(),
        format!("neuxdb-v{}", version).as_bytes(),
        100_000,
        &mut key,
    );
    Ok(key)
}
fn hmac_sha256_hex(key: &[u8], data: &str) -> Result<String> {
    let mut mac = HmacSha256::new_from_slice(key).map_err(|e| Error::Crypto(e.to_string()))?;
    mac.update(data.as_bytes());
    let hmac = mac.finalize().into_bytes();
    Ok(hex::encode(hmac))
}
pub fn seal(payload_json: &str, passphrase: &str) -> Result<String> {
    let version = config::DB_VERSION;
    let key = derive_key(passphrase, version)?;
    let hmac_hex = hmac_sha256_hex(&key, payload_json)?;
    Ok(format!(
        "{}HMAC={}\n{}",
        config::header_magic(version),
        hmac_hex,
        payload_json
    ))
}
pub fn unseal(payload: &str, passphrase: &str) -> Result<Zeroizing<String>> {
    let newline_pos = payload
        .find('\n')
        .ok_or_else(|| Error::InvalidFormat("Missing header newline".into()))?;
    let header = &payload[..newline_pos];
    let json = &payload[newline_pos + 1..];
    if !header.starts_with("[NEUXDB:v") || !header.contains("]HMAC=") {
        return Err(Error::InvalidFormat(
            "Invalid or unsupported database format".into(),
        ));
    }
    let version_start = header.find("v").unwrap() + 1;
    let version_end = header.find("]").unwrap();
    let version: u8 = header[version_start..version_end]
        .parse()
        .map_err(|_| Error::InvalidFormat("Invalid version number".into()))?;
    let expected_hmac_hex = &header[header.find("HMAC=").unwrap() + 5..];
    let key = derive_key(passphrase, version)?;
    let computed_hmac_hex = hmac_sha256_hex(&key, json)?;
    if expected_hmac_hex != computed_hmac_hex {
        return Err(Error::Integrity("Data tampered or corrupted!".into()));
    }
    Ok(Zeroizing::new(json.to_string()))
}
pub fn encrypt(plaintext: &str, passphrase: &str) -> Result<Vec<u8>> {
    let sealed = seal(plaintext, passphrase)?;
    let ciphertext = age_crypto::encrypt_with_passphrase(sealed.as_bytes(), passphrase)
        .map_err(|e| Error::Crypto(e.to_string()))?;
    Ok(ciphertext.to_vec())
}
pub fn decrypt(ciphertext: &[u8], passphrase: &str) -> Result<Zeroizing<String>> {
    let plain =
        age_crypto::decrypt_with_passphrase(ciphertext, passphrase).map_err(|e| match e {
            age_crypto::Error::Decrypt(_) => Error::InvalidPassword,
            _ => Error::Crypto(e.to_string()),
        })?;
    let sealed = String::from_utf8(plain)
        .map_err(|_| Error::InvalidFormat("Payload is not UTF-8".into()))?;
    unseal(&sealed, passphrase)
}
