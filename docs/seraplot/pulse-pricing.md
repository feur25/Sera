# Sera Pulse — Pricing & Activation

<div class="lang-en">

Sera Pulse is the streaming layer on top of every native chart: `.push()`, `.push_vector()`, `.serve()`, `.record()`, `.replay()`, `.on_anomaly()`. The rendering itself (`sp.scatter()`, `sp.bar()`, `sp.heatmap()`, `sp.candlestick()`, and every other native fast-path chart) stays free and open-source — Pulse only gates the *incremental update* layer built on top of it.

### What's included

Everything below is under one gate — the same active trial or license unlocks all of it, Solo and Team alike (Team only adds seats and priority support, not extra features):

- **Streaming** — `push()`, `push_vector()`, `serve()`, `record()`/`replay()`, `on_anomaly()`, `export_standalone()`.
- **Sera Firehose** — `sp.firehose(...)` + `firehose_push()`, a ring-buffered chart for very high-frequency streams, rendered through WebGL2.
- **Sera Live** — live cursors, pinned comments, a shared "replay together" on any `.serve()`'d chart, and a one-click dark/light theme toggle.
- **Sera Board** — `serve_board()`/`export_board_html()`, a Figma-style infinite whiteboard: draggable/zoomable chart frames, pins, freehand drawings, dropped images, all live and collaborative and saved to a persistent session you can switch between with one click, or exported as a static file.
- **Sera Secure** — `SeraDFrame`, AES-256-GCM encryption on entry, plaintext never cached or logged.
- **SeraReport** — `export_pdf()`, `export_docx()`, `export_pdf_report()`, each with an optional live companion link.
- **SeraStudio** — `export_video()`, `export_gif()`, session recording/replay for shareable chart clips.

Every method above calls into the same license check — one key activates on exactly one machine, and every price below is locked in for good: new paid features join this same catalog at no extra cost, they never move behind a new tier.

---

## Free trial

The first time any Pulse method is called on a machine, a 90-day (3-month) trial starts automatically — **no card, no signup, no network call**. You'll see it reflected in:

```python
import seraplot as sp
sp.pulse_status()
# {'state': 'trial', 'days_left': '90'}
```

Everything (`push`, `push_vector`, `serve`, `record`, `replay`, `on_anomaly`) works exactly the same during the trial as with a paid license — there is no feature difference, only a time limit.

---

## After the trial

Once the 90 days are up, Pulse methods raise a clear `PermissionError` pointing you to activation rather than failing silently or degrading:

```python
sp.scatter(x=x, y=y).push(idx, x, y)
# PermissionError: seraplot pulse: free trial (90 days) has ended.
# Activate a license with sp.pulse_activate(key) to keep using
# push()/serve()/record()/replay()/on_anomaly().
```

Activate with the key you receive after purchase:

```python
sp.pulse_activate("eyJjIjoi...ZTFhMTki.MEUCIQDx...")
sp.pulse_status()
# {'state': 'licensed', 'customer': '...', 'plan': 'pro', 'expires_at': '...'}
```

A license key is a self-contained, cryptographically signed token — activation works fully offline, no phone-home, no telemetry.

Machine-locked plans (see Solo below) are tied to a local machine id rather than being freely copyable between computers:

```python
sp.pulse_machine_id()
# 'DESKTOP-ABC123-alice-...'
```

Send this id when purchasing a machine-locked key — it gets embedded in the signed license and checked on activation.

---

## Plans

