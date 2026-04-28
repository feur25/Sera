/* SeraPlot — Work-in-progress notice (bottom-right, dismissible).
   Shown on every page until the user closes it (persisted in localStorage). */
(function () {
  var STORAGE_KEY = "seraplot_wip_notice_dismissed_v1";
  if (typeof window === "undefined" || typeof document === "undefined") return;
  try { if (localStorage.getItem(STORAGE_KEY)) return; } catch (_e) {}

  function dismiss() {
    try { localStorage.setItem(STORAGE_KEY, "1"); } catch (_e) {}
    var el = document.getElementById("sp-wip-notice");
    if (!el) return;
    el.style.transition = "opacity .25s ease, transform .25s ease";
    el.style.opacity = "0";
    el.style.transform = "translateY(20px)";
    setTimeout(function () { if (el.parentNode) el.parentNode.removeChild(el); }, 280);
  }

  function inject() {
    if (document.getElementById("sp-wip-notice")) return;

    var style = document.createElement("style");
    style.id = "sp-wip-notice-style";
    style.textContent = [
      "#sp-wip-notice{",
      "  position:fixed;right:18px;bottom:18px;z-index:99998;",
      "  width:min(360px,calc(100vw - 36px));",
      "  background:linear-gradient(180deg,#1a1f2e 0%,#11151e 100%);",
      "  border:1px solid #2a3343;border-left:3px solid #f59e0b;",
      "  border-radius:12px;color:#e2e8f0;",
      "  font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;font-size:12.5px;line-height:1.55;",
      "  box-shadow:0 12px 36px -10px rgba(0,0,0,.55),0 0 0 1px rgba(255,255,255,.02) inset;",
      "  opacity:0;transform:translateY(20px);",
      "  transition:opacity .35s ease, transform .35s ease;",
      "  overflow:hidden;",
      "}",
      "#sp-wip-notice.sp-show{opacity:1;transform:translateY(0)}",
      "#sp-wip-notice .sp-wip-head{",
      "  display:flex;align-items:center;gap:8px;",
      "  padding:10px 12px 8px;border-bottom:1px solid rgba(255,255,255,.05);",
      "}",
      "#sp-wip-notice .sp-wip-badge{",
      "  display:inline-flex;align-items:center;gap:5px;",
      "  font-size:10px;font-weight:700;letter-spacing:.6px;text-transform:uppercase;",
      "  color:#f59e0b;background:rgba(245,158,11,.08);",
      "  border:1px solid rgba(245,158,11,.25);",
      "  padding:3px 8px;border-radius:6px;",
      "}",
      "#sp-wip-notice .sp-wip-title{font-weight:600;font-size:12px;color:#e2e8f0;flex:1;min-width:0}",
      "#sp-wip-notice .sp-wip-close{",
      "  background:transparent;border:1px solid transparent;color:#7e8a9c;",
      "  width:22px;height:22px;border-radius:5px;cursor:pointer;",
      "  display:inline-flex;align-items:center;justify-content:center;font-size:14px;line-height:1;",
      "  transition:all .15s;flex-shrink:0;",
      "}",
      "#sp-wip-notice .sp-wip-close:hover{color:#e2e8f0;border-color:#2a3343;background:rgba(255,255,255,.04)}",
      "#sp-wip-notice .sp-wip-body{padding:10px 14px 12px;color:#bcc6d4}",
      "#sp-wip-notice .sp-wip-body p{margin:0 0 8px}",
      "#sp-wip-notice .sp-wip-body p:last-child{margin-bottom:0}",
      "#sp-wip-notice .lang-en, #sp-wip-notice .lang-fr{display:none}",
      "@media (prefers-reduced-motion:reduce){",
      "  #sp-wip-notice{transition:none}",
      "}"
    ].join("\n");
    document.head.appendChild(style);

    var el = document.createElement("div");
    el.id = "sp-wip-notice";
    el.setAttribute("role", "status");
    el.setAttribute("aria-live", "polite");
    el.innerHTML = [
      '<div class="sp-wip-head">',
      '  <span class="sp-wip-badge">⚠ Warning</span>',
      '  <span class="sp-wip-title lang-en">Work in progress</span>',
      '  <span class="sp-wip-title lang-fr">En cours de rédaction</span>',
      '  <button class="sp-wip-close" type="button" aria-label="Close" title="Close">×</button>',
      '</div>',
      '<div class="sp-wip-body">',
      '  <div class="lang-fr">',
      '    <p>La doc est fortement vouée à être modifiée — je prends beaucoup de temps à la rédiger et à retravailler le framework. Ne pouvant pas être à temps plein dessus, je suis navré.</p>',
      '    <p>J\'essaie de vous pondre le meilleur outil que je peux, à mon échelle. Merci de votre compréhension.</p>',
      '  </div>',
      '  <div class="lang-en">',
      '    <p>This documentation is very much a work in progress — writing it and reworking the framework takes a lot of time. I cannot be on it full-time, so my apologies.</p>',
      '    <p>I\'m doing my best to ship the finest tool I can at my own scale. Thanks for your patience and understanding.</p>',
      '  </div>',
      '</div>'
    ].join("");

    document.body.appendChild(el);
    requestAnimationFrame(function () { el.classList.add("sp-show"); });

    el.querySelector(".sp-wip-close").addEventListener("click", dismiss);
    // lang-switcher.js handles .lang-en / .lang-fr visibility automatically
    // via its MutationObserver + applyLang — nothing extra needed here.
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", inject);
  } else {
    inject();
  }
})();
