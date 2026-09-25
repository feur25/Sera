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

  function normalizedType(ty) {
    return String(ty || "").replace(/\s*\|\s*none\s*$/i, "").trim().toLowerCase();
  }

  function sampleCandidates(ty, name, n) {
    var t = normalizedType(ty);
    var nm = String(name || "").toLowerCase();
    n = n || 3;
    if (t === "bool") return [true, false];
    if (t === "int" || t === "i64" || t === "u64") {
      if (/idx|index/.test(nm)) return [0, 1];
      return [137, 253];
    }
    if (t === "float") {
      if (/opacity|frac|ratio/.test(nm)) return [0.72, 0.28];
      return [2.75, 0.35];
    }
    if (t === "str") {
      if (/color/.test(nm)) return ["#6366f1", "#ef4444"];
      if (/kind|shape|position|align|mode/.test(nm)) return ["rect", "circle"];
      return ["Sample", "Alt"];
    }
    if (t.indexOf("list[str]") === 0) {
      return [
        Array.from({ length: n }, function (_, i) { return "S" + i; }),
        Array.from({ length: n }, function (_, i) { return "T" + (n - i); }),
      ];
    }
    if (t.indexOf("list[float]") === 0 || t.indexOf("list[int]") === 0) {
      return [
        Array.from({ length: n }, function (_, i) { return i + 1; }),
        Array.from({ length: n }, function (_, i) { return (n - i) * 1.5; }),
      ];
    }
    if (t.indexOf("list[") === 0) return [[]];
    if (t.indexOf("dict") === 0) return [{}];
    return [null];
  }

  function syntheticArgVariants(methodName, n) {
    var reg = window.SeraPlotMethodRegistry;
    var docs = (reg && reg.docs) || [];
    var doc = docs.filter(function (d) { return d.name === methodName; })[0];
    var params = (doc && doc.params) || [];
    var variants = [{}, {}];
    params.forEach(function (p) {
      var candidates = sampleCandidates(p.ty, p.name, n);
      if (candidates[0] !== null) variants[0][p.name] = candidates[0];
      var alt = candidates.length > 1 ? candidates[1] : candidates[0];
      if (alt !== null) variants[1][p.name] = alt;
    });
    return variants;
  }

  var dispatchableCache = null;

  function dispatchableSet() {
    if (dispatchableCache) return dispatchableCache;
    var sp = window.SeraplotWASM;
    if (!sp || !sp.__ready) return {};
    var result = {};
    try { JSON.parse(sp.chartMethods()).forEach(function (n) { result[n] = true; }); } catch (e) {}
    dispatchableCache = result;
    return result;
  }

  function isDispatchable(methodName) {
    return !!dispatchableSet()[methodName];
  }

  function whenWasmReady(cb, triesLeft) {
    var sp = window.SeraplotWASM;
    if (sp && sp.__ready) { cb(); return; }
    if (triesLeft <= 0) { cb(); return; }
    setTimeout(function () { whenWasmReady(cb, triesLeft - 1); }, 150);
  }

  function chartMethodNames() {
    var reg = window.SeraPlotMethodRegistry;
    var docs = (reg && reg.docs) || [];
    var dispatchable = dispatchableSet();
    return docs.filter(function (d) { return d.category === "chart_method" && dispatchable[d.name]; }).map(function (d) { return d.name; });
  }

  function nonTrivialSelectors(js) {
    var out = [];
    var re = /querySelectorAll?\(\s*(['"])((?:(?!\1)[\s\S])*)\1\s*\)/g;
    var m;
    while ((m = re.exec(js))) {
      var sel = m[2];
      if (sel && sel !== "svg" && sel !== "canvas") out.push(sel);
    }
    return out;
  }

  function injectedSuffix(baseline, after) {
    var minLen = Math.min(baseline.length, after.length);
    var i = 0;
    while (i < minLen && baseline[i] === after[i]) i++;
    var baselineTail = baseline.slice(i);
    if (!baselineTail) return after.slice(i);
    if (after.length - baselineTail.length >= i && after.slice(after.length - baselineTail.length) === baselineTail) {
      return after.slice(i, after.length - baselineTail.length);
    }
    return null;
  }

  function isGenuineEffect(baselineDoc, baseline, after) {
    if (!after || after === baseline) return false;
    var injected = injectedSuffix(baseline, after);
    if (injected === null || !baselineDoc) return true;
    var selectors = nonTrivialSelectors(injected);
    for (var i = 0; i < selectors.length; i++) {
      try {
        if (baselineDoc.querySelectorAll(selectors[i]).length === 0) return false;
      } catch (e) {}
    }
    return true;
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
    if (STATE.running || STATE.map || STATE.pending) return;
    STATE.pending = true;
    whenWasmReady(function () {
      STATE.pending = false;
      runComputationReady();
    }, 120);
  }

  function runComputationReady() {
    if (STATE.running || STATE.map) return;
    var sp = window.SeraplotWASM;
    var parse = window.SeraPlotParseDemoInput;
    if (!sp || !sp.__ready || !parse) { STATE.map = {}; notify(); return; }

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
          var baselineDoc = null;
          try { baselineDoc = new DOMParser().parseFromString(baseline, "text/html"); } catch (e3) {}
          for (var mi = 0; mi < methods.length; mi++) {
            var method = methods[mi];
            var argVariants = syntheticArgVariants(method, n);
            var found = false;
            for (var ai = 0; ai < argVariants.length && !found; ai++) {
              var after;
              try { after = sp.applyChartMethod(baseline, method, JSON.stringify(argVariants[ai])); } catch (e2) { after = baseline; }
              if (isGenuineEffect(baselineDoc, baseline, after)) found = true;
            }
            if (found) map[method].push({ family: family, variant: variant });
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
      var busy = (STATE.running || STATE.pending) && !STATE.map;
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

  function pruneAndWire(root) {
    whenWasmReady(function () {
      var dispatchable = dispatchableSet();
      (root || document).querySelectorAll(".cm-fx-badge").forEach(function (btn) {
        if (!dispatchable[btn.getAttribute("data-method")]) btn.remove();
      });
      wireAll(root);
    }, 120);
  }

  STATE.listeners.push(refreshBadges);

  document.addEventListener("DOMContentLoaded", function () {
    pruneAndWire(document);
  });

  window.SeraPlotMethodEffects = { badge: badge, wireAll: wireAll, isDispatchable: isDispatchable };
})();
