use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::RngCore;
use sha2::{Digest, Sha256};

/// PKCE (Proof Key for Code Exchange) 구현
/// Reference: RFC 7636 - https://tools.ietf.org/html/rfc7636
/// 암호학적으로 안전한 code_verifier 생성
/// 43-128자 사이의 랜덤 문자열
pub fn generate_code_verifier() -> String {
    let mut bytes = [0u8; 64];
    rand::thread_rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

/// code_verifier로부터 code_challenge 생성 (S256 방식)
pub fn generate_code_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let hash = hasher.finalize();
    URL_SAFE_NO_PAD.encode(hash)
}

/// CSRF 방지를 위한 state 파라미터 생성
pub fn generate_state() -> String {
    let mut bytes = [0u8; 24];
    rand::thread_rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_verifier_length() {
        let verifier = generate_code_verifier();
        // 64 bytes -> 86 chars in base64url
        assert!(verifier.len() >= 43 && verifier.len() <= 128);
    }

    #[test]
    fn test_code_challenge() {
        let verifier = generate_code_verifier();
        let challenge = generate_code_challenge(&verifier);
        // SHA256 hash -> 32 bytes -> 43 chars in base64url
        assert_eq!(challenge.len(), 43);
    }

    #[test]
    fn test_state() {
        let state = generate_state();
        // 24 bytes -> 32 chars in base64url
        assert_eq!(state.len(), 32);
    }
}
