(function () {
  var LANG_KEY = "seraplot_lang";
  var PTAB_KEY = "seraplot_ptab";

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
      fixCodePanes();
      observer.observe(document.body, { childList: true, subtree: true });
    }, 80);
  });
  observer.observe(document.body, { childList: true, subtree: true });
})();
