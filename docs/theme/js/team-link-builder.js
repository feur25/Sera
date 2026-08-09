(function () {
  var STRINGS = {
    en: { seats: "Seats", months: "Months", cta: "Continue to checkout" },
    fr: { seats: "Postes", months: "Mois", cta: "Continuer vers le paiement" },
  };

  function buildPicker(container, lang, plan) {
    var t = STRINGS[lang];
    var seatInput = document.createElement("input");
    seatInput.type = "number";
    seatInput.min = "1";
    seatInput.max = "9999";
    seatInput.step = "1";
    seatInput.value = "1";
    seatInput.className = "sp-team-picker-input";

    var monthInput = document.createElement("input");
    monthInput.type = "number";
    monthInput.min = "1";
    monthInput.max = "36";
    monthInput.step = "1";
    monthInput.value = "1";
    monthInput.className = "sp-team-picker-input";

    var link = document.createElement("a");
    link.className = "sp-team-picker-cta";
    link.rel = "noopener";

    function clamp(input) {
      var min = parseInt(input.min, 10);
      var max = parseInt(input.max, 10);
      var v = parseInt(input.value, 10);
      if (!isFinite(v) || v < min) v = min;
      if (v > max) v = max;
      return v;
    }

    function refresh() {
      var seats = clamp(seatInput);
      var months = clamp(monthInput);
      link.href = "https://sera-payment.onrender.com/paypal/buy/" + plan + "/" + months + "?seats=" + seats;
      link.textContent = t.cta;
    }
    seatInput.addEventListener("input", refresh);
    monthInput.addEventListener("input", refresh);
    refresh();

    var seatField = document.createElement("div");
    seatField.className = "sp-team-picker-field";
    var seatLabel = document.createElement("span");
    seatLabel.textContent = t.seats;
    seatField.appendChild(seatLabel);
    seatField.appendChild(seatInput);

    var monthField = document.createElement("div");
    monthField.className = "sp-team-picker-field";
    var monthLabel = document.createElement("span");
    monthLabel.textContent = t.months;
    monthField.appendChild(monthLabel);
    monthField.appendChild(monthInput);

    container.appendChild(seatField);
    container.appendChild(monthField);
    container.appendChild(link);
  }

  var en = document.getElementById("sp-team-picker-en");
  if (en) buildPicker(en, "en", "team");
  var fr = document.getElementById("sp-team-picker-fr");
  if (fr) buildPicker(fr, "fr", "team");
})();
