use super::crypto::KEY_LEN;
use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::aead::OsRng;
use argon2::Argon2;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use seraplot_macros::sera_doc_impl;
use zeroize::Zeroizing;

pub const SALT_LEN: usize = 16;

fn random_bytes<const N: usize>() -> [u8; N] {
    let mut out = [0u8; N];
    OsRng.fill_bytes(&mut out);
    out
}

fn derive_from_passphrase(passphrase: &str, salt: &[u8]) -> PyResult<[u8; KEY_LEN]> {
    let mut out = [0u8; KEY_LEN];
    Argon2::default()
        .hash_password_into(passphrase.as_bytes(), salt, &mut out)
        .map_err(|e| PyRuntimeError::new_err(format!("sera-secure: key derivation failed: {}", e)))?;
    Ok(out)
}

#[pyclass(name = "SeraKey")]
pub struct SeraKey_ {
    bytes: Zeroizing<[u8; KEY_LEN]>,
}

inventory::submit! {
    crate::doc_registry::FnDoc {
        name: "SeraKey",
        category: "data_method",
        module: "secure",
        file: "seraplot/pulse-pricing.md",
        en: "AES-256 key material for SecureDFrame: random generation, or Argon2id derivation from a passphrase and salt. Never logs or exposes raw bytes except via to_bytes().",
        fr: "Materiel de cle AES-256 pour SecureDFrame : generation aleatoire, ou derivation Argon2id depuis une phrase secrete et un sel. Ne journalise ni n'expose les octets bruts sauf via to_bytes().",
        params: &[],
        aliases: &[],
    }
}

