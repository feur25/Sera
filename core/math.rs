use std::f64;
use serde::{Serialize, Deserialize};

pub fn mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    Some(values.iter().sum::<f64>() / values.len() as f64)
}

pub fn median(mut values: Vec<f64>) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let len = values.len();
    if len % 2 == 0 {
        Some((values[len / 2 - 1] + values[len / 2]) / 2.0)
    } else {
        Some(values[len / 2])
    }
}

pub fn std_dev(values: &[f64]) -> Option<f64> {
    let mean_val = mean(values)?;
    let variance = values.iter()
        .map(|x| (x - mean_val).powi(2))
        .sum::<f64>() / values.len() as f64;
    Some(variance.sqrt())
}

pub fn variance(values: &[f64]) -> Option<f64> {
    let mean_val = mean(values)?;
    Some(values.iter()
        .map(|x| (x - mean_val).powi(2))
        .sum::<f64>() / values.len() as f64)
}

pub fn percentile(mut values: Vec<f64>, p: f64) -> Option<f64> {
    if values.is_empty() || p < 0.0 || p > 100.0 {
        return None;
    }
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let idx = (p / 100.0 * (values.len() - 1) as f64).round() as usize;
    values.get(idx).copied()
}

pub fn quartiles(values: Vec<f64>) -> Option<(f64, f64, f64)> {
    let q1 = percentile(values.clone(), 25.0)?;
    let q2 = percentile(values.clone(), 50.0)?;
    let q3 = percentile(values, 75.0)?;
    Some((q1, q2, q3))
}

pub fn outliers(values: &[f64]) -> Vec<f64> {
    if let Some((q1, _, q3)) = quartiles(values.to_vec()) {
        let iqr = q3 - q1;
        let lower = q1 - 1.5 * iqr;
        let upper = q3 + 1.5 * iqr;
        values.iter()
            .filter(|&&v| v < lower || v > upper)
            .copied()
            .collect()
    } else {
        Vec::new()
    }
}

pub fn remove_outliers(mut values: Vec<f64>) -> Vec<f64> {
    if let Some((q1, _, q3)) = quartiles(values.clone()) {
        let iqr = q3 - q1;
        let lower = q1 - 1.5 * iqr;
        let upper = q3 + 1.5 * iqr;
        values.retain(|&v| v >= lower && v <= upper);
        values
    } else {
        values
    }
}

pub fn normalize(values: &[f64]) -> Vec<f64> {
    let range = Range::from_slice(values);
    if let Some(r) = range {
        let w = r.width();
        if w > 0.0 {
            values.iter().map(|x| (x - r.min) / w).collect()
        } else {
            values.to_vec()
        }
    } else {
        values.to_vec()
    }
}

pub fn standardize(values: &[f64]) -> Vec<f64> {
    if let (Some(m), Some(s)) = (mean(values), std_dev(values)) {
        if s > 0.0 {
            values.iter().map(|x| (x - m) / s).collect()
        } else {
            values.to_vec()
        }
    } else {
        values.to_vec()
    }
}

pub fn linear_regression(x: &[f64], y: &[f64]) -> Option<(f64, f64)> {
    let n = x.len().min(y.len()) as f64;
    if n < 2.0 {
        return None;
    }

    let sum_x = x.iter().take(n as usize).sum::<f64>();
    let sum_y = y.iter().take(n as usize).sum::<f64>();
    let sum_xy = x.iter().zip(y.iter()).map(|(xi, yi)| xi * yi).sum::<f64>();
    let sum_x2 = x.iter().map(|xi| xi * xi).sum::<f64>();

    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
    let intercept = (sum_y - slope * sum_x) / n;

    Some((slope, intercept))
}

pub fn correlation(x: &[f64], y: &[f64]) -> Option<f64> {
    let n = x.len().min(y.len());
    if n < 2 {
        return None;
    }

    let x_slice = &x[..n];
    let y_slice = &y[..n];

    let mean_x = mean(x_slice)?;
    let mean_y = mean(y_slice)?;

    let numerator: f64 = x_slice.iter().zip(y_slice.iter())
        .map(|(xi, yi)| (xi - mean_x) * (yi - mean_y))
        .sum();

    let std_x = std_dev(x_slice)?;
    let std_y = std_dev(y_slice)?;

    if std_x > 0.0 && std_y > 0.0 {
        Some(numerator / (n as f64 * std_x * std_y))
    } else {
        None
    }
}

