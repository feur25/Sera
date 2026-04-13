# Grid Layout

## Signature

```python
sp.build_grid(
    charts: list[Chart],
    *,
    cols: int = 2,
    width: int = 1200,
    height: int = 800,
    background: str | None = None,
    gap: int = 12,
    title: str = "",
) -> Chart
```

---

## Description

Arranges multiple charts in a responsive grid layout within a single HTML output.

Arrange plusieurs graphiques dans une mise en page grille responsive dans un seul fichier HTML.

Charts are placed left-to-right, top-to-bottom. When `len(charts)` is not divisible by `cols`, the last row is left-aligned.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `charts` | `list[Chart]` | required | Chart objects to embed |
| `cols` | `int` | `2` | Number of columns |
| `width` | `int` | `1200` | Total grid container width in pixels |
| `height` | `int` | `800` | Total grid container height in pixels |
| `background` | `str \| None` | `None` | Grid background color |
| `gap` | `int` | `12` | Gap in pixels between cells |
| `title` | `str` | `""` | Optional header above the grid |

---

## Returns

`Chart` (composite)

---

## Examples

### Dashboard with 4 charts

```python
import seraplot as sp

bar   = sp.build_bar_chart("Revenue", labels=["A","B","C"], values=[100,200,150])
pie   = sp.build_pie_chart("Share",   labels=["A","B"],     values=[60,40])
line  = sp.build_line_chart("Trend",  labels=["Jan","Feb","Mar"], values=[10,20,15])
hist  = sp.build_histogram("Dist",    values=[1,2,2,3,3,3,4,4,5])

dashboard = sp.build_grid(
    [bar, pie, line, hist],
    cols=2,
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<div style="width:100%;overflow:auto;border-radius:8px;margin:12px 0;background:#0d1117">
<!DOCTYPE html><html><head><meta charset="utf-8"><style>*{box-sizing:border-box}body{margin:0;padding:16px;background:#f0f2f5}.spg{display:grid;grid-template-columns:repeat(2,1fr);gap:16px}.spg-c iframe{width:100%;height:520px;border:none;display:block;border-radius:12px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,.07)}</style><style>.sp-bg{{fill:transparent!important}}.sp-ttl{{fill:#e2e8f0!important}}svg text{{fill:#cbd5e1!important}}.sp-ax-x,.sp-ax-y{{stroke:#475569!important}}.sp-gl{{stroke:#2d3748!important}}.sp-xl,.sp-yl{{fill:#94a3b8!important}}[id^='spp']{{box-shadow:none!important;border-radius:0!important}}</style></head><body><div class="spg"><div class="spg-c"><iframe srcdoc="<!DOCTYPE html><html><head><meta charset=&quot;utf-8&quot;><title>Revenue</title><style>#sp-tip{position:absolute;z-index:999999;pointer-events:none;opacity:0;transition:opacity .15s,transform .15s;transform:translateY(6px) scale(.97);background:#0b0e18;color:#f1f5f9;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;font-size:13px;border-radius:10px;min-width:160px;max-width:340px;box-shadow:0 4px 20px rgba(0,0,0,.45),0 0 0 1px rgba(255,255,255,.08);overflow:hidden}#sp-tip.sp-vis{opacity:1;transform:translateY(0) scale(1);pointer-events:auto}.sp-nav{display:flex;align-items:center;justify-content:space-between;padding:5px 10px;border-top:1px solid rgba(255,255,255,.08)}.sp-nav-btn{cursor:pointer;padding:0 10px;border-radius:5px;height:22px;line-height:22px;font-size:18px;background:rgba(255,255,255,.10);color:#e2e8f0;user-select:none;flex-shrink:0}.sp-nav-btn:hover{background:rgba(255,255,255,.22)}.sp-nav-dis{opacity:.25;pointer-events:none}.sp-nav-ctr{flex:1;text-align:center;font-size:11px;color:#94a3b8}.sp-head{padding:10px 14px 6px;font-weight:700;font-size:14px;color:#e2e8f0;border-bottom:1px solid rgba(255,255,255,.08)}.sp-body{padding:8px 14px 12px}.sp-row{display:flex;justify-content:space-between;align-items:baseline;gap:14px;padding:3px 0;border-bottom:1px solid rgba(255,255,255,.04)}.sp-row:last-child{border-bottom:none}.sp-key{color:#94a3b8;font-size:12px;white-space:nowrap}.sp-val{font-weight:600;font-size:12px;color:#f8fafc;text-align:right;word-break:break-all}#sp-tip img{display:block;width:100%;max-height:210px;object-fit:contain;border-top:1px solid rgba(255,255,255,.07)}#sp-tip video{display:block;width:100%;border-top:1px solid rgba(255,255,255,.07)}.sp-html{padding:8px 14px;font-size:12px;border-top:1px solid rgba(255,255,255,.07)}[data-idx]{cursor:pointer;transition:opacity .25s,filter .2s,transform .25s}[data-idx]:hover{filter:brightness(1.12) saturate(1.08)}.sp-cpanel{position:absolute;bottom:10px;left:50%;transform:translateX(-50%);background:#0b0e18;color:#f1f5f9;border-radius:10px;padding:8px 16px;font-size:12px;font-family:-apple-system,Arial,sans-serif;box-shadow:0 8px 24px rgba(0,0,0,.4);z-index:20;white-space:nowrap;display:none}.sp-cls-x{cursor:pointer;color:#94a3b8;margin-left:6px;font-size:13px}.sp-cls-x:hover{color:#f87171}</style><style>body{margin:0;background:#fff;display:flex;justify-content:center;padding:0}@keyframes sp-i{from{opacity:0;transform:translateY(8px) scale(.94)}}@keyframes sp-bar{from{opacity:0;transform:scaleY(0)}}@keyframes sp-pop{0%{opacity:0;transform:scale(0)}70%{transform:scale(1.08)}100%{opacity:1;transform:scale(1)}}@keyframes sp-arc{from{opacity:0;transform:scale(.82) rotate(-6deg)}}@keyframes sp-fn{from{opacity:0;transform:scaleX(.7) translateY(8px)}}svg rect[data-idx]{transform-box:fill-box;transform-origin:bottom center;animation:sp-bar .5s cubic-bezier(.22,.61,.36,1) both}svg circle[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-pop .42s cubic-bezier(.34,1.56,.64,1) both}svg path[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-arc .48s cubic-bezier(.22,.61,.36,1) both}svg polygon[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-fn .48s cubic-bezier(.22,.61,.36,1) both}svg line[data-idx]{animation:sp-i .45s cubic-bezier(.22,.61,.36,1) both}svg rect[data-idx]:hover{transform:scaleY(1.03);filter:brightness(1.12) saturate(1.1)}svg circle[data-idx]:hover{transform:scale(1.3);filter:brightness(1.15)}svg path[data-idx]:hover{filter:brightness(1.1) drop-shadow(0 2px 8px rgba(0,0,0,.2))}.sp-bg{fill:#fff}</style></head><body><div id=&quot;spp1&quot; style=&quot;position:relative;display:inline-block;border-radius:12px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,.07),0 0 0 1px rgba(0,0,0,.04)&quot;><svg xmlns=&quot;http://www.w3.org/2000/svg&quot; width=&quot;900&quot; height=&quot;480&quot; viewBox=&quot;0 0 900 480&quot; data-sp=&quot;52,36,828,396&quot;><rect class=&quot;sp-bg&quot; width=&quot;100%&quot; height=&quot;100%&quot;/><text x=&quot;466&quot; y=&quot;24&quot; text-anchor=&quot;middle&quot; font-family=&quot;-apple-system,Arial,sans-serif&quot; font-size=&quot;15&quot; font-weight=&quot;700&quot; fill=&quot;#1a202c&quot; class=&quot;sp-ttl&quot;>Revenue</text><text x=&quot;48&quot; y=&quot;435&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>0.00</text><text x=&quot;48&quot; y=&quot;355&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>40.00</text><text x=&quot;48&quot; y=&quot;276&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>80.00</text><text x=&quot;48&quot; y=&quot;197&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>120.00</text><text x=&quot;48&quot; y=&quot;118&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>160.00</text><text x=&quot;48&quot; y=&quot;39&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>200.00</text><line x1=&quot;52&quot; y1=&quot;36&quot; x2=&quot;52&quot; y2=&quot;432&quot; stroke=&quot;#cbd5e1&quot; stroke-width=&quot;1&quot; class=&quot;sp-ax-y&quot;/><line x1=&quot;52&quot; y1=&quot;432&quot; x2=&quot;880&quot; y2=&quot;432&quot; stroke=&quot;#cbd5e1&quot; stroke-width=&quot;1&quot; class=&quot;sp-ax-x&quot;/><rect data-idx=&quot;0&quot; data-series=&quot;0&quot; data-v=&quot;100.00&quot; data-lbl=&quot;A&quot; x=&quot;52&quot; y=&quot;234&quot; width=&quot;275&quot; height=&quot;198&quot; fill=&quot;#6366f1&quot; rx=&quot;2&quot;/><text x=&quot;189&quot; y=&quot;446&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#6b7280&quot;>A</text><rect data-idx=&quot;1&quot; data-series=&quot;1&quot; data-v=&quot;200.00&quot; data-lbl=&quot;B&quot; x=&quot;328&quot; y=&quot;36&quot; width=&quot;275&quot; height=&quot;396&quot; fill=&quot;#f43f5e&quot; rx=&quot;2&quot;/><text x=&quot;465&quot; y=&quot;446&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#6b7280&quot;>B</text><rect data-idx=&quot;2&quot; data-series=&quot;2&quot; data-v=&quot;150.00&quot; data-lbl=&quot;C&quot; x=&quot;604&quot; y=&quot;135&quot; width=&quot;275&quot; height=&quot;297&quot; fill=&quot;#10b981&quot; rx=&quot;2&quot;/><text x=&quot;741&quot; y=&quot;446&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#6b7280&quot;>C</text></svg><div class=&quot;sp-sel-ov&quot; style=&quot;display:none&quot;></div><div class=&quot;sp-cpanel&quot;></div><script>(function(){
var wrap=document.getElementById('spp1');if(!wrap)return;wrap.removeAttribute('id');
var svg=wrap.querySelector('svg');var data=[];

var tip=document.getElementById('sp-tip');
if(!tip){tip=document.createElement('div');tip.id='sp-tip';document.body.appendChild(tip);}
var tipIdxs=[],tipPos=0,lastE=null;
function esc(s){return String(s).replace(/&amp;/g,'&amp;amp;').replace(/</g,'&amp;lt;');}
function cardHTML(d){
 var h='';if(d.title)h+='<div class=&quot;sp-head&quot;>'+esc(d.title)+'</div>';
 var rows='';(d.kv||[]).forEach(function(p){
  rows+='<div class=&quot;sp-row&quot;><span class=&quot;sp-key&quot;>'+esc(p[0])+'</span><span class=&quot;sp-val&quot;>'+esc(p[1])+'</span></div>';});
 if(rows)h+='<div class=&quot;sp-body&quot;>'+rows+'</div>';
 if(d.image)h+='<img src=&quot;'+esc(d.image)+'&quot; alt=&quot;&quot; loading=&quot;lazy&quot;>';
 if(d.video)h+='<video src=&quot;'+esc(d.video)+'&quot; controls muted playsinline></video>';
 if(d.html)h+='<div class=&quot;sp-html&quot;>'+d.html+'</div>';
 return h;}
function placeTip(e){
 var sx=window.scrollX||0,sy=window.scrollY||0;
 var x=e.pageX+16,y=e.pageY-14;
 var tw=tip.offsetWidth||220,th=tip.offsetHeight||60;
 if(x+tw>sx+window.innerWidth-6)x=e.pageX-tw-16;
 if(x<sx+4)x=sx+4;
 if(y<sy+6)y=e.pageY+20;
 if(y+th>sy+window.innerHeight-6)y=sy+window.innerHeight-th-6;
 tip.style.left=x+'px';tip.style.top=y+'px';}

function getSlot(idx){
 var d=data[idx];if(d)return d;
 var el=svg.querySelector('[data-idx=&quot;'+idx+'&quot;]');if(!el)return null;
 var kv=[];
 var x=el.getAttribute('data-x'),y=el.getAttribute('data-y');
 if(x!=null){var xf=parseFloat(x);kv.push(['X',xf===xf?xf.toFixed(2):x]);}
 if(y!=null){kv.push(['Valeur',parseFloat(y).toFixed(2)]);}
 else{var v=el.getAttribute('data-v');
 if(v!=null){
  var r=el.getAttribute('data-r'),c=el.getAttribute('data-c');
  kv.push(['Valeur',parseFloat(v).toFixed(3)]);
  if(r)kv.push(['Ligne',r]);if(c)kv.push(['Colonne',c]);}}
 var z=el.getAttribute('data-z');
 if(z!=null){kv.push(['Z',parseFloat(z).toFixed(2)]);}
 var lbl=el.getAttribute('data-lbl');
 var an=el.attributes;if(an){for(var ai=0;ai<an.length;ai++){var a=an[ai];if(a.name.substring(0,8)==='data-kv-'){kv.push([a.name.substring(8),a.value]);}}}
 var title=lbl||(el.getAttribute('data-v')!=null&amp;&amp;el.getAttribute('data-r')!=null?el.getAttribute('data-r')+' \u00d7 '+el.getAttribute('data-c'):'Point '+(idx+1));
 return{title:title,kv:kv};}
function renderTip(){
 if(!tipIdxs.length){tip.classList.remove('sp-vis');return;}
 var d=getSlot(tipIdxs[tipPos]);if(!d){tip.classList.remove('sp-vis');return;}
 var h=cardHTML(d);
 if(tipIdxs.length>1){
  var p=tipPos,n=tipIdxs.length;
  h+='<div class=&quot;sp-nav&quot;>'
   +'<span class=&quot;sp-nav-btn'+(p>0?'':' sp-nav-dis')+'&quot; data-d=&quot;-1&quot;>&amp;#8249;</span>'
   +'<span class=&quot;sp-nav-ctr&quot;>'+(p+1)+' / '+n+'</span>'
   +'<span class=&quot;sp-nav-btn'+(p<n-1?'':' sp-nav-dis')+'&quot; data-d=&quot;1&quot;>&amp;#8250;</span>'
   +'</div>';}
 tip.innerHTML=h;tip.classList.add('sp-vis');
 if(lastE)placeTip(lastE);
 tip.querySelectorAll('.sp-nav-btn:not(.sp-nav-dis)').forEach(function(btn){
  btn.onclick=function(ev){ev.stopPropagation();
   var nd=tipPos+parseInt(btn.getAttribute('data-d'),10);
   if(nd>=0&amp;&amp;nd<tipIdxs.length){tipPos=nd;renderTip();}};});}
var dragging=false,dsx=0,dsy=0,moved=false,pinned=false;

wrap.addEventListener('mouseleave',function(e){
 if(pinned)return;
 var rt=e.relatedTarget;
 if(rt&amp;&amp;(rt===tip||tip.contains(rt)))return;
 tip.classList.remove('sp-vis');tipIdxs=[];});
wrap.addEventListener('mousemove',function(e){
 if(dragging||pinned)return;
 lastE=e;

 var hits=[];
 document.elementsFromPoint(e.clientX,e.clientY).forEach(function(el){
  if(el===tip||tip.contains(el))return;
  var found=null;
  for(var n=el;n&amp;&amp;n!==document.body;n=n.parentElement){
   if(found===null&amp;&amp;n.getAttribute&amp;&amp;n.getAttribute('data-idx')!==null)found=n;
   if(n===wrap){
    if(found!==null){var idx=parseInt(found.getAttribute('data-idx'),10);
     if(hits.indexOf(idx)===-1)hits.push(idx);}
    return;}
  }
 });
 if(hits.length){
  var same=hits.length===tipIdxs.length&amp;&amp;hits.every(function(v,i){return v===tipIdxs[i];});
  if(!same){tipIdxs=hits;tipPos=0;renderTip();}else placeTip(e);
 }else{tip.classList.remove('sp-vis');tipIdxs=[];}});

var origVB=svg.getAttribute('viewBox')||'';
function parseVB(s){return s.split(/[\s,]+/).map(Number);}
function animateVB(x,y,w,h){
 svg.style.transition='all 0.5s cubic-bezier(.4,0,.2,1)';
 svg.setAttribute('viewBox',x+' '+y+' '+w+' '+h);}
function resetVB(){
 svg.style.transition='all 0.45s cubic-bezier(.4,0,.2,1)';
 if(origVB)svg.setAttribute('viewBox',origVB);
 setTimeout(function(){svg.style.transition='';},500);}
function reAnim(){var els=svg.querySelectorAll('[data-idx]');els.forEach(function(el){el.style.animation='none';el.style.filter='';});void svg.offsetHeight;els.forEach(function(el,i){el.style.animation='';el.style.animationDelay=(i*14)+'ms';});}
var ov=wrap.querySelector('.sp-sel-ov');var panel=wrap.querySelector('.sp-cpanel');
wrap.addEventListener('mousedown',function(e){
 if(e.button!==0)return;
 var t=e.target;if(t===panel||panel.contains(t))return;if(t===tip||tip.contains(t))return;
 dragging=true;moved=false;
 var r=wrap.getBoundingClientRect();dsx=e.clientX-r.left;dsy=e.clientY-r.top;
 ov.style.cssText='display:none;position:absolute;pointer-events:none;border:2px solid #6366F1;background:rgba(99,102,241,.12);box-sizing:border-box;border-radius:3px;z-index:10';
 e.preventDefault();});
document.addEventListener('mousemove',function(e){if(!dragging)return;
 var r=wrap.getBoundingClientRect();var cx=e.clientX-r.left,cy=e.clientY-r.top;
 if(!moved&amp;&amp;Math.abs(cx-dsx)<8&amp;&amp;Math.abs(cy-dsy)<8)return;
 if(!moved){pinned=false;tip.classList.remove('sp-vis');tipIdxs=[];}
 moved=true;ov.style.display='block';
 ov.style.left=Math.min(dsx,cx)+'px';ov.style.top=Math.min(dsy,cy)+'px';
 ov.style.width=Math.abs(cx-dsx)+'px';ov.style.height=Math.abs(cy-dsy)+'px';});
document.addEventListener('mouseup',function(e){if(!dragging)return;dragging=false;
 ov.style.display='none';if(!moved){
 var ch=[];document.elementsFromPoint(e.clientX,e.clientY).forEach(function(el){if(el===tip||tip.contains(el))return;var fd=null;for(var n2=el;n2&amp;&amp;n2!==document.body;n2=n2.parentElement){if(fd===null&amp;&amp;n2.getAttribute&amp;&amp;n2.getAttribute('data-idx')!==null)fd=n2;if(n2===wrap){if(fd!==null){var idx=parseInt(fd.getAttribute('data-idx'),10);if(ch.indexOf(idx)===-1)ch.push(idx);}return;}}});
 if(ch.length){pinned=true;tipIdxs=ch;tipPos=0;lastE=e;renderTip();}else{pinned=false;tip.classList.remove('sp-vis');tipIdxs=[];}return;}
 var r=wrap.getBoundingClientRect();var cx=e.clientX-r.left,cy=e.clientY-r.top;
 var rx1=Math.min(dsx,cx),ry1=Math.min(dsy,cy),rx2=Math.max(dsx,cx),ry2=Math.max(dsy,cy);
 if(rx2-rx1<8&amp;&amp;ry2-ry1<8)return;
 var ctm=svg.getScreenCTM();if(!ctm)return;var inv=ctm.inverse();
 function toS(px,py){var pt=svg.createSVGPoint();pt.x=px+r.left;pt.y=py+r.top;return pt.matrixTransform(inv);}
 var p1=toS(rx1,ry1),p2=toS(rx2,ry2);
 var bx1=Math.min(p1.x,p2.x),by1=Math.min(p1.y,p2.y),bx2=Math.max(p1.x,p2.x),by2=Math.max(p1.y,p2.y);
 var pts=svg.querySelectorAll('[data-idx]');var sel=[],unsel=[];
 pts.forEach(function(el){try{var bb=el.getBBox();var ecx=bb.x+bb.width/2,ecy=bb.y+bb.height/2;
  if(ecx>=bx1&amp;&amp;ecx<=bx2&amp;&amp;ecy>=by1&amp;&amp;ecy<=by2)sel.push(el);else unsel.push(el);}catch(ex){unsel.push(el);}});
 if(!sel.length)return;
 sel.forEach(function(el){el.style.stroke='#6366F1';el.style.strokeWidth='2.5';el.style.opacity='';el.style.transform='';});
 unsel.forEach(function(el){
  el.style.transformBox='fill-box';el.style.transformOrigin='center';
  el.style.transition='transform 0.28s cubic-bezier(.4,0,.2,1),opacity 0.28s';
  el.style.transform='scale(0,0)';el.style.opacity='0';
  setTimeout(function(){el.style.display='none';el.style.transition='';},300);});

 var mnx=Infinity,mny=Infinity,mxx=-Infinity,mxy=-Infinity;
 sel.forEach(function(el){try{var bb=el.getBBox();
  mnx=Math.min(mnx,bb.x);mny=Math.min(mny,bb.y);
  mxx=Math.max(mxx,bb.x+bb.width);mxy=Math.max(mxy,bb.y+bb.height);}catch(ex){}});
 var vb=parseVB(origVB.length?origVB:(svg.getAttribute('viewBox')||'0 0 800 500'));
 var axL=mnx-vb[0],axT=mny-vb[1],axR=(vb[0]+vb[2])-mxx,axB=(vb[1]+vb[3])-mxy;
 var sw=mxx-mnx||1,sh=mxy-mny||1;
 var pL=Math.max(sw*0.12,axL>0?axL*0.7:sw*0.12);
 var pR=Math.max(sw*0.08,axR>0?axR*0.5:sw*0.08);
 var pT=Math.max(sh*0.12,axT>0?axT*0.7:sh*0.12);
 var pB=Math.max(sh*0.20,axB>0?axB*0.8:sh*0.20);
 animateVB(mnx-pL,mny-pT,(mxx+pR)-(mnx-pL),(mxy+pB)-(mny-pT));

 var xs=sel.map(function(el){return parseFloat(el.getAttribute('data-x'));}).filter(Number.isFinite);
 var ys=sel.map(function(el){return parseFloat(el.getAttribute('data-y'));}).filter(Number.isFinite);
 var s='<span style=&quot;color:#6366F1;font-weight:700&quot;>'+sel.length+' pts</span>';
 if(xs.length&amp;&amp;ys.length){
  var mx=xs.reduce(function(a,b){return a+b;})/xs.length;
  var my=ys.reduce(function(a,b){return a+b;})/ys.length;
  var vX=xs.map(function(v){return(v-mx)*(v-mx);}).reduce(function(a,b){return a+b;})/xs.length;
  var vY=ys.map(function(v){return(v-my)*(v-my);}).reduce(function(a,b){return a+b;})/ys.length;
  s+=' &amp;middot; X&amp;#772; <b>'+mx.toFixed(2)+'</b> &amp;plusmn;'+Math.sqrt(vX).toFixed(2);
  s+=' &amp;middot; Y&amp;#772; <b>'+my.toFixed(2)+'</b> &amp;plusmn;'+Math.sqrt(vY).toFixed(2);}
 s+=' <span class=&quot;sp-cls-x&quot;>&amp;#10005;</span>';
 panel.innerHTML=s;panel.style.display='block';
 panel.querySelector('.sp-cls-x').addEventListener('click',clearSel);});
function clearSel(){panel.style.display='none';resetVB();
 svg.querySelectorAll('[data-idx]').forEach(function(el){
  el.style.display='';el.style.opacity='';el.style.stroke='';el.style.strokeWidth='';el.style.filter='';
  el.style.transform='';el.style.transition='';el.style.transformBox='';el.style.transformOrigin='';});
 reAnim();}
document.addEventListener('keydown',function(e){if(e.key==='Escape'){if(dblZoomed){dblZoomed=false;resetVB();svg.querySelectorAll('[data-idx]').forEach(function(el){el.style.opacity='';el.style.display='';el.style.transform='';el.style.transition='';});reAnim();}if(pinned){pinned=false;tip.classList.remove('sp-vis');tipIdxs=[];}}});
var dblZoomed=false;
svg.addEventListener('dblclick',function(e){
 if(dblZoomed){dblZoomed=false;resetVB();svg.querySelectorAll('[data-idx]').forEach(function(el){el.style.opacity='';el.style.display='';el.style.transform='';el.style.transition='';el.style.filter='';});reAnim();return;}
 var found=null;
 for(var nd=e.target;nd&amp;&amp;nd!==svg;nd=nd.parentElement){if(nd.getAttribute&amp;&amp;nd.getAttribute('data-idx')!==null){found=nd;break;}}
 if(!found)return;
 e.stopPropagation();
 var idx=parseInt(found.getAttribute('data-idx'),10);
 var bb;try{bb=found.getBBox();}catch(ex){return;}
 var pad=Math.max(bb.width,bb.height)*1.5+30;
 svg.querySelectorAll('[data-idx]').forEach(function(el){
  var eli=parseInt(el.getAttribute('data-idx'),10);
  if(eli===idx){el.style.opacity='1';el.style.filter='drop-shadow(0 0 6px rgba(99,102,241,.45))';}else{el.style.display='none';}});
 var vb=parseVB(origVB.length?origVB:(svg.getAttribute('viewBox')||'0 0 800 500'));
 var nx=Math.max(vb[0],bb.x-pad),ny=Math.max(vb[1],bb.y-pad);
 var nw=Math.min(vb[2],bb.width+pad*2),nh=Math.min(vb[3],bb.height+pad*2);
 animateVB(nx,ny,nw,nh);
 dblZoomed=true;
},false);
var legs=svg.querySelectorAll('[data-legend]');
legs.forEach(function(lg){lg.style.cursor='pointer';
lg.addEventListener('click',function(){
 var s=lg.getAttribute('data-series');if(!s)return;
 var h=lg.getAttribute('data-hidden')==='1';
 var els=svg.querySelectorAll('[data-series=&quot;'+s+'&quot;]:not([data-legend])');
 if(h){els.forEach(function(el){el.style.display='';el.style.opacity='';el.style.transform='';el.style.transformBox='';el.style.transformOrigin='';el.style.transition='';});lg.style.opacity='';lg.removeAttribute('data-hidden');}
 else{els.forEach(function(el){
  el.style.transformBox='fill-box';el.style.transformOrigin='center';
  el.style.transition='transform 0.3s cubic-bezier(.4,0,.2,1),opacity 0.3s';
  el.style.transform='scale(0,0)';el.style.opacity='0';
  setTimeout(function(){el.style.display='none';el.style.transition='';},320);});
  lg.style.opacity='0.3';lg.setAttribute('data-hidden','1');}spRescale();});});
function spRescale(){
 var m=svg.getAttribute('data-sp');if(!m)return;
 var p=m.split(',').map(Number),pL=p[0],pT=p[1],pW=p[2],pH=p[3];
 var vals=[];
 svg.querySelectorAll('[data-pts]').forEach(function(el){if(el.style.display==='none')return;el.getAttribute('data-pts').split(',').forEach(function(v){var f=parseFloat(v);if(isFinite(f))vals.push(f);});});
 svg.querySelectorAll('circle[data-y]').forEach(function(el){if(el.style.display==='none')return;var f=parseFloat(el.getAttribute('data-y'));if(isFinite(f))vals.push(f);});
 svg.querySelectorAll('rect[data-y]').forEach(function(el){if(el.style.display==='none')return;var f=parseFloat(el.getAttribute('data-y'));if(isFinite(f))vals.push(f);});
 svg.querySelectorAll('rect[data-v]').forEach(function(el){if(el.style.display==='none')return;if(el.hasAttribute('data-y'))return;var f=parseFloat(el.getAttribute('data-v'));if(isFinite(f))vals.push(f);});
 if(!vals.length)return;
 var mn=Math.min.apply(null,vals),mx=Math.max.apply(null,vals);if(mn>0)mn=0;var rg=mx-mn;if(rg<1e-12)rg=1;
 svg.querySelectorAll('polyline[data-pts]').forEach(function(el){if(el.style.display==='none')return;var vs=el.getAttribute('data-pts').split(','),n=vs.length,sx=pW/Math.max(n-1,1),r='';for(var i=0;i<n;i++){var f=(parseFloat(vs[i])-mn)/rg;if(i>0)r+=' ';r+=(pL+Math.round(i*sx))+','+(pT+Math.round((1-f)*pH));}el.setAttribute('points',r);});
 svg.querySelectorAll('circle[data-y]').forEach(function(el){if(el.style.display==='none')return;var f=(parseFloat(el.getAttribute('data-y'))-mn)/rg;el.setAttribute('cy',pT+Math.round((1-f)*pH));});
 svg.querySelectorAll('rect[data-y]').forEach(function(el){if(el.style.display==='none')return;var v=parseFloat(el.getAttribute('data-y'));if(!isFinite(v))return;var f=(v-mn)/rg;var ny=pT+Math.round((1-f)*pH);var base=pT+pH;if(mn<0){base=pT+Math.round((0-mn)/rg*pH);}if(v>=0){el.setAttribute('y',ny);el.setAttribute('height',Math.max(1,base-ny));}else{el.setAttribute('y',base);el.setAttribute('height',Math.max(1,ny-base));}});
 svg.querySelectorAll('rect[data-v]').forEach(function(el){if(el.style.display==='none')return;if(el.hasAttribute('data-y'))return;var v=parseFloat(el.getAttribute('data-v'));if(!isFinite(v))return;var f=(v-mn)/rg;var ny=pT+Math.round((1-f)*pH);var base=pT+pH;if(mn<0){base=pT+Math.round((0-mn)/rg*pH);}if(v>=0){el.setAttribute('y',ny);el.setAttribute('height',Math.max(1,base-ny));}else{el.setAttribute('y',base);el.setAttribute('height',Math.max(1,ny-base));}});
 var tks=svg.querySelectorAll('.sp-yt'),nT=tks.length;if(nT>1)for(var i=0;i<nT;i++){var f=i/(nT-1);var v=mn+f*rg;tks[i].textContent=v>=1000?Math.round(v)+'':v.toFixed(2);tks[i].setAttribute('y',pT+Math.round((1-f)*pH)+3);}
 var gls=svg.querySelectorAll('.sp-gl');if(gls.length&amp;&amp;nT>1)for(var j=0;j<gls.length;j++){var f=(j+1)/(nT-1);var gy=pT+Math.round((1-f)*pH);gls[j].setAttribute('y1',gy);gls[j].setAttribute('y2',gy);}
}
svg.querySelectorAll('[data-idx]').forEach(function(el,i){el.style.animationDelay=(i*18)+'ms';});
})();</script></div></body></html>"></iframe></div><div class="spg-c"><iframe srcdoc="<!DOCTYPE html><html><head><meta charset=&quot;utf-8&quot;><title>Share</title><style>#sp-tip{position:absolute;z-index:999999;pointer-events:none;opacity:0;transition:opacity .15s,transform .15s;transform:translateY(6px) scale(.97);background:#0b0e18;color:#f1f5f9;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;font-size:13px;border-radius:10px;min-width:160px;max-width:340px;box-shadow:0 4px 20px rgba(0,0,0,.45),0 0 0 1px rgba(255,255,255,.08);overflow:hidden}#sp-tip.sp-vis{opacity:1;transform:translateY(0) scale(1);pointer-events:auto}.sp-nav{display:flex;align-items:center;justify-content:space-between;padding:5px 10px;border-top:1px solid rgba(255,255,255,.08)}.sp-nav-btn{cursor:pointer;padding:0 10px;border-radius:5px;height:22px;line-height:22px;font-size:18px;background:rgba(255,255,255,.10);color:#e2e8f0;user-select:none;flex-shrink:0}.sp-nav-btn:hover{background:rgba(255,255,255,.22)}.sp-nav-dis{opacity:.25;pointer-events:none}.sp-nav-ctr{flex:1;text-align:center;font-size:11px;color:#94a3b8}.sp-head{padding:10px 14px 6px;font-weight:700;font-size:14px;color:#e2e8f0;border-bottom:1px solid rgba(255,255,255,.08)}.sp-body{padding:8px 14px 12px}.sp-row{display:flex;justify-content:space-between;align-items:baseline;gap:14px;padding:3px 0;border-bottom:1px solid rgba(255,255,255,.04)}.sp-row:last-child{border-bottom:none}.sp-key{color:#94a3b8;font-size:12px;white-space:nowrap}.sp-val{font-weight:600;font-size:12px;color:#f8fafc;text-align:right;word-break:break-all}#sp-tip img{display:block;width:100%;max-height:210px;object-fit:contain;border-top:1px solid rgba(255,255,255,.07)}#sp-tip video{display:block;width:100%;border-top:1px solid rgba(255,255,255,.07)}.sp-html{padding:8px 14px;font-size:12px;border-top:1px solid rgba(255,255,255,.07)}[data-idx]{cursor:pointer;transition:opacity .25s,filter .2s,transform .25s}[data-idx]:hover{filter:brightness(1.12) saturate(1.08)}.sp-cpanel{position:absolute;bottom:10px;left:50%;transform:translateX(-50%);background:#0b0e18;color:#f1f5f9;border-radius:10px;padding:8px 16px;font-size:12px;font-family:-apple-system,Arial,sans-serif;box-shadow:0 8px 24px rgba(0,0,0,.4);z-index:20;white-space:nowrap;display:none}.sp-cls-x{cursor:pointer;color:#94a3b8;margin-left:6px;font-size:13px}.sp-cls-x:hover{color:#f87171}</style><style>body{margin:0;background:#fff;display:flex;justify-content:center;padding:0}@keyframes sp-i{from{opacity:0;transform:translateY(8px) scale(.94)}}@keyframes sp-bar{from{opacity:0;transform:scaleY(0)}}@keyframes sp-pop{0%{opacity:0;transform:scale(0)}70%{transform:scale(1.08)}100%{opacity:1;transform:scale(1)}}@keyframes sp-arc{from{opacity:0;transform:scale(.82) rotate(-6deg)}}@keyframes sp-fn{from{opacity:0;transform:scaleX(.7) translateY(8px)}}svg rect[data-idx]{transform-box:fill-box;transform-origin:bottom center;animation:sp-bar .5s cubic-bezier(.22,.61,.36,1) both}svg circle[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-pop .42s cubic-bezier(.34,1.56,.64,1) both}svg path[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-arc .48s cubic-bezier(.22,.61,.36,1) both}svg polygon[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-fn .48s cubic-bezier(.22,.61,.36,1) both}svg line[data-idx]{animation:sp-i .45s cubic-bezier(.22,.61,.36,1) both}svg rect[data-idx]:hover{transform:scaleY(1.03);filter:brightness(1.12) saturate(1.1)}svg circle[data-idx]:hover{transform:scale(1.3);filter:brightness(1.15)}svg path[data-idx]:hover{filter:brightness(1.1) drop-shadow(0 2px 8px rgba(0,0,0,.2))}.sp-bg{fill:#fff}</style></head><body><div id=&quot;spp2&quot; style=&quot;position:relative;display:inline-block;border-radius:12px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,.07),0 0 0 1px rgba(0,0,0,.04)&quot;><svg xmlns=&quot;http://www.w3.org/2000/svg&quot; width=&quot;720&quot; height=&quot;440&quot; viewBox=&quot;0 0 720 440&quot;><rect class=&quot;sp-bg&quot; width=&quot;100%&quot; height=&quot;100%&quot;/><text x=&quot;360&quot; y=&quot;22&quot; text-anchor=&quot;middle&quot; font-family=&quot;-apple-system,Arial,sans-serif&quot; font-size=&quot;15&quot; font-weight=&quot;700&quot; fill=&quot;#1a202c&quot;>Share</text><path data-idx=&quot;0&quot; d=&quot;M223.20,228.80 L223.20,55.83 A172.97,172.97 0 1,1 121.53,368.74 Z&quot; data-lbl=&quot;A&quot; data-v=&quot;60.00&quot; data-kv-Part=&quot;60.00%&quot; fill=&quot;#6366f1&quot; stroke=&quot;#fff&quot; stroke-width=&quot;1.8&quot;/><text x=&quot;331.77&quot; y=&quot;264.08&quot; text-anchor=&quot;middle&quot; dominant-baseline=&quot;central&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;11&quot; font-weight=&quot;700&quot; fill=&quot;#fff&quot;>60%</text><path data-idx=&quot;1&quot; d=&quot;M223.20,228.80 L121.53,368.74 A172.97,172.97 0 0,1 223.20,55.83 Z&quot; data-lbl=&quot;B&quot; data-v=&quot;40.00&quot; data-kv-Part=&quot;40.00%&quot; fill=&quot;#f43f5e&quot; stroke=&quot;#fff&quot; stroke-width=&quot;1.8&quot;/><text x=&quot;114.63&quot; y=&quot;193.52&quot; text-anchor=&quot;middle&quot; dominant-baseline=&quot;central&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;11&quot; font-weight=&quot;700&quot; fill=&quot;#fff&quot;>40%</text><g data-legend=&quot;1&quot; data-series=&quot;0&quot;><rect x=&quot;475&quot; y=&quot;198&quot; width=&quot;13&quot; height=&quot;13&quot; rx=&quot;3&quot; fill=&quot;#6366f1&quot;/><text x=&quot;492&quot; y=&quot;209&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;11&quot; fill=&quot;#374151&quot;>A (60.00%)</text></g><g data-legend=&quot;1&quot; data-series=&quot;1&quot;><rect x=&quot;475&quot; y=&quot;220&quot; width=&quot;13&quot; height=&quot;13&quot; rx=&quot;3&quot; fill=&quot;#f43f5e&quot;/><text x=&quot;492&quot; y=&quot;231&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;11&quot; fill=&quot;#374151&quot;>B (40.00%)</text></g></svg><div class=&quot;sp-sel-ov&quot; style=&quot;display:none&quot;></div><div class=&quot;sp-cpanel&quot;></div><script>(function(){
var wrap=document.getElementById('spp2');if(!wrap)return;wrap.removeAttribute('id');
var svg=wrap.querySelector('svg');var data=[];

var tip=document.getElementById('sp-tip');
if(!tip){tip=document.createElement('div');tip.id='sp-tip';document.body.appendChild(tip);}
var tipIdxs=[],tipPos=0,lastE=null;
function esc(s){return String(s).replace(/&amp;/g,'&amp;amp;').replace(/</g,'&amp;lt;');}
function cardHTML(d){
 var h='';if(d.title)h+='<div class=&quot;sp-head&quot;>'+esc(d.title)+'</div>';
 var rows='';(d.kv||[]).forEach(function(p){
  rows+='<div class=&quot;sp-row&quot;><span class=&quot;sp-key&quot;>'+esc(p[0])+'</span><span class=&quot;sp-val&quot;>'+esc(p[1])+'</span></div>';});
 if(rows)h+='<div class=&quot;sp-body&quot;>'+rows+'</div>';
 if(d.image)h+='<img src=&quot;'+esc(d.image)+'&quot; alt=&quot;&quot; loading=&quot;lazy&quot;>';
 if(d.video)h+='<video src=&quot;'+esc(d.video)+'&quot; controls muted playsinline></video>';
 if(d.html)h+='<div class=&quot;sp-html&quot;>'+d.html+'</div>';
 return h;}
function placeTip(e){
 var sx=window.scrollX||0,sy=window.scrollY||0;
 var x=e.pageX+16,y=e.pageY-14;
 var tw=tip.offsetWidth||220,th=tip.offsetHeight||60;
 if(x+tw>sx+window.innerWidth-6)x=e.pageX-tw-16;
 if(x<sx+4)x=sx+4;
 if(y<sy+6)y=e.pageY+20;
 if(y+th>sy+window.innerHeight-6)y=sy+window.innerHeight-th-6;
 tip.style.left=x+'px';tip.style.top=y+'px';}

function getSlot(idx){
 var d=data[idx];if(d)return d;
 var el=svg.querySelector('[data-idx=&quot;'+idx+'&quot;]');if(!el)return null;
 var kv=[];
 var x=el.getAttribute('data-x'),y=el.getAttribute('data-y');
 if(x!=null){var xf=parseFloat(x);kv.push(['X',xf===xf?xf.toFixed(2):x]);}
 if(y!=null){kv.push(['Valeur',parseFloat(y).toFixed(2)]);}
 else{var v=el.getAttribute('data-v');
 if(v!=null){
  var r=el.getAttribute('data-r'),c=el.getAttribute('data-c');
  kv.push(['Valeur',parseFloat(v).toFixed(3)]);
  if(r)kv.push(['Ligne',r]);if(c)kv.push(['Colonne',c]);}}
 var z=el.getAttribute('data-z');
 if(z!=null){kv.push(['Z',parseFloat(z).toFixed(2)]);}
 var lbl=el.getAttribute('data-lbl');
 var an=el.attributes;if(an){for(var ai=0;ai<an.length;ai++){var a=an[ai];if(a.name.substring(0,8)==='data-kv-'){kv.push([a.name.substring(8),a.value]);}}}
 var title=lbl||(el.getAttribute('data-v')!=null&amp;&amp;el.getAttribute('data-r')!=null?el.getAttribute('data-r')+' \u00d7 '+el.getAttribute('data-c'):'Point '+(idx+1));
 return{title:title,kv:kv};}
function renderTip(){
 if(!tipIdxs.length){tip.classList.remove('sp-vis');return;}
 var d=getSlot(tipIdxs[tipPos]);if(!d){tip.classList.remove('sp-vis');return;}
 var h=cardHTML(d);
 if(tipIdxs.length>1){
  var p=tipPos,n=tipIdxs.length;
  h+='<div class=&quot;sp-nav&quot;>'
   +'<span class=&quot;sp-nav-btn'+(p>0?'':' sp-nav-dis')+'&quot; data-d=&quot;-1&quot;>&amp;#8249;</span>'
   +'<span class=&quot;sp-nav-ctr&quot;>'+(p+1)+' / '+n+'</span>'
   +'<span class=&quot;sp-nav-btn'+(p<n-1?'':' sp-nav-dis')+'&quot; data-d=&quot;1&quot;>&amp;#8250;</span>'
   +'</div>';}
 tip.innerHTML=h;tip.classList.add('sp-vis');
 if(lastE)placeTip(lastE);
 tip.querySelectorAll('.sp-nav-btn:not(.sp-nav-dis)').forEach(function(btn){
  btn.onclick=function(ev){ev.stopPropagation();
   var nd=tipPos+parseInt(btn.getAttribute('data-d'),10);
   if(nd>=0&amp;&amp;nd<tipIdxs.length){tipPos=nd;renderTip();}};});}
var dragging=false,dsx=0,dsy=0,moved=false,pinned=false;

wrap.addEventListener('mouseleave',function(e){
 if(pinned)return;
 var rt=e.relatedTarget;
 if(rt&amp;&amp;(rt===tip||tip.contains(rt)))return;
 tip.classList.remove('sp-vis');tipIdxs=[];});
wrap.addEventListener('mousemove',function(e){
 if(dragging||pinned)return;
 lastE=e;

 var hits=[];
 document.elementsFromPoint(e.clientX,e.clientY).forEach(function(el){
  if(el===tip||tip.contains(el))return;
  var found=null;
  for(var n=el;n&amp;&amp;n!==document.body;n=n.parentElement){
   if(found===null&amp;&amp;n.getAttribute&amp;&amp;n.getAttribute('data-idx')!==null)found=n;
   if(n===wrap){
    if(found!==null){var idx=parseInt(found.getAttribute('data-idx'),10);
     if(hits.indexOf(idx)===-1)hits.push(idx);}
    return;}
  }
 });
 if(hits.length){
  var same=hits.length===tipIdxs.length&amp;&amp;hits.every(function(v,i){return v===tipIdxs[i];});
  if(!same){tipIdxs=hits;tipPos=0;renderTip();}else placeTip(e);
 }else{tip.classList.remove('sp-vis');tipIdxs=[];}});

var origVB=svg.getAttribute('viewBox')||'';
function parseVB(s){return s.split(/[\s,]+/).map(Number);}
function animateVB(x,y,w,h){
 svg.style.transition='all 0.5s cubic-bezier(.4,0,.2,1)';
 svg.setAttribute('viewBox',x+' '+y+' '+w+' '+h);}
function resetVB(){
 svg.style.transition='all 0.45s cubic-bezier(.4,0,.2,1)';
 if(origVB)svg.setAttribute('viewBox',origVB);
 setTimeout(function(){svg.style.transition='';},500);}
function reAnim(){var els=svg.querySelectorAll('[data-idx]');els.forEach(function(el){el.style.animation='none';el.style.filter='';});void svg.offsetHeight;els.forEach(function(el,i){el.style.animation='';el.style.animationDelay=(i*14)+'ms';});}
var ov=wrap.querySelector('.sp-sel-ov');var panel=wrap.querySelector('.sp-cpanel');
wrap.addEventListener('mousedown',function(e){
 if(e.button!==0)return;
 var t=e.target;if(t===panel||panel.contains(t))return;if(t===tip||tip.contains(t))return;
 dragging=true;moved=false;
 var r=wrap.getBoundingClientRect();dsx=e.clientX-r.left;dsy=e.clientY-r.top;
 ov.style.cssText='display:none;position:absolute;pointer-events:none;border:2px solid #6366F1;background:rgba(99,102,241,.12);box-sizing:border-box;border-radius:3px;z-index:10';
 e.preventDefault();});
document.addEventListener('mousemove',function(e){if(!dragging)return;
 var r=wrap.getBoundingClientRect();var cx=e.clientX-r.left,cy=e.clientY-r.top;
 if(!moved&amp;&amp;Math.abs(cx-dsx)<8&amp;&amp;Math.abs(cy-dsy)<8)return;
 if(!moved){pinned=false;tip.classList.remove('sp-vis');tipIdxs=[];}
 moved=true;ov.style.display='block';
 ov.style.left=Math.min(dsx,cx)+'px';ov.style.top=Math.min(dsy,cy)+'px';
 ov.style.width=Math.abs(cx-dsx)+'px';ov.style.height=Math.abs(cy-dsy)+'px';});
document.addEventListener('mouseup',function(e){if(!dragging)return;dragging=false;
 ov.style.display='none';if(!moved){
 var ch=[];document.elementsFromPoint(e.clientX,e.clientY).forEach(function(el){if(el===tip||tip.contains(el))return;var fd=null;for(var n2=el;n2&amp;&amp;n2!==document.body;n2=n2.parentElement){if(fd===null&amp;&amp;n2.getAttribute&amp;&amp;n2.getAttribute('data-idx')!==null)fd=n2;if(n2===wrap){if(fd!==null){var idx=parseInt(fd.getAttribute('data-idx'),10);if(ch.indexOf(idx)===-1)ch.push(idx);}return;}}});
 if(ch.length){pinned=true;tipIdxs=ch;tipPos=0;lastE=e;renderTip();}else{pinned=false;tip.classList.remove('sp-vis');tipIdxs=[];}return;}
 var r=wrap.getBoundingClientRect();var cx=e.clientX-r.left,cy=e.clientY-r.top;
 var rx1=Math.min(dsx,cx),ry1=Math.min(dsy,cy),rx2=Math.max(dsx,cx),ry2=Math.max(dsy,cy);
 if(rx2-rx1<8&amp;&amp;ry2-ry1<8)return;
 var ctm=svg.getScreenCTM();if(!ctm)return;var inv=ctm.inverse();
 function toS(px,py){var pt=svg.createSVGPoint();pt.x=px+r.left;pt.y=py+r.top;return pt.matrixTransform(inv);}
 var p1=toS(rx1,ry1),p2=toS(rx2,ry2);
 var bx1=Math.min(p1.x,p2.x),by1=Math.min(p1.y,p2.y),bx2=Math.max(p1.x,p2.x),by2=Math.max(p1.y,p2.y);
 var pts=svg.querySelectorAll('[data-idx]');var sel=[],unsel=[];
 pts.forEach(function(el){try{var bb=el.getBBox();var ecx=bb.x+bb.width/2,ecy=bb.y+bb.height/2;
  if(ecx>=bx1&amp;&amp;ecx<=bx2&amp;&amp;ecy>=by1&amp;&amp;ecy<=by2)sel.push(el);else unsel.push(el);}catch(ex){unsel.push(el);}});
 if(!sel.length)return;
 sel.forEach(function(el){el.style.stroke='#6366F1';el.style.strokeWidth='2.5';el.style.opacity='';el.style.transform='';});
 unsel.forEach(function(el){
  el.style.transformBox='fill-box';el.style.transformOrigin='center';
  el.style.transition='transform 0.28s cubic-bezier(.4,0,.2,1),opacity 0.28s';
  el.style.transform='scale(0,0)';el.style.opacity='0';
  setTimeout(function(){el.style.display='none';el.style.transition='';},300);});

 var mnx=Infinity,mny=Infinity,mxx=-Infinity,mxy=-Infinity;
 sel.forEach(function(el){try{var bb=el.getBBox();
  mnx=Math.min(mnx,bb.x);mny=Math.min(mny,bb.y);
  mxx=Math.max(mxx,bb.x+bb.width);mxy=Math.max(mxy,bb.y+bb.height);}catch(ex){}});
 var vb=parseVB(origVB.length?origVB:(svg.getAttribute('viewBox')||'0 0 800 500'));
 var axL=mnx-vb[0],axT=mny-vb[1],axR=(vb[0]+vb[2])-mxx,axB=(vb[1]+vb[3])-mxy;
 var sw=mxx-mnx||1,sh=mxy-mny||1;
 var pL=Math.max(sw*0.12,axL>0?axL*0.7:sw*0.12);
 var pR=Math.max(sw*0.08,axR>0?axR*0.5:sw*0.08);
 var pT=Math.max(sh*0.12,axT>0?axT*0.7:sh*0.12);
 var pB=Math.max(sh*0.20,axB>0?axB*0.8:sh*0.20);
 animateVB(mnx-pL,mny-pT,(mxx+pR)-(mnx-pL),(mxy+pB)-(mny-pT));

 var xs=sel.map(function(el){return parseFloat(el.getAttribute('data-x'));}).filter(Number.isFinite);
 var ys=sel.map(function(el){return parseFloat(el.getAttribute('data-y'));}).filter(Number.isFinite);
 var s='<span style=&quot;color:#6366F1;font-weight:700&quot;>'+sel.length+' pts</span>';
 if(xs.length&amp;&amp;ys.length){
  var mx=xs.reduce(function(a,b){return a+b;})/xs.length;
  var my=ys.reduce(function(a,b){return a+b;})/ys.length;
  var vX=xs.map(function(v){return(v-mx)*(v-mx);}).reduce(function(a,b){return a+b;})/xs.length;
  var vY=ys.map(function(v){return(v-my)*(v-my);}).reduce(function(a,b){return a+b;})/ys.length;
  s+=' &amp;middot; X&amp;#772; <b>'+mx.toFixed(2)+'</b> &amp;plusmn;'+Math.sqrt(vX).toFixed(2);
  s+=' &amp;middot; Y&amp;#772; <b>'+my.toFixed(2)+'</b> &amp;plusmn;'+Math.sqrt(vY).toFixed(2);}
 s+=' <span class=&quot;sp-cls-x&quot;>&amp;#10005;</span>';
 panel.innerHTML=s;panel.style.display='block';
 panel.querySelector('.sp-cls-x').addEventListener('click',clearSel);});
function clearSel(){panel.style.display='none';resetVB();
 svg.querySelectorAll('[data-idx]').forEach(function(el){
  el.style.display='';el.style.opacity='';el.style.stroke='';el.style.strokeWidth='';el.style.filter='';
  el.style.transform='';el.style.transition='';el.style.transformBox='';el.style.transformOrigin='';});
 reAnim();}
document.addEventListener('keydown',function(e){if(e.key==='Escape'){if(dblZoomed){dblZoomed=false;resetVB();svg.querySelectorAll('[data-idx]').forEach(function(el){el.style.opacity='';el.style.display='';el.style.transform='';el.style.transition='';});reAnim();}if(pinned){pinned=false;tip.classList.remove('sp-vis');tipIdxs=[];}}});
var dblZoomed=false;
svg.addEventListener('dblclick',function(e){
 if(dblZoomed){dblZoomed=false;resetVB();svg.querySelectorAll('[data-idx]').forEach(function(el){el.style.opacity='';el.style.display='';el.style.transform='';el.style.transition='';el.style.filter='';});reAnim();return;}
 var found=null;
 for(var nd=e.target;nd&amp;&amp;nd!==svg;nd=nd.parentElement){if(nd.getAttribute&amp;&amp;nd.getAttribute('data-idx')!==null){found=nd;break;}}
 if(!found)return;
 e.stopPropagation();
 var idx=parseInt(found.getAttribute('data-idx'),10);
 var bb;try{bb=found.getBBox();}catch(ex){return;}
 var pad=Math.max(bb.width,bb.height)*1.5+30;
 svg.querySelectorAll('[data-idx]').forEach(function(el){
  var eli=parseInt(el.getAttribute('data-idx'),10);
  if(eli===idx){el.style.opacity='1';el.style.filter='drop-shadow(0 0 6px rgba(99,102,241,.45))';}else{el.style.display='none';}});
 var vb=parseVB(origVB.length?origVB:(svg.getAttribute('viewBox')||'0 0 800 500'));
 var nx=Math.max(vb[0],bb.x-pad),ny=Math.max(vb[1],bb.y-pad);
 var nw=Math.min(vb[2],bb.width+pad*2),nh=Math.min(vb[3],bb.height+pad*2);
 animateVB(nx,ny,nw,nh);
 dblZoomed=true;
},false);
var legs=svg.querySelectorAll('[data-legend]');
legs.forEach(function(lg){lg.style.cursor='pointer';
lg.addEventListener('click',function(){
 var s=lg.getAttribute('data-series');if(!s)return;
 var h=lg.getAttribute('data-hidden')==='1';
 var els=svg.querySelectorAll('[data-series=&quot;'+s+'&quot;]:not([data-legend])');
 if(h){els.forEach(function(el){el.style.display='';el.style.opacity='';el.style.transform='';el.style.transformBox='';el.style.transformOrigin='';el.style.transition='';});lg.style.opacity='';lg.removeAttribute('data-hidden');}
 else{els.forEach(function(el){
  el.style.transformBox='fill-box';el.style.transformOrigin='center';
  el.style.transition='transform 0.3s cubic-bezier(.4,0,.2,1),opacity 0.3s';
  el.style.transform='scale(0,0)';el.style.opacity='0';
  setTimeout(function(){el.style.display='none';el.style.transition='';},320);});
  lg.style.opacity='0.3';lg.setAttribute('data-hidden','1');}spRescale();});});
function spRescale(){
 var m=svg.getAttribute('data-sp');if(!m)return;
 var p=m.split(',').map(Number),pL=p[0],pT=p[1],pW=p[2],pH=p[3];
 var vals=[];
 svg.querySelectorAll('[data-pts]').forEach(function(el){if(el.style.display==='none')return;el.getAttribute('data-pts').split(',').forEach(function(v){var f=parseFloat(v);if(isFinite(f))vals.push(f);});});
 svg.querySelectorAll('circle[data-y]').forEach(function(el){if(el.style.display==='none')return;var f=parseFloat(el.getAttribute('data-y'));if(isFinite(f))vals.push(f);});
 svg.querySelectorAll('rect[data-y]').forEach(function(el){if(el.style.display==='none')return;var f=parseFloat(el.getAttribute('data-y'));if(isFinite(f))vals.push(f);});
 svg.querySelectorAll('rect[data-v]').forEach(function(el){if(el.style.display==='none')return;if(el.hasAttribute('data-y'))return;var f=parseFloat(el.getAttribute('data-v'));if(isFinite(f))vals.push(f);});
 if(!vals.length)return;
 var mn=Math.min.apply(null,vals),mx=Math.max.apply(null,vals);if(mn>0)mn=0;var rg=mx-mn;if(rg<1e-12)rg=1;
 svg.querySelectorAll('polyline[data-pts]').forEach(function(el){if(el.style.display==='none')return;var vs=el.getAttribute('data-pts').split(','),n=vs.length,sx=pW/Math.max(n-1,1),r='';for(var i=0;i<n;i++){var f=(parseFloat(vs[i])-mn)/rg;if(i>0)r+=' ';r+=(pL+Math.round(i*sx))+','+(pT+Math.round((1-f)*pH));}el.setAttribute('points',r);});
 svg.querySelectorAll('circle[data-y]').forEach(function(el){if(el.style.display==='none')return;var f=(parseFloat(el.getAttribute('data-y'))-mn)/rg;el.setAttribute('cy',pT+Math.round((1-f)*pH));});
 svg.querySelectorAll('rect[data-y]').forEach(function(el){if(el.style.display==='none')return;var v=parseFloat(el.getAttribute('data-y'));if(!isFinite(v))return;var f=(v-mn)/rg;var ny=pT+Math.round((1-f)*pH);var base=pT+pH;if(mn<0){base=pT+Math.round((0-mn)/rg*pH);}if(v>=0){el.setAttribute('y',ny);el.setAttribute('height',Math.max(1,base-ny));}else{el.setAttribute('y',base);el.setAttribute('height',Math.max(1,ny-base));}});
 svg.querySelectorAll('rect[data-v]').forEach(function(el){if(el.style.display==='none')return;if(el.hasAttribute('data-y'))return;var v=parseFloat(el.getAttribute('data-v'));if(!isFinite(v))return;var f=(v-mn)/rg;var ny=pT+Math.round((1-f)*pH);var base=pT+pH;if(mn<0){base=pT+Math.round((0-mn)/rg*pH);}if(v>=0){el.setAttribute('y',ny);el.setAttribute('height',Math.max(1,base-ny));}else{el.setAttribute('y',base);el.setAttribute('height',Math.max(1,ny-base));}});
 var tks=svg.querySelectorAll('.sp-yt'),nT=tks.length;if(nT>1)for(var i=0;i<nT;i++){var f=i/(nT-1);var v=mn+f*rg;tks[i].textContent=v>=1000?Math.round(v)+'':v.toFixed(2);tks[i].setAttribute('y',pT+Math.round((1-f)*pH)+3);}
 var gls=svg.querySelectorAll('.sp-gl');if(gls.length&amp;&amp;nT>1)for(var j=0;j<gls.length;j++){var f=(j+1)/(nT-1);var gy=pT+Math.round((1-f)*pH);gls[j].setAttribute('y1',gy);gls[j].setAttribute('y2',gy);}
}
svg.querySelectorAll('[data-idx]').forEach(function(el,i){el.style.animationDelay=(i*18)+'ms';});
})();</script></div></body></html>"></iframe></div><div class="spg-c"><iframe srcdoc="<!DOCTYPE html><html><head><meta charset=&quot;utf-8&quot;><title>Trend</title><style>#sp-tip{position:absolute;z-index:999999;pointer-events:none;opacity:0;transition:opacity .15s,transform .15s;transform:translateY(6px) scale(.97);background:#0b0e18;color:#f1f5f9;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;font-size:13px;border-radius:10px;min-width:160px;max-width:340px;box-shadow:0 4px 20px rgba(0,0,0,.45),0 0 0 1px rgba(255,255,255,.08);overflow:hidden}#sp-tip.sp-vis{opacity:1;transform:translateY(0) scale(1);pointer-events:auto}.sp-nav{display:flex;align-items:center;justify-content:space-between;padding:5px 10px;border-top:1px solid rgba(255,255,255,.08)}.sp-nav-btn{cursor:pointer;padding:0 10px;border-radius:5px;height:22px;line-height:22px;font-size:18px;background:rgba(255,255,255,.10);color:#e2e8f0;user-select:none;flex-shrink:0}.sp-nav-btn:hover{background:rgba(255,255,255,.22)}.sp-nav-dis{opacity:.25;pointer-events:none}.sp-nav-ctr{flex:1;text-align:center;font-size:11px;color:#94a3b8}.sp-head{padding:10px 14px 6px;font-weight:700;font-size:14px;color:#e2e8f0;border-bottom:1px solid rgba(255,255,255,.08)}.sp-body{padding:8px 14px 12px}.sp-row{display:flex;justify-content:space-between;align-items:baseline;gap:14px;padding:3px 0;border-bottom:1px solid rgba(255,255,255,.04)}.sp-row:last-child{border-bottom:none}.sp-key{color:#94a3b8;font-size:12px;white-space:nowrap}.sp-val{font-weight:600;font-size:12px;color:#f8fafc;text-align:right;word-break:break-all}#sp-tip img{display:block;width:100%;max-height:210px;object-fit:contain;border-top:1px solid rgba(255,255,255,.07)}#sp-tip video{display:block;width:100%;border-top:1px solid rgba(255,255,255,.07)}.sp-html{padding:8px 14px;font-size:12px;border-top:1px solid rgba(255,255,255,.07)}[data-idx]{cursor:pointer;transition:opacity .25s,filter .2s,transform .25s}[data-idx]:hover{filter:brightness(1.12) saturate(1.08)}.sp-cpanel{position:absolute;bottom:10px;left:50%;transform:translateX(-50%);background:#0b0e18;color:#f1f5f9;border-radius:10px;padding:8px 16px;font-size:12px;font-family:-apple-system,Arial,sans-serif;box-shadow:0 8px 24px rgba(0,0,0,.4);z-index:20;white-space:nowrap;display:none}.sp-cls-x{cursor:pointer;color:#94a3b8;margin-left:6px;font-size:13px}.sp-cls-x:hover{color:#f87171}</style><style>body{margin:0;background:#fff;display:flex;justify-content:center;padding:0}@keyframes sp-i{from{opacity:0;transform:translateY(8px) scale(.94)}}@keyframes sp-bar{from{opacity:0;transform:scaleY(0)}}@keyframes sp-pop{0%{opacity:0;transform:scale(0)}70%{transform:scale(1.08)}100%{opacity:1;transform:scale(1)}}@keyframes sp-arc{from{opacity:0;transform:scale(.82) rotate(-6deg)}}@keyframes sp-fn{from{opacity:0;transform:scaleX(.7) translateY(8px)}}svg rect[data-idx]{transform-box:fill-box;transform-origin:bottom center;animation:sp-bar .5s cubic-bezier(.22,.61,.36,1) both}svg circle[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-pop .42s cubic-bezier(.34,1.56,.64,1) both}svg path[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-arc .48s cubic-bezier(.22,.61,.36,1) both}svg polygon[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-fn .48s cubic-bezier(.22,.61,.36,1) both}svg line[data-idx]{animation:sp-i .45s cubic-bezier(.22,.61,.36,1) both}svg rect[data-idx]:hover{transform:scaleY(1.03);filter:brightness(1.12) saturate(1.1)}svg circle[data-idx]:hover{transform:scale(1.3);filter:brightness(1.15)}svg path[data-idx]:hover{filter:brightness(1.1) drop-shadow(0 2px 8px rgba(0,0,0,.2))}.sp-bg{fill:#fff}</style></head><body><div id=&quot;spp3&quot; style=&quot;position:relative;display:inline-block;border-radius:12px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,.07),0 0 0 1px rgba(0,0,0,.04)&quot;><svg xmlns=&quot;http://www.w3.org/2000/svg&quot; width=&quot;900&quot; height=&quot;480&quot; viewBox=&quot;0 0 900 480&quot;><rect class=&quot;sp-bg&quot; width=&quot;100%&quot; height=&quot;100%&quot;/><text x=&quot;450&quot; y=&quot;22&quot; text-anchor=&quot;middle&quot; font-family=&quot;-apple-system,Arial,sans-serif&quot; font-size=&quot;14&quot; font-weight=&quot;700&quot; fill=&quot;#1a202c&quot;>Trend</text><text x=&quot;48&quot; y=&quot;435&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>0.00</text><text x=&quot;48&quot; y=&quot;355&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>4.00</text><text x=&quot;48&quot; y=&quot;276&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>8.00</text><text x=&quot;48&quot; y=&quot;197&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>12.00</text><text x=&quot;48&quot; y=&quot;118&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>16.00</text><text x=&quot;48&quot; y=&quot;39&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>20.00</text><line x1=&quot;52&quot; y1=&quot;36&quot; x2=&quot;52&quot; y2=&quot;432&quot; stroke=&quot;#cbd5e1&quot; stroke-width=&quot;1&quot; class=&quot;sp-ax-y&quot;/><line x1=&quot;52&quot; y1=&quot;432&quot; x2=&quot;880&quot; y2=&quot;432&quot; stroke=&quot;#cbd5e1&quot; stroke-width=&quot;1&quot; class=&quot;sp-ax-x&quot;/><polyline fill=&quot;none&quot; stroke=&quot;#6366f1&quot; stroke-width=&quot;2&quot; stroke-linecap=&quot;round&quot; points=&quot;52,234 466,36 880,135&quot;/><circle data-idx=&quot;0&quot; data-y=&quot;10.00&quot; data-lbl=&quot;Jan&quot; cx=&quot;52&quot; cy=&quot;234&quot; r=&quot;4&quot; fill=&quot;#6366f1&quot; stroke=&quot;#fff&quot; stroke-width=&quot;1.5&quot;/><circle data-idx=&quot;1&quot; data-y=&quot;20.00&quot; data-lbl=&quot;Feb&quot; cx=&quot;466&quot; cy=&quot;36&quot; r=&quot;4&quot; fill=&quot;#f43f5e&quot; stroke=&quot;#fff&quot; stroke-width=&quot;1.5&quot;/><circle data-idx=&quot;2&quot; data-y=&quot;15.00&quot; data-lbl=&quot;Mar&quot; cx=&quot;880&quot; cy=&quot;135&quot; r=&quot;4&quot; fill=&quot;#10b981&quot; stroke=&quot;#fff&quot; stroke-width=&quot;1.5&quot;/><text x=&quot;52&quot; y=&quot;446&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#6b7280&quot;>Jan</text><text x=&quot;466&quot; y=&quot;446&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#6b7280&quot;>Feb</text><text x=&quot;880&quot; y=&quot;446&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#6b7280&quot;>Mar</text></svg><div class=&quot;sp-sel-ov&quot; style=&quot;display:none&quot;></div><div class=&quot;sp-cpanel&quot;></div><script>(function(){
var wrap=document.getElementById('spp3');if(!wrap)return;wrap.removeAttribute('id');
var svg=wrap.querySelector('svg');var data=[];

var tip=document.getElementById('sp-tip');
if(!tip){tip=document.createElement('div');tip.id='sp-tip';document.body.appendChild(tip);}
var tipIdxs=[],tipPos=0,lastE=null;
function esc(s){return String(s).replace(/&amp;/g,'&amp;amp;').replace(/</g,'&amp;lt;');}
function cardHTML(d){
 var h='';if(d.title)h+='<div class=&quot;sp-head&quot;>'+esc(d.title)+'</div>';
 var rows='';(d.kv||[]).forEach(function(p){
  rows+='<div class=&quot;sp-row&quot;><span class=&quot;sp-key&quot;>'+esc(p[0])+'</span><span class=&quot;sp-val&quot;>'+esc(p[1])+'</span></div>';});
 if(rows)h+='<div class=&quot;sp-body&quot;>'+rows+'</div>';
 if(d.image)h+='<img src=&quot;'+esc(d.image)+'&quot; alt=&quot;&quot; loading=&quot;lazy&quot;>';
 if(d.video)h+='<video src=&quot;'+esc(d.video)+'&quot; controls muted playsinline></video>';
 if(d.html)h+='<div class=&quot;sp-html&quot;>'+d.html+'</div>';
 return h;}
function placeTip(e){
 var sx=window.scrollX||0,sy=window.scrollY||0;
 var x=e.pageX+16,y=e.pageY-14;
 var tw=tip.offsetWidth||220,th=tip.offsetHeight||60;
 if(x+tw>sx+window.innerWidth-6)x=e.pageX-tw-16;
 if(x<sx+4)x=sx+4;
 if(y<sy+6)y=e.pageY+20;
 if(y+th>sy+window.innerHeight-6)y=sy+window.innerHeight-th-6;
 tip.style.left=x+'px';tip.style.top=y+'px';}

function getSlot(idx){
 var d=data[idx];if(d)return d;
 var el=svg.querySelector('[data-idx=&quot;'+idx+'&quot;]');if(!el)return null;
 var kv=[];
 var x=el.getAttribute('data-x'),y=el.getAttribute('data-y');
 if(x!=null){var xf=parseFloat(x);kv.push(['X',xf===xf?xf.toFixed(2):x]);}
 if(y!=null){kv.push(['Valeur',parseFloat(y).toFixed(2)]);}
 else{var v=el.getAttribute('data-v');
 if(v!=null){
  var r=el.getAttribute('data-r'),c=el.getAttribute('data-c');
  kv.push(['Valeur',parseFloat(v).toFixed(3)]);
  if(r)kv.push(['Ligne',r]);if(c)kv.push(['Colonne',c]);}}
 var z=el.getAttribute('data-z');
 if(z!=null){kv.push(['Z',parseFloat(z).toFixed(2)]);}
 var lbl=el.getAttribute('data-lbl');
 var an=el.attributes;if(an){for(var ai=0;ai<an.length;ai++){var a=an[ai];if(a.name.substring(0,8)==='data-kv-'){kv.push([a.name.substring(8),a.value]);}}}
 var title=lbl||(el.getAttribute('data-v')!=null&amp;&amp;el.getAttribute('data-r')!=null?el.getAttribute('data-r')+' \u00d7 '+el.getAttribute('data-c'):'Point '+(idx+1));
 return{title:title,kv:kv};}
function renderTip(){
 if(!tipIdxs.length){tip.classList.remove('sp-vis');return;}
 var d=getSlot(tipIdxs[tipPos]);if(!d){tip.classList.remove('sp-vis');return;}
 var h=cardHTML(d);
 if(tipIdxs.length>1){
  var p=tipPos,n=tipIdxs.length;
  h+='<div class=&quot;sp-nav&quot;>'
   +'<span class=&quot;sp-nav-btn'+(p>0?'':' sp-nav-dis')+'&quot; data-d=&quot;-1&quot;>&amp;#8249;</span>'
   +'<span class=&quot;sp-nav-ctr&quot;>'+(p+1)+' / '+n+'</span>'
   +'<span class=&quot;sp-nav-btn'+(p<n-1?'':' sp-nav-dis')+'&quot; data-d=&quot;1&quot;>&amp;#8250;</span>'
   +'</div>';}
 tip.innerHTML=h;tip.classList.add('sp-vis');
 if(lastE)placeTip(lastE);
 tip.querySelectorAll('.sp-nav-btn:not(.sp-nav-dis)').forEach(function(btn){
  btn.onclick=function(ev){ev.stopPropagation();
   var nd=tipPos+parseInt(btn.getAttribute('data-d'),10);
   if(nd>=0&amp;&amp;nd<tipIdxs.length){tipPos=nd;renderTip();}};});}
var dragging=false,dsx=0,dsy=0,moved=false,pinned=false;

wrap.addEventListener('mouseleave',function(e){
 if(pinned)return;
 var rt=e.relatedTarget;
 if(rt&amp;&amp;(rt===tip||tip.contains(rt)))return;
 tip.classList.remove('sp-vis');tipIdxs=[];});
wrap.addEventListener('mousemove',function(e){
 if(dragging||pinned)return;
 lastE=e;

 var hits=[];
 document.elementsFromPoint(e.clientX,e.clientY).forEach(function(el){
  if(el===tip||tip.contains(el))return;
  var found=null;
  for(var n=el;n&amp;&amp;n!==document.body;n=n.parentElement){
   if(found===null&amp;&amp;n.getAttribute&amp;&amp;n.getAttribute('data-idx')!==null)found=n;
   if(n===wrap){
    if(found!==null){var idx=parseInt(found.getAttribute('data-idx'),10);
     if(hits.indexOf(idx)===-1)hits.push(idx);}
    return;}
  }
 });
 if(hits.length){
  var same=hits.length===tipIdxs.length&amp;&amp;hits.every(function(v,i){return v===tipIdxs[i];});
  if(!same){tipIdxs=hits;tipPos=0;renderTip();}else placeTip(e);
 }else{tip.classList.remove('sp-vis');tipIdxs=[];}});

var origVB=svg.getAttribute('viewBox')||'';
function parseVB(s){return s.split(/[\s,]+/).map(Number);}
function animateVB(x,y,w,h){
 svg.style.transition='all 0.5s cubic-bezier(.4,0,.2,1)';
 svg.setAttribute('viewBox',x+' '+y+' '+w+' '+h);}
function resetVB(){
 svg.style.transition='all 0.45s cubic-bezier(.4,0,.2,1)';
 if(origVB)svg.setAttribute('viewBox',origVB);
 setTimeout(function(){svg.style.transition='';},500);}
function reAnim(){var els=svg.querySelectorAll('[data-idx]');els.forEach(function(el){el.style.animation='none';el.style.filter='';});void svg.offsetHeight;els.forEach(function(el,i){el.style.animation='';el.style.animationDelay=(i*14)+'ms';});}
var ov=wrap.querySelector('.sp-sel-ov');var panel=wrap.querySelector('.sp-cpanel');
wrap.addEventListener('mousedown',function(e){
 if(e.button!==0)return;
 var t=e.target;if(t===panel||panel.contains(t))return;if(t===tip||tip.contains(t))return;
 dragging=true;moved=false;
 var r=wrap.getBoundingClientRect();dsx=e.clientX-r.left;dsy=e.clientY-r.top;
 ov.style.cssText='display:none;position:absolute;pointer-events:none;border:2px solid #6366F1;background:rgba(99,102,241,.12);box-sizing:border-box;border-radius:3px;z-index:10';
 e.preventDefault();});
document.addEventListener('mousemove',function(e){if(!dragging)return;
 var r=wrap.getBoundingClientRect();var cx=e.clientX-r.left,cy=e.clientY-r.top;
 if(!moved&amp;&amp;Math.abs(cx-dsx)<8&amp;&amp;Math.abs(cy-dsy)<8)return;
 if(!moved){pinned=false;tip.classList.remove('sp-vis');tipIdxs=[];}
 moved=true;ov.style.display='block';
 ov.style.left=Math.min(dsx,cx)+'px';ov.style.top=Math.min(dsy,cy)+'px';
 ov.style.width=Math.abs(cx-dsx)+'px';ov.style.height=Math.abs(cy-dsy)+'px';});
document.addEventListener('mouseup',function(e){if(!dragging)return;dragging=false;
 ov.style.display='none';if(!moved){
 var ch=[];document.elementsFromPoint(e.clientX,e.clientY).forEach(function(el){if(el===tip||tip.contains(el))return;var fd=null;for(var n2=el;n2&amp;&amp;n2!==document.body;n2=n2.parentElement){if(fd===null&amp;&amp;n2.getAttribute&amp;&amp;n2.getAttribute('data-idx')!==null)fd=n2;if(n2===wrap){if(fd!==null){var idx=parseInt(fd.getAttribute('data-idx'),10);if(ch.indexOf(idx)===-1)ch.push(idx);}return;}}});
 if(ch.length){pinned=true;tipIdxs=ch;tipPos=0;lastE=e;renderTip();}else{pinned=false;tip.classList.remove('sp-vis');tipIdxs=[];}return;}
 var r=wrap.getBoundingClientRect();var cx=e.clientX-r.left,cy=e.clientY-r.top;
 var rx1=Math.min(dsx,cx),ry1=Math.min(dsy,cy),rx2=Math.max(dsx,cx),ry2=Math.max(dsy,cy);
 if(rx2-rx1<8&amp;&amp;ry2-ry1<8)return;
 var ctm=svg.getScreenCTM();if(!ctm)return;var inv=ctm.inverse();
 function toS(px,py){var pt=svg.createSVGPoint();pt.x=px+r.left;pt.y=py+r.top;return pt.matrixTransform(inv);}
 var p1=toS(rx1,ry1),p2=toS(rx2,ry2);
 var bx1=Math.min(p1.x,p2.x),by1=Math.min(p1.y,p2.y),bx2=Math.max(p1.x,p2.x),by2=Math.max(p1.y,p2.y);
 var pts=svg.querySelectorAll('[data-idx]');var sel=[],unsel=[];
 pts.forEach(function(el){try{var bb=el.getBBox();var ecx=bb.x+bb.width/2,ecy=bb.y+bb.height/2;
  if(ecx>=bx1&amp;&amp;ecx<=bx2&amp;&amp;ecy>=by1&amp;&amp;ecy<=by2)sel.push(el);else unsel.push(el);}catch(ex){unsel.push(el);}});
 if(!sel.length)return;
 sel.forEach(function(el){el.style.stroke='#6366F1';el.style.strokeWidth='2.5';el.style.opacity='';el.style.transform='';});
 unsel.forEach(function(el){
  el.style.transformBox='fill-box';el.style.transformOrigin='center';
  el.style.transition='transform 0.28s cubic-bezier(.4,0,.2,1),opacity 0.28s';
  el.style.transform='scale(0,0)';el.style.opacity='0';
  setTimeout(function(){el.style.display='none';el.style.transition='';},300);});

 var mnx=Infinity,mny=Infinity,mxx=-Infinity,mxy=-Infinity;
 sel.forEach(function(el){try{var bb=el.getBBox();
  mnx=Math.min(mnx,bb.x);mny=Math.min(mny,bb.y);
  mxx=Math.max(mxx,bb.x+bb.width);mxy=Math.max(mxy,bb.y+bb.height);}catch(ex){}});
 var vb=parseVB(origVB.length?origVB:(svg.getAttribute('viewBox')||'0 0 800 500'));
 var axL=mnx-vb[0],axT=mny-vb[1],axR=(vb[0]+vb[2])-mxx,axB=(vb[1]+vb[3])-mxy;
 var sw=mxx-mnx||1,sh=mxy-mny||1;
 var pL=Math.max(sw*0.12,axL>0?axL*0.7:sw*0.12);
 var pR=Math.max(sw*0.08,axR>0?axR*0.5:sw*0.08);
 var pT=Math.max(sh*0.12,axT>0?axT*0.7:sh*0.12);
 var pB=Math.max(sh*0.20,axB>0?axB*0.8:sh*0.20);
 animateVB(mnx-pL,mny-pT,(mxx+pR)-(mnx-pL),(mxy+pB)-(mny-pT));

 var xs=sel.map(function(el){return parseFloat(el.getAttribute('data-x'));}).filter(Number.isFinite);
 var ys=sel.map(function(el){return parseFloat(el.getAttribute('data-y'));}).filter(Number.isFinite);
 var s='<span style=&quot;color:#6366F1;font-weight:700&quot;>'+sel.length+' pts</span>';
 if(xs.length&amp;&amp;ys.length){
  var mx=xs.reduce(function(a,b){return a+b;})/xs.length;
  var my=ys.reduce(function(a,b){return a+b;})/ys.length;
  var vX=xs.map(function(v){return(v-mx)*(v-mx);}).reduce(function(a,b){return a+b;})/xs.length;
  var vY=ys.map(function(v){return(v-my)*(v-my);}).reduce(function(a,b){return a+b;})/ys.length;
  s+=' &amp;middot; X&amp;#772; <b>'+mx.toFixed(2)+'</b> &amp;plusmn;'+Math.sqrt(vX).toFixed(2);
  s+=' &amp;middot; Y&amp;#772; <b>'+my.toFixed(2)+'</b> &amp;plusmn;'+Math.sqrt(vY).toFixed(2);}
 s+=' <span class=&quot;sp-cls-x&quot;>&amp;#10005;</span>';
 panel.innerHTML=s;panel.style.display='block';
 panel.querySelector('.sp-cls-x').addEventListener('click',clearSel);});
function clearSel(){panel.style.display='none';resetVB();
 svg.querySelectorAll('[data-idx]').forEach(function(el){
  el.style.display='';el.style.opacity='';el.style.stroke='';el.style.strokeWidth='';el.style.filter='';
  el.style.transform='';el.style.transition='';el.style.transformBox='';el.style.transformOrigin='';});
 reAnim();}
document.addEventListener('keydown',function(e){if(e.key==='Escape'){if(dblZoomed){dblZoomed=false;resetVB();svg.querySelectorAll('[data-idx]').forEach(function(el){el.style.opacity='';el.style.display='';el.style.transform='';el.style.transition='';});reAnim();}if(pinned){pinned=false;tip.classList.remove('sp-vis');tipIdxs=[];}}});
var dblZoomed=false;
svg.addEventListener('dblclick',function(e){
 if(dblZoomed){dblZoomed=false;resetVB();svg.querySelectorAll('[data-idx]').forEach(function(el){el.style.opacity='';el.style.display='';el.style.transform='';el.style.transition='';el.style.filter='';});reAnim();return;}
 var found=null;
 for(var nd=e.target;nd&amp;&amp;nd!==svg;nd=nd.parentElement){if(nd.getAttribute&amp;&amp;nd.getAttribute('data-idx')!==null){found=nd;break;}}
 if(!found)return;
 e.stopPropagation();
 var idx=parseInt(found.getAttribute('data-idx'),10);
 var bb;try{bb=found.getBBox();}catch(ex){return;}
 var pad=Math.max(bb.width,bb.height)*1.5+30;
 svg.querySelectorAll('[data-idx]').forEach(function(el){
  var eli=parseInt(el.getAttribute('data-idx'),10);
  if(eli===idx){el.style.opacity='1';el.style.filter='drop-shadow(0 0 6px rgba(99,102,241,.45))';}else{el.style.display='none';}});
 var vb=parseVB(origVB.length?origVB:(svg.getAttribute('viewBox')||'0 0 800 500'));
 var nx=Math.max(vb[0],bb.x-pad),ny=Math.max(vb[1],bb.y-pad);
 var nw=Math.min(vb[2],bb.width+pad*2),nh=Math.min(vb[3],bb.height+pad*2);
 animateVB(nx,ny,nw,nh);
 dblZoomed=true;
},false);
var legs=svg.querySelectorAll('[data-legend]');
legs.forEach(function(lg){lg.style.cursor='pointer';
lg.addEventListener('click',function(){
 var s=lg.getAttribute('data-series');if(!s)return;
 var h=lg.getAttribute('data-hidden')==='1';
 var els=svg.querySelectorAll('[data-series=&quot;'+s+'&quot;]:not([data-legend])');
 if(h){els.forEach(function(el){el.style.display='';el.style.opacity='';el.style.transform='';el.style.transformBox='';el.style.transformOrigin='';el.style.transition='';});lg.style.opacity='';lg.removeAttribute('data-hidden');}
 else{els.forEach(function(el){
  el.style.transformBox='fill-box';el.style.transformOrigin='center';
  el.style.transition='transform 0.3s cubic-bezier(.4,0,.2,1),opacity 0.3s';
  el.style.transform='scale(0,0)';el.style.opacity='0';
  setTimeout(function(){el.style.display='none';el.style.transition='';},320);});
  lg.style.opacity='0.3';lg.setAttribute('data-hidden','1');}spRescale();});});
function spRescale(){
 var m=svg.getAttribute('data-sp');if(!m)return;
 var p=m.split(',').map(Number),pL=p[0],pT=p[1],pW=p[2],pH=p[3];
 var vals=[];
 svg.querySelectorAll('[data-pts]').forEach(function(el){if(el.style.display==='none')return;el.getAttribute('data-pts').split(',').forEach(function(v){var f=parseFloat(v);if(isFinite(f))vals.push(f);});});
 svg.querySelectorAll('circle[data-y]').forEach(function(el){if(el.style.display==='none')return;var f=parseFloat(el.getAttribute('data-y'));if(isFinite(f))vals.push(f);});
 svg.querySelectorAll('rect[data-y]').forEach(function(el){if(el.style.display==='none')return;var f=parseFloat(el.getAttribute('data-y'));if(isFinite(f))vals.push(f);});
 svg.querySelectorAll('rect[data-v]').forEach(function(el){if(el.style.display==='none')return;if(el.hasAttribute('data-y'))return;var f=parseFloat(el.getAttribute('data-v'));if(isFinite(f))vals.push(f);});
 if(!vals.length)return;
 var mn=Math.min.apply(null,vals),mx=Math.max.apply(null,vals);if(mn>0)mn=0;var rg=mx-mn;if(rg<1e-12)rg=1;
 svg.querySelectorAll('polyline[data-pts]').forEach(function(el){if(el.style.display==='none')return;var vs=el.getAttribute('data-pts').split(','),n=vs.length,sx=pW/Math.max(n-1,1),r='';for(var i=0;i<n;i++){var f=(parseFloat(vs[i])-mn)/rg;if(i>0)r+=' ';r+=(pL+Math.round(i*sx))+','+(pT+Math.round((1-f)*pH));}el.setAttribute('points',r);});
 svg.querySelectorAll('circle[data-y]').forEach(function(el){if(el.style.display==='none')return;var f=(parseFloat(el.getAttribute('data-y'))-mn)/rg;el.setAttribute('cy',pT+Math.round((1-f)*pH));});
 svg.querySelectorAll('rect[data-y]').forEach(function(el){if(el.style.display==='none')return;var v=parseFloat(el.getAttribute('data-y'));if(!isFinite(v))return;var f=(v-mn)/rg;var ny=pT+Math.round((1-f)*pH);var base=pT+pH;if(mn<0){base=pT+Math.round((0-mn)/rg*pH);}if(v>=0){el.setAttribute('y',ny);el.setAttribute('height',Math.max(1,base-ny));}else{el.setAttribute('y',base);el.setAttribute('height',Math.max(1,ny-base));}});
 svg.querySelectorAll('rect[data-v]').forEach(function(el){if(el.style.display==='none')return;if(el.hasAttribute('data-y'))return;var v=parseFloat(el.getAttribute('data-v'));if(!isFinite(v))return;var f=(v-mn)/rg;var ny=pT+Math.round((1-f)*pH);var base=pT+pH;if(mn<0){base=pT+Math.round((0-mn)/rg*pH);}if(v>=0){el.setAttribute('y',ny);el.setAttribute('height',Math.max(1,base-ny));}else{el.setAttribute('y',base);el.setAttribute('height',Math.max(1,ny-base));}});
 var tks=svg.querySelectorAll('.sp-yt'),nT=tks.length;if(nT>1)for(var i=0;i<nT;i++){var f=i/(nT-1);var v=mn+f*rg;tks[i].textContent=v>=1000?Math.round(v)+'':v.toFixed(2);tks[i].setAttribute('y',pT+Math.round((1-f)*pH)+3);}
 var gls=svg.querySelectorAll('.sp-gl');if(gls.length&amp;&amp;nT>1)for(var j=0;j<gls.length;j++){var f=(j+1)/(nT-1);var gy=pT+Math.round((1-f)*pH);gls[j].setAttribute('y1',gy);gls[j].setAttribute('y2',gy);}
}
svg.querySelectorAll('[data-idx]').forEach(function(el,i){el.style.animationDelay=(i*18)+'ms';});
})();</script></div></body></html>"></iframe></div><div class="spg-c"><iframe srcdoc="<!DOCTYPE html><html><head><meta charset=&quot;utf-8&quot;><title>Dist</title><style>#sp-tip{position:absolute;z-index:999999;pointer-events:none;opacity:0;transition:opacity .15s,transform .15s;transform:translateY(6px) scale(.97);background:#0b0e18;color:#f1f5f9;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;font-size:13px;border-radius:10px;min-width:160px;max-width:340px;box-shadow:0 4px 20px rgba(0,0,0,.45),0 0 0 1px rgba(255,255,255,.08);overflow:hidden}#sp-tip.sp-vis{opacity:1;transform:translateY(0) scale(1);pointer-events:auto}.sp-nav{display:flex;align-items:center;justify-content:space-between;padding:5px 10px;border-top:1px solid rgba(255,255,255,.08)}.sp-nav-btn{cursor:pointer;padding:0 10px;border-radius:5px;height:22px;line-height:22px;font-size:18px;background:rgba(255,255,255,.10);color:#e2e8f0;user-select:none;flex-shrink:0}.sp-nav-btn:hover{background:rgba(255,255,255,.22)}.sp-nav-dis{opacity:.25;pointer-events:none}.sp-nav-ctr{flex:1;text-align:center;font-size:11px;color:#94a3b8}.sp-head{padding:10px 14px 6px;font-weight:700;font-size:14px;color:#e2e8f0;border-bottom:1px solid rgba(255,255,255,.08)}.sp-body{padding:8px 14px 12px}.sp-row{display:flex;justify-content:space-between;align-items:baseline;gap:14px;padding:3px 0;border-bottom:1px solid rgba(255,255,255,.04)}.sp-row:last-child{border-bottom:none}.sp-key{color:#94a3b8;font-size:12px;white-space:nowrap}.sp-val{font-weight:600;font-size:12px;color:#f8fafc;text-align:right;word-break:break-all}#sp-tip img{display:block;width:100%;max-height:210px;object-fit:contain;border-top:1px solid rgba(255,255,255,.07)}#sp-tip video{display:block;width:100%;border-top:1px solid rgba(255,255,255,.07)}.sp-html{padding:8px 14px;font-size:12px;border-top:1px solid rgba(255,255,255,.07)}[data-idx]{cursor:pointer;transition:opacity .25s,filter .2s,transform .25s}[data-idx]:hover{filter:brightness(1.12) saturate(1.08)}.sp-cpanel{position:absolute;bottom:10px;left:50%;transform:translateX(-50%);background:#0b0e18;color:#f1f5f9;border-radius:10px;padding:8px 16px;font-size:12px;font-family:-apple-system,Arial,sans-serif;box-shadow:0 8px 24px rgba(0,0,0,.4);z-index:20;white-space:nowrap;display:none}.sp-cls-x{cursor:pointer;color:#94a3b8;margin-left:6px;font-size:13px}.sp-cls-x:hover{color:#f87171}</style><style>body{margin:0;background:#fff;display:flex;justify-content:center;padding:0}@keyframes sp-i{from{opacity:0;transform:translateY(8px) scale(.94)}}@keyframes sp-bar{from{opacity:0;transform:scaleY(0)}}@keyframes sp-pop{0%{opacity:0;transform:scale(0)}70%{transform:scale(1.08)}100%{opacity:1;transform:scale(1)}}@keyframes sp-arc{from{opacity:0;transform:scale(.82) rotate(-6deg)}}@keyframes sp-fn{from{opacity:0;transform:scaleX(.7) translateY(8px)}}svg rect[data-idx]{transform-box:fill-box;transform-origin:bottom center;animation:sp-bar .5s cubic-bezier(.22,.61,.36,1) both}svg circle[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-pop .42s cubic-bezier(.34,1.56,.64,1) both}svg path[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-arc .48s cubic-bezier(.22,.61,.36,1) both}svg polygon[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-fn .48s cubic-bezier(.22,.61,.36,1) both}svg line[data-idx]{animation:sp-i .45s cubic-bezier(.22,.61,.36,1) both}svg rect[data-idx]:hover{transform:scaleY(1.03);filter:brightness(1.12) saturate(1.1)}svg circle[data-idx]:hover{transform:scale(1.3);filter:brightness(1.15)}svg path[data-idx]:hover{filter:brightness(1.1) drop-shadow(0 2px 8px rgba(0,0,0,.2))}.sp-bg{fill:#fff}</style></head><body><div id=&quot;spp4&quot; style=&quot;position:relative;display:inline-block;border-radius:12px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,.07),0 0 0 1px rgba(0,0,0,.04)&quot;><svg xmlns=&quot;http://www.w3.org/2000/svg&quot; width=&quot;860&quot; height=&quot;380&quot; viewBox=&quot;0 0 860 380&quot;><rect class=&quot;sp-bg&quot; width=&quot;100%&quot; height=&quot;100%&quot;/><text x=&quot;446&quot; y=&quot;26&quot; text-anchor=&quot;middle&quot; font-family=&quot;-apple-system,Arial,sans-serif&quot; font-size=&quot;15&quot; font-weight=&quot;700&quot; fill=&quot;#1a202c&quot; class=&quot;sp-ttl&quot;>Dist</text><text x=&quot;48&quot; y=&quot;337&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>0</text><text x=&quot;48&quot; y=&quot;277&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>1</text><text x=&quot;48&quot; y=&quot;217&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>1</text><text x=&quot;48&quot; y=&quot;158&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>2</text><text x=&quot;48&quot; y=&quot;98&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>2</text><text x=&quot;48&quot; y=&quot;39&quot; text-anchor=&quot;end&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-yt&quot;>3</text><line x1=&quot;52&quot; y1=&quot;36&quot; x2=&quot;52&quot; y2=&quot;334&quot; stroke=&quot;#cbd5e1&quot; stroke-width=&quot;1&quot; class=&quot;sp-ax-y&quot;/><line x1=&quot;52&quot; y1=&quot;334&quot; x2=&quot;840&quot; y2=&quot;334&quot; stroke=&quot;#cbd5e1&quot; stroke-width=&quot;1&quot; class=&quot;sp-ax-x&quot;/><text x=&quot;14&quot; y=&quot;185&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;10&quot; fill=&quot;#6b7280&quot; transform=&quot;rotate(-90,14,185)&quot; class=&quot;sp-yl&quot;>Count</text><rect data-idx=&quot;0&quot; data-series=&quot;0&quot; data-lbl=&quot;1.00–1.80&quot; data-kv-Count=&quot;1&quot; data-kv-Min=&quot;1.00&quot; data-kv-Max=&quot;1.80&quot; x=&quot;52&quot; y=&quot;235&quot; width=&quot;156&quot; height=&quot;99&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;1&quot; data-series=&quot;0&quot; data-lbl=&quot;1.80–2.60&quot; data-kv-Count=&quot;2&quot; data-kv-Min=&quot;1.80&quot; data-kv-Max=&quot;2.60&quot; x=&quot;209&quot; y=&quot;136&quot; width=&quot;156&quot; height=&quot;198&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;2&quot; data-series=&quot;0&quot; data-lbl=&quot;2.60–3.40&quot; data-kv-Count=&quot;3&quot; data-kv-Min=&quot;2.60&quot; data-kv-Max=&quot;3.40&quot; x=&quot;367&quot; y=&quot;36&quot; width=&quot;156&quot; height=&quot;298&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;3&quot; data-series=&quot;0&quot; data-lbl=&quot;3.40–4.20&quot; data-kv-Count=&quot;2&quot; data-kv-Min=&quot;3.40&quot; data-kv-Max=&quot;4.20&quot; x=&quot;524&quot; y=&quot;136&quot; width=&quot;156&quot; height=&quot;198&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><rect data-idx=&quot;4&quot; data-series=&quot;0&quot; data-lbl=&quot;4.20–5.00&quot; data-kv-Count=&quot;1&quot; data-kv-Min=&quot;4.20&quot; data-kv-Max=&quot;5.00&quot; x=&quot;682&quot; y=&quot;235&quot; width=&quot;156&quot; height=&quot;99&quot; fill=&quot;#6366f1&quot; fill-opacity=&quot;0.80&quot; rx=&quot;2&quot; stroke=&quot;#fff&quot; stroke-width=&quot;0.4&quot;/><text x=&quot;52&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>1.00</text><text x=&quot;209&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>1.80</text><text x=&quot;367&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>2.60</text><text x=&quot;524&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>3.40</text><text x=&quot;682&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>4.20</text><text x=&quot;840&quot; y=&quot;348&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;9&quot; fill=&quot;#9ca3af&quot; class=&quot;sp-xt&quot;>5.00</text></svg><div class=&quot;sp-sel-ov&quot; style=&quot;display:none&quot;></div><div class=&quot;sp-cpanel&quot;></div><script>(function(){
var wrap=document.getElementById('spp4');if(!wrap)return;wrap.removeAttribute('id');
var svg=wrap.querySelector('svg');var data=[];

var tip=document.getElementById('sp-tip');
if(!tip){tip=document.createElement('div');tip.id='sp-tip';document.body.appendChild(tip);}
var tipIdxs=[],tipPos=0,lastE=null;
function esc(s){return String(s).replace(/&amp;/g,'&amp;amp;').replace(/</g,'&amp;lt;');}
function cardHTML(d){
 var h='';if(d.title)h+='<div class=&quot;sp-head&quot;>'+esc(d.title)+'</div>';
 var rows='';(d.kv||[]).forEach(function(p){
  rows+='<div class=&quot;sp-row&quot;><span class=&quot;sp-key&quot;>'+esc(p[0])+'</span><span class=&quot;sp-val&quot;>'+esc(p[1])+'</span></div>';});
 if(rows)h+='<div class=&quot;sp-body&quot;>'+rows+'</div>';
 if(d.image)h+='<img src=&quot;'+esc(d.image)+'&quot; alt=&quot;&quot; loading=&quot;lazy&quot;>';
 if(d.video)h+='<video src=&quot;'+esc(d.video)+'&quot; controls muted playsinline></video>';
 if(d.html)h+='<div class=&quot;sp-html&quot;>'+d.html+'</div>';
 return h;}
function placeTip(e){
 var sx=window.scrollX||0,sy=window.scrollY||0;
 var x=e.pageX+16,y=e.pageY-14;
 var tw=tip.offsetWidth||220,th=tip.offsetHeight||60;
 if(x+tw>sx+window.innerWidth-6)x=e.pageX-tw-16;
 if(x<sx+4)x=sx+4;
 if(y<sy+6)y=e.pageY+20;
 if(y+th>sy+window.innerHeight-6)y=sy+window.innerHeight-th-6;
 tip.style.left=x+'px';tip.style.top=y+'px';}

function getSlot(idx){
 var d=data[idx];if(d)return d;
 var el=svg.querySelector('[data-idx=&quot;'+idx+'&quot;]');if(!el)return null;
 var kv=[];
 var x=el.getAttribute('data-x'),y=el.getAttribute('data-y');
 if(x!=null){var xf=parseFloat(x);kv.push(['X',xf===xf?xf.toFixed(2):x]);}
 if(y!=null){kv.push(['Valeur',parseFloat(y).toFixed(2)]);}
 else{var v=el.getAttribute('data-v');
 if(v!=null){
  var r=el.getAttribute('data-r'),c=el.getAttribute('data-c');
  kv.push(['Valeur',parseFloat(v).toFixed(3)]);
  if(r)kv.push(['Ligne',r]);if(c)kv.push(['Colonne',c]);}}
 var z=el.getAttribute('data-z');
 if(z!=null){kv.push(['Z',parseFloat(z).toFixed(2)]);}
 var lbl=el.getAttribute('data-lbl');
 var an=el.attributes;if(an){for(var ai=0;ai<an.length;ai++){var a=an[ai];if(a.name.substring(0,8)==='data-kv-'){kv.push([a.name.substring(8),a.value]);}}}
 var title=lbl||(el.getAttribute('data-v')!=null&amp;&amp;el.getAttribute('data-r')!=null?el.getAttribute('data-r')+' \u00d7 '+el.getAttribute('data-c'):'Point '+(idx+1));
 return{title:title,kv:kv};}
function renderTip(){
 if(!tipIdxs.length){tip.classList.remove('sp-vis');return;}
 var d=getSlot(tipIdxs[tipPos]);if(!d){tip.classList.remove('sp-vis');return;}
 var h=cardHTML(d);
 if(tipIdxs.length>1){
  var p=tipPos,n=tipIdxs.length;
  h+='<div class=&quot;sp-nav&quot;>'
   +'<span class=&quot;sp-nav-btn'+(p>0?'':' sp-nav-dis')+'&quot; data-d=&quot;-1&quot;>&amp;#8249;</span>'
   +'<span class=&quot;sp-nav-ctr&quot;>'+(p+1)+' / '+n+'</span>'
   +'<span class=&quot;sp-nav-btn'+(p<n-1?'':' sp-nav-dis')+'&quot; data-d=&quot;1&quot;>&amp;#8250;</span>'
   +'</div>';}
 tip.innerHTML=h;tip.classList.add('sp-vis');
 if(lastE)placeTip(lastE);
 tip.querySelectorAll('.sp-nav-btn:not(.sp-nav-dis)').forEach(function(btn){
  btn.onclick=function(ev){ev.stopPropagation();
   var nd=tipPos+parseInt(btn.getAttribute('data-d'),10);
   if(nd>=0&amp;&amp;nd<tipIdxs.length){tipPos=nd;renderTip();}};});}
var dragging=false,dsx=0,dsy=0,moved=false,pinned=false;

wrap.addEventListener('mouseleave',function(e){
 if(pinned)return;
 var rt=e.relatedTarget;
 if(rt&amp;&amp;(rt===tip||tip.contains(rt)))return;
 tip.classList.remove('sp-vis');tipIdxs=[];});
wrap.addEventListener('mousemove',function(e){
 if(dragging||pinned)return;
 lastE=e;

 var hits=[];
 document.elementsFromPoint(e.clientX,e.clientY).forEach(function(el){
  if(el===tip||tip.contains(el))return;
  var found=null;
  for(var n=el;n&amp;&amp;n!==document.body;n=n.parentElement){
   if(found===null&amp;&amp;n.getAttribute&amp;&amp;n.getAttribute('data-idx')!==null)found=n;
   if(n===wrap){
    if(found!==null){var idx=parseInt(found.getAttribute('data-idx'),10);
     if(hits.indexOf(idx)===-1)hits.push(idx);}
    return;}
  }
 });
 if(hits.length){
  var same=hits.length===tipIdxs.length&amp;&amp;hits.every(function(v,i){return v===tipIdxs[i];});
  if(!same){tipIdxs=hits;tipPos=0;renderTip();}else placeTip(e);
 }else{tip.classList.remove('sp-vis');tipIdxs=[];}});

var origVB=svg.getAttribute('viewBox')||'';
function parseVB(s){return s.split(/[\s,]+/).map(Number);}
function animateVB(x,y,w,h){
 svg.style.transition='all 0.5s cubic-bezier(.4,0,.2,1)';
 svg.setAttribute('viewBox',x+' '+y+' '+w+' '+h);}
function resetVB(){
 svg.style.transition='all 0.45s cubic-bezier(.4,0,.2,1)';
 if(origVB)svg.setAttribute('viewBox',origVB);
 setTimeout(function(){svg.style.transition='';},500);}
function reAnim(){var els=svg.querySelectorAll('[data-idx]');els.forEach(function(el){el.style.animation='none';el.style.filter='';});void svg.offsetHeight;els.forEach(function(el,i){el.style.animation='';el.style.animationDelay=(i*14)+'ms';});}
var ov=wrap.querySelector('.sp-sel-ov');var panel=wrap.querySelector('.sp-cpanel');
wrap.addEventListener('mousedown',function(e){
 if(e.button!==0)return;
 var t=e.target;if(t===panel||panel.contains(t))return;if(t===tip||tip.contains(t))return;
 dragging=true;moved=false;
 var r=wrap.getBoundingClientRect();dsx=e.clientX-r.left;dsy=e.clientY-r.top;
 ov.style.cssText='display:none;position:absolute;pointer-events:none;border:2px solid #6366F1;background:rgba(99,102,241,.12);box-sizing:border-box;border-radius:3px;z-index:10';
 e.preventDefault();});
document.addEventListener('mousemove',function(e){if(!dragging)return;
 var r=wrap.getBoundingClientRect();var cx=e.clientX-r.left,cy=e.clientY-r.top;
 if(!moved&amp;&amp;Math.abs(cx-dsx)<8&amp;&amp;Math.abs(cy-dsy)<8)return;
 if(!moved){pinned=false;tip.classList.remove('sp-vis');tipIdxs=[];}
 moved=true;ov.style.display='block';
 ov.style.left=Math.min(dsx,cx)+'px';ov.style.top=Math.min(dsy,cy)+'px';
 ov.style.width=Math.abs(cx-dsx)+'px';ov.style.height=Math.abs(cy-dsy)+'px';});
document.addEventListener('mouseup',function(e){if(!dragging)return;dragging=false;
 ov.style.display='none';if(!moved){
 var ch=[];document.elementsFromPoint(e.clientX,e.clientY).forEach(function(el){if(el===tip||tip.contains(el))return;var fd=null;for(var n2=el;n2&amp;&amp;n2!==document.body;n2=n2.parentElement){if(fd===null&amp;&amp;n2.getAttribute&amp;&amp;n2.getAttribute('data-idx')!==null)fd=n2;if(n2===wrap){if(fd!==null){var idx=parseInt(fd.getAttribute('data-idx'),10);if(ch.indexOf(idx)===-1)ch.push(idx);}return;}}});
 if(ch.length){pinned=true;tipIdxs=ch;tipPos=0;lastE=e;renderTip();}else{pinned=false;tip.classList.remove('sp-vis');tipIdxs=[];}return;}
 var r=wrap.getBoundingClientRect();var cx=e.clientX-r.left,cy=e.clientY-r.top;
 var rx1=Math.min(dsx,cx),ry1=Math.min(dsy,cy),rx2=Math.max(dsx,cx),ry2=Math.max(dsy,cy);
 if(rx2-rx1<8&amp;&amp;ry2-ry1<8)return;
 var ctm=svg.getScreenCTM();if(!ctm)return;var inv=ctm.inverse();
 function toS(px,py){var pt=svg.createSVGPoint();pt.x=px+r.left;pt.y=py+r.top;return pt.matrixTransform(inv);}
 var p1=toS(rx1,ry1),p2=toS(rx2,ry2);
 var bx1=Math.min(p1.x,p2.x),by1=Math.min(p1.y,p2.y),bx2=Math.max(p1.x,p2.x),by2=Math.max(p1.y,p2.y);
 var pts=svg.querySelectorAll('[data-idx]');var sel=[],unsel=[];
 pts.forEach(function(el){try{var bb=el.getBBox();var ecx=bb.x+bb.width/2,ecy=bb.y+bb.height/2;
  if(ecx>=bx1&amp;&amp;ecx<=bx2&amp;&amp;ecy>=by1&amp;&amp;ecy<=by2)sel.push(el);else unsel.push(el);}catch(ex){unsel.push(el);}});
 if(!sel.length)return;
 sel.forEach(function(el){el.style.stroke='#6366F1';el.style.strokeWidth='2.5';el.style.opacity='';el.style.transform='';});
 unsel.forEach(function(el){
  el.style.transformBox='fill-box';el.style.transformOrigin='center';
  el.style.transition='transform 0.28s cubic-bezier(.4,0,.2,1),opacity 0.28s';
  el.style.transform='scale(0,0)';el.style.opacity='0';
  setTimeout(function(){el.style.display='none';el.style.transition='';},300);});

 var mnx=Infinity,mny=Infinity,mxx=-Infinity,mxy=-Infinity;
 sel.forEach(function(el){try{var bb=el.getBBox();
  mnx=Math.min(mnx,bb.x);mny=Math.min(mny,bb.y);
  mxx=Math.max(mxx,bb.x+bb.width);mxy=Math.max(mxy,bb.y+bb.height);}catch(ex){}});
 var vb=parseVB(origVB.length?origVB:(svg.getAttribute('viewBox')||'0 0 800 500'));
 var axL=mnx-vb[0],axT=mny-vb[1],axR=(vb[0]+vb[2])-mxx,axB=(vb[1]+vb[3])-mxy;
 var sw=mxx-mnx||1,sh=mxy-mny||1;
 var pL=Math.max(sw*0.12,axL>0?axL*0.7:sw*0.12);
 var pR=Math.max(sw*0.08,axR>0?axR*0.5:sw*0.08);
 var pT=Math.max(sh*0.12,axT>0?axT*0.7:sh*0.12);
 var pB=Math.max(sh*0.20,axB>0?axB*0.8:sh*0.20);
 animateVB(mnx-pL,mny-pT,(mxx+pR)-(mnx-pL),(mxy+pB)-(mny-pT));

 var xs=sel.map(function(el){return parseFloat(el.getAttribute('data-x'));}).filter(Number.isFinite);
 var ys=sel.map(function(el){return parseFloat(el.getAttribute('data-y'));}).filter(Number.isFinite);
 var s='<span style=&quot;color:#6366F1;font-weight:700&quot;>'+sel.length+' pts</span>';
 if(xs.length&amp;&amp;ys.length){
  var mx=xs.reduce(function(a,b){return a+b;})/xs.length;
  var my=ys.reduce(function(a,b){return a+b;})/ys.length;
  var vX=xs.map(function(v){return(v-mx)*(v-mx);}).reduce(function(a,b){return a+b;})/xs.length;
  var vY=ys.map(function(v){return(v-my)*(v-my);}).reduce(function(a,b){return a+b;})/ys.length;
  s+=' &amp;middot; X&amp;#772; <b>'+mx.toFixed(2)+'</b> &amp;plusmn;'+Math.sqrt(vX).toFixed(2);
  s+=' &amp;middot; Y&amp;#772; <b>'+my.toFixed(2)+'</b> &amp;plusmn;'+Math.sqrt(vY).toFixed(2);}
 s+=' <span class=&quot;sp-cls-x&quot;>&amp;#10005;</span>';
 panel.innerHTML=s;panel.style.display='block';
 panel.querySelector('.sp-cls-x').addEventListener('click',clearSel);});
function clearSel(){panel.style.display='none';resetVB();
 svg.querySelectorAll('[data-idx]').forEach(function(el){
  el.style.display='';el.style.opacity='';el.style.stroke='';el.style.strokeWidth='';el.style.filter='';
  el.style.transform='';el.style.transition='';el.style.transformBox='';el.style.transformOrigin='';});
 reAnim();}
document.addEventListener('keydown',function(e){if(e.key==='Escape'){if(dblZoomed){dblZoomed=false;resetVB();svg.querySelectorAll('[data-idx]').forEach(function(el){el.style.opacity='';el.style.display='';el.style.transform='';el.style.transition='';});reAnim();}if(pinned){pinned=false;tip.classList.remove('sp-vis');tipIdxs=[];}}});
var dblZoomed=false;
svg.addEventListener('dblclick',function(e){
 if(dblZoomed){dblZoomed=false;resetVB();svg.querySelectorAll('[data-idx]').forEach(function(el){el.style.opacity='';el.style.display='';el.style.transform='';el.style.transition='';el.style.filter='';});reAnim();return;}
 var found=null;
 for(var nd=e.target;nd&amp;&amp;nd!==svg;nd=nd.parentElement){if(nd.getAttribute&amp;&amp;nd.getAttribute('data-idx')!==null){found=nd;break;}}
 if(!found)return;
 e.stopPropagation();
 var idx=parseInt(found.getAttribute('data-idx'),10);
 var bb;try{bb=found.getBBox();}catch(ex){return;}
 var pad=Math.max(bb.width,bb.height)*1.5+30;
 svg.querySelectorAll('[data-idx]').forEach(function(el){
  var eli=parseInt(el.getAttribute('data-idx'),10);
  if(eli===idx){el.style.opacity='1';el.style.filter='drop-shadow(0 0 6px rgba(99,102,241,.45))';}else{el.style.display='none';}});
 var vb=parseVB(origVB.length?origVB:(svg.getAttribute('viewBox')||'0 0 800 500'));
 var nx=Math.max(vb[0],bb.x-pad),ny=Math.max(vb[1],bb.y-pad);
 var nw=Math.min(vb[2],bb.width+pad*2),nh=Math.min(vb[3],bb.height+pad*2);
 animateVB(nx,ny,nw,nh);
 dblZoomed=true;
},false);
var legs=svg.querySelectorAll('[data-legend]');
legs.forEach(function(lg){lg.style.cursor='pointer';
lg.addEventListener('click',function(){
 var s=lg.getAttribute('data-series');if(!s)return;
 var h=lg.getAttribute('data-hidden')==='1';
 var els=svg.querySelectorAll('[data-series=&quot;'+s+'&quot;]:not([data-legend])');
 if(h){els.forEach(function(el){el.style.display='';el.style.opacity='';el.style.transform='';el.style.transformBox='';el.style.transformOrigin='';el.style.transition='';});lg.style.opacity='';lg.removeAttribute('data-hidden');}
 else{els.forEach(function(el){
  el.style.transformBox='fill-box';el.style.transformOrigin='center';
  el.style.transition='transform 0.3s cubic-bezier(.4,0,.2,1),opacity 0.3s';
  el.style.transform='scale(0,0)';el.style.opacity='0';
  setTimeout(function(){el.style.display='none';el.style.transition='';},320);});
  lg.style.opacity='0.3';lg.setAttribute('data-hidden','1');}spRescale();});});
function spRescale(){
 var m=svg.getAttribute('data-sp');if(!m)return;
 var p=m.split(',').map(Number),pL=p[0],pT=p[1],pW=p[2],pH=p[3];
 var vals=[];
 svg.querySelectorAll('[data-pts]').forEach(function(el){if(el.style.display==='none')return;el.getAttribute('data-pts').split(',').forEach(function(v){var f=parseFloat(v);if(isFinite(f))vals.push(f);});});
 svg.querySelectorAll('circle[data-y]').forEach(function(el){if(el.style.display==='none')return;var f=parseFloat(el.getAttribute('data-y'));if(isFinite(f))vals.push(f);});
 svg.querySelectorAll('rect[data-y]').forEach(function(el){if(el.style.display==='none')return;var f=parseFloat(el.getAttribute('data-y'));if(isFinite(f))vals.push(f);});
 svg.querySelectorAll('rect[data-v]').forEach(function(el){if(el.style.display==='none')return;if(el.hasAttribute('data-y'))return;var f=parseFloat(el.getAttribute('data-v'));if(isFinite(f))vals.push(f);});
 if(!vals.length)return;
 var mn=Math.min.apply(null,vals),mx=Math.max.apply(null,vals);if(mn>0)mn=0;var rg=mx-mn;if(rg<1e-12)rg=1;
 svg.querySelectorAll('polyline[data-pts]').forEach(function(el){if(el.style.display==='none')return;var vs=el.getAttribute('data-pts').split(','),n=vs.length,sx=pW/Math.max(n-1,1),r='';for(var i=0;i<n;i++){var f=(parseFloat(vs[i])-mn)/rg;if(i>0)r+=' ';r+=(pL+Math.round(i*sx))+','+(pT+Math.round((1-f)*pH));}el.setAttribute('points',r);});
 svg.querySelectorAll('circle[data-y]').forEach(function(el){if(el.style.display==='none')return;var f=(parseFloat(el.getAttribute('data-y'))-mn)/rg;el.setAttribute('cy',pT+Math.round((1-f)*pH));});
 svg.querySelectorAll('rect[data-y]').forEach(function(el){if(el.style.display==='none')return;var v=parseFloat(el.getAttribute('data-y'));if(!isFinite(v))return;var f=(v-mn)/rg;var ny=pT+Math.round((1-f)*pH);var base=pT+pH;if(mn<0){base=pT+Math.round((0-mn)/rg*pH);}if(v>=0){el.setAttribute('y',ny);el.setAttribute('height',Math.max(1,base-ny));}else{el.setAttribute('y',base);el.setAttribute('height',Math.max(1,ny-base));}});
 svg.querySelectorAll('rect[data-v]').forEach(function(el){if(el.style.display==='none')return;if(el.hasAttribute('data-y'))return;var v=parseFloat(el.getAttribute('data-v'));if(!isFinite(v))return;var f=(v-mn)/rg;var ny=pT+Math.round((1-f)*pH);var base=pT+pH;if(mn<0){base=pT+Math.round((0-mn)/rg*pH);}if(v>=0){el.setAttribute('y',ny);el.setAttribute('height',Math.max(1,base-ny));}else{el.setAttribute('y',base);el.setAttribute('height',Math.max(1,ny-base));}});
 var tks=svg.querySelectorAll('.sp-yt'),nT=tks.length;if(nT>1)for(var i=0;i<nT;i++){var f=i/(nT-1);var v=mn+f*rg;tks[i].textContent=v>=1000?Math.round(v)+'':v.toFixed(2);tks[i].setAttribute('y',pT+Math.round((1-f)*pH)+3);}
 var gls=svg.querySelectorAll('.sp-gl');if(gls.length&amp;&amp;nT>1)for(var j=0;j<gls.length;j++){var f=(j+1)/(nT-1);var gy=pT+Math.round((1-f)*pH);gls[j].setAttribute('y1',gy);gls[j].setAttribute('y2',gy);}
}
svg.querySelectorAll('[data-idx]').forEach(function(el,i){el.style.animationDelay=(i*18)+'ms';});
})();</script></div></body></html>"></iframe></div></div></body></html>
</div>

</details>

---

## See also

- [Slideshow](slideshow.md)
