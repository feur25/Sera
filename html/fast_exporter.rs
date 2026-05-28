use crate::bindings::utils::simd_ops;
use crate::bindings::utils::lazy_builders::{LazyJsonBuilder};
use crate::bindings::utils::{RingBuffer, RenderArena};
use crate::data::Dataset;
use super::fast_builders::{SvgBuilder, HtmlBuilder};

pub struct FastHtmlExporter {
    width: i32,
    height: i32,
    title: String,
    colors: parking_lot::Mutex<Vec<u32>>,
}

impl FastHtmlExporter {
    #[inline(always)]
    pub fn new(width: i32, height: i32, title: String) -> Self {
        Self {
            width,
            height,
            title,
            colors: parking_lot::Mutex::new(Vec::with_capacity(100000)),
        }
    }

    #[inline(always)]
    pub fn build_from_dataset(&self, data: Dataset<f64>) -> String {
        let n = data.len();
        if n == 0 { return String::new(); }

        let values = data.to_values_vec();
        let (_, max_val) = simd_ops::find_minmax(&values);
        let max_val = max_val.max(1.0);

        let mut color_cache: RingBuffer<u32, 512> = RingBuffer::new();
        let mut colors_raw: Vec<u32> = Vec::with_capacity(n.min(512));
        simd_ops::compute_hex_colors_batch_into(n, &mut colors_raw);
        for &c in &colors_raw { color_cache.push(c); }

        let bar_width = 1200.0 / (n as f32).max(1.0);
        let mut arena = RenderArena::new(n * 80, n * 64);
        let mut svg = SvgBuilder::new(self.width as f32, self.height as f32, n * 80);
        let scale_factor = 500.0 / max_val;

        for i in 0..n {
            let height = (values[i] * scale_factor) as i32;
            let x = (i as f32 * bar_width) as i32;
            let y = 550 - height;
            let w = bar_width as i32;
            let color = color_cache.get(i % 512).unwrap_or(0x6464C8);
            svg.add_bar_fast(x, y, w, height, color);
        }

        let svg_str = svg.finish();
        let mut json_builder = LazyJsonBuilder::new(n);
        for (label, &val) in data.labels().zip(values.iter()) {
            json_builder.add_point(label.to_string(), val);
        }
        let json = json_builder.build().to_string();
        let _ = arena.svg_buf();
        drop(arena);

        HtmlBuilder::new(&self.title, &svg_str, &json).build()
    }

    #[inline(always)]
    pub fn build_optimized(&self, labels: Vec<String>, values: Vec<f64>) -> String {
        let n = values.len().min(labels.len());
        if n == 0 {
            return String::new();
        }

        let (_, max_val) = simd_ops::find_minmax(&values[..n]);
        let max_val = max_val.max(1.0);

        let mut colors = Vec::with_capacity(n);
        simd_ops::compute_hex_colors_batch_into(n, &mut colors);

        let bar_width = 1200.0 / (n as f32).max(1.0);
        let mut svg = SvgBuilder::new(self.width as f32, self.height as f32, n * 80);
        let scale_factor = 500.0 / max_val;

        for i in 0..n {
            let height = (values[i] * scale_factor) as i32;
            let x = (i as f32 * bar_width) as i32;
            let y = (550 - height) as i32;
            let w = bar_width as i32;
            let color = colors.get(i).copied().unwrap_or(0x6464C8);
            svg.add_bar_fast(x, y, w, height, color);
        }

        let svg_str = svg.finish();
        let mut json_builder = LazyJsonBuilder::new(n);
        
        for i in 0..n {
            json_builder.add_point(labels[i].clone(), values[i]);
        }
        let json = json_builder.build().to_string();

        HtmlBuilder::new(&self.title, &svg_str, &json).build()
    }

    pub fn build_streaming<F>(&self, mut chunk_handler: F)
    where
        F: FnMut(&str),
    {
        chunk_handler("<!DOCTYPE html><html><head><meta charset=UTF-8><title>");
        chunk_handler(&self.title);
        chunk_handler("</title><style>*{margin:0;padding:0;box-sizing:border-box}body{background:#f5f5f5}svg{width:100%;height:100%;display:block}</style></head><body><div class=chart-container>");
    }

    pub fn batch_export(&self, batch: Vec<(String, f64)>) -> String {
        let (labels, values): (Vec<_>, Vec<_>) = batch.into_iter().unzip();
        self.build_optimized(labels, values)
    }
}


