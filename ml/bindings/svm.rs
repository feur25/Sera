use super::helpers::*;

#[crate::sera_doc(
    category = "SVM",
    en = "LinearSVC — linear Support Vector Machine for classification via dual coordinate descent.",
    fr = "LinearSVC — Machine à vecteurs de support linéaire pour classification.",
    file = "svm.md"
)]
#[crate::sera_alias("linear_svc", "svc")]
pub fn ml_linear_svc(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let c = jf(&v, "C", 1.0);
    let max_iter = ju(&v, "max_iter", 1000);
    let tol = jf(&v, "tol", 1e-4);
    let mut model = crate::ml::svm::svm::LinearSVC::new(c, max_iter, tol);
    let y = yi(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default()
    )
}

#[crate::sera_doc(
    category = "SVM",
    en = "LinearSVR — linear Support Vector Regressor.",
    fr = "LinearSVR — regresseur a vecteurs de support lineaire.",
    file = "svm.md"
)]
#[crate::sera_alias("linear_svr", "svr")]
pub fn ml_linear_svr(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let c = jf(&v, "C", 1.0);
    let epsilon = jf(&v, "epsilon", 0.1);
    let max_iter = ju(&v, "max_iter", 1000);
    let tol = jf(&v, "tol", 1e-4);
    let mut model = crate::ml::svm::svm::LinearSVR::new(c, epsilon, max_iter, tol);
    let y = yf(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default()
    )
}
