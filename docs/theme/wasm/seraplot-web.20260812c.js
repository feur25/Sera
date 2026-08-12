/* @ts-self-types="./seraplot.d.ts" */

export class Chart {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function accessiblePalette(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.accessiblePalette(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} method
 * @param {string} alias
 * @returns {boolean}
 */
export function aliasAdd(method, alias) {
    const ptr0 = passStringToWasm0(method, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(alias, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.aliasAdd(ptr0, len0, ptr1, len1);
    return ret !== 0;
}

/**
 * @returns {string}
 */
export function aliasList() {
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

/**
 * @param {string} json
 * @returns {boolean}
 */
export function aliasLoadJson(json) {
    const ptr0 = passStringToWasm0(json, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.aliasLoadJson(ptr0, len0);
    return ret !== 0;
}

/**
 * @param {string} method
 * @param {string} alias
 * @returns {boolean}
 */
export function aliasRemove(method, alias) {
    const ptr0 = passStringToWasm0(method, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(alias, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.aliasRemove(ptr0, len0, ptr1, len1);
    return ret !== 0;
}

export function aliasReset() {
    wasm.aliasReset();
}

/**
 * @param {string} name
 * @returns {string | undefined}
 */
export function aliasResolve(name) {
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

/**
 * @param {string} html
 * @param {string} name
 * @param {string} args_json
 * @returns {string}
 */
export function applyChartMethod(html, name, args_json) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildArcDiagram(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildArcDiagram(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildAreaChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildBar(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildBar3dChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildBarChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildBoxplot(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildBubble(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildBubble3dChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildBubbleMap(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildBullet(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildCandlestick(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildCandlestick3dChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildChord(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildChord(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildChoropleth(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildCirclePack(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildCirclePack(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildConeChart(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildConeChart(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildCorrelogram(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildCorrelogram(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildDbscanChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildDbscanChart3d(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildDendrogram(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildDendrogram(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildDonutChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildDumbbell(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildDumbbell3dChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildEventplot(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildEventplot(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildFacet(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildFacet(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildFirehoseChart(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildFirehoseChart(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildFunnel(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildFunnel3dChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildGantt(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildGantt(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildGauge(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildGlobe3dChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildGroupedBar(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildHbar(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildHeatmap(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildHeatmap3dChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildHexbin(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildHexbin(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildHistogram(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildHive(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildHive(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildIcicle(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildIcicle(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildIsosurfaceChart(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildIsosurfaceChart(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildJoint(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildJoint(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildKde3dChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildKdeChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildKmeansChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildLine(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildLine3dChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildLineChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildLollipop3dChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildLollipopChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildMesh3dChart(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildMesh3dChart(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildMultilineChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildOrbita(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildOrbita(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildParallel(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildParcats(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildParcats(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildPie(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildPie3dChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildPieChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildPlotWeb(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildPlotWeb(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildPulse(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildPulse(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildRadar3dChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildRadarChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildRidgeline3dChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildRidgelineChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildSankey(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildSankey(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildScatter3dChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildScatterChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildScatterTernary(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildScatterTernary(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildSlope(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildSplom(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildSplom(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildStackedBar(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildStackedBar3dChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildStackplot(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildStackplot(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildStreamtubeChart(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildStreamtubeChart(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildSunburst(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildSunburst3dChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildSurface3dChart(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildSurface3dChart(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildTreemap(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildVenn(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildVenn(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildViolin(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildViolin3dChart(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildVoxelsChart(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildVoxelsChart(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildWaterfall(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function buildWireframe3dChart(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.buildWireframe3dChart(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function buildWordcloud(input) {
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

/**
 * @returns {string}
 */
export function chartAliases() {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function chartAppend(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function chartDiff(input) {
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

/**
 * @returns {string}
 */
export function chartThemes() {
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

/**
 * @returns {string}
 */
export function chartVariants() {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function csvChunkRead(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function csvCountRows(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function demo(input) {
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

/**
 * @returns {string}
 */
export function docs() {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function downsampleLttb(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function driftKs(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function exportHtmlFile(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function mlAdaboostClassifier(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlAdaboostClassifier(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlAdaboostRegressor(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlAdaboostRegressor(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlBernoulliNb(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlBernoulliNb(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlCrossValScore(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlCrossValScore(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlDbscanFitPredict(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlDbscanFitPredict(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlDecisionTreeClassifier(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlDecisionTreeClassifier(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlDecisionTreeRegressor(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlDecisionTreeRegressor(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlElasticNet(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlElasticNet(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlFitTransform(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlFitTransform(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlGaussianNb(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlGaussianNb(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlGradientBoostingClassifier(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlGradientBoostingClassifier(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlGradientBoostingRegressor(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlGradientBoostingRegressor(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlGridSearchCv(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlGridSearchCv(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlIsolationForest(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlIsolationForest(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlKfoldSplit(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlKfoldSplit(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlKmeansFitPredict(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlKmeansFitPredict(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlKnnClassifier(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlKnnClassifier(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlKnnRegressor(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlKnnRegressor(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlLasso(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlLasso(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlLinearRegression(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlLinearRegression(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlLinearSvc(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlLinearSvc(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlLinearSvr(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlLinearSvr(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlLoadModel(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlLoadModel(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlLogisticRegression(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlLogisticRegression(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlMetricCurve(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlMetricCurve(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlMetricScore(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlMetricScore(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlMinmaxScaler(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlMinmaxScaler(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlMultinomialNb(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlMultinomialNb(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlNearestCentroid(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlNearestCentroid(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlPca(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlPca(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlPermutationImportance(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlPermutationImportance(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlRandomForestClassifier(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlRandomForestClassifier(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlRandomForestRegressor(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlRandomForestRegressor(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlRidge(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlRidge(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlRidgeClassifier(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlRidgeClassifier(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlRobustScaler(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlRobustScaler(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlSaveModel(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlSaveModel(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlSgdClassifier(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlSgdClassifier(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlSgdRegressor(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlSgdRegressor(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlStandardScaler(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlStandardScaler(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function mlTruncatedSvd(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.mlTruncatedSvd(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function params(input) {
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

/**
 * @param {string} title
 * @param {Float64Array} x
 * @param {Float64Array} y
 * @param {number} width
 * @param {number} height
 * @param {number} color_hex
 * @param {boolean} gridlines
 * @param {string} x_label
 * @param {string} y_label
 * @returns {string}
 */
export function render_line_wasm(title, x, y, width, height, color_hex, gridlines, x_label, y_label) {
    let deferred6_0;
    let deferred6_1;
    try {
        const ptr0 = passStringToWasm0(title, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayF64ToWasm0(x, wasm.__wbindgen_malloc_command_export);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passArrayF64ToWasm0(y, wasm.__wbindgen_malloc_command_export);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(x_label, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len3 = WASM_VECTOR_LEN;
        const ptr4 = passStringToWasm0(y_label, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len4 = WASM_VECTOR_LEN;
        const ret = wasm.render_line_wasm(ptr0, len0, ptr1, len1, ptr2, len2, width, height, color_hex, gridlines, ptr3, len3, ptr4, len4);
        deferred6_0 = ret[0];
        deferred6_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred6_0, deferred6_1, 1);
    }
}

/**
 * @param {string} title
 * @param {Float64Array} x
 * @param {Float64Array} y
 * @param {number} width
 * @param {number} height
 * @param {number} color_hex
 * @param {boolean} gridlines
 * @param {string} x_label
 * @param {string} y_label
 * @returns {string}
 */
export function render_scatter_wasm(title, x, y, width, height, color_hex, gridlines, x_label, y_label) {
    let deferred6_0;
    let deferred6_1;
    try {
        const ptr0 = passStringToWasm0(title, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayF64ToWasm0(x, wasm.__wbindgen_malloc_command_export);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passArrayF64ToWasm0(y, wasm.__wbindgen_malloc_command_export);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passStringToWasm0(x_label, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len3 = WASM_VECTOR_LEN;
        const ptr4 = passStringToWasm0(y_label, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len4 = WASM_VECTOR_LEN;
        const ret = wasm.render_scatter_wasm(ptr0, len0, ptr1, len1, ptr2, len2, width, height, color_hex, gridlines, ptr3, len3, ptr4, len4);
        deferred6_0 = ret[0];
        deferred6_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred6_0, deferred6_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function requiredParams(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function resetGlobalBackground(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function resetTheme(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function scalePlan(input) {
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

/**
 * @returns {string}
 */
export function scenes3d() {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function setGlobalBackground(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function setTheme(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function systemProfile(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function themes(input) {
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

/**
 * @param {string} input
 * @returns {string}
 */
export function trueRequiredParams(input) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.trueRequiredParams(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free_command_export(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @param {string} input
 * @returns {string}
 */
export function validateInput(input) {
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

/**
 * @returns {any}
 */
export function wasm_adaptive_degrade_level() {
    const ret = wasm.wasm_adaptive_degrade_level();
    return ret;
}

/**
 * @param {string} html
 * @returns {any}
 */
export function wasm_apply_color_bindings_html(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_apply_color_bindings_html(ptr0, len0);
    return ret;
}

/**
 * @returns {any}
 */
export function wasm_chart_variants() {
    const ret = wasm.wasm_chart_variants();
    return ret;
}

/**
 * @returns {any}
 */
export function wasm_clear_color_bindings() {
    const ret = wasm.wasm_clear_color_bindings();
    return ret;
}

/**
 * @param {string} html
 * @returns {any}
 */
export function wasm_color_density_html(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_color_density_html(ptr0, len0);
    return ret;
}

/**
 * @param {string} html
 * @returns {any}
 */
export function wasm_crosshair_html(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_crosshair_html(ptr0, len0);
    return ret;
}

/**
 * @param {string} html
 * @param {number | null | undefined} step
 * @param {number} gap
 * @param {string | null} [color]
 * @returns {any}
 */
export function wasm_cut_bars_html(html, step, gap, color) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    var ptr1 = isLikeNone(color) ? 0 : passStringToWasm0(color, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    var len1 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_cut_bars_html(ptr0, len0, !isLikeNone(step), isLikeNone(step) ? 0 : step, gap, ptr1, len1);
    return ret;
}

/**
 * @param {string} chart
 * @param {string | null} [variant]
 * @returns {any}
 */
export function wasm_demo(chart, variant) {
    const ptr0 = passStringToWasm0(chart, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    var ptr1 = isLikeNone(variant) ? 0 : passStringToWasm0(variant, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    var len1 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_demo(ptr0, len0, ptr1, len1);
    return ret;
}

/**
 * @returns {any}
 */
export function wasm_demos() {
    const ret = wasm.wasm_demos();
    return ret;
}

/**
 * @param {string} html
 * @param {Uint32Array | null | undefined} indices
 * @param {number} factor
 * @returns {any}
 */
export function wasm_desaturate_html(html, indices, factor) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    var ptr1 = isLikeNone(indices) ? 0 : passArray32ToWasm0(indices, wasm.__wbindgen_malloc_command_export);
    var len1 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_desaturate_html(ptr0, len0, ptr1, len1, factor);
    return ret;
}

/**
 * @param {string} name
 * @returns {any}
 */
export function wasm_doc(name) {
    const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_doc(ptr0, len0);
    return ret;
}

/**
 * @returns {any}
 */
export function wasm_docs() {
    const ret = wasm.wasm_docs();
    return ret;
}

/**
 * @param {string} html
 * @param {string} color
 * @returns {any}
 */
export function wasm_draw_tool_html(html, color) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(color, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_draw_tool_html(ptr0, len0, ptr1, len1);
    return ret;
}

/**
 * @param {string} html
 * @returns {any}
 */
export function wasm_export_button_html(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_export_button_html(ptr0, len0);
    return ret;
}

/**
 * @param {string} html
 * @returns {any}
 */
export function wasm_flip_html(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_flip_html(ptr0, len0);
    return ret;
}

/**
 * @returns {any}
 */
export function wasm_get_metrics() {
    const ret = wasm.wasm_get_metrics();
    return ret;
}

/**
 * @param {string} html
 * @param {number} value
 * @param {string} color
 * @param {string | null} [label]
 * @returns {any}
 */
export function wasm_grid_at_html(html, value, color, label) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(color, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len1 = WASM_VECTOR_LEN;
    var ptr2 = isLikeNone(label) ? 0 : passStringToWasm0(label, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    var len2 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_grid_at_html(ptr0, len0, value, ptr1, len1, ptr2, len2);
    return ret;
}

/**
 * @param {string} html
 * @returns {any}
 */
export function wasm_grid_x_html(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_grid_x_html(ptr0, len0);
    return ret;
}

/**
 * @param {string} html
 * @returns {any}
 */
export function wasm_grid_y_html(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_grid_y_html(ptr0, len0);
    return ret;
}

/**
 * @param {string} html
 * @param {number} dim
 * @returns {any}
 */
export function wasm_group_hover_opacity_html(html, dim) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_group_hover_opacity_html(ptr0, len0, dim);
    return ret;
}

/**
 * @param {string} html
 * @param {string[]} labels
 * @param {number} dim
 * @returns {any}
 */
export function wasm_highlight_group_html(html, labels, dim) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayJsValueToWasm0(labels, wasm.__wbindgen_malloc_command_export);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_highlight_group_html(ptr0, len0, ptr1, len1, dim);
    return ret;
}

/**
 * @param {string} html
 * @param {string} slots_json
 * @returns {any}
 */
export function wasm_hover_slots_html(html, slots_json) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(slots_json, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_hover_slots_html(ptr0, len0, ptr1, len1);
    return ret;
}

/**
 * @returns {any}
 */
export function wasm_hw() {
    const ret = wasm.wasm_hw();
    return ret;
}

/**
 * @param {string} html
 * @param {string} css
 * @returns {any}
 */
export function wasm_inject_css_html(html, css) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(css, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_inject_css_html(ptr0, len0, ptr1, len1);
    return ret;
}

/**
 * @param {string} html
 * @param {string} js
 * @returns {any}
 */
export function wasm_inject_js_html(html, js) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(js, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_inject_js_html(ptr0, len0, ptr1, len1);
    return ret;
}

/**
 * @returns {any}
 */
export function wasm_models() {
    const ret = wasm.wasm_models();
    return ret;
}

/**
 * @param {string} category
 * @returns {any}
 */
export function wasm_models_for_category(category) {
    const ptr0 = passStringToWasm0(category, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_models_for_category(ptr0, len0);
    return ret;
}

/**
 * @param {string} domain
 * @returns {any}
 */
export function wasm_models_for_domain(domain) {
    const ptr0 = passStringToWasm0(domain, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_models_for_domain(ptr0, len0);
    return ret;
}

/**
 * @param {string} html
 * @returns {any}
 */
export function wasm_no_axes_html(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_no_axes_html(ptr0, len0);
    return ret;
}

/**
 * @param {string} html
 * @returns {any}
 */
export function wasm_no_background_html(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_no_background_html(ptr0, len0);
    return ret;
}

/**
 * @param {string} html
 * @returns {any}
 */
export function wasm_no_hover_html(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_no_hover_html(ptr0, len0);
    return ret;
}

/**
 * @param {string} html
 * @returns {any}
 */
export function wasm_no_legend_html(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_no_legend_html(ptr0, len0);
    return ret;
}

/**
 * @param {string} html
 * @returns {any}
 */
export function wasm_no_select_html(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_no_select_html(ptr0, len0);
    return ret;
}

/**
 * @param {string} html
 * @returns {any}
 */
export function wasm_no_title_html(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_no_title_html(ptr0, len0);
    return ret;
}

/**
 * @param {string} html
 * @returns {any}
 */
export function wasm_no_x_axis_html(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_no_x_axis_html(ptr0, len0);
    return ret;
}

/**
 * @param {string} html
 * @returns {any}
 */
export function wasm_no_y_axis_html(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_no_y_axis_html(ptr0, len0);
    return ret;
}

/**
 * @param {string | null} [chart]
 * @param {string | null} [variant]
 * @returns {any}
 */
export function wasm_params(chart, variant) {
    var ptr0 = isLikeNone(chart) ? 0 : passStringToWasm0(chart, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    var len0 = WASM_VECTOR_LEN;
    var ptr1 = isLikeNone(variant) ? 0 : passStringToWasm0(variant, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    var len1 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_params(ptr0, len0, ptr1, len1);
    return ret;
}

/**
 * @param {string | null} [chart]
 * @param {string | null} [variant]
 * @returns {any}
 */
export function wasm_required_params(chart, variant) {
    var ptr0 = isLikeNone(chart) ? 0 : passStringToWasm0(chart, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    var len0 = WASM_VECTOR_LEN;
    var ptr1 = isLikeNone(variant) ? 0 : passStringToWasm0(variant, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    var len1 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_required_params(ptr0, len0, ptr1, len1);
    return ret;
}

/**
 * @returns {any}
 */
export function wasm_reset_config() {
    const ret = wasm.wasm_reset_config();
    return ret;
}

/**
 * @returns {any}
 */
export function wasm_reset_global_background() {
    const ret = wasm.wasm_reset_global_background();
    return ret;
}

/**
 * @returns {any}
 */
export function wasm_reset_perf_state() {
    const ret = wasm.wasm_reset_perf_state();
    return ret;
}

/**
 * @returns {any}
 */
export function wasm_reset_theme() {
    const ret = wasm.wasm_reset_theme();
    return ret;
}

/**
 * @param {string} html
 * @returns {any}
 */
export function wasm_responsive_html(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_responsive_html(ptr0, len0);
    return ret;
}

/**
 * @param {boolean} on
 * @returns {any}
 */
export function wasm_set_adaptive_retry(on) {
    const ret = wasm.wasm_set_adaptive_retry(on);
    return ret;
}

/**
 * @param {boolean} enabled
 * @returns {any}
 */
export function wasm_set_auto_display(enabled) {
    const ret = wasm.wasm_set_auto_display(enabled);
    return ret;
}

/**
 * @param {string} color
 * @returns {any}
 */
export function wasm_set_global_background(color) {
    const ptr0 = passStringToWasm0(color, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_set_global_background(ptr0, len0);
    return ret;
}

/**
 * @param {string} html
 * @returns {any}
 */
export function wasm_show_grid_html(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_show_grid_html(ptr0, len0);
    return ret;
}

/**
 * @param {string} html
 * @returns {any}
 */
export function wasm_sparse_grid_html(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_sparse_grid_html(ptr0, len0);
    return ret;
}

/**
 * @param {boolean} enabled
 * @returns {any}
 */
export function wasm_telemetry_consent(enabled) {
    const ret = wasm.wasm_telemetry_consent(enabled);
    return ret;
}

/**
 * @returns {any}
 */
export function wasm_telemetry_path() {
    const ret = wasm.wasm_telemetry_path();
    return ret;
}

/**
 * @returns {any}
 */
export function wasm_themes() {
    const ret = wasm.wasm_themes();
    return ret;
}

/**
 * @param {string | null} [chart]
 * @param {string | null} [variant]
 * @returns {any}
 */
export function wasm_true_required_params(chart, variant) {
    var ptr0 = isLikeNone(chart) ? 0 : passStringToWasm0(chart, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    var len0 = WASM_VECTOR_LEN;
    var ptr1 = isLikeNone(variant) ? 0 : passStringToWasm0(variant, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    var len1 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_true_required_params(ptr0, len0, ptr1, len1);
    return ret;
}

/**
 * @param {string} html
 * @returns {any}
 */
export function wasm_zoom_html(html) {
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_zoom_html(ptr0, len0);
    return ret;
}
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_Error_92b29b0548f8b746: function(arg0, arg1) {
            const ret = Error(getStringFromWasm0(arg0, arg1));
            return ret;
        },
        __wbg___wbindgen_is_string_ea5e6cc2e4141dfe: function(arg0) {
            const ret = typeof(arg0) === 'string';
            return ret;
        },
        __wbg___wbindgen_string_get_b0ca35b86a603356: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc_command_export, wasm.__wbindgen_realloc_command_export);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_344f42d3211c4765: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_new_32b398fb48b6d94a: function() {
            const ret = new Array();
            return ret;
        },
        __wbg_new_7796ffc7ed656783: function() {
            const ret = new Map();
            return ret;
        },
        __wbg_new_da52cf8fe3429cb2: function() {
            const ret = new Object();
            return ret;
        },
        __wbg_set_575dd786d51585f8: function(arg0, arg1, arg2) {
            const ret = arg0.set(arg1, arg2);
            return ret;
        },
        __wbg_set_6be42768c690e380: function(arg0, arg1, arg2) {
            arg0[arg1] = arg2;
        },
        __wbg_set_8a16b38e4805b298: function(arg0, arg1, arg2) {
            arg0[arg1 >>> 0] = arg2;
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
    : new FinalizationRegistry(ptr => wasm.__wbg_chart_free(ptr, 1));

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc_command_export();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat64ArrayMemory0 = null;
function getFloat64ArrayMemory0() {
    if (cachedFloat64ArrayMemory0 === null || cachedFloat64ArrayMemory0.byteLength === 0) {
        cachedFloat64ArrayMemory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint32ArrayMemory0 = null;
function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
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

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getUint32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayF64ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 8, 8) >>> 0;
    getFloat64ArrayMemory0().set(arg, ptr / 8);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    for (let i = 0; i < array.length; i++) {
        const add = addToExternrefTable0(array[i]);
        getDataViewMemory0().setUint32(ptr + 4 * i, add, true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
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
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
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

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedFloat64ArrayMemory0 = null;
    cachedUint32ArrayMemory0 = null;
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

    if (module_or_path === undefined) {
        module_or_path = new URL('seraplot_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
