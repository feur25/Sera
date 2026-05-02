(function () {
  var PS = 7;

  function tabs(rail) {
    if (rail.classList.contains('sp-cls-rail')) return Array.from(rail.querySelectorAll(':scope > .sp-cls-tab'));
    if (rail.classList.contains('sp-cls-rail-side')) return Array.from(rail.querySelectorAll(':scope > .sp-rail-btn'));
    return [];
  }

  function pg(rail) { return parseInt(rail.dataset.spPg || '0', 10); }

  function draw(rail, page) {
    var bs = tabs(rail);
    var total = Math.ceil(bs.length / PS);
    page = Math.max(0, Math.min(page, total - 1));
    rail.dataset.spPg = page;
    bs.forEach(function (b, i) {
      b.style.display = (i >= page * PS && i < (page + 1) * PS) ? '' : 'none';
    });
    var ind = rail.querySelector('.sp-rpg-ind');
    if (ind) ind.textContent = (page + 1) + '\u00a0/\u00a0' + total;
    var prev = rail.querySelector('.sp-rpg-prev');
    var nxt = rail.querySelector('.sp-rpg-next');
    if (prev) prev.disabled = page === 0;
    if (nxt) nxt.disabled = page >= total - 1;
  }

  function ensureVisible(rail) {
    var bs = tabs(rail);
    var ai = bs.findIndex(function (b) { return b.classList.contains('sp-cact'); });
    if (ai < 0) return;
    var tp = Math.floor(ai / PS);
    if (tp !== pg(rail)) draw(rail, tp);
  }

  function paginate(rail) {
    if (rail.dataset.spPaged) return;
    var bs = tabs(rail);
    if (bs.length <= PS) return;
    rail.dataset.spPaged = '1';
    var pager = document.createElement('div');
    pager.className = 'sp-rpg';
    pager.innerHTML =
      '<button class="sp-rpg-prev" title="Prev" disabled>\u25b2</button>' +
      '<span class="sp-rpg-ind">1\u00a0/\u00a0' + Math.ceil(bs.length / PS) + '</span>' +
      '<button class="sp-rpg-next" title="Next">\u25bc</button>';
    rail.appendChild(pager);
    pager.querySelector('.sp-rpg-prev').addEventListener('click', function () { draw(rail, pg(rail) - 1); });
    pager.querySelector('.sp-rpg-next').addEventListener('click', function () { draw(rail, pg(rail) + 1); });
    draw(rail, 0);
  }

  function paginateAll() {
    document.querySelectorAll('.sp-cls-rail, .sp-cls-rail-side').forEach(paginate);
  }

  var obs = new MutationObserver(function (muts) {
    muts.forEach(function (m) {
      m.addedNodes.forEach(function (n) {
        if (!n.querySelectorAll) return;
        if (n.classList && (n.classList.contains('sp-cls-rail') || n.classList.contains('sp-cls-rail-side'))) paginate(n);
        n.querySelectorAll('.sp-cls-rail, .sp-cls-rail-side').forEach(paginate);
      });
      if (m.type === 'attributes' && m.attributeName === 'class' && m.target.classList.contains('sp-cact')) {
        var r = m.target.closest ? m.target.closest('.sp-cls-rail, .sp-cls-rail-side') : null;
        if (r) ensureVisible(r);
      }
    });
  });

  obs.observe(document.body, { childList: true, subtree: true, attributes: true, attributeFilter: ['class'] });

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', paginateAll);
  } else {
    paginateAll();
  }
})();
