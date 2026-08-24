import datetime
import math
import re
import subprocess
from pathlib import Path

import seraplot as sp

REPO_ROOT = Path(__file__).resolve().parent.parent
OUT_HTML = REPO_ROOT / "docs" / "commit-history.html"
OUT_PNG = REPO_ROOT / "docs" / "theme" / "images" / "commit-history-preview.png"

BOT_AVATAR = "https://avatars.githubusercontent.com/in/15368?v=4"
KNOWN_AUTHORS = {
    "feur25": {"key": "feur25", "display": "FeurKing (feur25)", "avatar": "https://avatars.githubusercontent.com/u/39668417?v=4"},
    "FeurKing": {"key": "feur25", "display": "FeurKing (feur25)", "avatar": "https://avatars.githubusercontent.com/u/39668417?v=4"},
}

TYPE_MAP = {
    "feat": "feat", "fix": "fix", "docs": "docs",
    "refactor": "refactor", "refractor": "refactor", "move": "refactor", "rename": "refactor",
    "redesign": "refactor", "polish": "refactor",
    "chore": "chore", "build": "chore", "ci": "chore",
    "style": "style", "stye": "style",
    "perf": "perf", "test": "test",
}
PREFIX_RE = re.compile(r"^([a-zA-Z]+)(\([a-zA-Z0-9_./-]+\))?!?:")


def commit_type(subject):
    m = PREFIX_RE.match(subject)
    if not m:
        return "other"
    return TYPE_MAP.get(m.group(1).lower(), "other")


def resolve_author(name, email):
    if "[bot]" in name:
        return {"key": name, "display": name, "avatar": BOT_AVATAR}
    if name in KNOWN_AUTHORS:
        return KNOWN_AUTHORS[name]
    login = email.split("@")[0].split("+")[-1] if "users.noreply.github.com" in email else name.replace(" ", "")
    return {"key": name, "display": name, "avatar": f"https://github.com/{login}.png"}


def load_commits():
    fmt = "@@%H\x1f%ad\x1f%an\x1f%ae\x1f%s"
    raw = subprocess.run(
        ["git", "log", "--reverse", "--date=short", "--shortstat", f"--pretty=format:{fmt}"],
        cwd=REPO_ROOT, capture_output=True, text=True, encoding="utf-8", errors="replace",
    ).stdout
    commits = []
    cur = None
    for line in raw.split("\n"):
        if line.startswith("@@"):
            if cur is not None:
                commits.append(cur)
            parts = line[2:].split("\x1f")
            if len(parts) != 5:
                cur = None
                continue
            h, date, author, email, subject = parts
            cur = {"hash": h[:7], "date": date, "author": author, "email": email, "subject": subject, "ins": 0, "del": 0, "files": 0}
        elif cur is not None and "changed" in line:
            m_files = re.search(r"(\d+) files? changed", line)
            m_ins = re.search(r"(\d+) insertion", line)
            m_del = re.search(r"(\d+) deletion", line)
            cur["files"] = int(m_files.group(1)) if m_files else 0
            cur["ins"] = int(m_ins.group(1)) if m_ins else 0
            cur["del"] = int(m_del.group(1)) if m_del else 0
    if cur is not None:
        commits.append(cur)

    for c in commits:
        d = datetime.date.fromisoformat(c["date"])
        monday = d - datetime.timedelta(days=d.weekday())
        c["week"] = monday.isoformat()
        c["type"] = commit_type(c["subject"])
        c["bot"] = "1" if "[bot]" in c["author"] else "0"
        who = resolve_author(c["author"], c["email"])
        c["author_key"] = who["key"]
        c["display_name"] = who["display"]
        c["avatar"] = who["avatar"]
        if c["ins"] == 0 and c["del"] == 0:
            c["ratio"] = "none"
        elif c["ins"] > c["del"]:
            c["ratio"] = "ins"
        elif c["del"] > c["ins"]:
            c["ratio"] = "del"
        else:
            c["ratio"] = "even"

    first_monday = datetime.date.fromisoformat(commits[0]["week"])
    last_monday = datetime.date.fromisoformat(commits[-1]["week"])
    weeks = []
    cur_w = first_monday
    while cur_w <= last_monday:
        weeks.append(cur_w.isoformat())
        cur_w += datetime.timedelta(days=7)
    return commits, weeks


commits, weeks = load_commits()

