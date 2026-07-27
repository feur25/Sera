(function () {
  function hexPalette(hexes) {
    return hexes
      .map(function (h) {
        var n = parseInt(h, 16);
        var css = '#' + (n & 0xffffff).toString(16).padStart(6, '0');
        return '<span class="sp-sw" style="background:' + css + '"></span>';
      })
      .join('');
  }

  function bgCell(bg) {
    if (!bg) return 'transparent';
    var swatch = '<span class="sp-bg" style="background:' + bg + '"></span> <code>' + bg + '</code>';
    return swatch;
  }

  function renderTable(themes, lang) {
    var head = lang === 'fr'
      ? '<tr><th>Thème</th><th>Fond</th><th>Quadrillage</th><th>Palette principale</th></tr>'
      : '<tr><th>Theme</th><th>Background</th><th>Gridlines</th><th>Primary palette</th></tr>';
    var rows = themes.map(function (t) {
      var grid = t.gridlines ? '✓' : '—';
      var pal = '<span class="sp-pal">' + hexPalette(t.palette.slice(0, 5)) + '</span>';
      return '<tr><td><code>"' + t.name + '"</code></td><td>' + bgCell(t.bg) + '</td><td style="text-align:center">' + grid + '</td><td>' + pal + '</td></tr>';
    }).join('');
    return '<table><thead>' + head + '</thead><tbody>' + rows + '</tbody></table>';
  }

  function renderPalettes(themes) {
    return themes.map(function (t) {
      var values = t.palette.map(function (h) { return '0x' + h.replace(/^0x/i, '').toUpperCase(); });
      var lines = [];
      for (var i = 0; i < values.length; i += 5) {
        lines.push(values.slice(i, i + 5).join(', '));
      }
      var code = lines.join(',\n ');
      return '<h4><code>"' + t.name + '"</code></h4><pre><code class="language-python">[' + code + ']</code></pre>';
    }).join('');
  }

  function mount() {
    var reg = window.SeraPlotDocRegistry;
    if (!reg || !reg.paletteThemes || !reg.paletteThemes.length) return;
    var themes = reg.paletteThemes;
    document.querySelectorAll('[data-sp-theme-table]').forEach(function (el) {
      el.innerHTML = renderTable(themes, el.dataset.spThemeTable);
    });
    document.querySelectorAll('[data-sp-theme-palettes]').forEach(function (el) {
      el.innerHTML = renderPalettes(themes);
    });
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', mount);
  } else {
    mount();
  }
})();
