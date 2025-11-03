use anyhow::Result;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::Engine;

const KEY_SIZE: usize = 32; // 256 bits

pub struct EncryptionService {
    cipher: Aes256Gcm,
}

impl EncryptionService {
    pub fn new(key: &[u8]) -> Result<Self> {
        if key.len() != KEY_SIZE {
            anyhow::bail!("Key must be {} bytes", KEY_SIZE);
        }

        let key = aes_gcm::Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(key);

        Ok(EncryptionService { cipher })
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = self
            .cipher
            .encrypt(&nonce, plaintext.as_bytes())
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

        // Combine nonce and ciphertext
        let mut result = nonce.to_vec();
        result.extend_from_slice(&ciphertext);
        
        Ok(base64::engine::general_purpose::STANDARD.encode(result))
    }

    pub fn decrypt(&self, ciphertext: &str) -> Result<String> {
        let data = base64::engine::general_purpose::STANDARD.decode(ciphertext)?;
        
        if data.len() < 12 {
            anyhow::bail!("Invalid ciphertext length");
        }

        let nonce = Nonce::from_slice(&data[..12]);
        let encrypted = &data[12..];

        let plaintext = self
            .cipher
            .decrypt(nonce, encrypted)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

        String::from_utf8(plaintext)
            .map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))
    }
}