pub fn moving_average(values: &[f64], window: usize) -> Vec<f64> {
    if window == 0 || values.is_empty() {
        return values.to_vec();
    }
    values.windows(window.min(values.len()))
        .map(|w| w.iter().sum::<f64>() / w.len() as f64)
        .collect()
}

pub fn exponential_moving_average(values: &[f64], alpha: f64) -> Vec<f64> {
    if values.is_empty() {
        return Vec::new();
    }

    let mut ema = Vec::with_capacity(values.len());
    ema.push(values[0]);

    for &val in &values[1..] {
        let last_ema = ema[ema.len() - 1];
        ema.push(alpha * val + (1.0 - alpha) * last_ema);
    }

    ema
}

pub fn interpolate_linear(x: &[f64], y: &[f64], new_x: &[f64]) -> Vec<f64> {
    new_x.iter().map(|xi| {
        let mut low = 0;
        let mut high = x.len() - 1;

        while low < high {
            let mid = (low + high) / 2;
            if x[mid] < *xi {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        if low == 0 {
            y[0]
        } else if low >= x.len() {
            y[x.len() - 1]
        } else {
            let x0 = x[low - 1];
            let x1 = x[low];
            let y0 = y[low - 1];
            let y1 = y[low];
            y0 + (y1 - y0) * (xi - x0) / (x1 - x0)
        }
    }).collect()
}

pub fn histogram(values: &[f64], bins: usize) -> Vec<u32> {
    if values.is_empty() || bins == 0 {
        return Vec::new();
    }

    let range = Range::from_slice(values).unwrap();
    let bin_width = range.width() / bins as f64;
    let mut hist = vec![0u32; bins];

    for &val in values {
        let bin_idx = ((val - range.min) / bin_width).floor() as usize;
        if bin_idx < bins {
            hist[bin_idx] += 1;
        } else if bin_idx == bins && (val - range.min).abs() < 1e-10 {
            hist[bins - 1] += 1;
        }
    }

    hist
}

pub fn aggregate(values: &[f64], op: &AggOp) -> Option<f64> {
    match op {
        AggOp::Sum => Some(values.iter().sum()),
        AggOp::Avg => mean(values),
        AggOp::Min => values.iter().cloned().min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)),
        AggOp::Max => values.iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)),
        AggOp::Count => Some(values.len() as f64),
        AggOp::Median => median(values.to_vec()),
        AggOp::Std => std_dev(values),
        AggOp::Var => variance(values),
        AggOp::Percentile(p) => percentile(values.to_vec(), *p),
    }
}

