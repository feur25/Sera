(function () {
  function isEnglish() {
    return (localStorage.getItem("seraplot_lang") || "en").toLowerCase() !== "fr";
  }

  function titleCase(slug) {
    return slug
      .split("_")
      .filter(Boolean)
      .map(function (w) { return w.charAt(0).toUpperCase() + w.slice(1); })
      .join(" ");
  }

  function hasTopLevelKey(kwargs, key) {
    var re = new RegExp("(^|,)\\s*" + key + "\\s*=");
    return re.test(kwargs);
  }

  function pushSnippet(family, kwargs) {
    if (hasTopLevelKey(kwargs, "series")) {
      return (
        "idx = list(range(len(series[0])))\n" +
        "chart.push_vector(idx, series)"
      );
    }
    if (hasTopLevelKey(kwargs, "x") && hasTopLevelKey(kwargs, "y")) {
      return (
        "idx = list(range(len(x)))\n" +
        "chart.push(idx, x, y)"
      );
    }
    if (hasTopLevelKey(kwargs, "values")) {
      return (
        "idx = list(range(len(values)))\n" +
        "chart.push(idx, values)"
      );
    }
    return "# see the " + family + " family's own data shape for push(...)";
  }

  function buildCode(family, variant, kwargs) {
    var title = titleCase(family) + " (" + variant + ") — SeraStudio";
    var variantLine = hasTopLevelKey(kwargs, "variant") ? "" : '    variant="' + variant + '",\n';
    return (
      "import seraplot as sp\n\n" +
      "chart = sp." + family + "(\n" +
      '    "' + title + '",\n' +
      "    " + kwargs + ",\n" +
      variantLine +
      ")\n\n" +
      'chart.record("session.spls")\n\n' +
      pushSnippet(family, kwargs) + "\n\n" +
      "chart.stop_record()\n" +
      'chart.export_video("session.mp4", "session.spls", format="mp4", fps=30)\n'
    );
  }

  var codeStoreCounter = 0;

  function ensureCodeScript(code) {
    var id = "sp-catalog-code-" + (codeStoreCounter += 1);
    var el = document.createElement("script");
    el.type = "text/plain";
    el.id = id;
    el.textContent = code;
    document.body.appendChild(el);
    return id;
  }

  function card(family, variant, kwargs, en) {
    var el = document.createElement("div");
    el.className = "sp-catalog-card";
    el.dataset.search = (family + " " + variant).toLowerCase();

    var fam = document.createElement("div");
    fam.className = "sp-catalog-card-family";
    fam.textContent = family;
    el.appendChild(fam);

    var title = document.createElement("div");
    title.className = "sp-catalog-card-title";
    title.textContent = titleCase(variant);
    el.appendChild(title);

    var btn = document.createElement("button");
    btn.className = "sp-code-btn";
    btn.textContent = en ? "View code" : "Voir le code";
    btn.dataset.spCodeTitle = family + "_" + variant + "_serastudio.py";
    btn.addEventListener("click", function () {
      if (!btn.dataset.spCodeTarget) {
        var code = buildCode(family, variant, kwargs);
        btn.dataset.spCodeTarget = ensureCodeScript(code);
        btn.setAttribute("data-sp-code-target", btn.dataset.spCodeTarget);
      }
    }, { once: false });
    el.appendChild(btn);

    return el;
  }

  function build(root) {
    var reg = window.SeraPlotDocRegistry;
    var params = (reg && reg.params) || {};
    var en = isEnglish();
    var keys = Object.keys(params).sort();

    root.innerHTML = "";

    var toolbar = document.createElement("div");
    toolbar.className = "sp-catalog-toolbar";
    var search = document.createElement("input");
    search.type = "search";
    search.className = "sp-catalog-search";
    search.placeholder = en
      ? "Search a chart family or variant…"
      : "Rechercher une famille ou un variant…";
    var count = document.createElement("span");
    count.className = "sp-catalog-count";
    toolbar.appendChild(search);
    toolbar.appendChild(count);
    root.appendChild(toolbar);

    var grid = document.createElement("div");
    grid.className = "sp-catalog-grid";
    root.appendChild(grid);

    var empty = document.createElement("div");
    empty.className = "sp-catalog-empty";
    empty.textContent = en ? "No matching chart." : "Aucun chart correspondant.";
    empty.style.display = "none";
    root.appendChild(empty);

    var cards = keys.map(function (key) {
      var sep = key.indexOf(":");
      var family = key.slice(0, sep);
      var variant = key.slice(sep + 1);
      var el = card(family, variant, params[key], en);
      grid.appendChild(el);
      return el;
    });

    function refreshCount(visible) {
      count.textContent = visible + "/" + cards.length + (en ? " charts" : " charts");
    }
    refreshCount(cards.length);

    search.addEventListener("input", function () {
      var q = search.value.trim().toLowerCase();
      var visible = 0;
      cards.forEach(function (el) {
        var match = !q || el.dataset.search.indexOf(q) !== -1;
        el.style.display = match ? "" : "none";
        if (match) visible += 1;
      });
      empty.style.display = visible === 0 ? "" : "none";
      grid.style.display = visible === 0 ? "none" : "";
      refreshCount(visible);
    });
  }

  function init() {
    var enRoot = document.getElementById("sp-serastudio-catalog-en");
    var frRoot = document.getElementById("sp-serastudio-catalog-fr");
    if (!enRoot && !frRoot) return;
    var tries = 0;
    var wait = setInterval(function () {
      tries += 1;
      if (window.SeraPlotDocRegistry || tries > 40) {
        clearInterval(wait);
        if (enRoot) build(enRoot);
        if (frRoot) build(frRoot);
      }
    }, 50);
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }
})();
