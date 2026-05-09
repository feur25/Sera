use std::sync::{Arc, Mutex};
use crate::bindings::{FastChartRenderer, FastChartConfig};

lazy_static::lazy_static! {
    static ref CHART_SORT: Mutex<usize> = Mutex::new(0);
}

struct ChartData {
    labels: Vec<String>,
    values: Vec<f64>,
    title: String,
}

pub struct HybridChartApp {
    data: Arc<Mutex<Option<ChartData>>>,
    mode: RenderMode,
    use_fast_svg: bool,
    svg_cache: String,
}

#[derive(Clone, Copy, PartialEq)]
enum RenderMode {
    FastSVG,
    Interactive,
}

impl HybridChartApp {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(None)),
            mode: RenderMode::FastSVG,
            use_fast_svg: true,
            svg_cache: String::new(),
        }
    }

    pub fn set_data(&mut self, labels: Vec<String>, values: Vec<f64>, title: String) {
        {
            let mut data = self.data.lock().unwrap();
            *data = Some(ChartData { labels, values, title });
        }
        self.regenerate_svg();
    }

    fn regenerate_svg(&mut self) {
        let data_lock = self.data.lock().unwrap();
        if let Some(d) = data_lock.as_ref() {
            let config = FastChartConfig::bar_horizontal();
            let renderer = FastChartRenderer::new(config, &d.title)
                .with_data(d.labels.clone(), d.values.clone());
            self.svg_cache = renderer.render_svg();
        }
        drop(data_lock);
    }

    pub fn render_fast(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(self.mode == RenderMode::FastSVG, "Fast SVG").clicked() {
                    self.mode = RenderMode::FastSVG;
                    self.use_fast_svg = true;
                }
                if ui.selectable_label(self.mode == RenderMode::Interactive, "Interactive").clicked() {
                    self.mode = RenderMode::Interactive;
                    self.use_fast_svg = false;
                }
            });

            ui.separator();

            match self.mode {
                RenderMode::FastSVG => {
                    if !self.svg_cache.is_empty() {
                        ui.heading("SVG Rendered (Instant)");
                        let mut svg_text = self.svg_cache.clone();
                        ui.text_edit_multiline(&mut svg_text);
                        
                        if ui.button("Save SVG").clicked() {
                            let _ = std::fs::write("export.svg", &self.svg_cache);
                        }
                    }
                }
                RenderMode::Interactive => {
                    ui.label("Interactive mode would load here");
                }
            }
        });
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        self.render_fast(ctx);
    }
}

#[cfg(feature = "gui")]
pub fn launch_hybrid_viewer(labels: Vec<String>, values: Vec<f64>, title: String) {
    let options = eframe::NativeOptions::default();
    let mut app = HybridChartApp::new();
    app.set_data(labels, values, title);

    let _ = eframe::run_native(
        "SeraPlot Hybrid Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    );
}

#[cfg(feature = "gui")]
impl eframe::App for HybridChartApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        HybridChartApp::update(self, ctx);
    }
}


