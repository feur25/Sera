use serde::Serialize;

#[derive(Serialize)]
pub struct PbiColumn {
    pub name: String,
    #[serde(rename = "dataType")]
    pub data_type: String,
}

#[derive(Serialize)]
pub struct PbiTable {
    pub name: String,
    pub columns: Vec<PbiColumn>,
    pub rows: Vec<Vec<serde_json::Value>>,
}

#[derive(Serialize)]
pub struct PbiDataset {
    pub name: String,
    #[serde(rename = "defaultMode")]
    pub default_mode: String,
    pub tables: Vec<PbiTable>,
}

pub fn build_dataset(
    name: &str,
    table_name: &str,
    columns: &[(String, String)],
    rows: Vec<Vec<serde_json::Value>>,
) -> PbiDataset {
    let cols = columns
        .iter()
        .map(|(n, t)| PbiColumn {
            name: n.clone(),
            data_type: t.clone(),
        })
        .collect();
    PbiDataset {
        name: name.to_string(),
        default_mode: "Push".to_string(),
        tables: vec![PbiTable {
            name: table_name.to_string(),
            columns: cols,
            rows,
        }],
    }
}

pub fn to_json(ds: &PbiDataset) -> String {
    serde_json::to_string_pretty(ds).unwrap_or_else(|_| "{}".to_string())
}

pub fn columns_from_features(p: usize, target: bool, prediction: bool) -> Vec<(String, String)> {
    let mut c: Vec<(String, String)> = (0..p)
        .map(|i| (format!("feature_{}", i), "Double".to_string()))
        .collect();
    if target {
        c.push(("target".to_string(), "Double".to_string()));
    }
    if prediction {
        c.push(("prediction".to_string(), "Double".to_string()));
    }
    c
}

pub fn rows_from_matrix(
    x: &[f64],
    n: usize,
    p: usize,
    y: Option<&[f64]>,
    yhat: Option<&[f64]>,
) -> Vec<Vec<serde_json::Value>> {
    (0..n)
        .map(|i| {
            let mut row: Vec<serde_json::Value> =
                (0..p).map(|j| serde_json::json!(x[i * p + j])).collect();
            if let Some(yv) = y {
                row.push(serde_json::json!(yv[i]));
            }
            if let Some(yp) = yhat {
                row.push(serde_json::json!(yp[i]));
            }
            row
        })
        .collect()
}
