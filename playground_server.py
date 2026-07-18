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


def _html_of(obj):
    for attr in ("_repr_html_", "html", "to_html"):
        member = getattr(obj, attr, None)
        if member is None:
            continue
        return member() if callable(member) else member
    return None


def _emit(loop, queue, item):
    loop.call_soon_threadsafe(queue.put_nowait, item)


async def _stream_job(run_id, send, job):
    loop = asyncio.get_event_loop()
    queue = asyncio.Queue()

    def _stream(chart):
        html = _html_of(chart)
        if html:
            _emit(loop, queue, {"t": "frame", "id": run_id, "html": html})

    def _worker():
        try:
            result = job(_stream)
            html = _html_of(result) if result is not None else None
            if html:
                _emit(loop, queue, {"t": "frame", "id": run_id, "html": html})
        except Exception:
            _emit(loop, queue, {"t": "err", "id": run_id, "msg": traceback.format_exc()})
        finally:
            _emit(loop, queue, {"t": "done", "id": run_id})

    threading.Thread(target=_worker, daemon=True).start()

    while True:
        item = await queue.get()
        await send(json.dumps(item))
        if item["t"] in ("done", "err"):
            break


def _exec_job(code):
    def job(_stream):
        sp = _sp()
        ns = {"sp": sp, "_stream": _stream, "__builtins__": __builtins__}
        exec(compile(code, "<playground>", "exec"), ns)
        for name in ("c", "chart", "fig", "result", "out"):
            if name in ns and ns[name] is not None:
                return ns[name]
        return None
    return job


def _call_job(fn_name, kwargs):
    def job(_stream):
        sp = _sp()
        fn = getattr(sp, fn_name, None) if sp else None
        if fn is None:
            raise NameError(f"sp.{fn_name} is not a known function")
        return fn(**kwargs)
    return job


async def _handle_ping(ws, msg):
    sp = _sp()
    ver = getattr(sp, "__version__", "?") if sp else "not found"
    await ws.send(json.dumps({"t": "pong", "version": ver}))


async def _handle_list(ws, msg):
    await ws.send(json.dumps({"t": "list", "fns": _list_fns()}))


async def _handle_sig(ws, msg):
    params = _signature(msg.get("fn", ""))
    await ws.send(json.dumps({"t": "sig", "fn": msg.get("fn"), "params": params}))


async def _handle_run(ws, msg):
    run_id = msg.get("id", 0)
    if not _sp():
        await ws.send(json.dumps({"t": "err", "id": run_id, "msg": "seraplot not found — activate your venv"}))
        return
    await _stream_job(run_id, ws.send, _exec_job(msg.get("code", "")))


async def _handle_call(ws, msg):
    run_id = msg.get("id", 0)
    if not _sp():
        await ws.send(json.dumps({"t": "err", "id": run_id, "msg": "seraplot not found — activate your venv"}))
        return
    await _stream_job(run_id, ws.send, _call_job(msg.get("fn", ""), msg.get("kwargs") or {}))


_ACTIONS = {
    "ping": _handle_ping,
    "list": _handle_list,
    "sig": _handle_sig,
    "run": _handle_run,
    "call": _handle_call,
}


async def _handler(ws):
    async for raw in ws:
        try:
            msg = json.loads(raw)
            handler = _ACTIONS.get(msg.get("a"))
            if handler:
                await handler(ws, msg)
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
