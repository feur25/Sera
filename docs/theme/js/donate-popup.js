(function () {
  var STORAGE_KEY = "seraplot_donate_dismissed";

  if (localStorage.getItem(STORAGE_KEY)) return;
  if (Math.floor(Math.random() * 7) !== 0) return;

  function dismiss() {
    localStorage.setItem(STORAGE_KEY, "1");
    var el = document.getElementById("sp-donate-popup");
    if (el) {
      el.style.transition = "opacity .3s, transform .3s";
      el.style.opacity = "0";
      el.style.transform = "translateY(20px)";
      setTimeout(function () { if (el.parentNode) el.parentNode.removeChild(el); }, 350);
    }
  }

  function inject() {
    var popup = document.createElement("div");
    popup.id = "sp-donate-popup";
    popup.setAttribute("role", "dialog");
    popup.setAttribute("aria-labelledby", "sp-donate-title");
    popup.style.cssText = [
      "position:fixed",
      "bottom:24px",
      "left:50%",
      "transform:translateX(-50%) translateY(30px)",
      "width:min(520px,calc(100vw - 32px))",
      "background:#1a1f2e",
      "border:1px solid #30363d",
      "border-radius:12px",
      "box-shadow:0 8px 32px rgba(0,0,0,.55)",
      "padding:20px 24px 18px",
      "z-index:9999",
      "font-family:system-ui,sans-serif",
      "color:#e6e6e6",
      "opacity:0",
      "transition:opacity .4s, transform .4s",
    ].join(";");

    var lang = (localStorage.getItem("seraplot_lang") || "en").toLowerCase();
    var isEn = lang !== "fr";

    var title = isEn ? "SeraPlot is free \u2014 and always will be." : "SeraPlot est gratuit \u2014 et le restera.";
    var body = isEn
      ? "This documentation and the whole framework are built solo, on top of a day job. If SeraPlot saves you time, consider buying me a coffee \u2014 it helps keep the project alive."
      : "Cette documentation et l\u2019ensemble du framework sont construits seul, en plus d\u2019un travail \u00e0 plein temps. Si SeraPlot vous fait gagner du temps, pensez \u00e0 m\u2019offrir un caf\u00e9 \u2014 \u00e7a aide \u00e0 maintenir le projet.";
    var donateLabel = isEn ? "\u2665\ufe0f Support SeraPlot" : "\u2665\ufe0f Soutenir SeraPlot";
    var closeLabel = isEn ? "Maybe later" : "Peut-\u00eatre plus tard";

    popup.innerHTML =
      '<div style="display:flex;align-items:flex-start;gap:14px">' +
        '<div style="font-size:28px;line-height:1;flex-shrink:0">\ud83c\udf19</div>' +
        '<div style="flex:1;min-width:0">' +
          '<div id="sp-donate-title" style="font-size:14px;font-weight:700;color:#e2e8f0;margin-bottom:6px">' + title + '</div>' +
          '<div style="font-size:12px;color:#94a3b8;line-height:1.5;margin-bottom:14px">' + body + '</div>' +
          '<div style="display:flex;gap:10px;align-items:center;flex-wrap:wrap">' +
            '<a href="https://paypal.me/seraplot" target="_blank" rel="noopener" ' +
              'style="background:linear-gradient(135deg,#6366f1,#8b5cf6);color:#fff;text-decoration:none;' +
              'padding:8px 18px;border-radius:7px;font-size:12px;font-weight:700;white-space:nowrap">' +
              donateLabel +
            '</a>' +
            '<button id="sp-donate-close" ' +
              'style="background:transparent;border:1px solid #30363d;color:#64748b;padding:7px 14px;' +
              'border-radius:7px;font-size:12px;cursor:pointer;font-family:inherit;white-space:nowrap">' +
              closeLabel +
            '</button>' +
          '</div>' +
        '</div>' +
        '<button id="sp-donate-x" aria-label="Close" ' +
          'style="background:none;border:none;color:#64748b;font-size:18px;cursor:pointer;' +
          'line-height:1;padding:0;flex-shrink:0;align-self:flex-start">\u00d7</button>' +
      '</div>';

    document.body.appendChild(popup);

    document.getElementById("sp-donate-close").addEventListener("click", dismiss);
    document.getElementById("sp-donate-x").addEventListener("click", dismiss);

    requestAnimationFrame(function () {
      requestAnimationFrame(function () {
        popup.style.opacity = "1";
        popup.style.transform = "translateX(-50%) translateY(0)";
      });
    });
  }

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", inject);
  } else {
    inject();
  }
})();
