use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

static NEXT_HANDLE: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ModelHandle(pub u64);

pub enum BindingError {
    UnknownHandle,
    UnknownKind(String),
    InvalidParams(String),
    ComputeError(String),
}

impl std::fmt::Display for BindingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownHandle => write!(f, "unknown model handle"),
            Self::UnknownKind(k) => write!(f, "unknown model kind: {k}"),
            Self::InvalidParams(e) => write!(f, "invalid params: {e}"),
            Self::ComputeError(e) => write!(f, "compute error: {e}"),
        }
    }
}

pub trait Model: Send + Sync {
    fn fit_json(&mut self, x_json: &str, y_json: &str) -> Result<(), BindingError>;
    fn predict_json(&self, x_json: &str) -> Result<String, BindingError>;
    fn score_json(&self, x_json: &str, y_json: &str) -> Result<f64, BindingError>;
    fn get_params_json(&self) -> String;
    fn set_params_json(&mut self, params_json: &str) -> Result<(), BindingError>;
}

struct ModelStore {
    entries: HashMap<u64, Box<dyn Model>>,
}

lazy_static::lazy_static! {
    static ref STORE: Mutex<ModelStore> = Mutex::new(ModelStore { entries: HashMap::new() });
}

fn alloc(model: Box<dyn Model>) -> Result<ModelHandle, BindingError> {
    let id = NEXT_HANDLE.fetch_add(1, Ordering::SeqCst);
    let mut s = STORE
        .lock()
        .map_err(|_| BindingError::ComputeError("store lock".into()))?;
    s.entries.insert(id, model);
    Ok(ModelHandle(id))
}

pub fn model_free(handle: ModelHandle) {
    match STORE.lock() {
        Ok(mut s) => {
            s.entries.remove(&handle.0);
        }
        Err(_) => eprintln!("seraplot ml: model_free({}) skipped, store lock poisoned", handle.0),
    }
}

pub fn model_fit(handle: ModelHandle, x_json: &str, y_json: &str) -> Result<(), BindingError> {
    let mut s = STORE
        .lock()
        .map_err(|_| BindingError::ComputeError("store lock".into()))?;
    let m = s
        .entries
        .get_mut(&handle.0)
        .ok_or(BindingError::UnknownHandle)?;
    m.fit_json(x_json, y_json)
}

pub fn model_predict(handle: ModelHandle, x_json: &str) -> Result<String, BindingError> {
    let s = STORE
        .lock()
        .map_err(|_| BindingError::ComputeError("store lock".into()))?;
    let m = s
        .entries
        .get(&handle.0)
        .ok_or(BindingError::UnknownHandle)?;
    m.predict_json(x_json)
}

pub fn model_score(handle: ModelHandle, x_json: &str, y_json: &str) -> Result<f64, BindingError> {
    let s = STORE
        .lock()
        .map_err(|_| BindingError::ComputeError("store lock".into()))?;
    let m = s
        .entries
        .get(&handle.0)
        .ok_or(BindingError::UnknownHandle)?;
    m.score_json(x_json, y_json)
}

pub fn model_get_params(handle: ModelHandle) -> Result<String, BindingError> {
    let s = STORE
        .lock()
        .map_err(|_| BindingError::ComputeError("store lock".into()))?;
    let m = s
        .entries
        .get(&handle.0)
        .ok_or(BindingError::UnknownHandle)?;
    Ok(m.get_params_json())
}

pub fn model_set_params(handle: ModelHandle, params_json: &str) -> Result<(), BindingError> {
    let mut s = STORE
        .lock()
        .map_err(|_| BindingError::ComputeError("store lock".into()))?;
    let m = s
        .entries
        .get_mut(&handle.0)
        .ok_or(BindingError::UnknownHandle)?;
    m.set_params_json(params_json)
}

pub fn model_create(kind: &str, params_json: &str) -> Result<ModelHandle, BindingError> {
    use crate::ml::bindings::helpers::{jb, jf};
    let v: serde_json::Value = serde_json::from_str(params_json)
        .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));
    let boxed: Box<dyn Model> = match kind {
        "linear_regression" => Box::new(LinearRegressionHandle {
            inner: crate::ml::linear::ols::LinearRegression::new(jb(&v, "fit_intercept", true)),
        }),
        "ridge" => Box::new(RidgeHandle {
            inner: crate::ml::linear::ridge::Ridge::new(
                jf(&v, "alpha", 1.0),
                jb(&v, "fit_intercept", true),
            ),
        }),
        _ => return Err(BindingError::UnknownKind(kind.into())),
    };
    alloc(boxed)
}

struct LinearRegressionHandle {
    inner: crate::ml::linear::ols::LinearRegression,
}

