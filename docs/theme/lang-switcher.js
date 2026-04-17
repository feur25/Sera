(function () {
  var STORAGE_KEY = "seraplot_lang";

  function getLang() {
    return localStorage.getItem(STORAGE_KEY) || "en";
  }

  function setLang(lang) {
    localStorage.setItem(STORAGE_KEY, lang);
    applyLang(lang);
  }

  function applyLang(lang) {
    document.querySelectorAll(".lang-en, .lang-fr").forEach(function (el) {
      if (el.classList.contains("lang-" + lang)) {
        el.style.display = "";
      } else {
        el.style.display = "none";
      }
    });
    var btn = document.getElementById("lang-toggle-btn");
    if (btn) {
      btn.textContent = lang === "en" ? "🇫🇷 Français" : "🇬🇧 English";
      btn.title = lang === "en" ? "Passer en français" : "Switch to English";
    }
  }

  function injectButton() {
    if (document.getElementById("lang-toggle-btn")) return;
    var btn = document.createElement("button");
    btn.id = "lang-toggle-btn";
    btn.className = "lang-toggle-btn";
    var lang = getLang();
    btn.textContent = lang === "en" ? "🇫🇷 Français" : "🇬🇧 English";
    btn.title = lang === "en" ? "Passer en français" : "Switch to English";
    btn.addEventListener("click", function () {
      setLang(getLang() === "en" ? "fr" : "en");
    });
    var menu = document.querySelector(".right-buttons");
    if (menu) {
      menu.insertBefore(btn, menu.firstChild);
    } else {
      document.body.appendChild(btn);
    }
  }

  function init() {
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
      injectButton();
      applyLang(getLang());
      observer.observe(document.body, { childList: true, subtree: true });
    }, 80);
  });
  observer.observe(document.body, { childList: true, subtree: true });
})();
