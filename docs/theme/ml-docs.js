(function () {
  function esc(s) {
    return String(s == null ? "" : s)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;");
  }

  function codeList(xs) {
    xs = (xs || []).filter(Boolean);
    return xs.length ? xs.map(function (x) { return "<code>" + esc(x) + "</code>"; }).join(", ") : "-";
  }

  function reg() {
    return window.SeraPlotMlRegistry || { docs: [], models: [] };
  }

  function lang() {
    return localStorage.getItem("seraplot_lang") || "en";
  }

  function docsFor(el) {
    var r = reg();
    var file = el.getAttribute("data-file") || "";
    var category = el.getAttribute("data-category") || "";
    var names = (el.getAttribute("data-names") || "").split(",").map(function (s) { return s.trim(); }).filter(Boolean);
    return (r.docs || []).filter(function (d) {
      if (file && d.file !== file) return false;
      if (category && d.category !== category) return false;
      if (names.length && names.indexOf(d.name) === -1) return false;
      return true;
    });
  }

  function modelsFor(el) {
    var r = reg();
    var category = el.getAttribute("data-category") || "";
    var names = (el.getAttribute("data-models") || "").split(",").map(function (s) { return s.trim(); }).filter(Boolean);
    return (r.models || []).filter(function (m) {
      if (m.domain && m.domain !== "ml") return false;
      if (category && m.category !== category) return false;
      if (names.length && names.indexOf(m.name) === -1) return false;
      return true;
    });
  }

  function renderDocsTable(rows, l) {
    if (!rows.length) return "<p>" + (l === "fr" ? "Aucune fonction ML enregistrée pour cette page." : "No registered ML function for this page.") + "</p>";
    var head = l === "fr"
      ? "<tr><th>Fonction JSON</th><th>Catégorie</th><th>Alias</th><th>Entrées</th><th>Retour</th></tr>"
      : "<tr><th>JSON function</th><th>Category</th><th>Aliases</th><th>Inputs</th><th>Returns</th></tr>";
    return "<table><thead>" + head + "</thead><tbody>" + rows.map(function (d) {
      return "<tr><td><code>" + esc(d.name) + "</code></td><td>" + esc(d.category || "-") + "</td><td>" + codeList(d.aliases) + "</td><td>" + codeList(d.params) + "</td><td>" + codeList(d.returns) + "</td></tr>";
    }).join("") + "</tbody></table>";
  }

  function renderModelsTable(rows, l) {
    if (!rows.length) return "<p>" + (l === "fr" ? "Aucun modèle décoré pour cette page." : "No decorated model for this page.") + "</p>";
    var head = l === "fr"
      ? "<tr><th>Modèle</th><th>Catégorie</th><th>Champs publics</th></tr>"
      : "<tr><th>Model</th><th>Category</th><th>Public fields</th></tr>";
    return "<table><thead>" + head + "</thead><tbody>" + rows.map(function (m) {
      var fields = (m.fields || []).map(function (f) { return f.name + ": " + f.ty; });
      return "<tr><td><code>" + esc(m.name) + "</code></td><td>" + esc(m.category || "-") + "</td><td>" + codeList(fields) + "</td></tr>";
    }).join("") + "</tbody></table>";
  }

  function renderCategoryIndex(el, l) {
    var docs = docsFor(el);
    var models = modelsFor(el);
    el.innerHTML =
      "<h2>" + (l === "fr" ? "Fonctions JSON" : "JSON Functions") + "</h2>" +
      renderDocsTable(docs, l) +
      "<h2>" + (l === "fr" ? "Modèles décorés" : "Decorated Models") + "</h2>" +
      renderModelsTable(models, l);
  }

  function renderGlobalIndex(el, l) {
    var r = reg();
    var cats = {};
    (r.docs || []).forEach(function (d) { if (d.category) cats[d.category] = true; });
    (r.models || []).forEach(function (m) { if (m.category) cats[m.category] = true; });
    var names = Object.keys(cats).sort();
    var head = l === "fr"
      ? "<tr><th>Catégorie</th><th>Fonctions JSON</th><th>Modèles décorés</th></tr>"
      : "<tr><th>Category</th><th>JSON functions</th><th>Decorated models</th></tr>";
    el.innerHTML = "<table><thead>" + head + "</thead><tbody>" + names.map(function (c) {
      var dc = (r.docs || []).filter(function (d) { return d.category === c; }).length;
      var mc = (r.models || []).filter(function (m) { return m.category === c; }).length;
      return "<tr><td>" + esc(c) + "</td><td>" + dc + "</td><td>" + mc + "</td></tr>";
    }).join("") + "</tbody></table>";
  }

  function render(root) {
    var l = lang();
    (root || document).querySelectorAll("[data-sp-ml-doc]").forEach(function (el) {
      el.innerHTML = renderDocsTable(docsFor(el), l) + "<h2>" + (l === "fr" ? "Modèles décorés" : "Decorated Models") + "</h2>" + renderModelsTable(modelsFor(el), l);
    });
    (root || document).querySelectorAll("[data-sp-ml-category]").forEach(function (el) {
      renderCategoryIndex(el, l);
    });
    (root || document).querySelectorAll("[data-sp-ml-index]").forEach(function (el) {
      renderGlobalIndex(el, l);
    });
  }

  document.addEventListener("DOMContentLoaded", function () {
    render(document);
  });
  window.SeraPlotRenderMlDocs = render;
})();
