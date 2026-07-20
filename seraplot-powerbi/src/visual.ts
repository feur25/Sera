"use strict";

import powerbi from "powerbi-visuals-api";
import { FormattingSettingsService } from "powerbi-visuals-utils-formattingmodel";
import "./../style/visual.less";

import VisualConstructorOptions = powerbi.extensibility.visual.VisualConstructorOptions;
import VisualUpdateOptions = powerbi.extensibility.visual.VisualUpdateOptions;
import IVisual = powerbi.extensibility.visual.IVisual;
import IVisualEventService = powerbi.extensibility.IVisualEventService;
import DataView = powerbi.DataView;

import { VisualFormattingSettingsModel } from "./settings";

declare global {
    interface Window {
        SeraplotWASM?: any;
    }
}

const WASM_BASE = "https://feur25.github.io/Sera/docs/theme/";
const WASM_JS_URL = WASM_BASE + "seraplot-web.20260720.js";
const WASM_BIN_URL = WASM_BASE + "seraplot_bg.wasm";

const BUILD_FN: Record<string, string> = {
    bar: "buildBar",
    line: "buildLine",
    pie: "buildPie",
    area: "buildAreaChart",
    histogram: "buildHistogram",
    scatter: "buildScatterChart",
};

let wasmLoadPromise: Promise<any> | null = null;

function loadWasm(): Promise<any> {
    if (window.SeraplotWASM && window.SeraplotWASM.__ready) {
        return Promise.resolve(window.SeraplotWASM);
    }
    if (wasmLoadPromise) return wasmLoadPromise;
    wasmLoadPromise = new Promise((resolve, reject) => {
        const existing = document.getElementById("sp-wasm-glue");
        const afterGlue = () => {
            const sp = window.SeraplotWASM;
            if (!sp || typeof sp.__init !== "function") {
                reject(new Error("SeraplotWASM glue did not initialize"));
                return;
            }
            sp.__init(WASM_BIN_URL).then(() => resolve(sp)).catch(reject);
        };
        if (existing) {
            afterGlue();
            return;
        }
        const script = document.createElement("script");
        script.id = "sp-wasm-glue";
        script.src = WASM_JS_URL;
        script.onload = afterGlue;
        script.onerror = () => reject(new Error("Failed to load " + WASM_JS_URL));
        document.head.appendChild(script);
    });
    return wasmLoadPromise;
}

function extractSeries(dataView: DataView | undefined) {
    const empty = { labels: [] as string[], measure1: [] as number[], measure2: [] as number[] };
    if (!dataView || !dataView.categorical) return empty;
    const cat = dataView.categorical;
    const labels = (cat.categories && cat.categories[0] ? cat.categories[0].values : []).map((v) => String(v));
    const values = cat.values || [];
    const measure1 = values[0] ? values[0].values.map((v) => Number(v)) : [];
    const measure2 = values[1] ? values[1].values.map((v) => Number(v)) : [];
    return { labels, measure1, measure2 };
}

function buildParams(chartType: string, series: ReturnType<typeof extractSeries>, width: number, height: number, background: string) {
    const base: Record<string, unknown> = { width, height, bg_color: background };
    if (chartType === "scatter") {
        return Object.assign(base, {
            x_values: series.measure1,
            y_values: series.measure2,
            labels: series.labels,
        });
    }
    return Object.assign(base, {
        labels: series.labels,
        values: series.measure1,
    });
}

export class Visual implements IVisual {
    private events: IVisualEventService;
    private target: HTMLElement;
    private iframe: HTMLIFrameElement;
    private statusEl: HTMLElement;
    private formattingSettings: VisualFormattingSettingsModel;
    private formattingSettingsService: FormattingSettingsService;
    private lastBlobUrl: string | null = null;

    constructor(options: VisualConstructorOptions) {
        this.events = options.host.eventService;
        this.formattingSettingsService = new FormattingSettingsService();
        this.target = options.element;
        this.target.style.cssText = "position:relative;width:100%;height:100%;overflow:hidden;";

        this.statusEl = document.createElement("div");
        this.statusEl.style.cssText = "position:absolute;inset:0;display:flex;align-items:center;justify-content:center;color:#94a3b8;font:12px sans-serif;text-align:center;padding:12px;box-sizing:border-box;";
        this.statusEl.textContent = "Loading SeraPlot…";
        this.target.appendChild(this.statusEl);

        this.iframe = document.createElement("iframe");
        this.iframe.sandbox.add("allow-scripts");
        this.iframe.style.cssText = "border:none;width:100%;height:100%;display:none;";
        this.target.appendChild(this.iframe);
    }

    public update(options: VisualUpdateOptions) {
        this.events.renderingStarted(options);
        try {
            const dataView = options.dataViews && options.dataViews[0];
            this.formattingSettings = this.formattingSettingsService.populateFormattingSettingsModel(VisualFormattingSettingsModel, dataView);

            const chartType = this.formattingSettings.chartSettings.chartType.value.value as string;
            const background = this.formattingSettings.chartSettings.backgroundColor.value.value;
            const series = extractSeries(dataView);

            if (!series.labels.length && !series.measure1.length) {
                this.showStatus("Add a Category and a Values field to render a chart.");
                this.events.renderingFinished(options);
                return;
            }

            const width = Math.max(60, Math.floor(options.viewport.width));
            const height = Math.max(60, Math.floor(options.viewport.height));
            const params = buildParams(chartType, series, width, height, background);
            const fnName = BUILD_FN[chartType] || BUILD_FN.bar;

            this.showStatus("Loading SeraPlot…");
            loadWasm()
                .then((sp) => {
                    const fn = sp[fnName];
                    if (typeof fn !== "function") {
                        this.showStatus("SeraPlot function not available: " + fnName);
                        this.events.renderingFinished(options);
                        return;
                    }
                    const html = fn(JSON.stringify(params));
                    this.renderHtml(html);
                    this.events.renderingFinished(options);
                })
                .catch((err) => {
                    this.showStatus("Failed to load SeraPlot WASM: " + (err && err.message ? err.message : String(err)));
                    this.events.renderingFailed(options, String(err));
                });
        } catch (error) {
            this.showStatus("Error: " + String(error));
            this.events.renderingFailed(options, String(error));
        }
    }

    private showStatus(text: string) {
        this.statusEl.textContent = text;
        this.statusEl.style.display = "flex";
        this.iframe.style.display = "none";
    }

    private renderHtml(html: string) {
        if (!html) {
            this.showStatus("SeraPlot returned no output for this chart type/data combination.");
            return;
        }
        const blob = new Blob([html], { type: "text/html" });
        const url = URL.createObjectURL(blob);
        const prev = this.lastBlobUrl;
        this.iframe.src = url;
        this.lastBlobUrl = url;
        this.statusEl.style.display = "none";
        this.iframe.style.display = "block";
        if (prev) setTimeout(() => URL.revokeObjectURL(prev), 500);
    }

    public getFormattingModel(): powerbi.visuals.FormattingModel {
        return this.formattingSettingsService.buildFormattingModel(this.formattingSettings);
    }
}