RATIO_COLOR = {"ins": "#2dd4bf", "del": "#ec4899", "even": "#1e3a8a", "none": "#f4b400"}
RATIO_LABEL = {
    "ins": "Turquoise ring: more insertions than deletions",
    "del": "Pink ring: more deletions than insertions",
    "even": "Dark blue circle: as many insertions as deletions",
    "none": "Small yellow circle: commit with no file changed",
}
INK = "#1e2430"
SUB = "#64748b"
FAINT = "#94a3b8"
BG = "#fbfbfd"

MONTHS = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December",
]


def fmt_date(iso):
    d = datetime.date.fromisoformat(iso)
    return f"{MONTHS[d.month - 1]} {d.day}, {d.year}"


by_week = {}
for c in commits:
    by_week.setdefault(c["week"], []).append(c)

ins_color = int(RATIO_COLOR["ins"][1:], 16)
del_color = int(RATIO_COLOR["del"][1:], 16)
week_short = [wk[5:] for wk in weeks]
weekly_ins = [sum(c["ins"] for c in by_week.get(wk, [])) for wk in weeks]
weekly_del = [sum(c["del"] for c in by_week.get(wk, [])) for wk in weeks]
ins_max = max(weekly_ins) or 1
del_max = max(weekly_del) or 1
scale_max = max(ins_max, del_max)
BAR_SPAN = 280.0
RIGHT_LABEL_W = 220.0
RIGHT_TICK_W = 80.0

max_lines = max((c["ins"] + c["del"]) for c in commits)
R_MIN, R_MAX = 2.4, 10.5


def radius_of(c):
    lines = c["ins"] + c["del"]
    return R_MIN + math.sqrt(lines / max_lines) * (R_MAX - R_MIN)


def pack(radii, padding=1.2):
    n = len(radii)
    pos = [(0.0, 0.0)] * n
    if n <= 1:
        return pos
    pos[1] = (radii[0] + radii[1] + padding, 0.0)
    if n == 2:
        return pos
    for i in range(2, n):
        ri = radii[i]
        cand_start = max(0, i - 70)
        check_start = max(0, i - 200)
        best = None
        best_dist = float("inf")
        for a in range(cand_start, i):
            for b in range(a + 1, i):
                ax, ay = pos[a]
                bx, by = pos[b]
                ra = radii[a] + ri + padding
                rb = radii[b] + ri + padding
                dx = bx - ax
                dy = by - ay
                d = math.hypot(dx, dy)
                if d < 1e-9 or d > ra + rb or d < abs(ra - rb):
                    continue
                aa = (ra * ra - rb * rb + d * d) / (2.0 * d)
                h2 = ra * ra - aa * aa
                if h2 < 0.0:
                    continue
                h = math.sqrt(h2)
                mx = ax + aa * dx / d
                my = ay + aa * dy / d
                ox = -dy / d * h
                oy = dx / d * h
                for px, py in ((mx + ox, my + oy), (mx - ox, my - oy)):
                    ok = True
                    for j in range(check_start, i):
                        if j == a or j == b:
                            continue
                        jx, jy = pos[j]
                        if math.hypot(jx - px, jy - py) < radii[j] + ri + padding - 1e-6:
                            ok = False
                            break
                    if ok:
                        dist = math.hypot(px, py)
                        if dist < best_dist:
                            best_dist = dist
                            best = (px, py)
        if best is None:
            angle = 0.0
            radius = ri + 5.0
            for _ in range(2500):
                px = radius * math.cos(angle)
                py = radius * math.sin(angle)
                ok = True
                for j in range(check_start, i):
                    jx, jy = pos[j]
                    if math.hypot(jx - px, jy - py) < radii[j] + ri + padding:
                        ok = False
                        break
                if ok:
                    best = (px, py)
                    break
                angle += 0.31
                radius += 0.6
            if best is None:
                best = (radius, 0.0)
        pos[i] = best
    return pos


TOP_H = BAR_SPAN * (ins_max / scale_max) + 60.0
RIGHT_W = RIGHT_LABEL_W + BAR_SPAN * (del_max / scale_max) + RIGHT_TICK_W
TYPE_H = 210.0
COLS = 9
CELL = 340.0
X0 = 220.0
Y0 = 240.0 + TOP_H
ROWS = math.ceil(len(weeks) / COLS)
GRID_W = (COLS - 1) * CELL
W = int(X0 + GRID_W + 250 + RIGHT_W)
H = int(Y0 + (ROWS - 1) * CELL + 420 + TYPE_H)
CELL_R = CELL * 0.44

