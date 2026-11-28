use super::super::series::Series;
use super::super::{SeraDFrame, SeraDFrame_};
use crate::sera_doc_impl;
use pyo3::prelude::*;
use rayon::prelude::*;
use std::sync::Arc;

fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m as i64 + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn ymdhms_to_epoch(y: i64, m: u32, d: u32, h: u32, mi: u32, s: f64) -> f64 {
    let days = days_from_civil(y, m, d);
    (days * 86400 + h as i64 * 3600 + mi as i64 * 60) as f64 + s
}

struct DateParts {
    year: i64,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    weekday: u32,
    dayofyear: u32,
}

fn epoch_to_parts(epoch: f64) -> DateParts {
    let secs = epoch.floor() as i64;
    let days = secs.div_euclid(86400);
    let sod = secs.rem_euclid(86400);
    let (y, m, d) = civil_from_days(days);
    let weekday = ((days + 3).rem_euclid(7)) as u32;
    let dayofyear = (days_from_civil(y, m, d) - days_from_civil(y, 1, 1) + 1) as u32;
    DateParts {
        year: y,
        month: m,
        day: d,
        hour: (sod / 3600) as u32,
        minute: ((sod % 3600) / 60) as u32,
        second: (sod % 60) as u32,
        weekday,
        dayofyear,
    }
}

fn parse_datetime(s: &str) -> Option<f64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    let (date_part, time_part) = match s.find(['T', ' ']) {
        Some(i) => (&s[..i], Some(&s[i + 1..])),
        None => (s, None),
    };
    let mut dparts = date_part.splitn(3, '-');
    let y: i64 = dparts.next()?.parse().ok()?;
    let m: u32 = dparts.next()?.parse().ok()?;
    let d: u32 = dparts.next()?.parse().ok()?;
    let (h, mi, s) = match time_part {
        Some(t) => {
            let mut tparts = t.splitn(3, ':');
            let h: u32 = tparts.next()?.parse().ok()?;
            let mi: u32 = tparts.next().unwrap_or("0").parse().ok()?;
            let sec: f64 = tparts.next().unwrap_or("0").parse().ok()?;
            (h, mi, sec)
        }
        None => (0, 0, 0.0),
    };
    Some(ymdhms_to_epoch(y, m, d, h, mi, s))
}

fn unit_seconds(unit: &str) -> PyResult<f64> {
    Ok(match unit {
        "second" | "s" => 1.0,
        "minute" | "min" | "m" => 60.0,
        "hour" | "h" => 3600.0,
        "day" | "d" => 86400.0,
        _ => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "unsupported unit '{}', expected second/minute/hour/day",
                unit
            )))
        }
    })
}

impl SeraDFrame_ {
    fn dt_field(&self, col: &str, suffix: &str, extract: impl Fn(f64) -> f64 + Sync) -> PyResult<SeraDFrame_> {
        let vals = self.inner.get(col)?.to_f64_vec();
        let out: Vec<f64> = vals.par_iter().map(|&v| if v.is_nan() { f64::NAN } else { extract(v) }).collect();
        self.push_derived(col, suffix, Series::Num(Arc::new(out)))
    }
}

