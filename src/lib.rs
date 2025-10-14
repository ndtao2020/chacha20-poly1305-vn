use chacha20poly1305::aead::{Aead, KeyInit};
use wasm_bindgen::prelude::*;

const SECRET_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 12;
const XNONCE_LENGTH: usize = 24;

#[wasm_bindgen]
pub struct ChaCha20Poly1305 {
    secret_key: Vec<u8>,
}

#[wasm_bindgen]
impl ChaCha20Poly1305 {
    #[wasm_bindgen(constructor)]
    pub fn new(secret_bytes: &[u8]) -> Result<ChaCha20Poly1305, JsValue> {
        if secret_bytes.len() != SECRET_LENGTH {
            return Err(JsValue::from_str("Invalid secret key length !"));
        }
        Ok(ChaCha20Poly1305 {
            secret_key: secret_bytes.to_vec(),
        })
    }

    #[allow(deprecated)]
    pub fn encrypt(&self, nonce_bytes: &[u8], payload_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
        if nonce_bytes.len() != NONCE_LENGTH {
            return Err(JsValue::from_str("Invalid nonce key length !"));
        }
        let cipher = chacha20poly1305::ChaCha20Poly1305::new_from_slice(&self.secret_key)
            .map_err(|e| JsValue::from_str(&format!("Cipher initialization failed: {}", e)))?;

        let nonce = chacha20poly1305::Nonce::from_slice(nonce_bytes);

        cipher
            .encrypt(nonce, payload_bytes)
            .map_err(|e| JsValue::from_str(&format!("Encryption failed: {}", e)))
    }

    #[allow(deprecated)]
    pub fn decrypt(&self, nonce_bytes: &[u8], encrypted_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
        if nonce_bytes.len() != NONCE_LENGTH {
            return Err(JsValue::from_str("Invalid nonce key length !"));
        }
        let cipher = chacha20poly1305::ChaCha20Poly1305::new_from_slice(&self.secret_key)
            .map_err(|e| JsValue::from_str(&format!("Cipher initialization failed: {}", e)))?;

        let nonce = chacha20poly1305::Nonce::from_slice(nonce_bytes);

        cipher
            .decrypt(nonce, encrypted_bytes)
            .map_err(|e| JsValue::from_str(&format!("Decryption failed: {}", e)))
    }
}

#[wasm_bindgen]
pub struct XChaCha20Poly1305 {
    secret_key: Vec<u8>,
}

#[wasm_bindgen]
impl XChaCha20Poly1305 {
    #[wasm_bindgen(constructor)]
    pub fn new(secret_bytes: &[u8]) -> Result<XChaCha20Poly1305, JsValue> {
        if secret_bytes.len() != SECRET_LENGTH {
            return Err(JsValue::from_str("Invalid secret key length !"));
        }
        Ok(XChaCha20Poly1305 {
            secret_key: secret_bytes.to_vec(),
        })
    }

    #[allow(deprecated)]
    pub fn encrypt(&self, nonce_bytes: &[u8], data: &[u8]) -> Result<Vec<u8>, JsValue> {
        if nonce_bytes.len() != XNONCE_LENGTH {
            return Err(JsValue::from_str("Invalid nonce key length !"));
        }
        let cipher = chacha20poly1305::XChaCha20Poly1305::new_from_slice(&self.secret_key)
            .map_err(|e| JsValue::from_str(&format!("Cipher initialization failed: {}", e)))?;

        let nonce = chacha20poly1305::XNonce::from_slice(nonce_bytes);

        cipher
            .encrypt(nonce, data)
            .map_err(|e| JsValue::from_str(&format!("Encryption failed: {}", e)))
    }

    #[allow(deprecated)]
    pub fn decrypt(&self, nonce_bytes: &[u8], encrypted_data: &[u8]) -> Result<Vec<u8>, JsValue> {
        if nonce_bytes.len() != XNONCE_LENGTH {
            return Err(JsValue::from_str("Invalid nonce key length !"));
        }
        let cipher = chacha20poly1305::XChaCha20Poly1305::new_from_slice(&self.secret_key)
            .map_err(|e| JsValue::from_str(&format!("Cipher initialization failed: {}", e)))?;

        let nonce = chacha20poly1305::XNonce::from_slice(nonce_bytes);

        cipher
            .decrypt(nonce, encrypted_data)
            .map_err(|e| JsValue::from_str(&format!("Decryption failed: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> Vec<u8> {
        let mut secret: [u8; SECRET_LENGTH] = [0; SECRET_LENGTH]; // Initialize with default values
        let _ = getrandom::getrandom(&mut secret);
        secret.to_vec()
    }

    fn test_nonce() -> Vec<u8> {
        let mut nonce: [u8; NONCE_LENGTH] = [0; NONCE_LENGTH]; // Initialize with default values
        let _ = getrandom::getrandom(&mut nonce);
        nonce.to_vec()
    }

    fn test_xnonce() -> Vec<u8> {
        let mut nonce: [u8; XNONCE_LENGTH] = [0; XNONCE_LENGTH]; // Initialize with default values
        let _ = getrandom::getrandom(&mut nonce);
        nonce.to_vec()
    }

    #[test]
    fn test_chacha20poly1305_encrypt_decrypt() {
        let key = test_key();
        let nonce = test_nonce();

        let plaintext = b"Hello, World! This is a test message.";

        let cipher = ChaCha20Poly1305::new(&key).unwrap();

        // Test encryption
        let encrypted = cipher.encrypt(&nonce, plaintext).unwrap();

        assert_ne!(encrypted, plaintext);
        assert!(encrypted.len() > plaintext.len()); // Should have auth tag

        // Test decryption
        let decrypted = cipher.decrypt(&nonce, &encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_xchacha20poly1305_encrypt_decrypt() {
        let key = test_key();
        let nonce = test_xnonce();

        let plaintext = b"Hello, World! This is a test message for XChaCha20.";

        let cipher = XChaCha20Poly1305::new(&key).unwrap();

        let encrypted = cipher.encrypt(&nonce, plaintext).unwrap();
        assert_ne!(encrypted, plaintext);

        let decrypted = cipher.decrypt(&nonce, &encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }
}
