(function () {
  var LANG_KEY = "seraplot_lang";

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

  function init() {
    purgeOldTabs();
    injectButton();
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
      purgeOldTabs();
      injectButton();
      applyLang(getLang());
      observer.observe(document.body, { childList: true, subtree: true });
    }, 80);
  });
  observer.observe(document.body, { childList: true, subtree: true });
})();
