use super::super::series::column_from_pyobjects;
use super::super::{SeraDFrame, Series};
use super::{SecureDFrame_, SecureFrame};
use pyo3::prelude::*;
use seraplot_macros::sera_doc_impl;
use std::collections::HashMap;
use std::sync::Arc;

#[pyclass]
pub struct SecureDFrameBuilder {
    order: Vec<String>,
    columns: HashMap<String, Series>,
}

inventory::submit! {
    crate::doc_registry::FnDoc {
        name: "SecureDFrameBuilder",
        category: "data_method",
        module: "secure",
        file: "seraplot/pulse-pricing.md",
        en: "Chainable column-by-column builder for an encrypted dataframe.",
        fr: "Constructeur chainable colonne par colonne pour un dataframe chiffre.",
        params: &[],
        aliases: &[],
    }
}

#[sera_doc_impl]
#[pymethods]
impl SecureDFrameBuilder {
    #[new]
    fn new() -> Self {
        SecureDFrameBuilder {
            order: Vec::new(),
            columns: HashMap::new(),
        }
    }

    #[sera_doc(
        category = "data_method",
        module = "secure",
        file = "seraplot/pulse-pricing.md",
        en = "Adds or replaces a column. Chainable — returns self.",
        fr = "Ajoute ou remplace une colonne. Chainable — retourne self.",
        param(name = "name", ty = "str", en = "Column name.", fr = "Nom de la colonne."),
        param(name = "values", ty = "list", en = "Column values; dtype is inferred from the first element.", fr = "Valeurs de la colonne ; le type est deduit du premier element.")
    )]
    fn column<'py>(mut slf: PyRefMut<'py, Self>, name: &str, values: &Bound<'py, PyAny>) -> PyResult<PyRefMut<'py, Self>> {
        let items: Vec<Bound<'py, PyAny>> = values.extract()?;
        let series = column_from_pyobjects(items);
        if !slf.columns.contains_key(name) {
            slf.order.push(name.to_string());
        }
        slf.columns.insert(name.to_string(), series);
        Ok(slf)
    }

    #[sera_doc(
        category = "data_method",
        module = "secure",
        file = "seraplot/pulse-pricing.md",
        en = "Encrypts every column added so far into a SecureDFrame.",
        fr = "Chiffre toutes les colonnes ajoutees jusqu'ici en un SecureDFrame.",
        param(name = "key", ty = "bytes", en = "32-byte AES-256 key.", fr = "Cle AES-256 de 32 octets.")
    )]
    fn build(&self, key: Vec<u8>) -> PyResult<SecureDFrame_> {
        let nrows = self.columns.values().map(|c| c.len()).max().unwrap_or(0);
        let plain = SeraDFrame::from_parts(self.order.clone(), self.columns.clone(), nrows);
        let secure = SecureFrame::from_plain(&plain, &key)?;
        Ok(SecureDFrame_ { inner: Arc::new(secure) })
    }
}
