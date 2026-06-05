use super::helpers::*;

#[crate::sera_doc(
    category = "Naive Bayes",
    en = "Gaussian Naive Bayes — likelihood modelled as Gaussian per class per feature.",
    fr = "Naive Bayes Gaussien — vraisemblance modélisée comme Gaussienne par classe et feature.",
    file = "naive-bayes.md"
)]
#[crate::sera_alias("gaussian_nb")]
pub fn ml_gaussian_nb(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let var_smoothing = jf(&v, "var_smoothing", 1e-9);
    let mut model = crate::ml::naive_bayes::gaussian::GaussianNB::new();
    let _ = var_smoothing;
    let y = yi(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default()
    )
}

#[crate::sera_alias("multinomial_nb")]
pub fn ml_multinomial_nb(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let alpha = jf(&v, "alpha", 1.0);
    let mut model = crate::ml::naive_bayes::multinomial::MultinomialNB::new(alpha);
    let y = yi(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default()
    )
}

#[crate::sera_alias("bernoulli_nb")]
pub fn ml_bernoulli_nb(input: &str) -> String {
    let (v, xf, n, p, xtf, nt) = ml_parse(input);
    let alpha = jf(&v, "alpha", 1.0);
    let binarize = jf(&v, "binarize", 0.0);
    let mut model = crate::ml::naive_bayes::bernoulli::BernoulliNB::new(alpha, binarize);
    let y = yi(&v);
    model.fit(&xf, n, p, &y);
    let preds = model.predict(&xtf, nt, p);
    format!(
        r#"{{"predictions":{}}}"#,
        serde_json::to_string(&preds).unwrap_or_default()
    )
}
