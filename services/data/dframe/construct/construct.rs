use super::super::series::{column_from_pyobjects, Series};
use super::super::{SeraDFrame, SeraDFrame_};
use crate::data::table::Cell;
use crate::data::Table;
use crate::sera_doc_impl;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::sync::Arc;

fn trim_cr(l: &[u8]) -> &[u8] {
    if l.last() == Some(&b'\r') {
        &l[..l.len() - 1]
    } else {
        l
    }
}

impl SeraDFrame_ {
    fn from_csv_quoted(path: &str) -> PyResult<Self> {
        let file = File::open(path).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        let buffered = std::io::BufReader::with_capacity(1 << 20, file);
        let mut reader = csv::Reader::from_reader(buffered);
        let headers: Vec<String> = reader
            .headers()
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut raw: Vec<Vec<String>> = (0..headers.len()).map(|_| Vec::new()).collect();
        let mut nrows = 0usize;
        let mut record = csv::StringRecord::new();
        while reader
            .read_record(&mut record)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?
        {
            for (i, field) in record.iter().enumerate() {
                if i < raw.len() {
                    raw[i].push(field.to_string());
                }
            }
            nrows += 1;
        }
        Self::from_typed_columns(headers, raw, nrows)
    }

    fn from_csv_flat(bytes: &[u8]) -> PyResult<Self> {
        let mut lines: Vec<&[u8]> = bytes.split(|&b| b == b'\n').collect();
        while matches!(lines.last(), Some(l) if l.is_empty()) {
            lines.pop();
        }
        if lines.is_empty() {
            return Ok(SeraDFrame_ {
                inner: Arc::new(SeraDFrame::from_parts(Vec::new(), HashMap::new(), 0)),
            });
        }
        let header_line = trim_cr(lines[0]);
        let headers: Vec<String> = header_line
            .split(|&b| b == b',')
            .map(|f| String::from_utf8_lossy(f).into_owned())
            .collect();
        let data_lines = &lines[1..];
        let ncols = headers.len();
        let nrows = data_lines.len();
        let threads = rayon::current_num_threads().max(1);
        let chunk_size = ((nrows + threads - 1) / threads).max(1);
        let chunks: Vec<Vec<Vec<String>>> = data_lines
            .par_chunks(chunk_size)
            .map(|chunk| {
                let mut cols: Vec<Vec<String>> = (0..ncols).map(|_| Vec::with_capacity(chunk.len())).collect();
                for &line in chunk {
                    let line = trim_cr(line);
                    let mut i = 0usize;
                    for field in line.split(|&b| b == b',') {
                        if i < ncols {
                            cols[i].push(String::from_utf8_lossy(field).into_owned());
                        }
                        i += 1;
                    }
                    while i < ncols {
                        cols[i].push(String::new());
                        i += 1;
                    }
                }
                cols
            })
            .collect();
        let mut raw: Vec<Vec<String>> = (0..ncols).map(|_| Vec::with_capacity(nrows)).collect();
        for mut chunk_cols in chunks {
            for (ci, col) in chunk_cols.iter_mut().enumerate() {
                raw[ci].append(col);
            }
        }
        Self::from_typed_columns(headers, raw, nrows)
    }

