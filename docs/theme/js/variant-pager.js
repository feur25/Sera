(function () {
  var PS = 9;
  var HDR_ID = 'sp-rpg-hdr';

  function railTabs(rail) {
    if (!rail) return [];
    if (rail.classList.contains('sp-cls-rail-side')) return Array.from(rail.querySelectorAll(':scope > .sp-rail-btn'));
    if (rail.classList.contains('sp-cls-rail')) return Array.from(rail.querySelectorAll(':scope > .sp-cls-tab'));
    return [];
  }

  function findRail() {
    return document.querySelector('.sp-cls-rail-side') || document.querySelector('.sp-cls-rail');
  }

  function pg(rail) { return parseInt(rail.dataset.spPg || '0', 10); }

  function syncHdr(page, total) {
    var hdr = document.getElementById(HDR_ID);
    if (!hdr) return;
    hdr.querySelector('.sp-rpg-ind').textContent = (page + 1) + '\u00a0/\u00a0' + total;
    hdr.querySelector('.sp-rpg-prev').disabled = page === 0;
    hdr.querySelector('.sp-rpg-next').disabled = page >= total - 1;
  }

  function draw(rail, page) {
    var bs = railTabs(rail);
    var total = Math.ceil(bs.length / PS);
    page = Math.max(0, Math.min(page, total - 1));
    rail.dataset.spPg = page;
    bs.forEach(function (b, i) {
      b.style.display = (i >= page * PS && i < (page + 1) * PS) ? '' : 'none';
    });
    syncHdr(page, total);
  }

  function ensureVisible(rail) {
    var bs = railTabs(rail);
    var ai = bs.findIndex(function (b) { return b.classList.contains('sp-cact') || b.classList.contains('sp-rail-act'); });
    if (ai < 0) return;
    var tp = Math.floor(ai / PS);
    if (tp !== pg(rail)) draw(rail, tp);
  }

  function initRail(rail) {
    if (rail.dataset.spPaged) return;
    var bs = railTabs(rail);
    if (bs.length <= PS) return;
    rail.dataset.spPaged = '1';
    draw(rail, 0);
  }

  function injectHeader() {
    if (document.getElementById(HDR_ID)) return;
    var tog = document.querySelector('.sp-ph-rail-tog');
    if (!tog) return;
    var rail = findRail();
    if (!rail) return;
    var bs = railTabs(rail);
    if (bs.length <= PS) return;
    var total = Math.ceil(bs.length / PS);
    var hdr = document.createElement('div');
    hdr.id = HDR_ID;
    hdr.className = 'sp-rpg-hdr';
    hdr.setAttribute('role', 'group');
    hdr.innerHTML =
      '<button class="sp-rpg-prev" title="Previous" disabled>\u2039</button>' +
      '<span class="sp-rpg-ind">1\u00a0/\u00a0' + total + '</span>' +
      '<button class="sp-rpg-next" title="Next">\u203a</button>';
    tog.parentNode.insertBefore(hdr, tog.nextSibling);
    hdr.querySelector('.sp-rpg-prev').addEventListener('click', function () {
      var r = findRail(); if (r) draw(r, pg(r) - 1);
    });
    hdr.querySelector('.sp-rpg-next').addEventListener('click', function () {
      var r = findRail(); if (r) draw(r, pg(r) + 1);
    });
    initRail(rail);
  }

  var obs = new MutationObserver(function (muts) {
    muts.forEach(function (m) {
      m.addedNodes.forEach(function (n) {
        if (!n.querySelectorAll) return;
        if (n.classList && (n.classList.contains('sp-cls-rail-side') || n.classList.contains('sp-cls-rail'))) {
          initRail(n);
          setTimeout(injectHeader, 0);
        }
        n.querySelectorAll('.sp-cls-rail-side, .sp-cls-rail').forEach(function (r) { initRail(r); });
        if (n.classList && n.classList.contains('sp-ph-rail-tog')) setTimeout(injectHeader, 0);
        if (n.querySelectorAll) n.querySelectorAll('.sp-ph-rail-tog').forEach(function () { setTimeout(injectHeader, 0); });
      });
      if (m.type === 'attributes' && m.attributeName === 'class') {
        var t = m.target;
        if (t.classList && (t.classList.contains('sp-cact') || t.classList.contains('sp-rail-act'))) {
          var r = findRail(); if (r) ensureVisible(r);
        }
      }
    });
  });

  obs.observe(document.body, { childList: true, subtree: true, attributes: true, attributeFilter: ['class'] });

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', injectHeader);
  } else {
    injectHeader();
  }
})();