impl Model for LinearRegressionHandle {
    fn fit_json(&mut self, x_json: &str, y_json: &str) -> Result<(), BindingError> {
        let x: Vec<Vec<f64>> =
            serde_json::from_str(x_json).map_err(|e| BindingError::InvalidParams(e.to_string()))?;
        let y: Vec<f64> =
            serde_json::from_str(y_json).map_err(|e| BindingError::InvalidParams(e.to_string()))?;
        let n = x.len();
        let p = x.first().map(|r| r.len()).unwrap_or(0);
        let flat: Vec<f64> = x.into_iter().flatten().collect();
        self.inner.fit(&flat, n, p, &y);
        Ok(())
    }

    fn predict_json(&self, x_json: &str) -> Result<String, BindingError> {
        let x: Vec<Vec<f64>> =
            serde_json::from_str(x_json).map_err(|e| BindingError::InvalidParams(e.to_string()))?;
        let n = x.len();
        let p = x.first().map(|r| r.len()).unwrap_or(0);
        let flat: Vec<f64> = x.into_iter().flatten().collect();
        let preds = self.inner.predict(&flat, n, p);
        serde_json::to_string(&preds).map_err(|e| BindingError::ComputeError(e.to_string()))
    }

    fn score_json(&self, x_json: &str, y_json: &str) -> Result<f64, BindingError> {
        let x: Vec<Vec<f64>> =
            serde_json::from_str(x_json).map_err(|e| BindingError::InvalidParams(e.to_string()))?;
        let y: Vec<f64> =
            serde_json::from_str(y_json).map_err(|e| BindingError::InvalidParams(e.to_string()))?;
        let n = x.len();
        let p = x.first().map(|r| r.len()).unwrap_or(0);
        let flat: Vec<f64> = x.into_iter().flatten().collect();
        Ok(self.inner.score(&flat, n, p, &y))
    }

    fn get_params_json(&self) -> String {
        format!(r#"{{"fit_intercept":{}}}"#, self.inner.fit_intercept)
    }

    fn set_params_json(&mut self, params_json: &str) -> Result<(), BindingError> {
        let v: serde_json::Value = serde_json::from_str(params_json)
            .map_err(|e| BindingError::InvalidParams(e.to_string()))?;
        if let Some(b) = v.get("fit_intercept").and_then(|x| x.as_bool()) {
            self.inner.fit_intercept = b;
        }
        Ok(())
    }
}

struct RidgeHandle {
    inner: crate::ml::linear::ridge::Ridge,
}

impl Model for RidgeHandle {
    fn fit_json(&mut self, x_json: &str, y_json: &str) -> Result<(), BindingError> {
        let x: Vec<Vec<f64>> =
            serde_json::from_str(x_json).map_err(|e| BindingError::InvalidParams(e.to_string()))?;
        let y: Vec<f64> =
            serde_json::from_str(y_json).map_err(|e| BindingError::InvalidParams(e.to_string()))?;
        let n = x.len();
        let p = x.first().map(|r| r.len()).unwrap_or(0);
        let flat: Vec<f64> = x.into_iter().flatten().collect();
        self.inner.fit(&flat, n, p, &y);
        Ok(())
    }

    fn predict_json(&self, x_json: &str) -> Result<String, BindingError> {
        let x: Vec<Vec<f64>> =
            serde_json::from_str(x_json).map_err(|e| BindingError::InvalidParams(e.to_string()))?;
        let n = x.len();
        let p = x.first().map(|r| r.len()).unwrap_or(0);
        let flat: Vec<f64> = x.into_iter().flatten().collect();
        let preds = self.inner.predict(&flat, n, p);
        serde_json::to_string(&preds).map_err(|e| BindingError::ComputeError(e.to_string()))
    }

    fn score_json(&self, x_json: &str, y_json: &str) -> Result<f64, BindingError> {
        let x: Vec<Vec<f64>> =
            serde_json::from_str(x_json).map_err(|e| BindingError::InvalidParams(e.to_string()))?;
        let y: Vec<f64> =
            serde_json::from_str(y_json).map_err(|e| BindingError::InvalidParams(e.to_string()))?;
        let n = x.len();
        let p = x.first().map(|r| r.len()).unwrap_or(0);
        let flat: Vec<f64> = x.into_iter().flatten().collect();
        Ok(self.inner.score(&flat, n, p, &y))
    }

    fn get_params_json(&self) -> String {
        format!(
            r#"{{"alpha":{},"fit_intercept":{}}}"#,
            self.inner.alpha, self.inner.fit_intercept
        )
    }

    fn set_params_json(&mut self, params_json: &str) -> Result<(), BindingError> {
        let v: serde_json::Value = serde_json::from_str(params_json)
            .map_err(|e| BindingError::InvalidParams(e.to_string()))?;
        if let Some(a) = v.get("alpha").and_then(|x| x.as_f64()) {
            self.inner.alpha = a;
        }
        if let Some(b) = v.get("fit_intercept").and_then(|x| x.as_bool()) {
            self.inner.fit_intercept = b;
        }
        Ok(())
    }
}
