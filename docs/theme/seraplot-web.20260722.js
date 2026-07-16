let wasm_bindgen = (function(exports) {
    let script_src;
    if (typeof document !== 'undefined' && document.currentScript !== null) {
        script_src = new URL(document.currentScript.src, location.href).toString();
    }

    class Chart {
        __destroy_into_raw() {
            const ptr = this.__wbg_ptr;
            this.__wbg_ptr = 0;
            ChartFinalization.unregister(this);
            return ptr;
        }
        free() {
            const ptr = this.__destroy_into_raw();
            wasm.__wbg_chart_free(ptr, 0);
        }
    }
    if (Symbol.dispose) Chart.prototype[Symbol.dispose] = Chart.prototype.free;
    exports.Chart = Chart;

    /**
     * @param {string} method
     * @param {string} alias
     * @returns {boolean}
     */
    function aliasAdd(method, alias) {
        const ptr0 = passStringToWasm0(method, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(alias, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.aliasAdd(ptr0, len0, ptr1, len1);
        return ret !== 0;
    }
    exports.aliasAdd = aliasAdd;

    /**
     * @returns {string}
     */
    function aliasList() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.aliasList();
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred1_0, deferred1_1, 1);
        }
    }
    exports.aliasList = aliasList;

    /**
     * @param {string} json
     * @returns {boolean}
     */
    function aliasLoadJson(json) {
        const ptr0 = passStringToWasm0(json, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.aliasLoadJson(ptr0, len0);
        return ret !== 0;
    }
    exports.aliasLoadJson = aliasLoadJson;

    /**
     * @param {string} method
     * @param {string} alias
     * @returns {boolean}
     */
    function aliasRemove(method, alias) {
        const ptr0 = passStringToWasm0(method, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(alias, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.aliasRemove(ptr0, len0, ptr1, len1);
        return ret !== 0;
    }
    exports.aliasRemove = aliasRemove;

    function aliasReset() {
        wasm.aliasReset();
    }
    exports.aliasReset = aliasReset;

    /**
     * @param {string} name
     * @returns {string | undefined}
     */
    function aliasResolve(name) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.aliasResolve(ptr0, len0);
        let v2;
        if (ret[0] !== 0) {
            v2 = getStringFromWasm0(ret[0], ret[1]).slice();
            wasm.__wbindgen_free_command_export(ret[0], ret[1] * 1, 1);
        }
        return v2;
    }
    exports.aliasResolve = aliasResolve;

    /**
     * @param {string} html
     * @param {string} name
     * @param {string} args_json
     * @returns {string}
     */
    function applyChartMethod(html, name, args_json) {
        let deferred4_0;
        let deferred4_1;
        try {
            const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(name, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len1 = WASM_VECTOR_LEN;
            const ptr2 = passStringToWasm0(args_json, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len2 = WASM_VECTOR_LEN;
            const ret = wasm.applyChartMethod(ptr0, len0, ptr1, len1, ptr2, len2);
            deferred4_0 = ret[0];
            deferred4_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred4_0, deferred4_1, 1);
        }
    }
    exports.applyChartMethod = applyChartMethod;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildAreaChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildAreaChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildAreaChart = buildAreaChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildBar(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildBar(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildBar = buildBar;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildBar3dChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildBar3dChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildBar3dChart = buildBar3dChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildBarChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildBarChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildBarChart = buildBarChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildBoxplot(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildBoxplot(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildBoxplot = buildBoxplot;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildBubble(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildBubble(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildBubble = buildBubble;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildBubble3dChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildBubble3dChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildBubble3dChart = buildBubble3dChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildBubbleMap(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildBubbleMap(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildBubbleMap = buildBubbleMap;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildBullet(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildBullet(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildBullet = buildBullet;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildCandlestick(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildCandlestick(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildCandlestick = buildCandlestick;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildCandlestick3dChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildCandlestick3dChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildCandlestick3dChart = buildCandlestick3dChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildChoropleth(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildChoropleth(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildChoropleth = buildChoropleth;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildDbscanChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildDbscanChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildDbscanChart = buildDbscanChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildDbscanChart3d(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildDbscanChart3d(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildDbscanChart3d = buildDbscanChart3d;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildDonutChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildDonutChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildDonutChart = buildDonutChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildDumbbell(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildDumbbell(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildDumbbell = buildDumbbell;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildDumbbell3dChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildDumbbell3dChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildDumbbell3dChart = buildDumbbell3dChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildFunnel(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildFunnel(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildFunnel = buildFunnel;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildFunnel3dChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildFunnel3dChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildFunnel3dChart = buildFunnel3dChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildGauge(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildGauge(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildGauge = buildGauge;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildGlobe3dChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildGlobe3dChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildGlobe3dChart = buildGlobe3dChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildGroupedBar(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildGroupedBar(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildGroupedBar = buildGroupedBar;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildHbar(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildHbar(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildHbar = buildHbar;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildHeatmap(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildHeatmap(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildHeatmap = buildHeatmap;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildHeatmap3dChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildHeatmap3dChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildHeatmap3dChart = buildHeatmap3dChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildHistogram(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildHistogram(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildHistogram = buildHistogram;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildKde3dChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildKde3dChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildKde3dChart = buildKde3dChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildKdeChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildKdeChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildKdeChart = buildKdeChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildKmeansChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildKmeansChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildKmeansChart = buildKmeansChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildLine(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildLine(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildLine = buildLine;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildLine3dChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildLine3dChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildLine3dChart = buildLine3dChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildLineChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildLineChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildLineChart = buildLineChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildLollipop3dChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildLollipop3dChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildLollipop3dChart = buildLollipop3dChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildLollipopChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildLollipopChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildLollipopChart = buildLollipopChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildMultilineChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildMultilineChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildMultilineChart = buildMultilineChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildParallel(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildParallel(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildParallel = buildParallel;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildPie(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildPie(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildPie = buildPie;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildPie3dChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildPie3dChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildPie3dChart = buildPie3dChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildPieChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildPieChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildPieChart = buildPieChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildRadar3dChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildRadar3dChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildRadar3dChart = buildRadar3dChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildRadarChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildRadarChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildRadarChart = buildRadarChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildRidgeline3dChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildRidgeline3dChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildRidgeline3dChart = buildRidgeline3dChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildRidgelineChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildRidgelineChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildRidgelineChart = buildRidgelineChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildScatter3dChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildScatter3dChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildScatter3dChart = buildScatter3dChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildScatterChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildScatterChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildScatterChart = buildScatterChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildSlope(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildSlope(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildSlope = buildSlope;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildStackedBar(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildStackedBar(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildStackedBar = buildStackedBar;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildStackedBar3dChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildStackedBar3dChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildStackedBar3dChart = buildStackedBar3dChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildSunburst(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildSunburst(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildSunburst = buildSunburst;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildSunburst3dChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildSunburst3dChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildSunburst3dChart = buildSunburst3dChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildTreemap(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildTreemap(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildTreemap = buildTreemap;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildViolin(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildViolin(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildViolin = buildViolin;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildViolin3dChart(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildViolin3dChart(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildViolin3dChart = buildViolin3dChart;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildWaterfall(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildWaterfall(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildWaterfall = buildWaterfall;

    /**
     * @param {string} input
     * @returns {string}
     */
    function buildWordcloud(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.buildWordcloud(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.buildWordcloud = buildWordcloud;

    /**
     * @returns {string}
     */
    function chartAliases() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.chartAliases();
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred1_0, deferred1_1, 1);
        }
    }
    exports.chartAliases = chartAliases;

    /**
     * @param {string} input
     * @returns {string}
     */
    function chartAppend(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.chartAppend(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.chartAppend = chartAppend;

    /**
     * @param {string} input
     * @returns {string}
     */
    function chartDiff(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.chartDiff(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.chartDiff = chartDiff;

    /**
     * @returns {string}
     */
    function chartThemes() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.chartThemes();
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred1_0, deferred1_1, 1);
        }
    }
    exports.chartThemes = chartThemes;

    /**
     * @returns {string}
     */
    function chartVariants() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.chartVariants();
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred1_0, deferred1_1, 1);
        }
    }
    exports.chartVariants = chartVariants;

    /**
     * @param {string} input
     * @returns {string}
     */
    function csvChunkRead(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.csvChunkRead(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.csvChunkRead = csvChunkRead;

    /**
     * @param {string} input
     * @returns {string}
     */
    function csvCountRows(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.csvCountRows(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.csvCountRows = csvCountRows;

    /**
     * @param {string} input
     * @returns {string}
     */
    function demo(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.demo(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.demo = demo;

    /**
     * @returns {string}
     */
    function docs() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.docs();
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred1_0, deferred1_1, 1);
        }
    }
    exports.docs = docs;

    /**
     * @param {string} input
     * @returns {string}
     */
    function downsampleLttb(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.downsampleLttb(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.downsampleLttb = downsampleLttb;

    /**
     * @param {string} input
     * @returns {string}
     */
    function driftKs(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.driftKs(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.driftKs = driftKs;

    /**
     * @param {string} input
     * @returns {string}
     */
    function exportHtmlFile(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.exportHtmlFile(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.exportHtmlFile = exportHtmlFile;

    /**
     * @param {string} input
     * @returns {string}
     */
    function params(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.params(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.params = params;

    /**
     * @param {string} input
     * @returns {string}
     */
    function requiredParams(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.requiredParams(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.requiredParams = requiredParams;

    /**
     * @param {string} input
     * @returns {string}
     */
    function resetGlobalBackground(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.resetGlobalBackground(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.resetGlobalBackground = resetGlobalBackground;

    /**
     * @param {string} input
     * @returns {string}
     */
    function resetTheme(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.resetTheme(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.resetTheme = resetTheme;

    /**
     * @param {string} input
     * @returns {string}
     */
    function scalePlan(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.scalePlan(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.scalePlan = scalePlan;

    /**
     * @returns {string}
     */
    function scenes3d() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.scenes3d();
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred1_0, deferred1_1, 1);
        }
    }
    exports.scenes3d = scenes3d;

    /**
     * @param {string} input
     * @returns {string}
     */
    function setGlobalBackground(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.setGlobalBackground(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.setGlobalBackground = setGlobalBackground;

    /**
     * @param {string} input
     * @returns {string}
     */
    function setTheme(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.setTheme(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.setTheme = setTheme;

    /**
     * @param {string} input
     * @returns {string}
     */
    function systemProfile(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.systemProfile(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.systemProfile = systemProfile;

    /**
     * @param {string} input
     * @returns {string}
     */
    function themes(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.themes(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.themes = themes;

    /**
     * @param {string} input
     * @returns {string}
     */
    function validateInput(input) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.validateInput(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
        }
    }
    exports.validateInput = validateInput;

    /**
     * @returns {any}
     */
    function wasm_adaptive_degrade_level() {
        const ret = wasm.wasm_adaptive_degrade_level();
        return ret;
    }
    exports.wasm_adaptive_degrade_level = wasm_adaptive_degrade_level;

    /**
     * @returns {any}
     */
    function wasm_chart_variants() {
        const ret = wasm.wasm_chart_variants();
        return ret;
    }
    exports.wasm_chart_variants = wasm_chart_variants;

    /**
     * @param {string} chart
     * @param {string | null} [variant]
     * @returns {any}
     */
    function wasm_demo(chart, variant) {
        const ptr0 = passStringToWasm0(chart, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        var ptr1 = isLikeNone(variant) ? 0 : passStringToWasm0(variant, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        var len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasm_demo(ptr0, len0, ptr1, len1);
        return ret;
    }
    exports.wasm_demo = wasm_demo;

    /**
     * @returns {any}
     */
    function wasm_demos() {
        const ret = wasm.wasm_demos();
        return ret;
    }
    exports.wasm_demos = wasm_demos;

    /**
     * @param {string} name
     * @returns {any}
     */
    function wasm_doc(name) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasm_doc(ptr0, len0);
        return ret;
    }
    exports.wasm_doc = wasm_doc;

    /**
     * @returns {any}
     */
    function wasm_docs() {
        const ret = wasm.wasm_docs();
        return ret;
    }
    exports.wasm_docs = wasm_docs;

    /**
     * @returns {any}
     */
    function wasm_get_metrics() {
        const ret = wasm.wasm_get_metrics();
        return ret;
    }
    exports.wasm_get_metrics = wasm_get_metrics;

    /**
     * @returns {any}
     */
    function wasm_hw() {
        const ret = wasm.wasm_hw();
        return ret;
    }
    exports.wasm_hw = wasm_hw;

    /**
     * @returns {any}
     */
    function wasm_models() {
        const ret = wasm.wasm_models();
        return ret;
    }
    exports.wasm_models = wasm_models;

    /**
     * @param {string} category
     * @returns {any}
     */
    function wasm_models_for_category(category) {
        const ptr0 = passStringToWasm0(category, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasm_models_for_category(ptr0, len0);
        return ret;
    }
    exports.wasm_models_for_category = wasm_models_for_category;

    /**
     * @param {string} domain
     * @returns {any}
     */
    function wasm_models_for_domain(domain) {
        const ptr0 = passStringToWasm0(domain, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasm_models_for_domain(ptr0, len0);
        return ret;
    }
    exports.wasm_models_for_domain = wasm_models_for_domain;

    /**
     * @param {string | null} [chart]
     * @param {string | null} [variant]
     * @returns {any}
     */
    function wasm_params(chart, variant) {
        var ptr0 = isLikeNone(chart) ? 0 : passStringToWasm0(chart, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = isLikeNone(variant) ? 0 : passStringToWasm0(variant, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        var len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasm_params(ptr0, len0, ptr1, len1);
        return ret;
    }
    exports.wasm_params = wasm_params;

    /**
     * @param {string | null} [chart]
     * @param {string | null} [variant]
     * @returns {any}
     */
    function wasm_required_params(chart, variant) {
        var ptr0 = isLikeNone(chart) ? 0 : passStringToWasm0(chart, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = isLikeNone(variant) ? 0 : passStringToWasm0(variant, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        var len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasm_required_params(ptr0, len0, ptr1, len1);
        return ret;
    }
    exports.wasm_required_params = wasm_required_params;

    /**
     * @returns {any}
     */
    function wasm_reset_config() {
        const ret = wasm.wasm_reset_config();
        return ret;
    }
    exports.wasm_reset_config = wasm_reset_config;

    /**
     * @returns {any}
     */
    function wasm_reset_global_background() {
        const ret = wasm.wasm_reset_global_background();
        return ret;
    }
    exports.wasm_reset_global_background = wasm_reset_global_background;

    /**
     * @returns {any}
     */
    function wasm_reset_perf_state() {
        const ret = wasm.wasm_reset_perf_state();
        return ret;
    }
    exports.wasm_reset_perf_state = wasm_reset_perf_state;

    /**
     * @returns {any}
     */
    function wasm_reset_theme() {
        const ret = wasm.wasm_reset_theme();
        return ret;
    }
    exports.wasm_reset_theme = wasm_reset_theme;

    /**
     * @param {boolean} on
     * @returns {any}
     */
    function wasm_set_adaptive_retry(on) {
        const ret = wasm.wasm_set_adaptive_retry(on);
        return ret;
    }
    exports.wasm_set_adaptive_retry = wasm_set_adaptive_retry;

    /**
     * @param {boolean} enabled
     * @returns {any}
     */
    function wasm_set_auto_display(enabled) {
        const ret = wasm.wasm_set_auto_display(enabled);
        return ret;
    }
    exports.wasm_set_auto_display = wasm_set_auto_display;

    /**
     * @param {string} color
     * @returns {any}
     */
    function wasm_set_global_background(color) {
        const ptr0 = passStringToWasm0(color, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasm_set_global_background(ptr0, len0);
        return ret;
    }
    exports.wasm_set_global_background = wasm_set_global_background;

    /**
     * @param {boolean} enabled
     * @returns {any}
     */
    function wasm_telemetry_consent(enabled) {
        const ret = wasm.wasm_telemetry_consent(enabled);
        return ret;
    }
    exports.wasm_telemetry_consent = wasm_telemetry_consent;

    /**
     * @returns {any}
     */
    function wasm_telemetry_path() {
        const ret = wasm.wasm_telemetry_path();
        return ret;
    }
    exports.wasm_telemetry_path = wasm_telemetry_path;

    /**
     * @returns {any}
     */
    function wasm_themes() {
        const ret = wasm.wasm_themes();
        return ret;
    }
    exports.wasm_themes = wasm_themes;
    function __wbg_get_imports() {
        const import0 = {
            __proto__: null,
            __wbg_Error_960c155d3d49e4c2: function(arg0, arg1) {
                const ret = Error(getStringFromWasm0(arg0, arg1));
                return ret;
            },
            __wbg___wbindgen_is_string_6df3bf7ef1164ed3: function(arg0) {
                const ret = typeof(arg0) === 'string';
                return ret;
            },
            __wbg___wbindgen_throw_6b64449b9b9ed33c: function(arg0, arg1) {
                throw new Error(getStringFromWasm0(arg0, arg1));
            },
            __wbg_new_34d45cc8e36aaead: function() {
                const ret = new Map();
                return ret;
            },
            __wbg_new_682678e2f47e32bc: function() {
                const ret = new Array();
                return ret;
            },
            __wbg_new_aa8d0fa9762c29bd: function() {
                const ret = new Object();
                return ret;
            },
            __wbg_set_3bf1de9fab0cd644: function(arg0, arg1, arg2) {
                arg0[arg1 >>> 0] = arg2;
            },
            __wbg_set_6be42768c690e380: function(arg0, arg1, arg2) {
                arg0[arg1] = arg2;
            },
            __wbg_set_fde2cec06c23692b: function(arg0, arg1, arg2) {
                const ret = arg0.set(arg1, arg2);
                return ret;
            },
            __wbindgen_cast_0000000000000001: function(arg0) {
                // Cast intrinsic for `F64 -> Externref`.
                const ret = arg0;
                return ret;
            },
            __wbindgen_cast_0000000000000002: function(arg0) {
                // Cast intrinsic for `I64 -> Externref`.
                const ret = arg0;
                return ret;
            },
            __wbindgen_cast_0000000000000003: function(arg0, arg1) {
                // Cast intrinsic for `Ref(String) -> Externref`.
                const ret = getStringFromWasm0(arg0, arg1);
                return ret;
            },
            __wbindgen_cast_0000000000000004: function(arg0) {
                // Cast intrinsic for `U64 -> Externref`.
                const ret = BigInt.asUintN(64, arg0);
                return ret;
            },
            __wbindgen_init_externref_table: function() {
                const table = wasm.__wbindgen_externrefs;
                const offset = table.grow(4);
                table.set(0, undefined);
                table.set(offset + 0, undefined);
                table.set(offset + 1, null);
                table.set(offset + 2, true);
                table.set(offset + 3, false);
            },
        };
        return {
            __proto__: null,
            "./seraplot_bg.js": import0,
        };
    }

    const ChartFinalization = (typeof FinalizationRegistry === 'undefined')
        ? { register: () => {}, unregister: () => {} }
        : new FinalizationRegistry(ptr => wasm.__wbg_chart_free(ptr >>> 0, 1));

    function getStringFromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return decodeText(ptr, len);
    }

    let cachedUint8ArrayMemory0 = null;
    function getUint8ArrayMemory0() {
        if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
            cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachedUint8ArrayMemory0;
    }

    function isLikeNone(x) {
        return x === undefined || x === null;
    }

    function passStringToWasm0(arg, malloc, realloc) {
        if (realloc === undefined) {
            const buf = cachedTextEncoder.encode(arg);
            const ptr = malloc(buf.length, 1) >>> 0;
            getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
            WASM_VECTOR_LEN = buf.length;
            return ptr;
        }

        let len = arg.length;
        let ptr = malloc(len, 1) >>> 0;

        const mem = getUint8ArrayMemory0();

        let offset = 0;

        for (; offset < len; offset++) {
            const code = arg.charCodeAt(offset);
            if (code > 0x7F) break;
            mem[ptr + offset] = code;
        }
        if (offset !== len) {
            if (offset !== 0) {
                arg = arg.slice(offset);
            }
            ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
            const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
            const ret = cachedTextEncoder.encodeInto(arg, view);

            offset += ret.written;
            ptr = realloc(ptr, len, offset, 1) >>> 0;
        }

        WASM_VECTOR_LEN = offset;
        return ptr;
    }

    let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
    cachedTextDecoder.decode();
    function decodeText(ptr, len) {
        return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
    }

    const cachedTextEncoder = new TextEncoder();

    if (!('encodeInto' in cachedTextEncoder)) {
        cachedTextEncoder.encodeInto = function (arg, view) {
            const buf = cachedTextEncoder.encode(arg);
            view.set(buf);
            return {
                read: arg.length,
                written: buf.length
            };
        };
    }

    let WASM_VECTOR_LEN = 0;

    let wasmModule, wasm;
    function __wbg_finalize_init(instance, module) {
        wasm = instance.exports;
        wasmModule = module;
        cachedUint8ArrayMemory0 = null;
        wasm.__wbindgen_start();
        return wasm;
    }

    async function __wbg_load(module, imports) {
        if (typeof Response === 'function' && module instanceof Response) {
            if (typeof WebAssembly.instantiateStreaming === 'function') {
                try {
                    return await WebAssembly.instantiateStreaming(module, imports);
                } catch (e) {
                    const validResponse = module.ok && expectedResponseType(module.type);

                    if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                        console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                    } else { throw e; }
                }
            }

            const bytes = await module.arrayBuffer();
            return await WebAssembly.instantiate(bytes, imports);
        } else {
            const instance = await WebAssembly.instantiate(module, imports);

            if (instance instanceof WebAssembly.Instance) {
                return { instance, module };
            } else {
                return instance;
            }
        }

        function expectedResponseType(type) {
            switch (type) {
                case 'basic': case 'cors': case 'default': return true;
            }
            return false;
        }
    }

    function initSync(module) {
        if (wasm !== undefined) return wasm;


        if (module !== undefined) {
            if (Object.getPrototypeOf(module) === Object.prototype) {
                ({module} = module)
            } else {
                console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
            }
        }

        const imports = __wbg_get_imports();
        if (!(module instanceof WebAssembly.Module)) {
            module = new WebAssembly.Module(module);
        }
        const instance = new WebAssembly.Instance(module, imports);
        return __wbg_finalize_init(instance, module);
    }

    async function __wbg_init(module_or_path) {
        if (wasm !== undefined) return wasm;


        if (module_or_path !== undefined) {
            if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
                ({module_or_path} = module_or_path)
            } else {
                console.warn('using deprecated parameters for the initialization function; pass a single object instead')
            }
        }

        if (module_or_path === undefined && script_src !== undefined) {
            module_or_path = script_src.replace(/\.js$/, "_bg.wasm");
        }
        const imports = __wbg_get_imports();

        if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
            module_or_path = fetch(module_or_path);
        }

        const { instance, module } = await __wbg_load(await module_or_path, imports);

        return __wbg_finalize_init(instance, module);
    }

    return Object.assign(__wbg_init, { initSync }, exports);
})({ __proto__: null });

window.SeraplotWASM = wasm_bindgen;
window.SeraplotWASM.__init = function(path){return wasm_bindgen(path).then(function(){wasm_bindgen.__ready=true;});};
