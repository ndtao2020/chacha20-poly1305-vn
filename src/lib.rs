#![allow(deprecated)]

#[global_allocator]
pub static GLOBAL_ALLOCATOR: &alloc_cat::AllocCat = &alloc_cat::ALLOCATOR;

use chacha20poly1305::{
    Nonce, XNonce,
    aead::{Aead, KeyInit},
};
use wasm_bindgen::prelude::*;

const SECRET_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 12;
const XNONCE_LENGTH: usize = 24;

type Result<T> = std::result::Result<T, JsValue>;

fn js_error(msg: &str) -> JsValue {
    JsValue::from_str(msg)
}

#[wasm_bindgen]
/// Utility functions for key generation
pub struct CryptoUtils;

#[wasm_bindgen]
impl CryptoUtils {
    /// Generates cryptographically secure random bytes
    #[wasm_bindgen(js_name = randomBytes)]
    pub fn random_bytes(size: usize) -> Result<Vec<u8>> {
        let mut bytes = vec![0u8; size];
        getrandom::getrandom(&mut bytes)
            .map_err(|e| js_error(&format!("Failed to generate random bytes: {}", e)))?;
        Ok(bytes)
    }
}

#[wasm_bindgen]
pub struct ChaCha20Poly1305 {
    cipher: chacha20poly1305::ChaCha20Poly1305,
    nonce: Nonce,
}

#[wasm_bindgen]
impl ChaCha20Poly1305 {
    #[wasm_bindgen(constructor)]
    pub fn new(secret_bytes: &[u8], nonce_bytes: &[u8]) -> Result<Self> {
        if secret_bytes.len() != SECRET_LENGTH {
            return Err(js_error("Invalid secret key length!"));
        }
        if nonce_bytes.len() != NONCE_LENGTH {
            return Err(js_error("Invalid nonce key length!"));
        }

        let cipher = chacha20poly1305::ChaCha20Poly1305::new_from_slice(secret_bytes)
            .map_err(|e| js_error(&format!("Cipher initialization failed: {}", e)))?;

        let nonce = Nonce::from_slice(nonce_bytes);

        Ok(Self {
            cipher,
            nonce: *nonce,
        })
    }

    pub fn encrypt(&self, payload_bytes: &[u8]) -> Result<Vec<u8>> {
        self.cipher
            .encrypt(&self.nonce, payload_bytes)
            .map_err(|e| js_error(&format!("Encryption failed: {}", e)))
    }

    pub fn decrypt(&self, encrypted_bytes: &[u8]) -> Result<Vec<u8>> {
        self.cipher
            .decrypt(&self.nonce, encrypted_bytes)
            .map_err(|e| js_error(&format!("Decryption failed: {}", e)))
    }

    #[wasm_bindgen(js_name = generateKey)]
    pub fn generate_key() -> Result<Vec<u8>> {
        let mut key = [0u8; SECRET_LENGTH];
        getrandom::getrandom(&mut key)
            .map_err(|e| js_error(&format!("Failed to generate random key: {}", e)))?;
        Ok(key.to_vec())
    }

    #[wasm_bindgen(js_name = generateNonce)]
    pub fn generate_nonce() -> Result<Vec<u8>> {
        let mut nonce = [0u8; NONCE_LENGTH];
        getrandom::getrandom(&mut nonce)
            .map_err(|e| js_error(&format!("Failed to generate random nonce: {}", e)))?;
        Ok(nonce.to_vec())
    }
}

#[wasm_bindgen]
pub struct XChaCha20Poly1305 {
    cipher: chacha20poly1305::XChaCha20Poly1305,
    nonce: XNonce,
}

#[wasm_bindgen]
impl XChaCha20Poly1305 {
    #[wasm_bindgen(constructor)]
    pub fn new(secret_bytes: &[u8], nonce_bytes: &[u8]) -> Result<Self> {
        if secret_bytes.len() != SECRET_LENGTH {
            return Err(js_error("Invalid secret key length!"));
        }
        if nonce_bytes.len() != XNONCE_LENGTH {
            return Err(js_error("Invalid nonce key length!"));
        }

        let cipher = chacha20poly1305::XChaCha20Poly1305::new_from_slice(secret_bytes)
            .map_err(|e| js_error(&format!("Cipher initialization failed: {}", e)))?;

        let nonce = XNonce::from_slice(nonce_bytes);

        Ok(Self {
            cipher,
            nonce: *nonce,
        })
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.cipher
            .encrypt(&self.nonce, data)
            .map_err(|e| js_error(&format!("Encryption failed: {}", e)))
    }

    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>> {
        self.cipher
            .decrypt(&self.nonce, encrypted_data)
            .map_err(|e| js_error(&format!("Decryption failed: {}", e)))
    }

    #[wasm_bindgen(js_name = generateNonce)]
    pub fn generate_nonce() -> Result<Vec<u8>> {
        let mut nonce = [0u8; XNONCE_LENGTH];
        getrandom::getrandom(&mut nonce)
            .map_err(|e| js_error(&format!("Failed to generate random nonce: {}", e)))?;
        Ok(nonce.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> [u8; SECRET_LENGTH] {
        let mut secret = [0u8; SECRET_LENGTH];
        getrandom::getrandom(&mut secret).unwrap();
        secret
    }

    fn test_nonce() -> [u8; NONCE_LENGTH] {
        let mut nonce = [0u8; NONCE_LENGTH];
        getrandom::getrandom(&mut nonce).unwrap();
        nonce
    }

    fn test_xnonce() -> [u8; XNONCE_LENGTH] {
        let mut nonce = [0u8; XNONCE_LENGTH];
        getrandom::getrandom(&mut nonce).unwrap();
        nonce
    }

    #[test]
    fn test_chacha20poly1305_encrypt_decrypt() {
        let plaintext = b"Hello, World! This is a test message.";
        let key = test_key();
        let nonce = test_nonce();

        let cipher = ChaCha20Poly1305::new(&key, &nonce).unwrap();

        let encrypted = cipher.encrypt(plaintext).unwrap();
        assert_ne!(encrypted, plaintext);
        assert!(encrypted.len() > plaintext.len());

        let decrypted = cipher.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_xchacha20poly1305_encrypt_decrypt() {
        let plaintext = b"Hello, World! This is a test message for XChaCha20.";
        let key = test_key();
        let nonce = test_xnonce();

        let cipher = XChaCha20Poly1305::new(&key, &nonce).unwrap();

        let encrypted = cipher.encrypt(plaintext).unwrap();
        assert_ne!(encrypted, plaintext);

        let decrypted = cipher.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }
}
