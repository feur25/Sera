(function () {
  var LANG_KEY = "seraplot_lang";
  var PTAB_KEY = "seraplot_ptab";

  // Track the last-clicked code tab index per base group.
  // When language switches, the same index is replayed in the target section.
  // Group naming convention: EN = "<name>", FR = "<name>-fr".
  var _tabIdx = {};

  function getLang() { return localStorage.getItem(LANG_KEY) || "en"; }
  function setLang(lang) { localStorage.setItem(LANG_KEY, lang); applyLang(lang); }

  function getPageTab() { return localStorage.getItem(PTAB_KEY) || "code"; }
  function setPageTab(tab) { localStorage.setItem(PTAB_KEY, tab); applyPageTab(tab); }

  function applyLang(lang) {
    document.querySelectorAll(".lang-en, .lang-fr").forEach(function (el) {
      el.style.display = el.classList.contains("lang-" + lang) ? "" : "none";
    });
    var btn = document.getElementById("lang-toggle-btn");
    if (btn) {
      btn.textContent = lang === "en" ? "\uD83C\uDDEB\uD83C\uDDF7 Fran\u00E7ais" : "\uD83C\uDDEC\uD83C\uDDE7 English";
      btn.title       = lang === "en" ? "Passer en fran\u00E7ais" : "Switch to English";
    }
    var algoBtn = document.querySelector(".sp-ptb[data-tab='algo']");
    if (algoBtn) {
      algoBtn.textContent = lang === "en" ? "Algorithmic Functioning" : "Fonctionnement algorithmique";
    }
    // Sync the active language tab (Python / C++ / etc.) to the new section.
    syncCodeTabs(lang);
  }

  // Replay the last-clicked tab index in the language section that just became visible.
  // Directly updates DOM classes — no .click() to avoid re-triggering listeners.
  function syncCodeTabs(lang) {
    var suffix = lang === "fr" ? "-fr" : "";
    Object.keys(_tabIdx).forEach(function (baseG) {
      var idx = _tabIdx[baseG];
      var targetG = baseG + suffix;
      var container = document.getElementById(targetG);
      if (!container) return;
      var btns = container.querySelectorAll(".sp-tb");
      var panes = container.querySelectorAll(".sp-tc");
      if (!btns[idx]) return;
      btns.forEach(function (b) { b.classList.remove("sp-act"); });
      btns[idx].classList.add("sp-act");
      panes.forEach(function (p) { p.classList.remove("sp-on"); });
      if (panes[idx]) {
        panes[idx].classList.add("sp-on");
        if (window.hljs) {
          panes[idx].querySelectorAll("code").forEach(function (c) {
            try { (hljs.highlightElement || hljs.highlightBlock).call(hljs, c); } catch (e) {}
          });
        }
      }
    });
  }

  function applyPageTab(tab) {
    document.querySelectorAll(".sp-ptc-code, .sp-ptc-algo").forEach(function (d) {
      d.style.display = "none";
    });
    var sel = tab === "code" ? ".sp-ptc-code" : ".sp-ptc-algo";
    document.querySelectorAll(sel).forEach(function (d) {
      d.style.display = "";
    });
    document.querySelectorAll(".sp-ptb").forEach(function (b) {
      b.classList.toggle("sp-ptb-act", b.dataset.tab === tab);
    });
  }

  function buildPageTabs() {
    if (document.querySelector(".sp-page-tabs")) return;
    var builtAny = false;
    var algoIds = { en: "algorithmic-functioning", fr: "fonctionnement-algorithmique" };

    document.querySelectorAll(".lang-en, .lang-fr").forEach(function (langDiv) {
      if (langDiv.querySelector(".sp-ptc-code")) return;
      var lang = langDiv.classList.contains("lang-en") ? "en" : "fr";
      var algoH2 = langDiv.querySelector('h2[id="' + algoIds[lang] + '"]');
      if (!algoH2) return;
      builtAny = true;

      var codeNodes = [];
      var algoNodes = [];
      var inAlgo = false;
      Array.from(langDiv.childNodes).forEach(function (node) {
        if (node === algoH2) { inAlgo = true; }
        (inAlgo ? algoNodes : codeNodes).push(node);
      });

      var codeDiv = document.createElement("div");
      codeDiv.className = "sp-ptc-code";
      var algoDiv = document.createElement("div");
      algoDiv.className = "sp-ptc-algo";
      algoDiv.style.display = "none";

      codeNodes.forEach(function (n) { codeDiv.appendChild(n); });
      algoNodes.forEach(function (n) { algoDiv.appendChild(n); });
      langDiv.appendChild(codeDiv);
      langDiv.appendChild(algoDiv);
    });

    if (!builtAny) return;

    var lang = getLang();
    var bar = document.createElement("div");
    bar.className = "sp-page-tabs";

    var codeBtn = document.createElement("button");
    codeBtn.className = "sp-ptb sp-ptb-act";
    codeBtn.dataset.tab = "code";
    codeBtn.textContent = "Code";
    codeBtn.addEventListener("click", function () { setPageTab("code"); });

    var algoBtn = document.createElement("button");
    algoBtn.className = "sp-ptb";
    algoBtn.dataset.tab = "algo";
    algoBtn.textContent = lang === "en" ? "Algorithmic Functioning" : "Fonctionnement algorithmique";
    algoBtn.addEventListener("click", function () { setPageTab("algo"); });

    bar.appendChild(codeBtn);
    bar.appendChild(algoBtn);

    var main = document.querySelector("main");
    var h1 = main && main.querySelector("h1");
    if (h1 && h1.nextElementSibling) {
      h1.parentNode.insertBefore(bar, h1.nextElementSibling);
    } else if (main) {
      main.appendChild(bar);
    }

    applyPageTab(getPageTab());
  }

  function purgeOldTabs() {
    document.querySelectorAll(".doc-tabs").forEach(function (bar) { bar.remove(); });
    document.querySelectorAll(".tab-section-code, .tab-section-algo").forEach(function (sec) {
      var parent = sec.parentNode;
      while (sec.firstChild) parent.insertBefore(sec.firstChild, sec);
      sec.remove();
    });
    document.querySelectorAll("[data-tabs-built]").forEach(function (el) {
      el.removeAttribute("data-tabs-built");
    });
  }

  function injectButton() {
    if (document.getElementById("lang-toggle-btn")) return;
    var btn = document.createElement("button");
    btn.id = "lang-toggle-btn";
    btn.className = "lang-toggle-btn";
    var lang = getLang();
    btn.textContent = lang === "en" ? "\uD83C\uDDEB\uD83C\uDDF7 Fran\u00E7ais" : "\uD83C\uDDEC\uD83C\uDDE7 English";
    btn.title       = lang === "en" ? "Passer en fran\u00E7ais" : "Switch to English";
    btn.addEventListener("click", function () {
      setLang(getLang() === "en" ? "fr" : "en");
    });
    var menu = document.querySelector(".right-buttons");
    if (menu) menu.insertBefore(btn, menu.firstChild);
    else document.body.appendChild(btn);
  }

  function fixMathDisplay() {
    document.querySelectorAll(".content p").forEach(function (p) {
      var html = p.innerHTML;
      if (html.indexOf("$$") === -1) return;
      var fixed = html.replace(/<em>/g, "_").replace(/<\/em>/g, "_");
      fixed = fixed.replace(/<strong>/g, "__").replace(/<\/strong>/g, "__");
      if (fixed !== html) p.innerHTML = fixed;
    });
    if (window.MathJax && MathJax.Hub) {
      var content = document.querySelector(".content");
      if (content) MathJax.Hub.Queue(["Typeset", MathJax.Hub, content]);
    }
  }

  // Fix mdBook <p> injection + highlight all language tab code blocks.
  //
  // When a blank line appears inside <pre><code> in a .md file, mdBook's
  // Markdown parser exits "HTML block mode" and wraps the next paragraph in <p>.
  // The browser then auto-closes <code> before <p> (block inside phrasing is
  // invalid HTML), so the DOM ends up as:
  //   <pre> <code>first lines</code> <p>rest of code</p> </pre>
  // The old fix looked for p inside code — wrong level. This one looks at
  // <pre> children directly.
  function fixCodePanes() {
    document.querySelectorAll('.sp-tc').forEach(function(pane) {
      // Find the first <pre> that is a direct child of this tab pane.
      var pre = null;
      for (var i = 0; i < pane.childNodes.length; i++) {
        if (pane.childNodes[i].nodeName === 'PRE') { pre = pane.childNodes[i]; break; }
      }
      if (!pre) return;

      var code = pre.querySelector('code');
      if (!code) return;

      // Collect any <p> siblings of <code> inside <pre>.
      var pNodes = [];
      pre.childNodes.forEach(function(n) { if (n.nodeName === 'P') pNodes.push(n); });

      // Fallback: some browsers may keep <p> inside <code> (non-standard).
      var pInCode = code.querySelectorAll('p');

      if (pNodes.length > 0 || pInCode.length > 0) {
        // Reconstruct the full source code.
        // code.textContent = everything before the first <p> sibling.
        // Each <p> sibling represents a blank-line separated block.
        var parts = [code.textContent];
        pNodes.forEach(function(p) {
          var last = parts[parts.length - 1];
          if (!last.endsWith('\n\n')) {
            if (!last.endsWith('\n')) parts.push('\n');
            parts.push('\n'); // blank line
          }
          parts.push(p.textContent);
        });

        var text = parts.join('').replace(/^\n+/, '').replace(/\n+$/, '\n');

        // Replace code content (also strips any <p> that stayed inside code).
        code.textContent = text;
        // Remove the now-redundant <p> siblings from <pre>.
        pNodes.forEach(function(p) { if (p.parentNode === pre) pre.removeChild(p); });
      }

      // Apply (or re-apply) syntax highlighting to every pane (not just the active one).
      if (window.hljs) {
        try { (hljs.highlightElement || hljs.highlightBlock).call(hljs, code); } catch(e) {}
      }
    });
  }

  function init() {
    purgeOldTabs();
    injectButton();
    buildPageTabs();
    applyLang(getLang());
    fixMathDisplay();
    fixCodePanes();
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init);
  } else {
    init();
  }

  // Delegated click tracker: record which tab index was last activated per .sp-tabs group.
  // Works for all current and future DOM nodes — no need to wrap window.spTab.
  document.addEventListener("click", function (e) {
    var btn = e.target.closest ? e.target.closest(".sp-tb") : null;
    if (!btn) return;
    var container = btn.closest(".sp-tabs");
    if (!container || !container.id) return;
    var btns = container.querySelectorAll(".sp-tb");
    var idx = Array.from(btns).indexOf(btn);
    if (idx >= 0) _tabIdx[container.id.replace(/-fr$/, "")] = idx;
  });

  var debounceTimer = null;
  var observer = new MutationObserver(function () {
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(function () {
      debounceTimer = null;
      observer.disconnect();
      purgeOldTabs();
      injectButton();
      buildPageTabs();
      applyLang(getLang());
      applyPageTab(getPageTab());
      fixCodePanes();
      observer.observe(document.body, { childList: true, subtree: true });
    }, 80);
  });
  observer.observe(document.body, { childList: true, subtree: true });
})();

