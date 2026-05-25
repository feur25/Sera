declare namespace wasm_bindgen {
    /* tslint:disable */
    /* eslint-disable */

    export function buildAreaChart(input: string): string;

    export function buildBar(input: string): string;

    export function buildBar3dChart(input: string): string;

    export function buildBarChart(input: string): string;

    export function buildBoxplot(input: string): string;

    export function buildBubble(input: string): string;

    export function buildBubble3dChart(input: string): string;

    export function buildBubbleMap(input: string): string;

    export function buildBullet(input: string): string;

    export function buildCandlestick(input: string): string;

    export function buildCandlestick3dChart(input: string): string;

    export function buildChoropleth(input: string): string;

    export function buildDbscanChart(input: string): string;

    export function buildDbscanChart3d(input: string): string;

    export function buildDonutChart(input: string): string;

    export function buildDumbbell(input: string): string;

    export function buildDumbbell3dChart(input: string): string;

    export function buildFunnel(input: string): string;

    export function buildFunnel3dChart(input: string): string;

    export function buildGauge(input: string): string;

    export function buildGlobe3dChart(input: string): string;

    export function buildGrid(input: string): string;

    export function buildGroupedBar(input: string): string;

    export function buildHbar(input: string): string;

    export function buildHeatmap(input: string): string;

    export function buildHeatmap3dChart(input: string): string;

    export function buildHistogram(input: string): string;

    export function buildHistogramOverlay(input: string): string;

    export function buildHoverJson(input: string): string;

    export function buildHtmlChart(input: string): string;

    export function buildKde3dChart(input: string): string;

    export function buildKdeChart(input: string): string;

    export function buildKmeansChart(input: string): string;

    export function buildLine(input: string): string;

    export function buildLine3dChart(input: string): string;

    export function buildLineChart(input: string): string;

    export function buildLollipop3dChart(input: string): string;

    export function buildLollipopChart(input: string): string;

    export function buildMultilineChart(input: string): string;

    export function buildParallel(input: string): string;

    export function buildPie(input: string): string;

    export function buildPie3dChart(input: string): string;

    export function buildPieChart(input: string): string;

    export function buildRadar3dChart(input: string): string;

    export function buildRadarChart(input: string): string;

    export function buildRidgeline3dChart(input: string): string;

    export function buildRidgelineChart(input: string): string;

    export function buildScatter3dChart(input: string): string;

    export function buildScatterChart(input: string): string;

    export function buildSlideshow(input: string): string;

    export function buildSlope(input: string): string;

    export function buildStackedBar(input: string): string;

    export function buildStackedBar3dChart(input: string): string;

    export function buildSunburst(input: string): string;

    export function buildSunburst3dChart(input: string): string;

    export function buildTreemap(input: string): string;

    export function buildViolin(input: string): string;

    export function buildViolin3dChart(input: string): string;

    export function buildWaterfall(input: string): string;

    export function buildWordcloud(input: string): string;

    export function chartAppend(input: string): string;

    export function chartDiff(input: string): string;

    export function chartInfo(input: string): string;

    export function demo(input: string): string;

    export function downsampleLttb(input: string): string;

    export function driftKs(input: string): string;

    export function exportDataUrl(input: string): string;

    export function exportHtmlFile(input: string): string;

    export function exportSvg(input: string): string;

    export function mlDbscanFitPredict(input: string): string;

    export function mlDecisionTreeClassifier(input: string): string;

    export function mlDecisionTreeRegressor(input: string): string;

    export function mlElasticNet(input: string): string;

    export function mlFitTransform(input: string): string;

    export function mlGradientBoostingClassifier(input: string): string;

    export function mlGradientBoostingRegressor(input: string): string;

    export function mlIsolationForest(input: string): string;

    export function mlKfoldSplit(input: string): string;

    export function mlKmeansFitPredict(input: string): string;

    export function mlKnnClassifier(input: string): string;

    export function mlKnnRegressor(input: string): string;

    export function mlLasso(input: string): string;

    export function mlLinearRegression(input: string): string;

    export function mlLoadModel(input: string): string;

    export function mlLogisticRegression(input: string): string;

    export function mlMetricCurve(input: string): string;

    export function mlMetricScore(input: string): string;

    export function mlPermutationImportance(input: string): string;

    export function mlRandomForestClassifier(input: string): string;

    export function mlRandomForestRegressor(input: string): string;

    export function mlRidge(input: string): string;

    export function mlSaveModel(input: string): string;

    export function plotChart(input: string): string;

    export function resetGlobalBackground(input: string): string;

    export function resetTheme(input: string): string;

    export function setGlobalBackground(input: string): string;

    export function setTheme(input: string): string;

    export function themes(input: string): string;

    export function validateInput(input: string): string;

}
declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly buildAreaChart: (a: number, b: number) => [number, number];
    readonly buildBar: (a: number, b: number) => [number, number];
    readonly buildBar3dChart: (a: number, b: number) => [number, number];
    readonly buildBarChart: (a: number, b: number) => [number, number];
    readonly buildBoxplot: (a: number, b: number) => [number, number];
    readonly buildBubble: (a: number, b: number) => [number, number];
    readonly buildBubble3dChart: (a: number, b: number) => [number, number];
    readonly buildBubbleMap: (a: number, b: number) => [number, number];
    readonly buildBullet: (a: number, b: number) => [number, number];
    readonly buildCandlestick: (a: number, b: number) => [number, number];
    readonly buildCandlestick3dChart: (a: number, b: number) => [number, number];
    readonly buildChoropleth: (a: number, b: number) => [number, number];
    readonly buildDbscanChart: (a: number, b: number) => [number, number];
    readonly buildDbscanChart3d: (a: number, b: number) => [number, number];
    readonly buildDonutChart: (a: number, b: number) => [number, number];
    readonly buildDumbbell: (a: number, b: number) => [number, number];
    readonly buildDumbbell3dChart: (a: number, b: number) => [number, number];
    readonly buildFunnel: (a: number, b: number) => [number, number];
    readonly buildFunnel3dChart: (a: number, b: number) => [number, number];
    readonly buildGauge: (a: number, b: number) => [number, number];
    readonly buildGlobe3dChart: (a: number, b: number) => [number, number];
    readonly buildGrid: (a: number, b: number) => [number, number];
    readonly buildGroupedBar: (a: number, b: number) => [number, number];
    readonly buildHbar: (a: number, b: number) => [number, number];
    readonly buildHeatmap: (a: number, b: number) => [number, number];
    readonly buildHeatmap3dChart: (a: number, b: number) => [number, number];
    readonly buildHistogram: (a: number, b: number) => [number, number];
    readonly buildHistogramOverlay: (a: number, b: number) => [number, number];
    readonly buildHoverJson: (a: number, b: number) => [number, number];
    readonly buildHtmlChart: (a: number, b: number) => [number, number];
    readonly buildKde3dChart: (a: number, b: number) => [number, number];
    readonly buildKdeChart: (a: number, b: number) => [number, number];
    readonly buildKmeansChart: (a: number, b: number) => [number, number];
    readonly buildLine: (a: number, b: number) => [number, number];
    readonly buildLine3dChart: (a: number, b: number) => [number, number];
    readonly buildLineChart: (a: number, b: number) => [number, number];
    readonly buildLollipop3dChart: (a: number, b: number) => [number, number];
    readonly buildLollipopChart: (a: number, b: number) => [number, number];
    readonly buildMultilineChart: (a: number, b: number) => [number, number];
    readonly buildParallel: (a: number, b: number) => [number, number];
    readonly buildPie: (a: number, b: number) => [number, number];
    readonly buildPie3dChart: (a: number, b: number) => [number, number];
    readonly buildPieChart: (a: number, b: number) => [number, number];
    readonly buildRadar3dChart: (a: number, b: number) => [number, number];
    readonly buildRadarChart: (a: number, b: number) => [number, number];
    readonly buildRidgeline3dChart: (a: number, b: number) => [number, number];
    readonly buildRidgelineChart: (a: number, b: number) => [number, number];
    readonly buildScatter3dChart: (a: number, b: number) => [number, number];
    readonly buildScatterChart: (a: number, b: number) => [number, number];
    readonly buildSlideshow: (a: number, b: number) => [number, number];
    readonly buildSlope: (a: number, b: number) => [number, number];
    readonly buildStackedBar: (a: number, b: number) => [number, number];
    readonly buildStackedBar3dChart: (a: number, b: number) => [number, number];
    readonly buildSunburst: (a: number, b: number) => [number, number];
    readonly buildSunburst3dChart: (a: number, b: number) => [number, number];
    readonly buildTreemap: (a: number, b: number) => [number, number];
    readonly buildViolin: (a: number, b: number) => [number, number];
    readonly buildViolin3dChart: (a: number, b: number) => [number, number];
    readonly buildWaterfall: (a: number, b: number) => [number, number];
    readonly buildWordcloud: (a: number, b: number) => [number, number];
    readonly chartAppend: (a: number, b: number) => [number, number];
    readonly chartDiff: (a: number, b: number) => [number, number];
    readonly chartInfo: (a: number, b: number) => [number, number];
    readonly demo: (a: number, b: number) => [number, number];
    readonly downsampleLttb: (a: number, b: number) => [number, number];
    readonly driftKs: (a: number, b: number) => [number, number];
    readonly exportDataUrl: (a: number, b: number) => [number, number];
    readonly exportHtmlFile: (a: number, b: number) => [number, number];
    readonly exportSvg: (a: number, b: number) => [number, number];
    readonly mlDbscanFitPredict: (a: number, b: number) => [number, number];
    readonly mlDecisionTreeClassifier: (a: number, b: number) => [number, number];
    readonly mlDecisionTreeRegressor: (a: number, b: number) => [number, number];
    readonly mlElasticNet: (a: number, b: number) => [number, number];
    readonly mlFitTransform: (a: number, b: number) => [number, number];
    readonly mlGradientBoostingClassifier: (a: number, b: number) => [number, number];
    readonly mlGradientBoostingRegressor: (a: number, b: number) => [number, number];
    readonly mlIsolationForest: (a: number, b: number) => [number, number];
    readonly mlKfoldSplit: (a: number, b: number) => [number, number];
    readonly mlKmeansFitPredict: (a: number, b: number) => [number, number];
    readonly mlKnnClassifier: (a: number, b: number) => [number, number];
    readonly mlKnnRegressor: (a: number, b: number) => [number, number];
    readonly mlLasso: (a: number, b: number) => [number, number];
    readonly mlLinearRegression: (a: number, b: number) => [number, number];
    readonly mlLoadModel: (a: number, b: number) => [number, number];
    readonly mlLogisticRegression: (a: number, b: number) => [number, number];
    readonly mlMetricCurve: (a: number, b: number) => [number, number];
    readonly mlMetricScore: (a: number, b: number) => [number, number];
    readonly mlPermutationImportance: (a: number, b: number) => [number, number];
    readonly mlRandomForestClassifier: (a: number, b: number) => [number, number];
    readonly mlRandomForestRegressor: (a: number, b: number) => [number, number];
    readonly mlRidge: (a: number, b: number) => [number, number];
    readonly mlSaveModel: (a: number, b: number) => [number, number];
    readonly plotChart: (a: number, b: number) => [number, number];
    readonly resetGlobalBackground: (a: number, b: number) => [number, number];
    readonly resetTheme: (a: number, b: number) => [number, number];
    readonly sera_create_fast_chart: (a: number, b: number, c: number, d: number, e: number) => number;
    readonly sera_fast_build_html: (a: number, b: number, c: number, d: number) => number;
    readonly sera_fast_chart_free: (a: number) => void;
    readonly sera_fast_export_create: (a: number, b: number, c: number) => number;
    readonly sera_fast_export_destroy: (a: number) => void;
    readonly sera_get_chart_group_types: (a: number, b: number) => number;
    readonly sera_get_chart_type: (a: number) => number;
    readonly sera_get_current_chart_group: () => number;
    readonly sera_get_current_plot_3d_group: () => number;
    readonly sera_get_plot_3d_group_types: (a: number, b: number) => number;
    readonly sera_get_plot_3d_type: (a: number) => number;
    readonly sera_html_export_build: (a: number) => number;
    readonly sera_html_export_create: (a: number, b: number, c: number, d: number) => number;
    readonly sera_html_export_create_dark: (a: number, b: number) => number;
    readonly sera_html_export_create_light: (a: number, b: number) => number;
    readonly sera_html_export_free: (a: number) => void;
    readonly sera_html_export_save: (a: number, b: number) => number;
    readonly sera_html_export_set_data: (a: number, b: number, c: number, d: number) => void;
    readonly sera_html_export_set_state_json: (a: number, b: number) => number;
    readonly sera_html_export_set_svg: (a: number, b: number) => void;
    readonly sera_html_export_set_title: (a: number, b: number) => void;
    readonly sera_list_chart_groups: (a: number) => number;
    readonly sera_list_chart_types: () => number;
    readonly sera_list_plot_3d_groups: (a: number) => number;
    readonly sera_list_plot_3d_types: () => number;
    readonly sera_register_chart_group: (a: number, b: number, c: number) => number;
    readonly sera_register_chart_type: (a: number, b: number, c: number) => number;
    readonly sera_register_plot_3d_group: (a: number, b: number, c: number) => number;
    readonly sera_register_plot_3d_type: (a: number, b: number, c: number) => number;
    readonly sera_set_current_chart_group: (a: number) => number;
    readonly sera_set_current_plot_3d_group: (a: number) => number;
    readonly sera_state_storage_create: () => number;
    readonly sera_state_storage_free: (a: number) => void;
    readonly sera_state_storage_load: (a: number, b: number) => number;
    readonly sera_state_storage_save: (a: number, b: number, c: number) => number;
    readonly setGlobalBackground: (a: number, b: number) => [number, number];
    readonly setTheme: (a: number, b: number) => [number, number];
    readonly themes: (a: number, b: number) => [number, number];
    readonly validateInput: (a: number, b: number) => [number, number];
    readonly sera_free_html: (a: number) => void;
    readonly sera_string_free_html: (a: number) => void;
    readonly sera_html_export_build_fast: (a: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
declare function wasm_bindgen (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
