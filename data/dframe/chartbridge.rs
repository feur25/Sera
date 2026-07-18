use super::SeraDFrame_;
use crate::bindings::utils::data_processor::{bucket_downsample, bucket_downsample_multi};
use crate::sera_doc_impl;
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[pyo3(signature = (x, y, max_points = 2000, agg = "mean"))]
    #[sera_doc(
        name = "SeraDFrame.to_chart_data",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Downsamples this frame into chart-ready x/y arrays via parallel bucket aggregation (mean/sum/min/max/first/last/median), capping the output at max_points regardless of input size. Feed the result straight into sp.line(**d) / sp.bar(**d) / sp.scatter(**d).",
        fr = "Sous-echantillonne ce frame en tableaux x/y prets pour un graphique via agregation par buckets parallele (mean/sum/min/max/first/last/median), plafonnant la sortie a max_points quelle que soit la taille d'entree. Passez le resultat directement a sp.line(**d) / sp.bar(**d) / sp.scatter(**d)."
    )]
    fn to_chart_data(&self, py: Python<'_>, x: &str, y: &str, max_points: usize, agg: &str) -> PyResult<PyObject> {
        let xs = self.inner.get(x)?.to_f64_vec();
        let ys = self.inner.get(y)?.to_f64_vec();
        let (bx, by) = bucket_downsample(&xs, &ys, max_points, agg);
        let dict = PyDict::new_bound(py);
        dict.set_item("x", bx)?;
        dict.set_item("y", by)?;
        Ok(dict.into())
    }

    #[pyo3(signature = (x, ys, max_points = 2000, agg = "mean"))]
    #[sera_doc(
        name = "SeraDFrame.to_chart_data_multi",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Same as to_chart_data but downsamples several y columns at once against shared x buckets, for multi-series charts.",
        fr = "Comme to_chart_data mais sous-echantillonne plusieurs colonnes y d'un coup sur des buckets x partages, pour les graphiques multi-series."
    )]
    fn to_chart_data_multi(&self, py: Python<'_>, x: &str, ys: Vec<String>, max_points: usize, agg: &str) -> PyResult<PyObject> {
        let xs = self.inner.get(x)?.to_f64_vec();
        let y_series: Vec<Vec<f64>> = ys
            .iter()
            .map(|c| self.inner.get(c).map(|s| s.to_f64_vec()))
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