cv = sp.canvas(W, H, BG)

anchors = []
for i, wk in enumerate(weeks):
    row = i // COLS
    col = i % COLS
    cidx = col if row % 2 == 0 else COLS - 1 - col
    ax = X0 + cidx * CELL
    ay = Y0 + row * CELL
    count = len(by_week.get(wk, []))
    t = (count / max(len(v) for v in by_week.values())) ** 0.7
    r = CELL_R * (0.3 + 0.7 * t)
    anchors.append((ax, ay, r))


def glow_line(x1, y1, x2, y2, color, width, opacity):
    cv.line(x1, y1, x2, y2, color=color, width=width * 3.4, opacity=opacity * 0.25, layer="bg")
    cv.line(x1, y1, x2, y2, color=color, width=width, opacity=opacity, layer="bg")


def glow_curve(pts, color, width, opacity, tension):
    cv.curve(pts, color=color, width=width * 3.4, opacity=opacity * 0.22, tension=tension, layer="bg")
    cv.curve(pts, color=color, width=width, opacity=opacity, tension=tension, layer="bg")


LINE_COLOR = "#5aa9f2"
LINE_W = 3.4
RING_COLOR = "#7dd3fc"
RING_W = 3.0

for i in range(len(weeks) - 1):
    ax, ay, ra = anchors[i]
    bx, by, rb = anchors[i + 1]
    same_row = i // COLS == (i + 1) // COLS
    if same_row:
        dx, dy = bx - ax, by - ay
        dist = math.hypot(dx, dy) or 1.0
        ux, uy = dx / dist, dy / dist
        glow_line(ax + ux * ra, ay + uy * ra, bx - ux * rb, by - uy * rb, LINE_COLOR, LINE_W, 0.9)
    else:
        row_i = i // COLS
        col_i = i % COLS
        cidx_i = col_i if row_i % 2 == 0 else COLS - 1 - col_i
        dirn = -1.0 if cidx_i == 0 else 1.0
        clear = max(ra, rb) + 60.0
        bulge = max(CELL * 0.55, clear * 1.6) * dirn
        midx = (ax + bx) / 2.0 + bulge
        midy = (ay + by) / 2.0
        sx, sy = ax + dirn * ra, ay
        ex, ey = bx + dirn * rb, by
        c1x, c2x = ax + dirn * (ra + 40.0), bx + dirn * (rb + 40.0)
        glow_curve(
            [[sx, sy], [c1x, ay], [midx, midy], [c2x, by], [ex, ey]],
            LINE_COLOR, LINE_W, 0.9, 0.8,
        )

for i, (ax, ay, r) in enumerate(anchors):
    cv.circle(ax, ay, r, fill="none", stroke=RING_COLOR, stroke_width=RING_W, opacity=0.95, layer="fg")
    cv.circle(ax, ay, max(r - RING_W * 2.6, 1.0), fill="none", stroke=RING_COLOR, stroke_width=RING_W, opacity=0.55, layer="fg")
    cv.line(ax, ay + r, ax, ay + r + 16.0, color=RING_COLOR, width=1.8, opacity=0.75, layer="fg")
    cv.text(weeks[i][5:], ax, ay + r + 34.0, size=14.0, color="#475569", weight="600", anchor="middle")

