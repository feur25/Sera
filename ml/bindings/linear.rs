use super::helpers::*;

#[crate::sera_doc(category = "Linear", en = "Ordinary Least Squares linear regression — analytical Gram/Cholesky solver.", fr = "Régression linéaire par moindres carrés ordinaires — solveur analytique Gram/Cholesky.", file = "linear-regression.md")]
#[crate::sera_alias("linear_regression", "linreg")]
pub fn ml_linear_regression(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let fit_intercept = jb(&v, "fit_intercept", true);
    let mut model = crate::ml::linear::ols::LinearRegression::new(fit_intercept);
    let y = yf(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(r#"{{"predictions":{},"coef":{},"intercept":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default(),
        serde_json::to_string(&model.coef).unwrap_or_default(),
        model.intercept)
}

#[crate::sera_alias("ridge", "ridge_regression")]
pub fn ml_ridge(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let alpha = jf(&v, "alpha", 1.0);
    let fit_intercept = jb(&v, "fit_intercept", true);
    let mut model = crate::ml::linear::ridge::Ridge::new(alpha, fit_intercept);
    let y = yf(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(r#"{{"predictions":{},"coef":{},"intercept":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default(),
        serde_json::to_string(&model.coef).unwrap_or_default(),
        model.intercept)
}

#[crate::sera_alias("lasso")]
pub fn ml_lasso(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let alpha = jf(&v, "alpha", 1.0);
    let max_iter = ju(&v, "max_iter", 1000);
    let tol = jf(&v, "tol", 1e-4);
    let fit_intercept = jb(&v, "fit_intercept", true);
    let mut model = crate::ml::linear::lasso::Lasso::new(alpha, max_iter, tol, fit_intercept);
    let y = yf(&v);
    model.fit_resumable(&xf, n, p, &y, None);
    let preds = model.predict(&xtf, nt, p);
    format!(r#"{{"predictions":{},"coef":{},"intercept":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default(),
        serde_json::to_string(&model.coef().to_vec()).unwrap_or_default(),
        model.intercept())
}

#[crate::sera_alias("elastic_net", "elasticnet")]
pub fn ml_elastic_net(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let alpha = jf(&v, "alpha", 1.0);
    let l1_ratio = jf(&v, "l1_ratio", 0.5);
    let max_iter = ju(&v, "max_iter", 1000);
    let tol = jf(&v, "tol", 1e-4);
    let fit_intercept = jb(&v, "fit_intercept", true);
    let mut model = crate::ml::linear::elastic_net::ElasticNet::new(alpha, l1_ratio, max_iter, tol, fit_intercept);
    let y = yf(&v);
    model.fit_resumable(&xf, n, p, &y, None);
    let preds = model.predict(&xtf, nt, p);
    format!(r#"{{"predictions":{},"coef":{},"intercept":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default(),
        serde_json::to_string(&model.coef).unwrap_or_default(),
        model.intercept)
}

#[crate::sera_alias("logistic_regression", "logistic")]
pub fn ml_logistic_regression(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let c = jf(&v, "C", 1.0);
    let max_iter = ju(&v, "max_iter", 100);
    let tol = jf(&v, "tol", 1e-4);
    let fit_intercept = jb(&v, "fit_intercept", true);
    let mut model = crate::ml::linear::logistic::LogisticRegression::new(c, max_iter, tol, fit_intercept);
    let y = yi(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(r#"{{"predictions":{},"coef":{},"intercept":{},"classes":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default(),
        serde_json::to_string(&model.coef).unwrap_or_default(),
        model.intercept,
        serde_json::to_string(&model.classes).unwrap_or_default())
}

#[crate::sera_alias("ridge_classifier", "ridge_cls")]
pub fn ml_ridge_classifier(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let alpha = jf(&v, "alpha", 1.0);
    let mut model = crate::ml::linear::ridge::RidgeClassifier::new(alpha);
    let y = yi(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(r#"{{"predictions":{}}}"#, serde_json::to_string(&preds).unwrap_or_default())
}

#[crate::sera_alias("sgd_classifier", "sgd_cls")]
pub fn ml_sgd_classifier(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let loss = match js(&v, "loss", "hinge") {
        "log" | "log_loss" => crate::ml::linear::sgd::SGDLoss::Log,
        "modified_huber" => crate::ml::linear::sgd::SGDLoss::ModifiedHuber,
        "squared_hinge" => crate::ml::linear::sgd::SGDLoss::SquaredHinge,
        _ => crate::ml::linear::sgd::SGDLoss::Hinge,
    };
    let alpha = jf(&v, "alpha", 0.0001);
    let max_iter = ju(&v, "max_iter", 1000);
    let tol = jf(&v, "tol", 1e-3);
    let eta0 = jf(&v, "eta0", 1.0);
    let fit_intercept = jb(&v, "fit_intercept", true);
    let mut model = crate::ml::linear::sgd::SGDClassifier::new(loss, alpha, max_iter, tol, eta0, fit_intercept);
    let y = yi(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(r#"{{"predictions":{},"coef":{},"intercept":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default(),
        serde_json::to_string(&model.coef).unwrap_or_default(),
        model.intercept)
}

#[crate::sera_alias("sgd_regressor", "sgd_reg")]
pub fn ml_sgd_regressor(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let alpha = jf(&v, "alpha", 0.0001);
    let max_iter = ju(&v, "max_iter", 1000);
    let tol = jf(&v, "tol", 1e-3);
    let eta0 = jf(&v, "eta0", 0.01);
    let fit_intercept = jb(&v, "fit_intercept", true);
    let mut model = crate::ml::linear::sgd::SGDRegressor::new(alpha, max_iter, tol, eta0, fit_intercept);
    let y = yf(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(r#"{{"predictions":{},"coef":{},"intercept":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default(),
        serde_json::to_string(&model.coef).unwrap_or_default(),
        model.intercept)
}