| Plan | Price | Includes | |
|---|---|---|---|
| **Trial** | Free | Everything, 90 days (3 months), no card required. | [Get started](#free-trial) |
| **Solo** | 10,97 €/seat/mo or 109,70 €/seat/yr | `push`/`serve`/`record`/`replay`/`on_anomaly`, locked to one machine id (technically enforced, not just a policy). | [Buy monthly](https://sera-payment.onrender.com/paypal/buy/solo/monthly) · [Buy yearly](https://sera-payment.onrender.com/paypal/buy/solo/annual) |
| **Team** | 10,97 €/seat/mo or 109,70 €/seat/yr — same rate as Solo | Solo, once per seat — one machine-locked key per teammate, plus priority support. | pick your seats & months below |

Paid via PayPal — click a buy link, pay on PayPal's hosted checkout page, and your license key generates automatically the moment the payment is confirmed: a signed webhook triggers minting, checked against PayPal's own signature so nothing can fake a payment. Prefer crypto (USDC/Polygon, self-custodied) or a manual sale instead? Use the [support channel](../about/support.md).

This price is locked in for good: every paid feature this catalog gains later — Sera Firehose, Sera Secure, SeraReport, SeraStudio, whatever ships next — joins the same Solo/Team price above at no extra cost. There is no higher tier waiting behind a paywall.

Each key activates on exactly one machine — always. Team isn't a shared multi-seat key, it's one Solo-equivalent key per teammate, each locked to that teammate's own machine, billed together.

<div class="sp-team-picker" id="sp-team-picker-en"></div>

<small>Type any number of seats and any number of months (1 to 36) — the link charges the plain per-seat-per-month rate times that many seats and months, except exactly 12 months which uses the discounted annual per-seat rate shown above (same rate whether you type `12` here or use a yearly preset elsewhere — never a hidden or different price). Same pattern works for Solo: `.../paypal/buy/solo/<months>?seats=<seats>`.</small>

<small>Renewing an existing key keeps its customer identity and machine lock and just pushes the expiry out — it's a genuinely new signed token under the hood (a signature can't be edited after the fact), but same customer, same machine, same `sp.pulse_activate()` call on your end. Take your current key and drop it into `https://sera-payment.onrender.com/paypal/renew/solo/monthly?key=<your key>` (swap `solo`/`monthly` for your actual plan/period; works for `team` too, one seat's key at a time).</small>

---

## Sera Secure

`SecureDFrame`/`SecureDFrameBuilder`/`SeraKey` are AES-256-GCM encrypted-at-rest counterparts to `SeraDFrame` — columns stay ciphertext in memory, decrypted only transiently for a single read or chart render. They're part of the same paid catalog as the streaming methods above and share **the same trial/license** — no separate purchase, no separate activation:

```python
import seraplot as sp

key = sp.SeraKey.generate()
frame = sp.SecureDFrame({"x": x_values, "y": y_values}, key.to_bytes())
chart_data = frame.to_chart_data("x", "y", key.to_bytes(), max_points=2000)
```

### How it's laid out

<svg viewBox="0 0 900 210" style="width:100%;max-width:820px;height:auto;font-family:inherit" role="img" aria-label="SecureDFrame data flow: plaintext SeraDFrame is encrypted into a SecureDFrame, stored as ciphertext, then decrypted transiently only for a chart read">
  <defs>
    <marker id="sp-sec-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
      <path d="M0,0 L10,5 L0,10 z" fill="var(--icons)"/>
    </marker>
  </defs>
  <rect x="10" y="40" width="200" height="90" rx="10" fill="var(--theme-popup-bg)" stroke="var(--theme-popup-border, rgba(0,0,0,.2))"/>
  <text x="110" y="75" text-anchor="middle" font-size="14" font-weight="700" fill="var(--fg)">SeraDFrame</text>
  <text x="110" y="96" text-anchor="middle" font-size="11" fill="var(--icons)">plaintext columns</text>
  <text x="110" y="112" text-anchor="middle" font-size="11" fill="var(--icons)">in memory</text>
  <line x1="215" y1="85" x2="335" y2="85" stroke="var(--icons)" stroke-width="1.5" marker-end="url(#sp-sec-arrow)"/>
  <text x="275" y="72" text-anchor="middle" font-size="10.5" fill="var(--icons)">SeraKey +</text>
  <text x="275" y="105" text-anchor="middle" font-size="10.5" fill="var(--icons)">SecureDFrameBuilder</text>
  <rect x="340" y="30" width="220" height="110" rx="10" fill="var(--theme-popup-bg)" stroke="var(--sidebar-active)" stroke-width="1.5"/>
  <text x="450" y="60" text-anchor="middle" font-size="14" font-weight="700" fill="var(--fg)">SecureDFrame</text>
  <text x="450" y="82" text-anchor="middle" font-size="11" fill="var(--icons)">AES-256-GCM</text>
  <text x="450" y="98" text-anchor="middle" font-size="11" fill="var(--icons)">ciphertext at rest,</text>
  <text x="450" y="114" text-anchor="middle" font-size="11" fill="var(--icons)">never plaintext on disk</text>
  <text x="450" y="132" text-anchor="middle" font-size="16" fill="var(--sidebar-active)">&#128274;</text>
  <line x1="565" y1="85" x2="685" y2="85" stroke="var(--icons)" stroke-width="1.5" marker-end="url(#sp-sec-arrow)"/>
  <text x="625" y="72" text-anchor="middle" font-size="10.5" fill="var(--icons)">to_chart_data(key)</text>
  <text x="625" y="105" text-anchor="middle" font-size="10.5" fill="var(--icons)">transient decrypt</text>
  <rect x="690" y="40" width="200" height="90" rx="10" fill="var(--theme-popup-bg)" stroke="var(--theme-popup-border, rgba(0,0,0,.2))"/>
  <text x="790" y="75" text-anchor="middle" font-size="14" font-weight="700" fill="var(--fg)">Chart</text>
  <text x="790" y="96" text-anchor="middle" font-size="11" fill="var(--icons)">decrypted values live</text>
  <text x="790" y="112" text-anchor="middle" font-size="11" fill="var(--icons)">only for this render</text>
  <text x="450" y="185" text-anchor="middle" font-size="11" fill="var(--icons)">Same encrypted-at-rest guarantee for every chart family — nothing plaintext survives past the read that needed it.</text>
</svg>

### Why teams reach for this

A process dump, a stray core file, a debugger attached at the wrong moment — with a normal `SeraDFrame`, that's the whole column, in the clear. With `SecureDFrame`, it's ciphertext, full stop. The plaintext window is exactly one read: `to_chart_data()` decrypts, the chart renders, and there's nothing left sitting in memory to leak. If you're charting patient records, transaction history, PII, or anything else you'd rather not explain to a compliance officer after the fact, this is the one-line change (`SeraDFrame` → `SecureDFrame`) that makes the difference between "we had a memory leak" and "we had an incident."

It costs nothing extra to try — it's in the same trial and the same license as every other Pulse feature on this page, so there's no separate purchase decision to make. [Start the free trial](#free-trial) and swap one class name.

---

## SeraReport

`export_pdf()` converts a chart's own SVG directly into a native, vector PDF page — no headless browser, no screenshot, just a direct SVG-to-PDF conversion. `export_docx()` does the same into a Word document, the chart embedded as a real image. Same catalog, same trial/license as everything else on this page:

```python
import seraplot as sp

chart = sp.bar("Sales", labels=["Q1", "Q2", "Q3"], values=[120, 150, 90])
chart.export_pdf("sales.pdf")
chart.export_docx("sales.docx")
```

Needs the chart to have real SVG content — charts large enough to cross their native canvas-rendering threshold render via `<canvas>` instead of `<svg>` (the same boundary `export_svg()`/`export_png()` already have). `export_pdf_report(charts, path, titles=None)` combines several charts into one multi-page PDF, one page each.

Both methods take an `interactive` flag, `True` by default: alongside the PDF/DOCX, a companion `.html` file is written next to it — the chart's own real interactive page — and a clickable link is embedded in the document (a `/GoToR` link annotation in the PDF, a hyperlink paragraph in the DOCX) that opens it. Pass `interactive=False` for a plain static file with no companion:

```python
chart.export_pdf("sales.pdf", interactive=False)
sp.export_pdf_report(charts, "quarterly_report.pdf", titles=titles, interactive=True)
```

### Complete example

`export_pdf()` is a direct 1:1 conversion — the vector PDF page is exactly the chart's own SVG, nothing re-rendered or rasterized in between. `export_pdf_report()` does the same for a whole set of charts at once, one real page per chart. The PDF below was generated by the script underneath it — six different chart families (bar, line, scatter, pie, boxplot, heatmap), one multi-page report, open it right here:

<div class="sp-video-card" style="max-width:720px">
<iframe src="../previews/serareport-sample.pdf" style="width:100%;height:600px;border:1px solid var(--theme-popup-border, rgba(0,0,0,.2));border-radius:8px"></iframe>
<p><a href="../previews/serareport-sample.pdf" target="_blank" rel="noopener">Open in a new tab</a> if your browser doesn't preview PDFs inline.</p>
</div>

```python
import seraplot as sp

bar = sp.bar("Quarterly Sales", labels=["Q1", "Q2", "Q3", "Q4"], values=[120, 150, 90, 175], color_hex=0x3B82F6).show_grid().despine()
line = sp.line("Daily Active Users", x_labels=["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"], values=[820, 932, 901, 934, 1290, 1330, 1320], color_hex=0x22C55E).show_grid().despine()
scatter = sp.scatter("Latency vs Load", x=[10, 20, 30, 40, 50, 60, 70, 80], y=[12, 18, 22, 35, 40, 55, 61, 78], variant="regression").show_grid().despine()
pie = sp.pie("Traffic Sources", labels=["Organic", "Paid", "Referral", "Direct"], values=[45, 25, 15, 15])
boxplot = sp.boxplot("Response Time by Region", labels=["US"] * 6 + ["EU"] * 6 + ["APAC"] * 6, values=[120, 130, 125, 118, 140, 122, 200, 210, 195, 205, 220, 198, 90, 95, 88, 102, 97, 91])
heatmap = sp.heatmap("Weekly Activity", labels=["Mon", "Tue", "Wed", "Thu", "Fri"], col_labels=["8h", "12h", "16h", "20h"], values=[5, 9, 7, 3, 6, 12, 10, 4, 8, 15, 13, 7, 4, 8, 11, 5, 3, 7, 9, 2])

charts = [bar, line, scatter, pie, boxplot, heatmap]
titles = ["Quarterly Sales", "Daily Active Users", "Latency vs Load", "Traffic Sources", "Response Time by Region", "Weekly Activity"]
sp.export_pdf_report(charts, "quarterly_report.pdf", titles=titles)
```

---

## Sera Firehose

`sp.firehose(...)` is a chart built for one thing: absorbing a very fast stream of readings — sensor ticks, prices, request latencies — without the browser tab choking on it. It's a fixed-size ring buffer, server and browser alike: you never track write positions, you never grow an array forever. Once the buffer is full, each new reading silently overwrites the oldest one.

```python
import seraplot as sp

chart = sp.firehose("CPU %", capacity=2_000, min_val=0.0, max_val=100.0)
chart.serve(port=8787)
for reading in sensor_stream():
    chart.firehose_push([reading])
```

`firehose_push()` is `push()` with the indices computed for you — same rate limiting, same `on_anomaly()` detection, same `record()`/`replay()`, same `.serve()` broadcast over the same binary WebSocket frames every other native chart already streams over. Nothing about the transport is new; what's new is the client: instead of a `<canvas>` redrawn point-by-point on the CPU, Firehose renders through WebGL2 — one GPU buffer upload per animation frame, one `drawArrays` call, no per-point JavaScript work no matter how many readings landed since the last frame, in every current browser with nothing extra to install.

The ring buffer itself is a genuinely generic, reusable Rust type (`RingBuffer<T>`), not a one-off: fixed capacity, no reallocation once built, oldest-overwrite on push. In a release build it sustains over 200 million pushes/second doing pure buffer bookkeeping — the real ceiling for 100k+ readings/second is Python call overhead and the network, never the buffer.

### Live preview

This is the actual chart, actually rendering — not a screenshot. A tiny synthetic driver runs entirely in your browser, calling the exact same `sp_apply_<id>` function `firehose_push()` streams into, once per animation frame:

<div class="sp-video-card" style="max-width:720px">
<iframe src="../previews/firehose-demo.html" style="width:100%;height:340px;border:1px solid var(--theme-popup-border, rgba(0,0,0,.2));border-radius:8px;background:#fff"></iframe>
<p><a href="../previews/firehose-demo.html" target="_blank" rel="noopener">Open standalone</a> to see it full-size.</p>
</div>

```python
import random
import seraplot as sp

chart = sp.firehose("Live sensor feed", capacity=300, min_val=0.0, max_val=100.0, height=260)
chart.record("session.spls")
value = 50.0
for _ in range(1_200):
    value = max(0.0, min(100.0, value + random.uniform(-2.5, 2.5)))
    chart.firehose_push([value])
chart.stop_record()
chart.export_standalone("firehose-demo.html", "session.spls")
```

The embed above uses a small looping driver instead of a finite recording so it never runs out of data on this page, but it calls into the exact same `sp_apply_<id>` function either way — `export_standalone()` is the real, supported way to ship a self-contained demo like this one.

---

## Sera Live

A screen-share puts one person's window, at one person's scroll position, in front of everyone else squinting at a laggy stream and asking "wait, go back." Sera Live turns any `.serve()`'d chart into a real shared room instead — everyone who opens the link is *in* the chart together, full resolution, on their own screen.

```python
import seraplot as sp

chart = sp.bar("Error rate by region", labels=["us-east", "eu-west", "ap-south"], values=[12, 4, 31])
chart.record("incident.spls")
chart.serve(port=8787)
```

Send that link and every cursor in the room moves live, in real time, labeled and colored per person. Pin a note straight onto the bar that's spiking and it appears for everyone instantly — not a comment three scrolls down a thread, a label sitting exactly where the problem is. Circle it, underline it, draw an arrow at the thing everyone should be looking at, freehand, in whichever color makes the point land. Hit **Replay together** and every connected tab steps through the recorded incident in lockstep — reliving what happened at the same pace, together, instead of five people reconstructing the same timeline five different ways. One click flips the whole room into a dark theme built for a wall-mounted screen or a 2am war room, remembered per browser. Join four minutes late and you haven't missed anything — every pin and stroke made before you connected replays the moment you land.

None of this is a bolted-on side system: cursors, pins, drawings, and replay all ride the exact same WebSocket `.serve()` already opens. Pins are real `bookmark()` annotations, not a throwaway layer — list them later with `chart.bookmarks()` same as ever. Want the old single-viewer behavior instead, nothing about you or your cursor ever leaving your machine? `collab=False` turns it off:

```python
chart.serve(port=8787, collab=False)
```

<small>Closing a tab drops that cursor for everyone else immediately — a 15-second window only ever catches a genuine crash, never a normal close. Pins and replay share the same rate-limiting the data stream itself uses, so a stray script — or two people hitting Replay at the same instant — can't flood the room. `username`/`password` keep a link private, exactly as they would for the stream underneath it.</small>

---

## Sera Board

One chart is plenty for one metric. A real incident never is — error rate, latency, queue depth, deploy markers, related but rarely the same shape, and normally scattered across five browser tabs nobody can see side by side. Sera Board puts every chart on one infinite canvas instead: a sidebar lists them all, each sits in its own frame you drag anywhere, and you zoom from a full command-center view down to one chart filling the screen.

`serve_board()` is Sera Board joined to Sera Live — the same shared room, the same live cursors, now moving over a whole canvas instead of a single chart:

```python
import seraplot as sp

errors = sp.bar("Error rate", labels=["us-east", "eu-west", "ap-south"], values=[12, 4, 31])
latency = sp.line("p99 latency (ms)", x=list(range(20)), y=[120, 118, 125, 400, 410, 190, 130, 128, 126, 124, 122, 400, 405, 190, 128, 126, 124, 122, 120, 118])
queue = sp.scatter("Queue depth", x=list(range(30)), y=[3, 5, 4, 40, 38, 6, 5, 4, 3, 4] * 3)

sp.serve_board([errors, latency, queue], titles=["Errors", "Latency", "Queue"], session="2026-08-09", port=8787)
```

Send that link to the team. Pin a comment, drop a screenshot straight from the desktop, or draw on top of a chart, and it sticks to that chart's own frame — drag the frame anywhere and the annotation follows it. Group the charts that belong to the same story and drag the whole cluster together as one block; every move is shared live and saved, so the layout the team leaves the room in is the layout it opens to next time. Hover any frame and its own tooltips wake up right there, on the spot — the board never fights a chart for the mouse.

Everything — pins, drawings, images, layout — lives in a named **session** on disk, restored automatically the next time that session is served: close the server, reopen it a week later, the room is exactly where it was left. Jump the whole room to a different saved session with one click, live, no reload and no new link — everyone connected switches together. Need one frame streaming real live data instead of a frozen snapshot while the rest stay static? Point `live_urls` at it:

```python
errors.serve(port=8788)
sp.serve_board([errors, latency, queue], titles=["Errors", "Latency", "Queue"], live_urls=[f"http://127.0.0.1:8788", None, None], session="2026-08-09", port=8787)
```

That frame gets a small pulsing green dot and streams for real, continuously; the other two stay static snapshots.

Don't need it live — just a file to send around? `export_board_html()` writes the same board as one self-contained `.html`, no server required: same drag-and-zoom layout, each chart keeps its own hover and tooltips inside its frame.

```python
sp.export_board_html([errors, latency, queue], "war_room.html", titles=["Errors", "Latency", "Queue"])
```

<small>Presence (cursors) is board-wide rather than per-chart — each embedded chart is its own sandboxed `<iframe>`, so a purely static frame can't stream on its own; that's exactly what `live_urls` is for. A pin removed from the board disappears from the live view and the saved session, but not from `chart.bookmarks()` if that chart also keeps its own persistent bookmarks — the board's layer and a chart's permanent record stay independent on purpose. The live board and the static export both run on the same catalog and license as everything else on this page.</small>

---

## SeraStudio

`chart.export_gif(gif_path, spls_path)` renders a `.spls` recording (from `record()`) as an animated GIF — one frame per recorded update, only drawing a slot for indices that were actually pushed during the recording, so watching a handful of values update inside a much larger chart still produces a readable animation. Same catalog, same trial/license as everything else on this page:

```python
import seraplot as sp

chart = sp.bar("Sensors", labels=[str(i) for i in range(600)], values=[10.0] * 600)
chart.record("session.spls")
chart.push([3], [14.5])
chart.push([3], [16.0])
chart.stop_record()
chart.export_gif("session.gif", "session.spls")
```

When the recording carries its original chart-construction source (true for any `.spls` recorded since the source-tagged v4 format), every frame is replayed through that chart family's own real renderer — a themed heatmap shows its actual color grid, a `variant="pictogram"` bar shows real icons — instead of a generic substitute. Recordings from older `.spls` files, or families with no replay mapping, fall back automatically to a shape-based renderer keyed only on the recording's push shape (one value per index, an x/y pair, or an N-value vector): bar/heatmap recordings render as magnitude bars, scatter/line/bubble/area recordings render as a moving dot per index, and 4-value vector recordings render as real OHLC candlesticks (green/red for gain/loss; other vector widths fall back to grouped mini-bars) — so nothing fails outright even without a replay mapping. `color=(r, g, b)` customizes that fallback renderer's color (ignored whenever a family-correct replay renders the frame). `sort_by_value=True` re-ranks slots by current value every frame for a "bar chart race" look, compacting the frame down to just the racing rows and carrying each row's real label along as it re-ranks. `chart.gif_frame_count(spls_path)` peeks at a recording's frame count without rendering; `chart.gif_preview_frame(spls_path, frame_index, png_path)` renders a single frame as a PNG. Aliases: `to_gif`/`save_gif` for `export_gif`, `gif_length` for `gif_frame_count`.

For a real video file instead of a GIF, `chart.export_video(video_path, spls_path, format="mp4")` renders the same family-correct-when-available visualization through a real video codec (ffmpeg must be installed and on PATH) at a constant frame rate — bursts of rapid updates collapse into the latest state, quiet gaps repeat the last known frame, so playback speed always matches what really happened:

```python
chart.record("session.spls")
chart.push([3], [14.5])
chart.push([3], [16.0])
chart.stop_record()
chart.export_video("session.mp4", "session.spls", format="mp4", fps=30, sort_by_value=True)
```

`format` is one of `"mp4"` (H.264, broadly compatible), `"webm"` (VP9, smaller/web-native), `"mov"`, `"mkv"`, or `"avi"` — with a shorthand method for each (`export_mp4`, `export_webm`, `export_mov`, `export_mkv`, `export_avi`), plus `to_video`/`save_video` aliases for `export_video` itself.

Both `export_gif()` and `export_video()` (and `gif_preview_frame()`) also accept `smooth=True`: instead of snapping straight to each recorded state, it tweens the geometry of matching elements between two consecutive recorded states — correlated by each row's real identity, not its on-screen slot, so a racing row keeps its own tween even as `sort_by_value` reassigns slots around it. Values glide instead of jumping, and a racing row visibly slides past the others as it overtakes them, rather than teleporting to its new rank on the next frame. `smooth_speed` (default `1.0`) scales how many in-between steps a transition gets — `2.0` is twice as fast with fewer steps, `0.5` is twice as slow with more; `smooth_ease` (default `"linear"`) picks the pacing curve out of `"linear"`, `"ease_in"`, `"ease_out"`, `"ease_in_out"`:

```python
chart.export_video("session_smooth.mp4", "session.spls", format="mp4", fps=60, sort_by_value=True, smooth=True, smooth_speed=1.5, smooth_ease="ease_in_out")
```

`smooth` is opt-in and off by default — it costs extra render time per transition (each interpolated tick still rasterizes a full frame), so it is worth it for a polished, presentation-ready export but not needed for a quick preview. It has no effect when no replay source is available (a pre-v4 `.spls` file, or a family with no replay mapping): those still fall back to the shape-based renderer, unaffected.

For a known, complete dataset rather than a live stream, `reveal=True` replaces the real update timeline entirely: instead of replaying the recording's actual push events, it reveals the final touched state progressively in index order over `reveal_steps` frames, as if the chart were drawing itself — a curve or a set of bars appearing point by point rather than jumping between recorded snapshots. Pace it with the same `smooth_tick_ms` the export's `fps` already derives, so `reveal_steps` frames at a given `fps` take `reveal_steps / fps` seconds regardless of how the underlying `.spls` recording was actually timed:

```python
chart.export_video("reveal.mp4", "session.spls", format="mp4", fps=30, reveal=True, reveal_steps=150)
```

`breakpoint_at_ms` and `breakpoint_on_value` each hold the output on a frame for `breakpoint_pause_ms` the first time a condition is met — `breakpoint_at_ms` on elapsed output time, `breakpoint_on_value` (as `(index, threshold)` pairs) the first time a specific tracked index's decoded value reaches or exceeds a threshold. Every later frame shifts forward by the pause so the rest of the timeline stays intact; each breakpoint fires once. Both compose with `reveal` and with normal/`smooth` playback alike, since they operate on whichever frame timeline was already built:

```python
chart.export_video(
    "paused.mp4", "session.spls", format="mp4", fps=30,
    reveal=True, reveal_steps=150,
    breakpoint_at_ms=[2500], breakpoint_on_value=[(3, 90.0)], breakpoint_pause_ms=1200,
)
```

### Video showcases

Seven real exports, generated by the scripts shown under each — click "View code" to read the exact `.py` that produced it.

<div class="sp-video-grid">

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_60fps_bar_pictogram.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-pictogram" data-sp-code-title="serastudio_60fps_demo.py">View code</button>
</div>

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_smooth_battle_10s.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-battle" data-sp-code-title="serastudio_smooth_battle_demo.py">View code</button>
</div>

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_smooth_scatter_regression_12s.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-scatter" data-sp-code-title="serastudio_smooth_scatter_regression_demo.py">View code</button>
</div>

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_smooth_line_connected_scatter_12s.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-line-connected-scatter" data-sp-code-title="serastudio_smooth_line_connected_scatter_demo.py">View code</button>
</div>

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_smooth_line_multi_reveal.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-line-multi-reveal" data-sp-code-title="serastudio_smooth_line_multi_reveal_demo.py">View code</button>
</div>

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_smooth_line_stepped_12s.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-line-stepped" data-sp-code-title="serastudio_smooth_line_stepped_demo.py">View code</button>
</div>

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_smooth_line_cardiogram.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-line-cardiogram" data-sp-code-title="serastudio_smooth_line_cardiogram_demo.py">View code</button>
</div>

</div>

### Full catalog

Every registered chart family and variant, read live from the same registry that drives the [Showcase](../showcase.md) — search it, then click "View code" for a ready-to-run SeraStudio `record()` / `push()` / `export_video()` snippet built from that variant's own real construction parameters. Not every family supports `push()` yet (see the SeraStudio paragraphs above) — the generated snippet still shows the right shape to fill in.

<details class="sp-catalog-details">
<summary>Browse the full catalog</summary>
<div class="sp-catalog" id="sp-serastudio-catalog-en"></div>
</details>

---

## Method reference

Always matches the actual implementation. Use the tabs below to jump straight to a mini-module (streaming, anomaly detection, alerts, bookmarks, remote push, rate limiting, history/reports, health, licensing, Sera Secure, SeraReport, SeraStudio) instead of scanning the full list.

<div data-sp-registry-table="methods" data-file="seraplot/pulse-pricing.md" data-group-by="module"></div>

</div>

<div class="lang-fr">

Sera Pulse est la couche de streaming au-dessus de chaque chart natif : `.push()`, `.push_vector()`, `.serve()`, `.record()`, `.replay()`, `.on_anomaly()`. Le rendu lui-même (`sp.scatter()`, `sp.bar()`, `sp.heatmap()`, `sp.candlestick()`, et tout autre chart natif à chemin rapide) reste gratuit et open-source — Pulse ne verrouille que la couche de *mise à jour incrémentale* construite par-dessus.

### Ce qui est inclus

Tout ce qui suit est sous une seule et même porte — le même essai actif ou la même licence débloque tout, Solo comme Team (Team n'ajoute que des postes et du support prioritaire, pas de fonctionnalités en plus) :

- **Streaming** — `push()`, `push_vector()`, `serve()`, `record()`/`replay()`, `on_anomaly()`, `export_standalone()`.
- **Sera Firehose** — `sp.firehose(...)` + `firehose_push()`, un chart à tampon circulaire pour les flux très haute fréquence, rendu via WebGL2.
- **Sera Live** — curseurs en direct, commentaires épinglés, un « replay together » partagé sur tout chart `.serve()`, et un thème clair/sombre commutable en un clic.
- **Sera Board** — `serve_board()`/`export_board_html()`, un tableau blanc infini façon Figma : cadres de charts déplaçables/zoomables, épingles, dessins à main levée, images déposées, tout en direct et collaboratif et sauvegardé dans une session persistante qu'on change en un clic, ou exporté en fichier statique.
- **Sera Secure** — `SeraDFrame`, chiffrement AES-256-GCM à l'entrée, le texte en clair n'est jamais mis en cache ni journalisé.
- **SeraReport** — `export_pdf()`, `export_docx()`, `export_pdf_report()`, chacun avec un lien compagnon interactif optionnel.
- **SeraStudio** — `export_video()`, `export_gif()`, enregistrement/relecture de session pour des extraits de chart partageables.

Chaque méthode ci-dessus appelle la même vérification de licence — une clé s'active sur une seule machine, et chaque prix ci-dessous est fixé pour de bon : les nouvelles fonctionnalités payantes rejoignent ce même catalogue sans coût supplémentaire, elles ne passent jamais derrière un nouveau palier.

---

## Essai gratuit

La première fois qu'une méthode Pulse est appelée sur une machine, un essai de 90 jours (3 mois) démarre automatiquement — **sans carte, sans inscription, sans appel réseau**. Visible via :

```python
import seraplot as sp
sp.pulse_status()
# {'state': 'trial', 'days_left': '90'}
```

Tout (`push`, `push_vector`, `serve`, `record`, `replay`, `on_anomaly`) fonctionne exactement comme avec une licence payante pendant l'essai — aucune différence de fonctionnalité, seulement une limite de temps.

---

## Après l'essai

Une fois les 90 jours écoulés, les méthodes Pulse lèvent une `PermissionError` claire renvoyant vers l'activation plutôt que d'échouer silencieusement ou de se dégrader :

```python
sp.scatter(x=x, y=y).push(idx, x, y)
# PermissionError: seraplot pulse: free trial (90 days) has ended.
# Activate a license with sp.pulse_activate(key) to keep using
# push()/serve()/record()/replay()/on_anomaly().
```

Activez avec la clé reçue après achat :

```python
sp.pulse_activate("eyJjIjoi...ZTFhMTki.MEUCIQDx...")
sp.pulse_status()
# {'state': 'licensed', 'customer': '...', 'plan': 'pro', 'expires_at': '...'}
```

Une clé de licence est un jeton auto-suffisant signé cryptographiquement — l'activation fonctionne entièrement hors ligne, sans phone-home, sans télémétrie.

Les plans verrouillés à une machine (voir Solo ci-dessous) sont liés à un identifiant local plutôt que librement copiables d'un ordinateur à l'autre :

```python
sp.pulse_machine_id()
# 'DESKTOP-ABC123-alice-...'
```

Envoyez cet identifiant lors de l'achat d'une clé verrouillée à une machine — il est embarqué dans la licence signée et vérifié à l'activation.

---

## Formules

| Formule | Prix | Inclut | |
|---|---|---|---|
| **Essai** | Gratuit | Tout, 90 jours (3 mois), aucune carte requise. | [Commencer](#essai-gratuit) |
| **Solo** | 10,97 €/poste/mois ou 109,70 €/poste/an | `push`/`serve`/`record`/`replay`/`on_anomaly`, verrouillé à un identifiant de machine (techniquement vérifié, pas seulement une politique). | [Acheter mensuel](https://sera-payment.onrender.com/paypal/buy/solo/monthly) · [Acheter annuel](https://sera-payment.onrender.com/paypal/buy/solo/annual) |
| **Team** | 10,97 €/poste/mois ou 109,70 €/poste/an — même tarif que Solo | Solo, une fois par poste — une clé verrouillée machine par membre de l'équipe, plus support prioritaire. | choisissez postes et mois ci-dessous |

Payé via PayPal — cliquez un lien d'achat, payez sur la page de paiement hébergée par PayPal, et votre clé de licence est générée automatiquement dès que le paiement est confirmé : un webhook signé déclenche la génération, vérifié contre la signature de PayPal pour qu'il soit impossible de simuler un paiement. Préférence pour la crypto (USDC/Polygon, auto-hébergé) ou une vente manuelle ? Passez par le [canal de support](../about/support.md).

Ce prix est fixé pour de bon : chaque fonctionnalité payante que ce catalogue gagnera plus tard — Sera Firehose, Sera Secure, SeraReport, SeraStudio, ce qui sortira ensuite — rejoint le même prix Solo/Team ci-dessus sans coût supplémentaire. Il n'y a pas de palier supérieur caché derrière un mur de paiement.

Chaque clé s'active sur une seule machine — toujours. Team n'est pas une clé multi-postes partagée, c'est une clé équivalente à Solo par membre de l'équipe, chacune verrouillée à la machine de ce membre, facturées ensemble.

<div class="sp-team-picker" id="sp-team-picker-fr"></div>

<small>Saisissez n'importe quel nombre de postes et n'importe quel nombre de mois (1 à 36) — le lien facture le taux plein par poste et par mois multiplié par ce nombre de postes et de mois, sauf exactement 12 mois qui utilise le tarif annuel réduit indiqué ci-dessus (même tarif que vous tapiez `12` ici ou passiez par un forfait annuel ailleurs — jamais de prix caché ou différent). Même principe pour Solo : `.../paypal/buy/solo/<mois>?seats=<postes>`.</small>

<small>Renouveler une clé existante garde son identité client et son verrouillage machine, et repousse juste l'expiration — c'est en réalité un nouveau jeton signé sous le capot (une signature ne se modifie pas après coup), mais même client, même machine, même appel `sp.pulse_activate()` de votre côté. Prenez votre clé actuelle et placez-la dans `https://sera-payment.onrender.com/paypal/renew/solo/monthly?key=<votre clé>` (remplacez `solo`/`monthly` par votre vrai plan/période ; marche aussi pour `team`, une clé de poste à la fois).</small>

---

## Sera Secure

`SecureDFrame`/`SecureDFrameBuilder`/`SeraKey` sont les équivalents chiffrés en AES-256-GCM de `SeraDFrame` — les colonnes restent en texte chiffré en mémoire, déchiffrées seulement de façon transitoire pour une lecture ou un rendu de chart ponctuel. Ils font partie du même catalogue payant que les méthodes de streaming ci-dessus et partagent **le même essai/licence** — pas d'achat séparé, pas d'activation séparée :

```python
import seraplot as sp

key = sp.SeraKey.generate()
frame = sp.SecureDFrame({"x": x_values, "y": y_values}, key.to_bytes())
chart_data = frame.to_chart_data("x", "y", key.to_bytes(), max_points=2000)
```

### Comment c'est organisé

<svg viewBox="0 0 900 210" style="width:100%;max-width:820px;height:auto;font-family:inherit" role="img" aria-label="Flux de données SecureDFrame : le SeraDFrame en clair est chiffré dans un SecureDFrame, stocké en texte chiffré, puis déchiffré de façon transitoire uniquement pour une lecture de chart">
  <defs>
    <marker id="sp-sec-arrow-fr" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
      <path d="M0,0 L10,5 L0,10 z" fill="var(--icons)"/>
    </marker>
  </defs>
  <rect x="10" y="40" width="200" height="90" rx="10" fill="var(--theme-popup-bg)" stroke="var(--theme-popup-border, rgba(0,0,0,.2))"/>
  <text x="110" y="75" text-anchor="middle" font-size="14" font-weight="700" fill="var(--fg)">SeraDFrame</text>
  <text x="110" y="96" text-anchor="middle" font-size="11" fill="var(--icons)">colonnes en clair</text>
  <text x="110" y="112" text-anchor="middle" font-size="11" fill="var(--icons)">en mémoire</text>
  <line x1="215" y1="85" x2="335" y2="85" stroke="var(--icons)" stroke-width="1.5" marker-end="url(#sp-sec-arrow-fr)"/>
  <text x="275" y="72" text-anchor="middle" font-size="10.5" fill="var(--icons)">SeraKey +</text>
  <text x="275" y="105" text-anchor="middle" font-size="10.5" fill="var(--icons)">SecureDFrameBuilder</text>
  <rect x="340" y="30" width="220" height="110" rx="10" fill="var(--theme-popup-bg)" stroke="var(--sidebar-active)" stroke-width="1.5"/>
  <text x="450" y="60" text-anchor="middle" font-size="14" font-weight="700" fill="var(--fg)">SecureDFrame</text>
  <text x="450" y="82" text-anchor="middle" font-size="11" fill="var(--icons)">AES-256-GCM</text>
  <text x="450" y="98" text-anchor="middle" font-size="11" fill="var(--icons)">chiffré au repos,</text>
  <text x="450" y="114" text-anchor="middle" font-size="11" fill="var(--icons)">jamais en clair sur disque</text>
  <text x="450" y="132" text-anchor="middle" font-size="16" fill="var(--sidebar-active)">&#128274;</text>
  <line x1="565" y1="85" x2="685" y2="85" stroke="var(--icons)" stroke-width="1.5" marker-end="url(#sp-sec-arrow-fr)"/>
  <text x="625" y="72" text-anchor="middle" font-size="10.5" fill="var(--icons)">to_chart_data(key)</text>
  <text x="625" y="105" text-anchor="middle" font-size="10.5" fill="var(--icons)">déchiffrement transitoire</text>
  <rect x="690" y="40" width="200" height="90" rx="10" fill="var(--theme-popup-bg)" stroke="var(--theme-popup-border, rgba(0,0,0,.2))"/>
  <text x="790" y="75" text-anchor="middle" font-size="14" font-weight="700" fill="var(--fg)">Chart</text>
  <text x="790" y="96" text-anchor="middle" font-size="11" fill="var(--icons)">valeurs déchiffrées</text>
  <text x="790" y="112" text-anchor="middle" font-size="11" fill="var(--icons)">le temps du rendu seulement</text>
  <text x="450" y="185" text-anchor="middle" font-size="11" fill="var(--icons)">Même garantie de chiffrement au repos pour chaque famille de chart — rien en clair ne survit à la lecture qui en avait besoin.</text>
</svg>

### Pourquoi les équipes s'en servent

Un dump de process, un fichier core égaré, un débogueur attaché au mauvais moment — avec un `SeraDFrame` normal, c'est toute la colonne, en clair. Avec `SecureDFrame`, c'est du texte chiffré, un point c'est tout. La fenêtre en clair se limite exactement à une lecture : `to_chart_data()` déchiffre, le chart se rend, et il ne reste rien en mémoire qui puisse fuiter. Si vous affichez des dossiers patients, un historique de transactions, des données personnelles, ou tout ce que vous préféreriez ne pas avoir à expliquer à un responsable conformité après coup, c'est le changement d'une ligne (`SeraDFrame` → `SecureDFrame`) qui fait la différence entre « on a eu une fuite mémoire » et « on a eu un incident ».

Ça ne coûte rien de plus à essayer — c'est le même essai et la même licence que toutes les autres fonctionnalités Pulse de cette page, donc aucune décision d'achat séparée à prendre. [Démarrez l'essai gratuit](#essai-gratuit) et changez un nom de classe.

---

## SeraReport

`export_pdf()` convertit le SVG propre au chart directement en PDF vectoriel natif — sans navigateur headless, sans capture d'écran, juste une conversion SVG-vers-PDF directe. `export_docx()` fait pareil vers un document Word, le chart intégré en vraie image. Même catalogue, même essai/licence que le reste de cette page :

```python
import seraplot as sp

chart = sp.bar("Ventes", labels=["Q1", "Q2", "Q3"], values=[120, 150, 90])
chart.export_pdf("ventes.pdf")
chart.export_docx("ventes.docx")
```

Nécessite que le chart ait un vrai contenu SVG — les charts assez grands pour dépasser leur seuil de rendu canvas natif se rendent via `<canvas>` plutôt que `<svg>` (la même limite qu'ont déjà `export_svg()`/`export_png()`). `export_pdf_report(charts, path, titles=None)` combine plusieurs charts en un seul PDF multi-page, une page chacun.

Les deux méthodes prennent un paramètre `interactive`, vrai par défaut : en plus du PDF/DOCX, un fichier `.html` compagnon est écrit à côté — la vraie page interactive du chart — et un lien cliquable est intégré dans le document (une annotation de lien `/GoToR` dans le PDF, un paragraphe hyperlien dans le DOCX) qui l'ouvre. Passez `interactive=False` pour un simple fichier statique sans compagnon :

```python
chart.export_pdf("ventes.pdf", interactive=False)
sp.export_pdf_report(charts, "rapport_trimestriel.pdf", titles=titres, interactive=True)
```

### Exemple complet

`export_pdf()` est une conversion 1:1 directe — la page PDF vectorielle est exactement le SVG du chart, rien n'est re-rendu ni rastérisé entre les deux. `export_pdf_report()` fait pareil pour tout un ensemble de charts d'un coup, une vraie page par chart. Le PDF ci-dessous a été généré par le script juste en dessous — six familles de charts différentes (bar, line, scatter, pie, boxplot, heatmap), un seul rapport multi-page, ouvrez-le directement ici :

<div class="sp-video-card" style="max-width:720px">
<iframe src="../previews/serareport-sample.pdf" style="width:100%;height:600px;border:1px solid var(--theme-popup-border, rgba(0,0,0,.2));border-radius:8px"></iframe>
<p><a href="../previews/serareport-sample.pdf" target="_blank" rel="noopener">Ouvrir dans un nouvel onglet</a> si votre navigateur n'aperçoit pas les PDF en direct.</p>
</div>

```python
import seraplot as sp

bar = sp.bar("Ventes trimestrielles", labels=["Q1", "Q2", "Q3", "Q4"], values=[120, 150, 90, 175], color_hex=0x3B82F6).show_grid().despine()
line = sp.line("Utilisateurs actifs quotidiens", x_labels=["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"], values=[820, 932, 901, 934, 1290, 1330, 1320], color_hex=0x22C55E).show_grid().despine()
scatter = sp.scatter("Latence vs Charge", x=[10, 20, 30, 40, 50, 60, 70, 80], y=[12, 18, 22, 35, 40, 55, 61, 78], variant="regression").show_grid().despine()
pie = sp.pie("Sources de trafic", labels=["Organique", "Payant", "Référent", "Direct"], values=[45, 25, 15, 15])
boxplot = sp.boxplot("Temps de réponse par région", labels=["US"] * 6 + ["EU"] * 6 + ["APAC"] * 6, values=[120, 130, 125, 118, 140, 122, 200, 210, 195, 205, 220, 198, 90, 95, 88, 102, 97, 91])
heatmap = sp.heatmap("Activité hebdomadaire", labels=["Lun", "Mar", "Mer", "Jeu", "Ven"], col_labels=["8h", "12h", "16h", "20h"], values=[5, 9, 7, 3, 6, 12, 10, 4, 8, 15, 13, 7, 4, 8, 11, 5, 3, 7, 9, 2])

charts = [bar, line, scatter, pie, boxplot, heatmap]
titres = ["Ventes trimestrielles", "Utilisateurs actifs quotidiens", "Latence vs Charge", "Sources de trafic", "Temps de réponse par région", "Activité hebdomadaire"]
sp.export_pdf_report(charts, "rapport_trimestriel.pdf", titles=titres)
```

---

## Sera Firehose

`sp.firehose(...)` est un chart construit pour une seule chose : absorber un flux très rapide de valeurs — capteurs, prix, latences de requêtes — sans que l'onglet du navigateur s'étouffe dessus. C'est un tampon circulaire de taille fixe, côté serveur comme navigateur : vous ne suivez jamais de position d'écriture, vous ne faites jamais grossir un tableau indéfiniment. Une fois le tampon plein, chaque nouvelle valeur écrase silencieusement la plus ancienne.

```python
import seraplot as sp

chart = sp.firehose("CPU %", capacity=2_000, min_val=0.0, max_val=100.0)
chart.serve(port=8787)
for reading in flux_capteur():
    chart.firehose_push([reading])
```

`firehose_push()` est `push()` avec les indices calculés pour vous — même limitation de débit, même détection d'anomalies via `on_anomaly()`, même `record()`/`replay()`, même diffusion `.serve()` sur les mêmes trames WebSocket binaires que tout autre chart natif diffuse déjà. Rien de nouveau côté transport ; ce qui est nouveau c'est le client : au lieu d'un `<canvas>` redessiné point par point sur le CPU, Firehose se rend via WebGL2 — un envoi de buffer GPU par frame d'animation, un seul appel `drawArrays`, aucun travail JavaScript par point quel que soit le nombre de valeurs arrivées depuis la dernière frame, dans tous les navigateurs actuels sans rien à installer en plus.

Le tampon circulaire lui-même est un type Rust générique et réellement réutilisable (`RingBuffer<T>`), pas un bricolage ponctuel : capacité fixe, aucune réallocation une fois construit, écrasement du plus ancien à chaque ajout. En build release il tient plus de 200 millions d'ajouts par seconde en pure gestion de tampon — le vrai plafond pour 100k+ valeurs/seconde, c'est le coût d'appel Python et le réseau, jamais le tampon.

### Aperçu en direct

C'est le vrai chart, en train de vraiment se dessiner — pas une capture d'écran. Un petit générateur synthétique tourne entièrement dans votre navigateur, en appelant exactement la même fonction `sp_apply_<id>` dans laquelle `firehose_push()` streame :

<div class="sp-video-card" style="max-width:720px">
<iframe src="../previews/firehose-demo.html" style="width:100%;height:340px;border:1px solid var(--theme-popup-border, rgba(0,0,0,.2));border-radius:8px;background:#fff"></iframe>
<p><a href="../previews/firehose-demo.html" target="_blank" rel="noopener">Ouvrir en autonome</a> pour le voir en pleine taille.</p>
</div>

```python
import random
import seraplot as sp

chart = sp.firehose("Flux capteur en direct", capacity=300, min_val=0.0, max_val=100.0, height=260)
chart.record("session.spls")
valeur = 50.0
for _ in range(1_200):
    valeur = max(0.0, min(100.0, valeur + random.uniform(-2.5, 2.5)))
    chart.firehose_push([valeur])
chart.stop_record()
chart.export_standalone("firehose-demo.html", "session.spls")
```

L'aperçu ci-dessus utilise un petit générateur en boucle plutôt qu'un enregistrement fini pour ne jamais tomber à court de données sur cette page, mais il appelle exactement la même fonction `sp_apply_<id>` dans les deux cas — `export_standalone()` est la vraie méthode, officielle, pour livrer une démo autonome comme celle-ci.

---

## Sera Live

Un partage d'écran, c'est la fenêtre d'une seule personne, à la position de scroll d'une seule personne, décrite à voix haute à tous les autres qui plissent les yeux devant un flux qui rame. Sera Live transforme n'importe quel chart `.serve()` en vraie salle partagée : tout le monde qui ouvre le lien est *dans* le chart ensemble, en pleine résolution, sur son propre écran.

```python
import seraplot as sp

chart = sp.bar("Taux d'erreur par région", labels=["us-east", "eu-west", "ap-south"], values=[12, 4, 31])
chart.record("incident.spls")
chart.serve(port=8787)
```

Envoyez ce lien et tous les curseurs de la salle bougent en direct, en temps réel, nommés et colorés par personne. Épinglez une note directement sur la barre qui explose et elle apparaît pour tout le monde instantanément — pas un commentaire trois scrolls plus bas dans un fil, une étiquette posée exactement là où est le problème. Entourez-la, soulignez-la, dessinez une flèche vers ce que tout le monde doit regarder, à main levée, dans la couleur qui fait passer le message. Cliquez sur « Replay together » et chaque onglet connecté traverse l'incident enregistré au même rythme, ensemble, au lieu que cinq personnes reconstituent la même chronologie de cinq façons différentes. Un clic bascule toute la salle dans un thème sombre pensé pour un écran mural ou une war room à 2h du matin, mémorisé par navigateur. Rejoignez avec quatre minutes de retard et vous n'avez rien raté — chaque épingle et chaque trait posés avant votre connexion vous sont rejoués dès que vous arrivez.

Rien de tout ça n'est un système à part greffé par-dessus : curseurs, épingles, dessins et rejeu voyagent tous sur le même WebSocket que `.serve()` ouvre déjà. Les épingles sont de vraies annotations `bookmark()`, pas une couche jetable — listez-les ensuite avec `chart.bookmarks()` comme d'habitude. Besoin de l'ancien comportement mono-spectateur, où rien concernant vous ou votre curseur ne quitte jamais votre machine ? `collab=False` désactive tout ça :

```python
chart.serve(port=8787, collab=False)
```

<small>Fermer un onglet retire ce curseur pour tout le monde immédiatement — une fenêtre de 15 secondes ne sert qu'à rattraper un vrai crash, jamais une fermeture normale. Les épingles et le rejeu partagent la même limitation de débit que le flux de données lui-même, donc un script errant — ou deux personnes qui cliquent Replay au même instant — ne peuvent pas inonder la salle. `username`/`password` gardent un lien privé, exactement comme pour le flux sous-jacent.</small>

---

## Sera Board

Un chart à la fois, ça suffit pour une seule métrique. Un vrai incident, jamais — taux d'erreur, latence, profondeur de file, marqueurs de déploiement, liés mais rarement de la même forme, et normalement éparpillés sur cinq onglets que personne ne peut voir en même temps. Sera Board pose tous les charts sur une seule toile infinie à la place : une barre latérale les liste tous, chacun dans son propre cadre qu'on déplace où on veut, et on zoome d'une vue commandement complète jusqu'à un seul chart plein écran.

`serve_board()` réunit Sera Board et Sera Live — la même salle partagée, les mêmes curseurs en direct, désormais sur toute une toile plutôt qu'un seul chart :

```python
import seraplot as sp

erreurs = sp.bar("Taux d'erreur", labels=["us-east", "eu-west", "ap-south"], values=[12, 4, 31])
latence = sp.line("Latence p99 (ms)", x=list(range(20)), y=[120, 118, 125, 400, 410, 190, 130, 128, 126, 124, 122, 400, 405, 190, 128, 126, 124, 122, 120, 118])
file_attente = sp.scatter("Profondeur de file", x=list(range(30)), y=[3, 5, 4, 40, 38, 6, 5, 4, 3, 4] * 3)

sp.serve_board([erreurs, latence, file_attente], titles=["Erreurs", "Latence", "File"], session="2026-08-09", port=8787)
```

Envoyez ce lien à l'équipe. Épinglez un commentaire, déposez une capture d'écran directement depuis le bureau, ou dessinez sur un chart — ça se colle au cadre de ce chart, déplacez le cadre et l'annotation suit. Regroupez les charts qui racontent la même histoire et déplacez tout le bloc ensemble ; chaque mouvement est partagé en direct et sauvegardé, donc la disposition dans laquelle l'équipe laisse la salle est celle qu'elle retrouve la fois suivante. Survolez n'importe quel cadre et ses propres infobulles s'activent directement, sur place — la toile ne se dispute jamais la souris avec un chart.

Tout — épingles, dessins, images, disposition — vit dans une **session** nommée sur disque, restaurée automatiquement la prochaine fois que cette session est servie : fermez le serveur, rouvrez-le une semaine plus tard, la salle est exactement où elle était. Basculez toute la salle vers une autre session sauvegardée en un clic, en direct, sans recharger et sans nouveau lien — tout le monde connecté bascule ensemble. Besoin qu'un cadre précis streame de vraies données en direct plutôt qu'un instantané figé pendant que les autres restent statiques ? Pointez `live_urls` dessus :

```python
erreurs.serve(port=8788)
sp.serve_board([erreurs, latence, file_attente], titles=["Erreurs", "Latence", "File"], live_urls=[f"http://127.0.0.1:8788", None, None], session="2026-08-09", port=8787)
```

Ce cadre reçoit un petit point vert qui pulse et streame pour de vrai, en continu ; les deux autres restent des instantanés statiques.

Pas besoin que ce soit en direct — juste un fichier à envoyer ? `export_board_html()` écrit la même toile en un seul `.html` autonome, sans serveur : même disposition glisser-zoomer, chaque chart garde son propre survol et ses infobulles dans son cadre.

```python
sp.export_board_html([erreurs, latence, file_attente], "war_room.html", titles=["Erreurs", "Latence", "File"])
```

<small>La présence (curseurs) est propre à toute la toile plutôt qu'à chaque chart — chaque chart intégré est son propre `<iframe>` isolé, donc un cadre purement statique ne peut pas streamer par lui-même ; c'est exactement à ça que sert `live_urls`. Une épingle supprimée de la toile disparaît de la vue en direct et de la session sauvegardée, mais pas de `chart.bookmarks()` si ce chart garde aussi ses propres repères persistants — la couche de la toile et l'enregistrement permanent d'un chart restent volontairement indépendants. La toile en direct et l'export statique tournent tous deux sur le même catalogue et la même licence que le reste de cette page.</small>

---

## SeraStudio

`chart.export_gif(gif_path, spls_path)` rend un enregistrement `.spls` (issu de `record()`) en GIF animé — une frame par mise à jour enregistrée, en ne dessinant un emplacement que pour les indices réellement poussés pendant l'enregistrement, pour qu'observer quelques valeurs évoluer au sein d'un chart bien plus grand produise quand même une animation lisible. Même catalogue, même essai/licence que le reste de cette page :

```python
import seraplot as sp

chart = sp.bar("Capteurs", labels=[str(i) for i in range(600)], values=[10.0] * 600)
chart.record("session.spls")
chart.push([3], [14.5])
chart.push([3], [16.0])
chart.stop_record()
chart.export_gif("session.gif", "session.spls")
```

Quand l'enregistrement porte sa source de construction d'origine (vrai pour tout `.spls` enregistré depuis le format v4 avec source), chaque frame est rejouée via le vrai rendu de cette famille de chart — une heatmap themée montre sa vraie grille de couleurs, une barre `variant="pictogram"` montre de vraies icônes — plutôt qu'un substitut générique. Les enregistrements issus d'anciens fichiers `.spls`, ou des familles sans correspondance de rejeu, retombent automatiquement sur un rendu basé uniquement sur la forme des données poussées (une valeur par indice, une paire x/y, ou un vecteur à N valeurs) : les enregistrements bar/heatmap se rendent en barres de magnitude, les enregistrements scatter/ligne/bulle/aire se rendent en point mobile par indice, et les enregistrements vecteur à 4 valeurs se rendent en vrais chandeliers OHLC (vert/rouge pour hausse/baisse ; les autres largeurs de vecteur retombent sur des mini-barres groupées) — pour ne jamais échouer completement, même sans correspondance de rejeu. `color=(r, g, b)` personnalise la couleur de ce rendu de secours (ignorée dès qu'un rejeu fidèle à la famille rend la frame). `sort_by_value=True` reclasse les emplacements par valeur courante à chaque frame pour un effet « bar chart race », en compactant la frame sur les seules lignes en course et en emportant le vrai libellé de chaque ligne avec elle quand elle change de rang. `chart.gif_frame_count(spls_path)` consulte le nombre de frames d'un enregistrement sans rien rendre ; `chart.gif_preview_frame(spls_path, frame_index, png_path)` rend une seule frame en PNG. Alias : `to_gif`/`save_gif` pour `export_gif`, `gif_length` pour `gif_frame_count`.

Pour un vrai fichier vidéo plutôt qu'un GIF, `chart.export_video(video_path, spls_path, format="mp4")` rend la même visualisation fidèle à la famille quand elle est disponible, via un vrai codec vidéo (ffmpeg doit être installé et accessible sur le PATH) à un frame rate constant — les rafales de mises à jour rapides se compressent sur le dernier état, les silences répètent la dernière frame connue, donc la vitesse de lecture correspond toujours à ce qui s'est réellement passé :

```python
chart.record("session.spls")
chart.push([3], [14.5])
chart.push([3], [16.0])
chart.stop_record()
chart.export_video("session.mp4", "session.spls", format="mp4", fps=30, sort_by_value=True)
```

`format` est un parmi `"mp4"` (H.264, largement compatible), `"webm"` (VP9, plus léger/natif web), `"mov"`, `"mkv"`, ou `"avi"` — avec un raccourci par format (`export_mp4`, `export_webm`, `export_mov`, `export_mkv`, `export_avi`), plus les alias `to_video`/`save_video` pour `export_video` lui-même.

`export_gif()` et `export_video()` (ainsi que `gif_preview_frame()`) acceptent aussi `smooth=True` : au lieu de sauter directement à chaque état enregistré, la géométrie des éléments correspondants est animée entre deux états enregistrés consécutifs — mis en correspondance par l'identité réelle de chaque ligne, pas son emplacement à l'écran, pour qu'une ligne en course garde sa propre animation même quand `sort_by_value` réassigne les emplacements autour d'elle. Les valeurs évoluent en douceur, et une ligne en course glisse visiblement devant les autres en les dépassant, plutôt que de se téléporter à son nouveau rang à la frame suivante. `smooth_speed` (défaut `1.0`) ajuste le nombre d'étapes intermédiaires d'une transition — `2.0` est deux fois plus rapide avec moins d'étapes, `0.5` est deux fois plus lent avec plus d'étapes ; `smooth_ease` (défaut `"linear"`) choisit la courbe de rythme parmi `"linear"`, `"ease_in"`, `"ease_out"`, `"ease_in_out"` :

```python
chart.export_video("session_smooth.mp4", "session.spls", format="mp4", fps=60, sort_by_value=True, smooth=True, smooth_speed=1.5, smooth_ease="ease_in_out")
```

`smooth` est optionnel et désactivé par défaut — il coûte un temps de rendu supplémentaire par transition (chaque étape interpolée rastérise quand même une frame complète), donc utile pour un export soigné destiné à une présentation, mais pas nécessaire pour un aperçu rapide. Il n'a aucun effet sans source de rejeu disponible (fichier `.spls` pré-v4, ou famille sans correspondance de rejeu) : ceux-là retombent toujours sur le rendu basé sur la forme des données, sans changement.

Pour un jeu de données connu et complet plutôt qu'un flux temps réel, `reveal=True` remplace entièrement la vraie ligne de temps de mises à jour : au lieu de rejouer les vrais événements de push de l'enregistrement, il révèle l'état final touché progressivement dans l'ordre des indices sur `reveal_steps` frames, comme si le chart se dessinait lui-même — une courbe ou un jeu de barres qui apparaît point par point plutôt que de sauter entre états enregistrés. Le rythme utilise le même `smooth_tick_ms` que celui déjà dérivé du `fps` de l'export, donc `reveal_steps` frames à un `fps` donné prennent `reveal_steps / fps` secondes, peu importe le vrai timing de l'enregistrement `.spls` sous-jacent :

```python
chart.export_video("reveal.mp4", "session.spls", format="mp4", fps=30, reveal=True, reveal_steps=150)
```

`breakpoint_at_ms` et `breakpoint_on_value` maintiennent chacun la sortie sur une frame pendant `breakpoint_pause_ms` la première fois qu'une condition est atteinte — `breakpoint_at_ms` sur le temps écoulé en sortie, `breakpoint_on_value` (sous forme de paires `(indice, seuil)`) la première fois que la valeur décodée d'un indice suivi spécifique atteint ou dépasse un seuil. Chaque frame suivante est décalée en avant de la durée de la pause pour garder le reste de la ligne de temps intact ; chaque breakpoint se déclenche une seule fois. Les deux se combinent aussi bien avec `reveal` qu'avec la lecture normale/`smooth`, puisqu'ils opèrent sur la ligne de temps de frames déjà construite, quelle qu'elle soit :

```python
chart.export_video(
    "paused.mp4", "session.spls", format="mp4", fps=30,
    reveal=True, reveal_steps=150,
    breakpoint_at_ms=[2500], breakpoint_on_value=[(3, 90.0)], breakpoint_pause_ms=1200,
)
```

### Vitrines vidéo

Sept exports réels, générés par les scripts affichés sous chacun — cliquez sur « Voir le code » pour lire le `.py` exact qui l'a produit.

<div class="sp-video-grid">

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_60fps_bar_pictogram.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-pictogram" data-sp-code-title="serastudio_60fps_demo.py">Voir le code</button>
</div>

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_smooth_battle_10s.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-battle" data-sp-code-title="serastudio_smooth_battle_demo.py">Voir le code</button>
</div>

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_smooth_scatter_regression_12s.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-scatter" data-sp-code-title="serastudio_smooth_scatter_regression_demo.py">Voir le code</button>
</div>

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_smooth_line_connected_scatter_12s.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-line-connected-scatter" data-sp-code-title="serastudio_smooth_line_connected_scatter_demo.py">Voir le code</button>
</div>

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_smooth_line_multi_reveal.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-line-multi-reveal" data-sp-code-title="serastudio_smooth_line_multi_reveal_demo.py">Voir le code</button>
</div>

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_smooth_line_stepped_12s.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-line-stepped" data-sp-code-title="serastudio_smooth_line_stepped_demo.py">Voir le code</button>
</div>

<div class="sp-video-card">
<video controls autoplay loop muted playsinline preload="auto" src="videos/serastudio_smooth_line_cardiogram.mp4"></video>
<button class="sp-code-btn" data-sp-code-target="sp-code-line-cardiogram" data-sp-code-title="serastudio_smooth_line_cardiogram_demo.py">Voir le code</button>
</div>

</div>

### Catalogue complet

Toutes les familles et variants de charts enregistrés, lus en direct depuis le même registre qui alimente la [Vitrine](../showcase.md) — recherchez, puis cliquez sur « Voir le code » pour obtenir un extrait SeraStudio `record()` / `push()` / `export_video()` prêt à l'emploi, construit à partir des vrais paramètres de construction de ce variant. Toutes les familles ne supportent pas encore `push()` (voir les paragraphes SeraStudio ci-dessus) — l'extrait généré montre quand même la bonne forme à compléter.

<details class="sp-catalog-details">
<summary>Parcourir le catalogue complet</summary>
<div class="sp-catalog" id="sp-serastudio-catalog-fr"></div>
</details>

---

## Référence des méthodes

Toujours synchronisé avec l'implémentation réelle. Utilisez les onglets ci-dessous pour aller directement à un mini-module (streaming, détection d'anomalies, alertes, repères, push distant, limitation de débit, historique/rapports, santé, licence, Sera Secure, SeraReport, SeraStudio) plutôt que de parcourir la liste complète.

<div data-sp-registry-table="methods" data-file="seraplot/pulse-pricing.md" data-group-by="module"></div>

<script type="text/plain" id="sp-code-pictogram">
import math
import os
import time

import seraplot as sp

OUT_DIR = os.path.dirname(os.path.abspath(__file__))
FPS = 60

BAR_RACERS = [21, 122, 263, 404, 515, 46]
BAR_RATES = [2.1, 1.4, 2.8, 1.7, 2.3, 1.9]
BAR_STEPS = 260

HEATMAP_SIZE = 50
HEATMAP_CELLS = [200, 850, 1400, 1900, 90, 2340]
HEATMAP_STEPS = 220


def build_bar_pictogram_race():
    n = 600
    labels = [f"Item {i}" for i in range(n)]
    values = [float(i % 100) for i in range(n)]
    chart = (
        sp.bar("SeraStudio 60fps demo - bar chart race (pictogram variant)", labels=labels, values=values, variant="pictogram")
        .show_grid()
        .no_legend()
        .despine()
    )

    spls_path = os.path.join(OUT_DIR, "_tmp_serastudio_bar.spls")
    if os.path.exists(spls_path):
        os.remove(spls_path)

    chart.record(spls_path)
    current = {idx: 0.0 for idx in BAR_RACERS}
    for _ in range(BAR_STEPS):
        for idx, rate in zip(BAR_RACERS, BAR_RATES):
            current[idx] = min(99.0, current[idx] + rate * 0.35)
            chart.push([idx], [current[idx]])
        time.sleep(1 / FPS)
    chart.stop_record()

    out_path = os.path.join(OUT_DIR, "serastudio_60fps_bar_pictogram.mp4")
    chart.export_video(
        out_path,
        spls_path,
        format="mp4",
        fps=FPS,
        width=1280,
        height=720,
        max_frames=BAR_STEPS * len(BAR_RACERS) + 10,
        color=(56, 189, 248),
        sort_by_value=True,
    )
    os.remove(spls_path)
    return out_path


def build_correlogram_style_heatmap():
    labels = [f"V{i}" for i in range(HEATMAP_SIZE)]
    values = [float((i * 37 + 11) % 100) / 100.0 for i in range(HEATMAP_SIZE * HEATMAP_SIZE)]
    chart = (
        sp.build_heatmap(
            "SeraStudio 60fps demo - correlogram-style heatmap",
            labels=labels,
            col_labels=labels,
            values=values,
            color_low=0xF5F3FF,
            color_mid=0xA78BFA,
            color_high=0x4C1D95,
        )
        .no_axes()
        .no_legend()
    )

    spls_path = os.path.join(OUT_DIR, "_tmp_serastudio_heatmap.spls")
    if os.path.exists(spls_path):
        os.remove(spls_path)

    chart.record(spls_path)
    for step in range(HEATMAP_STEPS):
        t = step / HEATMAP_STEPS
        for phase, cell in enumerate(HEATMAP_CELLS):
            v = 0.5 + 0.5 * math.sin(t * math.pi * 2 + phase * 1.1)
            chart.push([cell], [v])
        time.sleep(1 / FPS)
    chart.stop_record()

    out_path = os.path.join(OUT_DIR, "serastudio_60fps_correlogram_heatmap.mp4")
    chart.export_video(
        out_path,
        spls_path,
        format="mp4",
        fps=FPS,
        width=900,
        height=900,
        max_frames=HEATMAP_STEPS * len(HEATMAP_CELLS) + 10,
        color=(167, 139, 250),
    )
    os.remove(spls_path)
    return out_path


def main():
    bar_path = build_bar_pictogram_race()
    print("wrote", bar_path)
    heatmap_path = build_correlogram_style_heatmap()
    print("wrote", heatmap_path)


if __name__ == "__main__":
    main()
</script>

<script type="text/plain" id="sp-code-battle">
import math
import os
import time

import seraplot as sp

OUT_DIR = os.path.dirname(os.path.abspath(__file__))
FPS = 60
N_BARS = 600
DURATION_S = 10.0
STEP_HZ = 12
STEP_PAUSE_S = 1.0 / STEP_HZ
STEPS = int(DURATION_S * STEP_HZ)

RACERS = [51, 152, 253, 354, 455, 556]
LABELS = ["Nord", "Sud", "Est", "Ouest", "Centre", "Alpha"]

WAVES = [
    (20.0, 14.0, 0.15, 0.0),
    (20.0, 13.0, 0.11, 1.0),
    (20.0, 15.0, 0.19, 2.3),
    (20.0, 12.0, 0.07, 3.1),
    (20.0, 14.0, 0.23, 4.2),
    (20.0, 13.0, 0.09, 5.5),
]


def value_at(wave, t):
    base, amp, freq, phase = wave
    return base + amp * math.sin(2 * math.pi * freq * t + phase)


def build_chart():
    labels = [f"Item {i}" for i in range(N_BARS)]
    values = [float(i % 30) for i in range(N_BARS)]
    for idx, name in zip(RACERS, LABELS):
        labels[idx] = name
    return (
        sp.bar("SeraStudio smooth demo - 6 bars battling for 10s", labels=labels, values=values)
        .show_grid()
        .no_legend()
        .despine()
    )


def record_session(spls_path):
    if os.path.exists(spls_path):
        os.remove(spls_path)
    chart = build_chart()
    chart.record(spls_path)
    for step in range(STEPS):
        t = step * STEP_PAUSE_S
        for idx, wave in zip(RACERS, WAVES):
            chart.push([idx], [value_at(wave, t)])
        time.sleep(STEP_PAUSE_S)
    chart.stop_record()
    return chart


def main():
    spls_path = os.path.join(OUT_DIR, "_tmp_serastudio_battle.spls")
    chart = record_session(spls_path)
    n_frames = chart.gif_frame_count(spls_path)
    print("recorded", n_frames, "raw pushes over", DURATION_S, "s")

    out_path = os.path.join(OUT_DIR, "serastudio_smooth_battle_10s.mp4")
    chart.export_video(
        out_path, spls_path, format="mp4", fps=FPS, width=1280, height=720,
        max_frames=n_frames + 20, color=(56, 189, 248), sort_by_value=True, smooth=True,
    )
    print("wrote", out_path)

    os.remove(spls_path)


if __name__ == "__main__":
    main()
</script>

<script type="text/plain" id="sp-code-scatter">
import math
import os
import random
import time

import seraplot as sp

OUT_DIR = os.path.dirname(os.path.abspath(__file__))
FPS = 60
DURATION_S = 12.0
STEP_HZ = 5
STEPS = int(DURATION_S * STEP_HZ)
STEP_PAUSE_S = 1.0 / STEP_HZ

N = 3200

random.seed(11)
BASE_X = [random.uniform(0.0, 100.0) for _ in range(N)]
NOISE = [random.uniform(-8.0, 8.0) for _ in range(N)]
ALL_IDX = list(range(N))

SLOPE_BASE = 0.6
SLOPE_AMPLITUDE = 0.5
SLOPE_FREQ = 0.1
INTERCEPT = 5.0


def slope_at(t):
    return SLOPE_BASE + SLOPE_AMPLITUDE * math.sin(2 * math.pi * SLOPE_FREQ * t)


def y_at(t):
    s = slope_at(t)
    return [s * x + INTERCEPT + n for x, n in zip(BASE_X, NOISE)]


def build_chart():
    return (
        sp.scatter("SeraStudio smooth demo - whole cloud evolving (regression)", x=BASE_X, y=y_at(0.0), variant="regression")
        .show_grid()
        .no_legend()
        .despine()
    )


def record_session(spls_path):
    if os.path.exists(spls_path):
        os.remove(spls_path)
    chart = build_chart()
    chart.record(spls_path)
    for step in range(STEPS):
        t = step * STEP_PAUSE_S
        chart.push(ALL_IDX, BASE_X, y_at(t))
        time.sleep(STEP_PAUSE_S)
    chart.stop_record()
    return chart


def main():
    spls_path = os.path.join(OUT_DIR, "_tmp_serastudio_scatter_regression.spls")
    chart = record_session(spls_path)
    n_frames = chart.gif_frame_count(spls_path)
    print("recorded", n_frames, "raw pushes over", DURATION_S, "s")

    out_path = os.path.join(OUT_DIR, "serastudio_smooth_scatter_regression_12s.mp4")
    chart.export_video(
        out_path, spls_path, format="mp4", fps=FPS, width=1000, height=750,
        max_frames=n_frames + 20, smooth=True,
    )
    print("wrote", out_path)

    os.remove(spls_path)


if __name__ == "__main__":
    main()
</script>

<script type="text/plain" id="sp-code-line-connected-scatter">
import math
import os
import random
import time

import seraplot as sp

OUT_DIR = os.path.dirname(os.path.abspath(__file__))
FPS = 60
DURATION_S = 12.0
STEP_HZ = 5
STEPS = int(DURATION_S * STEP_HZ)
STEP_PAUSE_S = 1.0 / STEP_HZ

YEARS_SPAN = 10
N = YEARS_SPAN * 366
START_YEAR = 2015.0

random.seed(23)
YEARS = [START_YEAR + i / 366.0 for i in range(N)]
NOISE = [random.uniform(-3.0, 3.0) for _ in range(N)]
ALL_IDX = list(range(N))

BASE = 50.0


def trend_slope_at(t):
    return 0.5 + 1.5 * math.sin(2 * math.pi * 0.08 * t)


def season_amplitude_at(t):
    return 15.0 + 8.0 * math.sin(2 * math.pi * 0.05 * t + 1.0)


def y_at(t):
    slope = trend_slope_at(t)
    amp = season_amplitude_at(t)
    return [
        BASE + slope * (year - START_YEAR) + amp * math.sin(2 * math.pi * year) + noise
        for year, noise in zip(YEARS, NOISE)
    ]


def build_chart():
    return (
        sp.line(
            "SeraStudio smooth demo - connected scatter over 10 years",
            x=YEARS,
            y=y_at(0.0),
            variant="connected_scatter",
            show_points=True,
        )
        .show_grid()
        .no_legend()
        .despine()
    )


def record_session(spls_path):
    if os.path.exists(spls_path):
        os.remove(spls_path)
    chart = build_chart()
    chart.record(spls_path)
    for step in range(STEPS):
        t = step * STEP_PAUSE_S
        chart.push(ALL_IDX, YEARS, y_at(t))
        time.sleep(STEP_PAUSE_S)
    chart.stop_record()
    return chart


def main():
    spls_path = os.path.join(OUT_DIR, "_tmp_serastudio_line_connected_scatter.spls")
    chart = record_session(spls_path)
    n_frames = chart.gif_frame_count(spls_path)
    print("recorded", n_frames, "raw pushes over", DURATION_S, "s")

    out_path = os.path.join(OUT_DIR, "serastudio_smooth_line_connected_scatter_12s.mp4")
    chart.export_video(
        out_path, spls_path, format="mp4", fps=FPS, width=1100, height=600,
        max_frames=n_frames + 20, smooth=True,
    )
    print("wrote", out_path)

    os.remove(spls_path)


if __name__ == "__main__":
    main()
</script>

<script type="text/plain" id="sp-code-line-multi-reveal">
import math
import os
import random

import seraplot as sp

OUT_DIR = os.path.dirname(os.path.abspath(__file__))
FPS = 30

YEARS_SPAN = 30
START_YEAR = 2000.0
YEARS = [START_YEAR + i for i in range(YEARS_SPAN)]
ALL_IDX = list(range(YEARS_SPAN))

random.seed(42)


def series_nord():
    return [20.0 + i * 0.9 + random.uniform(-2.0, 2.0) for i in range(YEARS_SPAN)]


def series_sud():
    return [35.0 - i * 0.3 + 6.0 * math.sin(i * 0.35) + random.uniform(-1.5, 1.5) for i in range(YEARS_SPAN)]


def series_centre():
    return [12.0 + i * 0.55 + 4.0 * math.sin(i * 0.2 + 1.0) + random.uniform(-1.5, 1.5) for i in range(YEARS_SPAN)]


def main():
    nord = series_nord()
    sud = series_sud()
    centre = series_centre()

    chart = (
        sp.line(
            "SeraStudio smooth demo - multi-series reveal with a breakpoint pause",
            x=YEARS,
            series=[nord, sud, centre],
            series_names=["Nord", "Sud", "Centre"],
            variant="multi",
        )
        .show_grid()
        .despine()
        .show_legend()
    )

    spls_path = os.path.join(OUT_DIR, "_tmp_serastudio_line_multi.spls")
    if os.path.exists(spls_path):
        os.remove(spls_path)
    chart.record(spls_path)
    chart.push_vector(ALL_IDX, [nord, sud, centre])
    chart.stop_record()
    n_frames = chart.gif_frame_count(spls_path)
    print("recorded", n_frames, "push(es) covering", YEARS_SPAN, "years x 3 series")

    out_path = os.path.join(OUT_DIR, "serastudio_smooth_line_multi_reveal.mp4")
    chart.export_video(
        out_path, spls_path, format="mp4", fps=FPS, width=1100, height=650,
        max_frames=n_frames + 20, reveal=True, reveal_steps=150,
        breakpoint_at_ms=[2500], breakpoint_pause_ms=1200,
    )
    print("wrote", out_path)

    os.remove(spls_path)


if __name__ == "__main__":
    main()
</script>

<script type="text/plain" id="sp-code-line-stepped">
import math
import os
import time

import seraplot as sp

OUT_DIR = os.path.dirname(os.path.abspath(__file__))
FPS = 60
DURATION_S = 12.0
STEP_HZ = 5
STEPS = int(DURATION_S * STEP_HZ)
STEP_PAUSE_S = 1.0 / STEP_HZ

N = 3200
X = [i * 0.05 for i in range(N)]

LEVELS = [18.0, 21.0, 19.0, 23.0, 17.0, 22.0, 20.0]
LEVEL_SPAN = 20.0


def y_at(t):
    phase_shift = t * 3.0
    out = []
    for x in X:
        level_idx = int((x + phase_shift) / LEVEL_SPAN) % len(LEVELS)
        out.append(LEVELS[level_idx])
    return out


def build_chart():
    return (
        sp.line("SeraStudio smooth demo - stepped thermostat target", x=X, y=y_at(0.0), variant="stepped", show_points=False)
        .show_grid()
        .no_legend()
        .despine()
    )


def record_session(spls_path):
    if os.path.exists(spls_path):
        os.remove(spls_path)
    chart = build_chart()
    chart.record(spls_path)
    for step in range(STEPS):
        t = step * STEP_PAUSE_S
        chart.push(list(range(N)), X, y_at(t))
        time.sleep(STEP_PAUSE_S)
    chart.stop_record()
    return chart


def main():
    spls_path = os.path.join(OUT_DIR, "_tmp_serastudio_line_stepped.spls")
    chart = record_session(spls_path)
    n_frames = chart.gif_frame_count(spls_path)
    print("recorded", n_frames, "raw pushes over", DURATION_S, "s")

    out_path = os.path.join(OUT_DIR, "serastudio_smooth_line_stepped_12s.mp4")
    chart.export_video(
        out_path, spls_path, format="mp4", fps=FPS, width=1100, height=600,
        max_frames=n_frames + 20, smooth=True,
    )
    print("wrote", out_path)

    os.remove(spls_path)


if __name__ == "__main__":
    main()
</script>

<script type="text/plain" id="sp-code-line-cardiogram">
import math
import os

import seraplot as sp

OUT_DIR = os.path.dirname(os.path.abspath(__file__))
FPS = 30

N = 4000
PERIOD = 4.0
BASELINE = 50.0
NORMAL_SPIKE = 30.0
ANOMALY_SPIKE = 55.0
ANOMALY_BEAT_INDEX = 12

X = [i * 0.02 for i in range(N)]
ALL_IDX = list(range(N))


def y_at_x(x):
    beat_num = int(x / PERIOD)
    phase = (x % PERIOD) / PERIOD
    spike_h = ANOMALY_SPIKE if beat_num == ANOMALY_BEAT_INDEX else NORMAL_SPIKE
    d = phase - 0.15
    pulse = spike_h * math.exp(-(d * d) / 0.0015)
    return BASELINE + pulse


def main():
    y_values = [y_at_x(x) for x in X]

    anomaly_range = [i for i, x in enumerate(X) if int(x / PERIOD) == ANOMALY_BEAT_INDEX]
    anomaly_peak_idx = max(anomaly_range, key=lambda i: y_values[i])
    anomaly_peak_value = y_values[anomaly_peak_idx]
    breakpoint_value = BASELINE + (anomaly_peak_value - BASELINE) * 0.9

    chart = (
        sp.line(
            "SeraStudio smooth demo - cardiogram sweep with a value breakpoint",
            x=X,
            y=y_values,
            variant="basic",
            show_points=False,
            color_hex=0x00A651,
        )
        .show_grid()
        .no_legend()
        .despine()
    )

    spls_path = os.path.join(OUT_DIR, "_tmp_serastudio_line_cardiogram.spls")
    if os.path.exists(spls_path):
        os.remove(spls_path)
    chart.record(spls_path)
    chart.push(ALL_IDX, X, y_values)
    chart.stop_record()
    n_frames = chart.gif_frame_count(spls_path)
    print("recorded", n_frames, "push(es) covering", N, "samples,", "anomaly beat peak at index", anomaly_peak_idx, "value", round(anomaly_peak_value, 2))

    out_path = os.path.join(OUT_DIR, "serastudio_smooth_line_cardiogram.mp4")
    chart.export_video(
        out_path, spls_path, format="mp4", fps=FPS, width=1100, height=550,
        max_frames=n_frames + 20, reveal=True, reveal_steps=200, reveal_fixed_scale=True,
        breakpoint_on_value=[(anomaly_peak_idx, breakpoint_value)], breakpoint_pause_ms=1500,
    )
    print("wrote", out_path)

    os.remove(spls_path)


if __name__ == "__main__":
    main()
</script>

</div>
