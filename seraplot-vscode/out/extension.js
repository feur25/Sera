"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const vscode = __importStar(require("vscode"));
const path = __importStar(require("path"));
const cp = __importStar(require("child_process"));
const fs = __importStar(require("fs"));
const os = __importStar(require("os"));
const runtimeCatalog_1 = require("./runtimeCatalog");
const wheelLoader_1 = require("./wheelLoader");
const RECENT_KEY = "seraplot.recentFiles";
let previewPanel;
let previewView;
let lastPreviewHtml = "";
let activeFile;
let statusItem;
let wheelStatusItem;
let recentProvider;
let liveTypeTimer = undefined;
let extCtx;
let wheelWatcher;
let lastPyEditor;
let galleryProvider;
let homeProvider;
function activate(context) {
    extCtx = context;
    if (vscode.window.activeTextEditor && vscode.window.activeTextEditor.document.languageId === "python") {
        lastPyEditor = vscode.window.activeTextEditor;
    }
    autoInstallSeraplot();
    checkPyPIUpdate();
    statusItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusItem.text = "$(graph) SeraPlot";
    statusItem.command = "seraplot.preview";
    statusItem.tooltip = "Open SeraPlot live preview";
    if (vscode.workspace.getConfiguration("seraplot").get("statusBar", true))
        statusItem.show();
    wheelStatusItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 99);
    wheelStatusItem.command = "seraplot.useWheel";
    wheelStatusItem.tooltip = "Click to drop a SeraPlot wheel";
    refreshWheelStatus();
    if (vscode.workspace.getConfiguration("seraplot").get("statusBar", true))
        wheelStatusItem.show();
    homeProvider = new HomeProvider(context);
    galleryProvider = new GalleryProvider();
    recentProvider = new RecentProvider(context);
    const livePreviewProvider = new LivePreviewProvider(context);
    context.subscriptions.push(statusItem, vscode.window.registerWebviewViewProvider("seraplotHome", homeProvider, { webviewOptions: { retainContextWhenHidden: true } }), vscode.window.registerWebviewViewProvider("seraplotLivePreview", livePreviewProvider, { webviewOptions: { retainContextWhenHidden: true } }), vscode.window.registerTreeDataProvider("seraplotGallery", galleryProvider), vscode.window.registerTreeDataProvider("seraplotPreviews", recentProvider), vscode.commands.registerCommand("seraplot.preview", () => openPreview(context)), vscode.commands.registerCommand("seraplot.refreshPreview", () => { if (activeFile)
        renderPreview(activeFile); }), vscode.commands.registerCommand("seraplot.themeStudio", () => openThemeStudio(context)), vscode.commands.registerCommand("seraplot.chartStudio", () => openChartStudio(context)), vscode.commands.registerCommand("seraplot.gallery", () => openGallery(context)), vscode.commands.registerCommand("seraplot.insertChart", (entry) => insertChart(entry, context)), vscode.commands.registerCommand("seraplot.insertConfig", () => insertConfig()), vscode.commands.registerCommand("seraplot.newDemo", () => newDemo()), vscode.commands.registerCommand("seraplot.exportHtml", () => exportHtml()), vscode.commands.registerCommand("seraplot.openDocs", () => vscode.env.openExternal(vscode.Uri.parse("https://feur25.github.io/seraplot/"))), vscode.commands.registerCommand("seraplot.sysmon", () => openSysmon(context)), vscode.commands.registerCommand("seraplot.methodToggle", () => openMethodToggle(context)), vscode.commands.registerCommand("seraplot.wasmPlayground", () => openWasmPlayground(context)), vscode.commands.registerCommand("seraplot.autoInstall", () => autoInstallSeraplot()), vscode.commands.registerCommand("seraplot.uninstall", () => uninstallSeraplot()), vscode.commands.registerCommand("seraplot.refreshCatalog", async () => {
        const cat = await (0, runtimeCatalog_1.loadCatalog)(context, { force: true });
        galleryProvider.refresh();
        if (cat.error)
            vscode.window.showErrorMessage("SeraPlot introspection failed: " + cat.error);
        else
            vscode.window.showInformationMessage(`SeraPlot ${cat.version}: ${cat.charts.length} charts, ${cat.ml_models.length} ML models, ${cat.ml_functions.length} ml_* dict APIs, ${cat.ml_transformers.length} transformers, ${cat.utilities.length} utilities.`);
    }), vscode.commands.registerCommand("seraplot.useWheel", async () => {
        const picked = await (0, wheelLoader_1.pickWheel)();
        if (!picked)
            return;
        const info = (0, wheelLoader_1.probeWheel)(picked);
        if (!info) {
            vscode.window.showErrorMessage("Not a valid wheel filename: " + path.basename(picked));
            return;
        }
        await vscode.workspace.getConfiguration("seraplot").update("wheelPath", picked, vscode.ConfigurationTarget.Global);
        (0, runtimeCatalog_1.invalidateCatalog)(context);
        await vscode.window.withProgress({ location: vscode.ProgressLocation.Notification, title: `SeraPlot: loading wheel ${info.name} ${info.version}` }, async (progress) => {
            try {
                await (0, wheelLoader_1.installWheel)(context, picked, { force: true, progress });
            }
            catch (e) {
                vscode.window.showErrorMessage("Wheel install failed: " + e.message);
                return;
            }
            progress.report({ message: "Introspecting framework..." });
            const cat = await (0, runtimeCatalog_1.loadCatalog)(context, { force: true });
            galleryProvider.refresh();
            refreshWheelStatus();
            attachWheelWatcher(context, galleryProvider);
            if (cat.error)
                vscode.window.showErrorMessage("Wheel loaded but introspection failed: " + cat.error);
            else
                vscode.window.showInformationMessage(`SeraPlot ${cat.version} loaded from wheel \u2014 ${cat.charts.length} charts, ${cat.ml_models.length} ML models${cat.chart_methods ? `, ${cat.chart_methods.length} chart methods` : ""}.`);
        });
    }), vscode.commands.registerCommand("seraplot.useSystem", async () => {
        await vscode.workspace.getConfiguration("seraplot").update("wheelPath", "", vscode.ConfigurationTarget.Global);
        (0, runtimeCatalog_1.invalidateCatalog)(context);
        disposeWheelWatcher();
        const cat = await (0, runtimeCatalog_1.loadCatalog)(context, { force: true });
        galleryProvider.refresh();
        refreshWheelStatus();
        if (cat.error)
            vscode.window.showErrorMessage("System SeraPlot introspection failed: " + cat.error);
        else
            vscode.window.showInformationMessage(`Using system SeraPlot ${cat.version} \u2014 ${cat.charts.length} charts.`);
    }), vscode.commands.registerCommand("seraplot.openCatalogJson", async () => {
        const cat = (0, runtimeCatalog_1.getCachedCatalog)(context);
        if (!cat) {
            vscode.window.showWarningMessage("No catalog loaded yet.");
            return;
        }
        const doc = await vscode.workspace.openTextDocument({ content: JSON.stringify(cat, null, 2), language: "json" });
        await vscode.window.showTextDocument(doc, { preview: false });
    }), vscode.commands.registerCommand("seraplot.clearWheelCache", async () => {
        const n = (0, wheelLoader_1.clearWheelCache)(context);
        (0, runtimeCatalog_1.invalidateCatalog)(context);
        refreshWheelStatus();
        vscode.window.showInformationMessage(`Cleared ${n} wheel cache slot(s).`);
    }), vscode.commands.registerCommand("seraplot.togglePreview", () => {
        if (hasPreviewSurface() && (previewPanel || (previewView && previewView.visible))) {
            if (previewPanel) {
                previewPanel.dispose();
                previewPanel = undefined;
            }
            lastPreviewHtml = "";
            if (previewView)
                previewView.webview.html = idleHtml();
            vscode.window.setStatusBarMessage("SeraPlot preview hidden", 2000);
        }
        else {
            openPreview(extCtx);
        }
    }), vscode.commands.registerCommand("seraplot.previewFloat", async () => {
        const cfg = vscode.workspace.getConfiguration("seraplot");
        await cfg.update("previewMode", "window", vscode.ConfigurationTarget.Global);
        if (previewPanel) {
            previewPanel.dispose();
            previewPanel = undefined;
        }
        openPreview(extCtx);
    }), vscode.commands.registerCommand("seraplot.previewMode", async () => {
        const cfg = vscode.workspace.getConfiguration("seraplot");
        const cur = cfg.get("previewMode", "panel");
        const order = ["panel", "tab", "window"];
        const next = order[(order.indexOf(cur) + 1) % order.length];
        await cfg.update("previewMode", next, vscode.ConfigurationTarget.Global);
        if (previewPanel) {
            previewPanel.dispose();
            previewPanel = undefined;
        }
        if (next === "panel") {
            await vscode.commands.executeCommand("seraplotLivePreview.focus");
            if (activeFile)
                renderPreview(activeFile);
            else if (lastPreviewHtml && previewView)
                previewView.webview.html = lastPreviewHtml;
        }
        else {
            if (previewView)
                previewView.show?.(true);
            openPreview(extCtx);
        }
        vscode.window.setStatusBarMessage(`SeraPlot preview mode: ${next}`, 2500);
    }), vscode.commands.registerCommand("seraplot.listWheels", async () => {
        const items = (0, wheelLoader_1.listInstalledWheels)(context);
        if (items.length === 0) {
            vscode.window.showInformationMessage("No wheels installed in cache.");
            return;
        }
        const picked = await vscode.window.showQuickPick(items.map(w => ({
            label: `$(package) ${path.basename(w.info.wheelPath)}`,
            description: `v${w.info.version}`,
            detail: w.slot,
            data: w,
        })), { placeHolder: "Installed wheels (cache)" });
        if (picked && picked.data) {
            const w = picked.data;
            if (fs.existsSync(w.info.wheelPath)) {
                await vscode.workspace.getConfiguration("seraplot").update("wheelPath", w.info.wheelPath, vscode.ConfigurationTarget.Global);
                (0, runtimeCatalog_1.invalidateCatalog)(context);
                await (0, runtimeCatalog_1.loadCatalog)(context, { force: true });
                galleryProvider.refresh();
                refreshWheelStatus();
                attachWheelWatcher(context, galleryProvider);
                vscode.window.showInformationMessage(`Activated cached wheel ${path.basename(w.info.wheelPath)}.`);
            }
        }
    }), vscode.workspace.onDidChangeConfiguration(ev => {
        if (ev.affectsConfiguration("seraplot.wheelPath")) {
            (0, runtimeCatalog_1.invalidateCatalog)(context);
            refreshWheelStatus();
            attachWheelWatcher(context, galleryProvider);
            (0, runtimeCatalog_1.loadCatalog)(context, { force: true }).then(() => galleryProvider.refresh());
        }
    }), vscode.workspace.onDidSaveTextDocument(doc => {
        const cfg = vscode.workspace.getConfiguration("seraplot");
        if (!cfg.get("autoReload", true))
            return;
        if (hasPreviewSurface() && activeFile && doc.fileName === activeFile)
            renderPreview(activeFile);
    }), vscode.workspace.onDidChangeTextDocument(ev => {
        const cfg = vscode.workspace.getConfiguration("seraplot");
        if (!cfg.get("liveType", true))
            return;
        if (!hasPreviewSurface() || !activeFile)
            return;
        if (ev.document.fileName !== activeFile)
            return;
        if (liveTypeTimer)
            clearTimeout(liveTypeTimer);
        liveTypeTimer = setTimeout(() => { if (activeFile)
            renderPreview(activeFile); }, 800);
    }), vscode.window.onDidChangeActiveTextEditor(ed => {
        if (ed && ed.document.languageId === "python")
            lastPyEditor = ed;
        const cfg = vscode.workspace.getConfiguration("seraplot");
        if (!cfg.get("previewOnOpen", false))
            return;
        if (ed && ed.document.languageId === "python" && !hasPreviewSurface())
            openPreview(context);
    }));
    (0, runtimeCatalog_1.loadCatalog)(context).then(cat => {
        if (cat.error) {
            vscode.window.showWarningMessage("SeraPlot: framework introspection failed. The Studio uses cached/empty data. Run \"SeraPlot: Refresh Framework Catalog\" after fixing the python path. (" + cat.error.slice(0, 160) + ")");
        }
        galleryProvider.refresh();
        homeProvider.setVersion(cat.version, cat.charts.length);
        refreshWheelStatus();
        attachWheelWatcher(context, galleryProvider);
    });
}
function deactivate() {
    if (previewPanel)
        previewPanel.dispose();
    if (statusItem)
        statusItem.dispose();
    if (wheelStatusItem)
        wheelStatusItem.dispose();
    disposeWheelWatcher();
}
function refreshWheelStatus() {
    if (!wheelStatusItem)
        return;
    const wp = (0, wheelLoader_1.getActiveWheelPath)();
    if (wp) {
        const info = (0, wheelLoader_1.probeWheel)(wp);
        if (info) {
            wheelStatusItem.text = `$(package) ${info.name} ${info.version}`;
            wheelStatusItem.tooltip = `Active SeraPlot wheel:\n${wp}\nClick to swap, right-click via Command Palette > SeraPlot.`;
            return;
        }
    }
    const cat = extCtx ? (0, runtimeCatalog_1.getCachedCatalog)(extCtx) : undefined;
    wheelStatusItem.text = `$(package) sp:system${cat ? " " + cat.version : ""}`;
    wheelStatusItem.tooltip = "SeraPlot resolved from system Python. Click to drop a wheel.";
}
function disposeWheelWatcher() {
    if (wheelWatcher) {
        try {
            wheelWatcher.dispose();
        }
        catch { }
        wheelWatcher = undefined;
    }
}
function attachWheelWatcher(context, galleryProvider) {
    disposeWheelWatcher();
    const wp = (0, wheelLoader_1.getActiveWheelPath)();
    if (!wp)
        return;
    let pending;
    wheelWatcher = (0, wheelLoader_1.watchWheel)(wp, () => {
        if (pending)
            clearTimeout(pending);
        pending = setTimeout(async () => {
            (0, runtimeCatalog_1.invalidateCatalog)(context);
            try {
                await (0, wheelLoader_1.installWheel)(context, wp, { force: true });
                const cat = await (0, runtimeCatalog_1.loadCatalog)(context, { force: true });
                galleryProvider.refresh();
                refreshWheelStatus();
                if (!cat.error)
                    vscode.window.setStatusBarMessage(`$(sync) SeraPlot wheel reloaded: ${cat.version}`, 4000);
            }
            catch (e) {
                vscode.window.showErrorMessage("Wheel auto-reload failed: " + e.message);
            }
        }, 400);
    });
    context.subscriptions.push(wheelWatcher);
}
function previewColumn() {
    const v = vscode.workspace.getConfiguration("seraplot").get("previewColumn", "beside");
    switch (v) {
        case "active": return vscode.ViewColumn.Active;
        case "one": return vscode.ViewColumn.One;
        case "two": return vscode.ViewColumn.Two;
        case "three": return vscode.ViewColumn.Three;
        default: return vscode.ViewColumn.Beside;
    }
}
function previewMode() {
    const v = vscode.workspace.getConfiguration("seraplot").get("previewMode", "panel");
    if (v === "tab")
        return "tab";
    if (v === "window")
        return "window";
    return "panel";
}
function hasPreviewSurface() {
    return !!previewPanel || !!previewView;
}
function previewWebview() {
    if (previewMode() === "tab")
        return previewPanel?.webview;
    return previewView?.webview || previewPanel?.webview;
}
function setPreviewHtml(html) {
    lastPreviewHtml = html;
    if (previewPanel)
        previewPanel.webview.html = html;
    if (previewView)
        previewView.webview.html = html;
}
class LivePreviewProvider {
    constructor(ctx) {
        this.ctx = ctx;
    }
    resolveWebviewView(view) {
        previewView = view;
        view.webview.options = { enableScripts: true };
        view.webview.html = lastPreviewHtml || idleHtml();
        view.webview.onDidReceiveMessage(msg => {
            if (msg && msg.type === "refresh" && activeFile)
                renderPreview(activeFile);
            if (msg && msg.type === "open" && activeFile)
                vscode.window.showTextDocument(vscode.Uri.file(activeFile));
            if (msg && msg.type === "switchMode")
                vscode.commands.executeCommand("seraplot.previewMode");
            if (msg && msg.type === "float")
                vscode.commands.executeCommand("seraplot.previewFloat");
        });
        view.onDidDispose(() => { previewView = undefined; });
        view.onDidChangeVisibility(() => {
            if (view.visible && lastPreviewHtml)
                view.webview.html = lastPreviewHtml;
        });
    }
}
function idleHtml() {
    return `<!DOCTYPE html><html><body style="margin:0;font-family:system-ui;background:#0d1117;color:#94a3b8;padding:20px;line-height:1.6">
<h3 style="margin:0 0 8px;color:#e6e6e6;font-size:14px">SeraPlot \u2014 Live Preview</h3>
<p style="font-size:12px">Open a Python file and run <b>SeraPlot: Live Preview</b> to render charts here.</p>
<p style="font-size:11px;color:#64748b">Drag this panel anywhere (bottom \u2194 side), or switch to an editor tab via <b>SeraPlot: Switch Preview Mode</b>.</p>
</body></html>`;
}
function htmlEscape(s) {
    return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");
}
function makeNonce() {
    let s = "";
    const chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    for (let i = 0; i < 32; i++)
        s += chars.charAt(Math.floor(Math.random() * chars.length));
    return s;
}
function pyLit(f, v) {
    if (v === null || v === undefined)
        return "None";
    const s = String(v).trim();
    if (s === "")
        return "None";
    if (f.kind === "bool")
        return v === true || s === "true" || s === "True" ? "True" : "False";
    if (f.kind === "int")
        return String(parseInt(s, 10));
    if (f.kind === "float")
        return String(parseFloat(s));
    if (f.kind === "color") {
        const hex = s.replace(/^#/, "").toUpperCase();
        return /^[0-9A-F]{6}$/.test(hex) ? `0x${hex}` : JSON.stringify(s);
    }
    if (f.kind === "color_str") {
        const hex = s.replace(/^#/, "").toUpperCase();
        if (/^[0-9A-F]{6}$/.test(hex))
            return JSON.stringify("#" + hex);
        return JSON.stringify(s);
    }
    if (f.kind === "json")
        return s;
    return JSON.stringify(s);
}
function inferKwargField(key, common) {
    const known = common.find(k => k.key === key);
    if (known)
        return known;
    const lower = key.toLowerCase();
    if (/_hex$|color$/.test(lower) && !lower.includes("background"))
        return { key, kind: "color", default: "" };
    if (/^background|^bg$|_color$/.test(lower))
        return { key, kind: "color_str", default: "" };
    if (/^show_|^enable_|^is_|^has_|^use_/.test(lower))
        return { key, kind: "bool", default: false };
    if (/_size$|_width$|_height$|_margin$|_radius$|count$|^k$|^min_|^max_|^n_/.test(lower))
        return { key, kind: "int", default: 0 };
    if (/_alpha$|_eps$|_threshold$|_ratio$|_rate$|opacity$/.test(lower))
        return { key, kind: "float", default: 0 };
    return { key, kind: "string", default: "" };
}
function renderKwargParts(values, common) {
    const parts = [];
    for (const key of Object.keys(values)) {
        const v = values[key];
        if (v === null || v === undefined)
            continue;
        if (typeof v === "string" && v.trim() === "")
            continue;
        const f = inferKwargField(key, common);
        const lit = pyLit(f, v);
        if (lit === "None")
            continue;
        parts.push(`${key}=${lit}`);
    }
    return parts;
}
function renderTypedKwargParts(values, typed, common) {
    const parts = [];
    const typedMap = new Map();
    for (const f of typed || [])
        typedMap.set(f.key, f);
    for (const key of Object.keys(values)) {
        const v = values[key];
        if (v === null || v === undefined)
            continue;
        if (typeof v === "string" && v.trim() === "")
            continue;
        const f = typedMap.get(key) || inferKwargField(key, common);
        const lit = pyLit(f, v);
        if (lit === "None")
            continue;
        parts.push(`${key}=${lit}`);
    }
    return parts;
}
function renderChainSuffix(chain, methods) {
    if (!Array.isArray(chain) || chain.length === 0)
        return "";
    const methodMap = new Map();
    for (const m of methods)
        methodMap.set(m.name, m);
    const out = [];
    for (const item of chain) {
        if (!item || !item.name)
            continue;
        const def = methodMap.get(String(item.name));
        if (!def)
            continue;
        const args = [];
        const positional = item.positional && typeof item.positional === "object" ? item.positional : {};
        for (const p of (def.params || [])) {
            const v = positional[p];
            if (v === undefined || v === null || (typeof v === "string" && v.trim() === ""))
                continue;
            const f = (def.kwargs || []).find(k => k.key === p) || { key: p, kind: "string", default: "" };
            args.push(pyLit(f, v));
        }
        const kw = item.kwargs && typeof item.kwargs === "object" ? item.kwargs : {};
        for (const key of Object.keys(kw)) {
            const v = kw[key];
            if (v === null || v === undefined || (typeof v === "string" && v.trim() === ""))
                continue;
            const f = (def.kwargs || []).find(k => k.key === key) || { key, kind: "string", default: "" };
            const lit = pyLit(f, v);
            if (lit === "None")
                continue;
            args.push(`${key}=${lit}`);
        }
        out.push(`.${def.name}(${args.join(", ")})`);
    }
    return out.join("");
}
function buildSpawnEnv(cat) {
    const env = { ...process.env };
    if (cat.source === "wheel" && cat.wheelPath) {
        try {
            const probed = (0, wheelLoader_1.probeWheel)(cat.wheelPath);
            if (probed && extCtx) {
                const slot = path.join(extCtx.globalStorageUri.fsPath, "wheel-cache", `${probed.name}-${probed.version}-${probed.sha256}`);
                if (fs.existsSync(slot)) {
                    const sep = process.platform === "win32" ? ";" : ":";
                    env.PYTHONPATH = env.PYTHONPATH ? `${slot}${sep}${env.PYTHONPATH}` : slot;
                }
            }
        }
        catch { }
    }
    return env;
}
function findImportInsertLine(doc) {
    let line = 0;
    const total = Math.min(doc.lineCount, 30);
    for (let i = 0; i < total; i++) {
        const txt = doc.lineAt(i).text;
        if (i === 0 && /^#!/.test(txt)) {
            line = i + 1;
            continue;
        }
        if (/^#.*coding[:=]/.test(txt)) {
            line = i + 1;
            continue;
        }
        if (/^\s*from\s+__future__/.test(txt)) {
            line = i + 1;
            continue;
        }
        break;
    }
    return line;
}
function captureLiteralFrom(doc, startLine, openIdx) {
    const open = doc.lineAt(startLine).text[openIdx];
    const close = open === "[" ? "]" : open === "(" ? ")" : open === "{" ? "}" : null;
    if (!close)
        return null;
    let depth = 0;
    let buf = "";
    let inStr = null;
    let esc = false;
    const maxLines = Math.min(doc.lineCount, startLine + 60);
    for (let li = startLine; li < maxLines; li++) {
        const text = doc.lineAt(li).text;
        const start = li === startLine ? openIdx : 0;
        for (let i = start; i < text.length; i++) {
            const ch = text[i];
            buf += ch;
            if (inStr) {
                if (esc) {
                    esc = false;
                    continue;
                }
                if (ch === "\\") {
                    esc = true;
                    continue;
                }
                if (ch === inStr)
                    inStr = null;
                continue;
            }
            if (ch === "\"" || ch === "'") {
                inStr = ch;
                continue;
            }
            if (ch === "[" || ch === "(" || ch === "{")
                depth++;
            else if (ch === "]" || ch === ")" || ch === "}") {
                depth--;
                if (depth === 0)
                    return buf;
            }
        }
        buf += " ";
        if (buf.length > 4000)
            return null;
    }
    return null;
}
function scanContext(doc) {
    const ctx = { importAlias: "sp", importLine: null, labelsVar: null, valuesVar: null, labelsLit: null, valuesLit: null };
    for (let i = 0; i < doc.lineCount; i++) {
        const t = doc.lineAt(i).text;
        const im = t.match(/^\s*import\s+seraplot(?:\s+as\s+(\w+))?/);
        if (im) {
            ctx.importAlias = im[1] || "seraplot";
            ctx.importLine = i;
            continue;
        }
        const fim = t.match(/^\s*from\s+seraplot\s+import/);
        if (fim) {
            ctx.importAlias = "sp";
            ctx.importLine = i;
            continue;
        }
        const ass = t.match(/^\s*(\w+)\s*=\s*/);
        if (!ass)
            continue;
        const name = ass[1];
        const isLabels = /^labels?$/i.test(name);
        const isValues = /^values?$/i.test(name);
        if (!isLabels && !isValues)
            continue;
        if (isLabels && !ctx.labelsVar)
            ctx.labelsVar = name;
        if (isValues && !ctx.valuesVar)
            ctx.valuesVar = name;
        const rhsStart = ass[0].length;
        const rhs = t.slice(rhsStart);
        const trimmed = rhs.trim();
        if (trimmed.startsWith("[") || trimmed.startsWith("(")) {
            const openIdx = t.indexOf(trimmed[0], rhsStart);
            const lit = captureLiteralFrom(doc, i, openIdx);
            if (lit) {
                if (isLabels && !ctx.labelsLit)
                    ctx.labelsLit = lit;
                if (isValues && !ctx.valuesLit)
                    ctx.valuesLit = lit;
            }
        }
    }
    return ctx;
}
async function ensureImport(ed, ctx) {
    if (ctx.importLine !== null)
        return;
    const line = findImportInsertLine(ed.document);
    await ed.edit(b => b.insert(new vscode.Position(line, 0), `import seraplot as sp\n`));
    ctx.importAlias = "sp";
}
function openPreview(context) {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== "python") {
        vscode.window.showWarningMessage("SeraPlot: open a Python file first.");
        return;
    }
    activeFile = editor.document.fileName;
    if (recentProvider)
        recentProvider.add(activeFile);
    if (previewMode() === "panel") {
        if (previewPanel) {
            previewPanel.dispose();
            previewPanel = undefined;
        }
        Promise.resolve(vscode.commands.executeCommand("seraplotLivePreview.focus")).then(() => {
            if (activeFile)
                renderPreview(activeFile);
        });
        if (activeFile)
            renderPreview(activeFile);
        return;
    }
    const wantWindow = previewMode() === "window";
    if (!previewPanel) {
        previewPanel = vscode.window.createWebviewPanel("seraplotPreview", "SeraPlot \u2014 " + path.basename(activeFile), previewColumn(), { enableScripts: true, retainContextWhenHidden: true });
        try {
            previewPanel.iconPath = vscode.Uri.file(path.join(context.extensionPath, "logo.png"));
        }
        catch (_e) { }
        previewPanel.webview.onDidReceiveMessage(msg => {
            if (msg && msg.type === "refresh" && activeFile)
                renderPreview(activeFile);
            if (msg && msg.type === "open" && activeFile)
                vscode.window.showTextDocument(vscode.Uri.file(activeFile));
            if (msg && msg.type === "switchMode")
                vscode.commands.executeCommand("seraplot.previewMode");
            if (msg && msg.type === "float")
                vscode.commands.executeCommand("seraplot.previewFloat");
        });
        previewPanel.onDidDispose(() => { previewPanel = undefined; });
    }
    else {
        previewPanel.title = "SeraPlot \u2014 " + path.basename(activeFile);
        previewPanel.reveal(previewColumn(), true);
    }
    if (wantWindow) {
        setTimeout(() => {
            vscode.commands.executeCommand("workbench.action.moveEditorToNewWindow").then(undefined, () => { });
        }, 250);
    }
    renderPreview(activeFile);
}
function renderPreview(file) {
    const cfg = vscode.workspace.getConfiguration("seraplot");
    const py = cfg.get("pythonPath", "python");
    const timeout = cfg.get("timeout", 30000);
    const tmpJson = path.join(os.tmpdir(), `seraplot_preview_${Date.now()}.json`);
    const escFile = file.replace(/\\/g, "\\\\");
    const escOut = tmpJson.replace(/\\/g, "\\\\");
    const code = [
        "import runpy, json, seraplot as sp",
        "_seen_hashes = set()",
        "_captured = []",
        "_orig_save = sp.Chart.save",
        "def _spy(self, path, *a, **kw):",
        "    result = _orig_save(self, path, *a, **kw)",
        "    h = self.html",
        "    if (not h or len(h) <= 32):",
        "        try:",
        "            with open(path, 'r', encoding='utf-8') as _fp: h = _fp.read()",
        "        except Exception: pass",
        "    if h and len(h) > 32:",
        "        hh = hash(h)",
        "        if hh not in _seen_hashes:",
        "            _seen_hashes.add(hh); _captured.append(h)",
        "    return result",
        "sp.Chart.save = _spy",
        "sp.set_auto_display(False)",
        "_globals = runpy.run_path(r'" + escFile + "', run_name='__main__')",
        "for _v in _globals.values():",
        "    if isinstance(_v, sp.Chart) and _v.html and len(_v.html) > 32:",
        "        hh = hash(_v.html)",
        "        if hh not in _seen_hashes:",
        "            _seen_hashes.add(hh); _captured.append(_v.html)",
        "with open(r'" + escOut + "', 'w', encoding='utf-8') as _f:",
        "    json.dump(_captured, _f)"
    ].join("\n");
    if (statusItem)
        statusItem.text = "$(sync~spin) SeraPlot";
    cp.execFile(py, ["-c", code], { timeout }, (err, _stdout, stderr) => {
        if (statusItem)
            statusItem.text = "$(graph) SeraPlot";
        if (!hasPreviewSurface())
            return;
        if (err) {
            setPreviewHtml(errorHtml(stderr || err.message, file));
            return;
        }
        try {
            const raw = fs.readFileSync(tmpJson, "utf-8");
            try {
                fs.unlinkSync(tmpJson);
            }
            catch (_e) { }
            const charts = JSON.parse(raw);
            if (!charts || charts.length === 0) {
                setPreviewHtml(noChartHtml(file));
            }
            else {
                const valid = charts.filter(h => h && h.length > 32);
                if (valid.length === 0) {
                    setPreviewHtml(errorHtml("The framework produced " + charts.length + " chart(s) but all were empty (0 bytes).\nThis usually means a parameter type is wrong (e.g. `x_values=` instead of `labels=`).", file));
                }
                else {
                    setPreviewHtml(wrapMultiHtml(valid, file));
                }
            }
        }
        catch (e) {
            setPreviewHtml(errorHtml(String(e), file));
        }
    });
}
// ─────────────────────────────────────────────────────────────────────────────
// Live Preview UI — redesigned toolbar + responsive chart sizing
// ─────────────────────────────────────────────────────────────────────────────
// Inject responsive CSS into a generated SeraPlot chart so the SVG (which is
// emitted with hardcoded width="900" height="480") fits whatever iframe size
// it lands in instead of overflowing the preview frame.
function makeChartResponsive(html) {
    const css = `<style id="__sp_responsive__">
html,body{margin:0;padding:0;width:100%;height:100%;background:transparent;overflow:hidden;display:flex;align-items:center;justify-content:center}
body>div{max-width:100%;max-height:100%;width:auto !important;height:auto !important;display:flex !important;align-items:center;justify-content:center}
body>div>svg,body>div svg:first-of-type{width:auto !important;height:auto !important;max-width:100% !important;max-height:100% !important;display:block}
canvas{max-width:100% !important;max-height:100% !important}
</style>`;
    if (/<\/head>/i.test(html))
        return html.replace(/<\/head>/i, css + "</head>");
    if (/<body[^>]*>/i.test(html))
        return html.replace(/<body([^>]*)>/i, `<head>${css}</head><body$1>`);
    return css + html;
}
function wrapMultiHtml(charts, file) {
    const name = htmlEscape(path.basename(file));
    const total = charts.length;
    const responsive = charts.map(makeChartResponsive);
    const firstSrcdoc = responsive[0].replace(/&/g, "&amp;").replace(/"/g, "&quot;");
    const safeJson = JSON.stringify(responsive).replace(/<\/script/gi, "<\\/script");
    const pagerHtml = total > 1
        ? `<div class="sp-pager">
        <button class="sp-pg-btn" id="prev" onclick="nav(-1)" title="Previous chart" disabled>
          <svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="10,3 5,8 10,13"/></svg>
        </button>
        <span class="sp-counter" id="ctr"><b>1</b> / ${total}</span>
        <button class="sp-pg-btn" id="next" onclick="nav(1)" title="Next chart">
          <svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="6,3 11,8 6,13"/></svg>
        </button>
      </div>`
        : "";
    const pagerScript = total > 1 ? `
let idx=0;
const charts=${safeJson};
const frm=document.getElementById('frm');
const ctr=document.getElementById('ctr');
const prev=document.getElementById('prev');
const next=document.getElementById('next');
function nav(d){idx=(idx+d+charts.length)%charts.length;frm.srcdoc=charts[idx];ctr.innerHTML='<b>'+(idx+1)+'</b> / '+charts.length;prev.disabled=idx===0;next.disabled=idx===charts.length-1;}
document.addEventListener('keydown',e=>{if(e.key==='ArrowLeft'&&!prev.disabled)nav(-1);else if(e.key==='ArrowRight'&&!next.disabled)nav(1);});
` : "";
    return `<!DOCTYPE html><html><head><meta charset="utf-8"><style>
*{box-sizing:border-box}
:root{
  --bg:#0b0e14; --bg-bar:linear-gradient(180deg,#161b26 0%,#11151e 100%);
  --bd:#1f2733; --bd-hi:#2a3343; --tx:#e2e8f0; --tx-mu:#7e8a9c;
  --ac:#818cf8; --ac-on:#6366f1; --ac-glow:rgba(99,102,241,.35);
  --live:#10b981; --live-glow:rgba(16,185,129,.45);
  --frame:#0d1117;
}
html,body{margin:0;padding:0;height:100vh;background:var(--bg);color:var(--tx);font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;font-size:13px}
.sp-wrap{display:flex;flex-direction:column;height:100vh}

/* ── Toolbar ─────────────────────────────────────────────────────────── */
.sp-bar{
  display:flex;align-items:center;gap:10px;
  padding:0 14px;height:42px;flex-shrink:0;
  background:var(--bg-bar);
  border-bottom:1px solid var(--bd);
  box-shadow:0 1px 0 rgba(255,255,255,.02) inset, 0 4px 12px -8px rgba(0,0,0,.6);
}
.sp-brand{
  display:flex;align-items:center;gap:7px;font-weight:700;font-size:12px;
  letter-spacing:.4px;color:var(--tx);flex-shrink:0;user-select:none;
}
.sp-logo{
  width:18px;height:18px;border-radius:5px;
  background:linear-gradient(135deg,#818cf8 0%,#6366f1 50%,#4338ca 100%);
  box-shadow:0 0 0 1px rgba(255,255,255,.12) inset, 0 2px 8px var(--ac-glow);
  display:flex;align-items:center;justify-content:center;
  font-size:10px;font-weight:900;color:#fff;
}
.sp-live{
  display:inline-flex;align-items:center;gap:5px;
  font-size:9.5px;font-weight:700;letter-spacing:.7px;
  color:var(--live);text-transform:uppercase;
  padding:3px 8px;border-radius:10px;
  background:rgba(16,185,129,.08);border:1px solid rgba(16,185,129,.18);
  flex-shrink:0;
}
.sp-live::before{
  content:'';width:6px;height:6px;border-radius:50%;background:var(--live);
  box-shadow:0 0 6px var(--live-glow);
  animation:sp-pulse 2s ease-in-out infinite;
}
@keyframes sp-pulse{0%,100%{opacity:1}50%{opacity:.4}}
.sp-name{
  flex:1;min-width:0;font-size:11.5px;color:var(--tx-mu);font-weight:500;
  overflow:hidden;text-overflow:ellipsis;white-space:nowrap;
  font-family:'SF Mono','Monaco','Consolas',monospace;
}
.sp-act{display:flex;gap:4px;flex-shrink:0}
.sp-btn{
  display:inline-flex;align-items:center;gap:5px;
  background:transparent;color:var(--tx-mu);
  border:1px solid var(--bd-hi);border-radius:6px;
  padding:5px 10px;font-size:11px;font-weight:600;
  cursor:pointer;transition:all .15s;line-height:1;
  font-family:inherit;
}
.sp-btn:hover{
  color:var(--tx);background:rgba(129,140,248,.08);
  border-color:var(--ac);box-shadow:0 0 0 3px rgba(129,140,248,.08);
}
.sp-btn:active{transform:translateY(1px)}
.sp-btn svg{flex-shrink:0}

/* ── Pager ────────────────────────────────────────────────────────────── */
.sp-pager{
  display:flex;align-items:center;gap:2px;flex-shrink:0;
  background:rgba(255,255,255,.025);border:1px solid var(--bd-hi);
  border-radius:7px;padding:2px;
}
.sp-pg-btn{
  display:inline-flex;align-items:center;justify-content:center;
  width:24px;height:24px;background:transparent;color:var(--tx-mu);
  border:none;border-radius:5px;cursor:pointer;transition:all .15s;
}
.sp-pg-btn:hover:not(:disabled){background:rgba(129,140,248,.15);color:var(--ac)}
.sp-pg-btn:disabled{opacity:.25;cursor:default}
.sp-counter{
  font-size:10.5px;color:var(--tx-mu);font-weight:600;
  padding:0 8px;min-width:42px;text-align:center;letter-spacing:.3px;
  font-variant-numeric:tabular-nums;
}
.sp-counter b{color:var(--tx);font-weight:700}

/* ── Chart frame ─────────────────────────────────────────────────────── */
.sp-frame{
  flex:1;min-height:0;position:relative;
  background:var(--frame);
  background-image:
    linear-gradient(rgba(255,255,255,.015) 1px, transparent 1px),
    linear-gradient(90deg, rgba(255,255,255,.015) 1px, transparent 1px);
  background-size:24px 24px;
  overflow:hidden;
}
.sp-frame iframe{
  width:100%;height:100%;border:none;background:transparent;display:block;
}
</style></head><body>
<div class="sp-wrap">
  <div class="sp-bar">
    <div class="sp-brand"><span class="sp-logo">S</span><span>SeraPlot</span></div>
    <span class="sp-live">Live</span>
    <span class="sp-name">${name}</span>
    ${pagerHtml}
    <div class="sp-act">
      <button class="sp-btn" onclick="v.postMessage({type:'refresh'})" title="Re-run script (Ctrl+R)">
        <svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="13.5,2.5 13.5,6.5 9.5,6.5"/><path d="M13.5 6.5A6 6 0 1 0 14 11"/></svg>
        Refresh
      </button>
      <button class="sp-btn" onclick="v.postMessage({type:'open'})" title="Reveal source file">
        <svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2.5 2.5h7l4 4v7a1 1 0 0 1-1 1h-10a1 1 0 0 1-1-1v-10a1 1 0 0 1 1-1z"/><polyline points="9.5,2.5 9.5,6.5 13.5,6.5"/></svg>
        Source
      </button>
      <button class="sp-btn" onclick="v.postMessage({type:'switchMode'})" title="Cycle preview mode (panel \u2192 tab \u2192 window)">
        <svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="3" width="12" height="10" rx="1.5"/><line x1="2" y1="7" x2="14" y2="7"/></svg>
        Mode
      </button>
      <button class="sp-btn" onclick="v.postMessage({type:'float'})" title="Detach preview into a floating native window">
        <svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="1" y="2" width="9" height="7" rx="1"/><rect x="6" y="7" width="9" height="7" rx="1"/></svg>
        Float
      </button>
    </div>
  </div>
  <div class="sp-frame"><iframe id="frm" srcdoc="${firstSrcdoc}" sandbox="allow-scripts"></iframe></div>
</div>
<script>const v=acquireVsCodeApi();
document.addEventListener('keydown',e=>{if((e.ctrlKey||e.metaKey)&&e.key==='r'){e.preventDefault();v.postMessage({type:'refresh'});}});
${pagerScript}</script>
</body></html>`;
}
function noChartHtml(file) {
    const name = htmlEscape(path.basename(file));
    return `<!DOCTYPE html><html><head><meta charset="utf-8"><style>
*{box-sizing:border-box}
html,body{margin:0;padding:0;height:100vh;background:#0b0e14;color:#e2e8f0;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;display:flex;align-items:center;justify-content:center}
.card{max-width:480px;padding:36px;text-align:center;background:linear-gradient(180deg,#161b26 0%,#11151e 100%);border:1px solid #1f2733;border-radius:14px;box-shadow:0 20px 60px -20px rgba(0,0,0,.6)}
.icon{width:56px;height:56px;margin:0 auto 18px;border-radius:14px;background:linear-gradient(135deg,#475569 0%,#334155 100%);display:flex;align-items:center;justify-content:center;color:#94a3b8}
h2{margin:0 0 8px;font-size:17px;font-weight:700;color:#e2e8f0;letter-spacing:-.2px}
p{margin:6px 0;font-size:13px;color:#7e8a9c;line-height:1.55}
code{background:rgba(129,140,248,.10);color:#a5b4fc;padding:2px 7px;border-radius:4px;font-family:'SF Mono','Consolas',monospace;font-size:12px}
.fname{display:inline-block;margin-top:4px;padding:3px 9px;background:rgba(255,255,255,.03);border:1px solid #1f2733;border-radius:5px;font-family:'SF Mono','Consolas',monospace;font-size:11px;color:#94a3b8}
button{margin-top:22px;background:linear-gradient(135deg,#6366f1,#4338ca);color:#fff;border:none;padding:10px 22px;border-radius:8px;cursor:pointer;font-weight:600;font-size:12px;letter-spacing:.2px;box-shadow:0 4px 14px -2px rgba(99,102,241,.4);transition:all .15s;font-family:inherit}
button:hover{transform:translateY(-1px);box-shadow:0 6px 20px -2px rgba(99,102,241,.55)}
button:active{transform:translateY(0)}
</style></head><body>
<div class="card">
  <div class="icon"><svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 3v18h18"/><path d="M7 14l4-4 4 4 5-5"/></svg></div>
  <h2>No chart in this script</h2>
  <p>The file ran successfully, but didn't create any <code>sp.Chart</code>.</p>
  <p>Add <code>chart = sp.bar(...)</code> or <code>chart.save("out.html")</code> to see it here.</p>
  <p><span class="fname">${name}</span></p>
  <button onclick="acquireVsCodeApi().postMessage({type:'refresh'})">Run again</button>
</div></body></html>`;
}
function errorHtml(msg, file) {
    const safe = htmlEscape(msg);
    const name = htmlEscape(path.basename(file));
    return `<!DOCTYPE html><html><head><meta charset="utf-8"><style>
*{box-sizing:border-box}
html,body{margin:0;padding:0;min-height:100vh;background:#0b0e14;color:#e2e8f0;font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;font-size:13px}
.wrap{padding:24px;max-width:920px;margin:0 auto}
.head{display:flex;align-items:center;gap:10px;margin-bottom:18px}
.badge{display:inline-flex;align-items:center;gap:6px;background:rgba(244,63,94,.10);color:#fb7185;border:1px solid rgba(244,63,94,.25);padding:5px 11px;border-radius:7px;font-size:11px;font-weight:700;letter-spacing:.4px;text-transform:uppercase}
.fname{font-family:'SF Mono','Consolas',monospace;font-size:12px;color:#94a3b8;background:rgba(255,255,255,.03);padding:5px 10px;border-radius:6px;border:1px solid #1f2733}
pre{background:#0d1219;border:1px solid #1f2733;border-left:3px solid #f43f5e;padding:16px 18px;border-radius:8px;white-space:pre-wrap;line-height:1.6;color:#fda4af;font-family:'SF Mono','Monaco','Consolas',monospace;font-size:12px;overflow-x:auto;box-shadow:0 4px 12px -4px rgba(0,0,0,.4)}
button{margin-top:18px;background:linear-gradient(135deg,#6366f1,#4338ca);color:#fff;border:none;padding:9px 20px;border-radius:7px;cursor:pointer;font-weight:600;font-size:12px;box-shadow:0 4px 14px -2px rgba(99,102,241,.35);transition:all .15s;font-family:inherit;display:inline-flex;align-items:center;gap:6px}
button:hover{transform:translateY(-1px);box-shadow:0 6px 18px -2px rgba(99,102,241,.5)}
button:active{transform:translateY(0)}
</style></head><body>
<div class="wrap">
  <div class="head">
    <span class="badge">\u26a0 Error</span>
    <span class="fname">${name}</span>
  </div>
  <pre>${safe}</pre>
  <button onclick="acquireVsCodeApi().postMessage({type:'refresh'})">
    <svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="13.5,2.5 13.5,6.5 9.5,6.5"/><path d="M13.5 6.5A6 6 0 1 0 14 11"/></svg>
    Retry
  </button>
</div></body></html>`;
}
function buildChartCallSnippet(id, label, ctx, kwargParts, dataKwargs, params, chainTail) {
    const sp = ctx.importAlias;
    const tail = kwargParts.length ? ", " + kwargParts.join(", ") : "";
    const sigParams = params && params.length ? params : ["labels", "values"];
    const dataParts = [];
    const lblSrc = (dataKwargs && dataKwargs.labels) || ctx.labelsVar || `["A","B","C","D","E"]`;
    const valSrc = (dataKwargs && dataKwargs.values) || ctx.valuesVar || `[10, 20, 15, 25, 18]`;
    const numList = `[10, 20, 15, 25, 18]`;
    const matList = `[[1,2,3],[4,5,6]]`;
    for (const p of sigParams) {
        if (dataKwargs && dataKwargs[p]) {
            dataParts.push(`${p}=${dataKwargs[p]}`);
            continue;
        }
        if (p === "labels" || p === "x_labels" || p === "category_labels" || p === "categories" || p === "axes" || p === "words" || p === "dates" || p === "parents") {
            dataParts.push(`${p}=${lblSrc}`);
        }
        else if (p === "values" || p === "weights" || p === "frequencies") {
            dataParts.push(`${p}=${valSrc}`);
        }
        else if (p === "x" || p === "y" || p === "z" || p === "x_values" || p === "y_values" || p === "z_values" || p === "lats" || p === "lons" || p === "sizes" || p === "opens" || p === "highs" || p === "lows" || p === "closes" || p === "values_start" || p === "values_end" || p === "values_left" || p === "values_right" || p === "overlay_values") {
            dataParts.push(`${p}=${numList}`);
        }
        else if (p === "value") {
            dataParts.push(`${p}=42`);
        }
        else if (p === "series" || p === "series_values" || p === "flat_matrix") {
            dataParts.push(`${p}=${matList}`);
        }
        else {
            dataParts.push(`${p}=${valSrc}`);
        }
    }
    return `chart = ${sp}.${id}("${label}", ${dataParts.join(", ")}${tail})${chainTail || ""}`;
}
function buildMlModelSnippet(model, ctx, kwargParts) {
    const sp = ctx.importAlias;
    const args = kwargParts.length ? kwargParts.join(", ") : "";
    return [
        `model = ${sp}.${model.id}(${args})`,
        `model.fit(X, y)`,
        `pred = model.predict(X)`
    ].join("\n");
}
function buildMlFnSnippet(fn, ctx, kwargParts) {
    const sp = ctx.importAlias;
    const isCluster = fn.task === "clustering";
    const dictParts = isCluster
        ? [`    "x": xs`, `    "y": ys`]
        : [`    "x": X`, `    "y": y`];
    for (const k of kwargParts) {
        const m = k.match(/^(\w+)=(.*)$/);
        if (m)
            dictParts.push(`    ${JSON.stringify(m[1])}: ${m[2]}`);
    }
    const printer = isCluster ? `print("labels:", result.get("labels"))` : `print("score:", result.get("score"))`;
    return `result = ${sp}.${fn.id}({\n${dictParts.join(",\n")}\n})\n${printer}`;
}
async function insertChart(entry, context) {
    const cat = await (0, runtimeCatalog_1.loadCatalog)(context);
    let chosen = entry;
    if (!chosen) {
        const items = cat.charts.map(c => ({ label: c.label, description: `sp.${c.id}`, detail: c.group, _e: c }));
        const pick = await vscode.window.showQuickPick(items, { placeHolder: "Choose a chart to insert" });
        if (!pick)
            return;
        chosen = pick._e;
    }
    let ed = vscode.window.activeTextEditor;
    if (!ed || ed.document.languageId !== "python") {
        if (lastPyEditor && !lastPyEditor.document.isClosed) {
            ed = await vscode.window.showTextDocument(lastPyEditor.document, lastPyEditor.viewColumn || vscode.ViewColumn.One, false);
        }
        else {
            const visiblePy = vscode.window.visibleTextEditors.find(e => e.document.languageId === "python");
            if (visiblePy) {
                ed = await vscode.window.showTextDocument(visiblePy.document, visiblePy.viewColumn || vscode.ViewColumn.One, false);
            }
            else {
                vscode.window.showWarningMessage("SeraPlot: open a Python file first.");
                return;
            }
        }
    }
    const ctx = scanContext(ed.document);
    await ensureImport(ed, ctx);
    const variantParts = chosen.variant ? [`variant="${chosen.variant}"`] : [];
    const body = buildChartCallSnippet(chosen.id, chosen.label, ctx, variantParts, undefined, chosen.params);
    const block = `\n${body}\nchart.save("${chosen.id}.html")\n`;
    const pos = ed.selection.active;
    const startCol = ed.document.lineAt(pos.line).text.length;
    await ed.edit(eb => eb.insert(new vscode.Position(pos.line, startCol), block));
    if (vscode.workspace.getConfiguration("seraplot").get("autoOpenPreview", true) && !previewPanel) {
        openPreview(context);
    }
}
async function insertConfig() {
    const ed = vscode.window.activeTextEditor;
    if (!ed) {
        vscode.window.showWarningMessage("SeraPlot: open an editor first.");
        return;
    }
    const cfg = vscode.workspace.getConfiguration("seraplot");
    const palette = cfg.get("defaultPalette", ["#6366F1", "#F43F5E", "#10B981", "#F59E0B", "#8B5CF6", "#06B6D4"]);
    const bg = cfg.get("defaultBackground", "#0f172a");
    const palHex = palette.map(c => "0x" + c.replace(/^#/, "").toUpperCase()).join(", ");
    const ctx = scanContext(ed.document);
    await ensureImport(ed, ctx);
    const body = `\n${ctx.importAlias}.config(\n    background=${JSON.stringify(bg)},\n    palette=[${palHex}],\n    animation=True,\n    crosshair=True,\n    tooltip=True\n)\n`;
    const pos = ed.selection.active;
    const startCol = ed.document.lineAt(pos.line).text.length;
    await ed.edit(eb => eb.insert(new vscode.Position(pos.line, startCol), body));
}
async function newDemo() {
    const content = `import seraplot as sp\n\nsp.set_auto_display(False)\n\nchart = sp.bar("Demo", labels=["A","B","C","D"], values=[10, 20, 15, 25], background="#0f172a")\nchart.save("demo.html")\n`;
    const doc = await vscode.workspace.openTextDocument({ language: "python", content });
    await vscode.window.showTextDocument(doc);
}
async function exportHtml() {
    const ed = vscode.window.activeTextEditor;
    if (!ed) {
        vscode.window.showWarningMessage("SeraPlot: open a Python file first.");
        return;
    }
    const file = ed.document.fileName;
    const out = await vscode.window.showSaveDialog({ filters: { HTML: ["html"] }, defaultUri: vscode.Uri.file(path.join(path.dirname(file), path.basename(file, path.extname(file)) + ".html")) });
    if (!out)
        return;
    const py = vscode.workspace.getConfiguration("seraplot").get("pythonPath", "python");
    const escFile = file.replace(/\\/g, "\\\\");
    const escOut = out.fsPath.replace(/\\/g, "\\\\");
    const code = [
        "import runpy, seraplot as sp",
        "_captured = []",
        "_orig = sp.Chart.save",
        "def _spy(self, p, *a, **k):",
        "    _captured.append(self.html); return _orig(self, p, *a, **k)",
        "sp.Chart.save = _spy",
        "sp.set_auto_display(False)",
        "_g = runpy.run_path(r'" + escFile + "', run_name='__main__')",
        "for _v in _g.values():",
        "    if isinstance(_v, sp.Chart): _captured.append(_v.html)",
        "if _captured: open(r'" + escOut + "', 'w', encoding='utf-8').write(_captured[-1])"
    ].join("\n");
    cp.execFile(py, ["-c", code], { timeout: 30000 }, (err) => {
        if (err) {
            vscode.window.showErrorMessage("Export failed: " + err.message);
            return;
        }
        vscode.window.showInformationMessage("SeraPlot: exported to " + out.fsPath);
    });
}
function autoInstallSeraplot() {
    const py = vscode.workspace.getConfiguration("seraplot").get("pythonPath", "python");
    cp.execFile(py, ["-m", "pip", "install", "-q", "seraplot"], (err) => {
        if (!err)
            return;
        vscode.window.showErrorMessage("Failed to auto-install seraplot: " + err.message);
    });
}
function checkPyPIUpdate() {
    const py = vscode.workspace.getConfiguration("seraplot").get("pythonPath", "python");
    cp.execFile(py, ["-c", "import seraplot; print(seraplot.__version__)"], { timeout: 5000 }, (err, stdout) => {
        const current = err ? null : String(stdout).trim();
        const https = require("https");
        const req = https.get("https://pypi.org/pypi/seraplot/json", { headers: { "User-Agent": "seraplot-vscode" } }, (res) => {
            let raw = "";
            res.on("data", (d) => { raw += d; });
            res.on("end", () => {
                try {
                    const latest = JSON.parse(raw)?.info?.version;
                    if (!latest || latest === current)
                        return;
                    cp.execFile(py, ["-m", "pip", "install", "-q", "--upgrade", "seraplot"], { timeout: 120000 }, (e2) => {
                        if (e2)
                            return;
                        (0, runtimeCatalog_1.invalidateCatalog)(extCtx);
                        (0, runtimeCatalog_1.loadCatalog)(extCtx, { force: true }).then(cat => {
                            if (galleryProvider)
                                galleryProvider.refresh();
                            if (homeProvider)
                                homeProvider.setVersion(cat.version, cat.charts.length);
                            refreshWheelStatus();
                        });
                    });
                }
                catch { }
            });
        });
        req.on("error", () => { });
        req.end();
    });
}
function uninstallSeraplot() {
    const py = vscode.workspace.getConfiguration("seraplot").get("pythonPath", "python");
    vscode.window.showQuickPick(["Yes", "No"], { placeHolder: "Uninstall seraplot?" }).then(choice => {
        if (choice !== "Yes")
            return;
        cp.execFile(py, ["-m", "pip", "uninstall", "-y", "-q", "seraplot"], (err) => {
            if (err)
                vscode.window.showErrorMessage("Failed to uninstall seraplot: " + err.message);
            else
                vscode.window.showInformationMessage("SeraPlot uninstalled.");
        });
    });
}
class GalleryItem extends vscode.TreeItem {
    constructor(payload) {
        super("", vscode.TreeItemCollapsibleState.None);
        this.payload = payload;
        if (payload.kind === "group") {
            this.label = `${payload.group}  \u00b7  ${payload.children.length}`;
            this.collapsibleState = vscode.TreeItemCollapsibleState.Collapsed;
            this.contextValue = "group";
        }
        else if (payload.kind === "chart") {
            const c = payload.chart;
            this.label = c.label;
            this.description = `sp.${c.id}`;
            this.tooltip = `${c.label} (${c.group}) \u2014 click to insert`;
            this.contextValue = "chart";
            if (payload.hasVariants) {
                this.collapsibleState = vscode.TreeItemCollapsibleState.Collapsed;
            }
            this.command = { command: "seraplot.insertChart", title: "Insert", arguments: [c] };
        }
        else {
            const vk = payload.variantKey;
            this.label = vk[0].toUpperCase() + vk.slice(1).replace(/_/g, " ");
            this.description = `variant="${vk}"`;
            this.tooltip = `Insert ${payload.chart.id}(..., variant="${vk}")`;
            this.contextValue = "variant";
            const entry = { ...payload.chart, variant: vk };
            this.command = { command: "seraplot.insertChart", title: "Insert", arguments: [entry] };
        }
    }
}
class GalleryProvider {
    constructor() {
        this._emitter = new vscode.EventEmitter();
        this.onDidChangeTreeData = this._emitter.event;
    }
    refresh() { this._emitter.fire(); }
    getTreeItem(el) { return el; }
    getChildren(el) {
        const cat = (0, runtimeCatalog_1.getCachedCatalog)(extCtx);
        if (!cat)
            return [new GalleryItem({ kind: "group", group: "Loading\u2026", children: [] })];
        if (!el) {
            const groups = new Map();
            for (const c of cat.charts) {
                if (!groups.has(c.group))
                    groups.set(c.group, []);
                groups.get(c.group).push(c);
            }
            const out = [];
            for (const [g, list] of groups)
                out.push(new GalleryItem({ kind: "group", group: g, children: list }));
            return out;
        }
        if (el.payload.kind === "group") {
            return el.payload.children.map(c => {
                const hasVariants = !!(cat.chart_variants && cat.chart_variants[c.id] && cat.chart_variants[c.id].variants.length > 0);
                return new GalleryItem({ kind: "chart", chart: c, hasVariants });
            });
        }
        if (el.payload.kind === "chart" && el.payload.hasVariants) {
            const p = el.payload;
            const cv = cat.chart_variants[p.chart.id];
            return cv.variants.map(v => new GalleryItem({ kind: "variant", chart: p.chart, variantKey: v.key }));
        }
        return [];
    }
}
class RecentProvider {
    constructor(ctx) {
        this.ctx = ctx;
        this._emitter = new vscode.EventEmitter();
        this.onDidChangeTreeData = this._emitter.event;
    }
    add(file) {
        const list = this.ctx.globalState.get(RECENT_KEY, []);
        const next = [file, ...list.filter(f => f !== file)].slice(0, 12);
        this.ctx.globalState.update(RECENT_KEY, next);
        this._emitter.fire();
    }
    getTreeItem(el) { return el; }
    getChildren() {
        const list = this.ctx.globalState.get(RECENT_KEY, []);
        if (list.length === 0)
            return [new vscode.TreeItem("No recent previews")];
        return list.map(f => {
            const it = new vscode.TreeItem(path.basename(f));
            it.description = path.dirname(f);
            it.tooltip = f;
            it.command = { command: "vscode.open", title: "Open", arguments: [vscode.Uri.file(f)] };
            return it;
        });
    }
}
class HomeProvider {
    constructor(ctx) {
        this.ctx = ctx;
    }
    resolveWebviewView(view) {
        this._view = view;
        view.webview.options = { enableScripts: true, localResourceRoots: [vscode.Uri.file(this.ctx.extensionPath)] };
        view.webview.html = this.html(view.webview);
        view.webview.onDidReceiveMessage(msg => {
            if (msg && msg.cmd)
                vscode.commands.executeCommand(msg.cmd);
        });
        view.onDidDispose(() => { this._view = undefined; });
        const cat = (0, runtimeCatalog_1.getCachedCatalog)(this.ctx);
        if (cat)
            this.setVersion(cat.version, cat.charts.length);
    }
    setVersion(version, chartCount) {
        if (this._view)
            this._view.webview.postMessage({ type: "version", version, chartCount });
    }
    html(webview) {
        const nonce = makeNonce();
        const csp = `default-src 'none'; style-src ${webview.cspSource} 'unsafe-inline'; script-src 'nonce-${nonce}'; img-src ${webview.cspSource} data: https:;`;
        return `<!DOCTYPE html><html><head><meta charset="utf-8">
<meta http-equiv="Content-Security-Policy" content="${csp}">
<style>
*{box-sizing:border-box}
body{margin:0;padding:10px;font-family:var(--vscode-font-family,system-ui,sans-serif);font-size:12px;color:var(--vscode-sideBar-foreground,#cccccc);background:var(--vscode-sideBar-background,#1e1e1e)}
.title{display:flex;align-items:center;gap:6px;font-size:12px;font-weight:600;margin:0 0 10px}
.dot{width:8px;height:8px;border-radius:50%;background:linear-gradient(135deg,#6366f1,#a78bfa);box-shadow:0 0 6px rgba(99,102,241,.6);flex-shrink:0}
.ver{font-size:10px;color:#6366f1;background:rgba(99,102,241,.12);border-radius:4px;padding:1px 5px;font-weight:500;display:none}
.btn{display:block;width:100%;padding:7px 10px;margin-bottom:5px;background:var(--vscode-button-secondaryBackground,#2d2d2d);color:var(--vscode-button-secondaryForeground,#cccccc);border:1px solid var(--vscode-panel-border,#3c3c3c);border-radius:5px;cursor:pointer;font-size:12px;text-align:left;font-family:inherit}
.btn:hover{background:var(--vscode-button-secondaryHoverBackground,#3a3a3a);border-color:#6366f1}
.btn.primary{background:linear-gradient(135deg,#6366f1,#8b5cf6);color:#fff;border:1px solid transparent;font-weight:600}
.lbl{font-size:9px;text-transform:uppercase;letter-spacing:.6px;color:var(--vscode-descriptionForeground,#888);margin:9px 0 4px;font-weight:600}
.hint{font-size:11px;color:var(--vscode-descriptionForeground,#888);margin-top:10px;line-height:1.45;padding:7px 9px;border-radius:5px;background:rgba(99,102,241,.08);border-left:2px solid #6366f1}
</style></head><body>
<div class="title"><span class="dot"></span><span>SeraPlot</span><span class="ver" id="ver"></span></div>
<div class="lbl">Live preview</div>
<button class="btn primary" data-cmd="seraplot.preview">Open Live Preview</button>
<button class="btn" data-cmd="seraplot.refreshPreview">Refresh Preview</button>
<div class="lbl">Build</div>
<button class="btn primary" data-cmd="seraplot.chartStudio">Chart Studio</button>
<button class="btn" data-cmd="seraplot.insertChart">Insert Chart\u2026</button>
<button class="btn" data-cmd="seraplot.insertConfig">Insert sp.config(\u2026)</button>
<button class="btn" data-cmd="seraplot.newDemo">New Demo File</button>
<div class="lbl">Tools</div>
<button class="btn" data-cmd="seraplot.themeStudio">Theme Studio</button>
<button class="btn" data-cmd="seraplot.gallery">Open Gallery</button>
<button class="btn" data-cmd="seraplot.exportHtml">Export as HTML</button>
<button class="btn" data-cmd="seraplot.refreshCatalog">Refresh Framework Catalog</button>
<div class="lbl">Manage</div>
<button class="btn" data-cmd="seraplot.autoInstall">Install / Update SeraPlot</button>
<button class="btn" data-cmd="seraplot.uninstall">Uninstall SeraPlot</button>
<div class="lbl">Help</div>
<button class="btn" data-cmd="seraplot.openDocs">Documentation</button>
<div class="hint" id="hint">Catalog auto-discovered from installed seraplot. Charts and ML models appear automatically.</div>
<script nonce="${nonce}">
(function(){
  const v = acquireVsCodeApi();
  document.querySelectorAll('button[data-cmd]').forEach(function(b){
    b.addEventListener('click', function(){ v.postMessage({cmd: b.getAttribute('data-cmd')}); });
  });
  window.addEventListener('message', function(e){
    const m = e.data;
    if(m && m.type === 'version'){
      const el = document.getElementById('ver');
      if(el){ el.textContent = 'v' + m.version; el.style.display = 'inline'; }
      const h = document.getElementById('hint');
      if(h) h.textContent = m.chartCount + ' charts \u00b7 seraplot v' + m.version + ' \u2014 auto-updated from PyPI.';
    }
  });
})();
</script>
</body></html>`;
    }
}
async function openGallery(context) {
    const cat = await (0, runtimeCatalog_1.loadCatalog)(context);
    const panel = vscode.window.createWebviewPanel("seraplotGallery", "SeraPlot Gallery", vscode.ViewColumn.Active, { enableScripts: true });
    try {
        panel.iconPath = vscode.Uri.file(path.join(context.extensionPath, "logo.png"));
    }
    catch (_e) { }
    panel.webview.html = galleryHtml(cat);
    panel.webview.onDidReceiveMessage(async (msg) => {
        if (!msg)
            return;
        if (msg.type === "insert" && msg.id) {
            const entry = cat.charts.find(c => c.id === msg.id);
            if (entry) {
                const e = msg.variant ? { ...entry, variant: msg.variant } : entry;
                await vscode.commands.executeCommand("seraplot.insertChart", e);
            }
        }
        if (msg.type === "studio")
            await vscode.commands.executeCommand("seraplot.chartStudio");
    });
}
function galleryHtml(cat) {
    const groups = new Map();
    for (const c of cat.charts) {
        if (!groups.has(c.group))
            groups.set(c.group, []);
        groups.get(c.group).push(c);
    }
    const sections = [];
    for (const [g, list] of groups) {
        const cards = list.map(c => {
            const cv = cat.chart_variants && cat.chart_variants[c.id];
            const variantBtns = cv && cv.variants.length > 0
                ? `<div class="variants">${cv.variants.map(v => `<button class="vbtn" onclick="insv('${c.id}','${v.key}')" title="${v.key}">${v.key[0].toUpperCase() + v.key.slice(1).replace(/_/g, " ")}</button>`).join("")}</div>`
                : "";
            return `<div class="card"><div class="cn" onclick="ins('${c.id}')">${htmlEscape(c.label)}</div><code onclick="ins('${c.id}')">sp.${c.id}(...)</code>${variantBtns}</div>`;
        }).join("");
        sections.push(`<h3>${htmlEscape(g)}<span class="count">${list.length}</span></h3><div class="grid">${cards}</div>`);
    }
    const mlByCat = new Map();
    for (const m of cat.ml_models) {
        if (!mlByCat.has(m.category))
            mlByCat.set(m.category, []);
        mlByCat.get(m.category).push({ id: m.id, label: m.label, kind: "class" });
    }
    if (cat.ml_functions.length) {
        const arr = cat.ml_functions.map(m => ({ id: m.id, label: m.label, kind: "function" }));
        mlByCat.set("ml_dict (one-shot)", arr);
    }
    if (cat.ml_transformers && cat.ml_transformers.length) {
        for (const t of cat.ml_transformers) {
            const key = "transformers \u00b7 " + t.category;
            if (!mlByCat.has(key))
                mlByCat.set(key, []);
            mlByCat.get(key).push({ id: t.id, label: t.label, kind: "class" });
        }
    }
    for (const [c, list] of mlByCat) {
        const cards = list.map(m => `<div class="card ml" onclick="stu()"><div class="cn">${htmlEscape(m.label)}</div><code>sp.${m.id}</code></div>`).join("");
        sections.push(`<h3>ML \u00b7 ${htmlEscape(c)}<span class="count">${list.length}</span></h3><div class="grid">${cards}</div>`);
    }
    const total = cat.charts.length + cat.ml_models.length + cat.ml_functions.length + (cat.ml_transformers ? cat.ml_transformers.length : 0);
    return `<!DOCTYPE html><html><head><meta charset="utf-8"><style>
body{font-family:system-ui,sans-serif;background:#0d1117;color:#e6e6e6;padding:24px;margin:0}
h1{margin:0 0 4px;font-size:24px;background:linear-gradient(90deg,#6366f1,#a78bfa);-webkit-background-clip:text;background-clip:text;color:transparent}
.sub{color:#94a3b8;font-size:13px;margin-bottom:24px}
h3{margin-top:28px;margin-bottom:10px;color:#c084fc;font-weight:600;font-size:14px;letter-spacing:.5px;text-transform:uppercase;display:flex;align-items:center;gap:10px}
.count{color:#64748b;font-size:11px;font-weight:400;background:#1e293b;padding:2px 8px;border-radius:10px}
.grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(200px,1fr));gap:8px}
.card{background:#161b22;border:1px solid #30363d;border-radius:8px;padding:12px;transition:all .15s}
.card:hover{border-color:#6366f1;background:#1c2230}
.card.ml{border-left:3px solid #a78bfa}
.cn{font-weight:600;font-size:13px;margin-bottom:4px;cursor:pointer}
.cn:hover{color:#a78bfa}
code{font-family:Consolas,monospace;font-size:11px;color:#94a3b8;cursor:pointer;display:block;margin-bottom:6px}
.variants{display:flex;flex-wrap:wrap;gap:4px;margin-top:6px}
.vbtn{background:#1e293b;border:1px solid #2d3748;color:#94a3b8;border-radius:4px;padding:2px 7px;font-size:10px;cursor:pointer;font-family:inherit}
.vbtn:hover{background:#6366f1;color:#fff;border-color:#6366f1}
.act{display:flex;gap:8px;margin-bottom:18px}
.act button{background:linear-gradient(135deg,#6366f1,#8b5cf6);color:#fff;border:none;padding:10px 18px;border-radius:6px;cursor:pointer;font-weight:600;font-size:13px;font-family:inherit}
input{background:#161b22;border:1px solid #30363d;color:#e6e6e6;padding:9px 14px;border-radius:6px;width:100%;box-sizing:border-box;margin-bottom:18px;font-size:13px}
</style></head><body>
<h1>SeraPlot Gallery</h1>
<div class="sub">${total} entries auto-discovered from seraplot ${htmlEscape(cat.version)} \u00b7 click a chart to insert \u00b7 click variant badge to insert with variant</div>
<div class="act"><button onclick="stu()">Open Chart Studio</button></div>
<input id="q" placeholder="Filter\u2026" oninput="flt()">
<div id="all">${sections.join("")}</div>
<script>
const v=acquireVsCodeApi();
function ins(id){v.postMessage({type:'insert',id})}
function insv(id,vk){v.postMessage({type:'insert',id,variant:vk})}
function stu(){v.postMessage({type:'studio'})}
function flt(){
  const q=document.getElementById('q').value.toLowerCase();
  document.querySelectorAll('.card').forEach(c=>{c.style.display=c.textContent.toLowerCase().includes(q)?'':'none'});
  document.querySelectorAll('h3').forEach(h=>{const sib=h.nextElementSibling;const any=Array.from(sib.children).some(c=>c.style.display!=='none');h.style.display=any?'':'none';sib.style.display=any?'':'none'});
}
</script>
</body></html>`;
}
function sysmonFetchHtml(python) {
    return new Promise(resolve => {
        const script = "import seraplot as sp; chart = sp.sysmon(); print(chart.html)";
        cp.exec(`"${python}" -c "${script}"`, { timeout: 25000 }, (e, stdout, stderr) => {
            if (e || !stdout.trim()) {
                resolve({ html: "", err: (stderr || e?.message || "No output").slice(0, 900) });
            }
            else {
                resolve({ html: stdout.trim() });
            }
        });
    });
}
function sysmonShellHtml(inner) {
    const esc = inner.replace(/`/g, "\\`").replace(/\$/g, "\\$");
    return `<!DOCTYPE html><html><head><meta charset="UTF-8">
<style>*{box-sizing:border-box;margin:0;padding:0}html,body{height:100%;background:#08090e;font-family:system-ui,Arial,sans-serif}
#toolbar{display:flex;align-items:center;gap:10px;padding:8px 14px;background:#0d1117;border-bottom:1px solid #1e293b}
#toolbar span{color:#94a3b8;font-size:12px;font-weight:600;letter-spacing:.04em;flex:1}
button{background:#1e293b;color:#e2e8f0;border:1px solid #334155;border-radius:6px;padding:5px 12px;font-size:12px;cursor:pointer;display:flex;align-items:center;gap:5px}
button:hover{background:#334155}
button.active{background:#4f46e5;border-color:#6366f1;color:#fff}
#frame{width:100%;height:calc(100vh - 41px);border:none;display:block}
#loading{position:absolute;inset:0;top:41px;display:flex;align-items:center;justify-content:center;background:#08090e;color:#475569;font-size:14px;pointer-events:none;transition:opacity .3s}
</style></head>
<body>
<div id="toolbar">
  <span>&#x1F4BB; System Monitor</span>
  <button id="btnRefresh" onclick="refresh()">&#x21BB; Refresh</button>
</div>
<div id="loading">Loading system data...</div>
<iframe id="frame" srcdoc="" frameborder="0" sandbox="allow-scripts"></iframe>
<script>
const vscode = acquireVsCodeApi();
const frame = document.getElementById('frame');
const loading = document.getElementById('loading');
let html = \`${esc}\`;
function show(h) {
  frame.srcdoc = h;
  loading.style.opacity = '0';
  setTimeout(() => { loading.style.display = 'none'; }, 350);
}
function refresh() {
  loading.style.display = 'flex';
  loading.style.opacity = '1';
  loading.textContent = 'Refreshing...';
  vscode.postMessage({ type: 'refresh' });
}
show(html);
window.addEventListener('message', ev => {
  if (ev.data && ev.data.type === 'update') show(ev.data.html);
  if (ev.data && ev.data.type === 'error') {
    loading.style.display = 'flex';
    loading.style.opacity = '1';
    loading.textContent = 'Error: ' + ev.data.msg;
  }
});
</script>
</body></html>`;
}
function sysmonErrorHtml(msg) {
    return `<!DOCTYPE html><html><body style="background:#0d1117;color:#f87171;font-family:system-ui;padding:24px">
<h3 style="margin-bottom:10px">SeraPlot sysmon failed</h3>
<pre style="font-size:12px;color:#fca5a5;white-space:pre-wrap">${htmlEscape(msg)}</pre>
<p style="color:#94a3b8;font-size:12px;margin-top:14px">Requires seraplot &ge; 2.4.48. Check your Python path in Settings.</p>
</body></html>`;
}
function openSysmon(context) {
    const panel = vscode.window.createWebviewPanel("seraplotSysmon", "System Monitor", vscode.ViewColumn.Active, { enableScripts: true, retainContextWhenHidden: true });
    try {
        panel.iconPath = vscode.Uri.file(path.join(context.extensionPath, "logo.png"));
    }
    catch (_e) { }
    const python = vscode.workspace.getConfiguration("seraplot").get("pythonPath", "python");
    panel.webview.html = sysmonShellHtml("Loading...");
    sysmonFetchHtml(python).then(res => {
        if (!res.err && res.html) {
            panel.webview.html = sysmonShellHtml(res.html);
        }
        else {
            panel.webview.html = sysmonErrorHtml(res.err ?? "unknown error");
        }
    });
    panel.webview.onDidReceiveMessage(msg => {
        if (msg?.type === "refresh") {
            sysmonFetchHtml(python).then(res => {
                if (!res.err && res.html) {
                    panel.webview.postMessage({ type: "update", html: res.html });
                }
                else {
                    panel.webview.postMessage({ type: "error", msg: res.err ?? "unknown error" });
                }
            });
        }
    });
}
const METHOD_GROUPS = [
    {
        group: "Interactivity",
        methods: [
            { id: "zoom", label: "Zoom", code: ".zoom()" },
            { id: "crosshair", label: "Crosshair", code: ".crosshair()" },
            { id: "export_button", label: "Export Button", code: ".export_button()" },
        ]
    },
    {
        group: "Visual",
        methods: [
            { id: "animate", label: "Animate", code: ".animate()" },
            { id: "responsive", label: "Responsive", code: ".responsive()" },
            { id: "no_axes", label: "No Axes", code: ".no_axes()" },
            { id: "no_title", label: "No Title", code: ".no_title()" },
            { id: "no_legend", label: "No Legend", code: ".no_legend()" },
            { id: "show_grid", label: "Show Grid", code: ".show_grid()" },
            { id: "text_auto", label: "Text Auto", code: ".text_auto(True)" },
            { id: "csp_safe", label: "CSP Safe", code: ".csp_safe()" },
        ]
    },
    {
        group: "Layout",
        methods: [
            { id: "flip", label: "Flip / Horizontal", code: ".flip()" },
            { id: "rotate_90", label: "Rotate 90°", code: ".rotate(90)" },
            { id: "sort_desc", label: "Sort Desc", code: ".sort_by('desc')" },
            { id: "sort_asc", label: "Sort Asc", code: ".sort_by('asc')" },
            { id: "legend_right", label: "Legend Right", code: ".legend('right')" },
            { id: "legend_bottom", label: "Legend Bottom", code: ".legend('bottom')" },
            { id: "legend_none", label: "Legend None", code: ".legend('none')" },
        ]
    },
    {
        group: "Style",
        methods: [
            { id: "border_radius", label: "Border Radius", code: ".border_radius(12)" },
            { id: "corner_bars", label: "Corner Bars", code: ".corner_radius_bars(6)" },
            { id: "bar_gap", label: "Bar Gap 0.3", code: ".bar_gap(0.3)" },
            { id: "opacity_80", label: "Opacity 80%", code: ".set_opacity(0.8)" },
            { id: "downsample", label: "Downsample", code: ".downsample(2000)" },
        ]
    },
];
function methodToggleHtml() {
    const groupsJson = JSON.stringify(METHOD_GROUPS);
    return `<!DOCTYPE html><html><head><meta charset="UTF-8">
<style>
*{box-sizing:border-box;margin:0;padding:0}
html,body{background:#08090e;color:#e2e8f0;font-family:system-ui,Arial,sans-serif;height:100%;overflow:auto}
#header{padding:14px 16px 10px;border-bottom:1px solid #1e293b}
#header h1{font-size:15px;font-weight:700;color:#f1f5f9;margin-bottom:3px}
#header p{font-size:11px;color:#64748b}
.group{padding:10px 14px 0}
.glabel{font-size:10px;font-weight:700;text-transform:uppercase;letter-spacing:.08em;color:#475569;margin-bottom:8px}
.methods{display:flex;flex-wrap:wrap;gap:6px;margin-bottom:12px}
.toggle{display:flex;align-items:center;gap:7px;background:#0d1117;border:1px solid #1e293b;border-radius:7px;padding:5px 10px;cursor:pointer;user-select:none;font-size:12px;color:#94a3b8;transition:all .15s}
.toggle:hover{border-color:#334155;color:#e2e8f0}
.toggle.on{background:#1e1b4b;border-color:#6366f1;color:#a5b4fc}
.toggle .dot{width:8px;height:8px;border-radius:50%;background:#1e293b;transition:background .15s;flex-shrink:0}
.toggle.on .dot{background:#6366f1}
#footer{position:sticky;bottom:0;background:#0d1117;border-top:1px solid #1e293b;padding:10px 14px;display:flex;gap:8px;align-items:center}
#preview{flex:1;font-family:Consolas,monospace;font-size:11px;color:#a5d6a7;white-space:pre-wrap;word-break:break-all;min-height:32px;background:#0b1018;border:1px solid #1e293b;border-radius:6px;padding:7px 10px;max-height:80px;overflow:auto}
button{background:#4f46e5;color:#fff;border:none;border-radius:6px;padding:7px 14px;font-size:12px;cursor:pointer;font-weight:600;flex-shrink:0}
button:hover{background:#6366f1}
button.ghost{background:#1e293b;color:#94a3b8;border:1px solid #334155}
button.ghost:hover{background:#334155;color:#e2e8f0}
#varname{background:#0d1117;border:1px solid #334155;color:#e2e8f0;padding:5px 8px;border-radius:5px;font-size:12px;font-family:Consolas,monospace;width:100px}
</style>
</head><body>
<div id="header">
  <h1>&#x26A1; Method Toggle</h1>
  <p>Toggle methods ON/OFF then insert the chain at cursor</p>
</div>
<div id="groups"></div>
<div id="footer">
  <input id="varname" type="text" value="chart" placeholder="var name">
  <div id="preview"></div>
  <button class="ghost" onclick="clearAll()">Clear</button>
  <button onclick="insert()">Insert</button>
</div>
<script>
const vscode = acquireVsCodeApi();
const GROUPS = ${groupsJson};
const active = new Set();
function build() {
  const c = document.getElementById('groups');
  c.innerHTML = '';
  GROUPS.forEach(g => {
    const d = document.createElement('div'); d.className = 'group';
    const lbl = document.createElement('div'); lbl.className = 'glabel'; lbl.textContent = g.group; d.appendChild(lbl);
    const row = document.createElement('div'); row.className = 'methods';
    g.methods.forEach(m => {
      const el = document.createElement('div'); el.className = 'toggle' + (active.has(m.id) ? ' on' : '');
      el.innerHTML = '<span class="dot"></span>' + m.label;
      el.onclick = () => { if (active.has(m.id)) active.delete(m.id); else active.add(m.id); refresh(); };
      row.appendChild(el);
    });
    d.appendChild(row); c.appendChild(d);
  });
}
function getChain() {
  const v = document.getElementById('varname').value.trim() || 'chart';
  const all = GROUPS.flatMap(g => g.methods);
  const parts = all.filter(m => active.has(m.id)).map(m => m.code);
  if (!parts.length) return v;
  return v + parts.join('');
}
function refresh() { build(); document.getElementById('preview').textContent = getChain(); }
function clearAll() { active.clear(); refresh(); }
function insert() { vscode.postMessage({ type: 'insert', code: getChain() }); }
document.getElementById('varname').addEventListener('input', () => { document.getElementById('preview').textContent = getChain(); });
build();
</script>
</body></html>`;
}
function openMethodToggle(context) {
    const panel = vscode.window.createWebviewPanel("seraplotMethodToggle", "Method Toggle", vscode.ViewColumn.Beside, { enableScripts: true, retainContextWhenHidden: true });
    try {
        panel.iconPath = vscode.Uri.file(path.join(context.extensionPath, "logo.png"));
    }
    catch (_e) { }
    panel.webview.html = methodToggleHtml();
    panel.webview.onDidReceiveMessage(async (msg) => {
        if (msg?.type !== "insert" || typeof msg.code !== "string")
            return;
        const ed = lastPyEditor ?? vscode.window.activeTextEditor;
        if (!ed) {
            vscode.window.showWarningMessage("SeraPlot: open a Python file first.");
            return;
        }
        const pos = ed.selection.active;
        const line = ed.document.lineAt(pos.line).text;
        await ed.edit(eb => eb.insert(new vscode.Position(pos.line, line.length), "\n" + msg.code));
        await vscode.window.showTextDocument(ed.document, { viewColumn: ed.viewColumn, preserveFocus: true });
        vscode.window.setStatusBarMessage("SeraPlot: inserted " + msg.code.split(".").length + " method(s)", 2500);
    });
}
function openThemeStudio(context) {
    const panel = vscode.window.createWebviewPanel("seraplotThemeStudio", "SeraPlot Theme Studio", vscode.ViewColumn.Active, { enableScripts: true });
    try {
        panel.iconPath = vscode.Uri.file(path.join(context.extensionPath, "logo.png"));
    }
    catch (_e) { }
    const cfg = vscode.workspace.getConfiguration("seraplot");
    const palette = cfg.get("defaultPalette", ["#6366F1", "#F43F5E", "#10B981", "#F59E0B", "#8B5CF6", "#06B6D4"]);
    const bg = cfg.get("defaultBackground", "#0f172a");
    panel.webview.html = themeStudioHtml(palette, bg);
    panel.webview.onDidReceiveMessage(async (msg) => {
        if (msg.type === "copy" && typeof msg.code === "string") {
            await vscode.env.clipboard.writeText(msg.code);
            vscode.window.showInformationMessage("SeraPlot config copied.");
        }
        if (msg.type === "insert" && typeof msg.code === "string") {
            const ed = vscode.window.activeTextEditor;
            if (!ed) {
                vscode.window.showWarningMessage("SeraPlot: open an editor.");
                return;
            }
            const ctx = scanContext(ed.document);
            await ensureImport(ed, ctx);
            const pos = ed.selection.active;
            await ed.edit(eb => eb.insert(new vscode.Position(pos.line, ed.document.lineAt(pos.line).text.length), "\n" + msg.code + "\n"));
        }
        if (msg.type === "saveDefaults" && msg.palette && msg.bg) {
            await vscode.workspace.getConfiguration("seraplot").update("defaultPalette", msg.palette, vscode.ConfigurationTarget.Global);
            await vscode.workspace.getConfiguration("seraplot").update("defaultBackground", msg.bg, vscode.ConfigurationTarget.Global);
            vscode.window.showInformationMessage("SeraPlot defaults saved.");
        }
    });
}
function themeStudioHtml(initialPalette, initialBg) {
    const palJson = JSON.stringify(initialPalette);
    return `<!DOCTYPE html><html><head><meta charset="utf-8"><style>
body{font-family:system-ui,sans-serif;background:#0d1117;color:#e6e6e6;padding:24px;margin:0;max-width:1100px}
h1{margin:0 0 4px;font-size:24px;background:linear-gradient(90deg,#6366f1,#a78bfa);-webkit-background-clip:text;background-clip:text;color:transparent}
.sub{color:#94a3b8;font-size:13px;margin-bottom:24px}
.layout{display:grid;grid-template-columns:1fr 360px;gap:20px}
.panel{background:#161b22;border:1px solid #30363d;border-radius:10px;padding:18px}
h2{margin:0 0 14px;font-size:13px;text-transform:uppercase;letter-spacing:.5px;color:#c084fc}
.row{display:flex;gap:10px;align-items:center;margin-bottom:10px}
.row label{flex:1;font-size:13px;color:#cbd5e1}
.row input[type=text],.row input[type=number]{background:#0d1117;border:1px solid #30363d;color:#e6e6e6;padding:6px 10px;border-radius:5px;font-family:Consolas,monospace;font-size:12px;width:120px}
.row input[type=color]{width:40px;height:32px;border:none;background:none;cursor:pointer;padding:0}
.palette{display:flex;gap:6px;margin:10px 0;flex-wrap:wrap}
.swatch{position:relative;width:42px;height:42px;border-radius:8px;border:2px solid #30363d;cursor:pointer}
.swatch input{position:absolute;inset:0;width:100%;height:100%;opacity:0;cursor:pointer}
.add,.del{width:42px;height:42px;border-radius:8px;border:1px dashed #30363d;background:transparent;color:#64748b;cursor:pointer;font-size:18px}
.preview{display:flex;gap:4px;margin:14px 0;height:100px;border-radius:8px;padding:8px}
.preview .bar{flex:1;border-radius:4px}
button.primary{background:linear-gradient(135deg,#6366f1,#8b5cf6);color:#fff;border:none;padding:10px 18px;border-radius:6px;cursor:pointer;font-weight:600;margin-right:8px}
button.ghost{background:#21262d;color:#e6e6e6;border:1px solid #30363d;padding:9px 16px;border-radius:6px;cursor:pointer}
pre{background:#0b1018;border:1px solid #30363d;padding:14px;border-radius:8px;font-family:Consolas,monospace;font-size:12px;line-height:1.5;color:#a5d6a7;white-space:pre-wrap}
</style></head><body>
<h1>SeraPlot Theme Studio</h1>
<div class="sub">Tune your palette and global config, copy or insert into your file.</div>
<div class="layout">
<div class="panel">
  <h2>Background</h2>
  <div class="row"><input type="color" id="bg" value="${initialBg}"><input type="text" id="bgHex" value="${initialBg}"></div>
  <h2>Palette</h2>
  <div class="palette" id="palette"></div>
  <div class="preview" id="preview"></div>
</div>
<div class="panel">
  <h2>Output</h2>
  <pre id="out"></pre>
  <div style="margin-top:14px"><button class="primary" id="copy">Copy</button><button class="ghost" id="insert">Insert at cursor</button><button class="ghost" id="save" style="float:right">Save defaults</button></div>
</div>
</div>
<script>
const vscode = acquireVsCodeApi();
const palette = ${palJson};
const palDiv = document.getElementById('palette');
const preview = document.getElementById('preview');
const out = document.getElementById('out');
const bg = document.getElementById('bg');
const bgHex = document.getElementById('bgHex');
function paletteUI(){
  palDiv.innerHTML='';
  palette.forEach((c,i)=>{
    const w = document.createElement('div'); w.className='swatch'; w.style.background = c;
    const inp = document.createElement('input'); inp.type='color'; inp.value=c;
    inp.oninput = ()=>{ palette[i]=inp.value; w.style.background=inp.value; render(); };
    w.appendChild(inp); palDiv.appendChild(w);
  });
  if (palette.length > 1) { const del = document.createElement('button'); del.className='del'; del.textContent='-';
    del.onclick = ()=>{ palette.pop(); paletteUI(); render(); }; palDiv.appendChild(del); }
  if (palette.length < 12) { const add = document.createElement('button'); add.className='add'; add.textContent='+';
    add.onclick = ()=>{ palette.push('#888888'); paletteUI(); render(); }; palDiv.appendChild(add); }
}
function render(){
  preview.innerHTML=''; preview.style.background = bg.value;
  palette.forEach(c => { const b = document.createElement('div'); b.className='bar'; b.style.background=c; preview.appendChild(b); });
  bgHex.value = bg.value;
  const hex = palette.map(c=>'0x'+c.replace(/^#/,'').toUpperCase()).join(', ');
  out.textContent = 'sp.config(\\n    background=' + JSON.stringify(bg.value) + ',\\n    palette=[' + hex + '],\\n)';
}
bg.oninput = render;
bgHex.oninput = ()=>{ if(/^#[0-9a-f]{6}$/i.test(bgHex.value)){ bg.value=bgHex.value; render(); } };
document.getElementById('copy').onclick = ()=> vscode.postMessage({type:'copy', code: out.textContent});
document.getElementById('insert').onclick = ()=> vscode.postMessage({type:'insert', code: out.textContent});
document.getElementById('save').onclick = ()=> vscode.postMessage({type:'saveDefaults', palette, bg: bg.value});
paletteUI(); render();
</script>
</body></html>`;
}
function chartStem(id) {
    let s = id.toLowerCase();
    s = s.replace(/^build_/, "");
    s = s.replace(/_chart$/, "");
    s = s.replace(/_family$/, "");
    s = s.replace(/_unified$/, "");
    return s;
}
function dedupCharts(charts) {
    const byStem = new Map();
    for (const c of charts) {
        const k = chartStem(c.id);
        if (!byStem.has(k))
            byStem.set(k, []);
        byStem.get(k).push(c);
    }
    for (const [k, group] of byStem) {
        if (group.length === 1)
            continue;
        const k2 = k.endsWith("s") ? k.slice(0, -1) : null;
        if (k2 && byStem.has(k2)) {
            byStem.get(k2).push(...group);
            byStem.delete(k);
        }
    }
    const canonical = [];
    const variants = {};
    for (const group of byStem.values()) {
        group.sort((a, b) => a.id.length - b.id.length || a.id.localeCompare(b.id));
        const head = group[0];
        canonical.push(head);
        const rest = group.slice(1).map(g => g.id);
        if (rest.length)
            variants[head.id] = rest;
    }
    canonical.sort((a, b) => a.label.localeCompare(b.label));
    return { canonical, variants };
}
function buildStudioCatalog(cat) {
    const out = [];
    const { canonical, variants } = dedupCharts(cat.charts);
    const universal = [
        { key: "sort_order", kind: "enum", default: "none", options: ["none", "asc", "desc", "alpha", "alpha_desc"], hint: "Sort categories" },
        { key: "x_label", kind: "string", default: "", hint: "X axis label" },
        { key: "y_label", kind: "string", default: "", hint: "Y axis label" },
        { key: "gridlines", kind: "bool", default: false, hint: "Show gridlines" },
        { key: "legend_position", kind: "enum", default: "right", options: ["right", "left", "top", "bottom"], hint: "Legend placement" },
        { key: "background", kind: "color_str", default: "", hint: "Background color (hex/CSS)" },
        { key: "width", kind: "int", default: 900, hint: "Canvas width (px)" },
        { key: "height", kind: "int", default: 500, hint: "Canvas height (px)" },
        { key: "stroke_width", kind: "float", default: 1.4, hint: "Outline thickness" },
        { key: "fill_opacity", kind: "float", default: 0.55, hint: "Body fill opacity (0-1)" },
        { key: "show_points", kind: "bool", default: false, hint: "Overlay individual points" },
        { key: "show_values", kind: "bool", default: false, hint: "Show numeric value labels" },
        { key: "filled", kind: "bool", default: false, hint: "Fill area under curve" },
        { key: "orientation", kind: "enum", default: "vertical", options: ["vertical", "horizontal"], hint: "Plot orientation" },
    ];
    const merge = (base) => {
        const seen = new Set((base || []).map(k => k.key));
        const extra = universal.filter(u => !seen.has(u.key));
        return [...(base || []), ...extra];
    };
    for (const c of canonical)
        out.push({ kind: "chart", id: c.id, label: c.label, group: c.group, signature: c.signature, params: c.params, kwargs: merge(c.kwargs) });
    for (const m of cat.ml_models)
        out.push({ kind: "ml_class", id: m.id, label: m.label, group: "ML \u00b7 " + m.category, signature: m.signature, kwargs: m.kwargs });
    for (const m of cat.ml_functions)
        out.push({ kind: "ml_fn", id: m.id, label: m.label, group: "ML \u00b7 ml_dict (" + m.task + ")", signature: m.signature, task: m.task, kwargs: m.kwargs });
    if (cat.ml_transformers) {
        for (const t of cat.ml_transformers)
            out.push({ kind: "ml_class", id: t.id, label: t.label, group: "ML \u00b7 transformers (" + t.category + ")", signature: t.signature, kwargs: t.kwargs });
    }
    return { entries: out, mergedVariants: variants };
}
function openWasmPlayground(context) {
    const panel = vscode.window.createWebviewPanel("seraplotWasmPlayground", "SeraPlot \u2014 WASM Playground", { viewColumn: vscode.ViewColumn.Beside, preserveFocus: false }, { enableScripts: true, retainContextWhenHidden: true, localResourceRoots: [vscode.Uri.file(path.join(context.extensionPath, "media"))] });
    try { panel.iconPath = vscode.Uri.file(path.join(context.extensionPath, "logo.png")); } catch (_e) {}
    const wasmJsUri = panel.webview.asWebviewUri(vscode.Uri.file(path.join(context.extensionPath, "media", "seraplot-web.js")));
    const wasmBinUri = panel.webview.asWebviewUri(vscode.Uri.file(path.join(context.extensionPath, "media", "seraplot_bg.wasm")));
    const monacoBase = "https://cdn.jsdelivr.net/npm/monaco-editor@0.45.0/min/vs";
    const ed = vscode.window.activeTextEditor;
    const initCode = (ed && ed.document.languageId === "python") ? ed.document.getText() : 'import seraplot as sp\n\nsp.bar(\n    labels=["A","B","C","D"],\n    values=[10, 25, 18, 32],\n    title="Hello SeraPlot"\n)\n';
    panel.webview.html = wasmPlaygroundHtml(wasmJsUri.toString(), wasmBinUri.toString(), monacoBase, initCode);
    panel.webview.onDidReceiveMessage(msg => {
        if (msg && msg.type === "insert" && msg.code) {
            const activeEd = vscode.window.activeTextEditor;
            if (activeEd && activeEd.document.languageId === "python") {
                activeEd.edit(eb => { eb.replace(activeEd.selection, msg.code); });
            }
        }
    });
}
function wasmPlaygroundHtml(wasmJsUri, wasmBinUri, monacoBase, initCode) {
    const escaped = initCode.replace(/\\/g, "\\\\").replace(/`/g, "\\`").replace(/\$/g, "\\$");
    return `<!DOCTYPE html><html><head><meta charset="utf-8">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'unsafe-inline' 'unsafe-eval' 'wasm-unsafe-eval' https://cdn.jsdelivr.net ${wasmJsUri.split('/').slice(0,-1).join('/')}/ vscode-webview-resource: https://file+.vscode-resource.vscode-cdn.net; style-src 'unsafe-inline'; img-src data: blob: https:; connect-src blob: data: vscode-webview-resource: https:; worker-src blob: data:;">
<style>
*{box-sizing:border-box;margin:0;padding:0}
body{font-family:system-ui,sans-serif;background:#0d1117;color:#e6e6e6;height:100vh;overflow:hidden;display:flex;flex-direction:column}
.hdr{display:flex;align-items:center;gap:10px;padding:8px 14px;background:#161b22;border-bottom:1px solid #30363d;flex-shrink:0}
.hdr-title{font-size:13px;font-weight:700;color:#818cf8;letter-spacing:.04em}
.hdr-sub{font-size:11px;color:#64748b}
.conn{display:flex;align-items:center;gap:5px;margin-left:auto;font-size:11px;color:#94a3b8}
.dot{width:8px;height:8px;border-radius:50%;background:#f59e0b;flex-shrink:0}
.dot.ready{background:#10b981}
.dot.err{background:#ef4444}
.main{display:flex;flex:1;min-height:0;overflow:hidden}
.ecol{display:flex;flex-direction:column;width:50%;min-width:180px;border-right:1px solid #30363d;overflow:hidden}
.ehdr{background:#1e2432;padding:4px 10px;font-size:11px;color:#818cf8;letter-spacing:.08em;font-weight:700;border-bottom:1px solid #30363d;flex-shrink:0}
.ebody{flex:1;position:relative;overflow:hidden}
.pcol{display:flex;flex-direction:column;flex:1;min-width:0;overflow:hidden;background:#0d1117}
.phdr{background:#1e2432;padding:4px 10px;font-size:11px;color:#818cf8;letter-spacing:.08em;font-weight:700;border-bottom:1px solid #30363d;flex-shrink:0}
.pbody{flex:1;position:relative;overflow:hidden}
iframe{width:100%;height:100%;border:none;display:block}
.loader{position:absolute;inset:0;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:10px;background:#0d1117;font-size:12px;color:#94a3b8;z-index:10}
.spinner{width:28px;height:28px;border:3px solid #334155;border-top-color:#6366f1;border-radius:50%;animation:spin .8s linear infinite}
@keyframes spin{to{transform:rotate(360deg)}}
.err-bar{position:absolute;bottom:0;left:0;right:0;background:#450a0a;border-top:1px solid #ef4444;color:#fca5a5;padding:6px 12px;font-size:11px;font-family:'Cascadia Code','Consolas',monospace;white-space:pre-wrap;max-height:100px;overflow-y:auto;z-index:20;display:none}
.divider{width:5px;cursor:col-resize;background:#161b22;flex-shrink:0;transition:background .15s}
.divider:hover{background:#6366f1}
</style></head><body>
<div class="hdr">
  <div class="hdr-title">SeraPlot · WASM Playground</div>
  <div class="hdr-sub">in-browser · no Python needed</div>
  <div class="conn"><div class="dot" id="dot"></div><span id="dotTxt">Loading</span></div>
</div>
<div class="main">
  <div class="ecol" id="ecol">
    <div class="ehdr">playground.py</div>
    <div class="ebody"><div id="monacoHost" style="position:absolute;inset:0"></div></div>
  </div>
  <div class="divider" id="divider"></div>
  <div class="pcol">
    <div class="phdr">Live Preview</div>
    <div class="pbody">
      <iframe id="iframe" sandbox="allow-scripts allow-same-origin"></iframe>
      <div class="loader" id="loader"><div class="spinner"></div><div>Loading WASM…</div></div>
      <div class="err-bar" id="errBar"></div>
    </div>
  </div>
</div>
<script src="${wasmJsUri}"></script>
<script>
var WASM_BIN_URI = ${JSON.stringify(wasmBinUri)};
var MONACO_BASE = ${JSON.stringify(monacoBase)};
var INIT_CODE = \`${escaped}\`;
var editor, wasmReady = false, debTimer, lastSent, wasmAliases = null;

function buildDynamicAliases(sp) {
    if (typeof sp.chartAliases === 'function') {
        try { return JSON.parse(sp.chartAliases()); } catch(e) {}
    }
    var aliases = {};
    var keys = Object.keys(sp);
    for (var i = 0; i < keys.length; i++) {
        var k = keys[i];
        if (typeof sp[k] !== 'function') continue;
        var snake = k.replace(/[A-Z]/g, function(c) { return '_' + c.toLowerCase(); });
        var base = snake.indexOf('build_') === 0 ? snake.slice(6) : snake;
        var stripped = base.length > 6 && base.slice(-6) === '_chart' ? base.slice(0, -6) : base;
        aliases[base] = k;
        if (stripped !== base) aliases[stripped] = k;
        if (base.slice(-1) !== 's') aliases[base + 's'] = k;
        if (stripped !== base && stripped.slice(-1) !== 's') aliases[stripped + 's'] = k;
    }
    return aliases;
}

function fnCandidates(fn) {
    var camel = fn.split('_').map(function(s,i){ return i===0?s:s.charAt(0).toUpperCase()+s.slice(1); }).join('');
    var base = camel.charAt(0).toUpperCase()+camel.slice(1);
    var out = [];
    if (camel.indexOf('build')===0) out.push(camel);
    else { out.push('build'+base); out.push('build'+base+'Chart'); out.push(camel); }
    out.push('build'+base+'Chart');
    var seen={}, dedup=[];
    for (var i=0;i<out.length;i++) { if (!seen[out[i]]) { seen[out[i]]=1; dedup.push(out[i]); } }
    return dedup;
}

function resolveWasmFn(fn) {
    var sp = window.SeraplotWASM;
    if (!sp) return null;
    if (wasmAliases && wasmAliases[fn] && typeof sp[wasmAliases[fn]] === 'function') return { name: wasmAliases[fn], fn: sp[wasmAliases[fn]] };
    var cands = fnCandidates(fn);
    for (var i=0;i<cands.length;i++) { if (typeof sp[cands[i]]==='function') return { name:cands[i], fn:sp[cands[i]] }; }
    if (fn.slice(-1) === 's') {
        var singular = fn.slice(0, -1);
        var cands2 = fnCandidates(singular);
        for (var j=0;j<cands2.length;j++) { if (typeof sp[cands2[j]]==='function') return { name:cands2[j], fn:sp[cands2[j]] }; }
    }
    return null;
}

function parsePyVal(s) {
    s=s.trim(); if(!s.length) return null;
    if(s[0]==='"'||s[0]==="'"){var q=s[0],i=1,r='';while(i<s.length&&s[i]!==q){if(s[i]==='\\\\')i++;r+=s[i++];}return r;}
    if(s[0]==='['){var out=[],depth=0,buf='',inS=false,q2='',i2=1;while(i2<s.length){var c=s[i2];if(inS){if(c===q2)inS=false;buf+=c;}else{if(c==='"'||c==="'"){inS=true;q2=c;buf+=c;}else if(c==='['){{depth++;buf+=c;}}else if(c===']'){if(depth===0){if(buf.trim())out.push(parsePyVal(buf.trim()));break;}depth--;buf+=c;}else if(c===','&&depth===0){if(buf.trim())out.push(parsePyVal(buf.trim()));buf='';}else buf+=c;}i2++;}return out;}
    if(s==='True')return true; if(s==='False')return false; if(s==='None'||s==='null')return null;
    var n=Number(s); return isNaN(n)?s:n;
}

function parsePyArgs(body) {
    var args=[],kwargs={},i=0,depth=0,buf='',inStr=false,q='';
    while(i<body.length){var c=body[i];if(inStr){if(c==='\\\\'&&i+1<body.length){buf+=c+body[i+1];i+=2;continue;}if(c===q)inStr=false;buf+=c;}else{if(c==='"'||c==="'"){inStr=true;q=c;buf+=c;}else if(c==='('||c==='['||c==='{'){depth++;buf+=c;}else if(c===')'||c===']'||c==='}'){depth--;buf+=c;}else if(c===','&&depth===0){pushArg(buf,args,kwargs);buf='';}else buf+=c;}i++;}
    if(buf.trim().length) pushArg(buf,args,kwargs);
    return {args:args,kwargs:kwargs};
}

function pushArg(raw,args,kwargs){var s=raw.trim();if(!s.length)return;var eq=-1,depth=0,inStr=false,q='';for(var i=0;i<s.length;i++){var c=s[i];if(inStr){if(c===q)inStr=false;}else{if(c==='"'||c==="'"){inStr=true;q=c;}else if(c==='('||c==='['||c==='{')depth++;else if(c===')'||c===']'||c==='}')depth--;else if(c==='='&&depth===0){eq=i;break;}}}if(eq===-1)args.push(parsePyVal(s));else{var k=s.slice(0,eq).trim(),v=parsePyVal(s.slice(eq+1).trim());kwargs[k]=v;}}

function extractAllCalls(code) {
    var calls=[],i=0;
    while(i<code.length){var idx=code.indexOf('sp.',i);if(idx===-1)break;var rest=code.slice(idx);var m=rest.match(/^sp\.([a-zA-Z_][a-zA-Z0-9_]*)\\s*\\(/);if(!m){i=idx+3;continue;}var bodyStart=idx+m[0].length,depth=1,j=bodyStart,inStr=false,q='';while(j<code.length&&depth>0){var c=code[j];if(inStr){if(c==='\\\\')j++;if(code[j]===q)inStr=false;}else{if(c==='"'||c==="'"){inStr=true;q=c;}else if(c==='(')depth++;else if(c===')')depth--;}j++;}calls.push({fn:m[1],body:code.slice(bodyStart,j-1)});i=j;}
    return calls;
}

function renderHtml(html) {
    var ifr=document.getElementById('iframe');
    if(/<\\/head>/i.test(html)) html=html.replace(/<\\/head>/i,'<style>body{margin:0}</style></head>');
    else html='<style>body{margin:0}</style>'+html;
    var blob=new Blob([html],{type:'text/html'});
    var url=URL.createObjectURL(blob);
    var prev=ifr.dataset.blobUrl;
    ifr.src=url; ifr.dataset.blobUrl=url;
    if(prev) setTimeout(function(){URL.revokeObjectURL(prev);},2000);
}

function showErr(msg){var b=document.getElementById('errBar');b.style.display='block';b.textContent=msg;}
function hideErr(){document.getElementById('errBar').style.display='none';}

function runOnce(force) {
    if(!editor) return;
    var code=editor.getValue();
    if(!force&&code===lastSent) return;
    lastSent=code;
    var calls=extractAllCalls(code);
    if(!calls.length){showErr('No sp.<chart>(...) call detected');return;}
    var sp=window.SeraplotWASM;
    if(!sp||!sp.__ready){showErr('WASM not ready');return;}
    var lastHtml='';
    for(var ci=0;ci<calls.length;ci++){
        var call=calls[ci];
        try{
            var entry=resolveWasmFn(call.fn);
            if(!entry){showErr('No WASM entry for sp.'+call.fn+'()');continue;}
            var parsed=parsePyArgs(call.body);
            var kw=parsed.kwargs;
            var html=entry.fn(JSON.stringify(kw));
            if(html&&html.length>32) lastHtml=html;
        } catch(e){showErr('Error: '+e.message);}
    }
    if(lastHtml){hideErr();renderHtml(lastHtml);}
}

function debRun(){if(debTimer)clearTimeout(debTimer);debTimer=setTimeout(function(){runOnce(false);},400);}

function setStatus(kind,txt){var dot=document.getElementById('dot'),t=document.getElementById('dotTxt');dot.className='dot'+(kind==='ready'?' ready':kind==='err'?' err':'');t.textContent=txt;}

function initMonaco(){
    var loader=document.createElement('script');
    loader.src=MONACO_BASE+'/loader.js';
    loader.onload=function(){
        window.require.config({paths:{vs:MONACO_BASE}});
        window.MonacoEnvironment={getWorkerUrl:function(){return 'data:text/javascript;charset=utf-8,';} };
        window.require(['vs/editor/editor.main'],function(){
            editor=window.monaco.editor.create(document.getElementById('monacoHost'),{
                value:INIT_CODE,language:'python',theme:'vs-dark',automaticLayout:true,
                fontFamily:'"Cascadia Code","JetBrains Mono","Fira Code","Consolas",monospace',
                fontSize:13,lineHeight:21,minimap:{enabled:false},scrollBeyondLastLine:false,
                renderLineHighlight:'all',tabSize:4,insertSpaces:true,wordWrap:'on',
                padding:{top:10,bottom:10},smoothScrolling:true,cursorBlinking:'smooth',
            });
            editor.onDidChangeModelContent(debRun);
            setStatus('loading','Loading WASM');
            var sp=window.SeraplotWASM;
            if(!sp){setStatus('err','WASM script not loaded');return;}
            sp.__init(WASM_BIN_URI).then(function(){
                wasmAliases = buildDynamicAliases(window.SeraplotWASM);
                document.getElementById('loader').style.display='none';
                setStatus('ready','Live · WASM');
                wasmReady=true;
                runOnce(true);
            }).catch(function(e){setStatus('err','WASM failed');document.getElementById('loader').innerHTML='<div style="color:#fca5a5;font-size:12px">WASM load failed: '+(e&&e.message?e.message:e)+'</div>';});
        });
    };
    document.head.appendChild(loader);
}

(function attachDivider(){
    var divider=document.getElementById('divider'),ecol=document.getElementById('ecol');
    var dragging=false,startX=0,startW=0,parentW=0;
    function down(e){dragging=true;startX=e.clientX||(e.touches&&e.touches[0].clientX)||0;startW=ecol.getBoundingClientRect().width;parentW=ecol.parentElement.getBoundingClientRect().width;document.body.style.userSelect='none';e.preventDefault();}
    function move(e){if(!dragging)return;var x=e.clientX||(e.touches&&e.touches[0].clientX)||0;var w=startW+(x-startX);if(w<180)w=180;if(w>parentW-160)w=parentW-160;ecol.style.width=w+'px';if(editor&&editor.layout)editor.layout();}
    function up(){dragging=false;document.body.style.userSelect='';}
    divider.addEventListener('mousedown',down);
    document.addEventListener('mousemove',move);
    document.addEventListener('mouseup',up);
})();

initMonaco();
</script>
</body></html>`;
}
async function openChartStudio(context) {
    const cat = await (0, runtimeCatalog_1.loadCatalog)(context);
    const panel = vscode.window.createWebviewPanel("seraplotChartStudio", "SeraPlot Chart Studio", { viewColumn: vscode.ViewColumn.Beside, preserveFocus: false }, { enableScripts: true, retainContextWhenHidden: true });
    try {
        panel.iconPath = vscode.Uri.file(path.join(context.extensionPath, "logo.png"));
    }
    catch (_e) { }
    const studioCat = buildStudioCatalog(cat);
    const mergedAliases = { ...(cat.aliases || {}) };
    for (const head of Object.keys(studioCat.mergedVariants)) {
        const extra = studioCat.mergedVariants[head];
        const existing = mergedAliases[head] || [];
        const all = Array.from(new Set([...existing, ...extra])).sort();
        mergedAliases[head] = all;
    }
    const ed = vscode.window.activeTextEditor;
    const ctx = ed && ed.document.languageId === "python" ? scanContext(ed.document) : null;
    const payload = {
        version: cat.version,
        catalog: studioCat.entries,
        common_kwargs: cat.common_kwargs,
        chart_methods: cat.chart_methods || [],
        themes: cat.themes || [],
        aliases: mergedAliases,
        variants: cat.chart_variants || {},
        catalog_error: cat.error || null,
        detected_labels: ctx?.labelsVar || null,
        detected_values: ctx?.valuesVar || null,
        detected_labels_lit: ctx?.labelsLit || null,
        detected_values_lit: ctx?.valuesLit || null,
        wheel_path: cat.wheelPath || null,
        source: cat.source || "system",
    };
    panel.webview.html = chartStudioHtml(JSON.stringify(payload));
    panel.webview.onDidReceiveMessage(async (msg) => {
        if (!msg)
            return;
        if (msg.type === "render" && msg.id && msg.kind === "chart") {
            const rawLabels = String(msg.labels || "").trim();
            const rawValues = String(msg.values || "").trim();
            const labelsLit = `["A","B","C","D","E"]`;
            const valuesLit = `[10, 20, 15, 25, 18]`;
            const isLiteral = (s) => !!s && (s.startsWith("[") || s.startsWith("(") || s.startsWith("\"") || s.startsWith("'"));
            const labels = isLiteral(rawLabels) ? rawLabels : labelsLit;
            const values = isLiteral(rawValues) ? rawValues : valuesLit;
            const title = String(msg.title || msg.id);
            const chartEntry = cat.charts.find(c => c.id === msg.id);
            const kwargParts = renderTypedKwargParts(msg.kwargs || {}, chartEntry?.kwargs, cat.common_kwargs);
            const tail = kwargParts.length ? ", " + kwargParts.join(", ") : "";
            const chainTail = renderChainSuffix(msg.chain || [], cat.chart_methods || []);
            const themeLine = msg.theme ? `    sp.theme(${JSON.stringify(String(msg.theme))})` : "";
            const tmpHtml = path.join(os.tmpdir(), `seraplot_studio_${Date.now()}_${Math.floor(Math.random() * 1e6)}.html`);
            const escOut = tmpHtml.replace(/\\/g, "\\\\");
            const py = vscode.workspace.getConfiguration("seraplot").get("pythonPath", "python");
            const codeLines = [
                "import sys, os, json, traceback",
                "try:",
                "    import seraplot as sp",
                "    sp.set_auto_display(False)",
            ];
            if (themeLine)
                codeLines.push(themeLine);
            codeLines.push(`    chart = sp.${msg.id}(${JSON.stringify(title)}, labels=${labels}, values=${values}${tail})${chainTail}`, "    info = json.loads(sp.chart_info(chart))", `    chart.save(r'${escOut}')`, `    sz = os.path.getsize(r'${escOut}') if os.path.exists(r'${escOut}') else 0`, "    sys.stderr.write('META ' + json.dumps({'info': info, 'file_size': sz}))", "except Exception as e:", "    sys.stderr.write('ERR ' + repr(e) + '\\n' + traceback.format_exc())");
            const code = codeLines.join("\n");
            const env = buildSpawnEnv(cat);
            cp.execFile(py, ["-c", code], { timeout: 25000, maxBuffer: 32 * 1024 * 1024, env }, (err, _stdout, stderr) => {
                const errMsg = String(stderr || "");
                let html = "";
                try {
                    html = fs.readFileSync(tmpHtml, "utf-8");
                }
                catch (_e) { }
                try {
                    fs.unlinkSync(tmpHtml);
                }
                catch (_e) { }
                if (errMsg.startsWith("ERR")) {
                    panel.webview.postMessage({ type: "rendered", id: msg.id, error: "Python exception:\n" + errMsg.slice(4) });
                    return;
                }
                let meta = {};
                if (errMsg.startsWith("META ")) {
                    try {
                        meta = JSON.parse(errMsg.slice(5));
                    }
                    catch (_e) { }
                }
                const isEmpty = (!html || html.length < 32) || (meta && meta.info && meta.info.size === 0);
                if (isEmpty) {
                    const explain = meta && meta.info
                        ? `chart_info reports size=${meta.info.size}, has_svg=${meta.info.has_svg}, paths=${meta.info.paths}, rects=${meta.info.rects}, circles=${meta.info.circles}.\n\n` +
                            `The framework call \`sp.${msg.id}(title, labels=..., values=..., ...)\` returned an empty chart. ` +
                            `This means the values you provided don't match what this specific chart expects, OR this chart variant isn't fully implemented in the wheel for these parameters. ` +
                            `Try simpler defaults (labels=["A","B","C"], values=[1,2,3]) or pick a different chart.`
                        : "The framework returned no usable HTML and chart_info was unavailable.";
                    panel.webview.postMessage({ type: "rendered", id: msg.id, error: explain, meta });
                    return;
                }
                panel.webview.postMessage({ type: "rendered", id: msg.id, html, bytes: html.length, meta });
            });
            return;
        }
        if (msg.type === "insert" && msg.id && msg.kind) {
            let target = vscode.window.activeTextEditor;
            if (!target || target.document.languageId !== "python") {
                if (lastPyEditor && !lastPyEditor.document.isClosed) {
                    target = await vscode.window.showTextDocument(lastPyEditor.document, lastPyEditor.viewColumn || vscode.ViewColumn.One, false);
                }
                else {
                    const visiblePy = vscode.window.visibleTextEditors.find(e => e.document.languageId === "python");
                    if (visiblePy) {
                        target = await vscode.window.showTextDocument(visiblePy.document, visiblePy.viewColumn || vscode.ViewColumn.One, false);
                    }
                    else {
                        const doc = await vscode.workspace.openTextDocument({ language: "python", content: "import seraplot as sp\nsp.set_auto_display(False)\n\n" });
                        target = await vscode.window.showTextDocument(doc, { viewColumn: vscode.ViewColumn.One, preserveFocus: false });
                    }
                }
            }
            const ed2 = target;
            if (!ed2)
                return;
            const ctx = scanContext(ed2.document);
            await ensureImport(ed2, ctx);
            const chartEntryIns = cat.charts.find(c => c.id === msg.id);
            const kwargParts = msg.kind === "chart"
                ? renderTypedKwargParts(msg.kwargs || {}, chartEntryIns?.kwargs, cat.common_kwargs)
                : renderKwargParts(msg.kwargs || {}, cat.common_kwargs);
            let block = "";
            if (msg.kind === "chart") {
                const rawLabelsIn = String(msg.labels || "").trim();
                const rawValuesIn = String(msg.values || "").trim();
                const isLiteral = (s) => !!s && (s.startsWith("[") || s.startsWith("(") || s.startsWith("\"") || s.startsWith("'") || s.startsWith("{"));
                const isIdentifier = (s) => /^[A-Za-z_][A-Za-z0-9_]*(?:\s*\.\s*[A-Za-z_][A-Za-z0-9_]*)*(?:\s*\(.*\))?$/.test(s) || /^[A-Za-z_][A-Za-z0-9_]*\s*\[.*\]$/.test(s);
                const isExpr = (s) => isLiteral(s) || isIdentifier(s);
                const labelsExpr = isExpr(rawLabelsIn) ? rawLabelsIn : (ctx.labelsVar || `["A","B","C","D","E"]`);
                const valuesExpr = isExpr(rawValuesIn) ? rawValuesIn : (ctx.valuesVar || `[10, 20, 15, 25, 18]`);
                const labelsIsBoundIdent = isIdentifier(labelsExpr) && !!ctx.labelsVar && labelsExpr === ctx.labelsVar;
                const valuesIsBoundIdent = isIdentifier(valuesExpr) && !!ctx.valuesVar && valuesExpr === ctx.valuesVar;
                const needsLabelsPrelude = !labelsIsBoundIdent && !ctx.labelsVar;
                const needsValuesPrelude = !valuesIsBoundIdent && !ctx.valuesVar;
                const labelsPreludeVal = isLiteral(labelsExpr) ? labelsExpr : `["A","B","C","D","E"]`;
                const valuesPreludeVal = isLiteral(valuesExpr) ? valuesExpr : `[10, 20, 15, 25, 18]`;
                const dataKwargs = { labels: labelsExpr, values: valuesExpr };
                const themeLine = msg.theme ? `sp.theme(${JSON.stringify(String(msg.theme))})\n` : "";
                const chainTail = renderChainSuffix(msg.chain || [], cat.chart_methods || []);
                const body = buildChartCallSnippet(msg.id, String(msg.title || msg.id), ctx, kwargParts, dataKwargs, chartEntryIns?.params, chainTail);
                let dataPrelude = "";
                if (needsLabelsPrelude)
                    dataPrelude += `labels = ${labelsPreludeVal}\n`;
                if (needsValuesPrelude)
                    dataPrelude += `values = ${valuesPreludeVal}\n`;
                block = `\n${dataPrelude}${themeLine}${body}\nchart.save("${msg.id}.html")\n`;
            }
            else if (msg.kind === "ml_class") {
                const m = cat.ml_models.find(x => x.id === msg.id);
                if (!m)
                    return;
                block = `\n${buildMlModelSnippet(m, ctx, kwargParts)}\n`;
            }
            else if (msg.kind === "ml_fn") {
                const m = cat.ml_functions.find(x => x.id === msg.id);
                if (!m)
                    return;
                block = `\n${buildMlFnSnippet(m, ctx, kwargParts)}\n`;
            }
            const pos = ed2.selection.active;
            const startCol = ed2.document.lineAt(pos.line).text.length;
            await ed2.edit(eb => eb.insert(new vscode.Position(pos.line, startCol), block));
            if (msg.kind === "chart" && vscode.workspace.getConfiguration("seraplot").get("autoOpenPreview", true) && !previewPanel) {
                openPreview(context);
            }
        }
    });
}
function chartStudioHtml(payloadJson) {
    return `<!DOCTYPE html><html><head><meta charset="utf-8"><style>
*{box-sizing:border-box}
body{margin:0;font-family:system-ui,sans-serif;background:#0d1117;color:#e6e6e6;height:100vh;overflow:hidden}
body.theme-light{background:#f8fafc;color:#0f172a}
body.theme-light .left,body.theme-light .mid{background:#ffffff;border-color:#e2e8f0}
body.theme-light .search{border-color:#e2e8f0}
body.theme-light .search input,body.theme-light .f input,body.theme-light .f select,body.theme-light .f textarea,body.theme-light .add-kw select,body.theme-light .add-kw input,body.theme-light .chain-grid,body.theme-light .chain-grid button,body.theme-light .chain-item,body.theme-light .chain-item input,body.theme-light .chain-item select,body.theme-light .snippet,body.theme-light .preview-bar,body.theme-light .foot,body.theme-light .aliases,body.theme-light .aliases select,body.theme-light .qchip,body.theme-light .theme-pick button,body.theme-light .btn{background:#f1f5f9;border-color:#cbd5e1;color:#0f172a}
body.theme-light .grp,body.theme-light .head h1{color:#475569}
body.theme-light .head h1{background:linear-gradient(90deg,#4f46e5,#7c3aed);-webkit-background-clip:text;background-clip:text;color:transparent}
body.theme-light .item{color:#0f172a}
body.theme-light .item:hover{background:#e2e8f0}
body.theme-light .item code,body.theme-light .f .hint,body.theme-light .preview-bar,body.theme-light .alias-hint,body.theme-light .alias-lbl{color:#64748b}
body.theme-light .preview-msg{background:#ffffff;color:#475569}
body.theme-light .f{background:#ffffff;border-color:#e2e8f0}
body.theme-light .resizer-v,body.theme-light .resizer-h{background:#e2e8f0}
body.theme-light .resizer-v:hover,body.theme-light .resizer-h:hover{background:#a78bfa}
body.theme-light .panel-bar{background:#f8fafc;border-color:#e2e8f0;color:#475569}
.app{display:flex;flex-direction:row;height:100vh;width:100vw}
.left,.mid{background:#0b1018;display:flex;flex-direction:column;min-height:0;min-width:0;overflow:hidden}
.left{width:260px;flex-shrink:0;border-right:1px solid #30363d}
.mid{flex:1}
.left.collapsed{width:34px!important}
.left.collapsed .search,.left.collapsed .list{display:none}
.resizer-v{width:5px;cursor:col-resize;background:#161b22;flex-shrink:0;transition:background .15s}
.resizer-v:hover{background:#6366f1}
.resizer-h{height:5px;cursor:row-resize;background:#161b22;flex-shrink:0;transition:background .15s}
.resizer-h:hover{background:#6366f1}
.panel-bar{display:flex;align-items:center;gap:6px;padding:4px 8px;background:#0a0e15;border-bottom:1px solid #1f2733;font-size:10px;color:#94a3b8;text-transform:uppercase;letter-spacing:.5px;font-weight:700;user-select:none}
.panel-bar .pb-title{flex:1}
.panel-bar button{background:transparent;border:1px solid #30363d;color:#94a3b8;cursor:pointer;font-size:11px;padding:2px 7px;border-radius:4px;font-family:inherit;line-height:1}
.panel-bar button:hover{border-color:#6366f1;color:#e2e8f0}
.search{padding:10px;border-bottom:1px solid #30363d}
.search input{width:100%;background:#161b22;border:1px solid #30363d;color:#e6e6e6;padding:7px 10px;border-radius:6px;font-size:12px;font-family:inherit}
.search input:focus{outline:none;border-color:#6366f1}
.list{flex:1;overflow-y:auto;padding:6px 4px}
.grp{font-size:10px;text-transform:uppercase;letter-spacing:.6px;color:#94a3b8;padding:10px 10px 4px;font-weight:700}
.item{padding:6px 12px;font-size:12px;cursor:pointer;border-radius:5px;margin:1px 4px;display:flex;justify-content:space-between;align-items:center;gap:6px}
.item:hover{background:#1c2230}
.item.sel{background:linear-gradient(135deg,rgba(99,102,241,.18),rgba(139,92,246,.12));border-left:2px solid #6366f1;padding-left:10px}
.item code{color:#94a3b8;font-size:10.5px;font-family:Consolas,monospace}
.head{padding:14px 18px;border-bottom:1px solid #30363d;display:flex;align-items:center;gap:10px}
.head h1{margin:0;font-size:15px;background:linear-gradient(90deg,#6366f1,#a78bfa);-webkit-background-clip:text;background-clip:text;color:transparent}
.head .tag{font-size:10px;padding:3px 8px;border-radius:10px;background:#1e293b;color:#94a3b8;text-transform:uppercase;font-weight:600}
.head .head-spacer{flex:1}
.head .theme-toggle{background:#161b22;border:1px solid #30363d;color:#cbd5e1;padding:5px 10px;border-radius:6px;cursor:pointer;font-size:11px;font-family:inherit}
.head .theme-toggle:hover{border-color:#6366f1}
.body{flex:1;overflow-y:auto;padding:14px}
.empty{color:#64748b;text-align:center;margin-top:60px;font-size:13px;padding:0 20px;line-height:1.5}
.section{margin-bottom:14px}
.section h2{margin:0 0 8px;font-size:11px;text-transform:uppercase;letter-spacing:.5px;color:#c084fc;font-weight:600;cursor:pointer;user-select:none;display:flex;align-items:center;gap:6px}
.section h2::before{content:'\u25BE';font-size:9px;color:#64748b;transition:transform .15s}
.section.collapsed h2::before{transform:rotate(-90deg)}
.section.collapsed > *:not(h2){display:none}
.f{background:#161b22;border:1px solid #30363d;border-radius:8px;padding:10px 12px;margin-bottom:6px}
.f label{display:block;font-size:11px;color:#cbd5e1;margin-bottom:6px;font-weight:500;font-family:Consolas,monospace}
.f .hint{font-size:10px;color:#64748b;margin-top:4px}
.f input[type=text],.f input[type=number],.f select,.f textarea{width:100%;background:#0b1018;border:1px solid #30363d;color:#e6e6e6;padding:6px 9px;border-radius:5px;font-size:12px;font-family:Consolas,monospace}
.f textarea{resize:vertical;min-height:50px}
.f input:focus,.f select:focus,.f textarea:focus{outline:none;border-color:#6366f1}
.f input[type=color]{width:38px;height:30px;border:none;background:none;cursor:pointer;padding:0}
.f .row{display:flex;align-items:center;gap:8px}
.f .switch{position:relative;width:36px;height:20px;background:#30363d;border-radius:10px;cursor:pointer;flex-shrink:0;transition:background .15s}
.f .switch.on{background:#6366f1}
.f .switch::after{content:'';position:absolute;width:14px;height:14px;background:#fff;border-radius:50%;top:3px;left:3px;transition:left .15s}
.f .switch.on::after{left:19px}
.f .x{cursor:pointer;color:#64748b;background:none;border:none;font-size:14px;font-family:inherit;padding:0 6px}
.f .x:hover{color:#f43f5e}
.add-kw{display:flex;gap:6px}
.add-kw select{flex:1;background:#0b1018;border:1px solid #30363d;color:#e6e6e6;padding:6px;border-radius:5px;font-size:12px;font-family:inherit}
.add-kw button{background:linear-gradient(135deg,#6366f1,#8b5cf6);color:#fff;border:none;padding:6px 14px;border-radius:5px;cursor:pointer;font-size:12px;font-weight:600;font-family:inherit}
.qchip{background:#1e293b;color:#94a3b8;border:1px solid #30363d;padding:3px 9px;border-radius:12px;cursor:pointer;font-size:11px;font-family:inherit}
.qchip:hover{border-color:#6366f1;color:#e2e8f0}
.right{display:none}
.mini-preview{flex-shrink:0;background:#fff;border-bottom:1px solid #30363d;height:280px;overflow:hidden;position:relative;display:flex;flex-direction:column;min-height:60px}
.mini-preview.collapsed{height:30px!important}
.mini-preview.collapsed .preview-wrap{display:none}
.mini-preview .preview-wrap{flex:1;overflow:hidden;background:#fff;position:relative}
.mini-preview iframe{border:none;background:#fff;transform-origin:top left;display:block;position:absolute;left:0;top:0}
.preview-wrap{flex:1;overflow:hidden;min-height:0;background:#fff;position:relative}
.preview-wrap iframe{display:block;border:none;background:#fff}
.preview-msg{padding:24px;color:#94a3b8;font-size:13px;line-height:1.6;background:#0d1117;height:100%;box-sizing:border-box;overflow:auto}
.preview-msg pre{background:#161b22;border:1px solid #30363d;padding:10px;border-radius:6px;color:#f48771;font-size:11px;font-family:Consolas,monospace;white-space:pre-wrap;max-height:340px;overflow:auto}
.preview-msg b{color:#f43f5e}
.preview-bar{padding:8px 14px;border-bottom:1px solid #30363d;background:#0b1018;display:flex;align-items:center;gap:8px;font-size:11px;color:#94a3b8}
.preview-bar .dot{width:8px;height:8px;border-radius:50%;background:#10b981;box-shadow:0 0 8px rgba(16,185,129,.6)}
.preview-bar.busy .dot{background:#f59e0b;animation:pulse .9s infinite}
.preview-bar.err .dot{background:#f43f5e}
.preview-bar .seg{display:inline-flex;border:1px solid #30363d;border-radius:5px;overflow:hidden}
.preview-bar .seg button{background:transparent;color:#94a3b8;border:none;padding:3px 9px;font-size:10px;cursor:pointer;font-family:inherit}
.preview-bar .seg button.on{background:#6366f1;color:#fff}
.preview-bar .bytes{color:#64748b;font-size:10px}
@keyframes pulse{50%{opacity:.4}}
.foot{padding:12px 18px;border-top:1px solid #30363d;display:flex;gap:8px;align-items:center;background:#0b1018}
.btn{background:#21262d;color:#e6e6e6;border:1px solid #30363d;padding:8px 14px;border-radius:6px;cursor:pointer;font-size:12px;font-weight:500;font-family:inherit}
.btn:hover{border-color:#6366f1}
.btn.primary{background:linear-gradient(135deg,#6366f1,#8b5cf6);color:#fff;border:none;font-weight:600;padding:9px 20px}
.btn.primary:disabled{opacity:.4;cursor:not-allowed}
.snippet{font-family:Consolas,monospace;font-size:11px;color:#a5d6a7;background:#0b1018;border:1px solid #30363d;padding:10px;border-radius:6px;white-space:pre-wrap;max-height:160px;overflow:auto;margin-top:10px}
.warn{margin:8px 12px;padding:8px 12px;background:rgba(244,63,94,.1);border:1px solid rgba(244,63,94,.3);border-radius:5px;color:#fca5a5;font-size:11px}
.chain-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(115px,1fr));gap:5px;margin-bottom:8px;max-height:180px;overflow-y:auto;padding:4px;background:#0b1018;border:1px solid #30363d;border-radius:6px}
.chain-grid button{background:#161b22;border:1px solid #30363d;color:#cbd5e1;padding:5px 7px;border-radius:5px;font-size:11px;font-family:Consolas,monospace;cursor:pointer;text-align:left;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.chain-grid button:hover{border-color:#6366f1;color:#e0e7ff}
.chain-grid button.on{background:linear-gradient(135deg,rgba(99,102,241,.2),rgba(139,92,246,.15));border-color:#6366f1;color:#e0e7ff}
.chain-item{background:#0b1018;border:1px solid #30363d;border-left:2px solid #a78bfa;border-radius:6px;padding:8px 10px;margin-bottom:5px}
.chain-item .chain-hd{display:flex;align-items:center;gap:6px;margin-bottom:4px}
.chain-item .chain-hd code{flex:1;font-size:11px;color:#a78bfa;font-family:Consolas,monospace}
.chain-item .chain-args{display:grid;grid-template-columns:auto 1fr;gap:4px 8px;font-size:11px}
.chain-item .chain-args label{color:#94a3b8;font-family:Consolas,monospace;align-self:center}
.chain-item .chain-args input,.chain-item .chain-args select{background:#161b22;border:1px solid #30363d;color:#e6e6e6;padding:3px 6px;border-radius:4px;font-size:11px;font-family:Consolas,monospace}
.chain-item .chain-doc{font-size:10px;color:#64748b;margin-top:4px;font-style:italic}
.theme-pick{display:flex;flex-wrap:wrap;gap:4px;margin-bottom:6px}
.theme-pick button{background:#161b22;border:1px solid #30363d;color:#cbd5e1;padding:5px 11px;border-radius:14px;font-size:11px;cursor:pointer;font-family:inherit}
.theme-pick button.on{background:linear-gradient(135deg,#6366f1,#8b5cf6);color:#fff;border-color:transparent;font-weight:600}
.src-tag{font-size:9.5px;padding:2px 7px;border-radius:8px;background:rgba(99,102,241,.15);color:#a5b4fc;border:1px solid rgba(99,102,241,.3);text-transform:uppercase;letter-spacing:.5px;font-weight:700;margin-left:6px}
.aliases{display:flex;flex-wrap:wrap;gap:8px;align-items:center;padding:6px 14px;background:#0a0e15;border-bottom:1px solid #1f2733}
.aliases .alias-lbl{font-size:9.5px;text-transform:uppercase;letter-spacing:.6px;color:#64748b;font-weight:700}
.aliases select{background:#161b22;border:1px solid #30363d;color:#a78bfa;padding:3px 8px;border-radius:5px;font-size:11px;font-family:Consolas,monospace;cursor:pointer}
.aliases select:focus{outline:none;border-color:#a78bfa}
.aliases .alias-hint{font-size:9.5px;color:#64748b;font-style:italic}
</style></head><body>
<div class="app">
<div class="left" id="leftPanel">
<div class="panel-bar"><span class="pb-title">Charts</span><button id="leftCollapse" title="Collapse / expand">\u25C0</button></div>
<div class="search"><input id="q" type="text" placeholder="Filter\u2026" autofocus></div>
<div class="list" id="list"></div>
</div>
<div class="resizer-v" id="resizeLeft"></div>
<div class="mid">
<div class="mini-preview" id="miniPrev">
  <div class="preview-bar" id="pbar"><span class="dot"></span><span id="pstatus">Idle</span><span class="bytes" id="pbytes"></span><span style="flex:1"></span><button class="btn" id="rerender" style="padding:3px 9px;font-size:10px">Re-render</button><button class="btn" id="miniCollapse" style="padding:3px 9px;font-size:10px" title="Collapse / expand preview">\u25B2</button></div>
  <div class="preview-wrap" id="pwrap"><div class="preview-msg" id="pmsg" style="padding:14px;font-size:11px;color:#94a3b8;text-align:center">Pick a chart on the left.</div></div>
</div>
<div class="resizer-h" id="resizeMini"></div>
<div class="head"><h1 id="title">Configuration</h1><span class="tag" id="tag">pick a chart</span><span class="src-tag" id="srcTag"></span><span class="head-spacer"></span><button class="theme-toggle" id="themeToggle" title="Toggle dark / light theme">\u263E Dark</button></div>
<div id="variants" class="aliases" style="display:none"></div>
<div id="aliases" class="aliases" style="display:none"></div>
<div id="catalogWarn"></div>
<div class="body" id="body"><div class="empty">Pick a chart on the left.<br><br>Every chart you select unlocks <b>all its kwargs</b> + <b>every chainable method</b> (<code>.set_bg</code>, <code>.text_auto</code>, <code>.crosshair</code>, …) discovered from the active wheel.</div></div>
<div class="foot"><button class="btn" id="reset">Reset</button><button class="btn primary" id="insert" disabled>Insert at cursor</button></div>
</div>
</div>
<script>
const v = acquireVsCodeApi();
const PAYLOAD = ${payloadJson};
const CATALOG = PAYLOAD.catalog;
const COMMON = PAYLOAD.common_kwargs || [];
const CHART_METHODS = PAYLOAD.chart_methods || [];
const THEMES = PAYLOAD.themes || [];
const CHART_METHOD_MAP = {}; for (const m of CHART_METHODS) CHART_METHOD_MAP[m.name] = m;

const UI_STATE_DEFAULTS = { leftWidth:260, miniHeight:280, leftCollapsed:false, miniCollapsed:false, themeMode:'dark', sectionCollapsed:{}, lastChart:null, lastKwargs:{}, lastChain:[], lastTheme:'', lastTitle:'', lastLabels:'', lastValues:'' };
let UI_STATE = Object.assign({}, UI_STATE_DEFAULTS, (v.getState && v.getState()) || {});
function saveUI(){ try{ v.setState(UI_STATE); }catch(e){} }
function applyUIState(){
  const lp = document.getElementById('leftPanel');
  const mp = document.getElementById('miniPrev');
  if (lp){ lp.style.width = (UI_STATE.leftWidth||260)+'px'; lp.classList.toggle('collapsed', !!UI_STATE.leftCollapsed); }
  if (mp){ mp.style.height = (UI_STATE.miniHeight||280)+'px'; mp.classList.toggle('collapsed', !!UI_STATE.miniCollapsed); }
  document.body.classList.toggle('theme-light', UI_STATE.themeMode === 'light');
  const tt = document.getElementById('themeToggle');
  if (tt) tt.textContent = UI_STATE.themeMode === 'light' ? '\u2600 Light' : '\u263E Dark';
}
function setupResizer(handleId, target, axis, key, minSz){
  const handle = document.getElementById(handleId);
  if (!handle) return;
  let dragging=false, startPos=0, startSize=0;
  handle.addEventListener('mousedown', function(e){ dragging=true; startPos = axis==='x'?e.clientX:e.clientY; startSize = axis==='x'?target.getBoundingClientRect().width:target.getBoundingClientRect().height; document.body.style.cursor = axis==='x'?'col-resize':'row-resize'; document.body.style.userSelect='none'; e.preventDefault(); });
  window.addEventListener('mousemove', function(e){ if (!dragging) return; const delta = (axis==='x'?e.clientX:e.clientY) - startPos; const next = Math.max(minSz, startSize + delta); if (axis==='x') target.style.width = next+'px'; else target.style.height = next+'px'; UI_STATE[key] = next; });
  window.addEventListener('mouseup', function(){ if (!dragging) return; dragging=false; document.body.style.cursor=''; document.body.style.userSelect=''; saveUI(); if (typeof scaleMini==='function') scaleMini(); });
}
function initStudioUI(){
  applyUIState();
  setupResizer('resizeLeft', document.getElementById('leftPanel'), 'x', 'leftWidth', 60);
  setupResizer('resizeMini', document.getElementById('miniPrev'), 'y', 'miniHeight', 60);
  const lc = document.getElementById('leftCollapse');
  if (lc) lc.addEventListener('click', function(){ UI_STATE.leftCollapsed = !UI_STATE.leftCollapsed; applyUIState(); saveUI(); });
  const mc = document.getElementById('miniCollapse');
  if (mc) mc.addEventListener('click', function(){ UI_STATE.miniCollapsed = !UI_STATE.miniCollapsed; applyUIState(); saveUI(); if (typeof scaleMini==='function') scaleMini(); });
  const tt = document.getElementById('themeToggle');
  if (tt) tt.addEventListener('click', function(){ UI_STATE.themeMode = UI_STATE.themeMode === 'light' ? 'dark' : 'light'; applyUIState(); saveUI(); });
  document.addEventListener('click', function(e){
    const t = e.target;
    if (t && t.tagName === 'H2' && t.parentElement && t.parentElement.classList && t.parentElement.classList.contains('section')){
      const sec = t.parentElement;
      const key = (t.textContent||'').trim().split(/\\s+/)[0];
      sec.classList.toggle('collapsed');
      UI_STATE.sectionCollapsed[key] = sec.classList.contains('collapsed');
      saveUI();
    }
  });
}
let current = null;
let title = '';
let labels = PAYLOAD.detected_labels_lit ? PAYLOAD.detected_labels_lit : (PAYLOAD.detected_labels ? PAYLOAD.detected_labels : '["A","B","C","D","E"]');
let values = PAYLOAD.detected_values_lit ? PAYLOAD.detected_values_lit : (PAYLOAD.detected_values ? PAYLOAD.detected_values : '[10, 20, 15, 25, 18]');
let kwargs = {};
let chain = [];
let theme = '';
let renderTimer = null;

const list = document.getElementById('list');
const q = document.getElementById('q');
const body = document.getElementById('body');
const titleEl = document.getElementById('title');
const tag = document.getElementById('tag');
const insertBtn = document.getElementById('insert');
const resetBtn = document.getElementById('reset');
const rerenderBtn = document.getElementById('rerender');
const pbar = document.getElementById('pbar');
const pstatus = document.getElementById('pstatus');
const pbytes = document.getElementById('pbytes');
const pwrap = document.getElementById('pwrap');
const catalogWarn = document.getElementById('catalogWarn');

if (PAYLOAD.catalog_error) {
  catalogWarn.innerHTML = '<div class="warn">Framework introspection error: ' + (PAYLOAD.catalog_error+'').replace(/</g,'&lt;') + '</div>';
}
const srcTag = document.getElementById('srcTag');
if (srcTag) srcTag.textContent = (PAYLOAD.source === 'wheel' ? 'wheel · v' : 'system · v') + (PAYLOAD.version || '');

function commonKw(key) { return COMMON.find(k => k.key === key); }
function chartKw(key) { if (!current || !current.kwargs) return null; return current.kwargs.find(k => k.key === key) || null; }
function lookupKw(key) { return chartKw(key) || commonKw(key); }

function renderList(filter){
  const f = (filter||'').toLowerCase();
  const groups = {};
  for (const e of CATALOG) {
    const hay = (e.label + ' ' + e.id + ' ' + e.group).toLowerCase();
    if (f && hay.indexOf(f) === -1) continue;
    if (!groups[e.group]) groups[e.group] = [];
    groups[e.group].push(e);
  }
  list.innerHTML = '';
  for (const g of Object.keys(groups)) {
    const h = document.createElement('div'); h.className='grp'; h.textContent = g; list.appendChild(h);
    for (const e of groups[g]) {
      const it = document.createElement('div'); it.className='item';
      if (current && current.id === e.id && current.kind === e.kind) it.classList.add('sel');
      it.innerHTML = '<span>'+e.label+'</span><code>'+e.id+'</code>';
      it.addEventListener('click', () => select(e));
      list.appendChild(it);
    }
  }
}

function select(e) {
  current = e;
  title = e.label;
  if (e.kind === 'chart') {
    labels = PAYLOAD.detected_labels_lit ? PAYLOAD.detected_labels_lit : (PAYLOAD.detected_labels ? PAYLOAD.detected_labels : '["A","B","C","D","E"]');
    values = PAYLOAD.detected_values_lit ? PAYLOAD.detected_values_lit : (PAYLOAD.detected_values ? PAYLOAD.detected_values : '[10, 20, 15, 25, 18]');
  }
  kwargs = {};
  chain = [];
  theme = '';
  titleEl.textContent = e.label;
  tag.textContent = e.kind === 'chart' ? 'sp.' + e.id : (e.kind === 'ml_class' ? 'class \u00b7 ' + e.id : 'dict-API \u00b7 ' + e.id);
  const aliasList = (PAYLOAD.aliases && PAYLOAD.aliases[e.id]) || [];
  const aliasEl = document.getElementById('aliases');
  if (aliasEl) {
    if (aliasList.length === 0) { aliasEl.innerHTML = ''; aliasEl.style.display = 'none'; }
    else {
      aliasEl.style.display = 'flex';
      const opts = [e.id].concat(aliasList).map(n => '<option value="' + n + '"' + (n === (current.callId || e.id) ? ' selected' : '') + '>sp.' + n + '</option>').join('');
      aliasEl.innerHTML = '<span class="alias-lbl">alias</span><select id="aliasSel">' + opts + '</select><span class="alias-hint">' + aliasList.length + ' alias' + (aliasList.length>1?'es':'') + ' \u2014 same callable, alternative spelling</span>';
      const sel = document.getElementById('aliasSel');
      sel.addEventListener('change', () => { current.callId = sel.value; updateSnippet(); scheduleRender(); });
    }
  }
  const variantsEl = document.getElementById('variants');
  const stem = e.id.toLowerCase().replace(/^build_/,'').replace(/_chart$/,'').replace(/_family$/,'').replace(/_unified$/,'').replace(/s$/,'');
  const variantSpec = (PAYLOAD.variants && (PAYLOAD.variants[e.id] || PAYLOAD.variants[stem])) || null;
  if (variantsEl) {
    if (!variantSpec || e.kind !== 'chart') {
      variantsEl.innerHTML = '';
      variantsEl.style.display = 'none';
      current.variantKey = null;
    } else {
      variantsEl.style.display = 'flex';
      if (!current.variantKey) current.variantKey = variantSpec.default;
      var vopts = variantSpec.variants.map(function(vv){
        var lbl = vv.aliases && vv.aliases.length ? (vv.key + '  \u2014  ' + vv.aliases.join(', ')) : vv.key;
        return '<option value="' + vv.key + '"' + (vv.key === current.variantKey ? ' selected' : '') + '>' + lbl + '</option>';
      }).join('');
      variantsEl.innerHTML = '<span class="alias-lbl" style="color:#a78bfa">variant</span><select id="variantSel">' + vopts + '</select><span class="alias-hint">' + variantSpec.variants.length + ' sub-types \u2014 different rendering, same family</span>';
      var vs = document.getElementById('variantSel');
      vs.addEventListener('change', function(){ current.variantKey = vs.value; updateSnippet(); scheduleRender(); });
    }
  }
  insertBtn.disabled = false;
  renderList(q.value);
  renderForm();
  scheduleRender();
}

function pyLitJS(field, val) {
  if (val === null || val === undefined) return 'None';
  const s = String(val).trim();
  if (s === '') return 'None';
  if (field.kind === 'bool') return (val === true || s === 'true' || s === 'True') ? 'True' : 'False';
  if (field.kind === 'int') return String(parseInt(s,10));
  if (field.kind === 'float') return String(parseFloat(s));
  if (field.kind === 'color') {
    const hex = s.replace(/^#/,'').toUpperCase();
    return /^[0-9A-F]{6}$/.test(hex) ? '0x'+hex : JSON.stringify(s);
  }
  if (field.kind === 'color_str') {
    const hex = s.replace(/^#/,'').toUpperCase();
    return /^[0-9A-F]{6}$/.test(hex) ? JSON.stringify('#'+hex) : JSON.stringify(s);
  }
  if (field.kind === 'json') return s;
  return JSON.stringify(s);
}

function buildSnippetText() {
  if (!current) return '';
  const parts = [];
  if (current.kind === 'chart' && current.variantKey) parts.push('variant=' + JSON.stringify(current.variantKey));
  for (const k of Object.keys(kwargs)) {
    const v = kwargs[k];
    if (v === '' || v === null || v === undefined) continue;
    const f = lookupKw(k) || { key:k, kind:'string' };
    const lit = pyLitJS(f, v);
    if (lit === 'None') continue;
    parts.push(k + '=' + lit);
  }
  if (current.kind === 'chart') {
    const tail = parts.length ? ', ' + parts.join(', ') : '';
    const chainStr = renderChainJS();
    const themeLine = theme ? 'sp.theme(' + JSON.stringify(theme) + ')\\n' : '';
    return themeLine + 'chart = sp.' + current.id + '(' + JSON.stringify(title) + ', labels=' + labels + ', values=' + values + tail + ')' + chainStr + '\\nchart.save(' + JSON.stringify(current.id+'.html') + ')';
  }
  if (current.kind === 'ml_class') {
    return 'model = sp.' + current.id + '(' + parts.join(', ') + ')\\nmodel.fit(X, y)\\npred = model.predict(X)';
  }
  const isCluster = current.task === 'clustering';
  const dictParts = isCluster ? ['    "x": xs', '    "y": ys'] : ['    "x": X', '    "y": y'];
  for (const p of parts) { const m = p.match(/^(\\w+)=(.*)$/); if (m) dictParts.push('    ' + JSON.stringify(m[1]) + ': ' + m[2]); }
  const printer = isCluster ? 'print("labels:", result.get("labels"))' : 'print("score:", result.get("score"))';
  return 'result = sp.' + current.id + '({\\n' + dictParts.join(',\\n') + '\\n})\\n' + printer;
}

function renderChainJS() {
  if (!chain.length) return '';
  const out = [];
  for (const item of chain) {
    const def = CHART_METHOD_MAP[item.name];
    if (!def) continue;
    const args = [];
    const positional = item.positional || {};
    for (const p of (def.params || [])) {
      const val = positional[p];
      if (val === undefined || val === null || (typeof val === 'string' && val.trim() === '')) continue;
      const f = (def.kwargs || []).find(k => k.key === p) || { key:p, kind:'string' };
      args.push(pyLitJS(f, val));
    }
    const kw = item.kwargs || {};
    for (const key of Object.keys(kw)) {
      const val = kw[key];
      if (val === null || val === undefined || (typeof val === 'string' && val.trim() === '')) continue;
      const f = (def.kwargs || []).find(k => k.key === key) || { key, kind:'string' };
      const lit = pyLitJS(f, val);
      if (lit === 'None') continue;
      args.push(key + '=' + lit);
    }
    out.push('.' + def.name + '(' + args.join(', ') + ')');
  }
  return out.join('');
}

function fieldInputHtml(f, value) {
  const id = 'fld_' + f.key;
  if (f.kind === 'bool') {
    return '<div class="row"><div class="switch ' + (value ? 'on':'') + '" data-k="'+f.key+'" data-toggle="1"></div><span style="font-size:11px;color:#94a3b8">'+(value?'on':'off')+'</span></div>';
  }
  if (f.kind === 'enum') {
    const opts = (f.options||['']).map(o => '<option ' + (String(o) === String(value||'') ? 'selected':'') + '>'+o+'</option>').join('');
    return '<select id="'+id+'" data-k="'+f.key+'">'+opts+'</select>';
  }
  if (f.kind === 'color' || f.kind === 'color_str') {
    const safe = /^#[0-9A-Fa-f]{6}$/.test(String(value||'')) ? String(value) : '#888888';
    return '<div class="row"><input type="color" id="'+id+'_c" value="'+safe+'" data-k="'+f.key+'"><input type="text" id="'+id+'" data-k="'+f.key+'" value="'+(value||'')+'" placeholder="empty = none" style="flex:1"></div>';
  }
  if (f.kind === 'int' || f.kind === 'float') {
    return '<input type="number" id="'+id+'" data-k="'+f.key+'" value="'+(value!==undefined?value:'')+'" step="'+(f.kind==='float'?'0.01':'1')+'">';
  }
  return '<input type="text" id="'+id+'" data-k="'+f.key+'" value="'+(value||'')+'">';
}

function renderForm() {
  if (!current) return;
  let html = '';
  if (current.kind === 'chart') {
    html += '<div class="section"><h2>Chart data</h2>';
    if (PAYLOAD.detected_labels || PAYLOAD.detected_values || PAYLOAD.detected_labels_lit || PAYLOAD.detected_values_lit) {
      const lblName = PAYLOAD.detected_labels_lit ? 'literal' : (PAYLOAD.detected_labels || '\u2014');
      const valName = PAYLOAD.detected_values_lit ? 'literal' : (PAYLOAD.detected_values || '\u2014');
      const lbl = lblName !== '\u2014' ? '<code>'+lblName+'</code>' : '\u2014';
      const val = valName !== '\u2014' ? '<code>'+valName+'</code>' : '\u2014';
      html += '<div class="hint" style="margin-bottom:10px;background:rgba(99,102,241,.08);border-left:2px solid #6366f1;padding:6px 10px;border-radius:4px;font-size:11px;color:#94a3b8">Detected from your file \u2014 labels: '+lbl+', values: '+val+'</div>';
    }
    html += '<div class="f"><label>title</label><input type="text" id="dataTitle" value="' + (title||'').replace(/"/g,'&quot;') + '"></div>';
    html += '<div class="f"><label>labels</label><textarea id="dataLabels">' + labels + '</textarea></div>';
    html += '<div class="f"><label>values</label><textarea id="dataValues">' + values + '</textarea></div>';
    html += '</div>';
    if (THEMES.length) {
      const tBtns = THEMES.map(t => '<button class="' + (theme === t ? 'on':'') + '" data-theme="'+t+'">'+t+'</button>').join('');
      html += '<div class="section"><h2>Theme</h2><div class="theme-pick"><button class="' + (!theme ? 'on':'') + '" data-theme="">none</button>' + tBtns + '</div></div>';
    }
    if (CHART_METHODS.length) {
      html += '<div class="section"><h2>Chain methods <span style="color:#64748b;font-size:10px;font-weight:400;text-transform:none;letter-spacing:0">click to add</span></h2><div class="chain-grid">';
      for (const m of CHART_METHODS) {
        const on = chain.find(c => c.name === m.name) ? ' on' : '';
        html += '<button class="' + on.trim() + '" data-method="'+m.name+'" title="'+(m.doc||m.name).replace(/"/g,'&quot;').slice(0,140)+'">.'+m.name+'</button>';
      }
      html += '</div><div id="chainList"></div></div>';
    }
    if (current.kwargs && current.kwargs.length) {
      const inUse = current.kwargs.filter(k => !(k.key in kwargs));
      if (inUse.length) {
        const opts = inUse.map(c => '<option value="'+c.key+'">'+c.key+' \u00b7 '+c.kind+'</option>').join('');
        html += '<div class="section"><h2>Per-chart kwargs <span style="color:#64748b;font-size:10px;font-weight:400;text-transform:none;letter-spacing:0">from sp.'+current.id+' signature</span></h2><div class="add-kw"><select id="chartKwPicker">'+opts+'</select><button id="chartKwAdd">Add</button></div></div>';
      }
    }
  } else if (current.kind === 'ml_fn') {
    html += '<div class="section"><h2>Signature</h2><div class="f"><label>' + current.id + current.signature + '</label><div class="hint">Inputs (x/y) come from your code variables. Optional kwargs go into the dict.</div></div></div>';
  } else {
    html += '<div class="section"><h2>Constructor kwargs</h2><div class="f"><label>(any keyword goes to ' + current.id + '(...))</label><div class="hint">Add kwargs below; they are passed to the constructor.</div></div></div>';
  }

  html += '<div class="section"><h2>Common kwargs</h2><div id="kwList"></div>';
  const remaining = COMMON.filter(c => !(c.key in kwargs));
  const quickKeys = ['background','font_size','border_radius','animation','tooltip','crosshair','legend','grid'];
  const quickAvail = quickKeys.filter(k => remaining.find(c => c.key === k));
  if (quickAvail.length) {
    const chips = quickAvail.map(k => '<button class="qchip" onclick="quickAdd(\\''+k+'\\')">+ '+k+'</button>').join('');
    html += '<div style="display:flex;flex-wrap:wrap;gap:4px;margin-bottom:8px">'+chips+'</div>';
  }
  if (remaining.length) {
    const opts = remaining.map(c => '<option value="'+c.key+'">'+c.key+' \u00b7 '+c.kind+'</option>').join('');
    html += '<div class="add-kw"><select id="kwPicker">'+opts+'</select><input id="kwCustom" type="text" placeholder="or type name\u2026" style="flex:1;background:#0b1018;border:1px solid #30363d;color:#e6e6e6;padding:6px;border-radius:5px;font-family:Consolas,monospace;font-size:12px"><button id="kwAdd">Add</button></div>';
  } else {
    html += '<div class="add-kw"><input id="kwCustom" type="text" placeholder="custom kwarg name\u2026" style="flex:1;background:#0b1018;border:1px solid #30363d;color:#e6e6e6;padding:6px;border-radius:5px;font-family:Consolas,monospace;font-size:12px"><button id="kwAdd">Add</button></div>';
  }
  html += '</div><pre class="snippet" id="snip"></pre>';

  body.innerHTML = html;
  attachHandlers();
  renderKwList();
  renderChainList();
  updateSnippet();
  if (UI_STATE && UI_STATE.sectionCollapsed){
    const secs = body.querySelectorAll('.section');
    secs.forEach(function(sec){
      const h = sec.querySelector('h2'); if (!h) return;
      const key = (h.textContent||'').trim().split(/\\s+/)[0];
      if (UI_STATE.sectionCollapsed[key]) sec.classList.add('collapsed');
    });
  }
}

function renderChainList() {
  const div = document.getElementById('chainList');
  if (!div) return;
  if (!chain.length) { div.innerHTML = ''; return; }
  let html = '<div style="margin-top:8px">';
  chain.forEach((item, idx) => {
    const def = CHART_METHOD_MAP[item.name];
    if (!def) return;
    html += '<div class="chain-item"><div class="chain-hd"><code>.' + item.name + '(\u2026)</code><button class="x" data-chainup="'+idx+'" title="up">\u2191</button><button class="x" data-chaindn="'+idx+'" title="down">\u2193</button><button class="x" data-chainrm="'+idx+'">\u00d7</button></div>';
    if (def.doc) html += '<div class="chain-doc">' + def.doc.replace(/</g,'&lt;').slice(0,160) + '</div>';
    if (!def.no_args && (def.params && def.params.length || def.kwargs && def.kwargs.length)) {
      html += '<div class="chain-args">';
      for (const p of (def.params || [])) {
        const f = (def.kwargs || []).find(k => k.key === p) || { key:p, kind:'string' };
        const cur = (item.positional && item.positional[p] !== undefined) ? item.positional[p] : (f.default !== undefined ? f.default : '');
        html += '<label>'+p+'</label>' + chainFieldHtml(f, cur, idx, p, true);
      }
      for (const f of (def.kwargs || [])) {
        if ((def.params || []).indexOf(f.key) !== -1) continue;
        const cur = (item.kwargs && item.kwargs[f.key] !== undefined) ? item.kwargs[f.key] : (f.default !== undefined ? f.default : '');
        html += '<label>'+f.key+'</label>' + chainFieldHtml(f, cur, idx, f.key, false);
      }
      html += '</div>';
    }
    html += '</div>';
  });
  html += '</div>';
  div.innerHTML = html;
  div.querySelectorAll('[data-chainrm]').forEach(b => b.addEventListener('click', () => { chain.splice(parseInt(b.getAttribute('data-chainrm')),1); renderForm(); scheduleRender(); }));
  div.querySelectorAll('[data-chainup]').forEach(b => b.addEventListener('click', () => { const i = parseInt(b.getAttribute('data-chainup')); if (i>0) { const t = chain[i-1]; chain[i-1] = chain[i]; chain[i] = t; renderForm(); scheduleRender(); } }));
  div.querySelectorAll('[data-chaindn]').forEach(b => b.addEventListener('click', () => { const i = parseInt(b.getAttribute('data-chaindn')); if (i<chain.length-1) { const t = chain[i+1]; chain[i+1] = chain[i]; chain[i] = t; renderForm(); scheduleRender(); } }));
  div.querySelectorAll('[data-chainfld]').forEach(el => {
    el.addEventListener('input', () => {
      const idx = parseInt(el.getAttribute('data-chainidx'));
      const key = el.getAttribute('data-chainfld');
      const isPos = el.getAttribute('data-chainpos') === '1';
      const target = isPos ? (chain[idx].positional = chain[idx].positional || {}) : (chain[idx].kwargs = chain[idx].kwargs || {});
      let val = el.value;
      if (el.type === 'checkbox') val = el.checked;
      else if (el.type === 'number') val = el.value === '' ? '' : Number(el.value);
      target[key] = val;
      updateSnippet(); scheduleRender();
    });
  });
}

function chainFieldHtml(f, value, idx, key, isPos) {
  const attrs = 'data-chainfld="'+key+'" data-chainidx="'+idx+'" data-chainpos="'+(isPos?'1':'0')+'"';
  if (f.kind === 'bool') return '<input type="checkbox" '+attrs+(value===true||value==='true'?' checked':'')+'>';
  if (f.kind === 'enum') {
    const opts = (f.options||['']).map(o => '<option ' + (String(o) === String(value||'') ? 'selected':'') + '>'+o+'</option>').join('');
    return '<select '+attrs+'>'+opts+'</select>';
  }
  if (f.kind === 'color' || f.kind === 'color_str') return '<input type="text" '+attrs+' value="'+(value||'')+'" placeholder="#RRGGBB">';
  if (f.kind === 'int' || f.kind === 'float') return '<input type="number" '+attrs+' value="'+(value!==undefined&&value!==null?value:'')+'" step="'+(f.kind==='float'?'0.01':'1')+'">';
  return '<input type="text" '+attrs+' value="'+(value!==undefined&&value!==null?String(value).replace(/"/g,'&quot;'):'')+'">';
}

function attachHandlers() {
  const dt = document.getElementById('dataTitle'); if (dt) dt.addEventListener('input', e => { title = e.target.value; updateSnippet(); scheduleRender(); });
  const dl = document.getElementById('dataLabels'); if (dl) dl.addEventListener('input', e => { labels = e.target.value; updateSnippet(); scheduleRender(); });
  const dv = document.getElementById('dataValues'); if (dv) dv.addEventListener('input', e => { values = e.target.value; updateSnippet(); scheduleRender(); });
  const add = document.getElementById('kwAdd');
  if (add) add.addEventListener('click', () => {
    const cust = document.getElementById('kwCustom').value.trim();
    const sel = document.getElementById('kwPicker');
    const key = cust || (sel && sel.value);
    if (!key) return;
    if (!(key in kwargs)) {
      const f = lookupKw(key);
      kwargs[key] = f ? (f.default !== undefined ? f.default : '') : '';
    }
    renderForm(); scheduleRender();
  });
  const cAdd = document.getElementById('chartKwAdd');
  if (cAdd) cAdd.addEventListener('click', () => {
    const sel = document.getElementById('chartKwPicker');
    const key = sel && sel.value; if (!key) return;
    const f = chartKw(key);
    kwargs[key] = f ? (f.default !== undefined ? f.default : '') : '';
    renderForm(); scheduleRender();
  });
  document.querySelectorAll('[data-theme]').forEach(b => b.addEventListener('click', () => {
    theme = b.getAttribute('data-theme') || '';
    renderForm(); scheduleRender();
  }));
  document.querySelectorAll('[data-method]').forEach(b => b.addEventListener('click', () => {
    const name = b.getAttribute('data-method');
    const def = CHART_METHOD_MAP[name]; if (!def) return;
    const positional = {};
    const kw = {};
    for (const f of (def.kwargs || [])) {
      if (f.default !== undefined && f.default !== null && f.default !== '') {
        if ((def.params || []).indexOf(f.key) !== -1) positional[f.key] = f.default;
        else kw[f.key] = f.default;
      }
    }
    chain.push({ name, positional, kwargs: kw });
    renderForm(); scheduleRender();
  }));
}

function quickAdd(key) {
  if (!(key in kwargs)) {
    const f = lookupKw(key);
    kwargs[key] = f ? (f.default !== undefined ? f.default : '') : '';
  }
  renderForm(); scheduleRender();
}

function renderKwList() {
  const div = document.getElementById('kwList');
  if (!div) return;
  if (Object.keys(kwargs).length === 0) {
    div.innerHTML = '<div class="hint" style="font-size:11px;color:#64748b;padding:6px">No kwargs yet. Pick one above.</div>';
    return;
  }
  let html = '';
  for (const key of Object.keys(kwargs)) {
    const f = lookupKw(key) || { key, kind: 'string' };
    html += '<div class="f"><label>' + key + ' <button class="x" data-rm="'+key+'">\u00d7</button></label>' + fieldInputHtml(f, kwargs[key]) + (f.hint ? '<div class="hint">'+f.hint+'</div>' : '') + '</div>';
  }
  div.innerHTML = html;
  div.querySelectorAll('input,select').forEach(el => {
    el.addEventListener('input', () => {
      const k = el.getAttribute('data-k'); if (!k) return;
      if (el.type === 'number') kwargs[k] = el.value === '' ? '' : Number(el.value);
      else kwargs[k] = el.value;
      if (el.type === 'color') {
        const txt = document.getElementById(el.id.replace('_c','')); if (txt) { txt.value = el.value; kwargs[k] = el.value; }
      } else if (!el.id.endsWith('_c')) {
        const c = document.getElementById(el.id + '_c'); if (c && /^#[0-9A-Fa-f]{6}$/.test(el.value)) c.value = el.value;
      }
      updateSnippet(); scheduleRender();
    });
  });
  div.querySelectorAll('.switch').forEach(s => {
    s.addEventListener('click', () => {
      const k = s.getAttribute('data-k'); if (!k) return;
      kwargs[k] = !kwargs[k]; s.classList.toggle('on', kwargs[k]);
      const span = s.parentElement.querySelector('span'); if (span) span.textContent = kwargs[k] ? 'on':'off';
      updateSnippet(); scheduleRender();
    });
  });
  div.querySelectorAll('button[data-rm]').forEach(b => {
    b.addEventListener('click', () => { delete kwargs[b.getAttribute('data-rm')]; renderForm(); scheduleRender(); });
  });
}

function updateSnippet() { const s = document.getElementById('snip'); if (s) s.textContent = buildSnippetText(); if (typeof persistChart === 'function') persistChart(); }

function scheduleRender() {
  if (!current) return;
  if (current.kind !== 'chart') {
    pwrap.innerHTML = '<div class="preview-msg" style="padding:14px;font-size:11px;color:#94a3b8;text-align:center">No preview for ML entries.</div>';
    pbytes.textContent = ''; pstatus.textContent = 'ML \u2014 no preview'; pbar.classList.remove('busy','err');
    return;
  }
  if (renderTimer) clearTimeout(renderTimer);
  pbar.classList.remove('err'); pbar.classList.add('busy');
  pstatus.textContent = 'Rendering sp.' + current.id + '\u2026';
  renderTimer = setTimeout(() => {
    const sendKwargs = Object.assign({}, kwargs);
    if (current.kind === 'chart' && current.variantKey) sendKwargs.variant = current.variantKey;
    v.postMessage({ type:'render', id: current.callId || current.id, kind: current.kind, title, labels, values, kwargs: sendKwargs, chain, theme });
  }, 350);
}

window.addEventListener('message', ev => {
  const m = ev.data;
  if (!m || m.type !== 'rendered') return;
  pbar.classList.remove('busy');
  if (m.error) {
    pbar.classList.add('err'); pstatus.textContent = 'Render failed'; pbytes.textContent = '';
    pwrap.innerHTML = '<div class="preview-msg" style="padding:10px;font-size:10.5px;color:#fca5a5;text-align:left;overflow:auto;height:100%"><b>No usable HTML.</b><br>' + (m.error||'').replace(/</g,'&lt;').replace(/\\n/g,'<br>') + '</div>';
    return;
  }
  pbar.classList.remove('err');
  const kb = m.bytes ? (m.bytes >= 1024 ? (m.bytes/1024).toFixed(1)+' KB' : m.bytes+' B') : '';
  pstatus.textContent = 'Live \u00b7 sp.' + (current ? (current.callId || current.id) : '');
  pbytes.textContent = kb + (m.meta && m.meta.info ? ' \u00b7 svg=' + (m.meta.info.has_svg?'y':'n') + ' rects=' + m.meta.info.rects + ' paths=' + m.meta.info.paths : '');
  const iframe = document.createElement('iframe');
  iframe.srcdoc = m.html;
  iframe.style.width = '900px';
  iframe.style.height = '600px';
  const fit = () => {
    try {
      const cw = pwrap.clientWidth || 1;
      const ch = pwrap.clientHeight || 1;
      const doc = iframe.contentDocument;
      let nw = 900, nh = 600;
      if (doc) {
        const svg = doc.querySelector('svg');
        if (svg) {
          const vb = svg.getAttribute('viewBox');
          if (vb) {
            const p = vb.split(/\s+/).map(parseFloat);
            if (p.length === 4) { nw = p[2]; nh = p[3]; }
          } else {
            nw = svg.clientWidth || svg.getBoundingClientRect().width || 900;
            nh = svg.clientHeight || svg.getBoundingClientRect().height || 600;
          }
        } else if (doc.documentElement) {
          nw = doc.documentElement.scrollWidth || 900;
          nh = doc.documentElement.scrollHeight || 600;
        }
      }
      const sx = cw / nw, sy = ch / nh;
      const s = Math.min(sx, sy, 1);
      iframe.style.width = nw + 'px';
      iframe.style.height = nh + 'px';
      iframe.style.transform = 'scale(' + s + ')';
      iframe.style.transformOrigin = '0 0';
      iframe.style.position = 'absolute';
      iframe.style.left = Math.max(0, (cw - nw * s) / 2) + 'px';
      iframe.style.top = Math.max(0, (ch - nh * s) / 2) + 'px';
    } catch (_e) { }
  };
  iframe.addEventListener('load', () => { requestAnimationFrame(() => { fit(); setTimeout(fit, 80); setTimeout(fit, 240); }); });
  if (window._spFitObs) { try { window._spFitObs.disconnect(); } catch(_e){} }
  try { const ro = new ResizeObserver(() => fit()); ro.observe(pwrap); window._spFitObs = ro; } catch (_e) { }
  pwrap.innerHTML = '';
  pwrap.appendChild(iframe);
});

resetBtn.addEventListener('click', () => { if (current) { kwargs = {}; chain = []; theme = ''; if (current.kind === 'chart') { labels = '["A","B","C","D","E"]'; values = '[10, 20, 15, 25, 18]'; } renderForm(); scheduleRender(); persistChart(); } });
insertBtn.addEventListener('click', () => { if (current) { const sendKwargs = Object.assign({}, kwargs); if (current.kind === 'chart' && current.variantKey) sendKwargs.variant = current.variantKey; v.postMessage({ type:'insert', id: current.callId || current.id, kind: current.kind, title, labels, values, kwargs: sendKwargs, chain, theme }); } });
rerenderBtn.addEventListener('click', () => scheduleRender());
q.addEventListener('input', () => renderList(q.value));

function persistChart(){ if (!current) return; UI_STATE.lastChart = { id: current.id, kind: current.kind, callId: current.callId||null, variantKey: current.variantKey||null }; UI_STATE.lastKwargs = kwargs; UI_STATE.lastChain = chain; UI_STATE.lastTheme = theme; UI_STATE.lastTitle = title; UI_STATE.lastLabels = labels; UI_STATE.lastValues = values; saveUI(); }

initStudioUI();
renderList('');
(function restoreLast(){
  const lc = UI_STATE.lastChart; if (!lc) return;
  const ent = CATALOG.find(e => e.id === lc.id && e.kind === lc.kind); if (!ent) return;
  select(ent);
  if (lc.callId) current.callId = lc.callId;
  if (lc.variantKey) current.variantKey = lc.variantKey;
  if (UI_STATE.lastKwargs) kwargs = Object.assign({}, UI_STATE.lastKwargs);
  if (UI_STATE.lastChain) chain = UI_STATE.lastChain.slice();
  theme = UI_STATE.lastTheme || '';
  if (UI_STATE.lastTitle) title = UI_STATE.lastTitle;
  if (UI_STATE.lastLabels) labels = UI_STATE.lastLabels;
  if (UI_STATE.lastValues) values = UI_STATE.lastValues;
  renderForm(); scheduleRender();
})();
</script>
</body></html>`;
}
//# sourceMappingURL=extension.js.map