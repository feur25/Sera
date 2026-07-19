use super::super::series::Series;
use super::super::{str_series, SeraDFrame, SeraDFrame_};
use crate::sera_doc_impl;
use pyo3::prelude::*;
use rayon::prelude::*;
use std::sync::Arc;

impl SeraDFrame_ {
    fn str_transform(&self, col: &str, f: impl Fn(&str) -> String + Sync) -> PyResult<SeraDFrame_> {
        let series = self.inner.get(col)?;
        let vals = series.to_str_vec();
        let out: Vec<String> = vals.par_iter().map(|s| f(s)).collect();
        let mut columns = self.inner.columns.clone();
        columns.insert(col.to_string(), str_series(out));
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(self.inner.order.clone(), columns, self.inner.nrows)),
        })
    }
}

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[sera_doc(
        name = "SeraDFrame.str_upper",
        category = "data_method", file = "canvas/dframe.md", en = "Uppercases a string column.", fr = "Met une colonne texte en majuscules.")]
    fn str_upper(&self, col: &str) -> PyResult<SeraDFrame_> {
        self.str_transform(col, |s| s.to_uppercase())
    }

    #[sera_doc(
        name = "SeraDFrame.str_lower",
        category = "data_method", file = "canvas/dframe.md", en = "Lowercases a string column.", fr = "Met une colonne texte en minuscules.")]
    fn str_lower(&self, col: &str) -> PyResult<SeraDFrame_> {
        self.str_transform(col, |s| s.to_lowercase())
    }

    #[sera_doc(
        name = "SeraDFrame.str_strip",
        category = "data_method", file = "canvas/dframe.md", en = "Trims whitespace from a string column.", fr = "Retire les espaces d'une colonne texte.")]
    fn str_strip(&self, col: &str) -> PyResult<SeraDFrame_> {
        self.str_transform(col, |s| s.trim().to_string())
    }

    #[sera_doc(
        name = "SeraDFrame.str_replace",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Replaces a substring in every value of a column.",
        fr = "Remplace une sous-chaine dans chaque valeur d'une colonne."
    )]
    fn str_replace(&self, col: &str, pattern: &str, replacement: &str) -> PyResult<SeraDFrame_> {
        self.str_transform(col, |s| s.replace(pattern, replacement))
    }

    #[sera_doc(
        name = "SeraDFrame.str_len",
        category = "data_method", file = "canvas/dframe.md", en = "Adds {col}_len, character count per value.", fr = "Ajoute {col}_len, nombre de caracteres par valeur.")]
    fn str_len(&self, col: &str) -> PyResult<SeraDFrame_> {
        let series = self.inner.get(col)?;
        let vals = series.to_str_vec();
        let out: Vec<f64> = vals.par_iter().map(|s| s.chars().count() as f64).collect();
        self.push_derived(col, "len", Series::Num(Arc::new(out)))
    }

    #[pyo3(signature = (col, sep, index))]
    #[sera_doc(
        name = "SeraDFrame.str_split_get",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_splitN, the Nth field after splitting by a separator.",
        fr = "Ajoute {col}_splitN, le Nieme champ apres decoupage par separateur."
    )]
    fn str_split_get(&self, col: &str, sep: &str, index: usize) -> PyResult<SeraDFrame_> {
        let series = self.inner.get(col)?;
        let vals = series.to_str_vec();
        let out: Vec<String> = vals
            .par_iter()
            .map(|s| s.split(sep).nth(index).unwrap_or("").to_string())
            .collect();
        self.push_derived(col, &format!("split{}", index), str_series(out))
    }

    #[sera_doc(
        name = "SeraDFrame.str_contains",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Boolean mask of values containing a substring.",
        fr = "Masque booleen des valeurs contenant une sous-chaine."
    )]
    fn str_contains(&self, col: &str, pattern: &str) -> PyResult<Vec<bool>> {
        let series = self.inner.get(col)?;
        let owned;
        let strs: &[Arc<str>] = match series.as_str_slice() {
            Some(v) => v,
            None => {
                owned = series.to_str_vec().into_iter().map(Arc::from).collect::<Vec<_>>();
                &owned
            }
        };
        Ok(strs.par_iter().map(|s| s.contains(pattern)).collect())
    }

    #[sera_doc(
        name = "SeraDFrame.str_startswith",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Boolean mask of values starting with a prefix.",
        fr = "Masque booleen des valeurs commencant par un prefixe."
    )]
    fn str_startswith(&self, col: &str, prefix: &str) -> PyResult<Vec<bool>> {
        let series = self.inner.get(col)?;
        let owned;
        let strs: &[Arc<str>] = match series.as_str_slice() {
            Some(v) => v,
            None => {
                owned = series.to_str_vec().into_iter().map(Arc::from).collect::<Vec<_>>();
                &owned
            }
        };
        Ok(strs.par_iter().map(|s| s.starts_with(prefix)).collect())
    }

    #[sera_doc(
        name = "SeraDFrame.str_endswith",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Boolean mask of values ending with a suffix.",
        fr = "Masque booleen des valeurs finissant par un suffixe."
    )]
    fn str_endswith(&self, col: &str, suffix: &str) -> PyResult<Vec<bool>> {
        let series = self.inner.get(col)?;
        let owned;
        let strs: &[Arc<str>] = match series.as_str_slice() {
            Some(v) => v,
            None => {
                owned = series.to_str_vec().into_iter().map(Arc::from).collect::<Vec<_>>();
                &owned
            }
        };
        Ok(strs.par_iter().map(|s| s.ends_with(suffix)).collect())
    }

    #[sera_doc(
        name = "SeraDFrame.str_extract",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_extract, the first regex capture group (or full match).",
        fr = "Ajoute {col}_extract, le premier groupe capture regex (ou la correspondance complete)."
    )]
    fn str_extract(&self, col: &str, pattern: &str) -> PyResult<SeraDFrame_> {
        let re = regex::Regex::new(pattern).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let series = self.inner.get(col)?;
        let vals = series.to_str_vec();
        let out: Vec<String> = vals
            .par_iter()
            .map(|s| {
                re.captures(s)
                    .and_then(|c| c.get(1).or_else(|| c.get(0)))
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default()
            })
            .collect();
        self.push_derived(col, "extract", str_series(out))
    }

    #[sera_doc(
        name = "SeraDFrame.str_count",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_count, the number of non-overlapping regex matches.",
        fr = "Ajoute {col}_count, le nombre de correspondances regex non chevauchantes."
    )]
    fn str_count(&self, col: &str, pattern: &str) -> PyResult<SeraDFrame_> {
        let re = regex::Regex::new(pattern).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let series = self.inner.get(col)?;
        let vals = series.to_str_vec();
        let out: Vec<f64> = vals.par_iter().map(|s| re.find_iter(s).count() as f64).collect();
        self.push_derived(col, "count", Series::Num(Arc::new(out)))
    }

    #[sera_doc(
        name = "SeraDFrame.str_match",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Boolean mask of values matching a regex pattern.",
        fr = "Masque booleen des valeurs correspondant a un motif regex."
    )]
    fn str_match(&self, col: &str, pattern: &str) -> PyResult<Vec<bool>> {
        let re = regex::Regex::new(pattern).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        let series = self.inner.get(col)?;
        let vals = series.to_str_vec();
        Ok(vals.par_iter().map(|s| re.is_match(s)).collect())
    }

    #[sera_doc(
        name = "SeraDFrame.str_cat",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Concatenates several string columns row-wise into a new column, joined by sep.",
        fr = "Concatene plusieurs colonnes texte ligne par ligne dans une nouvelle colonne, jointes par sep.",
        aliases("concat_str", "str_join")
    )]
    fn str_cat(&self, cols: Vec<String>, sep: &str, out: &str) -> PyResult<SeraDFrame_> {
        let parts: Vec<Vec<String>> = cols
            .iter()
            .map(|c| self.inner.get(c).map(|s| s.to_str_vec()))
            .collect::<PyResult<Vec<_>>>()?;
        let joined: Vec<String> = (0..self.inner.nrows)
            .into_par_iter()
            .map(|i| parts.iter().map(|col| col[i].as_str()).collect::<Vec<_>>().join(sep))
            .collect();
        let mut order = self.inner.order.clone();
        let mut columns = self.inner.columns.clone();
        if !columns.contains_key(out) {
            order.push(out.to_string());
        }
        columns.insert(out.to_string(), str_series(joined));
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, self.inner.nrows)),
        })
    }

    #[pyo3(signature = (col, width, side = "left", fillchar = " "))]
    #[sera_doc(
        name = "SeraDFrame.str_pad",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Pads a string column to a fixed character width with fillchar; side is 'left', 'right', or 'both'. Values already at or over width are unchanged.",
        fr = "Complete une colonne texte a une largeur fixe de caracteres avec fillchar ; side vaut 'left', 'right' ou 'both'. Les valeurs deja a ou au-dessus de width sont inchangees.",
        aliases("pad_str")
    )]
    fn str_pad(&self, col: &str, width: usize, side: &str, fillchar: &str) -> PyResult<SeraDFrame_> {
        let fill = fillchar.chars().next().unwrap_or(' ');
        self.str_transform(col, |s| {
            let len = s.chars().count();
            if len >= width {
                return s.to_string();
            }
            let pad = width - len;
            match side {
                "right" => format!("{}{}", s, fill.to_string().repeat(pad)),
                "both" => {
                    let left = pad / 2;
                    let right = pad - left;
                    format!("{}{}{}", fill.to_string().repeat(left), s, fill.to_string().repeat(right))
                }
                _ => format!("{}{}", fill.to_string().repeat(pad), s),
            }
        })
    }

    #[pyo3(signature = (col, start, end = None))]
    #[sera_doc(
        name = "SeraDFrame.str_slice",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Extracts a character substring [start, end) from a string column; end defaults to the string's own length. Negative start/end count from the end, like Python slicing.",
        fr = "Extrait une sous-chaine de caracteres [start, end) d'une colonne texte ; end vaut par defaut la longueur propre de la chaine. Les start/end negatifs comptent depuis la fin, comme le slicing Python.",
        aliases("substr")
    )]
    fn str_slice(&self, col: &str, start: i64, end: Option<i64>) -> PyResult<SeraDFrame_> {
        self.str_transform(col, move |s| {
            let chars: Vec<char> = s.chars().collect();
            let len = chars.len() as i64;
            let resolve = |i: i64| -> usize { (if i < 0 { (i + len).max(0) } else { i }).min(len) as usize };
            let s0 = resolve(start);
            let e0 = end.map(resolve).unwrap_or(len as usize).max(s0);
            chars[s0..e0].iter().collect()
        })
    }
}
