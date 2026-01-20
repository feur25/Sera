use crate::wiki::{WikiExport, MethodDoc, ModuleDoc, ParamDoc, CodeExample};

pub fn generate_seraplot_docs() -> WikiExport {
    let mut export = WikiExport::new("SeraPlot", env!("CARGO_PKG_VERSION"));
    
    let core_methods = vec![
        MethodDoc {
            name: "version".to_string(),
            module: "Core".to_string(),
            description: "Returns the current version of SeraPlot".to_string(),
            parameters: vec![],
            returns: Some("String".to_string()),
            examples: vec![CodeExample::new(
                "print(seraplot.version())",
                "Console.WriteLine(SeraPlot.Version());",
                "std::cout << sera_version() << std::endl;",
                "println!(\"{}\", SeraPlot::version());",
            )],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
            python_signature: "def version() -> str".to_string(),
            csharp_signature: "public static string Version()".to_string(),
            cpp_signature: "const char* sera_version()".to_string(),
            rust_signature: "pub fn version() -> String".to_string(),
        },
        MethodDoc {
            name: "load_csv".to_string(),
            module: "Core".to_string(),
            description: "Loads CSV file with type detection".to_string(),
            parameters: vec![
                ParamDoc { name: "path".to_string(), param_type: "str".to_string(), description: "Path to CSV".to_string() },
            ],
            returns: Some("CsvData".to_string()),
            examples: vec![CodeExample::new(
                "data = seraplot.load_csv('data.csv')",
                "var data = SeraPlot.LoadCsv(\"data.csv\");",
                "auto data = sera_load_csv(\"data.csv\");",
                "let data = SeraPlot::load_csv(\"data.csv\");",
            )],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
            python_signature: "def load_csv(path: str) -> CsvData".to_string(),
            csharp_signature: "public static CsvData LoadCsv(string path)".to_string(),
            cpp_signature: "sera_csv_data_t* sera_load_csv(const char* path)".to_string(),
            rust_signature: "pub fn load_csv<P: AsRef<Path>>(path: P) -> Result<CsvData>".to_string(),
        },
    ];
    
    let canvas_methods = vec![
        MethodDoc {
            name: "new".to_string(),
            module: "Canvas".to_string(),
            description: "Creates canvas with dimensions".to_string(),
            parameters: vec![
                ParamDoc { name: "width".to_string(), param_type: "float".to_string(), description: "Width".to_string() },
                ParamDoc { name: "height".to_string(), param_type: "float".to_string(), description: "Height".to_string() },
            ],
            returns: Some("Canvas".to_string()),
            examples: vec![CodeExample::new(
                "canvas = seraplot.Canvas(1200, 600)",
                "var canvas = new Canvas(1200, 600);",
                "auto canvas = sera_canvas_create(1200, 600);",
                "let canvas = SeraPlot::new(1200, 600);",
            )],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
            python_signature: "def __init__(self, width: float, height: float)".to_string(),
            csharp_signature: "public Canvas(float width, float height)".to_string(),
            cpp_signature: "sera_canvas_t* sera_canvas_create(float w, float h)".to_string(),
            rust_signature: "pub fn new(width: f32, height: f32) -> Canvas".to_string(),
        },
        MethodDoc {
            name: "add_trace".to_string(),
            module: "Canvas".to_string(),
            description: "Adds trace to canvas".to_string(),
            parameters: vec![
                ParamDoc { name: "trace".to_string(), param_type: "Trace".to_string(), description: "Trace".to_string() },
            ],
            returns: Some("Canvas".to_string()),
            examples: vec![CodeExample::new(
                "canvas = canvas.add_trace(trace)",
                "canvas = canvas.AddTrace(trace);",
                "canvas = sera_canvas_add_trace(canvas, trace);",
                "canvas = canvas.add_trace(trace);",
            )],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
            python_signature: "def add_trace(self, trace: Trace) -> Canvas".to_string(),
            csharp_signature: "public Canvas AddTrace(Trace trace)".to_string(),
            cpp_signature: "sera_canvas_t* sera_canvas_add_trace(sera_canvas_t* c, sera_trace_t* t)".to_string(),
            rust_signature: "pub fn add_trace(mut self, trace: Trace) -> Self".to_string(),
        },
        MethodDoc {
            name: "set_title".to_string(),
            module: "Canvas".to_string(),
            description: "Sets chart title".to_string(),
            parameters: vec![
                ParamDoc { name: "title".to_string(), param_type: "str".to_string(), description: "Title".to_string() },
            ],
            returns: Some("Canvas".to_string()),
            examples: vec![CodeExample::new(
                "canvas = canvas.set_title('My Chart')",
                "canvas = canvas.SetTitle(\"My Chart\");",
                "canvas = sera_canvas_set_title(canvas, \"My Chart\");",
                "canvas = canvas.set_title(\"My Chart\");",
            )],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
            python_signature: "def set_title(self, title: str) -> Canvas".to_string(),
            csharp_signature: "public Canvas SetTitle(string title)".to_string(),
            cpp_signature: "sera_canvas_t* sera_canvas_set_title(sera_canvas_t* c, const char* t)".to_string(),
            rust_signature: "pub fn set_title(mut self, title: &str) -> Self".to_string(),
        },
        MethodDoc {
            name: "render".to_string(),
            module: "Canvas".to_string(),
            description: "Renders to SVG".to_string(),
            parameters: vec![],
            returns: Some("String".to_string()),
            examples: vec![CodeExample::new(
                "svg = canvas.render()",
                "string svg = canvas.Render();",
                "const char* svg = sera_canvas_render(canvas);",
                "let svg = canvas.render();",
            )],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
            python_signature: "def render(self) -> str".to_string(),
            csharp_signature: "public string Render()".to_string(),
            cpp_signature: "const char* sera_canvas_render(sera_canvas_t* c)".to_string(),
            rust_signature: "pub fn render(&self) -> String".to_string(),
        },
    ];
    
    let trace_methods = vec![
        MethodDoc {
            name: "new".to_string(),
            module: "Trace".to_string(),
            description: "Creates trace with values".to_string(),
            parameters: vec![
                ParamDoc { name: "label".to_string(), param_type: "str".to_string(), description: "Label".to_string() },
                ParamDoc { name: "values".to_string(), param_type: "List[float]".to_string(), description: "Values".to_string() },
            ],
            returns: Some("Trace".to_string()),
            examples: vec![CodeExample::new(
                "trace = seraplot.Trace('Data', [1, 2, 3])",
                "var trace = new Trace(\"Data\", new[] {1f, 2f, 3f});",
                "auto trace = sera_trace_create(\"Data\");",
                "let trace = SeraPlot::new(\"Data\", vec![1.0, 2.0, 3.0]);",
            )],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
            python_signature: "class Trace:\n    def __init__(self, label: str, values: List[float])".to_string(),
            csharp_signature: "public class Trace {\n    public Trace(string label, float[] values)".to_string(),
            cpp_signature: "typedef struct sera_trace sera_trace_t;".to_string(),
            rust_signature: "pub fn new(label: &str, values: Vec<f64>) -> Trace".to_string(),
        },
        MethodDoc {
            name: "add_value".to_string(),
            module: "Trace".to_string(),
            description: "Adds value to trace".to_string(),
            parameters: vec![
                ParamDoc { name: "value".to_string(), param_type: "float".to_string(), description: "Value".to_string() },
            ],
            returns: Some("Trace".to_string()),
            examples: vec![CodeExample::new(
                "trace = trace.add_value(4.0)",
                "trace = trace.AddValue(4.0f);",
                "sera_trace_add_value(trace, 4.0f);",
                "trace = trace.add_value(4.0);",
            )],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
            python_signature: "def add_value(self, value: float) -> Trace".to_string(),
            csharp_signature: "public Trace AddValue(float value)".to_string(),
            cpp_signature: "void sera_trace_add_value(sera_trace_t* t, float v)".to_string(),
            rust_signature: "pub fn add_value(mut self, value: f64) -> Self".to_string(),
        },
    ];
    
    let csv_methods = vec![
        MethodDoc {
            name: "get_column".to_string(),
            module: "CsvData".to_string(),
            description: "Gets column as strings".to_string(),
            parameters: vec![
                ParamDoc { name: "name".to_string(), param_type: "str".to_string(), description: "Column".to_string() },
            ],
            returns: Some("List[str]".to_string()),
            examples: vec![CodeExample::new(
                "names = data.get_column('name')",
                "var names = data.GetColumn(\"name\");",
                "auto names = sera_csv_get_column(data, \"name\");",
                "let names = data.get_column(\"name\");",
            )],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
            python_signature: "def get_column(self, name: str) -> List[str]".to_string(),
            csharp_signature: "public List<string> GetColumn(string name)".to_string(),
            cpp_signature: "sera_string_array_t sera_csv_get_column(sera_csv_data_t* d, const char* n)".to_string(),
            rust_signature: "pub fn get_column(&self, name: &str) -> Option<Vec<String>>".to_string(),
        },
        MethodDoc {
            name: "get_numeric_column".to_string(),
            module: "CsvData".to_string(),
            description: "Gets numeric column".to_string(),
            parameters: vec![
                ParamDoc { name: "name".to_string(), param_type: "str".to_string(), description: "Column".to_string() },
            ],
            returns: Some("List[float]".to_string()),
            examples: vec![CodeExample::new(
                "values = data.get_numeric_column('price')",
                "var values = data.GetNumericColumn(\"price\");",
                "auto values = sera_csv_get_numeric(data, \"price\");",
                "let values = data.get_numeric_column(\"price\");",
            )],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
            python_signature: "def get_numeric_column(self, name: str) -> List[float]".to_string(),
            csharp_signature: "public List<float> GetNumericColumn(string name)".to_string(),
            cpp_signature: "sera_float_array_t sera_csv_get_numeric(sera_csv_data_t* d, const char* n)".to_string(),
            rust_signature: "pub fn get_numeric_column(&self, name: &str) -> Option<Vec<f64>>".to_string(),
        },
    ];
    
    let processor_methods = vec![
        MethodDoc {
            name: "filter".to_string(),
            module: "Processor".to_string(),
            description: "Filters data by threshold".to_string(),
            parameters: vec![
                ParamDoc { name: "data".to_string(), param_type: "List[float]".to_string(), description: "Values".to_string() },
                ParamDoc { name: "threshold".to_string(), param_type: "float".to_string(), description: "Threshold".to_string() },
            ],
            returns: Some("List[float]".to_string()),
            examples: vec![CodeExample::new(
                "filtered = seraplot.processor.filter(values, 10.0)",
                "var filtered = SeraPlot.Processor.Filter(values, 10.0f);",
                "auto filtered = sera_processor_filter(values, 10.0f);",
                "let filtered = SeraPlot::filter(&values, 10.0);",
            )],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
            python_signature: "def filter(data: List[float], threshold: float) -> List[float]".to_string(),
            csharp_signature: "public static List<float> Filter(List<float> data, float t)".to_string(),
            cpp_signature: "sera_float_array_t sera_processor_filter(sera_float_array_t d, float t)".to_string(),
            rust_signature: "pub fn filter(data: &[f64], threshold: f64) -> Vec<f64>".to_string(),
        },
        MethodDoc {
            name: "sort".to_string(),
            module: "Processor".to_string(),
            description: "Sorts data ascending or descending".to_string(),
            parameters: vec![
                ParamDoc { name: "data".to_string(), param_type: "List[float]".to_string(), description: "Values".to_string() },
                ParamDoc { name: "order".to_string(), param_type: "str".to_string(), description: "'asc' or 'desc'".to_string() },
            ],
            returns: Some("List[float]".to_string()),
            examples: vec![CodeExample::new(
                "sorted = seraplot.processor.sort(values, 'asc')",
                "var sorted = SeraPlot.Processor.Sort(values, \"asc\");",
                "auto sorted = sera_processor_sort(values, 1);",
                "let sorted = SeraPlot::sort(&values, \"asc\");",
            )],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
            python_signature: "def sort(data: List[float], order: str) -> List[float]".to_string(),
            csharp_signature: "public static List<float> Sort(List<float> data, string order)".to_string(),
            cpp_signature: "sera_float_array_t sera_processor_sort(sera_float_array_t d, int o)".to_string(),
            rust_signature: "pub fn sort(data: &mut [f64], order: &str)".to_string(),
        },
    ];
    
    let render_methods = vec![
        MethodDoc {
            name: "to_svg".to_string(),
            module: "Renderer".to_string(),
            description: "Renders to SVG format".to_string(),
            parameters: vec![
                ParamDoc { name: "canvas".to_string(), param_type: "Canvas".to_string(), description: "Canvas".to_string() },
            ],
            returns: Some("String".to_string()),
            examples: vec![CodeExample::new(
                "svg = seraplot.render.to_svg(canvas)",
                "string svg = SeraPlot.Renderer.ToSvg(canvas);",
                "const char* svg = sera_render_svg(canvas);",
                "let svg = canvas.render();",
            )],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
            python_signature: "def to_svg(canvas: Canvas) -> str".to_string(),
            csharp_signature: "public static string ToSvg(Canvas canvas)".to_string(),
            cpp_signature: "const char* sera_render_svg(sera_canvas_t* c)".to_string(),
            rust_signature: "pub fn to_svg(canvas: &Canvas) -> String".to_string(),
        },
        MethodDoc {
            name: "to_html".to_string(),
            module: "Renderer".to_string(),
            description: "Renders to standalone HTML".to_string(),
            parameters: vec![
                ParamDoc { name: "canvas".to_string(), param_type: "Canvas".to_string(), description: "Canvas".to_string() },
                ParamDoc { name: "title".to_string(), param_type: "str".to_string(), description: "Title".to_string() },
            ],
            returns: Some("String".to_string()),
            examples: vec![CodeExample::new(
                "html = seraplot.render.to_html(canvas, 'Page')",
                "string html = SeraPlot.Renderer.ToHtml(canvas, \"Page\");",
                "const char* html = sera_render_html(canvas, \"Page\");",
                "let html = canvas.render_html(\"Page\");",
            )],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
            python_signature: "def to_html(canvas: Canvas, title: str) -> str".to_string(),
            csharp_signature: "public static string ToHtml(Canvas c, string title)".to_string(),
            cpp_signature: "const char* sera_render_html(sera_canvas_t* c, const char* t)".to_string(),
            rust_signature: "pub fn to_html(canvas: &Canvas, title: &str) -> String".to_string(),
        },
        MethodDoc {
            name: "to_json".to_string(),
            module: "Renderer".to_string(),
            description: "Renders to JSON (Plotly compatible)".to_string(),
            parameters: vec![
                ParamDoc { name: "canvas".to_string(), param_type: "Canvas".to_string(), description: "Canvas".to_string() },
            ],
            returns: Some("String".to_string()),
            examples: vec![CodeExample::new(
                "json = seraplot.render.to_json(canvas)",
                "string json = SeraPlot.Renderer.ToJson(canvas);",
                "const char* json = sera_render_json(canvas);",
                "let json = canvas.to_json();",
            )],
            since_version: Some("0.1.0".to_string()),
            deprecated: false,
            python_signature: "def to_json(canvas: Canvas) -> str".to_string(),
            csharp_signature: "public static string ToJson(Canvas canvas)".to_string(),
            cpp_signature: "const char* sera_render_json(sera_canvas_t* c)".to_string(),
            rust_signature: "pub fn to_json(canvas: &Canvas) -> Result<String>".to_string(),
        },
    ];
    
    export.add_module(ModuleDoc {
        name: "Core".to_string(),
        description: "Version and CSV loading".to_string(),
        methods: core_methods,
    });
    
    export.add_module(ModuleDoc {
        name: "Canvas".to_string(),
        description: "Main drawing surface".to_string(),
        methods: canvas_methods,
    });
    
    export.add_module(ModuleDoc {
        name: "Trace".to_string(),
        description: "Dataset representation".to_string(),
        methods: trace_methods,
    });
    
    export.add_module(ModuleDoc {
        name: "CsvData".to_string(),
        description: "CSV operations".to_string(),
        methods: csv_methods,
    });
    
    export.add_module(ModuleDoc {
        name: "Processor".to_string(),
        description: "Data filtering and sorting".to_string(),
        methods: processor_methods,
    });
    
    export.add_module(ModuleDoc {
        name: "Renderer".to_string(),
        description: "Multi-format rendering".to_string(),
        methods: render_methods,
    });
    
    export
}
