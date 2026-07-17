(function () {
  function esc(s) {
    return String(s == null ? "" : s)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;");
  }

  function code(s) {
    return "<code>" + esc(s) + "</code>";
  }

  function codeList(xs) {
    xs = (xs || []).filter(Boolean);
    return xs.length ? xs.map(code).join(" ") : "<span class=\"ml-doc-muted\">-</span>";
  }

  function reg() {
    return window.SeraPlotMlRegistry || { docs: [], models: [] };
  }

  function elemLang(el) {
    return el.closest && el.closest(".lang-fr") ? "fr" : "en";
  }

  function attrList(el, name) {
    return (el.getAttribute(name) || "")
      .split(",")
      .map(function (s) { return s.trim(); })
      .filter(Boolean);
  }

  function byName(a, b) {
    return String(a.name || "").localeCompare(String(b.name || ""));
  }

  function uniq(xs) {
    var seen = {};
    return (xs || []).filter(function (x) {
      if (!x || seen[x]) return false;
      seen[x] = true;
      return true;
    });
  }

  function docsFor(el) {
    var r = reg();
    var file = el.getAttribute("data-file") || "";
    var category = el.getAttribute("data-category") || "";
    var names = attrList(el, "data-names");
    return (r.docs || []).filter(function (d) {
      if (file && d.file !== file) return false;
      if (category && d.category !== category) return false;
      if (names.length && names.indexOf(d.name) === -1) return false;
      return true;
    }).sort(byName);
  }

  function modelsFor(el) {
    var r = reg();
    var category = el.getAttribute("data-category") || "";
    var names = attrList(el, "data-models");
    return (r.models || []).filter(function (m) {
      if (m.domain && m.domain !== "ml") return false;
      if (category && m.category !== category) return false;
      if (names.length && names.indexOf(m.name) === -1) return false;
      return true;
    }).sort(byName);
  }

  function titleize(s) {
    return String(s || "")
      .replace(/^ml_/, "")
      .replace(/_/g, " ")
      .replace(/\b\w/g, function (c) { return c.toUpperCase(); });
  }

  function textFor(d, l) {
    return l === "fr" ? (d.fr || d.en || "") : (d.en || d.fr || "");
  }

  function categoryOf(rows, models) {
    var cats = uniq((rows || []).map(function (d) { return d.category; }).concat((models || []).map(function (m) { return m.category; })));
    return cats.length === 1 ? cats[0] : cats.join(", ");
  }

  function linkFor(file) {
    return esc(String(file || "index.md").replace(/\.md$/, ".html"));
  }

  function paramType(name) {
    if (/^(data|test_data|x)$/.test(name)) return "list[list[float]]";
    if (/^(target|y_true|y_pred|y_score|labels|labels_true|labels_pred|alphas)$/.test(name)) return "list";
    if (/^(fit_intercept|classification|stratified|with_mean|with_std|with_centering|with_scaling|include_bias|interaction_only)$/.test(name)) return "bool";
    if (/^(n|k|seed|n_neighbors|n_components|n_estimators|max_iter|n_init|n_repeats|max_depth|min_samples|min_samples_split|min_samples_leaf|degree|n_bins|n_quantiles|version|pos_label)$/.test(name)) return "int";
    if (/^(alpha|C|learning_rate|eta0|epsilon|tol|contamination|max_samples|eps|feature_range_min|feature_range_max|fill_value|l1_ratio|beta|binarize|var_smoothing)$/.test(name)) return "float";
    if (/^(params|metrics)$/.test(name)) return "dict";
    if (/^(tags)$/.test(name)) return "list[str]";
    return "str";
  }

  function paramText(name, l) {
    var fr = {
      data: "Donnees d'entrainement tabulaires.",
      test_data: "Donnees a transformer ou predire.",
      target: "Valeurs cibles pour l'apprentissage supervise.",
      x: "Donnees tabulaires utilisees par les metriques de clustering.",
      y_true: "Valeurs attendues.",
      y_pred: "Valeurs predites.",
      y_score: "Scores continus pour courbes et metriques.",
      labels: "Etiquettes de clusters.",
      labels_true: "Etiquettes de reference.",
      labels_pred: "Etiquettes predites.",
      name: "Nom de metrique, courbe, transformeur ou modele.",
      kind: "Famille du modele sauvegarde.",
      payload: "Charge utile JSON du modele.",
      version: "Version optionnelle du modele.",
      k: "Nombre de folds ou de groupes selon la fonction.",
      n: "Nombre d'observations.",
      seed: "Graine deterministe.",
      n_neighbors: "Nombre de voisins.",
      n_components: "Nombre de composantes.",
      n_estimators: "Nombre d'estimateurs.",
      max_depth: "Profondeur maximale.",
      learning_rate: "Taux d'apprentissage.",
      alpha: "Force de regularisation.",
      C: "Inverse de regularisation.",
      eps: "Rayon de voisinage ou tolerance numerique.",
      min_samples: "Nombre minimum de points.",
      weights: "Strategie de ponderation.",
      criterion: "Critere de split.",
      loss: "Fonction de perte.",
      with_mean: "Centre les donnees.",
      with_std: "Normalise l'ecart-type.",
      feature_range_min: "Borne basse de la plage cible.",
      feature_range_max: "Borne haute de la plage cible."
    };
    var en = {
      data: "Tabular training data.",
      test_data: "Data to transform or predict.",
      target: "Target values for supervised learning.",
      x: "Tabular data used by clustering metrics.",
      y_true: "Ground-truth values.",
      y_pred: "Predicted values.",
      y_score: "Continuous scores for curves and metrics.",
      labels: "Cluster labels.",
      labels_true: "Reference labels.",
      labels_pred: "Predicted labels.",
      name: "Metric, curve, transformer or model name.",
      kind: "Stored model family.",
      payload: "Model JSON payload.",
      version: "Optional model version.",
      k: "Number of folds or groups depending on the function.",
      n: "Number of observations.",
      seed: "Deterministic seed.",
      n_neighbors: "Number of neighbors.",
      n_components: "Number of components.",
      n_estimators: "Number of estimators.",
      max_depth: "Maximum depth.",
      learning_rate: "Learning rate.",
      alpha: "Regularization strength.",
      C: "Inverse regularization strength.",
      eps: "Neighborhood radius or numeric tolerance.",
      min_samples: "Minimum number of points.",
      weights: "Weighting strategy.",
      criterion: "Split criterion.",
      loss: "Loss function.",
      with_mean: "Centers the data.",
      with_std: "Scales to unit variance.",
      feature_range_min: "Lower bound of the output range.",
      feature_range_max: "Upper bound of the output range."
    };
    return (l === "fr" ? fr[name] : en[name]) || (l === "fr" ? "Cle JSON acceptee par la fonction." : "JSON key accepted by the function.");
  }

  function selectedParams(d) {
    if (d.name === "ml_metric_score") return ["name", "y_true", "y_pred"];
    if (d.name === "ml_metric_curve") return ["name", "y_true", "y_score", "pos_label"];
    if (d.name === "ml_fit_transform") return ["name", "data"];
    if (d.name === "ml_kfold_split") return ["n", "k", "seed"];
    if (d.name === "ml_cross_val_score" || d.name === "ml_grid_search_cv") return ["data", "target", "k", "seed"];
    if (d.name === "ml_load_model") return ["name"];
    var essential = ["data", "target", "test_data", "x", "y_true", "y_pred", "y_score", "labels", "labels_true", "labels_pred", "name", "n", "k", "seed", "kind", "payload", "version"];
    var options = ["n_neighbors", "weights", "alpha", "C", "eps", "min_samples", "n_estimators", "max_depth", "learning_rate", "n_components", "with_mean", "with_std", "feature_range_min", "feature_range_max", "criterion", "loss", "classification", "stratified", "pos_label"];
    var params = d.params || [];
    var out = params.filter(function (p) { return essential.indexOf(p) !== -1; });
    params.forEach(function (p) {
      if (out.indexOf(p) === -1 && options.indexOf(p) !== -1 && out.length < 9) out.push(p);
    });
    if (!out.length) out = params.slice(0, 6);
    return out;
  }

  function literalForParam(name, d) {
    if (name === "name" && d.name === "ml_metric_score") return "\"accuracy_score\"";
    if (name === "name" && d.name === "ml_metric_curve") return "\"roc_curve\"";
    if (name === "name" && d.name === "ml_fit_transform") return "\"StandardScaler\"";
    if (name === "name") return "\"demo_model\"";
    if (name === "kind") return "\"linear\"";
    if (name === "payload") return "\"{}\"";
    if (name === "version") return "1";
    if (name === "k") return d.name === "ml_kmeans_fit_predict" ? "2" : "3";
    if (name === "n") return "8";
    if (name === "seed") return "42";
    if (name === "n_neighbors") return "3";
    if (name === "n_components") return "2";
    if (name === "n_estimators") return "20";
    if (name === "max_depth") return "3";
    if (name === "learning_rate") return "0.1";
    if (name === "alpha") return "1.0";
    if (name === "C") return "1.0";
    if (name === "eps") return "0.5";
    if (name === "min_samples") return "2";
    if (name === "weights") return "\"uniform\"";
    if (name === "criterion") return "\"gini\"";
    if (name === "loss") return "\"hinge\"";
    if (name === "classification") return "False";
    if (name === "stratified") return "False";
    if (name === "with_mean" || name === "with_std") return "True";
    if (name === "feature_range_min") return "0.0";
    if (name === "feature_range_max") return "1.0";
    return null;
  }

  function varForParam(name) {
    if (name === "data" || name === "x") return "X";
    if (name === "test_data") return "X_test";
    if (name === "target") return "y";
    if (name === "y_true") return "y_true";
    if (name === "y_pred") return "y_pred";
    if (name === "y_score") return "y_score";
    if (name === "labels") return "labels";
    if (name === "labels_true") return "labels_true";
    if (name === "labels_pred") return "labels_pred";
    return null;
  }

  function variableLines(params) {
    var lines = [];
    if (params.indexOf("data") !== -1 || params.indexOf("x") !== -1) {
      lines.push("X = [[0.0, 1.0], [1.0, 2.0], [2.0, 1.0], [3.0, 3.0]]");
    }
    if (params.indexOf("target") !== -1) lines.push("y = [0, 1, 0, 1]");
    if (params.indexOf("test_data") !== -1) lines.push("X_test = [[1.5, 2.0], [3.5, 3.0]]");
    if (params.indexOf("y_true") !== -1) lines.push("y_true = [0, 1, 1, 0]");
    if (params.indexOf("y_pred") !== -1) lines.push("y_pred = [0, 1, 0, 0]");
    if (params.indexOf("y_score") !== -1) lines.push("y_score = [0.05, 0.81, 0.72, 0.12]");
    if (params.indexOf("labels") !== -1) lines.push("labels = [0, 0, 1, 1]");
    if (params.indexOf("labels_true") !== -1) lines.push("labels_true = [0, 0, 1, 1]");
    if (params.indexOf("labels_pred") !== -1) lines.push("labels_pred = [0, 1, 1, 1]");
    return lines;
  }

  function resultKey(d) {
    var returns = d.returns || [];
    if (returns.indexOf("predictions") !== -1) return "predictions";
    if (returns.indexOf("data") !== -1) return "data";
    if (returns.indexOf("value") !== -1) return "value";
    if (returns.indexOf("labels") !== -1) return "labels";
    if (returns.indexOf("scores") !== -1) return "scores";
    if (returns.indexOf("best_alpha") !== -1) return "best_alpha";
    if (returns.indexOf("version") !== -1) return "version";
    if (returns.indexOf("found") !== -1) return "found";
    if (returns.length) return returns[0];
    return "";
  }

  function exampleCode(d) {
    if (d.name === "ml_load_model") {
      return [
        "import json",
        "import seraplot as sp",
        "",
        "saved = json.loads(sp.ml_save_model(json.dumps({",
        "    \"name\": \"demo_model\",",
        "    \"kind\": \"linear\",",
        "    \"payload\": \"{}\",",
        "})))",
        "",
        "result = json.loads(sp.ml_load_model(json.dumps({",
        "    \"name\": saved[\"name\"],",
        "    \"version\": saved[\"version\"],",
        "})))",
        "print(result[\"found\"])"
      ].join("\n");
    }
    var params = selectedParams(d);
    var lines = ["import json", "import seraplot as sp"];
    var vars = variableLines(params);
    if (vars.length) lines = lines.concat([""]).concat(vars);
    lines.push("");
    lines.push("result = json.loads(sp." + d.name + "(json.dumps({");
    params.forEach(function (p) {
      var v = varForParam(p) || literalForParam(p, d);
      if (v != null) lines.push("    \"" + p + "\": " + v + ",");
    });
    lines.push("})))");
    var key = resultKey(d);
    lines.push(key ? "print(result[\"" + key + "\"])" : "print(result)");
    return lines.join("\n");
  }

  function renderHero(rows, models, l) {
    var cat = categoryOf(rows, models) || (l === "fr" ? "Machine learning" : "Machine Learning");
    var description = rows.length ? textFor(rows[0], l) : (l === "fr" ? "Reference generee depuis les decorateurs Rust." : "Reference generated from Rust decorators.");
    var labels = [
      rows.length + " " + (l === "fr" ? "fonctions JSON" : "JSON functions"),
      models.length + " " + (l === "fr" ? "modeles decores" : "decorated models")
    ];
    return "<section class=\"ml-doc-hero\">" +
      "<div><span class=\"ml-doc-kicker\">" + esc(cat) + "</span><p>" + esc(description) + "</p></div>" +
      "<div class=\"ml-doc-counts\">" + labels.map(function (x) { return "<span>" + esc(x) + "</span>"; }).join("") + "</div>" +
      "</section>";
  }

  function renderFunctionCards(rows, l) {
    if (!rows.length) return "<p class=\"ml-doc-empty\">" + (l === "fr" ? "Aucune fonction ML enregistree pour cette page." : "No registered ML function for this page.") + "</p>";
    return "<div class=\"ml-doc-api-grid\">" + rows.map(function (d) {
      return "<article class=\"ml-doc-api-card\">" +
        "<h3>" + code(d.name) + "</h3>" +
        "<p>" + esc(textFor(d, l) || titleize(d.name)) + "</p>" +
        "<dl>" +
          "<dt>" + (l === "fr" ? "Alias" : "Aliases") + "</dt><dd>" + codeList(d.aliases) + "</dd>" +
          "<dt>" + (l === "fr" ? "Entrees" : "Inputs") + "</dt><dd>" + codeList(d.params) + "</dd>" +
          "<dt>" + (l === "fr" ? "Retour" : "Returns") + "</dt><dd>" + codeList(d.returns) + "</dd>" +
        "</dl>" +
      "</article>";
    }).join("") + "</div>";
  }

  function renderParamTable(rows, l) {
    var byParam = {};
    rows.forEach(function (d) {
      (d.params || []).forEach(function (p) {
        if (!byParam[p]) byParam[p] = [];
        byParam[p].push(d.name);
      });
    });
    var params = Object.keys(byParam).sort();
    if (!params.length) return "";
    var head = l === "fr"
      ? "<tr><th>Parametre</th><th>Type JSON</th><th>Fonctions</th><th>Role</th></tr>"
      : "<tr><th>Parameter</th><th>JSON type</th><th>Functions</th><th>Role</th></tr>";
    return "<section class=\"ml-doc-section\"><h2>" + (l === "fr" ? "Parametres" : "Parameters") + "</h2>" +
      "<table class=\"ml-doc-table\"><thead>" + head + "</thead><tbody>" + params.map(function (p) {
        return "<tr><td>" + code(p) + "</td><td><span class=\"ml-doc-type\">" + esc(paramType(p)) + "</span></td><td>" + codeList(byParam[p]) + "</td><td>" + esc(paramText(p, l)) + "</td></tr>";
      }).join("") + "</tbody></table></section>";
  }

  function renderExamples(rows, l) {
    if (!rows.length) return "";
    return "<section class=\"ml-doc-section\"><h2>" + (l === "fr" ? "Exemples" : "Examples") + "</h2>" +
      "<div class=\"ml-doc-examples\">" + rows.map(function (d) {
        return "<article class=\"ml-doc-example\">" +
          "<div class=\"ml-doc-example-head\"><span>" + esc(titleize(d.name)) + "</span>" + code(d.name) + "</div>" +
          "<pre><code class=\"language-python\">" + esc(exampleCode(d)) + "</code></pre>" +
        "</article>";
      }).join("") + "</div></section>";
  }

  function renderModels(models, l) {
    if (!models.length) return "";
    var head = l === "fr"
      ? "<tr><th>Modele</th><th>Categorie</th><th>Champs publics</th></tr>"
      : "<tr><th>Model</th><th>Category</th><th>Public fields</th></tr>";
    return "<section class=\"ml-doc-section\"><h2>" + (l === "fr" ? "Modeles" : "Models") + "</h2>" +
      "<table class=\"ml-doc-table ml-doc-model-table\"><thead>" + head + "</thead><tbody>" + models.map(function (m) {
        var fields = (m.fields || []).map(function (f) { return f.name + ": " + f.ty; });
        return "<tr><td>" + code(m.name) + "</td><td>" + esc(m.category || "-") + "</td><td>" + codeList(fields) + "</td></tr>";
      }).join("") + "</tbody></table></section>";
  }

  function renderDoc(el, l) {
    var rows = docsFor(el);
    var models = modelsFor(el);
    el.innerHTML = "<div class=\"ml-doc-shell\">" +
      renderHero(rows, models, l) +
      "<section class=\"ml-doc-section\"><h2>" + (l === "fr" ? "Reference API" : "API Reference") + "</h2>" + renderFunctionCards(rows, l) + "</section>" +
      renderParamTable(rows, l) +
      renderExamples(rows, l) +
      renderModels(models, l) +
    "</div>";
  }

  function renderCategoryIndex(el, l) {
    renderDoc(el, l);
  }

  function renderGlobalIndex(el, l) {
    var r = reg();
    var cats = {};
    (r.docs || []).forEach(function (d) {
      if (!cats[d.category]) cats[d.category] = { docs: [], models: [] };
      cats[d.category].docs.push(d);
    });
    (r.models || []).forEach(function (m) {
      if (m.domain && m.domain !== "ml") return;
      if (!cats[m.category]) cats[m.category] = { docs: [], models: [] };
      cats[m.category].models.push(m);
    });
    var names = Object.keys(cats).sort();
    el.innerHTML = "<div class=\"ml-doc-index-grid\">" + names.map(function (cat) {
      var group = cats[cat];
      var files = uniq(group.docs.map(function (d) { return d.file; }));
      var links = files.slice(0, 4).map(function (file) {
        var first = group.docs.filter(function (d) { return d.file === file; })[0];
        return "<a href=\"" + linkFor(file) + "\">" + esc(first ? titleize(first.name) : file.replace(/\.md$/, "")) + "</a>";
      }).join("");
      var desc = group.docs[0] ? textFor(group.docs[0], l) : "";
      return "<article class=\"ml-doc-index-card\">" +
        "<h2>" + esc(cat || (l === "fr" ? "Sans categorie" : "Uncategorized")) + "</h2>" +
        "<p>" + esc(desc) + "</p>" +
        "<div class=\"ml-doc-counts\"><span>" + group.docs.length + " " + (l === "fr" ? "fonctions" : "functions") + "</span><span>" + group.models.length + " " + (l === "fr" ? "modeles" : "models") + "</span></div>" +
        "<div class=\"ml-doc-links\">" + links + "</div>" +
      "</article>";
    }).join("") + "</div>";
  }

  function render(root) {
    (root || document).querySelectorAll("[data-sp-ml-doc]").forEach(function (el) {
      renderDoc(el, elemLang(el));
    });
    (root || document).querySelectorAll("[data-sp-ml-category]").forEach(function (el) {
      renderCategoryIndex(el, elemLang(el));
    });
    (root || document).querySelectorAll("[data-sp-ml-index]").forEach(function (el) {
      renderGlobalIndex(el, elemLang(el));
    });
  }

  document.addEventListener("DOMContentLoaded", function () {
    render(document);
  });
  window.SeraPlotRenderMlDocs = render;
})();
