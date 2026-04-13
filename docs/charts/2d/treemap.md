# Treemap

## Signature

```python
sp.build_treemap(
    title: str,
    labels: list[str],
    values: list[float],
    *,
    parents: list[str] | None = None,
    width: int = 900,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    hover_json: str | None = None,
) -> Chart
```

---

## Description

Treemap — square-based space-filling hierarchy visualization.
Tiles are sized proportionally to their value.

When `parents` is provided, the hierarchy is rendered as nested rectangles.
Without `parents`, a flat treemap is drawn.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `labels` | `list[str]` | required | Tile labels |
| `values` | `list[float]` | required | Tile sizes |
| `parents` | `list[str] \| None` | `None` | Optional parent labels for hierarchy |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `palette` | `list[int] \| None` | `None` | Custom color palette |
| `background` | `str \| None` | `None` | Background color |
| `hover_json` | `str \| None` | `None` | Custom hover JSON |

---

## Returns

`Chart`

---

## Examples

### Flat treemap (market cap)

```python
import seraplot as sp

chart = sp.build_treemap(
    "S&P 500 Market Cap (Sample)",
    labels=["AAPL", "MSFT", "AMZN", "NVDA", "GOOGL", "META", "TSLA"],
    values=[2900, 2800, 1700, 1600, 1500, 1100, 750],
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe srcdoc="<!DOCTYPE html><html><head><meta charset=&quot;utf-8&quot;><title>S&amp;amp;P 500 Market Cap (Sample)</title><style>#sp-tip{position:absolute;z-index:999999;pointer-events:none;opacity:0;transition:opacity .15s,transform .15s;transform:translateY(6px) scale(.97);background:#0b0e18;color:#f1f5f9;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;font-size:13px;border-radius:10px;min-width:160px;max-width:340px;box-shadow:0 4px 20px rgba(0,0,0,.45),0 0 0 1px rgba(255,255,255,.08);overflow:hidden}#sp-tip.sp-vis{opacity:1;transform:translateY(0) scale(1);pointer-events:auto}.sp-nav{display:flex;align-items:center;justify-content:space-between;padding:5px 10px;border-top:1px solid rgba(255,255,255,.08)}.sp-nav-btn{cursor:pointer;padding:0 10px;border-radius:5px;height:22px;line-height:22px;font-size:18px;background:rgba(255,255,255,.10);color:#e2e8f0;user-select:none;flex-shrink:0}.sp-nav-btn:hover{background:rgba(255,255,255,.22)}.sp-nav-dis{opacity:.25;pointer-events:none}.sp-nav-ctr{flex:1;text-align:center;font-size:11px;color:#94a3b8}.sp-head{padding:10px 14px 6px;font-weight:700;font-size:14px;color:#e2e8f0;border-bottom:1px solid rgba(255,255,255,.08)}.sp-body{padding:8px 14px 12px}.sp-row{display:flex;justify-content:space-between;align-items:baseline;gap:14px;padding:3px 0;border-bottom:1px solid rgba(255,255,255,.04)}.sp-row:last-child{border-bottom:none}.sp-key{color:#94a3b8;font-size:12px;white-space:nowrap}.sp-val{font-weight:600;font-size:12px;color:#f8fafc;text-align:right;word-break:break-all}#sp-tip img{display:block;width:100%;max-height:210px;object-fit:contain;border-top:1px solid rgba(255,255,255,.07)}#sp-tip video{display:block;width:100%;border-top:1px solid rgba(255,255,255,.07)}.sp-html{padding:8px 14px;font-size:12px;border-top:1px solid rgba(255,255,255,.07)}[data-idx]{cursor:pointer;transition:opacity .25s,filter .2s,transform .25s}[data-idx]:hover{filter:brightness(1.12) saturate(1.08)}.sp-cpanel{position:absolute;bottom:10px;left:50%;transform:translateX(-50%);background:#0b0e18;color:#f1f5f9;border-radius:10px;padding:8px 16px;font-size:12px;font-family:-apple-system,Arial,sans-serif;box-shadow:0 8px 24px rgba(0,0,0,.4);z-index:20;white-space:nowrap;display:none}.sp-cls-x{cursor:pointer;color:#94a3b8;margin-left:6px;font-size:13px}.sp-cls-x:hover{color:#f87171}</style><style>body{margin:0;background:#0d1117;display:flex;justify-content:center;padding:0}@keyframes sp-i{from{opacity:0;transform:translateY(8px) scale(.94)}}@keyframes sp-bar{from{opacity:0;transform:scaleY(0)}}@keyframes sp-pop{0%{opacity:0;transform:scale(0)}70%{transform:scale(1.08)}100%{opacity:1;transform:scale(1)}}@keyframes sp-arc{from{opacity:0;transform:scale(.82) rotate(-6deg)}}@keyframes sp-fn{from{opacity:0;transform:scaleX(.7) translateY(8px)}}svg rect[data-idx]{transform-box:fill-box;transform-origin:bottom center;animation:sp-bar .5s cubic-bezier(.22,.61,.36,1) both}svg circle[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-pop .42s cubic-bezier(.34,1.56,.64,1) both}svg path[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-arc .48s cubic-bezier(.22,.61,.36,1) both}svg polygon[data-idx]{transform-box:fill-box;transform-origin:center;animation:sp-fn .48s cubic-bezier(.22,.61,.36,1) both}svg line[data-idx]{animation:sp-i .45s cubic-bezier(.22,.61,.36,1) both}svg rect[data-idx]:hover{transform:scaleY(1.03);filter:brightness(1.12) saturate(1.1)}svg circle[data-idx]:hover{transform:scale(1.3);filter:brightness(1.15)}svg path[data-idx]:hover{filter:brightness(1.1) drop-shadow(0 2px 8px rgba(0,0,0,.2))}.sp-bg{fill:#fff}</style><style>.sp-bg{fill:transparent!important}.sp-ttl{fill:#e2e8f0!important}svg text{fill:#cbd5e1!important}.sp-ax-x,.sp-ax-y{stroke:#475569!important}.sp-gl{stroke:#2d3748!important}.sp-xl,.sp-yl{fill:#94a3b8!important}[id^='spp']{box-shadow:none!important;border-radius:0!important}</style></head><body><div id=&quot;spp1&quot; style=&quot;position:relative;display:inline-block;border-radius:12px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,.07),0 0 0 1px rgba(0,0,0,.04)&quot;><svg xmlns=&quot;http://www.w3.org/2000/svg&quot; width=&quot;1100&quot; height=&quot;520&quot; viewBox=&quot;0 0 1100 520&quot;><rect class=&quot;sp-bg&quot; width=&quot;100%&quot; height=&quot;100%&quot;/><text x=&quot;550&quot; y=&quot;24&quot; text-anchor=&quot;middle&quot; font-family=&quot;-apple-system,Arial,sans-serif&quot; font-size=&quot;15&quot; font-weight=&quot;700&quot; fill=&quot;#1a202c&quot;>S&amp;amp;P 500 Market Cap (Sample)</text><rect data-idx=&quot;0&quot; data-lbl=&quot;AAPL&quot; data-v=&quot;2900.00&quot; data-kv-Pct=&quot;23.48%&quot; x=&quot;4.00&quot; y=&quot;38.00&quot; width=&quot;256.42&quot; height=&quot;478.00&quot; rx=&quot;4&quot; fill=&quot;#6366f1&quot; stroke=&quot;#fff&quot; stroke-width=&quot;2&quot;/><text x=&quot;132.21&quot; y=&quot;277.00&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;12&quot; font-weight=&quot;600&quot; fill=&quot;#fff&quot; dy=&quot;-0.3em&quot;>AAPL</text><text x=&quot;132.21&quot; y=&quot;277.00&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;11&quot; fill=&quot;rgba(255,255,255,0.8)&quot; dy=&quot;1.0em&quot;>23.48%</text><rect data-idx=&quot;1&quot; data-lbl=&quot;MSFT&quot; data-v=&quot;2800.00&quot; data-kv-Pct=&quot;22.67%&quot; x=&quot;260.42&quot; y=&quot;38.00&quot; width=&quot;247.58&quot; height=&quot;478.00&quot; rx=&quot;4&quot; fill=&quot;#f43f5e&quot; stroke=&quot;#fff&quot; stroke-width=&quot;2&quot;/><text x=&quot;384.21&quot; y=&quot;277.00&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;12&quot; font-weight=&quot;600&quot; fill=&quot;#fff&quot; dy=&quot;-0.3em&quot;>MSFT</text><text x=&quot;384.21&quot; y=&quot;277.00&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;11&quot; fill=&quot;rgba(255,255,255,0.8)&quot; dy=&quot;1.0em&quot;>22.67%</text><rect data-idx=&quot;2&quot; data-lbl=&quot;AMZN&quot; data-v=&quot;1700.00&quot; data-kv-Pct=&quot;13.77%&quot; x=&quot;508.00&quot; y=&quot;38.00&quot; width=&quot;291.79&quot; height=&quot;246.24&quot; rx=&quot;4&quot; fill=&quot;#10b981&quot; stroke=&quot;#fff&quot; stroke-width=&quot;2&quot;/><text x=&quot;653.89&quot; y=&quot;161.12&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;12&quot; font-weight=&quot;600&quot; fill=&quot;#fff&quot; dy=&quot;-0.3em&quot;>AMZN</text><text x=&quot;653.89&quot; y=&quot;161.12&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;11&quot; fill=&quot;rgba(255,255,255,0.8)&quot; dy=&quot;1.0em&quot;>13.77%</text><rect data-idx=&quot;3&quot; data-lbl=&quot;NVDA&quot; data-v=&quot;1600.00&quot; data-kv-Pct=&quot;12.96%&quot; x=&quot;508.00&quot; y=&quot;284.24&quot; width=&quot;291.79&quot; height=&quot;231.76&quot; rx=&quot;4&quot; fill=&quot;#f59e0b&quot; stroke=&quot;#fff&quot; stroke-width=&quot;2&quot;/><text x=&quot;653.89&quot; y=&quot;400.12&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;12&quot; font-weight=&quot;600&quot; fill=&quot;#fff&quot; dy=&quot;-0.3em&quot;>NVDA</text><text x=&quot;653.89&quot; y=&quot;400.12&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;11&quot; fill=&quot;rgba(255,255,255,0.8)&quot; dy=&quot;1.0em&quot;>12.96%</text><rect data-idx=&quot;4&quot; data-lbl=&quot;GOOGL&quot; data-v=&quot;1500.00&quot; data-kv-Pct=&quot;12.15%&quot; x=&quot;799.79&quot; y=&quot;38.00&quot; width=&quot;296.21&quot; height=&quot;214.03&quot; rx=&quot;4&quot; fill=&quot;#8b5cf6&quot; stroke=&quot;#fff&quot; stroke-width=&quot;2&quot;/><text x=&quot;947.89&quot; y=&quot;145.01&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;12&quot; font-weight=&quot;600&quot; fill=&quot;#fff&quot; dy=&quot;-0.3em&quot;>GOOGL</text><text x=&quot;947.89&quot; y=&quot;145.01&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;11&quot; fill=&quot;rgba(255,255,255,0.8)&quot; dy=&quot;1.0em&quot;>12.15%</text><rect data-idx=&quot;5&quot; data-lbl=&quot;META&quot; data-v=&quot;1100.00&quot; data-kv-Pct=&quot;8.91%&quot; x=&quot;799.79&quot; y=&quot;252.03&quot; width=&quot;176.13&quot; height=&quot;263.97&quot; rx=&quot;4&quot; fill=&quot;#06b6d4&quot; stroke=&quot;#fff&quot; stroke-width=&quot;2&quot;/><text x=&quot;887.85&quot; y=&quot;384.01&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;12&quot; font-weight=&quot;600&quot; fill=&quot;#fff&quot; dy=&quot;-0.3em&quot;>META</text><text x=&quot;887.85&quot; y=&quot;384.01&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;11&quot; fill=&quot;rgba(255,255,255,0.8)&quot; dy=&quot;1.0em&quot;>8.91%</text><rect data-idx=&quot;6&quot; data-lbl=&quot;TSLA&quot; data-v=&quot;750.00&quot; data-kv-Pct=&quot;6.07%&quot; x=&quot;975.91&quot; y=&quot;252.03&quot; width=&quot;120.09&quot; height=&quot;263.97&quot; rx=&quot;4&quot; fill=&quot;#ec4899&quot; stroke=&quot;#fff&quot; stroke-width=&quot;2&quot;/><text x=&quot;1035.96&quot; y=&quot;384.01&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;12&quot; font-weight=&quot;600&quot; fill=&quot;#fff&quot; dy=&quot;-0.3em&quot;>TSLA</text><text x=&quot;1035.96&quot; y=&quot;384.01&quot; text-anchor=&quot;middle&quot; font-family=&quot;Arial,sans-serif&quot; font-size=&quot;11&quot; fill=&quot;rgba(255,255,255,0.8)&quot; dy=&quot;1.0em&quot;>6.07%</text></svg><div class=&quot;sp-sel-ov&quot; style=&quot;display:none&quot;></div><div class=&quot;sp-cpanel&quot;></div><script>(function(){
var wrap=document.getElementById('spp1');if(!wrap)return;wrap.removeAttribute('id');
var svg=wrap.querySelector('svg');var data=[{&quot;title&quot;:&quot;AAPL&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;MSFT&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;AMZN&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;NVDA&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;GOOGL&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;META&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;},{&quot;title&quot;:&quot;TSLA&quot;,&quot;kv&quot;:[],&quot;image&quot;:&quot;https://raw.githubusercontent.com/feur25/seraplot/main/asset/logo.png&quot;}];

var tip=document.getElementById('sp-tip');
if(!tip){tip=document.createElement('div');tip.id='sp-tip';document.body.appendChild(tip);}
var tipIdxs=[],tipPos=0,lastE=null;
function esc(s){return String(s).replace(/&amp;/g,'&amp;amp;').replace(/</g,'&amp;lt;');}
function cardHTML(d){
 var h='';if(d.title)h+='<div class=&quot;sp-head&quot;>'+esc(d.title)+'</div>';
 var rows='';(d.kv||[]).forEach(function(p){
  rows+='<div class=&quot;sp-row&quot;><span class=&quot;sp-key&quot;>'+esc(p[0])+'</span><span class=&quot;sp-val&quot;>'+esc(p[1])+'</span></div>';});
 if(rows)h+='<div class=&quot;sp-body&quot;>'+rows+'</div>';
 if(d.image)h+='<img src=&quot;'+esc(d.image)+'&quot; alt=&quot;&quot; loading=&quot;eager&quot;>';
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
})();</script></div></body></html>" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117"></iframe>

</details>

### Hierarchical treemap

```python
import seraplot as sp

labels  = ["Electronics", "Phones", "Laptops", "Clothing", "Shirts", "Pants"]
parents = ["",            "Electronics","Electronics","","Clothing","Clothing"]
values  = [1, 400, 350, 1, 200, 150]

chart = sp.build_treemap(
    "Revenue by Category",
    labels=labels,
    values=values,
    parents=parents,
)
```

---

## See also

- [Sunburst](sunburst.md)
- [Bar Chart](bar.md)