// ── Global code-tab enhancements: copy + play + eye buttons ──────────────
(function () {
  var COPY_SVG = '<svg width="13" height="13" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"><rect x="4.5" y="1.5" width="9" height="11" rx="1.5"/><path d="M3 4.5H2A1.5 1.5 0 0 0 .5 6v8A1.5 1.5 0 0 0 2 15.5h7.5A1.5 1.5 0 0 0 11 14v-1"/></svg>';
  var EYE_SVG  = '<svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"/><circle cx="12" cy="12" r="3"/></svg>';

  function getActiveCode(tabs) {
    var pane = tabs.querySelector(".sp-tc.sp-on");
    var code = pane ? pane.querySelector("code") : null;
    return code ? (code.innerText || code.textContent) : "";
  }

  function flashCopy(btn) {
    btn.classList.add("sp-copied");
    setTimeout(function () { btn.classList.remove("sp-copied"); }, 1500);
  }

  function injectButtons(tabs) {
    var bar = tabs.querySelector(".sp-tab-btns");
    if (!bar || bar.querySelector(".sp-copy-btn")) return;

    // Spacer pushes action buttons to the right
    var spacer = document.createElement("div");
    spacer.style.cssText = "flex:1;min-width:4px;";
    bar.appendChild(spacer);

    // Copy button
    var copyBtn = document.createElement("button");
    copyBtn.className = "sp-copy-btn";
    copyBtn.title = "Copy code";
    copyBtn.innerHTML = COPY_SVG;
    copyBtn.addEventListener("click", function () {
      var code = getActiveCode(tabs);
      if (!code || !navigator.clipboard) return;
      navigator.clipboard.writeText(code).then(function () { flashCopy(copyBtn); });
    });
    bar.appendChild(copyBtn);

    // Eye button — only when there's a nearby preview iframe
    var variant = tabs.closest(".sp-variant") || tabs.closest(".sp-cls-item");
    var previewLabel = variant ? variant.querySelector(".sp-preview-label") : null;
    var previewFrame = variant ? variant.querySelector(".sp-preview-frame, iframe.sp-preview-frame") : null;
    if (previewFrame) {
      var eyeBtn = document.createElement("button");
      eyeBtn.className = "sp-eye-btn";
      eyeBtn.title = "Toggle preview";
      eyeBtn.innerHTML = EYE_SVG;
      // Start in visible state (iframes are shown by default)
      eyeBtn.classList.add("sp-eye-on");
      eyeBtn.addEventListener("click", function () {
        var on = eyeBtn.classList.toggle("sp-eye-on");
        previewFrame.style.display = on ? "" : "none";
        if (previewLabel) previewLabel.style.display = on ? "" : "none";
      });
      bar.appendChild(eyeBtn);
    }
  }

  function injectAll() {
    document.querySelectorAll(".sp-tabs").forEach(injectButtons);
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", injectAll);
  } else {
    injectAll();
  }

  // Re-inject when params panel injects new .sp-tabs nodes
  var _debounce = null;
  new MutationObserver(function () {
    clearTimeout(_debounce);
    _debounce = setTimeout(injectAll, 120);
  }).observe(document.body, { childList: true, subtree: true });
})();