    fn from_typed_columns(headers: Vec<String>, raw: Vec<Vec<String>>, nrows: usize) -> PyResult<Self> {
        let series: Vec<Series> = raw.into_par_iter().map(Series::from_strings).collect();
        let mut order = Vec::with_capacity(headers.len());
        let mut columns = HashMap::with_capacity(headers.len());
        for (name, s) in headers.into_iter().zip(series.into_iter()) {
            order.push(name.clone());
            columns.insert(name, s);
        }
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, nrows)),
        })
    }
}

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[new]
    fn new(columns: &Bound<'_, PyDict>) -> PyResult<Self> {
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::dict_to_frame(columns)?),
        })
    }

    #[staticmethod]
    #[sera_doc(
        name = "SeraDFrame.from_csv",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Reads a CSV into a SeraDFrame with parallel per-column type inference.",
        fr = "Lit un CSV en SeraDFrame avec inference de type parallele par colonne."
    )]
    fn from_csv(path: &str) -> PyResult<Self> {
        let bytes = std::fs::read(path).map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        if bytes.contains(&b'"') {
            return Self::from_csv_quoted(path);
        }
        Self::from_csv_flat(&bytes)
    }

    #[staticmethod]
    #[sera_doc(
        name = "SeraDFrame.from_pandas",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Builds a SeraDFrame from an existing pandas.DataFrame.",
        fr = "Construit un SeraDFrame depuis un pandas.DataFrame existant."
    )]
    fn from_pandas(df: &Bound<'_, PyAny>) -> PyResult<Self> {
        let as_dict = df.call_method1("to_dict", ("list",))?;
        let dict = as_dict.downcast::<PyDict>()?;
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::dict_to_frame(dict)?),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.to_pandas",
        category = "data_method", file = "canvas/dframe.md", en = "Converts to a pandas.DataFrame, losslessly.", fr = "Convertit vers un pandas.DataFrame, sans perte.")]
    fn to_pandas(&self, py: Python<'_>) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        for name in &self.inner.order {
            dict.set_item(name, self.inner.columns[name].clone().into_py(py))?;
        }
        let pandas = py.import_bound("pandas")?;
        Ok(pandas.call_method1("DataFrame", (dict,))?.into())
    }

    #[sera_doc(
        name = "SeraDFrame.to_table",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Bridges into SeraPlot's Table so chart-export helpers apply.",
        fr = "Passerelle vers le Table de SeraPlot pour les aides d'export chart."
    )]
    fn to_table(&self) -> Table {
        let columns: HashMap<String, Vec<Cell>> = self
            .inner
            .order
            .iter()
            .map(|n| {
                let cells = match &self.inner.columns[n] {
                    Series::Num(v) => v.iter().map(|x| Cell::Num(*x)).collect(),
                    Series::Str(v) => v.iter().map(|x| Cell::Str(x.to_string())).collect(),
                    Series::Bool(v) => v.iter().map(|x| Cell::Bool(*x)).collect(),
                };
                (n.clone(), cells)
            })
            .collect();
        Table::from_parts(self.inner.order.clone(), columns, self.inner.nrows)
    }

    #[getter]
    #[sera_doc(
        name = "SeraDFrame.shape",
        category = "data_method", file = "canvas/dframe.md", en = "Returns (rows, columns).", fr = "Renvoie (lignes, colonnes).")]
    fn shape(&self) -> (usize, usize) {
        (self.inner.nrows, self.inner.order.len())
    }

    #[sera_doc(
        name = "SeraDFrame.columns",
        category = "data_method", file = "canvas/dframe.md", en = "Lists column names in order.", fr = "Liste les noms de colonnes dans l'ordre.")]
    fn columns(&self) -> Vec<String> {
        self.inner.order.clone()
    }

    #[sera_doc(
        name = "SeraDFrame.dtypes",
        category = "data_method", file = "canvas/dframe.md", en = "Maps column name to inferred dtype.", fr = "Associe chaque colonne a son dtype inferre.")]
    fn dtypes(&self) -> HashMap<String, String> {
        self.inner
            .order
            .iter()
            .map(|n| (n.clone(), self.inner.columns[n].dtype().to_string()))
            .collect()
    }

    #[sera_doc(
        name = "SeraDFrame.memory_usage",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Estimated bytes held per column.",
        fr = "Estimation des octets occupes par colonne.",
        aliases("estimated_size")
    )]
    pub(crate) fn memory_usage(&self) -> HashMap<String, u64> {
        self.inner.order.iter().map(|n| (n.clone(), self.inner.columns[n].byte_size())).collect()
    }

    #[sera_doc(
        name = "SeraDFrame.glimpse",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Prints row/column counts and a per-column dtype + value preview.",
        fr = "Affiche le nombre de lignes/colonnes et un apercu dtype + valeurs par colonne."
    )]
    fn glimpse(&self, py: Python<'_>) -> PyResult<()> {
        let mut s = format!("Rows: {}\nColumns: {}\n", self.inner.nrows, self.inner.order.len());
        for name in &self.inner.order {
            let col = &self.inner.columns[name];
            let preview: Vec<String> = match col {
                Series::Num(v) => v.iter().take(5).map(|x| format!("{}", x)).collect(),
                Series::Str(v) => v.iter().take(5).map(|x| x.to_string()).collect(),
                Series::Bool(v) => v.iter().take(5).map(|x| x.to_string()).collect(),
            };
            s.push_str(&format!("$ {} <{}> {}\n", name, col.dtype(), preview.join(", ")));
        }
        let builtins = py.import_bound("builtins")?;
        builtins.call_method1("print", (s,))?;
        Ok(())
    }

    #[sera_doc(
        name = "SeraDFrame.select_dtypes",
        category = "data_method", file = "canvas/dframe.md", en = "Keeps only columns matching a dtype.", fr = "Ne garde que les colonnes correspondant a un dtype.")]
    fn select_dtypes(&self, dtype: &str) -> SeraDFrame_ {
        let names: Vec<String> = self
            .inner
            .order
            .iter()
            .filter(|n| self.inner.columns[*n].dtype() == dtype)
            .cloned()
            .collect();
        let columns: HashMap<String, Series> = names.iter().map(|n| (n.clone(), self.inner.columns[n].clone())).collect();
        SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(names, columns, self.inner.nrows)),
        }
    }

    #[sera_doc(
        name = "SeraDFrame.head",
        category = "data_method", file = "canvas/dframe.md", en = "First n rows.", fr = "Les n premieres lignes.")]
    fn head(&self, n: usize) -> SeraDFrame_ {
        let n = n.min(self.inner.nrows);
        let idx: Vec<usize> = (0..n).collect();
        SeraDFrame_ {
            inner: Arc::new(self.inner.row_take(&idx)),
        }
    }

    #[sera_doc(
        name = "SeraDFrame.tail",
        category = "data_method", file = "canvas/dframe.md", en = "Last n rows.", fr = "Les n dernieres lignes.")]
    fn tail(&self, n: usize) -> SeraDFrame_ {
        let n = n.min(self.inner.nrows);
        let idx: Vec<usize> = (self.inner.nrows - n..self.inner.nrows).collect();
        SeraDFrame_ {
            inner: Arc::new(self.inner.row_take(&idx)),
        }
    }

    fn column(&self, py: Python<'_>, name: &str) -> PyResult<PyObject> {
        Ok(self.inner.get(name)?.clone().into_py(py))
    }

    fn column_f64(&self, name: &str) -> PyResult<Vec<f64>> {
        Ok(self.inner.get(name)?.to_f64_vec())
    }

    fn column_str(&self, name: &str) -> PyResult<Vec<String>> {
        Ok(self.inner.get(name)?.to_str_vec())
    }

    #[sera_doc(
        name = "SeraDFrame.select",
        category = "data_method", file = "canvas/dframe.md", en = "Keeps only the given columns.", fr = "Ne garde que les colonnes donnees.")]
    fn select(&self, names: Vec<String>) -> PyResult<SeraDFrame_> {
        let mut columns = HashMap::new();
        for n in &names {
            columns.insert(n.clone(), self.inner.get(n)?.clone());
        }
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(names, columns, self.inner.nrows)),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.rename",
        category = "data_method", file = "canvas/dframe.md", en = "Renames columns via a mapping.", fr = "Renomme des colonnes via un mapping.")]
    fn rename(&self, mapping: HashMap<String, String>) -> SeraDFrame_ {
        let order: Vec<String> = self
            .inner
            .order
            .iter()
            .map(|n| mapping.get(n).cloned().unwrap_or_else(|| n.clone()))
            .collect();
        let mut columns = HashMap::new();
        for (old, new) in self.inner.order.iter().zip(order.iter()) {
            columns.insert(new.clone(), self.inner.columns[old].clone());
        }
        SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, self.inner.nrows)),
        }
    }

    #[pyo3(signature = (label_col = None))]
    #[sera_doc(
        name = "SeraDFrame.transpose",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Swaps rows and columns: former column names become the first column (or values from `label_col` if given), and each row becomes a new column named row0, row1, ... Cells are coerced to string since a transposed row can mix the original per-column types.",
        fr = "Echange lignes et colonnes : les anciens noms de colonnes deviennent la premiere colonne (ou les valeurs de `label_col` si fourni), et chaque ligne devient une nouvelle colonne nommee row0, row1, ... Les cellules sont converties en chaine car une ligne transposee peut melanger les types des colonnes d'origine.",
        aliases("T")
    )]
    pub(crate) fn transpose(&self, label_col: Option<&str>) -> PyResult<SeraDFrame_> {
        let source_order = self.inner.order.clone();
        let row_labels: Vec<String> = match label_col {
            Some(name) => self.inner.get(name)?.to_str_vec(),
            None => (0..self.inner.nrows).map(|i| format!("row{i}")).collect(),
        };
        let field_names: Vec<String> = match label_col {
            Some(name) => source_order.iter().filter(|c| c.as_str() != name).cloned().collect(),
            None => source_order.clone(),
        };
        let mut order = vec!["field".to_string()];
        let mut columns = HashMap::new();
        columns.insert("field".to_string(), super::super::str_series(field_names.clone()));
        for r in 0..self.inner.nrows {
            let col_name = row_labels.get(r).cloned().unwrap_or_else(|| format!("row{r}"));
            let cell_vals: Vec<String> = field_names
                .iter()
                .map(|fname| self.inner.columns[fname].value_str(r))
                .collect();
            order.push(col_name.clone());
            columns.insert(col_name, super::super::str_series(cell_vals));
        }
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, field_names.len())),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.assign",
        category = "data_method", file = "canvas/dframe.md", en = "Adds or replaces a column.", fr = "Ajoute ou remplace une colonne.")]
    fn assign(&self, name: &str, values: &Bound<'_, PyAny>) -> PyResult<SeraDFrame_> {
        let items: Vec<Bound<'_, PyAny>> = values.extract()?;
        let series = column_from_pyobjects(items);
        let mut order = self.inner.order.clone();
        let mut columns = self.inner.columns.clone();
        if !columns.contains_key(name) {
            order.push(name.to_string());
        }
        let nrows = series.len().max(self.inner.nrows);
        columns.insert(name.to_string(), series);
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, nrows)),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.insert",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Inserts a column at a specific position, unlike assign() which always appends at the end.",
        fr = "Insere une colonne a une position precise, contrairement a assign() qui ajoute toujours a la fin."
    )]
    fn insert(&self, loc: usize, name: &str, values: &Bound<'_, PyAny>) -> PyResult<SeraDFrame_> {
        let items: Vec<Bound<'_, PyAny>> = values.extract()?;
        let series = column_from_pyobjects(items);
        let mut order: Vec<String> = self.inner.order.iter().filter(|c| c.as_str() != name).cloned().collect();
        let loc = loc.min(order.len());
        order.insert(loc, name.to_string());
        let mut columns = self.inner.columns.clone();
        let nrows = series.len().max(self.inner.nrows);
        columns.insert(name.to_string(), series);
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(order, columns, nrows)),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.copy",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Returns an independent copy of the frame.",
        fr = "Retourne une copie independante du frame."
    )]
    fn copy(&self) -> SeraDFrame_ {
        SeraDFrame_ {
            inner: Arc::new((*self.inner).clone()),
        }
    }

    #[sera_doc(
        name = "SeraDFrame.info",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Human-readable summary: row/column count, each column's dtype, and estimated memory usage.",
        fr = "Resume lisible : nombre de lignes/colonnes, type de chaque colonne, et memoire estimee."
    )]
    fn info(&self) -> String {
        let mut out = format!("SeraDFrame: {} rows x {} columns\n", self.inner.nrows, self.inner.order.len());
        for name in &self.inner.order {
            let col = &self.inner.columns[name];
            out.push_str(&format!("  {:<24} {}\n", name, col.dtype()));
        }
        let total_bytes: u64 = self.inner.order.iter().map(|n| self.inner.columns[n].byte_size()).sum();
        out.push_str(&format!("memory usage: {:.1} KB\n", total_bytes as f64 / 1024.0));
        out
    }

    fn __len__(&self) -> usize {
        self.inner.nrows
    }

    fn __repr__(&self) -> String {
        format!("SeraDFrame({} rows x {} cols)", self.inner.nrows, self.inner.order.len())
    }

    fn __getitem__(&self, py: Python<'_>, key: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        if let Ok(name) = key.extract::<String>() {
            return self.column(py, &name);
        }
        if let Ok(names) = key.extract::<Vec<String>>() {
            return Ok(self.select(names)?.into_py(py));
        }
        Err(pyo3::exceptions::PyTypeError::new_err("expected str or list[str]"))
    }
}
