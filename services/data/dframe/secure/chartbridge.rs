use super::SecureDFrame_;
use crate::bindings::utils::data_processor::{bucket_downsample, bucket_downsample_multi};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use seraplot_macros::sera_doc_impl;

#[sera_doc_impl]
#[pymethods]
impl SecureDFrame_ {
    #[sera_doc(
        category = "data_method",
        module = "secure",
        file = "seraplot/pulse-pricing.md",
        en = "Decrypts exactly the two requested columns and downsamples them for charting in one step — never materializes a decrypted copy of the rest of the frame.",
        fr = "Dechiffre exactement les deux colonnes demandees et les sous-echantillonne pour l'affichage en une etape — ne materialise jamais de copie dechiffree du reste du frame.",
        param(name = "x", ty = "str", en = "Name of the column to use as x.", fr = "Nom de la colonne a utiliser comme x."),
        param(name = "y", ty = "str", en = "Name of the column to use as y.", fr = "Nom de la colonne a utiliser comme y."),
        param(name = "key", ty = "bytes", en = "32-byte AES-256 key.", fr = "Cle AES-256 de 32 octets."),
        param(name = "max_points", ty = "int", en = "Downsample target point count. Default: 2000.", fr = "Nombre de points cible apres sous-echantillonnage. Defaut : 2000."),
        param(name = "agg", ty = "str", en = "Aggregation used per bucket: 'mean', 'min', 'max', or 'first'. Default: 'mean'.", fr = "Agregation utilisee par case : 'mean', 'min', 'max' ou 'first'. Defaut : 'mean'.")
    )]
    #[pyo3(signature = (x, y, key, max_points = 2000, agg = "mean"))]
    fn to_chart_data(&self, py: Python<'_>, x: &str, y: &str, key: Vec<u8>, max_points: usize, agg: &str) -> PyResult<PyObject> {
        let xs = self.inner.decrypt_column(x, &key)?.to_f64_vec();
        let ys = self.inner.decrypt_column(y, &key)?.to_f64_vec();
        let (bx, by) = bucket_downsample(&xs, &ys, max_points, agg);
        let dict = PyDict::new_bound(py);
        dict.set_item("x", bx)?;
        dict.set_item("y", by)?;
        Ok(dict.into())
    }

    #[sera_doc(
        category = "data_method",
        module = "secure",
        file = "seraplot/pulse-pricing.md",
        en = "Decrypts one x column and several y columns and downsamples all of them together, in lock-step, for multi-series charting.",
        fr = "Dechiffre une colonne x et plusieurs colonnes y et les sous-echantillonne toutes ensemble, au meme rythme, pour un affichage multi-series.",
        param(name = "x", ty = "str", en = "Name of the column to use as x.", fr = "Nom de la colonne a utiliser comme x."),
        param(name = "ys", ty = "list[str]", en = "Names of the columns to use as y series.", fr = "Noms des colonnes a utiliser comme series y."),
        param(name = "key", ty = "bytes", en = "32-byte AES-256 key.", fr = "Cle AES-256 de 32 octets."),
        param(name = "max_points", ty = "int", en = "Downsample target point count. Default: 2000.", fr = "Nombre de points cible apres sous-echantillonnage. Defaut : 2000."),
        param(name = "agg", ty = "str", en = "Aggregation used per bucket: 'mean', 'min', 'max', or 'first'. Default: 'mean'.", fr = "Agregation utilisee par case : 'mean', 'min', 'max' ou 'first'. Defaut : 'mean'.")
    )]
    #[pyo3(signature = (x, ys, key, max_points = 2000, agg = "mean"))]
    fn to_chart_data_multi(&self, py: Python<'_>, x: &str, ys: Vec<String>, key: Vec<u8>, max_points: usize, agg: &str) -> PyResult<PyObject> {
        let xs = self.inner.decrypt_column(x, &key)?.to_f64_vec();
        let y_series: Vec<Vec<f64>> = ys
            .iter()
            .map(|c| self.inner.decrypt_column(c, &key).map(|s| s.to_f64_vec()))
            .collect::<PyResult<_>>()?;
        let y_refs: Vec<&[f64]> = y_series.iter().map(|v| v.as_slice()).collect();
        let (bx, bys) = bucket_downsample_multi(&xs, &y_refs, max_points, agg);
        let dict = PyDict::new_bound(py);
        dict.set_item("x", bx)?;
        let series_dict = PyDict::new_bound(py);
        for (name, series) in ys.iter().zip(bys.into_iter()) {
            series_dict.set_item(name, series)?;
        }
        dict.set_item("series", series_dict)?;
        Ok(dict.into())
    }
}

#[cfg(test)]
mod tests {
    use super::super::{SecureFrame, SecureDFrame_};
    use super::super::super::{SeraDFrame, Series};
    use crate::bindings::utils::data_processor::bucket_downsample;
    use std::collections::HashMap;
    use std::sync::Arc;

    const KEY: [u8; 32] = [11u8; 32];

    fn secure_frame_with_two_numeric_columns(n: usize) -> SecureDFrame_ {
        let a: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let b: Vec<f64> = (0..n).map(|i| (i as f64) * 2.0).collect();
        let mut columns = HashMap::new();
        columns.insert("x".to_string(), Series::Num(Arc::new(a)));
        columns.insert("y".to_string(), Series::Num(Arc::new(b)));
        let plain = SeraDFrame::from_parts(vec!["x".to_string(), "y".to_string()], columns, n);
        let inner = SecureFrame::from_plain(&plain, &KEY).unwrap();
        SecureDFrame_ { inner: Arc::new(inner) }
    }

    #[test]
    fn chart_bridge_downsample_matches_manual_decrypt_and_downsample() {
        let n = 5_000usize;
        let max_points = 100usize;
        let frame = secure_frame_with_two_numeric_columns(n);
        let xs = frame.inner.decrypt_column("x", &KEY).unwrap().to_f64_vec();
        let ys = frame.inner.decrypt_column("y", &KEY).unwrap().to_f64_vec();
        let (bx, by) = bucket_downsample(&xs, &ys, max_points, "mean");
        assert!(bx.len() <= max_points);
        assert_eq!(bx.len(), by.len());
        assert!(bx.windows(2).all(|w| w[0] <= w[1]));

        let chunk = (n + max_points - 1) / max_points;
        for (i, (&bxi, &byi)) in bx.iter().zip(by.iter()).enumerate() {
            let start = i * chunk;
            let end = (start + chunk).min(n);
            let expected_x = xs[start + (end - start) / 2];
            let expected_y_mean: f64 = ys[start..end].iter().sum::<f64>() / (end - start) as f64;
            assert!((bxi - expected_x).abs() < 1e-9, "bucket {i}: x should be the real midpoint sample, decrypted correctly");
            assert!((byi - expected_y_mean).abs() < 1e-6, "bucket {i}: y should be the true mean of the decrypted bucket");
        }
    }

    #[test]
    fn chart_bridge_rejects_unknown_column() {
        let frame = secure_frame_with_two_numeric_columns(10);
        assert!(frame.inner.decrypt_column("does_not_exist", &KEY).is_err());
    }
}
