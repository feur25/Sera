use std::fmt::Write;

pub struct TdsColumn {
    pub name: String,
    pub datatype: String,
    pub role: String,
}

pub struct TdsDescriptor {
    pub name: String,
    pub columns: Vec<TdsColumn>,
    pub rows: Vec<Vec<f64>>,
    pub row_strings: Vec<Vec<String>>,
    pub use_strings: bool,
}

pub fn columns_from_features(p: usize, target: bool, prediction: bool) -> Vec<TdsColumn> {
    let mut c: Vec<TdsColumn> = (0..p)
        .map(|i| TdsColumn {
            name: format!("feature_{}", i),
            datatype: "real".into(),
            role: "dimension".into(),
        })
        .collect();
    if target {
        c.push(TdsColumn {
            name: "target".into(),
            datatype: "real".into(),
            role: "measure".into(),
        });
    }
    if prediction {
        c.push(TdsColumn {
            name: "prediction".into(),
            datatype: "real".into(),
            role: "measure".into(),
        });
    }
    c
}

pub fn to_tds_xml(d: &TdsDescriptor) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "<?xml version='1.0' encoding='utf-8'?>");
    let _ = writeln!(
        s,
        "<datasource version='18.1' inline='true' name='{}'>",
        xml_escape(&d.name)
    );
    let _ = writeln!(s, "  <connection class='excel-direct'>");
    let _ = writeln!(
        s,
        "    <relation name='{}' type='table'/>",
        xml_escape(&d.name)
    );
    let _ = writeln!(s, "  </connection>");
    let _ = writeln!(s, "  <columns>");
    for c in &d.columns {
        let _ = writeln!(
            s,
            "    <column name='[{}]' datatype='{}' role='{}'/>",
            xml_escape(&c.name),
            c.datatype,
            c.role
        );
    }
    let _ = writeln!(s, "  </columns>");
    let _ = writeln!(s, "</datasource>");
    s
}

pub fn to_csv(d: &TdsDescriptor) -> String {
    let mut s = String::new();
    let header: Vec<String> = d.columns.iter().map(|c| c.name.clone()).collect();
    let _ = writeln!(s, "{}", header.join(","));
    if d.use_strings {
        for row in &d.row_strings {
            let cells: Vec<String> = row.iter().map(|v| csv_escape(v)).collect();
            let _ = writeln!(s, "{}", cells.join(","));
        }
    } else {
        for row in &d.rows {
            let cells: Vec<String> = row.iter().map(|v| format!("{}", v)).collect();
            let _ = writeln!(s, "{}", cells.join(","));
        }
    }
    s
}

pub fn rows_from_matrix(
    x: &[f64],
    n: usize,
    p: usize,
    y: Option<&[f64]>,
    yhat: Option<&[f64]>,
) -> Vec<Vec<f64>> {
    (0..n)
        .map(|i| {
            let mut row: Vec<f64> = (0..p).map(|j| x[i * p + j]).collect();
            if let Some(yv) = y {
                row.push(yv[i]);
            }
            if let Some(yp) = yhat {
                row.push(yp[i]);
            }
            row
        })
        .collect()
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\'', "&apos;")
        .replace('"', "&quot;")
}

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}
