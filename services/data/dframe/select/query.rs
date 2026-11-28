use super::super::series::ColView;
use super::super::SeraDFrame_;
use crate::sera_doc_impl;
use pyo3::prelude::*;
use rayon::prelude::*;
use std::sync::Arc;

#[derive(Clone, Copy)]
enum Op {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
}

enum Lit {
    Num(f64),
    Str(String),
    Bool(bool),
}

struct Clause {
    col: String,
    op: Op,
    lit: Lit,
}

fn parse_clause(re: &regex::Regex, s: &str) -> PyResult<Clause> {
    let s = s.trim();
    let caps = re
        .captures(s)
        .ok_or_else(|| pyo3::exceptions::PyValueError::new_err(format!("cannot parse query clause '{}'", s)))?;
    let col = caps[1].to_string();
    let op = match &caps[2] {
        "==" => Op::Eq,
        "!=" => Op::Ne,
        ">=" => Op::Ge,
        "<=" => Op::Le,
        ">" => Op::Gt,
        "<" => Op::Lt,
        _ => unreachable!(),
    };
    let lit_str = caps[3].trim();
    let lit = if (lit_str.starts_with('\'') && lit_str.ends_with('\'') && lit_str.len() >= 2)
        || (lit_str.starts_with('"') && lit_str.ends_with('"') && lit_str.len() >= 2)
    {
        Lit::Str(lit_str[1..lit_str.len() - 1].to_string())
    } else if lit_str == "True" {
        Lit::Bool(true)
    } else if lit_str == "False" {
        Lit::Bool(false)
    } else {
        Lit::Num(
            lit_str
                .parse()
                .map_err(|_| pyo3::exceptions::PyValueError::new_err(format!("invalid literal '{}'", lit_str)))?,
        )
    };
    Ok(Clause { col, op, lit })
}

fn eval_clause_at(view: &ColView, op: Op, lit: &Lit, i: usize) -> bool {
    match (view, lit) {
        (ColView::Num(v), Lit::Num(n)) => match op {
            Op::Eq => v[i] == *n,
            Op::Ne => v[i] != *n,
            Op::Gt => v[i] > *n,
            Op::Lt => v[i] < *n,
            Op::Ge => v[i] >= *n,
            Op::Le => v[i] <= *n,
        },
        (ColView::Str(v), Lit::Str(s)) => match op {
            Op::Eq => v[i].as_ref() == s.as_str(),
            Op::Ne => v[i].as_ref() != s.as_str(),
            _ => false,
        },
        (ColView::Bool(v), Lit::Bool(b)) => match op {
            Op::Eq => v[i] == *b,
            Op::Ne => v[i] != *b,
            _ => false,
        },
        _ => false,
    }
}

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[sera_doc(
        name = "SeraDFrame.query",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Filters rows via a bounded expression string (single 'and'/'or' combinator, comparisons only).",
        fr = "Filtre les lignes via une expression textuelle bornee (un seul combinateur 'and'/'or', comparaisons uniquement).",
        aliases("filter")
    )]
    pub(crate) fn query(&self, expr: &str) -> PyResult<SeraDFrame_> {
        let has_and = expr.contains(" and ");
        let has_or = expr.contains(" or ");
        if has_and && has_or {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "query() supports a single combinator per expression: all 'and' or all 'or', not mixed",
            ));
        }
        let sep = if has_or { " or " } else { " and " };
        let clause_re = regex::Regex::new(r"^([A-Za-z_][A-Za-z0-9_]*)\s*(==|!=|>=|<=|>|<)\s*(.+)$").unwrap();
        let clauses: Vec<Clause> = expr.split(sep).map(|s| parse_clause(&clause_re, s)).collect::<PyResult<Vec<_>>>()?;

        let mut views = Vec::with_capacity(clauses.len());
        for c in &clauses {
            let series = self.inner.get(&c.col)?;
            let view = series.as_view();
            let compatible = matches!(
                (&view, &c.lit),
                (ColView::Num(_), Lit::Num(_)) | (ColView::Str(_), Lit::Str(_)) | (ColView::Bool(_), Lit::Bool(_))
            );
            if !compatible {
                return Err(pyo3::exceptions::PyValueError::new_err(format!("type mismatch comparing column '{}'", c.col)));
            }
            if matches!(view, ColView::Str(_) | ColView::Bool(_)) && !matches!(c.op, Op::Eq | Op::Ne) {
                return Err(pyo3::exceptions::PyValueError::new_err(format!("only ==/!= supported for column '{}'", c.col)));
            }
            views.push(view);
        }

        let idx: Vec<usize> = if has_or {
            (0..self.inner.nrows)
                .into_par_iter()
                .filter(|&i| clauses.iter().zip(views.iter()).any(|(c, v)| eval_clause_at(v, c.op, &c.lit, i)))
                .collect()
        } else {
            (0..self.inner.nrows)
                .into_par_iter()
                .filter(|&i| clauses.iter().zip(views.iter()).all(|(c, v)| eval_clause_at(v, c.op, &c.lit, i)))
                .collect()
        };
        Ok(SeraDFrame_ {
            inner: Arc::new(self.inner.row_take(&idx)),
        })
    }
}
