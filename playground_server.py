import asyncio
import json
import os
import sys
import traceback
import inspect
import threading

try:
    import websockets
except ImportError:
    print("pip install websockets")
    sys.exit(1)

PORT = int(os.environ.get("PORT", 7842))
HOST = os.environ.get("HOST", "127.0.0.1")


def _sp():
    try:
        import seraplot as sp
        return sp
    except ImportError:
        return None


def _list_fns():
    sp = _sp()
    if not sp:
        return []
    return sorted([
        n for n in dir(sp)
        if callable(getattr(sp, n)) and not n.startswith("_")
    ])


def _signature(fn_name):
    sp = _sp()
    if not sp:
        return None
    fn = getattr(sp, fn_name, None)
    if not fn:
        return None
    try:
        sig = inspect.signature(fn)
        out = []
        for name, p in sig.parameters.items():
            empty = inspect.Parameter.empty
            out.append({
                "name": name,
                "required": p.default is empty,
                "default": None if p.default is empty else repr(p.default),
                "kind": p.kind.name,
            })
        return out
    except Exception:
        return None


async def _exec(run_id, code, send):
    sp = _sp()
    if not sp:
        await send(json.dumps({"t": "err", "id": run_id, "msg": "seraplot not found — activate your venv"}))
        return

    loop = asyncio.get_event_loop()
    queue = asyncio.Queue()

    def _stream(chart):
        html = None
        if hasattr(chart, "_repr_html_"):
            html = chart._repr_html_()
        elif hasattr(chart, "html"):
            html = chart.html
        elif hasattr(chart, "to_html"):
            html = chart.to_html()
        if html:
            loop.call_soon_threadsafe(
                queue.put_nowait,
                {"t": "frame", "id": run_id, "html": html}
            )

    ns = {
        "sp": sp,
        "_stream": _stream,
        "__builtins__": __builtins__,
    }

    def _run():
        try:
            exec(compile(code, "<playground>", "exec"), ns)
            result = None
            for name in ("c", "chart", "fig", "result", "out"):
                if name in ns and ns[name] is not None:
                    result = ns[name]
                    break
            if result is not None:
                html = None
                if hasattr(result, "_repr_html_"):
                    html = result._repr_html_()
                elif hasattr(result, "html"):
                    html = result.html
                elif hasattr(result, "to_html"):
                    html = result.to_html()
                if html:
                    loop.call_soon_threadsafe(
                        queue.put_nowait,
                        {"t": "frame", "id": run_id, "html": html}
                    )
        except Exception:
            loop.call_soon_threadsafe(
                queue.put_nowait,
                {"t": "err", "id": run_id, "msg": traceback.format_exc()}
            )
        finally:
            loop.call_soon_threadsafe(
                queue.put_nowait,
                {"t": "done", "id": run_id}
            )

    threading.Thread(target=_run, daemon=True).start()

    while True:
        item = await queue.get()
        await send(json.dumps(item))
        if item["t"] in ("done", "err"):
            break


async def _handler(ws):
    async for raw in ws:
        try:
            msg = json.loads(raw)
            a = msg.get("a")

            if a == "ping":
                sp = _sp()
                ver = getattr(sp, "__version__", "?") if sp else "not found"
                await ws.send(json.dumps({"t": "pong", "version": ver}))

            elif a == "list":
                await ws.send(json.dumps({"t": "list", "fns": _list_fns()}))

            elif a == "sig":
                params = _signature(msg.get("fn", ""))
                await ws.send(json.dumps({"t": "sig", "fn": msg.get("fn"), "params": params}))

            elif a == "run":
                await _exec(msg.get("id", 0), msg.get("code", ""), ws.send)

        except Exception:
            try:
                await ws.send(json.dumps({"t": "err", "id": 0, "msg": traceback.format_exc()}))
            except Exception:
                pass


async def _main():
    sp = _sp()
    ver = getattr(sp, "__version__", "not found") if sp else "not found"
    print(f"SeraPlot playground server  ws://{HOST}:{PORT}  (seraplot {ver})")
    async with websockets.serve(_handler, HOST, PORT, origins=None):
        await asyncio.Future()


if __name__ == "__main__":
    asyncio.run(_main())
