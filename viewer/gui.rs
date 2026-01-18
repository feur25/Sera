use crate::core::*;
use crate::data::loader::CsvData;
use crate::data::processor::{Dataset, DataPoint};
use crate::plot::Canvas;
use egui_plot::{Plot, Line, PlotPoints};

pub struct ViewerState {
    csv_data: Option<CsvData>,
    selected_x_col: Option<String>,
    selected_y_col: Option<String>,
    chart_type: ChartKind,
    traces: Vec<Trace>,
    canvas: Option<Canvas>,
}

impl ViewerState {
    pub fn new() -> Self {
        Self {
            csv_data: None,
            selected_x_col: None,
            selected_y_col: None,
            chart_type: ChartKind::Line,
            traces: Vec::new(),
            canvas: None,
        }
    }

    pub fn load_csv(&mut self, path: &str) -> Result<(), String> {
        self.csv_data = Some(CsvData::load(path).map_err(|e| e.to_string())?);
        if let Some(ref data) = self.csv_data {
            if !data.numeric_columns.is_empty() {
                self.selected_x_col = Some(data.numeric_columns[0].clone());
                if data.numeric_columns.len() > 1 {
                    self.selected_y_col = Some(data.numeric_columns[1].clone());
                }
            }
        }
        self.update_traces();
        Ok(())
    }

    pub fn update_traces(&mut self) {
        if let (Some(ref data), Some(ref x_col), Some(ref y_col)) = (&self.csv_data, &self.selected_x_col, &self.selected_y_col) {
            let x = data.get_numeric_column(x_col);
            let y = data.get_numeric_column(y_col);

            if !x.is_empty() && !y.is_empty() {
                let len = x.len().min(y.len());
                let x_trimmed = x[..len].to_vec();
                let y_trimmed = y[..len].to_vec();

                let mut trace = Trace::new(
                    "data".to_string(),
                    format!("{} vs {}", x_col, y_col),
                    self.chart_type,
                    x_trimmed,
                    y_trimmed,
                );
                trace.marker.color = Color { r: 31, g: 119, b: 180, a: 255 };
                trace.line.color = Color { r: 31, g: 119, b: 180, a: 255 };

                self.traces = vec![trace];
                self.rebuild_canvas();
            }
        }
    }

    pub fn update_traces_filtered<F>(&mut self, predicate: F) 
    where
        F: Fn(f64, f64) -> bool,
    {
        if let (Some(ref data), Some(ref x_col), Some(ref y_col)) = (&self.csv_data, &self.selected_x_col, &self.selected_y_col) {
            let x = data.get_numeric_column(x_col);
            let y = data.get_numeric_column(y_col);

            if !x.is_empty() && !y.is_empty() {
                let len = x.len().min(y.len());
                let mut x_filtered = Vec::new();
                let mut y_filtered = Vec::new();

                for i in 0..len {
                    if predicate(x[i], y[i]) {
                        x_filtered.push(x[i]);
                        y_filtered.push(y[i]);
                    }
                }

                if !x_filtered.is_empty() {
                    let mut trace = Trace::new(
                        "filtered_data".to_string(),
                        format!("Filtered: {} vs {}", x_col, y_col),
                        self.chart_type,
                        x_filtered,
                        y_filtered,
                    );
                    trace.marker.color = Color { r: 31, g: 119, b: 180, a: 255 };
                    trace.line.color = Color { r: 31, g: 119, b: 180, a: 255 };

                    self.traces = vec![trace];
                    self.rebuild_canvas();
                }
            }
        }
    }

    fn rebuild_canvas(&mut self) {
        if self.traces.is_empty() {
            return;
        }

        let mut layout = Layout::default();
        layout.width = 1000;
        layout.height = 600;
        if let Some(ref x_col) = self.selected_x_col {
            layout.x_axis.title = x_col.clone();
        }
        if let Some(ref y_col) = self.selected_y_col {
            layout.y_axis.title = y_col.clone();
        }

        let mut canvas = Canvas::new(1000.0, 600.0, layout);
        for trace in self.traces.clone() {
            canvas = canvas.add_trace(trace);
        }
        canvas.auto_scale();
        self.canvas = Some(canvas);
    }

