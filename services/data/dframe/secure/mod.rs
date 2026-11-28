mod crypto;
mod builder;
mod chartbridge;
mod key;

pub use builder::SecureDFrameBuilder;
pub use key::SeraKey_;

use super::{SeraDFrame, SeraDFrame_};
use crypto::{decrypt_series, encrypt_series, parse_key, EncryptedColumn};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rayon::prelude::*;
use seraplot_macros::sera_doc_impl;
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) struct SecureFrame {
    order: Vec<String>,
    columns: HashMap<String, EncryptedColumn>,
    nrows: usize,
}

impl SecureFrame {
    fn from_plain(frame: &SeraDFrame, key_bytes: &[u8]) -> PyResult<Self> {
        let key = parse_key(key_bytes)?;
        let columns: HashMap<String, EncryptedColumn> = frame
            .order
            .par_iter()
            .map(|name| {
                let series = &frame.columns[name];
                encrypt_series(&key, series).map(|enc| (name.clone(), enc))
            })
            .collect::<PyResult<Vec<_>>>()?
            .into_iter()
            .collect();
        Ok(SecureFrame {
            order: frame.order.clone(),
            columns,
            nrows: frame.nrows,
        })
    }

    fn decrypt_column(&self, name: &str, key_bytes: &[u8]) -> PyResult<super::Series> {
        let key = parse_key(key_bytes)?;
        let enc = self.columns.get(name).ok_or_else(|| {
            pyo3::exceptions::PyKeyError::new_err(format!("sera-secure: no column '{}'", name))
        })?;
        decrypt_series(&key, enc)
    }

    fn decrypt_all(&self, key_bytes: &[u8]) -> PyResult<SeraDFrame> {
        let key = parse_key(key_bytes)?;
        let columns: HashMap<String, super::Series> = self
            .order
            .par_iter()
            .map(|name| decrypt_series(&key, &self.columns[name]).map(|s| (name.clone(), s)))
            .collect::<PyResult<Vec<_>>>()?
            .into_iter()
            .collect();
        Ok(SeraDFrame::from_parts(self.order.clone(), columns, self.nrows))
    }

    fn byte_size(&self) -> u64 {
        self.columns.values().map(|c| c.ciphertext.len() as u64).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Series as PlainSeries;

    const KEY: [u8; 32] = [11u8; 32];
    const OTHER_KEY: [u8; 32] = [22u8; 32];

    fn series_eq(a: &PlainSeries, b: &PlainSeries) -> bool {
        match (a, b) {
            (PlainSeries::Num(x), PlainSeries::Num(y)) => {
                x.len() == y.len() && x.iter().zip(y.iter()).all(|(p, q)| p.to_bits() == q.to_bits())
            }
            (PlainSeries::Str(x), PlainSeries::Str(y)) => x == y,
            (PlainSeries::Bool(x), PlainSeries::Bool(y)) => x == y,
            _ => false,
        }
    }

    fn sample_frame(nrows: usize) -> SeraDFrame {
        let nums: Vec<f64> = (0..nrows).map(|i| i as f64 * 1.5).collect();
        let strs: Vec<Arc<str>> = (0..nrows).map(|i| Arc::from(format!("row-{i}").as_str())).collect();
        let bools: Vec<bool> = (0..nrows).map(|i| i % 3 == 0).collect();
        let mut columns = HashMap::new();
        columns.insert("value".to_string(), PlainSeries::Num(Arc::new(nums)));
        columns.insert("label".to_string(), PlainSeries::Str(Arc::new(strs)));
        columns.insert("flag".to_string(), PlainSeries::Bool(Arc::new(bools)));
        SeraDFrame::from_parts(vec!["value".to_string(), "label".to_string(), "flag".to_string()], columns, nrows)
    }

    #[test]
    fn round_trip_multi_column_multi_dtype_frame() {
        let plain = sample_frame(50);
        let secure = SecureFrame::from_plain(&plain, &KEY).unwrap();
        let decrypted = secure.decrypt_all(&KEY).unwrap();
        assert_eq!(decrypted.order, plain.order);
        assert_eq!(decrypted.nrows, plain.nrows);
        for name in &plain.order {
            assert!(series_eq(&plain.columns[name], &decrypted.columns[name]), "column '{name}' mismatch after round-trip");
        }
    }

    #[test]
    fn decrypt_column_returns_only_the_requested_column_correctly() {
        let plain = sample_frame(20);
        let secure = SecureFrame::from_plain(&plain, &KEY).unwrap();
        let value = secure.decrypt_column("value", &KEY).unwrap();
        assert!(series_eq(&value, &plain.columns["value"]));
    }

    #[test]
    fn decrypt_with_wrong_key_fails() {
        let plain = sample_frame(10);
        let secure = SecureFrame::from_plain(&plain, &KEY).unwrap();
        assert!(secure.decrypt_all(&OTHER_KEY).is_err());
        assert!(secure.decrypt_column("value", &OTHER_KEY).is_err());
    }

    #[test]
    fn unknown_column_name_errors() {
        let plain = sample_frame(5);
        let secure = SecureFrame::from_plain(&plain, &KEY).unwrap();
        assert!(secure.decrypt_column("does_not_exist", &KEY).is_err());
    }

    #[test]
    fn byte_size_is_nonzero_and_grows_with_row_count() {
        let small = SecureFrame::from_plain(&sample_frame(10), &KEY).unwrap();
        let large = SecureFrame::from_plain(&sample_frame(1000), &KEY).unwrap();
        assert!(small.byte_size() > 0);
        assert!(large.byte_size() > small.byte_size());
    }

    #[test]
    fn encrypt_decrypt_throughput_on_a_realistic_frame() {
        let nrows = 500_000;
        let plain = sample_frame(nrows);
        let plain_bytes: u64 = plain.columns.values().map(|s| s.byte_size()).sum();

        let t0 = std::time::Instant::now();
        let secure = SecureFrame::from_plain(&plain, &KEY).unwrap();
        let encrypt_elapsed = t0.elapsed();

        let t1 = std::time::Instant::now();
        let decrypted = secure.decrypt_all(&KEY).unwrap();
        let decrypt_elapsed = t1.elapsed();

        assert_eq!(decrypted.nrows, nrows);
        let mb = plain_bytes as f64 / (1024.0 * 1024.0);
        println!(
            "sera-secure perf: {nrows} rows, {mb:.2} MiB plaintext -> encrypt {:.1} MiB/s ({:?}), decrypt {:.1} MiB/s ({:?})",
            mb / encrypt_elapsed.as_secs_f64().max(1e-9),
            encrypt_elapsed,
            mb / decrypt_elapsed.as_secs_f64().max(1e-9),
            decrypt_elapsed,
        );
        assert!(encrypt_elapsed.as_secs_f64() < 5.0, "encryption unexpectedly slow: {encrypt_elapsed:?}");
        assert!(decrypt_elapsed.as_secs_f64() < 5.0, "decryption unexpectedly slow: {decrypt_elapsed:?}");
    }
}

#[pyclass(name = "SecureDFrame")]
pub struct SecureDFrame_ {
    pub(crate) inner: Arc<SecureFrame>,
}

inventory::submit! {
    crate::doc_registry::FnDoc {
        name: "SecureDFrame",
        category: "data_method",
        module: "secure",
        file: "seraplot/pulse-pricing.md",
        en: "AES-256-GCM encrypted columnar dataframe; columns are decrypted only transiently, for the duration of a single read.",
        fr: "Dataframe colonnaire chiffre en AES-256-GCM ; les colonnes ne sont dechiffrees que le temps d'une lecture ponctuelle.",
        params: &[],
        aliases: &[],
    }
}

#[sera_doc_impl]
#[pymethods]
impl SecureDFrame_ {
    #[new]
    fn new(dict: &Bound<'_, PyDict>, key: Vec<u8>) -> PyResult<Self> {
        let plain = SeraDFrame::dict_to_frame(dict)?;
        let secure = SecureFrame::from_plain(&plain, &key)?;
        Ok(SecureDFrame_ { inner: Arc::new(secure) })
    }

