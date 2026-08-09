(function () {
  function esc(s) {
    return String(s == null ? "" : s)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;");
  }

  function reg() {
    return window.SeraPlotMethodRegistry || { docs: [] };
  }

  function elemLang(el) {
    return el.closest && el.closest(".lang-fr") ? "fr" : "en";
  }

  function byName(a, b) {
    return String(a.name || "").localeCompare(String(b.name || ""));
  }

  function textFor(d, l) {
    return l === "fr" ? (d.fr || d.en || "") : (d.en || d.fr || "");
  }

  function paramTextFor(p, l) {
    return l === "fr" ? (p.fr || p.en || "") : (p.en || p.fr || "");
  }

  function attrList(el, name) {
    return (el.getAttribute(name) || "")
      .split(",")
      .map(function (s) { return s.trim(); })
      .filter(Boolean);
  }

  function docsFor(el) {
    var r = reg();
    var files = attrList(el, "data-file");
    var names = attrList(el, "data-names");
    return (r.docs || []).filter(function (d) {
      if (files.length && files.indexOf(d.file) === -1) return false;
      if (names.length && names.indexOf(d.name) === -1) return false;
      return true;
    }).sort(byName);
  }

  function signature(d) {
    var params = (d.params || []).map(function (p) { return p.name; }).join(", ");
    return d.name + "(" + params + ")";
  }

  function renderParams(d, l) {
    var params = d.params || [];
    if (!params.length) return "";
    return "<div class=\"cm-params\">" + params.map(function (p) {
      return "<div class=\"cm-param\"><code>" + esc(p.name) + "</code>" +
        "<span class=\"cm-param-ty\">" + esc(p.ty) + "</span>" +
        "<span class=\"cm-param-desc\">" + esc(paramTextFor(p, l)) + "</span></div>";
    }).join("") + "</div>";
  }

  function renderAliases(d) {
    return (d.aliases || []).map(function (a) {
      return "<span class=\"cm-tag cm-tag-alias\">" + esc(a) + "</span>";
    }).join("");
  }

  function renderCard(d, l) {
    return "<div class=\"cm-card\">" +
      "<div class=\"cm-name\"><code class=\"cm-fn\">" + esc(signature(d)) + "</code>" + renderAliases(d) + "</div>" +
      "<div class=\"cm-desc\">" + esc(textFor(d, l)) + "</div>" +
      renderParams(d, l) +
      "</div>";
  }

  function renderTable(el, l) {
    var rows = docsFor(el);
    if (!rows.length) {
      el.innerHTML = "<p class=\"cm-desc\">" + (l === "fr" ? "Aucune methode enregistree pour cette page." : "No registered method for this page.") + "</p>";
      return;
    }
    el.innerHTML = rows.map(function (d) { return renderCard(d, l); }).join("");
  }

  var MODULE_ORDER = ["license", "streaming", "anomaly", "alerts", "bookmarks", "remote", "rate_limit", "history", "health", "secure", "report", "studio"];

  var MODULE_LABELS = {
    license: { en: "Licensing", fr: "Licence" },
    streaming: { en: "Streaming", fr: "Streaming" },
    anomaly: { en: "Anomaly detection", fr: "Détection d'anomalies" },
    alerts: { en: "Alerts", fr: "Alertes" },
    bookmarks: { en: "Bookmarks", fr: "Repères" },
    remote: { en: "Remote push", fr: "Push distant" },
    rate_limit: { en: "Rate limiting", fr: "Limitation de débit" },
    history: { en: "History & reports", fr: "Historique & rapports" },
    health: { en: "Health & metrics", fr: "Santé & métriques" },
    secure: { en: "Sera Secure", fr: "Sera Secure" },
    report: { en: "SeraReport (PDF)", fr: "SeraReport (PDF)" },
    studio: { en: "SeraStudio (GIF)", fr: "SeraStudio (GIF)" }
  };

  function moduleLabel(mod, l) {
    var e = MODULE_LABELS[mod];
    if (!e) return mod;
    return l === "fr" ? e.fr : e.en;
  }

  function groupByModule(rows) {
    var groups = {};
    var order = [];
    rows.forEach(function (d) {
      var m = d.module || "";
      if (!groups[m]) {
        groups[m] = [];
        order.push(m);
      }
      groups[m].push(d);
    });
    order.sort(function (a, b) {
      var ia = MODULE_ORDER.indexOf(a);
      var ib = MODULE_ORDER.indexOf(b);
      if (ia === -1 && ib === -1) return a.localeCompare(b);
      if (ia === -1) return 1;
      if (ib === -1) return -1;
      return ia - ib;
    });
    return { groups: groups, order: order };
  }

  function ensureSpTab() {
    if (window.spTab) return;
    window.spTab = function (g, id, btn) {
      var r = document.getElementById(g);
      if (!r) return;
      r.querySelectorAll(".sp-tc").forEach(function (e) { e.classList.remove("sp-on"); });
      r.querySelectorAll(".sp-tb").forEach(function (b) { b.classList.remove("sp-act"); });
      var p = document.getElementById(id);
      if (p) p.classList.add("sp-on");
      btn.classList.add("sp-act");
      if (window.hljs && p) {
        p.querySelectorAll("code").forEach(function (c) {
          try { (hljs.highlightElement || hljs.highlightBlock).call(hljs, c); } catch (err) {}
        });
      }
    };
  }

  var groupSeq = 0;

  function renderGroupedTable(el, l) {
    var rows = docsFor(el);
    if (!rows.length) {
      el.innerHTML = "<p class=\"cm-desc\">" + (l === "fr" ? "Aucune methode enregistree pour cette page." : "No registered method for this page.") + "</p>";
      return;
    }
    if (!rows.some(function (d) { return d.module; })) {
      renderTable(el, l);
      return;
    }
    ensureSpTab();
    var g = groupByModule(rows);
    var gid = "sp-modnav-" + (l === "fr" ? "fr" : "en") + "-" + (groupSeq++);
    var allId = gid + "-all";
    var btns = ["<button class=\"sp-tb sp-act\" onclick=\"spTab('" + gid + "','" + allId + "',this)\">" +
      (l === "fr" ? "Tout" : "All") + " <span class=\"cm-modcount\">" + rows.length + "</span></button>"];
    var panels = ["<div id=\"" + allId + "\" class=\"sp-tc sp-on\">" + rows.map(function (d) { return renderCard(d, l); }).join("") + "</div>"];
    g.order.forEach(function (mod, i) {
      var pid = gid + "-m" + i;
      var label = mod ? moduleLabel(mod, l) : (l === "fr" ? "Autres" : "Other");
      btns.push("<button class=\"sp-tb\" onclick=\"spTab('" + gid + "','" + pid + "',this)\">" +
        esc(label) + " <span class=\"cm-modcount\">" + g.groups[mod].length + "</span></button>");
      panels.push("<div id=\"" + pid + "\" class=\"sp-tc\">" + g.groups[mod].map(function (d) { return renderCard(d, l); }).join("") + "</div>");
    });
    el.innerHTML = "<div class=\"sp-tabs cm-modnav\" id=\"" + gid + "\">" +
      "<div class=\"sp-tab-btns\">" + btns.join("") + "</div>" +
      panels.join("") +
      "</div>";
  }

  function render(root) {
    (root || document).querySelectorAll("[data-sp-registry-table]").forEach(function (el) {
      var l = elemLang(el);
      if (el.getAttribute("data-group-by") === "module") {
        renderGroupedTable(el, l);
      } else {
        renderTable(el, l);
      }
    });
  }

  document.addEventListener("DOMContentLoaded", function () {
    render(document);
  });
  window.SeraPlotRenderMethodRegistry = render;
})();