idx = 0
for wi, wk in enumerate(weeks):
    entries = by_week.get(wk, [])
    if not entries:
        continue
    ax, ay, ring_r = anchors[wi]
    order = sorted(range(len(entries)), key=lambda k: -(entries[k]["ins"] + entries[k]["del"]))
    radii = [radius_of(entries[k]) for k in order]
    local = pack(radii)
    min_x = min(px - r for (px, py), r in zip(local, radii))
    max_x = max(px + r for (px, py), r in zip(local, radii))
    min_y = min(py - r for (px, py), r in zip(local, radii))
    max_y = max(py + r for (px, py), r in zip(local, radii))
    mid_x = (min_x + max_x) / 2.0
    mid_y = (min_y + max_y) / 2.0
    enclosing_r = max(math.hypot(px - mid_x, py - mid_y) + r for (px, py), r in zip(local, radii))
    fit = min((ring_r - 8.0) / max(enclosing_r, 1e-6), 2.4)

    for k, orig_i in enumerate(order):
        c = entries[orig_i]
        lx, ly = local[k]
        cx = ax + (lx - mid_x) * fit
        cy = ay + (ly - mid_y) * fit
        r = radii[k] * fit
        ring_col = RATIO_COLOR[c["ratio"]]
        name = f"m{idx}"
        idx += 1
        if c["bot"] == "1":
            pts = [
                [cx + r * math.cos(math.pi / 3 * s - math.pi / 2), cy + r * math.sin(math.pi / 3 * s - math.pi / 2)]
                for s in range(6)
            ]
            cv.polygon(
                pts, fill=ring_col, stroke="#ffffff", stroke_width=max(r * 0.22, 0.6), opacity=0.95,
                hover_group=c["author_key"], name=name,
            )
        else:
            cv.circle(
                cx, cy, r, fill=ring_col, stroke="#ffffff", stroke_width=max(r * 0.32, 0.7), opacity=0.95,
                hover_group=c["author_key"], name=name,
            )
        files_label = f"{c['files']} file changed" if c["files"] == 1 else f"{c['files']} files changed"
        kv = [
            ("Date", f"committed on {fmt_date(c['date'])}"),
            ("Changes", files_label),
            ("Insertions", f"+{c['ins']}"),
            ("Deletions", f"-{c['del']}"),
            ("Hash", c["hash"]),
        ]
        cv.tooltip(name, c["display_name"], kv, avatar=c["avatar"], subtitle=c["subject"])


def flatten_chart(chart):
    html = chart.html.replace(".sp-bg{fill:#ffffff}", f".sp-bg{{fill:{BG}}}")
    html = html.replace(
        "border-radius:12px;overflow:hidden;box-shadow:0 2px 8px rgba(0,0,0,.07),0 0 0 1px rgba(0,0,0,.04)",
        "overflow:hidden",
    )
    return sp.Chart(html)


top_x = X0 - CELL_R
top_w = GRID_W + 2.0 * CELL_R
top_y = 210.0
top_h = Y0 - top_y - 36.0

right_x = X0 + GRID_W + CELL_R + 50.0
right_w = W - right_x - 60.0
right_y = Y0 - CELL_R - 10.0
right_h = (ROWS - 1) * CELL + 2.0 * CELL_R + 20.0

author_counts = {}
for c in commits:
    author_counts[c["author_key"]] = author_counts.get(c["author_key"], 0) + 1
author_order = sorted(author_counts, key=lambda a: -author_counts[a])

NO_PULSE = "animation:none!important;transform:none!important;filter:none!important;"


def place_margin(values_top, values_right, group, name_suffix):
    a_top = flatten_chart(sp.bar(
        "", labels=week_short, values=values_top, variant="basic",
        color_hex=ins_color, theme="none", show_values=False,
        width=int(top_w), height=int(top_h),
    ).hide_grid().segment_bars().no_select())
    a_right = flatten_chart(sp.bar(
        "", labels=week_short, values=[-d for d in values_right], variant="diverging",
        color_low=del_color, color_high=ins_color, theme="none", show_values=False,
        width=int(right_w), height=int(right_h),
    ).hide_grid().segment_bars().no_select())
    top_name = f"margin-top-{name_suffix}"
    right_name = f"margin-right-{name_suffix}"
    cv.place(a_top, top_x, top_y, top_w, top_h, group=group, name=top_name)
    cv.place(a_right, right_x, right_y, right_w, right_h, group=group, name=right_name)
    cv.style(top_name, NO_PULSE)
    cv.style(right_name, NO_PULSE)


for author in author_order:
    a_ins = [sum(c["ins"] for c in by_week.get(wk, []) if c["author_key"] == author) for wk in weeks]
    a_del = [sum(c["del"] for c in by_week.get(wk, []) if c["author_key"] == author) for wk in weeks]
    place_margin(a_ins, a_del, author, re.sub(r"[^a-zA-Z0-9]", "-", author))

place_margin(weekly_ins, weekly_del, "__combined__", "all")

TYPES_ALL = ["feat", "fix", "docs", "refactor", "perf", "test", "style", "chore", "other"]
type_counts_all = {}
for c in commits:
    type_counts_all[c["type"]] = type_counts_all.get(c["type"], 0) + 1
