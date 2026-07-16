window.SP_WASM_BUILD = window.SP_WASM_BUILD || "20260722";
(function () {
  var SP_WASM_BUILD = window.SP_WASM_BUILD;
  var POS_KEY    = "sp_params_pos";
  var COL_KEY    = "sp_params_col";
  var H_KEY      = "sp_params_h";
  var W_KEY      = "sp_params_w";
  var LANG_KEY   = "seraplot_lang";
  var PANEL_ID   = "sp-params-panel";

  var state = {
    pos: localStorage.getItem(POS_KEY) || "right",
    collapsed: localStorage.getItem(COL_KEY) === "1",
    bottomH: parseInt(localStorage.getItem(H_KEY) || "300", 10),
    rightW:  parseInt(localStorage.getItem(W_KEY) || "360", 10)
  };

  var sectionData = { en: null, fr: null };
  // Code-example tabs HTML keyed by lang, and the iframe src.
  var exampleData = { en: "", fr: "", iframeSrc: "" };
  // Body-appended rail position tracking
  var lastRailResizeHandler = null;
  var positionRailFn = null;

  function getLang() { return localStorage.getItem(LANG_KEY) || "en"; }

  function findH2(container, texts) {
    var found = null;
    container.querySelectorAll("h2").forEach(function (h) {
      var t = h.textContent.trim().toLowerCase();
      if (!found && texts.indexOf(t) !== -1) found = h;
    });
    return found;
  }

  function isAliasNode(el) {
    if (!el || !el.textContent) return false;
    var t = el.textContent.trim();
    return /^alias(es|)\s*[::]/i.test(t);
  }

  function isHrNode(el) { return el && el.tagName === "HR"; }

  function isExampleBoundary(el) {
    if (!el) return false;
    var tag = el.tagName;
    if (tag === "STYLE" || tag === "SCRIPT" || tag === "IFRAME") return true;
    if (tag === "DIV" && el.classList.contains("sp-tabs")) return true;
    return false;
  }

  function hasMoved(el, container) {
    var p = el.parentElement;
    while (p && p !== container) {
      if (p.classList.contains("sp-moved")) return true;
      p = p.parentElement;
    }
    return false;
  }

  // Collect the sp-tabs HTML for a given lang container and hide the original.
  function collectTabs(container) {
    if (!container) return "";
    var tabs = null;
    container.querySelectorAll(".sp-tabs").forEach(function (el) {
      if (tabs) return;
      if (!hasMoved(el, container)) tabs = el;
    });
    if (!tabs) return "";
    var html = tabs.outerHTML;
    tabs.classList.add("sp-moved");
    return html;
  }

  // Collect the iframe src for a given lang container and hide the original.
  function collectIframeSrc(container) {
    if (!container) return "";
    var iframe = null;
    container.querySelectorAll('iframe[loading="lazy"]').forEach(function (el) {
      if (iframe) return;
      if (!hasMoved(el, container)) iframe = el;
    });
    if (!iframe) return "";
    var src = iframe.getAttribute("src");
    iframe.classList.add("sp-moved");
    return src;
  }

  function remapPanelHtml(html) {
    return html
      .replace(/\bid="([^"]+)"/g, function (m, id) { return 'id="pp-' + id + '"'; })
      .replace(/spVar\('([^']+)'/g, function (m, scope) { return "spVar('pp-" + scope + "'"; })
      .replace(/spTab\('([^']+)','([^']+)',this\)/g, function (m, g, id) {
        return "spTab('pp-" + g + "','pp-" + id + "',this)";
      })
      .replace(/spCls\('([^']+)','([^']+)',this\)/g, function (m, scope, name) {
        return "spCls('pp-" + scope + "','" + name + "',this)";
      })
      .replace(/spClsTog\('([^']+)'\)/g, function (m, id) {
        return "spClsTog('pp-" + id + "')";
      });
  }

  function extractAndHide(h2El, splitAlias, hide) {
    if (!h2El) return null;
    if (hide !== false) h2El.classList.add("sp-moved");
    var sigHtml = "";
    var aliasHtml = "";
    var sib = h2El.nextElementSibling;
    while (sib && sib.tagName !== "H2") {
      if (isExampleBoundary(sib)) break;
      var next = sib.nextElementSibling;
      var html = sib.outerHTML;
      if (hide !== false) sib.classList.add("sp-moved");
      if (splitAlias && isAliasNode(sib)) {
        aliasHtml += html;
      } else if (!isHrNode(sib)) {
        sigHtml += html;
      }
      sib = next;
    }
    return { main: sigHtml, alias: aliasHtml };
  }

  function collectFrom(container) {
    if (!container) return null;
    var sigH2 = findH2(container, ["signature"]);
    var parH2 = findH2(container, ["parameters", "param\u00e8tres", "parametres"]);
    var retH2 = findH2(container, ["returns", "retour", "retours", "retourne"]);
    if (!parH2) return null;
    var parSource = !!(parH2.closest && parH2.closest("[data-sp-panel-source], .sp-panel-source"));

    var sig    = extractAndHide(sigH2, true, true);
    var params = extractAndHide(parH2, false, false);
    var rets   = extractAndHide(retH2, false, true);

    return {
      signature:  sig    ? sig.main   : "",
      functionName: signatureFunctionName(sig ? sig.main : ""),
      alias:      sig    ? sig.alias  : "",
      parameters: params ? params.main : "",
      parametersSource: parSource,
      returns:    rets   ? rets.main   : ""
    };
  }

  function escapeAttr(s) {
    return String(s || "")
      .replace(/&/g, "&amp;")
      .replace(/"/g, "&quot;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;");
  }

  function signatureFunctionName(html) {
    if (!html) return "";
    var div = document.createElement("div");
    div.innerHTML = html;
    var text = div.textContent || "";
    var m = text.match(/\bsp\.([A-Za-z_][A-Za-z0-9_]*)\s*\(/);
    return m ? m[1] : "";
  }

  function autoAliasHtml(fn) {
    return fn ? '<p class="sp-auto-alias" data-sp-alias-for="' + escapeAttr(fn) + '"></p>' : "";
  }

  function aliasMap() {
    var sp = window.SeraplotWASM;
    if (!sp || typeof sp.chartAliases !== "function") return null;
    try {
      var pairs = JSON.parse(sp.chartAliases());
      var map = {};
      for (var i = 0; i < pairs.length; i++) map[pairs[i][0]] = pairs[i][1];
      return map;
    } catch (e) { return null; }
  }

  function aliasesFor(fn, map) {
    if (!fn || !map) return [];
    var target = map[fn] || null;
    if (!target && fn.slice(-1) === "s") target = map[fn.slice(0, -1)] || null;
    if (!target) return map[fn] ? [fn] : [];
    var out = [];
    Object.keys(map).forEach(function (alias) {
      if (map[alias] === target) out.push(alias);
    });
    return out;
  }

  function normalizeVariantName(name) {
    return String(name || "").trim().replace(/-/g, "_");
  }

  function variantDomId(scope, key) {
    return scope + "-v-" + normalizeVariantName(key).replace(/[^A-Za-z0-9_]/g, "_");
  }

  function variantLabel(item) {
    if (!item) return "";
    return item.key || "";
  }

  function aliasesForItem(item) {
    var key = item && item.key ? item.key : "";
    var aliases = item && Array.isArray(item.aliases) ? item.aliases.slice() : [];
    var out = [];
    [key].concat(aliases).forEach(function (a) {
      a = String(a || "").trim();
      if (a && out.indexOf(a) === -1) out.push(a);
    });
    return out;
  }

  function buildVariantAliasesHtml(item, lang) {
    var aliases = aliasesForItem(item);
    if (!aliases.length) return "";
    return '<div class="sp-vmeta"><span>' + (lang === "fr" ? "Alias" : "Aliases") + '</span>' +
      aliases.map(function (a) { return '<code>' + escapeAttr(a) + '</code>'; }).join("") +
      '</div>';
  }

  function hydrateAliases(root) {
    var nodes = Array.prototype.slice.call((root || document).querySelectorAll(".sp-auto-alias"));
    if (!nodes.length) return;
    nodes.forEach(function (node) {
      var sec = node.closest ? node.closest(".sp-psec-alias") : null;
      if (sec) sec.style.display = "none";
    });
    var tries = 0;
    function paint() {
      var map = aliasMap();
      if (!map) return false;
      nodes.forEach(function (node) {
        var fn = node.getAttribute("data-sp-alias-for") || "";
        var aliases = aliasesFor(fn, map);
        var sec = node.closest ? node.closest(".sp-psec-alias") : null;
        if (!aliases.length) {
          if (sec) sec.style.display = "none";
          return;
        }
        node.innerHTML = aliases.map(function (alias) {
          return '<code>sp.' + escapeAttr(alias) + '</code>';
        }).join(", ");
        if (sec) sec.style.display = "";
      });
      return true;
    }
    if (paint()) return;
    var timer = setInterval(function () {
      tries += 1;
      if (paint() || tries > 120) clearInterval(timer);
    }, 80);
  }

  function isChartPage() {
    if (document.querySelector("[data-sp-ml-doc], [data-sp-ml-category], [data-sp-ml-index]")) return false;
    var en = document.querySelector(".lang-en");
    var fr = document.querySelector(".lang-fr");
    var test = en || fr;
    return !!(test && findH2(test, ["parameters", "param\u00e8tres"]));
  }

  // \u2500\u2500 Dynamic variant injection \u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500
  // Extracts unique variant names from the ## Parameters table HTML.
  // "all" / "toutes" rows are skipped \u2014 only explicit variant names count.
  var VARIANT_ICONS = {
    "basic":           '<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><rect x="2" y="5" width="3" height="9" rx=".5"/><rect x="6.5" y="3" width="3" height="11" rx=".5"/><rect x="11" y="7" width="3" height="7" rx=".5"/></svg>',
    "grouped":         '<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><rect x="1" y="7" width="2.5" height="7" rx=".5"/><rect x="4" y="4" width="2.5" height="10" rx=".5"/><rect x="9" y="7" width="2.5" height="7" rx=".5"/><rect x="12" y="4" width="2.5" height="10" rx=".5"/></svg>',
    "grouped_stacked": '<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><rect x="1" y="8" width="2" height="6" rx=".5"/><rect x="1" y="5" width="2" height="3" rx=".5" opacity=".5"/><rect x="4" y="6" width="2" height="8" rx=".5"/><rect x="4" y="3" width="2" height="3" rx=".5" opacity=".5"/><rect x="9" y="9" width="2" height="5" rx=".5"/><rect x="9" y="6" width="2" height="3" rx=".5" opacity=".5"/><rect x="12" y="5" width="2" height="9" rx=".5"/><rect x="12" y="2" width="2" height="3" rx=".5" opacity=".5"/></svg>',
    "stacked":         '<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><rect x="2" y="8" width="4" height="6" rx=".5"/><rect x="2" y="4" width="4" height="4" rx=".5" opacity=".5"/><rect x="2" y="2" width="4" height="2" rx=".5" opacity=".25"/><rect x="10" y="6" width="4" height="8" rx=".5"/><rect x="10" y="3" width="4" height="3" rx=".5" opacity=".5"/></svg>',
    "relative":        '<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><rect x="2" y="2" width="4" height="12" rx=".5" opacity=".2"/><rect x="2" y="6" width="4" height="8" rx=".5" opacity=".38"/><rect x="2" y="10" width="4" height="4" rx=".5"/><rect x="10" y="2" width="4" height="12" rx=".5" opacity=".2"/><rect x="10" y="5" width="4" height="9" rx=".5" opacity=".38"/><rect x="10" y="9" width="4" height="5" rx=".5"/></svg>',
    "marimekko":       '<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><rect x="1" y="3" width="5" height="11" rx=".5"/><rect x="1" y="3" width="5" height="6" rx=".5" opacity=".38"/><rect x="7" y="2" width="8" height="12" rx=".5"/><rect x="7" y="2" width="8" height="7" rx=".5" opacity=".32"/></svg>',
    "multicategory":   '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.2"><rect x="1" y="1" width="6" height="6" rx=".5"/><rect x="9" y="1" width="6" height="6" rx=".5"/><rect x="1" y="9" width="6" height="6" rx=".5"/><rect x="9" y="9" width="6" height="6" rx=".5"/></svg>',
    "pictogram":       '<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><circle cx="3" cy="3" r="1.3"/><circle cx="7" cy="3" r="1.3"/><circle cx="11" cy="3" r="1.3"/><circle cx="3" cy="7" r="1.3"/><circle cx="7" cy="7" r="1.3"/><circle cx="11" cy="7" r="1.3"/><circle cx="3" cy="11" r="1.3"/><circle cx="7" cy="11" r=".8" opacity=".4"/><circle cx="11" cy="11" r=".4" opacity=".2"/></svg>',
    "regression":      '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor"><circle cx="3" cy="12" r="1.5" fill="currentColor" stroke="none"/><circle cx="7" cy="8" r="1.5" fill="currentColor" stroke="none"/><circle cx="11" cy="5" r="1.5" fill="currentColor" stroke="none"/><circle cx="13" cy="10" r="1.5" fill="currentColor" stroke="none"/><line x1="2" y1="13.5" x2="14" y2="3.5" stroke-width="1.5"/></svg>',
    "categorical":     '<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><circle cx="4" cy="11" r="2.2"/><circle cx="8" cy="6" r="2.2" opacity=".6"/><circle cx="12" cy="9" r="2.2" opacity=".35"/><circle cx="5" cy="4" r="1.5" opacity=".8"/></svg>',
    "gradient":        '<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><circle cx="4" cy="11" r="2" opacity=".2"/><circle cx="8" cy="7" r="2.5" opacity=".55"/><circle cx="12" cy="4" r="2" opacity=".9"/></svg>',
    "labeled":         '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.2"><circle cx="5" cy="11" r="2" fill="currentColor" stroke="none"/><circle cx="11" cy="5" r="2" fill="currentColor" stroke="none"/><rect x="7" y="3" width="6" height="3" rx="1"/><rect x="1" y="9" width="6" height="3" rx="1"/></svg>',
    "nova":            '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="8" cy="8" r="2" fill="currentColor"/><line x1="8" y1="1" x2="8" y2="4.5"/><line x1="8" y1="11.5" x2="8" y2="15"/><line x1="1" y1="8" x2="4.5" y2="8"/><line x1="11.5" y1="8" x2="15" y2="8"/><line x1="3.2" y1="3.2" x2="5.5" y2="5.5"/><line x1="10.5" y1="10.5" x2="12.8" y2="12.8"/><line x1="12.8" y1="3.2" x2="10.5" y2="5.5"/><line x1="5.5" y1="10.5" x2="3.2" y2="12.8"/></svg>',
    "galaxy":          '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.2"><path d="M8 8 Q12 3 14 8 Q12 13 8 8 Q4 13 2 8 Q4 3 8 8"/><circle cx="8" cy="8" r="1.5" fill="currentColor" stroke="none"/></svg>',
    "symbols":         '<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><polygon points="4,1.5 5.8,5.5 2,5.5"/><rect x="9.5" y="1.5" width="4" height="4" rx=".5"/><circle cx="4" cy="12" r="2.5"/><polygon points="11.5,9.5 14.5,14 8.5,14"/></svg>',
    "line":            '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2"><polyline points="1.5,12 5,7 9,9 14.5,3"/></svg>',
    "area":            '<svg width="16" height="16" viewBox="0 0 16 16"><path d="M1.5 12 L5 7 L9 9 L14.5 3 L14.5 14 L1.5 14 Z" fill="currentColor" opacity=".3"/><polyline points="1.5,12 5,7 9,9 14.5,3" fill="none" stroke="currentColor" stroke-width="2"/></svg>',
    "stepped":         '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2"><polyline points="1.5,12 5,12 5,7 9,7 9,9 14.5,9 14.5,3"/></svg>',
    "smooth":          '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2"><path d="M1.5 12 C4 12 4 4 8 6 C11 8 11 2 14.5 3"/></svg>',
    "multiline":       '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><polyline points="1.5,10 5,6 9,8 14.5,4"/><polyline points="1.5,13 5,11 9,12 14.5,9" opacity=".5"/></svg>',
    "violin":          '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.3"><path d="M8 2 C5 2 3.5 5 3.5 8 C3.5 11 5 14 8 14 C11 14 12.5 11 12.5 8 C12.5 5 11 2 8 2"/><ellipse cx="8" cy="8" rx="3" ry="1.5" fill="currentColor" opacity=".3" stroke="none"/></svg>',
    "box":             '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="5" y="5" width="6" height="7" rx=".5"/><line x1="8" y1="5" x2="8" y2="2"/><line x1="8" y1="12" x2="8" y2="14.5"/><line x1="8" y1="8.5" x2="11" y2="8.5"/><line x1="6" y1="2" x2="10" y2="2"/><line x1="6" y1="14.5" x2="10" y2="14.5"/></svg>',
    "kde":             '<svg width="16" height="16" viewBox="0 0 16 16"><path d="M1 13 C2 13 3 3 8 3 C13 3 14 13 15 13 L15 14 L1 14 Z" fill="currentColor" opacity=".2"/><path d="M1 13 C2 13 3 3 8 3 C13 3 14 13 15 13" fill="none" stroke="currentColor" stroke-width="1.5"/></svg>',
    "histogram":       '<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><rect x="1" y="9" width="3" height="5.5" rx=".5"/><rect x="5" y="4" width="3" height="10.5" rx=".5"/><rect x="9" y="6" width="3" height="8.5" rx=".5"/><rect x="13" y="11" width="2" height="3.5" rx=".5"/></svg>',
    "waterfall":       '<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><rect x="1" y="8" width="3" height="6.5" rx=".5"/><rect x="5" y="5" width="3" height="3" rx=".5" opacity=".7"/><rect x="9" y="3" width="3" height="2" rx=".5"/><rect x="13" y="2" width="2" height="5" rx=".5" opacity=".6"/></svg>',
    "funnel":          '<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><rect x="2" y="2" width="12" height="3" rx="1"/><rect x="4" y="6.5" width="8" height="2.5" rx="1"/><rect x="6" y="10.5" width="4" height="2" rx="1"/><rect x="7" y="13.5" width="2" height="1.5" rx=".5"/></svg>',
    "bubble":          '<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><circle cx="4" cy="11" r="3" opacity=".5"/><circle cx="10" cy="6" r="4.5" opacity=".3"/><circle cx="13" cy="12" r="2" opacity=".6"/></svg>',
    "lollipop":        '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><line x1="3" y1="14" x2="3" y2="5.5"/><circle cx="3" cy="4" r="2" fill="currentColor" stroke="none"/><line x1="8" y1="14" x2="8" y2="3.5"/><circle cx="8" cy="2.5" r="2" fill="currentColor" stroke="none"/><line x1="13" y1="14" x2="13" y2="7.5"/><circle cx="13" cy="6.5" r="2" fill="currentColor" stroke="none"/></svg>',
    "slope":           '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="3" cy="12" r="1.8" fill="currentColor" stroke="none"/><circle cx="13" cy="4" r="1.8" fill="currentColor" stroke="none"/><line x1="3" y1="12" x2="13" y2="4"/></svg>',
    "heatmap":         '<svg width="16" height="16" viewBox="0 0 16 16"><rect x="1" y="1" width="4" height="4" rx=".5" fill="currentColor" opacity=".9"/><rect x="6" y="1" width="4" height="4" rx=".5" fill="currentColor" opacity=".5"/><rect x="11" y="1" width="4" height="4" rx=".5" fill="currentColor" opacity=".2"/><rect x="1" y="6" width="4" height="4" rx=".5" fill="currentColor" opacity=".4"/><rect x="6" y="6" width="4" height="4" rx=".5" fill="currentColor" opacity=".85"/><rect x="11" y="6" width="4" height="4" rx=".5" fill="currentColor" opacity=".3"/><rect x="1" y="11" width="4" height="4" rx=".5" fill="currentColor" opacity=".15"/><rect x="6" y="11" width="4" height="4" rx=".5" fill="currentColor" opacity=".6"/><rect x="11" y="11" width="4" height="4" rx=".5" fill="currentColor"/></svg>',
    "parallel":        '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.2"><line x1="3" y1="2" x2="3" y2="14"/><line x1="8" y1="2" x2="8" y2="14"/><line x1="13" y1="2" x2="13" y2="14"/><path d="M3 4 L8 9 L13 6" opacity=".8"/><path d="M3 7 L8 5 L13 11" opacity=".5"/></svg>',
    "radar":           '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.2"><polygon points="8,1.5 14.2,5.8 11.8,13 4.2,13 1.8,5.8"/><polygon points="8,4 11.6,6.5 10.2,11 5.8,11 4.4,6.5" fill="currentColor" opacity=".2"/></svg>',
    "wordcloud":       '<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><rect x="1" y="2" width="8" height="3" rx=".5"/><rect x="1" y="6.5" width="5" height="2" rx=".5" opacity=".7"/><rect x="7" y="6.5" width="8" height="2" rx=".5" opacity=".5"/><rect x="1" y="10" width="10" height="2" rx=".5" opacity=".8"/><rect x="12" y="10" width="3" height="2" rx=".5" opacity=".4"/><rect x="1" y="13" width="6" height="2" rx=".5" opacity=".6"/><rect x="8" y="13" width="4" height="2" rx=".5" opacity=".3"/></svg>',
    "dumbbell":        '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="3" cy="5" r="2" fill="currentColor" stroke="none"/><circle cx="13" cy="5" r="2" fill="currentColor" stroke="none"/><line x1="5" y1="5" x2="11" y2="5"/><circle cx="3" cy="11" r="2" fill="currentColor" stroke="none"/><circle cx="11" cy="11" r="2" fill="currentColor" stroke="none"/><line x1="5" y1="11" x2="9" y2="11"/></svg>',
    "donut":           '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="3.5"><circle cx="8" cy="8" r="4"/></svg>',
    "pie":             '<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M8 8 L8 2 A6 6 0 0 1 14 8 Z" opacity=".9"/><path d="M8 8 L14 8 A6 6 0 0 1 3.1 13.2 Z" opacity=".6"/><path d="M8 8 L3.1 13.2 A6 6 0 1 1 8 2 Z" opacity=".3"/></svg>',
    "ridgeline":       '<svg width="16" height="16" viewBox="0 0 16 16"><path d="M1 12 Q4 7 7 10 Q9 12 12 4 Q14 .5 15 12 L15 13 L1 13 Z" fill="currentColor" opacity=".2"/><path d="M1 12 Q4 7 7 10 Q9 12 12 4 Q14 .5 15 12" fill="none" stroke="currentColor" stroke-width="1.3"/><path d="M1 10 Q4 5 7 8 Q9 10 12 2 Q14 0 15 10" fill="none" stroke="currentColor" stroke-width="1.3" opacity=".45"/></svg>',
    "gauge":           '<svg width="16" height="16" viewBox="0 0 16 16"><path d="M2 13 A7 7 0 0 1 14 13" fill="none" stroke="currentColor" stroke-width="3" opacity=".2"/><path d="M2 13 A7 7 0 0 1 10.5 3.5" fill="none" stroke="currentColor" stroke-width="3"/><line x1="8" y1="13" x2="10" y2="6" stroke="currentColor" stroke-width="1.5"/></svg>',
    "scatter":         '<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><circle cx="4" cy="11" r="1.5"/><circle cx="7" cy="7" r="1.5"/><circle cx="11" cy="4" r="1.5"/><circle cx="5" cy="5" r="1.5"/><circle cx="12" cy="10" r="1.5"/></svg>',
    "band":            '<svg width="16" height="16" viewBox="0 0 16 16"><path d="M1.5 4 L5 3 L9 5 L14.5 2 L14.5 8 L9 11 L5 9 L1.5 10 Z" fill="currentColor" opacity=".2"/><polyline points="1.5,4 5,3 9,5 14.5,2" fill="none" stroke="currentColor" stroke-width="1.5"/><polyline points="1.5,10 5,9 9,11 14.5,8" fill="none" stroke="currentColor" stroke-width="1.5"/></svg>'
  };

  function variantIcon(name) {
    var svg = VARIANT_ICONS[name];
    if (!svg && name.indexOf("stacked") !== -1) svg = VARIANT_ICONS["stacked"];
    if (!svg && name.indexOf("group")   !== -1) svg = VARIANT_ICONS["grouped"];
    if (!svg && name.indexOf("line")    !== -1) svg = VARIANT_ICONS["line"];
    if (!svg && name.indexOf("area")    !== -1) svg = VARIANT_ICONS["area"];
    return svg || '<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><circle cx="8" cy="8" r="4"/></svg>';
  }

  function updateAliasForVariant(panel) {
    hydrateAliases(panel);
  }

  function extractVariantNames(paramsHtml) {
    if (!paramsHtml) return [];
    var div = document.createElement("div");
    div.innerHTML = paramsHtml;
    var seen = Object.create(null);
    var variants = [];
    var table = div.querySelector("table");
    if (table) {
      var heads = Array.prototype.slice.call(table.querySelectorAll("thead th, tr:first-child th, tr:first-child td")).map(function (h) {
        return (h.textContent || "").trim().toLowerCase();
      });
      var variantCol = heads.indexOf("variant");
      if (variantCol >= 0) {
        table.querySelectorAll("tbody tr, tr").forEach(function (tr) {
          var cells = tr.querySelectorAll("td");
          if (cells.length <= variantCol) return;
          var v = (cells[variantCol].textContent || "").trim().toLowerCase().replace(/`/g, "");
          if (!v || v === "variant" || v === "all" || v === "toutes" || v === "all variants") return;
          if (!seen[v]) { seen[v] = true; variants.push(v); }
        });
        return variants.sort(function (a, b) {
          if (a === "basic") return -1;
          if (b === "basic") return 1;
          return a.localeCompare(b);
        });
      }
    }
    div.querySelectorAll("tr").forEach(function (tr) {
      var cells = tr.querySelectorAll("td");
      if (cells.length < 2) return;
      cells[1].textContent.split(",").forEach(function (v) {
        v = v.trim().toLowerCase();
        if (!v || v === "all" || v === "toutes" || v === "all variants") return;
        if (!seen[v]) { seen[v] = true; variants.push(v); }
      });
    });
    return variants.sort(function (a, b) {
      if (a === "basic") return -1;
      if (b === "basic") return 1;
      return a.localeCompare(b);
    });
  }

  // Builds the inner HTML for a single-tab code block.
  function buildVariantCodeHtml(code) {
    var esc = code.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
    return '<div class="sp-demo-code-wrap"><div class="sp-tabs">' +
      '<div class="sp-tab-btns"></div>' +
      '<div class="sp-tc sp-on"><pre><code class="language-python">' + esc + "</code></pre></div>" +
      "</div></div>";
  }

  function fitPreviewHtml(html) {
    var fit = '<style>html,body{margin:0;padding:0;width:100%;height:100%;overflow:hidden;background:#fff}body>*{max-width:100%!important}svg,canvas{max-width:100%!important;height:auto!important;display:block;margin:0 auto}.chart-container,.plot-container,#chart,#plot{max-width:100%!important;width:100%!important;height:auto!important;box-sizing:border-box}</style>';
    if (/<\/head>/i.test(html)) return html.replace(/<\/head>/i, fit + "</head>");
    if (/<body[^>]*>/i.test(html)) return html.replace(/<body[^>]*>/i, function (m) { return m + fit; });
    return fit + html;
  }

  function splitTopLevel(s) {
    var out = [], buf = "", depth = 0, inStr = false, q = "";
    for (var i = 0; i < s.length; i++) {
      var c = s[i];
      if (inStr) {
        buf += c;
        if (c === "\\" && i + 1 < s.length) { buf += s[++i]; continue; }
        if (c === q) inStr = false;
      } else {
        if (c === '"' || c === "'") { inStr = true; q = c; buf += c; }
        else if (c === "[" || c === "(" || c === "{") { depth++; buf += c; }
        else if (c === "]" || c === ")" || c === "}") { depth--; buf += c; }
        else if (c === "," && depth === 0) { if (buf.trim()) out.push(buf.trim()); buf = ""; }
        else buf += c;
      }
    }
    if (buf.trim()) out.push(buf.trim());
    return out;
  }

  function parseDemoValue(raw) {
    raw = String(raw || "").trim();
    if (!raw) return null;
    if (raw === "True") return true;
    if (raw === "False") return false;
    if (raw === "None") return null;
    if (/^0x[0-9a-fA-F]+$/.test(raw)) return parseInt(raw, 16);
    if (raw[0] === '"' || raw[0] === "'") return raw.slice(1, -1).replace(/\\"/g, '"').replace(/\\'/g, "'");
    if (raw[0] === "[") {
      try { return JSON.parse(raw.replace(/'/g, '"')); } catch (e) { return null; }
    }
    var n = Number(raw);
    return isNaN(n) ? raw : n;
  }

  function parseDemoInput(code) {
    var call = code ? String(code).match(/sp\.([A-Za-z_][A-Za-z0-9_]*)\s*\(/) : null;
    if (!call) return null;
    var idx = call.index;
    if (idx < 0) return null;
    var start = idx + call[0].length, depth = 1, inStr = false, q = "", end = start;
    for (; end < code.length; end++) {
      var c = code[end];
      if (inStr) {
        if (c === "\\" && end + 1 < code.length) { end++; continue; }
        if (c === q) inStr = false;
      } else {
        if (c === '"' || c === "'") { inStr = true; q = c; }
        else if (c === "(") depth++;
        else if (c === ")") { depth--; if (depth === 0) break; }
      }
    }
    var parts = splitTopLevel(code.slice(start, end));
    var input = {};
    parts.forEach(function (p, i) {
      var eq = -1, d = 0, s = false, qq = "";
      for (var j = 0; j < p.length; j++) {
        var ch = p[j];
        if (s) {
          if (ch === "\\" && j + 1 < p.length) { j++; continue; }
          if (ch === qq) s = false;
        } else {
          if (ch === '"' || ch === "'") { s = true; qq = ch; }
          else if (ch === "[" || ch === "(" || ch === "{") d++;
          else if (ch === "]" || ch === ")" || ch === "}") d--;
          else if (ch === "=" && d === 0) { eq = j; break; }
        }
      }
      if (eq < 0) {
        if (i === 0) input.title = parseDemoValue(p);
      } else {
        input[p.slice(0, eq).trim()] = parseDemoValue(p.slice(eq + 1));
      }
    });
    return { fn: call[1], input: input };
  }

  function buildDynamicAliases(sp) {
    if (!sp) return {};
    if (typeof sp.chartAliases === "function") {
      try { return JSON.parse(sp.chartAliases()); } catch (e) {}
    }
    var aliases = {};
    Object.keys(sp).forEach(function (k) {
      if (typeof sp[k] !== "function") return;
      var snake = k.replace(/[A-Z]/g, function (c) { return "_" + c.toLowerCase(); });
      var base = snake.indexOf("build_") === 0 ? snake.slice(6) : snake;
      var stripped = base.length > 6 && base.slice(-6) === "_chart" ? base.slice(0, -6) : base;
      aliases[base] = k;
      aliases[stripped] = k;
      if (base.slice(-1) !== "s") aliases[base + "s"] = k;
      if (stripped.slice(-1) !== "s") aliases[stripped + "s"] = k;
    });
    return aliases;
  }

  function fnCandidates(fn) {
    fn = normalizeVariantName(fn);
    var camel = fn.split("_").map(function (s, i) {
      return i === 0 ? s : s.charAt(0).toUpperCase() + s.slice(1);
    }).join("");
    var base = camel.charAt(0).toUpperCase() + camel.slice(1);
    var out = [];
    if (camel.indexOf("build") === 0) out.push(camel);
    else {
      out.push("build" + base);
      out.push("build" + base + "Chart");
      out.push(camel);
    }
    out.push("build" + base + "Chart");
    var seen = {};
    return out.filter(function (name) {
      if (seen[name]) return false;
      seen[name] = true;
      return true;
    });
  }

  function resolveWasmFn(sp, fn) {
    if (!sp || !fn) return null;
    var aliases = buildDynamicAliases(sp);
    var resolved = aliases && aliases[fn] ? aliases[fn] : null;
    if (resolved && typeof sp[resolved] === "function") return sp[resolved];
    var cands = fnCandidates(fn);
    for (var i = 0; i < cands.length; i++) {
      if (typeof sp[cands[i]] === "function") return sp[cands[i]];
    }
    if (fn.slice(-1) === "s") return resolveWasmFn(sp, fn.slice(0, -1));
    return null;
  }

  function buildChartPreviewHtml(sp, family, variant, code, geomVariant) {
    if (!sp || !code) return "";
    try {
      var parsed = parseDemoInput(code);
      if (!parsed) return "";
      var input = parsed.input || {};
      if (geomVariant) input.variant = geomVariant;
      else if (!input.variant) input.variant = variant;
      input.width = 900;
      input.height = 480;
      var build = resolveWasmFn(sp, parsed.fn || family);
      return build ? build(JSON.stringify(input)) || "" : "";
    } catch (e) {
      return "";
    }
  }

  var GEOMETRY_COMPANION = { bar_3d: "bar" };

  function geometryOptionsFor(family) {
    var companion = GEOMETRY_COMPANION[family];
    if (!companion) return null;
    var map = chartVariantsMap();
    var opts = normalizeVariantList(map && map[companion]);
    return opts.length ? opts : null;
  }

  function buildCombinedDemoCode(sceneCode, geomKey) {
    if (!sceneCode || !geomKey || geomKey === "basic") return sceneCode;
    var q = geomKey.replace(/"/g, "");
    return sceneCode.replace(/\n\)\n$/, ',\n    variant="' + q + '"\n)\n');
  }

  function buildGeometryPickerHtml(clsId, sceneKey, options, lang) {
    var label = lang === "fr" ? "Géométrie 2D (variant)" : "2D geometry (variant)";
    var html = '<div class="sp-geom-picker" style="margin:8px 0;display:flex;align-items:center;gap:8px;font-size:12px">' +
      '<label style="color:#94a3b8">' + escapeAttr(label) + '</label>' +
      '<select class="sp-geom-select" data-cls="' + escapeAttr(clsId) + '" data-scene="' + escapeAttr(sceneKey) + '" style="background:rgba(15,23,42,.6);color:#e2e8f0;border:1px solid rgba(255,255,255,.12);border-radius:6px;padding:3px 8px">';
    options.forEach(function (item) {
      html += '<option value="' + escapeAttr(item.key) + '">' + escapeAttr(variantLabel(item)) + '</option>';
    });
    html += '</select></div>';
    return html;
  }

  function wireGeometryPickers(root, sp, family, lang, panel) {
    root.querySelectorAll(".sp-geom-select").forEach(function (sel) {
      sel.addEventListener("change", function () {
        var clsId = sel.getAttribute("data-cls");
        var sceneKey = sel.getAttribute("data-scene");
        var geomKey = sel.value;
        var varDiv = document.getElementById(variantDomId(clsId, sceneKey));
        if (!varDiv || !sp || typeof sp.demo !== "function") return;
        try {
          var sceneCode = sp.demo(JSON.stringify({ family: family, variant: sceneKey })) || "";
          var code = buildCombinedDemoCode(sceneCode, geomKey);
          var codeBlock = varDiv.querySelector(".sp-demo-code-wrap");
          if (codeBlock) codeBlock.remove();
          var oldWrap = varDiv.querySelector(".sp-iframe-wrap");
          if (oldWrap) oldWrap.remove();
          var oldLabel = varDiv.querySelector(".sp-preview-label");
          if (oldLabel) oldLabel.remove();
          var oldFrame = varDiv.querySelector(".sp-preview-frame");
          if (oldFrame) oldFrame.remove();
          if (code) varDiv.insertAdjacentHTML("beforeend", buildVariantCodeHtml(code));
          var chartHtml = buildChartPreviewHtml(sp, family, sceneKey, code, geomKey);
          var previewHtml = buildVariantPreviewHtml(chartHtml, lang === "fr" ? "Aperçu" : "Preview");
          if (previewHtml) varDiv.insertAdjacentHTML("beforeend", previewHtml);
          if (window.hljs) {
            var hFn = hljs.highlightElement || hljs.highlightBlock;
            if (hFn) varDiv.querySelectorAll("pre code").forEach(function (c) {
              try { hFn.call(hljs, c); } catch (e) {}
            });
          }
          if (panel) rescaleIframesInPanel(panel);
        } catch (e) {}
      });
    });
  }

  function buildVariantPreviewHtml(html, label) {
    if (!html) return "";
    var src = fitPreviewHtml(html)
      .replace(/&/g, "&amp;")
      .replace(/"/g, "&quot;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;");
    return '<div class="sp-preview-label">' + escapeAttr(label || "Preview") + '</div>' +
      '<iframe class="sp-preview-frame" srcdoc="' + src + '" scrolling="no" sandbox="allow-scripts allow-same-origin"></iframe>';
  }

  function normalizeVariantList(raw) {
    if (!raw) return [];
    if (Array.isArray(raw)) {
      return raw.map(function (v) {
        if (typeof v === "string") return { key: normalizeVariantName(v), aliases: [] };
        return {
          key: normalizeVariantName(v.key || v.name || ""),
          aliases: Array.isArray(v.aliases) ? v.aliases.map(function (a) { return String(a || "").trim(); }).filter(Boolean) : []
        };
      }).filter(function (v) { return v.key; });
    }
    if (raw.variants) return normalizeVariantList(raw.variants);
    return [];
  }

  function chartVariantsMap() {
    return registryJson("chartVariants", null, null);
  }

  function canonicalFamilyName(family, variantsMap) {
    if (!family || !variantsMap) return family || "";
    if (variantsMap[family]) return family;
    var aliases = aliasMap();
    var target = aliases && aliases[family] ? aliases[family] : null;
    if (target && variantsMap[target]) return target;
    if (family.slice(-1) === "s" && variantsMap[family.slice(0, -1)]) return family.slice(0, -1);
    return family;
  }

  function variantsFor(family, paramsHtml) {
    var map = chartVariantsMap();
    if (map) {
      var canonical = canonicalFamilyName(family, map);
      var fromWasm = normalizeVariantList(map && map[canonical]);
      if (fromWasm.length) return fromWasm;
    }
    return extractVariantNames(paramsHtml).map(function (v) {
      return { key: normalizeVariantName(v), aliases: [] };
    });
  }

  function pageRegistryFamily() {
    var el = document.querySelector("[data-sp-registry-table][data-family]");
    return el ? el.getAttribute("data-family") : "";
  }

  function injectVariantCls(panel, body, data, lang) {
    var family = data.functionName;
    if (!family) return;
    var map = chartVariantsMap();
    if (map && !map[family]) {
      var pageFamily = pageRegistryFamily();
      if (pageFamily && map[pageFamily]) family = pageFamily;
    }
    var variants = variantsFor(family, data.parameters);
    if (!variants.length) return;

    var clsId = "pp-" + family + "-cls";
    if (document.getElementById(clsId)) return;

    var sp = window.SeraplotWASM;

    var clsDiv = document.createElement("div");
    clsDiv.className = "sp-cls";
    clsDiv.id = clsId;

    variants.forEach(function (item, i) {
      var v = item.key;
      var label = variantLabel(item);
      var btn = document.createElement("button");
      btn.className = "sp-cls-tab" + (i === 0 ? " sp-cact" : "");
      btn.setAttribute("onclick", "spCls('" + clsId + "','" + v.replace(/'/g, "\\'") + "',this)");
      btn.setAttribute("data-variant", v);
      btn.setAttribute("data-aliases", JSON.stringify(aliasesForItem(item).map(function (a) { return "sp." + a; })));
      btn.innerHTML = '<span class="sp-cic">' + variantIcon(v) + '</span><span class="sp-clb">' + escapeAttr(label) + '</span>';
      clsDiv.appendChild(btn);

      var varDiv = document.createElement("div");
      varDiv.id = variantDomId(clsId, v);
      varDiv.setAttribute("data-variant", v);
      varDiv.className = "sp-variant" + (i === 0 ? " sp-von" : "");
      varDiv.style.display = i === 0 ? "block" : "none";
      varDiv.innerHTML = buildVariantAliasesHtml(item, lang);

      var geomOpts = geometryOptionsFor(family);
      if (geomOpts) varDiv.innerHTML += buildGeometryPickerHtml(clsId, v, geomOpts, lang);

      if (sp && typeof sp.demo === "function") {
        try {
          var code = sp.demo(JSON.stringify({ family: family, variant: v })) || "";
          if (code) varDiv.innerHTML += buildVariantCodeHtml(code);
        } catch (e) {}
      }
      var previewHtml = buildVariantPreviewHtml(buildChartPreviewHtml(sp, family, v, code), lang === "fr" ? "Aper\u00e7u" : "Preview");
      if (previewHtml) varDiv.innerHTML += previewHtml;

      clsDiv.appendChild(varDiv);
    });

    body.appendChild(clsDiv);
    wireGeometryPickers(clsDiv, sp, family, lang, panel);

    updateAliasForVariant(panel);

    // If WASM was not ready yet, poll and fill code + rebuild rail when it loads.
    if (!sp || typeof sp.demo !== "function") {
      var tries = 0;
      var timer = setInterval(function () {
        tries++;
        var sp2 = window.SeraplotWASM;
        if ((sp2 && typeof sp2.demo === "function") || tries > 120) {
          clearInterval(timer);
          if (!sp2 || typeof sp2.demo !== "function") return;
          var root = document.getElementById(clsId);
          if (!root) return;
          var liveVariants = variantsFor(family, data.parameters);
          root.querySelectorAll(".sp-cls-tab").forEach(function (btn) {
            var variant = btn.getAttribute("data-variant") || "";
            var item = liveVariants.filter(function (it) { return it.key === variant; })[0] || { key: variant, aliases: [] };
            btn.setAttribute("data-aliases", JSON.stringify(aliasesForItem(item).map(function (a) { return "sp." + a; })));
          });
          var geomOpts2 = geometryOptionsFor(family);
          root.querySelectorAll(".sp-variant").forEach(function (varDiv) {
            var variant = varDiv.getAttribute("data-variant") || "basic";
            var item = liveVariants.filter(function (it) { return it.key === variant; })[0] || { key: variant, aliases: [] };
            try {
              var code = sp2.demo(JSON.stringify({ family: family, variant: variant })) || "";
              if (code) {
                varDiv.innerHTML = buildVariantAliasesHtml(item, getLang());
                if (geomOpts2) varDiv.innerHTML += buildGeometryPickerHtml(clsId, variant, geomOpts2, getLang());
                if (code) varDiv.innerHTML += buildVariantCodeHtml(code);
                varDiv.innerHTML += buildVariantPreviewHtml(buildChartPreviewHtml(sp2, family, variant, code), getLang() === "fr" ? "Aper\u00e7u" : "Preview");
                if (window.hljs) {
                  var hFn = hljs.highlightElement || hljs.highlightBlock;
                  if (hFn) varDiv.querySelectorAll("pre code").forEach(function (c) {
                    try { hFn.call(hljs, c); } catch (e) {}
                  });
                }
              }
            } catch (e) {}
          });
          wireGeometryPickers(root, sp2, family, getLang(), panel);
          hoistClassRails(panel);
          refreshRailToggleLabel();
          updateAliasForVariant(panel);
        }
      }, 80);
    }
  }

  function renderBody(panel) {
    var body = panel.querySelector(".sp-pb");
    if (!body) return;
    body.innerHTML = "";
    var lang = getLang();
    var data = sectionData[lang] || sectionData.en || sectionData.fr;
    if (!data) return;

    var labels = lang === "fr"
      ? { sig: "Signature", alias: "Alias", par: "Param\u00e8tres", ret: "Retour" }
      : { sig: "Signature", alias: "Aliases", par: "Parameters", ret: "Returns" };

    function addSec(html, label, cls) {
      if (!html || !html.trim()) return;
      var w = document.createElement("div");
      w.className = "sp-psec " + cls;
      var l = document.createElement("div");
      l.className = "sp-psec-lbl";
      l.textContent = label;
      w.appendChild(l);
      var c = document.createElement("div");
      c.className = "sp-psec-content";
      c.innerHTML = html;
      w.appendChild(c);
      body.appendChild(w);
    }

    var remappedSig = remapPanelHtml(data.signature);
    var scopeMatch = remappedSig.match(/\bid="(pp-[a-z0-9_-]+)"/i);
    var previewId = scopeMatch ? scopeMatch[1] + "-preview" : "";
    addSec(remappedSig,                        labels.sig,   "sp-psec-sig");
    addSec(remapPanelHtml(data.alias || autoAliasHtml(data.functionName)), labels.alias, "sp-psec-alias");
    addSec(remapPanelHtml(data.returns),        labels.ret,   "sp-psec-returns");
    hydrateAliases(body);
    injectVariantCls(panel, body, data, lang);

    // ── Chart preview iframe (full-size + CSS scale to fit panel width) ────
    if (exampleData.iframeSrc) {
      var CHART_W = 900, CHART_H = 490;
      var iw = document.createElement("div");
      iw.className = "sp-psec sp-psec-preview";
      var il = document.createElement("div");
      il.className = "sp-psec-lbl";
      var eyeSvg = '<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>';
      il.innerHTML = eyeSvg + (lang === "fr" ? "Aper\u00e7u" : "Preview");
      iw.appendChild(il);
      // Wrapper: height controlled programmatically so layout = visual (no scroll).
      var wrap = document.createElement("div");
      wrap.style.cssText = "width:100%;position:relative;overflow:hidden;border-radius:6px;";
      var pIframe = document.createElement("iframe");
      if (previewId) pIframe.id = previewId;
      pIframe.src = exampleData.iframeSrc;
      pIframe.setAttribute("scrolling", "no");
      // No loading="lazy" — avoids the global CSS height:380px !important override.
      // position:absolute removes the iframe from flow so wrap height rules the layout.
      pIframe.style.cssText = "position:absolute;top:0;left:0;width:" + CHART_W + "px;height:" + CHART_H + "px;border:none;transform-origin:0 0;background:#0d1117;overflow:hidden;";
      wrap.appendChild(pIframe);
      iw.appendChild(wrap);
      body.appendChild(iw);
      // Scale iframe to fit panel width, re-run on load and resize.
      function rescaleIframe() {
        var aw = wrap.offsetWidth;
        if (!aw) return;
        var scale = aw / CHART_W;
        pIframe.style.transform = "scale(" + scale + ")";
        wrap.style.height = Math.ceil(CHART_H * scale) + "px";
      }
      pIframe.onload = rescaleIframe;
      setTimeout(rescaleIframe, 50);
      setTimeout(rescaleIframe, 300);
      if (window.ResizeObserver) {
        new ResizeObserver(rescaleIframe).observe(wrap);
      } else {
        window.addEventListener("resize", rescaleIframe);
      }
    }

    if (window.hljs) {
      var hFn = hljs.highlightElement || hljs.highlightBlock;
      if (hFn) {
        body.querySelectorAll("pre code").forEach(function (c) {
          try { hFn.call(hljs, c); } catch (e) {}
        });
      }
    }

    hoistClassRails(panel);
    rescaleIframesInPanel(panel);
  }

  // Build a filing-cabinet ticket rail appended to document.body (position:fixed,
  // z-index:799) so it sits behind the panel (800) in the global stacking context.
  // JS positions it flush to the panel's left border via getBoundingClientRect.
  function hoistClassRails(panel) {
    // Remove any previously-appended body rail and its resize listener
    document.querySelectorAll(".sp-cls-rail-side").forEach(function (r) {
      if (r.parentNode) r.parentNode.removeChild(r);
    });
    if (lastRailResizeHandler) {
      window.removeEventListener("resize", lastRailResizeHandler);
      lastRailResizeHandler = null;
    }
    positionRailFn = null;

    // Unwrap old .sp-panel-row, restore .sp-pb as direct child
    var oldRow = Array.prototype.filter.call(panel.childNodes, function (c) {
      return c.classList && c.classList.contains("sp-panel-row");
    })[0];
    if (oldRow) {
      var oldPb = oldRow.querySelector(".sp-pb");
      if (oldPb) panel.insertBefore(oldPb, oldRow);
      panel.removeChild(oldRow);
    }

    var pb = Array.prototype.filter.call(panel.childNodes, function (c) {
      return c.classList && c.classList.contains("sp-pb");
    })[0];
    if (!pb) return;

    var clsContainers = Array.prototype.slice.call(pb.querySelectorAll(".sp-cls"));
    if (!clsContainers.length) return;

    var rail = document.createElement("div");
    rail.className = "sp-cls-rail-side";
    // Restore wide-state from previous session
    if (localStorage.getItem("sp_rail_wide") === "1") rail.classList.add("sp-rail-wide");

    clsContainers.forEach(function (cls) {
      var scope = cls.id;
      if (!scope) return;
      cls.querySelectorAll(".sp-cls-tab").forEach(function (origBtn) {
        var oc = origBtn.getAttribute("onclick") || "";
        var m  = oc.match(/spCls\('([^']+)','([^']+)'/);
        if (!m) return;
        var name = m[2];
        var icon = origBtn.querySelector(".sp-cic");
        var lbl  = origBtn.querySelector(".sp-clb");

        var btn = document.createElement("button");
        btn.className = "sp-rail-btn" + (origBtn.classList.contains("sp-cact") ? " sp-cact" : "");
        btn.setAttribute("data-scope", scope);
        btn.setAttribute("data-name",  name);
        btn.setAttribute("data-variant", origBtn.getAttribute("data-variant") || name);
        btn.innerHTML =
          '<span class="sp-cic">'  + (icon ? icon.innerHTML   : "") + '</span>' +
          '<span class="sp-clb">'  + (lbl  ? lbl.textContent  : name) + '</span>';

        btn.addEventListener("click", function () {
          var root = document.getElementById(scope);
          if (!root) return;
          root.querySelectorAll(".sp-variant").forEach(function (s) {
            s.classList.remove("sp-von");
            s.style.display = "none";
          });
          rail.querySelectorAll('.sp-rail-btn[data-scope="' + scope + '"]').forEach(function (b) { b.classList.remove("sp-cact"); });
          var variantKey = btn.getAttribute("data-variant") || name;
          var v = document.getElementById(variantDomId(scope, variantKey));
          if (v) {
            v.classList.add("sp-von");
            v.style.display = "block";
          }
          btn.classList.add("sp-cact");
          updateAliasForVariant(panel);
          if (window.hljs && v) {
            v.querySelectorAll("code").forEach(function (c) {
              try { (hljs.highlightElement || hljs.highlightBlock).call(hljs, c); } catch (e) {}
            });
          }
          rescaleIframesInPanel(panel);
        });
        rail.appendChild(btn);
      });
    });

    // .sp-panel-row wraps .sp-pb only; rail goes to body for correct stacking
    var row = document.createElement("div");
    row.className = "sp-panel-row";
    panel.insertBefore(row, pb);
    row.appendChild(pb);

    // Append rail to body so it's behind the panel (z-index:799 < panel:800)
    document.body.appendChild(rail);

    function positionRail() {
      if (!document.body.contains(panel)) return;
      var isBottom    = panel.classList.contains("sp-p-bottom");
      var isCollapsed = panel.classList.contains("sp-p-collapsed");
      if (isCollapsed) { rail.style.display = "none"; return; }
      rail.style.display = "";
      rail.classList.toggle("sp-rail-bottom", isBottom);
      var r  = panel.getBoundingClientRect();
      var vh = document.documentElement.clientHeight;
      if (isBottom) {
        // Bottom mode: horizontal strip above the panel.
        // left/right match panel edges exactly.
        rail.style.left      = r.left + "px";
        rail.style.right     = (document.documentElement.clientWidth - r.right) + "px";
        rail.style.bottom    = (vh - r.top) + "px";
        rail.style.top       = "";
        rail.style.height    = "";
        rail.style.width     = "";
        rail.style.transform = "";
      } else {
        // Right mode: vertical strip to the LEFT of the panel.
        // Anchor left edge to r.left and shift left by 100% own-width.
        // This avoids any viewport-width computation — no scrollbar gap possible.
        var inset = 12;
        rail.style.left      = r.left + "px";
        rail.style.transform = "translateX(-100%)";
        rail.style.right     = "";
        rail.style.top       = (r.top  + inset) + "px";
        rail.style.bottom    = (vh - r.bottom + inset) + "px";
        rail.style.height    = "";
        rail.style.width     = "";
      }
    }
    positionRail();
    positionRailFn = positionRail;
    lastRailResizeHandler = positionRail;
    window.addEventListener("resize", positionRail);

    // Inline-style display fix: makes variants/tabs visible regardless of CSS
    // cascade conflicts between custom.css and bar.md's inline <style> block.
    clsContainers.forEach(function (cls) {
      cls.querySelectorAll(".sp-variant").forEach(function (v) {
        v.style.display = v.classList.contains("sp-von") ? "block" : "none";
      });
      cls.querySelectorAll(".sp-tc").forEach(function (pane) {
        pane.style.display = pane.classList.contains("sp-on") ? "block" : "none";
      });
    });

    // Fix .sp-tb click handlers: mdBook HTML-encodes ' as &#39; in onclick attrs,
    // so spTab('id',…) doesn't get remapped and getElementById returns null.
    // Replace every .sp-tb onclick with an index-based closure — no IDs needed.
    pb.querySelectorAll(".sp-tabs").forEach(function (tabs) {
      var btns  = Array.prototype.slice.call(tabs.querySelectorAll(".sp-tb"));
      var panes = Array.prototype.slice.call(tabs.querySelectorAll(".sp-tc"));
      btns.forEach(function (btn, idx) {
        btn.onclick = null;
        btn.addEventListener("click", function () {
          panes.forEach(function (p) { p.classList.remove("sp-on"); p.style.display = "none"; });
          btns.forEach(function  (b) { b.classList.remove("sp-act"); });
          if (panes[idx]) {
            panes[idx].classList.add("sp-on");
            panes[idx].style.display = "block";
            if (window.hljs) {
              panes[idx].querySelectorAll("code").forEach(function (c) {
                try { (hljs.highlightElement || hljs.highlightBlock).call(hljs, c); } catch (e) {}
              });
            }
          }
          btn.classList.add("sp-act");
        });
      });
    });
  }

  // Scale each .sp-preview-frame inside the panel to fit the available width.
  function rescaleIframesInPanel(panel) {
    var CHART_W = 900, CHART_H = 480;
    panel.querySelectorAll(".sp-preview-frame").forEach(function (iframe) {
      // If the iframe is already inside a dedicated wrapper, re-use it.
      // Otherwise INSERT a new wrapper div around JUST the iframe so we never
      // use a .sp-variant (or any parent with other content) as the wrap —
      // setting height+overflow:hidden on the variant would clip the code tabs.
      var wrap;
      if (iframe.parentElement && iframe.parentElement.classList.contains("sp-iframe-wrap")) {
        wrap = iframe.parentElement;
      } else {
        wrap = document.createElement("div");
        wrap.className = "sp-iframe-wrap";
        wrap.style.cssText = "position:relative;overflow:hidden;border-radius:8px;width:100%;";
        iframe.parentElement.insertBefore(wrap, iframe);
        wrap.appendChild(iframe);
        iframe.setAttribute("scrolling", "no");
        iframe.setAttribute("frameborder", "0");
        iframe.style.cssText = "position:absolute;top:0;left:0;width:" + CHART_W + "px;height:" + CHART_H + "px;border:none;transform-origin:0 0;background:#0d1117;overflow:hidden;display:block;";
      }
      function scale() {
        var aw = wrap.offsetWidth;
        if (!aw) return;
        var sc = aw / CHART_W;
        iframe.style.transform = "scale(" + sc + ")";
        wrap.style.height = Math.ceil(CHART_H * sc) + "px";
      }
      iframe.onload = scale;
      scale();
      setTimeout(scale, 200);
      if (window.ResizeObserver && !iframe._roAttached) {
        iframe._roAttached = true;
        new ResizeObserver(scale).observe(wrap);
      }
    });
  }

  function applyPos(panel, posBtn) {
    panel.classList.remove("sp-p-right", "sp-p-bottom");
    document.body.classList.remove("sp-body-right", "sp-body-bottom");
    panel.classList.add("sp-p-" + state.pos);
    document.body.classList.add("sp-body-" + state.pos);

    if (state.pos === "right") {
      panel.style.width  = state.rightW + "px";
      panel.style.height = "";
      document.documentElement.style.setProperty("--sp-pp-w", state.rightW + "px");
    } else {
      panel.style.height = state.bottomH + "px";
      panel.style.width  = "";
      document.documentElement.style.setProperty("--sp-pp-h", state.bottomH + "px");
    }
    posBtn.innerHTML = state.pos === "right"
      ? '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2"><path d="M8 3v10M4 9l4 4 4-4"/></svg>'
      : '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 8h10M9 4l4 4-4 4"/></svg>';
    posBtn.title = state.pos === "right"
      ? (getLang() === "fr" ? "Ancrer en bas" : "Dock to bottom")
      : (getLang() === "fr" ? "Ancrer \u00e0 droite" : "Dock to right");
    if (positionRailFn) positionRailFn();
  }

  function applyCollapsed(panel, colBtn) {
    panel.classList.toggle("sp-p-collapsed", state.collapsed);
    // When collapsed, the panel shrinks to just the header bar —
    // remove the content offset so the page fills the full width again.
    if (state.collapsed) {
      document.body.classList.remove("sp-body-right", "sp-body-bottom");
    } else {
      document.body.classList.add("sp-body-" + state.pos);
    }
    colBtn.innerHTML = state.collapsed
      ? '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 6l4 4 4-4"/></svg>'
      : '<svg viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2"><path d="M4 10l4-4 4 4"/></svg>';
    colBtn.title = state.collapsed
      ? (getLang() === "fr" ? "D\u00e9plier" : "Expand")
      : (getLang() === "fr" ? "Replier" : "Collapse");
    if (positionRailFn) positionRailFn();
  }

  function attachResize(panel) {
    var handle = panel.querySelector(".sp-resize");
    if (!handle) return;
    handle.addEventListener("mousedown", function (e) {
      e.preventDefault();
      var startX = e.clientX, startY = e.clientY;
      var rect = panel.getBoundingClientRect();
      var startW = rect.width, startH = rect.height;
      document.body.classList.add("sp-resizing");

      function onMove(ev) {
        if (state.pos === "right") {
          var dx = startX - ev.clientX;
          var w = Math.min(Math.max(260, startW + dx), Math.min(900, window.innerWidth - 200));
          state.rightW = w;
          panel.style.width = w + "px";
          document.documentElement.style.setProperty("--sp-pp-w", w + "px");
        } else {
          var dy = startY - ev.clientY;
          var h = Math.min(Math.max(140, startH + dy), Math.min(800, window.innerHeight - 80));
          state.bottomH = h;
          panel.style.height = h + "px";
          document.documentElement.style.setProperty("--sp-pp-h", h + "px");
        }
        if (positionRailFn) positionRailFn();
      }
      function onUp() {
        document.removeEventListener("mousemove", onMove);
        document.removeEventListener("mouseup", onUp);
        document.body.classList.remove("sp-resizing");
        localStorage.setItem(W_KEY, state.rightW);
        localStorage.setItem(H_KEY, state.bottomH);
      }
      document.addEventListener("mousemove", onMove);
      document.addEventListener("mouseup", onUp);
    });
  }

  function themeBase() {
    var parts = window.location.pathname.split("/").filter(Boolean);
    parts.pop();
    return new Array(parts.length + 1).join("../") + "docs/theme/";
  }

  function ensureWasm(cb) {
    var sp = window.SeraplotWASM;
    if (!sp) { setTimeout(function () { ensureWasm(cb); }, 80); return; }
    if (sp.__ready) { cb(); return; }
    if (sp.__loading) { sp.__loading.then(cb, function () { sp.__loading = null; }); return; }
    sp.__loading = sp.__init(themeBase() + "seraplot_bg.wasm?v=" + SP_WASM_BUILD).then(cb).catch(function () { sp.__loading = null; });
  }

  function registryJson(fn, input, fallback) {
    var reg = window.SeraPlotDocRegistry;
    var sp = window.SeraplotWASM;
    if (sp && typeof sp[fn] === "function") {
      try {
        var parsed = JSON.parse(input == null ? sp[fn]() : sp[fn](JSON.stringify(input)));
        if (fn === "chartVariants" && reg && reg.variants) {
          var keys = parsed && typeof parsed === "object" ? Object.keys(parsed) : [];
          var first = keys.length ? parsed[keys[0]] : null;
          if (Array.isArray(first)) return reg.variants;
        }
        return parsed;
      } catch (e) {}
    }
    if (!reg) return fallback;
    if (fn === "chartVariants") return reg.variants || fallback;
    if (fn === "chartThemes") return reg.themes || fallback;
    if (fn === "params" && input) {
      return reg.params && reg.params[(input.family || input.chart || "") + ":" + (input.variant || "")] || fallback;
    }
    if (fn === "requiredParams" && input) {
      return reg.required && reg.required[(input.family || input.chart || "") + ":" + (input.variant || "")] || fallback;
    }
    if (fn === "docs") return reg.docs || fallback;
    return fallback;
  }

  function registryFamily(el) {
    var f = el.getAttribute("data-family") || "";
    if (f) return f;
    var data = sectionData[getLang()] || sectionData.en || sectionData.fr;
    return data && data.functionName ? data.functionName : "";
  }

  function codeList(items) {
    items = (items || []).filter(Boolean);
    return items.length ? items.map(function (x) { return "<code>" + escapeAttr(x) + "</code>"; }).join(", ") : "-";
  }

  function demoArgNames(src) {
    var out = [];
    var seen = Object.create(null);
    var s = String(src || "").replace(/\\"/g, '"');
    var depth = 0;
    var inStr = false;
    var esc = false;
    var start = 0;
    function add(chunk) {
      var m = String(chunk || "").match(/^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=/);
      if (m && m[1] !== "variant" && !seen[m[1]]) {
        seen[m[1]] = true;
        out.push(m[1]);
      }
    }
    for (var i = 0; i < s.length; i++) {
      var ch = s.charAt(i);
      if (esc) { esc = false; continue; }
      if (ch === "\\") { if (inStr) esc = true; continue; }
      if (ch === '"') { inStr = !inStr; continue; }
      if (inStr) continue;
      if (ch === "[" || ch === "{" || ch === "(") depth++;
      else if (ch === "]" || ch === "}" || ch === ")") depth = Math.max(0, depth - 1);
      else if (ch === "," && depth === 0) {
        add(s.slice(start, i));
        start = i + 1;
      }
    }
    add(s.slice(start));
    return out;
  }

  function registryVariantRows(family) {
    var variantsMap = registryJson("chartVariants", null, null);
    var canonical = canonicalFamilyName(family, variantsMap || {});
    var raw = variantsMap && variantsMap[canonical];
    var variants = normalizeVariantList(raw);
    var def = raw && raw.default ? raw.default : "";
    return variants.map(function (item) {
      var demo = registryJson("params", { family: canonical, variant: item.key }, null);
      if (typeof demo !== "string") {
        var code = "";
        var sp = window.SeraplotWASM;
        try { code = sp && sp.demo ? sp.demo(JSON.stringify({ family: canonical, variant: item.key })) : ""; } catch (e) {}
        var parsed = parseDemoInput(code);
        demo = parsed && parsed.input ? Object.keys(parsed.input).filter(function (k) { return k !== "variant"; }).join(",") : "";
      }
      var required = registryJson("requiredParams", { family: canonical, variant: item.key }, []);
      var aliases = aliasesForItem(item);
      if (item.key === def && aliases.indexOf("default") === -1) aliases.push("default");
      var demoKeys = demoArgNames(demo);
      return {
        key: item.key,
        aliases: aliases.filter(function (a) { return a !== item.key; }),
        required: Array.isArray(required) ? required : [],
        demo: demoKeys
      };
    });
  }

  function renderRegistryVariants(el, family, lang) {
    var rows = registryVariantRows(family);
    if (!rows.length) return false;
    var head = lang === "fr"
      ? "<tr><th>Variante</th><th>Alias</th><th>Requis</th><th>Arguments de demo</th></tr>"
      : "<tr><th>Variant</th><th>Aliases</th><th>Required</th><th>Demo args</th></tr>";
    el.innerHTML = "<table><thead>" + head + "</thead><tbody>" + rows.map(function (r) {
      return "<tr><td><code>" + escapeAttr(r.key) + "</code></td><td>" + codeList(r.aliases) + "</td><td>" + codeList(r.required) + "</td><td>" + codeList(r.demo) + "</td></tr>";
    }).join("") + "</tbody></table>";
    return true;
  }

  function renderRegistryOptions(el, family, lang) {
    var rows = registryVariantRows(family);
    if (!rows.length) return false;
    var byParam = {};
    rows.forEach(function (r) {
      r.required.forEach(function (p) {
        if (!byParam[p]) byParam[p] = [];
        if (byParam[p].indexOf(r.key) === -1) byParam[p].push(r.key);
      });
    });
    var keys = Object.keys(byParam).sort();
    var head = lang === "fr"
      ? "<tr><th>Paramètre</th><th>Variantes</th></tr>"
      : "<tr><th>Parameter</th><th>Variants</th></tr>";
    el.innerHTML = "<table><thead>" + head + "</thead><tbody>" + keys.map(function (k) {
      return "<tr><td><code>" + escapeAttr(k) + "</code></td><td>" + codeList(byParam[k]) + "</td></tr>";
    }).join("") + "</tbody></table>";
    return true;
  }

  function renderRegistryThemes(el, lang) {
    var raw = registryJson("chartThemes", null, null);
    var themes = normalizeVariantList(raw && raw.themes ? raw.themes : []);
    var def = raw && raw.default ? raw.default : "";
    if (!themes.length) return false;
    var head = lang === "fr"
      ? "<tr><th>Thème</th><th>Alias</th></tr>"
      : "<tr><th>Theme</th><th>Aliases</th></tr>";
    el.innerHTML = "<table><thead>" + head + "</thead><tbody>" + themes.map(function (t) {
      var aliases = aliasesForItem(t).filter(function (a) { return a !== t.key; });
      if (t.key === def && aliases.indexOf("default") === -1) aliases.push("default");
      return "<tr><td><code>" + escapeAttr(t.key) + "</code></td><td>" + codeList(aliases) + "</td></tr>";
    }).join("") + "</tbody></table>";
    return true;
  }

  function renderRegistryMethods(el, files, lang) {
    var all = registryJson("docs", null, null);
    if (!all) return false;
    var fileList = files.split(",").map(function (s) { return s.trim(); }).filter(Boolean);
    var items = all.filter(function (it) { return fileList.indexOf(it.file) !== -1; });
    if (!items.length) return false;
    items.sort(function (a, b) { return a.name.localeCompare(b.name); });
    var tag = lang === "fr" ? "chaînable" : "chainable";
    el.innerHTML = items.map(function (it) {
      var desc = (lang === "fr" ? it.fr : it.en) || (lang === "fr" ? it.en : it.fr);
      var params = it.params || [];
      var argNames = params.map(function (p) { return p.name; }).join(", ");
      var sig = it.category === "chart_method"
        ? "." + it.name + "(" + argNames + ")"
        : "sp." + it.name + "(" + argNames + ")";
      var paramRows = params.map(function (p) {
        var pdesc = (lang === "fr" ? p.fr : p.en) || (lang === "fr" ? p.en : p.fr);
        return "<div class=\"cm-param\"><code>" + escapeAttr(p.name) + "</code>" +
          "<span class=\"cm-param-ty\">" + escapeAttr(p.ty) + "</span>" +
          "<span class=\"cm-param-desc\">" + escapeAttr(pdesc) + "</span></div>";
      }).join("");
      var aliases = it.aliases || [];
      var akaLabel = lang === "fr" ? "alias : " : "aka: ";
      return "<div class=\"cm-card\">" +
        "<div class=\"cm-name\"><code class=\"cm-fn\">" + escapeAttr(sig) + "</code>" +
        (it.category === "chart_method" ? "<span class=\"cm-tag cm-tag-chain\">" + tag + "</span>" : "") +
        (aliases.length ? "<span class=\"cm-tag cm-tag-alias\">" + escapeAttr(akaLabel + aliases.join(", ")) + "</span>" : "") +
        "</div>" +
        "<div class=\"cm-desc\">" + escapeAttr(desc) + "</div>" +
        (paramRows ? "<div class=\"cm-params\">" + paramRows + "</div>" : "") +
        "</div>";
    }).join("");
    return true;
  }

  function renderRegistryTables(root) {
    var lang = getLang();
    (root || document).querySelectorAll("[data-sp-registry-table]").forEach(function (el) {
      var family = registryFamily(el);
      var kind = el.getAttribute("data-sp-registry-table");
      var ok = false;
      if (kind === "variants") ok = renderRegistryVariants(el, family, lang);
      else if (kind === "options") ok = renderRegistryOptions(el, family, lang);
      else if (kind === "themes") ok = renderRegistryThemes(el, lang);
      else if (kind === "methods") ok = renderRegistryMethods(el, el.getAttribute("data-file") || "", lang);
      if (!ok) el.innerHTML = "<p>Loading...</p>";
    });
  }

  function buildPanel() {
    if (document.getElementById(PANEL_ID)) return;
    if (!isChartPage()) return;

    var en = document.querySelector(".lang-en");
    var fr = document.querySelector(".lang-fr");
    sectionData.en = collectFrom(en);
    sectionData.fr = collectFrom(fr);
    if (!sectionData.en && !sectionData.fr) return;

    // Collect code tabs and iframe for each language (read-only, no hiding).
    exampleData.en = collectTabs(en);
    exampleData.fr = collectTabs(fr);
    // Collect both iframes unconditionally so both get hidden (.sp-moved).
    var srcEn = collectIframeSrc(en);
    var srcFr = collectIframeSrc(fr);
    exampleData.iframeSrc = srcEn || srcFr;

    var panel = document.createElement("div");
    panel.id = PANEL_ID;

    var resize = document.createElement("div");
    resize.className = "sp-resize";
    panel.appendChild(resize);

    var hd = document.createElement("div");
    hd.className = "sp-ph";

    var htitle = document.createElement("div");
    htitle.className = "sp-ph-left";
    htitle.innerHTML =
      '<svg class="sp-ph-ico" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5">' +
        '<rect x="1" y="3" width="14" height="2" rx="1"/>' +
        '<rect x="1" y="7" width="10" height="2" rx="1"/>' +
        '<rect x="1" y="11" width="12" height="2" rx="1"/>' +
      '</svg>' +
      '<span class="sp-ph-tt">API</span>' +
      '<button class="sp-ph-rail-tog" type="button" aria-pressed="false"></button>';

    var ctrl = document.createElement("div");
    ctrl.className = "sp-ph-ctrl";

    var posBtn = document.createElement("button");
    posBtn.className = "sp-pc-btn sp-pc-pos";
    var colBtn = document.createElement("button");
    colBtn.className = "sp-pc-btn sp-pc-col";

    ctrl.appendChild(posBtn);
    ctrl.appendChild(colBtn);
    hd.appendChild(htitle);
    hd.appendChild(ctrl);
    panel.appendChild(hd);

    var body = document.createElement("div");
    body.className = "sp-pb";
    panel.appendChild(body);

    document.body.appendChild(panel);

    applyPos(panel, posBtn);
    applyCollapsed(panel, colBtn);
    attachResize(panel);

    posBtn.addEventListener("click", function () {
      state.pos = state.pos === "right" ? "bottom" : "right";
      localStorage.setItem(POS_KEY, state.pos);
      applyPos(panel, posBtn);
    });

    colBtn.addEventListener("click", function () {
      state.collapsed = !state.collapsed;
      localStorage.setItem(COL_KEY, state.collapsed ? "1" : "0");
      applyCollapsed(panel, colBtn);
    });

    renderBody(panel);
    renderRegistryTables(document);
    refreshRailToggleLabel();

    ensureWasm(function () {
      renderBody(panel);
      renderRegistryTables(document);
      refreshRailToggleLabel();
    });
  }

  function refreshRailToggleLabel() {
    var btn = document.querySelector("#" + PANEL_ID + " .sp-ph-rail-tog");
    if (!btn) return;
    var rail = document.querySelector(".sp-cls-rail-side");
    var tabCount = rail ? rail.querySelectorAll(".sp-rail-btn").length : 0;
    if (tabCount <= 1) {
      btn.style.display = "none";
      return;
    }
    btn.style.display = "";
    var wide = !!(rail && rail.classList.contains("sp-rail-wide"));
    var fr = getLang() === "fr";
    btn.textContent = wide
      ? (fr ? "Masquer les variantes" : "Hide variants")
      : (fr ? "Afficher les variantes" : "Show variants");
    btn.setAttribute("aria-pressed", wide ? "true" : "false");
    btn.classList.toggle("sp-rail-on", wide);
  }

  document.addEventListener("click", function (ev) {
    var t = ev.target;
    if (!t || !t.classList || !t.classList.contains("sp-ph-rail-tog")) return;
    var rail = document.querySelector(".sp-cls-rail-side");
    if (!rail) return;
    rail.classList.toggle("sp-rail-wide");
    localStorage.setItem("sp_rail_wide", rail.classList.contains("sp-rail-wide") ? "1" : "0");
    // Defer label update to the next frame so it doesn't interrupt transition start
    requestAnimationFrame(refreshRailToggleLabel);
  });

  var lastLang = getLang();
  setInterval(function () {
    var cur = getLang();
    if (cur !== lastLang) {
      lastLang = cur;
      var panel = document.getElementById(PANEL_ID);
      if (panel) {
        renderBody(panel);
        var posBtn = panel.querySelector(".sp-pc-pos");
        var colBtn = panel.querySelector(".sp-pc-col");
        if (posBtn) applyPos(panel, posBtn);
        if (colBtn) applyCollapsed(panel, colBtn);
        renderRegistryTables(document);
        refreshRailToggleLabel();
      }
    }
  }, 250);

  if (document.querySelector("[data-sp-registry-table]")) {
    ensureWasm(function () { renderRegistryTables(document); });
  }

  var registryTries = 0;
  var registryTimer = setInterval(function () {
    registryTries++;
    var pending = document.querySelector("[data-sp-registry-table]");
    if (pending) renderRegistryTables(document);
    var sp = window.SeraplotWASM;
    if (!pending || registryTries > 120 || (sp && sp.__ready)) clearInterval(registryTimer);
  }, 150);

  function tryBuild() {
    if (document.getElementById(PANEL_ID)) return;
    if (document.readyState !== "loading" && isChartPage()) buildPanel();
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", tryBuild);
  } else {
    tryBuild();
  }

  var _t = null;
  var _obs = new MutationObserver(function () {
    if (_t) clearTimeout(_t);
    _t = setTimeout(function () {
      _t = null;
      if (!document.getElementById(PANEL_ID)) tryBuild();
    }, 150);
  });
  _obs.observe(document.body, { childList: true, subtree: true });
})();