pub fn group_by_bins(x: &[f64], y: &[f64], bins: usize, op: AggOp) -> (Vec<f64>, Vec<f64>) {
    if x.is_empty() || y.is_empty() {
        return (Vec::new(), Vec::new());
    }

    let range = Range::from_slice(x).unwrap();
    let bin_width = range.width() / bins as f64;
    let mut bin_values: Vec<Vec<f64>> = vec![Vec::new(); bins];
    let mut bin_centers: Vec<f64> = Vec::new();

    for (xi, yi) in x.iter().zip(y.iter()) {
        let bin_idx = ((xi - range.min) / bin_width).floor() as usize;
        if bin_idx < bins {
            bin_values[bin_idx].push(*yi);
        }
    }

    let agg_vals: Vec<f64> = bin_values.iter()
        .enumerate()
        .filter_map(|(i, vals)| {
            if vals.is_empty() {
                None
            } else {
                bin_centers.push(range.min + (i as f64 + 0.5) * bin_width);
                aggregate(vals, &op)
            }
        })
        .collect();

    (bin_centers, agg_vals)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AggOp {
    Sum,
    Avg,
    Min,
    Max,
    Count,
    Median,
    Std,
    Var,
    Percentile(f64),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

impl Range {
    pub fn new(min: f64, max: f64) -> Self { Self { min, max } }
    pub fn from_slice(values: &[f64]) -> Option<Self> {
        if values.is_empty() { return None; }
        let (min, max) = (values.iter().cloned().fold(f64::INFINITY, f64::min), values.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
        Some(Self { min, max })
    }
    pub fn width(&self) -> f64 { self.max - self.min }
    pub fn center(&self) -> f64 { (self.min + self.max) / 2.0 }
    pub fn pad(&mut self, percent: f64) { let w = self.width() * percent; self.min -= w; self.max += w; }
    pub fn contains(&self, value: f64) -> bool { self.min <= value && value <= self.max }
}

pub fn mercator_project(lat: f64, lon: f64) -> (f64, f64) {
    let x = (lon + 180.0) / 360.0;
    let lat_rad = lat.to_radians();
    let y = 0.5 - ((lat_rad.tan() + 1.0 / lat_rad.cos()).ln()) / (2.0 * std::f64::consts::PI);
    (x.clamp(0.0, 1.0), y.clamp(0.0, 1.0))
}

pub fn equirectangular_project(lat: f64, lon: f64) -> (f64, f64) {
    let x = (lon + 180.0) / 360.0;
    let y = (90.0 - lat) / 180.0;
    (x.clamp(0.0, 1.0), y.clamp(0.0, 1.0))
}

pub fn spherical_to_cartesian(lat: f64, lon: f64) -> (f64, f64, f64) {
    let lat_rad = lat.to_radians();
    let lon_rad = lon.to_radians();
    let x = lat_rad.cos() * lon_rad.cos();
    let y = lat_rad.cos() * lon_rad.sin();
    let z = lat_rad.sin();
    (x, y, z)
}

pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6371.0;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    R * c
}

pub fn region_centroid(region: &str) -> (f64, f64) {
    match region {
        "Africa" => (1.5, 20.0),
        "Americas" => (15.0, -80.0),
        "Asia" => (35.0, 100.0),
        "Europe" => (50.0, 15.0),
        "Oceania" => (-25.0, 135.0),
        _ => (0.0, 0.0),
    }
}

pub fn sub_region_centroid(sub_region: &str) -> (f64, f64) {
    match sub_region {
        "Northern Africa" => (30.0, 10.0),
        "Sub-Saharan Africa" | "Eastern Africa" => (-5.0, 35.0),
        "Western Africa" => (10.0, -5.0),
        "Middle Africa" => (0.0, 20.0),
        "Southern Africa" => (-30.0, 25.0),
        "Northern America" => (45.0, -100.0),
        "Latin America and the Caribbean" | "Caribbean" => (15.0, -70.0),
        "Central America" => (15.0, -87.0),
        "South America" => (-15.0, -60.0),
        "Eastern Asia" => (35.0, 115.0),
        "South-eastern Asia" => (5.0, 110.0),
        "Southern Asia" => (25.0, 78.0),
        "Western Asia" => (30.0, 45.0),
        "Central Asia" => (42.0, 65.0),
        "Northern Europe" => (60.0, 15.0),
        "Western Europe" => (48.0, 5.0),
        "Eastern Europe" => (52.0, 30.0),
        "Southern Europe" => (42.0, 15.0),
        "Australia and New Zealand" => (-30.0, 145.0),
        "Melanesia" => (-8.0, 155.0),
        "Micronesia" => (8.0, 155.0),
        "Polynesia" => (-15.0, -170.0),
        _ => (0.0, 0.0),
    }
}

pub fn heat_color(value: f64, max_val: f64) -> (u8, u8, u8) {
    let t = (value / max_val.max(1.0)).clamp(0.0, 1.0);
    let r = (255.0 * (2.0 * t - 0.5).clamp(0.0, 1.0)) as u8;
    let g = (255.0 * (1.0 - (2.0 * t - 1.0).abs()).clamp(0.0, 1.0)) as u8;
    let b = (255.0 * (1.0 - 2.0 * t).clamp(0.0, 1.0)) as u8;
    (r, g, b)
}

