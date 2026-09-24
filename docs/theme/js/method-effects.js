(function () {
  var STATE = { map: null, running: false, done: 0, total: 0, listeners: [] };

  function variantKeysOf(raw) {
    if (!raw) return [];
    if (Array.isArray(raw)) {
      return raw.map(function (v) { return typeof v === "string" ? v : (v.key || v.name || ""); }).filter(Boolean);
    }
    if (raw.variants) return variantKeysOf(raw.variants);
    return [];
  }

  function inputCardinality(input) {
    var max = 0;
    Object.keys(input || {}).forEach(function (k) {
      if (Array.isArray(input[k])) max = Math.max(max, input[k].length);
    });
    return max || 3;
  }

  function sampleForType(ty, name, n) {
    ty = String(ty || "").toLowerCase();
    var nm = String(name || "").toLowerCase();
    n = n || 3;
    if (ty === "bool") return true;
    if (ty === "int") return /idx|index/.test(nm) ? 0 : 5;
    if (ty === "float") return /opacity|frac|ratio/.test(nm) ? 0.5 : 1.0;
    if (ty === "str") {
      if (/color/.test(nm)) return "#6366f1";
      if (/kind|shape|position|align|mode/.test(nm)) return "rect";
      return "Sample";
    }
    if (ty.indexOf("list[str]") === 0) return Array.from({ length: n }, function (_, i) { return "S" + i; });
    if (ty.indexOf("list[float]") === 0 || ty.indexOf("list[int]") === 0) return Array.from({ length: n }, function (_, i) { return i + 1; });
    if (ty.indexOf("list[") === 0) return [];
    if (ty.indexOf("dict") === 0) return {};
    return null;
  }

  function syntheticArgsFor(methodName, n) {
    var reg = window.SeraPlotMethodRegistry;
    var docs = (reg && reg.docs) || [];
    var doc = docs.filter(function (d) { return d.name === methodName; })[0];
    var args = {};
    (doc && doc.params || []).forEach(function (p) {
      var v = sampleForType(p.ty, p.name, n);
      if (v !== null) args[p.name] = v;
    });
    return args;
  }

  function chartMethodNames() {
    var reg = window.SeraPlotMethodRegistry;
    var docs = (reg && reg.docs) || [];
    return docs.filter(function (d) { return d.category === "chart_method"; }).map(function (d) { return d.name; });
  }

  function notify() {
    STATE.listeners.forEach(function (fn) { fn(STATE); });
  }

  function allTargets(families) {
    var out = [];
    Object.keys(families || {}).forEach(function (family) {
      var variants = variantKeysOf(families[family]);
      if (!variants.length) variants = ["basic"];
      variants.forEach(function (variant) { out.push({ family: family, variant: variant }); });
    });
    return out;
  }

  function runComputation() {
    if (STATE.running || STATE.map) return;
    var sp = window.SeraplotWASM;
    var parse = window.SeraPlotParseDemoInput;
    if (!sp || !parse) { STATE.map = {}; notify(); return; }

    var families = {};
    try { families = JSON.parse(sp.chartVariants()) || {}; } catch (e) {}
    var methods = chartMethodNames();
    var map = {};
    methods.forEach(function (m) { map[m] = []; });

    var targets = allTargets(families);
    STATE.running = true;
    STATE.done = 0;
    STATE.total = targets.length;
    var idx = 0;

    function step() {
      var t0 = performance.now();
      while (idx < targets.length && performance.now() - t0 < 40) {
        var target = targets[idx++];
        STATE.done = idx;
        var family = target.family, variant = target.variant;
        try {
          var snippet = sp.demo(JSON.stringify({ family: family, variant: variant }));
          if (!snippet) continue;
          var parsed = parse(snippet);
          if (!parsed || !parsed.input) continue;
          var baseline = sp.call(family, JSON.stringify(parsed.input));
          if (!baseline) continue;
          var n = inputCardinality(parsed.input);
          for (var mi = 0; mi < methods.length; mi++) {
            var method = methods[mi];
            var args = JSON.stringify(syntheticArgsFor(method, n));
            var after;
            try { after = sp.applyChartMethod(baseline, method, args); } catch (e2) { after = baseline; }
            if (after && after !== baseline) map[method].push({ family: family, variant: variant });
          }
        } catch (e) {}
      }
      notify();
      if (idx < targets.length) {
        setTimeout(step, 0);
      } else {
        STATE.running = false;
        STATE.map = map;
        notify();
      }
    }
    step();
  }

  function closeModal() {
    var m = document.getElementById("sp-fx-modal");
    if (m) m.remove();
    document.removeEventListener("keydown", onKeyClose);
  }

  function onKeyClose(e) {
    if (e.key === "Escape") closeModal();
  }

  function renderModalList(entries, isEn) {
    var byFamily = {};
    entries.forEach(function (e) { (byFamily[e.family] = byFamily[e.family] || []).push(e.variant); });
    var families = Object.keys(byFamily).sort();
    if (!families.length) {
      return '<p class="sp-fx-empty">' + (isEn ? "No measurable effect found on any chart's default render." : "Aucun effet mesurable trouvé sur le rendu par défaut d'un graphique.") + "</p>";
    }
    return families.map(function (f) {
      return '<div class="sp-fx-row"><code class="sp-fx-fam">' + f + "</code>"
        + '<span class="sp-fx-variants">' + byFamily[f].map(function (v) { return "<code>" + v + "</code>"; }).join(" ") + "</span></div>";
    }).join("");
  }

  function openModal(methodName, isEn) {
    closeModal();
    var backdrop = document.createElement("div");
    backdrop.id = "sp-fx-modal";
    backdrop.className = "sp-fx-backdrop";
    backdrop.addEventListener("click", function (e) { if (e.target === backdrop) closeModal(); });

    function paint() {
      var entries = (STATE.map && STATE.map[methodName]) || [];
      var busy = STATE.running && !STATE.map;
      var count = STATE.map ? entries.length : "…";
      backdrop.innerHTML =
        '<div class="sp-fx-modal" role="dialog" aria-modal="true" aria-label="' + methodName + '">'
        + '<div class="sp-fx-head"><code>' + methodName + "</code>"
        + '<span class="sp-fx-count">' + count + (isEn ? " chart types" : " types de graphique") + "</span>"
        + '<button type="button" class="sp-fx-close" aria-label="Close">×</button></div>'
        + '<div class="sp-fx-sub">' + (isEn
            ? "Measured empirically: each chart's own demo is built, this method applied, and only the charts whose rendered output actually changed are kept."
            : "Mesuré empiriquement : chaque graphique est construit depuis sa propre démo, la méthode appliquée, seuls ceux dont le rendu change réellement sont gardés.")
        + "</div>"
        + (busy
            ? '<div class="sp-fx-progress">' + (isEn ? "Measuring against every chart type… " : "Mesure sur chaque type de graphique… ") + STATE.done + "/" + STATE.total + "</div>"
            : "")
        + '<div class="sp-fx-list">' + renderModalList(entries, isEn) + "</div>"
        + "</div>";
      backdrop.querySelector(".sp-fx-close").addEventListener("click", closeModal);
    }
    paint();
    document.body.appendChild(backdrop);
    document.addEventListener("keydown", onKeyClose);
    var listener = function () { if (document.getElementById("sp-fx-modal") === backdrop) paint(); };
    STATE.listeners.push(listener);
    var origRemove = backdrop.remove.bind(backdrop);
    backdrop.remove = function () {
      STATE.listeners = STATE.listeners.filter(function (l) { return l !== listener; });
      origRemove();
    };
    runComputation();
  }

  function badgeId(methodName) {
    return "sp-fx-btn-" + methodName.replace(/[^a-zA-Z0-9_]/g, "_");
  }

  function refreshBadges() {
    document.querySelectorAll(".cm-fx-badge").forEach(function (btn) {
      var name = btn.getAttribute("data-method");
      if (STATE.map) {
        var n = (STATE.map[name] || []).length;
        btn.textContent = n + (n === 1 ? " type" : " types");
        btn.classList.toggle("cm-fx-badge-empty", n === 0);
        btn.disabled = false;
      } else if (STATE.running) {
        btn.textContent = "…";
      }
    });
  }

  function wireAll(root) {
    (root || document).querySelectorAll(".cm-fx-badge").forEach(function (btn) {
      if (btn.getAttribute("data-fx-wired")) return;
      btn.setAttribute("data-fx-wired", "1");
      var name = btn.getAttribute("data-method");
      btn.addEventListener("click", function (e) {
        e.stopPropagation();
        var isEn = !btn.closest(".lang-fr");
        openModal(name, isEn);
      });
    });
  }

  function badge(methodName) {
    return '<button type="button" class="cm-fx-badge" id="' + badgeId(methodName) + '" data-method="' + methodName + '">effects</button>';
  }

  STATE.listeners.push(refreshBadges);

  document.addEventListener("DOMContentLoaded", function () {
    wireAll(document);
  });

  window.SeraPlotMethodEffects = { badge: badge, wireAll: wireAll };
})();
