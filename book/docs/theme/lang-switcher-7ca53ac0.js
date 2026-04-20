(function () {
  var LANG_KEY = "seraplot_lang";
  var TAB_KEY  = "seraplot_tab";

  /* ── Language ──────────────────────────────────────────────────────────── */
  function getLang() { return localStorage.getItem(LANG_KEY) || "en"; }
  function setLang(lang) { localStorage.setItem(LANG_KEY, lang); applyLang(lang); }

  function applyLang(lang) {
    document.querySelectorAll(".lang-en, .lang-fr").forEach(function (el) {
      el.style.display = el.classList.contains("lang-" + lang) ? "" : "none";
    });
    var btn = document.getElementById("lang-toggle-btn");
    if (btn) {
      btn.textContent = lang === "en" ? "\uD83C\uDDEB\uD83C\uDDF7 Fran\u00E7ais" : "\uD83C\uDDEC\uD83C\uDDE7 English";
      btn.title       = lang === "en" ? "Passer en fran\u00E7ais" : "Switch to English";
    }
    document.querySelectorAll(".tab-btn[data-tab='algo']").forEach(function (b) {
      b.textContent = lang === "en" ? "Algorithmic Functioning" : "Fonctionnement algorithmique";
    });
  }

  /* ── Tabs ──────────────────────────────────────────────────────────────── */
  function getTab() { return localStorage.getItem(TAB_KEY) || "code"; }
  function setTab(tab) { localStorage.setItem(TAB_KEY, tab); applyTab(tab); }

  function applyTab(tab) {
    document.querySelectorAll(".tab-section-code, .tab-section-algo").forEach(function (s) {
      s.style.display = s.classList.contains("tab-section-" + tab) ? "" : "none";
    });
    document.querySelectorAll(".tab-btn[data-tab]").forEach(function (b) {
      b.classList.toggle("active", b.dataset.tab === tab);
    });
  }

  function buildTabsForDiv(langDiv) {
    var codeSection = document.createElement("div");
    codeSection.className = "tab-section-code";
    var algoSection = document.createElement("div");
    algoSection.className = "tab-section-algo";

    var beforeHr = true;
    var foundHr = false;
    var skipNextH2 = false;

    Array.from(langDiv.children).forEach(function (child) {
      if (child.tagName === "HR" && !foundHr) {
        foundHr = true; beforeHr = false; skipNextH2 = true; return;
      }
      if (child.tagName === "H2" && beforeHr) return;
      if (child.tagName === "H2" && skipNextH2) { skipNextH2 = false; return; }
      skipNextH2 = false;
      (beforeHr ? codeSection : algoSection).appendChild(child);
    });

    if (!foundHr) return false;
    while (langDiv.firstChild) langDiv.removeChild(langDiv.firstChild);
    langDiv.appendChild(codeSection);
    langDiv.appendChild(algoSection);
    return true;
  }

  function initTabs() {
    document.querySelectorAll(".lang-en:not([data-tabs-built])").forEach(function (enDiv) {
      enDiv.setAttribute("data-tabs-built", "1");
      var built = buildTabsForDiv(enDiv);
      if (!built) return;

      var frDiv = null;
      var sib = enDiv.nextElementSibling;
      while (sib) {
        if (sib.classList.contains("lang-fr")) { frDiv = sib; break; }
        sib = sib.nextElementSibling;
      }
      if (frDiv) buildTabsForDiv(frDiv);

      var tabBar = document.createElement("div");
      tabBar.className = "doc-tabs";

      var codeBtn = document.createElement("button");
      codeBtn.className = "tab-btn";
      codeBtn.dataset.tab = "code";
      codeBtn.textContent = "Code";
      codeBtn.addEventListener("click", function () { setTab("code"); });

      var algoBtn = document.createElement("button");
      algoBtn.className = "tab-btn";
      algoBtn.dataset.tab = "algo";
      algoBtn.textContent = getLang() === "en" ? "Algorithmic Functioning" : "Fonctionnement algorithmique";
      algoBtn.addEventListener("click", function () { setTab("algo"); });

      tabBar.appendChild(codeBtn);
      tabBar.appendChild(algoBtn);
      enDiv.parentNode.insertBefore(tabBar, enDiv);
    });
    applyTab(getTab());
  }

  /* ── Language button ───────────────────────────────────────────────────── */
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

  function init() {
    injectButton();
    initTabs();
    applyLang(getLang());
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
      injectButton();
      initTabs();
      applyLang(getLang());
      observer.observe(document.body, { childList: true, subtree: true });
    }, 80);
  });
  observer.observe(document.body, { childList: true, subtree: true });
})();
