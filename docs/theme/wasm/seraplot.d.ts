declare namespace wasm_bindgen {
    /* tslint:disable */
    /* eslint-disable */

    export class Chart {
        private constructor();
        free(): void;
        [Symbol.dispose](): void;
    }

    export function accessiblePalette(input: string): string;

    export function aliasAdd(method: string, alias: string): boolean;

    export function aliasList(): string;

    export function aliasLoadJson(json: string): boolean;

    export function aliasRemove(method: string, alias: string): boolean;

    export function aliasReset(): void;

    export function aliasResolve(name: string): string | undefined;

    export function applyChartMethod(html: string, name: string, args_json: string): string;

    export function buildArcDiagram(input: string): string;

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

    export function buildChord(input: string): string;

    export function buildChoropleth(input: string): string;

    export function buildCirclePack(input: string): string;

    export function buildConeChart(input: string): string;

    export function buildCorrelogram(input: string): string;

    export function buildDbscanChart(input: string): string;

    export function buildDbscanChart3d(input: string): string;

    export function buildDendrogram(input: string): string;

    export function buildDonutChart(input: string): string;

    export function buildDumbbell(input: string): string;

    export function buildDumbbell3dChart(input: string): string;

    export function buildEventplot(input: string): string;

    export function buildFacet(input: string): string;

    export function buildFunnel(input: string): string;

    export function buildFunnel3dChart(input: string): string;

    export function buildGantt(input: string): string;

    export function buildGauge(input: string): string;

    export function buildGlobe3dChart(input: string): string;

    export function buildGroupedBar(input: string): string;

    export function buildHbar(input: string): string;

    export function buildHeatmap(input: string): string;

    export function buildHeatmap3dChart(input: string): string;

    export function buildHexbin(input: string): string;

    export function buildHistogram(input: string): string;

    export function buildHive(input: string): string;

    export function buildIcicle(input: string): string;

    export function buildIsosurfaceChart(input: string): string;

    export function buildJoint(input: string): string;

    export function buildKde3dChart(input: string): string;

    export function buildKdeChart(input: string): string;

    export function buildKmeansChart(input: string): string;

    export function buildLine(input: string): string;

    export function buildLine3dChart(input: string): string;

    export function buildLineChart(input: string): string;

    export function buildLollipop3dChart(input: string): string;

    export function buildLollipopChart(input: string): string;

    export function buildMesh3dChart(input: string): string;

    export function buildMultilineChart(input: string): string;

    export function buildOrbita(input: string): string;

    export function buildParallel(input: string): string;

    export function buildParcats(input: string): string;

    export function buildPie(input: string): string;

    export function buildPie3dChart(input: string): string;

    export function buildPieChart(input: string): string;

    export function buildPlotWeb(input: string): string;

    export function buildPulse(input: string): string;

    export function buildRadar3dChart(input: string): string;

    export function buildRadarChart(input: string): string;

    export function buildRidgeline3dChart(input: string): string;

    export function buildRidgelineChart(input: string): string;

    export function buildSankey(input: string): string;

    export function buildScatter3dChart(input: string): string;

    export function buildScatterChart(input: string): string;

    export function buildScatterTernary(input: string): string;

    export function buildSlope(input: string): string;

    export function buildSplom(input: string): string;

    export function buildStackedBar(input: string): string;

    export function buildStackedBar3dChart(input: string): string;

    export function buildStackplot(input: string): string;

    export function buildStreamtubeChart(input: string): string;

    export function buildSunburst(input: string): string;

    export function buildSunburst3dChart(input: string): string;

    export function buildSurface3dChart(input: string): string;

    export function buildTreemap(input: string): string;

    export function buildVenn(input: string): string;

    export function buildViolin(input: string): string;

    export function buildViolin3dChart(input: string): string;

    export function buildVoxelsChart(input: string): string;

    export function buildWaterfall(input: string): string;

    export function buildWireframe3dChart(input: string): string;

    export function buildWordcloud(input: string): string;

    export function chartAliases(): string;

    export function chartAppend(input: string): string;

    export function chartDiff(input: string): string;

    export function chartThemes(): string;

    export function chartVariants(): string;

    export function csvChunkRead(input: string): string;

    export function csvCountRows(input: string): string;

    export function demo(input: string): string;

    export function docs(): string;

    export function downsampleLttb(input: string): string;

    export function driftKs(input: string): string;

    export function exportHtmlFile(input: string): string;

    export function mlAdaboostClassifier(input: string): string;

    export function mlAdaboostRegressor(input: string): string;

    export function mlBernoulliNb(input: string): string;

    export function mlCrossValScore(input: string): string;

    export function mlDbscanFitPredict(input: string): string;

    export function mlDecisionTreeClassifier(input: string): string;

    export function mlDecisionTreeRegressor(input: string): string;

    export function mlElasticNet(input: string): string;

    export function mlFitTransform(input: string): string;

    export function mlGaussianNb(input: string): string;

    export function mlGradientBoostingClassifier(input: string): string;

    export function mlGradientBoostingRegressor(input: string): string;

    export function mlGridSearchCv(input: string): string;

    export function mlIsolationForest(input: string): string;

    export function mlKfoldSplit(input: string): string;

    export function mlKmeansFitPredict(input: string): string;

    export function mlKnnClassifier(input: string): string;

    export function mlKnnRegressor(input: string): string;

    export function mlLasso(input: string): string;

    export function mlLinearRegression(input: string): string;

    export function mlLinearSvc(input: string): string;

    export function mlLinearSvr(input: string): string;

    export function mlLoadModel(input: string): string;

    export function mlLogisticRegression(input: string): string;

    export function mlMetricCurve(input: string): string;

    export function mlMetricScore(input: string): string;

    export function mlMinmaxScaler(input: string): string;

    export function mlMultinomialNb(input: string): string;

    export function mlNearestCentroid(input: string): string;

    export function mlPca(input: string): string;

    export function mlPermutationImportance(input: string): string;

    export function mlRandomForestClassifier(input: string): string;

    export function mlRandomForestRegressor(input: string): string;

    export function mlRidge(input: string): string;

    export function mlRidgeClassifier(input: string): string;

    export function mlRobustScaler(input: string): string;

    export function mlSaveModel(input: string): string;

    export function mlSgdClassifier(input: string): string;

    export function mlSgdRegressor(input: string): string;

    export function mlStandardScaler(input: string): string;

    export function mlTruncatedSvd(input: string): string;

    export function params(input: string): string;

    export function requiredParams(input: string): string;

    export function resetGlobalBackground(input: string): string;

    export function resetTheme(input: string): string;

    export function scalePlan(input: string): string;

    export function scenes3d(): string;

    export function setGlobalBackground(input: string): string;

    export function setTheme(input: string): string;

    export function systemProfile(input: string): string;

    export function themes(input: string): string;

    export function trueRequiredParams(input: string): string;

    export function validateInput(input: string): string;

    export function wasm_adaptive_degrade_level(): any;

    export function wasm_apply_color_bindings_html(html: string): any;

    export function wasm_chart_variants(): any;

    export function wasm_clear_color_bindings(): any;

    export function wasm_color_density_html(html: string): any;

    export function wasm_crosshair_html(html: string): any;

    export function wasm_cut_bars_html(html: string, step: number | null | undefined, gap: number, color?: string | null): any;

    export function wasm_demo(chart: string, variant?: string | null): any;

    export function wasm_demos(): any;

    export function wasm_desaturate_html(html: string, indices: Uint32Array | null | undefined, factor: number): any;

    export function wasm_doc(name: string): any;

    export function wasm_docs(): any;

    export function wasm_draw_tool_html(html: string, color: string): any;

    export function wasm_export_button_html(html: string): any;

    export function wasm_flip_html(html: string): any;

    export function wasm_get_metrics(): any;

    export function wasm_grid_at_html(html: string, value: number, color: string, label?: string | null): any;

    export function wasm_grid_x_html(html: string): any;

    export function wasm_grid_y_html(html: string): any;

    export function wasm_group_hover_opacity_html(html: string, dim: number): any;

    export function wasm_highlight_group_html(html: string, labels: string[], dim: number): any;

    export function wasm_hover_slots_html(html: string, slots_json: string): any;

    export function wasm_hw(): any;

    export function wasm_inject_css_html(html: string, css: string): any;

    export function wasm_inject_js_html(html: string, js: string): any;

    export function wasm_models(): any;

    export function wasm_models_for_category(category: string): any;

    export function wasm_models_for_domain(domain: string): any;

    export function wasm_no_axes_html(html: string): any;

    export function wasm_no_background_html(html: string): any;

    export function wasm_no_hover_html(html: string): any;

    export function wasm_no_legend_html(html: string): any;

    export function wasm_no_select_html(html: string): any;

    export function wasm_no_title_html(html: string): any;

    export function wasm_no_x_axis_html(html: string): any;

    export function wasm_no_y_axis_html(html: string): any;

    export function wasm_params(chart?: string | null, variant?: string | null): any;

    export function wasm_required_params(chart?: string | null, variant?: string | null): any;

    export function wasm_reset_config(): any;

    export function wasm_reset_global_background(): any;

    export function wasm_reset_perf_state(): any;

    export function wasm_reset_theme(): any;

    export function wasm_responsive_html(html: string): any;

    export function wasm_set_adaptive_retry(on: boolean): any;

    export function wasm_set_auto_display(enabled: boolean): any;

    export function wasm_set_global_background(color: string): any;

    export function wasm_show_grid_html(html: string): any;

    export function wasm_sparse_grid_html(html: string): any;

    export function wasm_telemetry_consent(enabled: boolean): any;

    export function wasm_telemetry_path(): any;

    export function wasm_themes(): any;

    export function wasm_true_required_params(chart?: string | null, variant?: string | null): any;

    export function wasm_zoom_html(html: string): any;

}
declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_chart_free: (a: number, b: number) => void;
    readonly accessiblePalette: (a: number, b: number) => [number, number];
    readonly aliasAdd: (a: number, b: number, c: number, d: number) => number;
    readonly aliasList: () => [number, number];
    readonly aliasLoadJson: (a: number, b: number) => number;
    readonly aliasRemove: (a: number, b: number, c: number, d: number) => number;
    readonly aliasResolve: (a: number, b: number) => [number, number];
    readonly applyChartMethod: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
    readonly buildArcDiagram: (a: number, b: number) => [number, number];
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
    readonly buildChord: (a: number, b: number) => [number, number];
    readonly buildChoropleth: (a: number, b: number) => [number, number];
    readonly buildCirclePack: (a: number, b: number) => [number, number];
    readonly buildConeChart: (a: number, b: number) => [number, number];
    readonly buildCorrelogram: (a: number, b: number) => [number, number];
    readonly buildDbscanChart: (a: number, b: number) => [number, number];
    readonly buildDbscanChart3d: (a: number, b: number) => [number, number];
    readonly buildDendrogram: (a: number, b: number) => [number, number];
    readonly buildDonutChart: (a: number, b: number) => [number, number];
    readonly buildDumbbell: (a: number, b: number) => [number, number];
    readonly buildDumbbell3dChart: (a: number, b: number) => [number, number];
    readonly buildEventplot: (a: number, b: number) => [number, number];
    readonly buildFacet: (a: number, b: number) => [number, number];
    readonly buildFunnel: (a: number, b: number) => [number, number];
    readonly buildFunnel3dChart: (a: number, b: number) => [number, number];
    readonly buildGantt: (a: number, b: number) => [number, number];
    readonly buildGauge: (a: number, b: number) => [number, number];
    readonly buildGlobe3dChart: (a: number, b: number) => [number, number];
    readonly buildGroupedBar: (a: number, b: number) => [number, number];
    readonly buildHbar: (a: number, b: number) => [number, number];
    readonly buildHeatmap: (a: number, b: number) => [number, number];
    readonly buildHeatmap3dChart: (a: number, b: number) => [number, number];
    readonly buildHexbin: (a: number, b: number) => [number, number];
    readonly buildHistogram: (a: number, b: number) => [number, number];
    readonly buildHive: (a: number, b: number) => [number, number];
    readonly buildIcicle: (a: number, b: number) => [number, number];
    readonly buildIsosurfaceChart: (a: number, b: number) => [number, number];
    readonly buildJoint: (a: number, b: number) => [number, number];
    readonly buildKde3dChart: (a: number, b: number) => [number, number];
    readonly buildKdeChart: (a: number, b: number) => [number, number];
    readonly buildKmeansChart: (a: number, b: number) => [number, number];
    readonly buildLine: (a: number, b: number) => [number, number];
    readonly buildLine3dChart: (a: number, b: number) => [number, number];
    readonly buildLineChart: (a: number, b: number) => [number, number];
    readonly buildLollipop3dChart: (a: number, b: number) => [number, number];
    readonly buildLollipopChart: (a: number, b: number) => [number, number];
    readonly buildMesh3dChart: (a: number, b: number) => [number, number];
    readonly buildMultilineChart: (a: number, b: number) => [number, number];
    readonly buildOrbita: (a: number, b: number) => [number, number];
    readonly buildParallel: (a: number, b: number) => [number, number];
    readonly buildParcats: (a: number, b: number) => [number, number];
    readonly buildPie: (a: number, b: number) => [number, number];
    readonly buildPie3dChart: (a: number, b: number) => [number, number];
    readonly buildPieChart: (a: number, b: number) => [number, number];
    readonly buildPlotWeb: (a: number, b: number) => [number, number];
    readonly buildPulse: (a: number, b: number) => [number, number];
    readonly buildRadar3dChart: (a: number, b: number) => [number, number];
    readonly buildRadarChart: (a: number, b: number) => [number, number];
    readonly buildRidgeline3dChart: (a: number, b: number) => [number, number];
    readonly buildRidgelineChart: (a: number, b: number) => [number, number];
    readonly buildSankey: (a: number, b: number) => [number, number];
    readonly buildScatter3dChart: (a: number, b: number) => [number, number];
    readonly buildScatterChart: (a: number, b: number) => [number, number];
    readonly buildScatterTernary: (a: number, b: number) => [number, number];
    readonly buildSlope: (a: number, b: number) => [number, number];
    readonly buildSplom: (a: number, b: number) => [number, number];
    readonly buildStackedBar: (a: number, b: number) => [number, number];
    readonly buildStackedBar3dChart: (a: number, b: number) => [number, number];
    readonly buildStackplot: (a: number, b: number) => [number, number];
    readonly buildStreamtubeChart: (a: number, b: number) => [number, number];
    readonly buildSunburst: (a: number, b: number) => [number, number];
    readonly buildSunburst3dChart: (a: number, b: number) => [number, number];
    readonly buildSurface3dChart: (a: number, b: number) => [number, number];
    readonly buildTreemap: (a: number, b: number) => [number, number];
    readonly buildVenn: (a: number, b: number) => [number, number];
    readonly buildViolin: (a: number, b: number) => [number, number];
    readonly buildViolin3dChart: (a: number, b: number) => [number, number];
    readonly buildVoxelsChart: (a: number, b: number) => [number, number];
    readonly buildWaterfall: (a: number, b: number) => [number, number];
    readonly buildWireframe3dChart: (a: number, b: number) => [number, number];
    readonly buildWordcloud: (a: number, b: number) => [number, number];
    readonly chartAliases: () => [number, number];
    readonly chartAppend: (a: number, b: number) => [number, number];
    readonly chartDiff: (a: number, b: number) => [number, number];
    readonly chartThemes: () => [number, number];
    readonly chartVariants: () => [number, number];
    readonly csvChunkRead: (a: number, b: number) => [number, number];
    readonly csvCountRows: (a: number, b: number) => [number, number];
    readonly demo: (a: number, b: number) => [number, number];
    readonly docs: () => [number, number];
    readonly downsampleLttb: (a: number, b: number) => [number, number];
    readonly driftKs: (a: number, b: number) => [number, number];
    readonly exportHtmlFile: (a: number, b: number) => [number, number];
    readonly mlAdaboostClassifier: (a: number, b: number) => [number, number];
    readonly mlAdaboostRegressor: (a: number, b: number) => [number, number];
    readonly mlBernoulliNb: (a: number, b: number) => [number, number];
    readonly mlCrossValScore: (a: number, b: number) => [number, number];
    readonly mlDbscanFitPredict: (a: number, b: number) => [number, number];
    readonly mlDecisionTreeClassifier: (a: number, b: number) => [number, number];
    readonly mlDecisionTreeRegressor: (a: number, b: number) => [number, number];
    readonly mlElasticNet: (a: number, b: number) => [number, number];
    readonly mlFitTransform: (a: number, b: number) => [number, number];
    readonly mlGaussianNb: (a: number, b: number) => [number, number];
    readonly mlGradientBoostingClassifier: (a: number, b: number) => [number, number];
    readonly mlGradientBoostingRegressor: (a: number, b: number) => [number, number];
    readonly mlGridSearchCv: (a: number, b: number) => [number, number];
    readonly mlIsolationForest: (a: number, b: number) => [number, number];
    readonly mlKfoldSplit: (a: number, b: number) => [number, number];
    readonly mlKmeansFitPredict: (a: number, b: number) => [number, number];
    readonly mlKnnClassifier: (a: number, b: number) => [number, number];
    readonly mlKnnRegressor: (a: number, b: number) => [number, number];
    readonly mlLasso: (a: number, b: number) => [number, number];
    readonly mlLinearRegression: (a: number, b: number) => [number, number];
    readonly mlLinearSvc: (a: number, b: number) => [number, number];
    readonly mlLinearSvr: (a: number, b: number) => [number, number];
    readonly mlLoadModel: (a: number, b: number) => [number, number];
    readonly mlLogisticRegression: (a: number, b: number) => [number, number];
    readonly mlMetricCurve: (a: number, b: number) => [number, number];
    readonly mlMetricScore: (a: number, b: number) => [number, number];
    readonly mlMinmaxScaler: (a: number, b: number) => [number, number];
    readonly mlMultinomialNb: (a: number, b: number) => [number, number];
    readonly mlNearestCentroid: (a: number, b: number) => [number, number];
    readonly mlPca: (a: number, b: number) => [number, number];
    readonly mlPermutationImportance: (a: number, b: number) => [number, number];
    readonly mlRandomForestClassifier: (a: number, b: number) => [number, number];
    readonly mlRandomForestRegressor: (a: number, b: number) => [number, number];
    readonly mlRidge: (a: number, b: number) => [number, number];
    readonly mlRidgeClassifier: (a: number, b: number) => [number, number];
    readonly mlRobustScaler: (a: number, b: number) => [number, number];
    readonly mlSaveModel: (a: number, b: number) => [number, number];
    readonly mlSgdClassifier: (a: number, b: number) => [number, number];
    readonly mlSgdRegressor: (a: number, b: number) => [number, number];
    readonly mlStandardScaler: (a: number, b: number) => [number, number];
    readonly mlTruncatedSvd: (a: number, b: number) => [number, number];
    readonly params: (a: number, b: number) => [number, number];
    readonly requiredParams: (a: number, b: number) => [number, number];
    readonly resetGlobalBackground: (a: number, b: number) => [number, number];
    readonly resetTheme: (a: number, b: number) => [number, number];
    readonly scalePlan: (a: number, b: number) => [number, number];
    readonly scenes3d: () => [number, number];
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
    readonly systemProfile: (a: number, b: number) => [number, number];
    readonly themes: (a: number, b: number) => [number, number];
    readonly trueRequiredParams: (a: number, b: number) => [number, number];
    readonly validateInput: (a: number, b: number) => [number, number];
    readonly wasm_adaptive_degrade_level: () => any;
    readonly wasm_apply_color_bindings_html: (a: number, b: number) => any;
    readonly wasm_chart_variants: () => any;
    readonly wasm_color_density_html: (a: number, b: number) => any;
    readonly wasm_crosshair_html: (a: number, b: number) => any;
    readonly wasm_cut_bars_html: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => any;
    readonly wasm_demo: (a: number, b: number, c: number, d: number) => any;
    readonly wasm_demos: () => any;
    readonly wasm_desaturate_html: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly wasm_doc: (a: number, b: number) => any;
    readonly wasm_docs: () => any;
    readonly wasm_draw_tool_html: (a: number, b: number, c: number, d: number) => any;
    readonly wasm_export_button_html: (a: number, b: number) => any;
    readonly wasm_flip_html: (a: number, b: number) => any;
    readonly wasm_get_metrics: () => any;
    readonly wasm_grid_at_html: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => any;
    readonly wasm_grid_x_html: (a: number, b: number) => any;
    readonly wasm_grid_y_html: (a: number, b: number) => any;
    readonly wasm_group_hover_opacity_html: (a: number, b: number, c: number) => any;
    readonly wasm_highlight_group_html: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly wasm_hover_slots_html: (a: number, b: number, c: number, d: number) => any;
    readonly wasm_hw: () => any;
    readonly wasm_inject_css_html: (a: number, b: number, c: number, d: number) => any;
    readonly wasm_inject_js_html: (a: number, b: number, c: number, d: number) => any;
    readonly wasm_models: () => any;
    readonly wasm_models_for_category: (a: number, b: number) => any;
    readonly wasm_models_for_domain: (a: number, b: number) => any;
    readonly wasm_no_axes_html: (a: number, b: number) => any;
    readonly wasm_no_background_html: (a: number, b: number) => any;
    readonly wasm_no_hover_html: (a: number, b: number) => any;
    readonly wasm_no_legend_html: (a: number, b: number) => any;
    readonly wasm_no_select_html: (a: number, b: number) => any;
    readonly wasm_no_title_html: (a: number, b: number) => any;
    readonly wasm_no_x_axis_html: (a: number, b: number) => any;
    readonly wasm_no_y_axis_html: (a: number, b: number) => any;
    readonly wasm_params: (a: number, b: number, c: number, d: number) => any;
    readonly wasm_required_params: (a: number, b: number, c: number, d: number) => any;
    readonly wasm_responsive_html: (a: number, b: number) => any;
    readonly wasm_set_adaptive_retry: (a: number) => any;
    readonly wasm_set_auto_display: (a: number) => any;
    readonly wasm_set_global_background: (a: number, b: number) => any;
    readonly wasm_show_grid_html: (a: number, b: number) => any;
    readonly wasm_sparse_grid_html: (a: number, b: number) => any;
    readonly wasm_telemetry_consent: (a: number) => any;
    readonly wasm_telemetry_path: () => any;
    readonly wasm_themes: () => any;
    readonly wasm_true_required_params: (a: number, b: number, c: number, d: number) => any;
    readonly wasm_zoom_html: (a: number, b: number) => any;
    readonly sera_free_html: (a: number) => void;
    readonly sera_string_free_html: (a: number) => void;
    readonly sera_html_export_build_fast: (a: number) => number;
    readonly aliasReset: () => void;
    readonly wasm_reset_config: () => any;
    readonly wasm_reset_global_background: () => any;
    readonly wasm_reset_theme: () => any;
    readonly wasm_clear_color_bindings: () => any;
    readonly wasm_reset_perf_state: () => any;
    readonly __wbindgen_malloc_command_export: (a: number, b: number) => number;
    readonly __wbindgen_realloc_command_export: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free_command_export: (a: number, b: number, c: number) => void;
    readonly __externref_table_alloc_command_export: () => number;
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
