pub mod anomaly;
pub mod state;
pub use state::*;
pub mod clustering;
pub mod decomposition;
pub mod ensemble;
pub mod helpers;
pub mod importance;
pub mod linear;
pub mod metrics;
pub mod model_selection;
pub mod naive_bayes;
pub mod neighbors;
pub mod persistence;
pub mod preprocessing;
pub mod svm;
pub mod tree;

pub use anomaly::*;
pub use clustering::*;
pub use decomposition::*;
pub use ensemble::*;
pub use importance::*;
pub use linear::*;
pub use metrics::*;
pub use model_selection::*;
pub use naive_bayes::*;
pub use neighbors::*;
pub use persistence::*;
pub use preprocessing::*;
pub use svm::*;
pub use tree::*;

#[cfg(feature = "python")]
mod python_bindings {
    use super::*;
    use numpy::{IntoPyArray, PyArrayMethods, PyReadonlyArray1, PyReadonlyArray2, PyUntypedArrayMethods};
    use pyo3::prelude::*;
    use pyo3::types::{PyAny, PyDict};
    use rayon::prelude::*;

    include!("python_bindings/common.rs");
    include!("python_bindings/estimators.rs");
    include!("python_bindings/model_selection.rs");
    include!("python_bindings/clustering.rs");
    include!("python_bindings/registry.rs");
    include!("python_bindings/register.rs");
}

#[cfg(feature = "python")]
pub use python_bindings::*;