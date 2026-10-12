use super::super::Series;
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::PyResult;
use std::sync::Arc;
use zeroize::Zeroizing;

pub const KEY_LEN: usize = 32;
pub const NONCE_LEN: usize = 12;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ColDType {
    Num,
    Str,
    Bool,
}

pub fn parse_key(raw: &[u8]) -> PyResult<Key<Aes256Gcm>> {
    if raw.len() != KEY_LEN {
        return Err(PyValueError::new_err(format!(
            "sera-secure: key must be {} bytes, got {}",
            KEY_LEN,
            raw.len()
        )));
    }
    Ok(*Key::<Aes256Gcm>::from_slice(raw))
}

fn serialize_series(series: &Series) -> (ColDType, Zeroizing<Vec<u8>>) {
    match series {
        Series::Num(v) => {
            let mut buf = Zeroizing::new(Vec::with_capacity(v.len() * 8));
            for f in v.iter() {
                buf.extend_from_slice(&f.to_bits().to_le_bytes());
            }
            (ColDType::Num, buf)
        }
        Series::Bool(v) => {
            let mut buf = Zeroizing::new(Vec::with_capacity(v.len()));
            for b in v.iter() {
                buf.push(*b as u8);
            }
            (ColDType::Bool, buf)
        }
        Series::Str(v) => {
            let mut buf = Zeroizing::new(Vec::new());
            for s in v.iter() {
                let bytes = s.as_bytes();
                buf.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
                buf.extend_from_slice(bytes);
            }
            (ColDType::Str, buf)
        }
    }
}

fn deserialize_series(dtype: ColDType, len_hint: usize, plaintext: &[u8]) -> PyResult<Series> {
    match dtype {
        ColDType::Num => {
            if plaintext.len() % 8 != 0 {
                return Err(PyRuntimeError::new_err("sera-secure: corrupt numeric column"));
            }
            let vals: Vec<f64> = plaintext
                .chunks_exact(8)
                .map(|c| f64::from_bits(u64::from_le_bytes(c.try_into().unwrap())))
                .collect();
            Ok(Series::Num(Arc::new(vals)))
        }
        ColDType::Bool => {
            let vals: Vec<bool> = plaintext.iter().map(|b| *b != 0).collect();
            Ok(Series::Bool(Arc::new(vals)))
        }
        ColDType::Str => {
            let mut vals = Vec::with_capacity(len_hint);
            let mut i = 0usize;
            while i + 4 <= plaintext.len() {
                let n = u32::from_le_bytes(plaintext[i..i + 4].try_into().unwrap()) as usize;
                i += 4;
                if i + n > plaintext.len() {
                    return Err(PyRuntimeError::new_err("sera-secure: corrupt string column"));
                }
                let s = std::str::from_utf8(&plaintext[i..i + n])
                    .map_err(|_| PyRuntimeError::new_err("sera-secure: corrupt utf8 in string column"))?;
                vals.push(Arc::from(s));
                i += n;
            }
            Ok(Series::Str(Arc::new(vals)))
        }
    }
}

pub struct EncryptedColumn {
    pub dtype: ColDType,
    pub len: usize,
    pub nonce: [u8; NONCE_LEN],
    pub ciphertext: Vec<u8>,
}

pub fn encrypt_series(key: &Key<Aes256Gcm>, series: &Series) -> PyResult<EncryptedColumn> {
    let (dtype, plaintext) = serialize_series(series);
    let cipher = Aes256Gcm::new(key);
    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_slice())
        .map_err(|_| PyRuntimeError::new_err("sera-secure: encryption failed"))?;
    Ok(EncryptedColumn {
        dtype,
        len: series.len(),
        nonce: nonce_bytes,
        ciphertext,
    })
}