#[sera_doc_impl]
#[pymethods]
impl SeraDFrame_ {
    #[sera_doc(
        name = "SeraDFrame.to_datetime",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Parses an ISO-ish date/time string column into epoch-second floats.",
        fr = "Parse une colonne texte de date/heure ISO en flottants secondes-epoch."
    )]
    fn to_datetime(&self, col: &str) -> PyResult<SeraDFrame_> {
        let series = self.inner.get(col)?;
        let strs = series.to_str_vec();
        let out: Vec<f64> = strs.par_iter().map(|s| parse_datetime(s).unwrap_or(f64::NAN)).collect();
        let mut columns = self.inner.columns.clone();
        columns.insert(col.to_string(), Series::Num(Arc::new(out)));
        Ok(SeraDFrame_ {
            inner: Arc::new(SeraDFrame::from_parts(self.inner.order.clone(), columns, self.inner.nrows)),
        })
    }

    #[sera_doc(
        name = "SeraDFrame.dt_year",
        category = "data_method", file = "canvas/dframe.md", en = "Adds {col}_year from an epoch-second column.", fr = "Ajoute {col}_year depuis une colonne secondes-epoch.")]
    fn dt_year(&self, col: &str) -> PyResult<SeraDFrame_> {
        self.dt_field(col, "year", |v| epoch_to_parts(v).year as f64)
    }

    #[sera_doc(
        name = "SeraDFrame.dt_month",
        category = "data_method", file = "canvas/dframe.md", en = "Adds {col}_month from an epoch-second column.", fr = "Ajoute {col}_month depuis une colonne secondes-epoch.")]
    fn dt_month(&self, col: &str) -> PyResult<SeraDFrame_> {
        self.dt_field(col, "month", |v| epoch_to_parts(v).month as f64)
    }

    #[sera_doc(
        name = "SeraDFrame.dt_day",
        category = "data_method", file = "canvas/dframe.md", en = "Adds {col}_day from an epoch-second column.", fr = "Ajoute {col}_day depuis une colonne secondes-epoch.")]
    fn dt_day(&self, col: &str) -> PyResult<SeraDFrame_> {
        self.dt_field(col, "day", |v| epoch_to_parts(v).day as f64)
    }

    #[sera_doc(
        name = "SeraDFrame.dt_hour",
        category = "data_method", file = "canvas/dframe.md", en = "Adds {col}_hour from an epoch-second column.", fr = "Ajoute {col}_hour depuis une colonne secondes-epoch.")]
    fn dt_hour(&self, col: &str) -> PyResult<SeraDFrame_> {
        self.dt_field(col, "hour", |v| epoch_to_parts(v).hour as f64)
    }

    #[sera_doc(
        name = "SeraDFrame.dt_minute",
        category = "data_method", file = "canvas/dframe.md", en = "Adds {col}_minute from an epoch-second column.", fr = "Ajoute {col}_minute depuis une colonne secondes-epoch.")]
    fn dt_minute(&self, col: &str) -> PyResult<SeraDFrame_> {
        self.dt_field(col, "minute", |v| epoch_to_parts(v).minute as f64)
    }

    #[sera_doc(
        name = "SeraDFrame.dt_second",
        category = "data_method", file = "canvas/dframe.md", en = "Adds {col}_second from an epoch-second column.", fr = "Ajoute {col}_second depuis une colonne secondes-epoch.")]
    fn dt_second(&self, col: &str) -> PyResult<SeraDFrame_> {
        self.dt_field(col, "second", |v| epoch_to_parts(v).second as f64)
    }

    #[sera_doc(
        name = "SeraDFrame.dt_weekday",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_weekday, 0=Monday..6=Sunday, from an epoch-second column.",
        fr = "Ajoute {col}_weekday, 0=lundi..6=dimanche, depuis une colonne secondes-epoch."
    )]
    fn dt_weekday(&self, col: &str) -> PyResult<SeraDFrame_> {
        self.dt_field(col, "weekday", |v| epoch_to_parts(v).weekday as f64)
    }

    #[sera_doc(
        name = "SeraDFrame.dt_dayofyear",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_dayofyear (1-366) from an epoch-second column.",
        fr = "Ajoute {col}_dayofyear (1-366) depuis une colonne secondes-epoch."
    )]
    fn dt_dayofyear(&self, col: &str) -> PyResult<SeraDFrame_> {
        self.dt_field(col, "dayofyear", |v| epoch_to_parts(v).dayofyear as f64)
    }

    #[sera_doc(
        name = "SeraDFrame.dt_quarter",
        category = "data_method", file = "canvas/dframe.md", en = "Adds {col}_quarter (1-4) from an epoch-second column.", fr = "Ajoute {col}_quarter (1-4) depuis une colonne secondes-epoch.")]
    fn dt_quarter(&self, col: &str) -> PyResult<SeraDFrame_> {
        self.dt_field(col, "quarter", |v| (((epoch_to_parts(v).month - 1) / 3) + 1) as f64)
    }

    #[sera_doc(
        name = "SeraDFrame.dt_floor",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_floor_UNIT, epoch rounded down to a second/minute/hour/day boundary.",
        fr = "Ajoute {col}_floor_UNIT, secondes-epoch arrondies vers le bas a la frontiere seconde/minute/heure/jour."
    )]
    fn dt_floor(&self, col: &str, unit: &str) -> PyResult<SeraDFrame_> {
        let step = unit_seconds(unit)?;
        self.dt_field(col, &format!("floor_{}", unit), move |v| (v / step).floor() * step)
    }

    #[sera_doc(
        name = "SeraDFrame.dt_ceil",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_ceil_UNIT, epoch rounded up to a second/minute/hour/day boundary.",
        fr = "Ajoute {col}_ceil_UNIT, secondes-epoch arrondies vers le haut a la frontiere seconde/minute/heure/jour."
    )]
    fn dt_ceil(&self, col: &str, unit: &str) -> PyResult<SeraDFrame_> {
        let step = unit_seconds(unit)?;
        self.dt_field(col, &format!("ceil_{}", unit), move |v| (v / step).ceil() * step)
    }

    #[sera_doc(
        name = "SeraDFrame.dt_round",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_round_UNIT, epoch rounded to the nearest second/minute/hour/day boundary.",
        fr = "Ajoute {col}_round_UNIT, secondes-epoch arrondies a la frontiere la plus proche seconde/minute/heure/jour."
    )]
    fn dt_round(&self, col: &str, unit: &str) -> PyResult<SeraDFrame_> {
        let step = unit_seconds(unit)?;
        self.dt_field(col, &format!("round_{}", unit), move |v| (v / step).round() * step)
    }

    #[sera_doc(
        name = "SeraDFrame.dt_add_seconds",
        category = "data_method",
        file = "canvas/dframe.md",
        en = "Adds {col}_shifted, the epoch column offset by a duration in seconds.",
        fr = "Ajoute {col}_shifted, la colonne epoch decalee d'une duree en secondes."
    )]
    fn dt_add_seconds(&self, col: &str, seconds: f64) -> PyResult<SeraDFrame_> {
        self.dt_field(col, "shifted", move |v| v + seconds)
    }
}