type_order = sorted((t for t in TYPES_ALL if type_counts_all.get(t, 0) > 0), key=lambda t: -type_counts_all[t])
type_counts = [type_counts_all[t] for t in type_order]
n_types = len(type_order)

type_w = GRID_W + 2.0 * CELL_R
type_chart_h = TYPE_H - 60.0
type_top_y = H - 230.0 - TYPE_H + 30.0
type_chart = flatten_chart(sp.scatter(
    "", x_values=list(range(n_types)), y_values=[0] * n_types, variant="sized",
    color_values=type_counts, min_size=16, max_size=52,
    color_low=ins_color, color_high=del_color, theme="none",
    width=int(type_w), height=int(type_chart_h),
).no_axes().hide_grid().no_select())
cv.place(type_chart, top_x, type_top_y, type_w, type_chart_h, name="type-scatter")
cv.style("type-scatter", "animation:none!important;transform:none!important;filter:none!important;")
cv.text("COMMIT TYPES", top_x, type_top_y - 10.0, size=11.0, color=FAINT, weight="700", letter_spacing=1.2)
for i, t in enumerate(type_order):
    tx = top_x + type_w * ((i + 0.5) / n_types)
    cv.text(f"{t} ({type_counts[i]})", tx, type_top_y + type_chart_h + 18.0, size=12.0, color=SUB, anchor="middle", weight="600")

cv.text("INSERTIONS PER WEEK", top_x, top_y - 10.0, size=11.0, color=FAINT, weight="700", letter_spacing=1.2)
cv.text("DELETIONS PER WEEK", right_x, right_y - 10.0, size=11.0, color=FAINT, weight="700", letter_spacing=1.2)

cv.text("Commit History", X0, 96, size=36, color=INK, weight="800", letter_spacing=1)
cv.text("every real commit from Sera's repo, one circle at a time", X0, 132, size=16, color=SUB)
cv.text(f"{weeks[0]} → {weeks[-1]}", W - 110, 96, size=16, color=FAINT, anchor="end", weight="700")

total = len(commits)
total_lines = sum(c["ins"] + c["del"] for c in commits)
authors = sorted({c["author_key"] for c in commits})
cv.text(
    f"{total} commits · {total_lines:,} lines changed · {len(authors)} contributors",
    W - 110, 132, size=15, color=SUB, anchor="end",
)

leg_x = X0
leg_top = H - 230
leg_y = leg_top
for key in ("ins", "del", "even", "none"):
    cv.circle(leg_x + 9, leg_y, 8.0, fill=RATIO_COLOR[key], stroke="#ffffff", stroke_width=3.0, opacity=0.95)
    cv.text(RATIO_LABEL[key], leg_x + 26, leg_y + 5, size=13.0, color=SUB)
    leg_y += 27

hex_pts = [[leg_x + 9 + 8.0 * math.cos(math.pi / 3 * s - math.pi / 2), leg_y + 8.0 * math.sin(math.pi / 3 * s - math.pi / 2)] for s in range(6)]
cv.polygon(hex_pts, fill="#94a3b8", stroke="#ffffff", stroke_width=2.6)
cv.text("A hexagon is an automated commit", leg_x + 26, leg_y + 5, size=13.0, color=SUB)
leg_y += 38

cv.text(
    "hover a commit for its details, or an author's mark to rescale the margins to their own activity",
    leg_x, leg_y + 5, size=13.0, color=FAINT,
)
cv.text(
    "seraplot · regenerated automatically every week from Sera's git history",
    W - 110, H - 40, size=13.0, color=FAINT, anchor="end",
)

chart = cv.build()
OUT_HTML.parent.mkdir(parents=True, exist_ok=True)
chart.save(str(OUT_HTML))
print("saved", OUT_HTML)

try:
    from playwright.sync_api import sync_playwright

    OUT_PNG.parent.mkdir(parents=True, exist_ok=True)
    with sync_playwright() as p:
        browser = p.chromium.launch()
        page = browser.new_page(viewport={"width": W + 80, "height": H + 80}, device_scale_factor=2)
        page.goto(OUT_HTML.resolve().as_uri())
        page.wait_for_timeout(1200)
        page.screenshot(path=str(OUT_PNG), full_page=True)
        browser.close()
    print("saved", OUT_PNG)
except ImportError:
    print("playwright not installed, skipping preview screenshot")