pub fn decrypt_series(key: &Key<Aes256Gcm>, col: &EncryptedColumn) -> PyResult<Series> {
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&col.nonce);
    let plaintext = Zeroizing::new(
        cipher
            .decrypt(nonce, col.ciphertext.as_slice())
            .map_err(|_| PyRuntimeError::new_err("sera-secure: decryption failed (wrong key or corrupted data)"))?,
    );
    deserialize_series(col.dtype, col.len, plaintext.as_slice())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key_a() -> Key<Aes256Gcm> {
        parse_key(&[7u8; KEY_LEN]).unwrap()
    }

    fn key_b() -> Key<Aes256Gcm> {
        parse_key(&[9u8; KEY_LEN]).unwrap()
    }

    fn series_eq(a: &Series, b: &Series) -> bool {
        match (a, b) {
            (Series::Num(x), Series::Num(y)) => x.iter().zip(y.iter()).all(|(p, q)| p.to_bits() == q.to_bits()) && x.len() == y.len(),
            (Series::Str(x), Series::Str(y)) => x == y,
            (Series::Bool(x), Series::Bool(y)) => x == y,
            _ => false,
        }
    }

    #[test]
    fn round_trip_preserves_numeric_column() {
        let series = Series::Num(Arc::new(vec![1.5, -2.25, 0.0, f64::MAX, f64::MIN_POSITIVE]));
        let enc = encrypt_series(&key_a(), &series).unwrap();
        let dec = decrypt_series(&key_a(), &enc).unwrap();
        assert!(series_eq(&series, &dec));
    }

    #[test]
    fn round_trip_preserves_string_column_including_unicode() {
        let series = Series::Str(Arc::new(vec![
            std::sync::Arc::from("hello"),
            std::sync::Arc::from(""),
            std::sync::Arc::from("unicode: h\u{e9}llo \u{65e5}\u{672c}\u{8a9e}"),
        ]));
        let enc = encrypt_series(&key_a(), &series).unwrap();
        let dec = decrypt_series(&key_a(), &enc).unwrap();
        assert!(series_eq(&series, &dec));
    }

    #[test]
    fn round_trip_preserves_bool_column() {
        let series = Series::Bool(Arc::new(vec![true, false, true, true, false]));
        let enc = encrypt_series(&key_a(), &series).unwrap();
        let dec = decrypt_series(&key_a(), &enc).unwrap();
        assert!(series_eq(&series, &dec));
    }

    #[test]
    fn round_trip_preserves_empty_column() {
        let series = Series::Num(Arc::new(vec![]));
        let enc = encrypt_series(&key_a(), &series).unwrap();
        let dec = decrypt_series(&key_a(), &enc).unwrap();
        assert!(series_eq(&series, &dec));
    }

    #[test]
    fn tampered_ciphertext_fails_to_decrypt() {
        let series = Series::Num(Arc::new(vec![1.0, 2.0, 3.0]));
        let mut enc = encrypt_series(&key_a(), &series).unwrap();
        enc.ciphertext[0] ^= 0xFF;
        assert!(decrypt_series(&key_a(), &enc).is_err());
    }

    #[test]
    fn wrong_key_fails_to_decrypt() {
        let series = Series::Str(Arc::new(vec![std::sync::Arc::from("secret")]));
        let enc = encrypt_series(&key_a(), &series).unwrap();
        assert!(decrypt_series(&key_b(), &enc).is_err());
    }

    #[test]
    fn each_encryption_uses_a_fresh_nonce_and_produces_different_ciphertext() {
        let series = Series::Num(Arc::new(vec![42.0]));
        let enc1 = encrypt_series(&key_a(), &series).unwrap();
        let enc2 = encrypt_series(&key_a(), &series).unwrap();
        assert_ne!(enc1.nonce, enc2.nonce);
        assert_ne!(enc1.ciphertext, enc2.ciphertext);
        assert!(series_eq(&decrypt_series(&key_a(), &enc1).unwrap(), &series));
        assert!(series_eq(&decrypt_series(&key_a(), &enc2).unwrap(), &series));
    }

    #[test]
    fn parse_key_rejects_wrong_length() {
        assert!(parse_key(&[0u8; 16]).is_err());
        assert!(parse_key(&[0u8; 33]).is_err());
        assert!(parse_key(&[0u8; KEY_LEN]).is_ok());
    }
}
