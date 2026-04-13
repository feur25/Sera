# Word Cloud

## Signature

```python
sp.build_wordcloud(
    title: str,
    words: list[str],
    weights: list[float],
    *,
    width: int = 900,
    height: int = 480,
    palette: list[int] | None = None,
    background: str | None = None,
    max_words: int = 200,
) -> Chart
```

---

## Description

Word cloud (tag cloud) where font size reflects the weight of each word.

Words with higher `weights` are displayed larger. Layout is computed via a spiral placement algorithm.

---

## Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `title` | `str` | required | Chart title |
| `words` | `list[str]` | required | Word|
| `weights` | `list[float]` | required | Weight per word (higher = larger) |
| `width` | `int` | `900` | Canvas width |
| `height` | `int` | `480` | Canvas height |
| `palette` | `list[int] \| None` | `None` | Custom color palette |
| `background` | `str \| None` | `None` | Background color |
| `max_words` | `int` | `200` | Maximum number of words rendered |

---

## Returns

`Chart`

---

## Examples

### Technology popularity

```python
import seraplot as sp
from collections import Counter

text = "python python rust rust rust go go java javascript python data ml deep learning neural"
counts = Counter(text.split())

chart = sp.build_wordcloud(
    "Tech Mentions",
    words=list(counts.keys()),
    frequencies=list(counts.values()),
    palette=[0x6366f1, 0x22d3ee, 0xf43f5e, 0xf59e0b, 0x10b981],
)
```


<details open>
<summary style="cursor:pointer;font-weight:600;padding:4px 0;color:#94a3b8">&#9654;&nbsp;Live Preview</summary>

<iframe src="../../previews/wordcloud.html" style="width:100%;height:520px;border:none;border-radius:8px;display:block;background:#0d1117" loading="lazy"></iframe>

</details>

---

## See also

- [Bar Chart](bar.md)
