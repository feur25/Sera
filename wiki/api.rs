use crate::wiki::{WikiExport, MethodDoc, ModuleDoc, ParamDoc, WikiExtractor};

pub fn generate_seraplot_docs() -> WikiExport {
    let mut export = WikiExport::new("SeraPlot", env!("CARGO_PKG_VERSION"));
    
    let seraplot_methods = vec![
        MethodDoc {
            name: "version".to_string(),
            module: "SeraPlot".to_string(),
            signature: "pub fn version() -> &'static str".to_string(),
            description: "Returns the current version of the SeraPlot framework".to_string(),
            parameters: vec![],
            returns: Some("&'static str".to_string()),
            examples: vec![
                "let v = SeraPlot::version();\nprintln!(\"SeraPlot {}\", v);".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "new_canvas".to_string(),
            module: "SeraPlot".to_string(),
            signature: "pub fn new_canvas(width: f32, height: f32) -> Canvas".to_string(),
            description: "Creates a new canvas with specified dimensions and default layout".to_string(),
            parameters: vec![
                ParamDoc { name: "width".to_string(), param_type: "f32".to_string(), description: "Canvas width in pixels".to_string() },
                ParamDoc { name: "height".to_string(), param_type: "f32".to_string(), description: "Canvas height in pixels".to_string() },
            ],
            returns: Some("Canvas".to_string()),
            examples: vec![
                "let canvas = SeraPlot::new_canvas(1200.0, 600.0);\nlet with_trace = canvas.add_trace(trace);".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "load_csv".to_string(),
            module: "SeraPlot".to_string(),
            signature: "pub fn load_csv<P: AsRef<std::path::Path>>(path: P) -> Result<crate::data::loader::CsvData, Box<dyn std::error::Error>>".to_string(),
            description: "Loads CSV data from a file with automatic numeric column detection".to_string(),
            parameters: vec![
                ParamDoc { name: "path".to_string(), param_type: "P".to_string(), description: "File path to CSV file".to_string() },
            ],
            returns: Some("Result<CsvData, Box<dyn Error>>".to_string()),
            examples: vec![
                "let data = SeraPlot::load_csv(\"ammos.csv\")?;\nlet physical_dmg = data.get_numeric_column(\"physical\");".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
    ];
    
    let canvas_methods = vec![
        MethodDoc {
            name: "new".to_string(),
            module: "Canvas".to_string(),
            signature: "pub fn new(width: f32, height: f32, layout: Layout) -> Self".to_string(),
            description: "Constructs a new canvas with given dimensions and layout configuration".to_string(),
            parameters: vec![
                ParamDoc { name: "width".to_string(), param_type: "f32".to_string(), description: "Canvas width in pixels".to_string() },
                ParamDoc { name: "height".to_string(), param_type: "f32".to_string(), description: "Canvas height in pixels".to_string() },
                ParamDoc { name: "layout".to_string(), param_type: "Layout".to_string(), description: "Layout configuration for the canvas".to_string() },
            ],
            returns: Some("Canvas".to_string()),
            examples: vec![
                "let layout = Layout::default();\nlet canvas = Canvas::new(1200.0, 600.0, layout);".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "add_trace".to_string(),
            module: "Canvas".to_string(),
            signature: "pub fn add_trace(self, trace: Trace) -> Self".to_string(),
            description: "Adds a trace to the canvas and returns the modified canvas for method chaining".to_string(),
            parameters: vec![
                ParamDoc { name: "trace".to_string(), param_type: "Trace".to_string(), description: "The trace to add to the canvas".to_string() },
            ],
            returns: Some("Canvas".to_string()),
            examples: vec![
                "let canvas = Canvas::new(1200.0, 600.0, Layout::default())\n    .add_trace(line_trace)\n    .add_trace(scatter_trace);".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "add_traces".to_string(),
            module: "Canvas".to_string(),
            signature: "pub fn add_traces(self, traces: Vec<Trace>) -> Self".to_string(),
            description: "Adds multiple traces to the canvas at once".to_string(),
            parameters: vec![
                ParamDoc { name: "traces".to_string(), param_type: "Vec<Trace>".to_string(), description: "Vector of traces to add".to_string() },
            ],
            returns: Some("Canvas".to_string()),
            examples: vec![
                "let traces = vec![trace1, trace2, trace3];\nlet canvas = Canvas::new(1200.0, 600.0, Layout::default()).add_traces(traces);".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "add_trace_mut".to_string(),
            module: "Canvas".to_string(),
            signature: "pub fn add_trace_mut(&mut self, trace: Trace)".to_string(),
            description: "Adds a trace to the canvas using mutable reference".to_string(),
            parameters: vec![
                ParamDoc { name: "trace".to_string(), param_type: "Trace".to_string(), description: "The trace to add".to_string() },
            ],
            returns: None,
            examples: vec![
                "let mut canvas = Canvas::new(1200.0, 600.0, Layout::default());\ncanvas.add_trace_mut(trace);".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "set_hover".to_string(),
            module: "Canvas".to_string(),
            signature: "pub fn set_hover(&mut self, x: f32, y: f32)".to_string(),
            description: "Sets the hover point position on the canvas".to_string(),
            parameters: vec![
                ParamDoc { name: "x".to_string(), param_type: "f32".to_string(), description: "X coordinate of hover point".to_string() },
                ParamDoc { name: "y".to_string(), param_type: "f32".to_string(), description: "Y coordinate of hover point".to_string() },
            ],
            returns: None,
            examples: vec![
                "canvas.set_hover(150.0, 300.0);".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "select_point".to_string(),
            module: "Canvas".to_string(),
            signature: "pub fn select_point(&mut self, trace_id: &str, idx: usize)".to_string(),
            description: "Selects a specific point in a trace by index".to_string(),
            parameters: vec![
                ParamDoc { name: "trace_id".to_string(), param_type: "&str".to_string(), description: "ID of the trace containing the point".to_string() },
                ParamDoc { name: "idx".to_string(), param_type: "usize".to_string(), description: "Index of the point to select".to_string() },
            ],
            returns: None,
            examples: vec![
                "canvas.select_point(\"line_1\", 5);".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "clear_selection".to_string(),
            module: "Canvas".to_string(),
            signature: "pub fn clear_selection(&mut self)".to_string(),
            description: "Clears all selected points on the canvas".to_string(),
            parameters: vec![],
            returns: None,
            examples: vec![
                "canvas.clear_selection();".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "traces".to_string(),
            module: "Canvas".to_string(),
            signature: "pub fn traces(&self) -> &[Trace]".to_string(),
            description: "Returns a slice of all traces in the canvas".to_string(),
            parameters: vec![],
            returns: Some("&[Trace]".to_string()),
            examples: vec![
                "for trace in canvas.traces() {\n    println!(\"Trace: {}\", trace.name);\n}".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "layout".to_string(),
            module: "Canvas".to_string(),
            signature: "pub fn layout(&self) -> &Layout".to_string(),
            description: "Returns a reference to the canvas layout configuration".to_string(),
            parameters: vec![],
            returns: Some("&Layout".to_string()),
            examples: vec![
                "let layout = canvas.layout();\nprintln!(\"Title: {}\", layout.title);".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "dimensions".to_string(),
            module: "Canvas".to_string(),
            signature: "pub fn dimensions(&self) -> (f32, f32)".to_string(),
            description: "Returns the canvas width and height dimensions".to_string(),
            parameters: vec![],
            returns: Some("(f32, f32)".to_string()),
            examples: vec![
                "let (width, height) = canvas.dimensions();\nprintln!(\"{}x{}\", width, height);".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "auto_scale".to_string(),
            module: "Canvas".to_string(),
            signature: "pub fn auto_scale(&mut self)".to_string(),
            description: "Automatically scales axes to fit all trace data with appropriate bounds".to_string(),
            parameters: vec![],
            returns: None,
            examples: vec![
                "canvas.auto_scale();".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "zoom".to_string(),
            module: "Canvas".to_string(),
            signature: "pub fn zoom(&mut self, center_x: f64, center_y: f64, factor: f32)".to_string(),
            description: "Zooms the canvas around a center point with the specified factor".to_string(),
            parameters: vec![
                ParamDoc { name: "center_x".to_string(), param_type: "f64".to_string(), description: "X coordinate of zoom center in data space".to_string() },
                ParamDoc { name: "center_y".to_string(), param_type: "f64".to_string(), description: "Y coordinate of zoom center in data space".to_string() },
                ParamDoc { name: "factor".to_string(), param_type: "f32".to_string(), description: "Zoom factor (>1.0 zooms in, <1.0 zooms out)".to_string() },
            ],
            returns: None,
            examples: vec![
                "canvas.zoom(10.0, 20.0, 1.5);".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "pan".to_string(),
            module: "Canvas".to_string(),
            signature: "pub fn pan(&mut self, dx: f64, dy: f64)".to_string(),
            description: "Pans the canvas by the specified offset in data space".to_string(),
            parameters: vec![
                ParamDoc { name: "dx".to_string(), param_type: "f64".to_string(), description: "X offset in data space".to_string() },
                ParamDoc { name: "dy".to_string(), param_type: "f64".to_string(), description: "Y offset in data space".to_string() },
            ],
            returns: None,
            examples: vec![
                "canvas.pan(5.0, -10.0);".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "pixel_to_data".to_string(),
            module: "Canvas".to_string(),
            signature: "pub fn pixel_to_data(&self, px: f32, py: f32) -> Option<(f64, f64)>".to_string(),
            description: "Converts pixel coordinates to data space coordinates".to_string(),
            parameters: vec![
                ParamDoc { name: "px".to_string(), param_type: "f32".to_string(), description: "Pixel X coordinate".to_string() },
                ParamDoc { name: "py".to_string(), param_type: "f32".to_string(), description: "Pixel Y coordinate".to_string() },
            ],
            returns: Some("Option<(f64, f64)>".to_string()),
            examples: vec![
                "if let Some((x, y)) = canvas.pixel_to_data(400.0, 300.0) {\n    println!(\"Data: ({}, {})\", x, y);\n}".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "data_to_pixel".to_string(),
            module: "Canvas".to_string(),
            signature: "pub fn data_to_pixel(&self, x: f64, y: f64) -> Option<(f32, f32)>".to_string(),
            description: "Converts data space coordinates to pixel coordinates".to_string(),
            parameters: vec![
                ParamDoc { name: "x".to_string(), param_type: "f64".to_string(), description: "X coordinate in data space".to_string() },
                ParamDoc { name: "y".to_string(), param_type: "f64".to_string(), description: "Y coordinate in data space".to_string() },
            ],
            returns: Some("Option<(f32, f32)>".to_string()),
            examples: vec![
                "if let Some((px, py)) = canvas.data_to_pixel(15.0, 25.0) {\n    println!(\"Pixel: ({}, {})\", px, py);\n}".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "nearest_point".to_string(),
            module: "Canvas".to_string(),
            signature: "pub fn nearest_point(&self, data_x: f64, data_y: f64, max_distance: f32) -> Option<(String, usize, f32)>".to_string(),
            description: "Finds the nearest point to given data coordinates within maximum distance".to_string(),
            parameters: vec![
                ParamDoc { name: "data_x".to_string(), param_type: "f64".to_string(), description: "X coordinate in data space".to_string() },
                ParamDoc { name: "data_y".to_string(), param_type: "f64".to_string(), description: "Y coordinate in data space".to_string() },
                ParamDoc { name: "max_distance".to_string(), param_type: "f32".to_string(), description: "Maximum search distance in pixels".to_string() },
            ],
            returns: Some("Option<(String, usize, f32)>".to_string()),
            examples: vec![
                "if let Some((trace_id, idx, dist)) = canvas.nearest_point(10.0, 20.0, 50.0) {\n    println!(\"Nearest: {} point {} dist {}\", trace_id, idx, dist);\n}".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
    ];
    
    let tracebuilder_methods = vec![
        MethodDoc {
            name: "new".to_string(),
            module: "TraceBuilder".to_string(),
            signature: "pub fn new<S: Into<String>>(name: S, kind: ChartKind) -> Self".to_string(),
            description: "Creates a new trace builder with the specified name and chart type".to_string(),
            parameters: vec![
                ParamDoc { name: "name".to_string(), param_type: "S".to_string(), description: "Name of the trace".to_string() },
                ParamDoc { name: "kind".to_string(), param_type: "ChartKind".to_string(), description: "Type of chart (Line, Scatter, Bar, etc)".to_string() },
            ],
            returns: Some("TraceBuilder".to_string()),
            examples: vec![
                "let builder = TraceBuilder::new(\"Data Series\", ChartKind::Line);".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "add_point".to_string(),
            module: "TraceBuilder".to_string(),
            signature: "pub fn add_point(self, x: f64, y: f64) -> Self".to_string(),
            description: "Adds a single data point to the trace builder".to_string(),
            parameters: vec![
                ParamDoc { name: "x".to_string(), param_type: "f64".to_string(), description: "X coordinate of the point".to_string() },
                ParamDoc { name: "y".to_string(), param_type: "f64".to_string(), description: "Y coordinate of the point".to_string() },
            ],
            returns: Some("TraceBuilder".to_string()),
            examples: vec![
                "let trace = TraceBuilder::new(\"Data\", ChartKind::Scatter)\n    .add_point(1.0, 2.0)\n    .add_point(2.0, 4.0)\n    .build();".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "add_points".to_string(),
            module: "TraceBuilder".to_string(),
            signature: "pub fn add_points(self, x: Vec<f64>, y: Vec<f64>) -> Self".to_string(),
            description: "Adds multiple data points to the trace builder".to_string(),
            parameters: vec![
                ParamDoc { name: "x".to_string(), param_type: "Vec<f64>".to_string(), description: "Vector of X coordinates".to_string() },
                ParamDoc { name: "y".to_string(), param_type: "Vec<f64>".to_string(), description: "Vector of Y coordinates".to_string() },
            ],
            returns: Some("TraceBuilder".to_string()),
            examples: vec![
                "let x = vec![1.0, 2.0, 3.0];\nlet y = vec![2.0, 4.0, 6.0];\nlet trace = TraceBuilder::new(\"Data\", ChartKind::Line)\n    .add_points(x, y)\n    .build();".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "build".to_string(),
            module: "TraceBuilder".to_string(),
            signature: "pub fn build(self) -> Trace".to_string(),
            description: "Builds the final Trace from the accumulated builder data".to_string(),
            parameters: vec![],
            returns: Some("Trace".to_string()),
            examples: vec![
                "let trace = TraceBuilder::new(\"Series\", ChartKind::Bar)\n    .add_point(1.0, 10.0)\n    .add_point(2.0, 20.0)\n    .build();".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
    ];
    
    let csvdata_methods = vec![
        MethodDoc {
            name: "load".to_string(),
            module: "CsvData".to_string(),
            signature: "pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, csv::Error>".to_string(),
            description: "Loads CSV data from file with automatic numeric column detection".to_string(),
            parameters: vec![
                ParamDoc { name: "path".to_string(), param_type: "P".to_string(), description: "Path to the CSV file".to_string() },
            ],
            returns: Some("Result<CsvData, csv::Error>".to_string()),
            examples: vec![
                "let data = CsvData::load(\"data.csv\")?;\nprintln!(\"Loaded {} rows\", data.len());".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "get_numeric_column".to_string(),
            module: "CsvData".to_string(),
            signature: "pub fn get_numeric_column(&self, col: &str) -> Vec<f64>".to_string(),
            description: "Extracts a column as numeric values, skipping invalid entries".to_string(),
            parameters: vec![
                ParamDoc { name: "col".to_string(), param_type: "&str".to_string(), description: "Name of the column to extract".to_string() },
            ],
            returns: Some("Vec<f64>".to_string()),
            examples: vec![
                "let values = data.get_numeric_column(\"price\");".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "get_string_column".to_string(),
            module: "CsvData".to_string(),
            signature: "pub fn get_string_column(&self, col: &str) -> Vec<String>".to_string(),
            description: "Extracts a column as string values".to_string(),
            parameters: vec![
                ParamDoc { name: "col".to_string(), param_type: "&str".to_string(), description: "Name of the column to extract".to_string() },
            ],
            returns: Some("Vec<String>".to_string()),
            examples: vec![
                "let names = data.get_string_column(\"product_name\");".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "len".to_string(),
            module: "CsvData".to_string(),
            signature: "pub fn len(&self) -> usize".to_string(),
            description: "Returns the number of rows in the CSV data".to_string(),
            parameters: vec![],
            returns: Some("usize".to_string()),
            examples: vec![
                "let row_count = data.len();".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
        MethodDoc {
            name: "is_empty".to_string(),
            module: "CsvData".to_string(),
            signature: "pub fn is_empty(&self) -> bool".to_string(),
            description: "Checks if the CSV data contains no rows".to_string(),
            parameters: vec![],
            returns: Some("bool".to_string()),
            examples: vec![
                "if !data.is_empty() {\n    println!(\"Data loaded successfully\");\n}".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
    ];
    
    let svg_methods = vec![
        MethodDoc {
            name: "render_traces".to_string(),
            module: "SvgRenderer".to_string(),
            signature: "pub fn render_traces(traces: &[Trace], layout: &Layout) -> String".to_string(),
            description: "Renders traces as SVG markup suitable for web viewing".to_string(),
            parameters: vec![
                ParamDoc { name: "traces".to_string(), param_type: "&[Trace]".to_string(), description: "Traces to render".to_string() },
                ParamDoc { name: "layout".to_string(), param_type: "&Layout".to_string(), description: "Layout configuration for the SVG".to_string() },
            ],
            returns: Some("String".to_string()),
            examples: vec![
                "let svg = SvgRenderer::render_traces(&canvas.traces(), &canvas.layout());\nstd::fs::write(\"plot.svg\", svg)?;".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
    ];
    
    let json_methods = vec![
        MethodDoc {
            name: "render".to_string(),
            module: "JsonRenderer".to_string(),
            signature: "pub fn render(traces: &[Trace], layout: &Layout) -> Result<String, Box<dyn std::error::Error>>".to_string(),
            description: "Renders traces as JSON compatible with Plotly.js".to_string(),
            parameters: vec![
                ParamDoc { name: "traces".to_string(), param_type: "&[Trace]".to_string(), description: "Traces to render".to_string() },
                ParamDoc { name: "layout".to_string(), param_type: "&Layout".to_string(), description: "Layout configuration for the JSON".to_string() },
            ],
            returns: Some("Result<String, Box<dyn Error>>".to_string()),
            examples: vec![
                "let json = JsonRenderer::render(&canvas.traces(), &canvas.layout())?;\nstd::fs::write(\"plot.json\", json)?;".to_string(),
            ],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
        },
    ];
    
    export.add_module(ModuleDoc {
        name: "SeraPlot".to_string(),
        description: "Main entry point for the SeraPlot framework with utility methods".to_string(),
        methods: seraplot_methods,
    });
    
    export.add_module(ModuleDoc {
        name: "Canvas".to_string(),
        description: "Primary drawing surface for creating and managing interactive plots".to_string(),
        methods: canvas_methods,
    });
    
    export.add_module(ModuleDoc {
        name: "TraceBuilder".to_string(),
        description: "Builder pattern for constructing traces with fluent API".to_string(),
        methods: tracebuilder_methods,
    });
    
    export.add_module(ModuleDoc {
        name: "CsvData".to_string(),
        description: "CSV data loading and column extraction utilities".to_string(),
        methods: csvdata_methods,
    });
    
    export.add_module(ModuleDoc {
        name: "SvgRenderer".to_string(),
        description: "SVG rendering engine for web-compatible output".to_string(),
        methods: svg_methods,
    });
    
    export.add_module(ModuleDoc {
        name: "JsonRenderer".to_string(),
        description: "JSON rendering for Plotly.js and interactive viewers".to_string(),
        methods: json_methods,
    });
    
    export
}