#[sera_doc_impl]
#[pymethods]
impl SeraKey_ {
    #[sera_doc(
        category = "data_method",
        module = "secure",
        file = "seraplot/pulse-pricing.md",
        en = "Generates a fresh random 256-bit key, suitable for SecureDFrame. Not derived from anything memorable — save to_bytes() somewhere durable, or use from_passphrase() instead if you'd rather remember a passphrase.",
        fr = "Genere une nouvelle cle aleatoire de 256 bits, adaptee a SecureDFrame. N'est derivee de rien de memorisable — sauvegardez to_bytes() quelque part de durable, ou utilisez from_passphrase() si vous preferez retenir une phrase secrete."
    )]
    #[staticmethod]
    fn generate() -> Self {
        SeraKey_ { bytes: Zeroizing::new(random_bytes::<KEY_LEN>()) }
    }

    #[sera_doc(
        category = "data_method",
        module = "secure",
        file = "seraplot/pulse-pricing.md",
        en = "Generates a fresh random salt for from_passphrase(). Salts are not secret — store or transmit alongside the ciphertext.",
        fr = "Genere un sel aleatoire pour from_passphrase(). Les sels ne sont pas secrets — a stocker ou transmettre avec le texte chiffre."
    )]
    #[staticmethod]
    fn generate_salt() -> Vec<u8> {
        random_bytes::<SALT_LEN>().to_vec()
    }

    #[sera_doc(
        category = "data_method",
        module = "secure",
        file = "seraplot/pulse-pricing.md",
        en = "Derives a key from a human passphrase and salt via Argon2id — the recommended way to turn something memorable into a real key. Deterministic: the same passphrase and salt always derive the same key.",
        fr = "Derive une cle depuis une phrase secrete humaine et un sel via Argon2id — la maniere recommandee de transformer quelque chose de memorisable en une vraie cle. Deterministe : la meme phrase et le meme sel derivent toujours la meme cle.",
        param(name = "passphrase", ty = "str", en = "Human passphrase.", fr = "Phrase secrete humaine."),
        param(name = "salt", ty = "bytes", en = "At least 16 bytes; use generate_salt() for a fresh one.", fr = "Au moins 16 octets ; utiliser generate_salt() pour en generer un nouveau.")
    )]
    #[staticmethod]
    fn from_passphrase(passphrase: &str, salt: Vec<u8>) -> PyResult<Self> {
        if salt.len() < SALT_LEN {
            return Err(PyValueError::new_err(format!(
                "sera-secure: salt must be at least {} bytes, got {}",
                SALT_LEN,
                salt.len()
            )));
        }
        let derived = derive_from_passphrase(passphrase, &salt)?;
        Ok(SeraKey_ { bytes: Zeroizing::new(derived) })
    }

    #[sera_doc(
        category = "data_method",
        module = "secure",
        file = "seraplot/pulse-pricing.md",
        en = "Wraps 32 raw key bytes from elsewhere (a secrets manager, an existing key file) as a SeraKey.",
        fr = "Enveloppe 32 octets de cle bruts venant d'ailleurs (un gestionnaire de secrets, un fichier de cle existant) en un SeraKey.",
        param(name = "raw", ty = "bytes", en = "Exactly 32 bytes.", fr = "Exactement 32 octets.")
    )]
    #[staticmethod]
    fn from_bytes(raw: Vec<u8>) -> PyResult<Self> {
        if raw.len() != KEY_LEN {
            return Err(PyValueError::new_err(format!(
                "sera-secure: key must be {} bytes, got {}",
                KEY_LEN,
                raw.len()
            )));
        }
        let mut bytes = [0u8; KEY_LEN];
        bytes.copy_from_slice(&raw);
        Ok(SeraKey_ { bytes: Zeroizing::new(bytes) })
    }

    #[sera_doc(
        category = "data_method",
        module = "secure",
        file = "seraplot/pulse-pricing.md",
        en = "Returns the raw 32 key bytes, to pass to SecureDFrame or to persist yourself. The only way this key's bytes ever leave the SeraKey object.",
        fr = "Retourne les 32 octets bruts de la cle, a passer a SecureDFrame ou a persister soi-meme. La seule maniere dont les octets de cette cle sortent jamais de l'objet SeraKey."
    )]
    fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_passphrase_and_salt_derive_the_same_key() {
        let salt = vec![1u8; SALT_LEN];
        let a = derive_from_passphrase("correct horse battery staple", &salt).unwrap();
        let b = derive_from_passphrase("correct horse battery staple", &salt).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn different_salt_derives_a_different_key() {
        let a = derive_from_passphrase("same passphrase", &[1u8; SALT_LEN]).unwrap();
        let b = derive_from_passphrase("same passphrase", &[2u8; SALT_LEN]).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn different_passphrase_derives_a_different_key() {
        let salt = vec![5u8; SALT_LEN];
        let a = derive_from_passphrase("passphrase one", &salt).unwrap();
        let b = derive_from_passphrase("passphrase two", &salt).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn generate_produces_key_len_bytes_and_varies_each_call() {
        let a = SeraKey_::generate();
        let b = SeraKey_::generate();
        assert_eq!(a.to_bytes().len(), KEY_LEN);
        assert_ne!(a.to_bytes(), b.to_bytes());
    }

    #[test]
    fn generate_salt_produces_salt_len_bytes_and_varies_each_call() {
        let a = SeraKey_::generate_salt();
        let b = SeraKey_::generate_salt();
        assert_eq!(a.len(), SALT_LEN);
        assert_ne!(a, b);
    }

    #[test]
    fn from_bytes_round_trips_exactly() {
        let raw = vec![3u8; KEY_LEN];
        let k = SeraKey_::from_bytes(raw.clone()).unwrap();
        assert_eq!(k.to_bytes(), raw);
    }

    #[test]
    fn from_bytes_rejects_wrong_length() {
        assert!(SeraKey_::from_bytes(vec![0u8; KEY_LEN - 1]).is_err());
        assert!(SeraKey_::from_bytes(vec![0u8; KEY_LEN + 1]).is_err());
    }

    #[test]
    fn from_passphrase_rejects_salt_shorter_than_minimum() {
        assert!(SeraKey_::from_passphrase("pw", vec![0u8; SALT_LEN - 1]).is_err());
        assert!(SeraKey_::from_passphrase("pw", vec![0u8; SALT_LEN]).is_ok());
    }

    #[test]
    fn from_passphrase_matches_direct_derivation() {
        let salt = SeraKey_::generate_salt();
        let via_wrapper = SeraKey_::from_passphrase("shared secret", salt.clone()).unwrap();
        let direct = derive_from_passphrase("shared secret", &salt).unwrap();
        assert_eq!(via_wrapper.to_bytes(), direct.to_vec());
    }
}
