(function () {
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

  // Tags that mark the start of the "examples" zone — stop extraction here.
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
      });
  }

  function extractAndHide(h2El, splitAlias) {
    if (!h2El) return null;
    h2El.classList.add("sp-moved");
    var sigHtml = "";
    var aliasHtml = "";
    var sib = h2El.nextElementSibling;
    while (sib && sib.tagName !== "H2") {
      // Stop before code-example elements so they stay in the main content.
      if (isExampleBoundary(sib)) break;
      var next = sib.nextElementSibling;
      var html = sib.outerHTML; // capture BEFORE adding .sp-moved
      sib.classList.add("sp-moved");
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
    var parH2 = findH2(container, ["parameters", "param\u00e8tres"]);
    var retH2 = findH2(container, ["returns", "retour", "retours", "retourne"]);
    if (!parH2) return null;

    var sig    = extractAndHide(sigH2, true);
    var params = extractAndHide(parH2, false);
    var rets   = extractAndHide(retH2, false);

    return {
      signature:  sig    ? sig.main   : "",
      alias:      sig    ? sig.alias  : "",
      parameters: params ? params.main : "",
      returns:    rets   ? rets.main   : ""
    };
  }

  function isChartPage() {
    var en = document.querySelector(".lang-en");
    var fr = document.querySelector(".lang-fr");
    var test = en || fr;
    return !!(test && findH2(test, ["parameters", "param\u00e8tres"]));
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

    addSec(remapPanelHtml(data.signature),  labels.sig,   "sp-psec-sig");
    addSec(data.alias,      labels.alias, "sp-psec-alias");
    addSec(data.parameters, labels.par,   "sp-psec-params");
    addSec(data.returns,    labels.ret,   "sp-psec-returns");

    // ── Code example tabs ────────────────────────────────────────────────
    var tabsHtml = (lang === "fr" ? exampleData.fr : exampleData.en) || exampleData.en || exampleData.fr;
    if (tabsHtml) {
      var exLabel = lang === "fr" ? "Exemple" : "Example";
      // Give each sp-tabs container inside the panel a unique id suffix to
      // avoid duplicate-id conflicts with the same tabs in the main content.
      var panelTabsHtml = tabsHtml.replace(
        /\bid="([^"]+)"/g,
        function (m, id) { return 'id="pp-' + id + '"'; }
      ).replace(
        /spTab\('([^']+)','([^']+)',this\)/g,
        function (m, g, id) { return "spTab('pp-" + g + "','pp-" + id + "',this)"; }
      );
      var w = document.createElement("div");
      w.className = "sp-psec sp-psec-example";
      var l = document.createElement("div");
      l.className = "sp-psec-lbl";
      l.textContent = exLabel;
      w.appendChild(l);
      var c = document.createElement("div");
      c.className = "sp-psec-content";
      c.innerHTML = panelTabsHtml;
      w.appendChild(c);
      body.appendChild(w);
    }

    // ── Chart preview iframe (full-size + CSS scale to fit panel width) ────
    if (exampleData.iframeSrc) {
      var CHART_W = 900, CHART_H = 490;
      var iw = document.createElement("div");
      iw.className = "sp-psec sp-psec-preview";
      var il = document.createElement("div");
      il.className = "sp-psec-lbl";
      il.textContent = lang === "fr" ? "Aper\u00e7u" : "Preview";
      iw.appendChild(il);
      // Wrapper: height controlled programmatically so layout = visual (no scroll).
      var wrap = document.createElement("div");
      wrap.style.cssText = "width:100%;position:relative;overflow:hidden;border-radius:6px;";
      var pIframe = document.createElement("iframe");
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
      '<span class="sp-ph-tt">API</span>';

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
  }

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
      }
    }
  }, 250);

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
