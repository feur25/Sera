import json
from pathlib import Path

import seraplot as sp

OUTPUT = Path(__file__).resolve().parents[1] / "theme" / "js" / "method-registry.js"


def main():
    docs = sorted(sp.docs(), key=lambda d: d["name"])
    payload = json.dumps({"docs": docs}, ensure_ascii=False, separators=(",", ":"))
    OUTPUT.write_text("window.SeraPlotMethodRegistry=" + payload + ";", encoding="utf-8")
    print(f"wrote {OUTPUT} ({len(docs)} methods, {OUTPUT.stat().st_size} bytes)")


if __name__ == "__main__":
    main()
