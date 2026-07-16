use super::helpers::*;

#[crate::sera_doc(
    category = "Linear",
    en = "Ordinary Least Squares linear regression — analytical Gram/Cholesky solver.",
    fr = "Régression linéaire par moindres carrés ordinaires — solveur analytique Gram/Cholesky.",
    file = "linear-regression.md"
)]
#[crate::sera_alias("linear_regression", "linreg")]
#[crate::sera_builder]
pub fn ml_linear_regression(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let fit_intercept = jb(&v, "fit_intercept", true);
    let mut model = crate::ml::linear::ols::LinearRegression::new(fit_intercept);
    let y = yf(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{},"coef":{},"intercept":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default(),
        serde_json::to_string(&model.coef).unwrap_or_default(),
        model.intercept
    )
}

#[crate::sera_doc(
    category = "Linear",
    en = "Ridge regression — L2-regularized linear regression.",
    fr = "Regression Ridge — regression lineaire regularisee L2.",
    file = "ridge.md"
)]
#[crate::sera_alias("ridge", "ridge_regression")]
#[crate::sera_builder]
pub fn ml_ridge(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let alpha = jf(&v, "alpha", 1.0);
    let fit_intercept = jb(&v, "fit_intercept", true);
    let mut model = crate::ml::linear::ridge::Ridge::new(alpha, fit_intercept);
    let y = yf(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{},"coef":{},"intercept":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default(),
        serde_json::to_string(&model.coef).unwrap_or_default(),
        model.intercept
    )
}

#[crate::sera_doc(
    category = "Linear",
    en = "Lasso regression — L1-regularized linear regression.",
    fr = "Regression Lasso — regression lineaire regularisee L1.",
    file = "lasso.md"
)]
#[crate::sera_alias("lasso")]
#[crate::sera_builder]
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
    format!(
        r#"{{"predictions":{},"coef":{},"intercept":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default(),
        serde_json::to_string(&model.coef().to_vec()).unwrap_or_default(),
        model.intercept()
    )
}

#[crate::sera_doc(
    category = "Linear",
    en = "Elastic Net regression — combined L1 and L2 regularization.",
    fr = "Regression Elastic Net — regularisation combinee L1 et L2.",
    file = "elastic-net.md"
)]
#[crate::sera_alias("elastic_net", "elasticnet")]
#[crate::sera_builder]
pub fn ml_elastic_net(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let alpha = jf(&v, "alpha", 1.0);
    let l1_ratio = jf(&v, "l1_ratio", 0.5);
    let max_iter = ju(&v, "max_iter", 1000);
    let tol = jf(&v, "tol", 1e-4);
    let fit_intercept = jb(&v, "fit_intercept", true);
    let mut model = crate::ml::linear::elastic_net::ElasticNet::new(
        alpha,
        l1_ratio,
        max_iter,
        tol,
        fit_intercept,
    );
    let y = yf(&v);
    model.fit_resumable(&xf, n, p, &y, None);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{},"coef":{},"intercept":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default(),
        serde_json::to_string(&model.coef).unwrap_or_default(),
        model.intercept
    )
}

#[crate::sera_doc(
    category = "Linear",
    en = "Logistic regression — linear classifier trained with iterative optimization.",
    fr = "Regression logistique — classifieur lineaire entraine par optimisation iterative.",
    file = "logistic-regression.md"
)]
#[crate::sera_alias("logistic_regression", "logistic")]
#[crate::sera_builder]
pub fn ml_logistic_regression(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let c = jf(&v, "C", 1.0);
    let max_iter = ju(&v, "max_iter", 100);
    let tol = jf(&v, "tol", 1e-4);
    let fit_intercept = jb(&v, "fit_intercept", true);
    let mut model =
        crate::ml::linear::logistic::LogisticRegression::new(c, max_iter, tol, fit_intercept);
    let y = yi(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{},"coef":{},"intercept":{},"classes":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default(),
        serde_json::to_string(&model.coef).unwrap_or_default(),
        model.intercept,
        serde_json::to_string(&model.classes).unwrap_or_default()
    )
}

#[crate::sera_doc(
    category = "Linear",
    en = "RidgeClassifier — L2-regularized linear classifier.",
    fr = "RidgeClassifier — classifieur lineaire regularise L2.",
    file = "ridge.md"
)]
#[crate::sera_alias("ridge_classifier", "ridge_cls")]
#[crate::sera_builder]
pub fn ml_ridge_classifier(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let alpha = jf(&v, "alpha", 1.0);
    let mut model = crate::ml::linear::ridge::RidgeClassifier::new(alpha);
    let y = yi(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default()
    )
}

#[crate::sera_doc(
    category = "Linear",
    en = "SGDClassifier — linear classifier optimized with stochastic gradient descent.",
    fr = "SGDClassifier — classifieur lineaire optimise par descente de gradient stochastique.",
    file = "sgd.md"
)]
#[crate::sera_alias("sgd_classifier", "sgd_cls")]
#[crate::sera_builder]
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
    let mut model =
        crate::ml::linear::sgd::SGDClassifier::new(loss, alpha, max_iter, tol, eta0, fit_intercept);
    let y = yi(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{},"coef":{},"intercept":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default(),
        serde_json::to_string(&model.coef).unwrap_or_default(),
        model.intercept
    )
}

#[crate::sera_doc(
    category = "Linear",
    en = "SGDRegressor — linear regressor optimized with stochastic gradient descent.",
    fr = "SGDRegressor — regresseur lineaire optimise par descente de gradient stochastique.",
    file = "sgd.md"
)]
#[crate::sera_alias("sgd_regressor", "sgd_reg")]
#[crate::sera_builder]
pub fn ml_sgd_regressor(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let alpha = jf(&v, "alpha", 0.0001);
    let max_iter = ju(&v, "max_iter", 1000);
    let tol = jf(&v, "tol", 1e-3);
    let eta0 = jf(&v, "eta0", 0.01);
    let fit_intercept = jb(&v, "fit_intercept", true);
    let mut model =
        crate::ml::linear::sgd::SGDRegressor::new(alpha, max_iter, tol, eta0, fit_intercept);
    let y = yf(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{},"coef":{},"intercept":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default(),
        serde_json::to_string(&model.coef).unwrap_or_default(),
        model.intercept
    )
}