    #[sera_doc(
        category = "data_method",
        module = "secure",
        file = "seraplot/pulse-pricing.md",
        en = "Decrypts every column and returns a plain, in-memory SeraDFrame. Prefer decrypt_column() or to_chart_data() when only some columns are needed.",
        fr = "Dechiffre toutes les colonnes et retourne un SeraDFrame en clair, en memoire. Preferer decrypt_column() ou to_chart_data() quand seules certaines colonnes sont necessaires.",
        param(name = "key", ty = "bytes", en = "32-byte AES-256 key.", fr = "Cle AES-256 de 32 octets.")
    )]
    fn decrypt(&self, key: Vec<u8>) -> PyResult<SeraDFrame_> {
        let plain = self.inner.decrypt_all(&key)?;
        Ok(SeraDFrame_ { inner: Arc::new(plain) })
    }

    #[sera_doc(
        category = "data_method",
        module = "secure",
        file = "seraplot/pulse-pricing.md",
        en = "Decrypts and returns a single column, without touching the rest of the frame.",
        fr = "Dechiffre et retourne une seule colonne, sans toucher au reste du frame.",
        param(name = "name", ty = "str", en = "Column name.", fr = "Nom de la colonne."),
        param(name = "key", ty = "bytes", en = "32-byte AES-256 key.", fr = "Cle AES-256 de 32 octets.")
    )]
    fn decrypt_column(&self, py: Python<'_>, name: &str, key: Vec<u8>) -> PyResult<PyObject> {
        let series = self.inner.decrypt_column(name, &key)?;
        Ok(series.into_py(py))
    }

    #[sera_doc(
        category = "data_method",
        module = "secure",
        file = "seraplot/pulse-pricing.md",
        en = "Column names, in original order. Does not require the key — names are not encrypted.",
        fr = "Noms des colonnes, dans l'ordre d'origine. Ne necessite pas la cle — les noms ne sont pas chiffres."
    )]
    #[getter]
    fn columns(&self) -> Vec<String> {
        self.inner.order.clone()
    }

    #[sera_doc(
        category = "data_method",
        module = "secure",
        file = "seraplot/pulse-pricing.md",
        en = "Row count. Does not require the key.",
        fr = "Nombre de lignes. Ne necessite pas la cle."
    )]
    #[getter]
    fn nrows(&self) -> usize {
        self.inner.nrows
    }

    #[sera_doc(
        category = "data_method",
        module = "secure",
        file = "seraplot/pulse-pricing.md",
        en = "Total ciphertext size in bytes across all columns. Does not require the key.",
        fr = "Taille totale du texte chiffre en octets, toutes colonnes confondues. Ne necessite pas la cle."
    )]
    #[getter]
    fn encrypted_size(&self) -> u64 {
        self.inner.byte_size()
    }
}
