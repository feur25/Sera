pub mod ols;
pub mod ridge;
pub mod lasso;
pub mod elastic_net;
pub mod logistic;
pub mod sgd;

pub use ols::*;
pub use ridge::*;
pub use lasso::*;
pub use elastic_net::*;
pub use logistic::*;
pub use sgd::*;