    pub fn set_chart_type(&mut self, chart_type: ChartKind) {
        self.chart_type = chart_type;
        self.update_traces();
    }

    pub fn set_x_column(&mut self, col: String) {
        self.selected_x_col = Some(col);
        self.update_traces();
    }

    pub fn set_y_column(&mut self, col: String) {
        self.selected_y_col = Some(col);
        self.update_traces();
    }

    pub fn get_numeric_columns(&self) -> Vec<String> {
        self.csv_data.as_ref()
            .map(|d| d.numeric_columns.clone())
            .unwrap_or_default()
    }

    pub fn get_all_columns(&self) -> Vec<String> {
        self.csv_data.as_ref()
            .map(|d| d.headers.clone())
            .unwrap_or_default()
    }

    pub fn get_row_count(&self) -> usize {
        self.csv_data.as_ref()
            .map(|d| d.len())
            .unwrap_or(0)
    }

    pub fn export_trace_json(&self) -> Option<String> {
        if self.traces.is_empty() {
            return None;
        }
        serde_json::to_string_pretty(&self.traces).ok()
    }
}

impl Default for ViewerState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ViewerApp {
    state: ViewerState,
    file_path: String,
    status_message: String,
}

impl ViewerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self {
            state: ViewerState::new(),
            file_path: String::new(),
            status_message: "Load a CSV file to start".to_string(),
        }
    }

    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("CSV File:");
            ui.text_edit_singleline(&mut self.file_path);
            if ui.button("Load").clicked() {
                match self.state.load_csv(&self.file_path) {
                    Ok(_) => self.status_message = format!("Loaded {} rows", self.state.get_row_count()),
                    Err(e) => self.status_message = format!("Error: {}", e),
                }
            }
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Chart Type:");
            let chart_name = format!("{:?}", self.state.chart_type);
            egui::ComboBox::from_label("")
                .selected_text(&chart_name)
                .show_ui(ui, |ui| {
                    let types = [
                        ChartKind::Line,
                        ChartKind::Scatter,
                        ChartKind::Bar,
                        ChartKind::Area,
                        ChartKind::Histogram,
                        ChartKind::Box,
                    ];
                    for t in types {
                        if ui.selectable_value(&mut self.state.chart_type, t, format!("{:?}", t)).changed() {
                            self.state.set_chart_type(t);
                        }
                    }
                });
        });

        let numeric_cols = self.state.get_numeric_columns();
        if !numeric_cols.is_empty() {
            ui.horizontal(|ui| {
                ui.label("X Axis:");
                let current_x = self.state.selected_x_col.clone().unwrap_or_default();
                egui::ComboBox::from_label("")
                    .selected_text(&current_x)
                    .show_ui(ui, |ui| {
                        for col in &numeric_cols {
                            if ui.selectable_value(&mut self.state.selected_x_col, Some(col.clone()), col).changed() {
                                self.state.update_traces();
                            }
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Y Axis:");
                let current_y = self.state.selected_y_col.clone().unwrap_or_default();
                egui::ComboBox::from_label("")
                    .selected_text(&current_y)
                    .show_ui(ui, |ui| {
                        for col in &numeric_cols {
                            if ui.selectable_value(&mut self.state.selected_y_col, Some(col.clone()), col).changed() {
                                self.state.update_traces();
                            }
                        }
                    });
            });
        }

        ui.separator();
        ui.label(&self.status_message);
    }

    fn render_plot(&self, ui: &mut egui::Ui) {
        if self.state.traces.is_empty() {
            ui.label("No data to display");
            return;
        }

        let trace = &self.state.traces[0];
        let points: PlotPoints = trace.x.iter()
            .zip(trace.y.iter())
            .map(|(x, y)| [*x, *y])
            .collect();

        let line = Line::new(points)
            .color(egui::Color32::from_rgb(31, 119, 180));

        Plot::new("plot")
            .view_aspect(2.0)
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });
    }
}

impl eframe::App for ViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("SeraPlot CSV Viewer");
            self.render_controls(ui);
            ui.separator();
            self.render_plot(ui);
        });
    }
}

pub fn run_viewer() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "seraplot",
        options,
        Box::new(|cc| Box::new(ViewerApp::new(cc))),
    )
}
