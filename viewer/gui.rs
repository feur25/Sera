use crate::data::loader::CsvData;
use crate::plot::Canvas;

pub struct ViewerState {
    csv_data: Option<CsvData>,
    selected_x_col: Option<String>,
    selected_y_col: Option<String>,
    selected_chart_type: u8,
    canvas: Option<Canvas>,
}

impl ViewerState {
    pub fn new() -> Self {
        Self {
            csv_data: None,
            selected_x_col: None,
            selected_y_col: None,
            selected_chart_type: 0,
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
        self.rebuild_canvas();
        Ok(())
    }

    pub fn rebuild_canvas(&mut self) {
        if let (Some(ref data), Some(ref x_col), Some(ref y_col)) =
            (&self.csv_data, &self.selected_x_col, &self.selected_y_col)
        {
            let x = data.get_numeric_column(x_col);
            let y = data.get_numeric_column(y_col);

            if !x.is_empty() && !y.is_empty() {
                let len = x.len().min(y.len());
                let labels: Vec<String> = (0..len).map(|i| i.to_string()).collect();
                let values: Vec<f64> = y[..len].to_vec();

                self.canvas = Some(Canvas::new_with_data(
                    &labels,
                    &values,
                    self.selected_chart_type,
                ));
            }
        }
    }

    pub fn set_chart_type(&mut self, chart_type: u8) {
        self.selected_chart_type = chart_type;
        self.rebuild_canvas();
    }

    pub fn set_x_column(&mut self, col: String) {
        self.selected_x_col = Some(col);
        self.rebuild_canvas();
    }

    pub fn set_y_column(&mut self, col: String) {
        self.selected_y_col = Some(col);
        self.rebuild_canvas();
    }

    pub fn get_numeric_columns(&self) -> Vec<String> {
        self.csv_data
            .as_ref()
            .map(|d| d.numeric_columns.clone())
            .unwrap_or_default()
    }

    pub fn get_all_columns(&self) -> Vec<String> {
        self.csv_data
            .as_ref()
            .map(|d| d.headers.clone())
            .unwrap_or_default()
    }

    pub fn get_row_count(&self) -> usize {
        self.csv_data.as_ref().map(|d| d.len()).unwrap_or(0)
    }

    pub fn export_chart_svg(&self) -> Option<String> {
        self.canvas.as_ref().map(|c| c.render_svg())
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
                    Ok(_) => {
                        self.status_message = format!("Loaded {} rows", self.state.get_row_count())
                    }
                    Err(e) => self.status_message = format!("Error: {}", e),
                }
            }
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Chart Type:");
            let chart_types = crate::plot::default::get_current_group_types();
            let current_name = chart_types
                .iter()
                .find(|(id, _)| *id == self.state.selected_chart_type)
                .map(|(_, name)| name.clone())
                .unwrap_or_else(|| "Unknown".to_string());

            egui::ComboBox::from_label("")
                .selected_text(&current_name)
                .show_ui(ui, |ui| {
                    for (type_id, type_name) in &chart_types {
                        if ui
                            .selectable_value(
                                &mut self.state.selected_chart_type,
                                *type_id,
                                type_name,
                            )
                            .changed()
                        {
                            self.state.set_chart_type(*type_id);
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
                            if ui
                                .selectable_value(
                                    &mut self.state.selected_x_col,
                                    Some(col.clone()),
                                    col,
                                )
                                .changed()
                            {
                                self.state.rebuild_canvas();
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
                            if ui
                                .selectable_value(
                                    &mut self.state.selected_y_col,
                                    Some(col.clone()),
                                    col,
                                )
                                .changed()
                            {
                                self.state.rebuild_canvas();
                            }
                        }
                    });
            });
        }

        ui.separator();
        ui.label(&self.status_message);
    }

    fn render_plot(&self, ui: &mut egui::Ui) {
        if self.state.canvas.is_none() {
            ui.label("No data to display");
            return;
        }

        ui.label("Chart renders via canvas");
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
