use serde::Deserialize;

#[derive(Deserialize, Default)]
struct DemoIn {
    variant: Option<String>,
}

fn parse_variant(input: &str) -> String {
    let v: DemoIn = serde_json::from_str(input).unwrap_or_default();
    v.variant.unwrap_or_else(|| "default".to_string())
}

fn cap(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

fn build(fn_name: &str, variant: &str, args: &str) -> String {
    let title = format!("{} demo", cap(variant));
    let var_kw = if variant == "default" || variant.is_empty() {
        String::new()
    } else {
        format!(",\n    variant=\"{}\"", variant)
    };
    format!(
        "import seraplot as sp\n\nc = sp.{}(\n    \"{}\",\n    {}{}\n)\n",
        fn_name, title, args, var_kw
    )
}

const BAR_SINGLE: &str = "labels=[\"Alpha\",\"Beta\",\"Gamma\",\"Delta\",\"Epsilon\"], values=[24,38,17,42,29]";
const BAR_MULTI: &str = "labels=[\"Q1\",\"Q2\",\"Q3\",\"Q4\"], series=[[24,38,17,42],[18,29,33,21],[12,15,28,30]], series_names=[\"Product A\",\"Product B\",\"Product C\"]";
const BAR_PICTO: &str = "labels=[\"Bikes\",\"Cars\",\"Buses\",\"Trains\"], values=[24,38,17,42], unit_description=\"units\", units_per_icon=2.0, icon_size=24, max_icons_per_column=10";
const BAR_MULTICAT: &str = "labels=[\"Q1\",\"Q2\",\"Q3\",\"Q4\"], series=[[24,38,17,42],[18,29,33,21]], series_names=[\"2023\",\"2024\"], super_categories=[\"H1\",\"H1\",\"H2\",\"H2\"]";

pub fn demo_bar(input: &str) -> String {
    let v = parse_variant(input);
    let args = match v.as_str() {
        "grouped" | "group" | "stacked" | "stack" | "relative" | "rel" | "grouped_stacked" | "groupstack" | "marimekko" | "mekko" | "mosaic" => BAR_MULTI,
        "multicategory" | "multi" | "hierarchical" => BAR_MULTICAT,
        "pictogram" | "icon" => BAR_PICTO,
        _ => BAR_SINGLE,
    };
    build("bar", &v, args)
}

pub fn demo_hbar(input: &str) -> String {
    let v = parse_variant(input);
    build("hbar", &v, "labels=[\"Apples\",\"Bananas\",\"Cherries\",\"Dates\"], values=[42,28,33,17]")
}

pub fn demo_line(input: &str) -> String {
    let v = parse_variant(input);
    let args = match v.as_str() {
        "multi" | "multiline" | "multiple" | "connected_scatter" | "markers" | "lines+markers" =>
            "x_labels=[\"Jan\",\"Feb\",\"Mar\",\"Apr\",\"May\"], series=[[12,18,25,22,30],[8,14,17,21,24]], series_names=[\"Revenue\",\"Profit\"]",
        _ => "x_labels=[\"Jan\",\"Feb\",\"Mar\",\"Apr\",\"May\",\"Jun\"], values=[12,18,25,22,30,28]",
    };
    build("line", &v, args)
}

pub fn demo_multiline(input: &str) -> String {
    let v = parse_variant(input);
    build("multiline", &v, "x_labels=[\"Jan\",\"Feb\",\"Mar\",\"Apr\",\"May\"], series=[[12,18,25,22,30],[8,14,17,21,24],[15,11,19,23,17]], series_names=[\"A\",\"B\",\"C\"]")
}

pub fn demo_area(input: &str) -> String {
    let v = parse_variant(input);
    build("area", &v, "x_labels=[\"Jan\",\"Feb\",\"Mar\",\"Apr\",\"May\"], series=[[12,18,25,22,30],[8,14,17,21,24]], series_names=[\"Revenue\",\"Profit\"]")
}

pub fn demo_scatter(input: &str) -> String {
    let v = parse_variant(input);
    let args = match v.as_str() {
        "categorical" | "grouped" | "groups" | "category" =>
            "x=[1.2,2.4,3.1,4.8,5.6,6.9,7.3,8.1,9.5], y=[2.1,3.4,5.2,4.6,6.8,7.1,8.4,9.0,9.8], categories=[\"A\",\"A\",\"A\",\"B\",\"B\",\"B\",\"C\",\"C\",\"C\"]",
        "gradient" | "colorscale" | "continuous" | "scaled" =>
            "x=[1.2,2.4,3.1,4.8,5.6,6.9,7.3,8.1,9.5], y=[2.1,3.4,5.2,4.6,6.8,7.1,8.4,9.0,9.8], color_values=[10,22,34,46,58,70,82,94,100]",
        "labeled" | "labels" | "text" | "annotated" =>
            "x=[1.2,2.4,3.1,4.8,5.6], y=[2.1,3.4,5.2,4.6,6.8], labels=[\"P1\",\"P2\",\"P3\",\"P4\",\"P5\"]",
        _ => "x=[1.2,2.4,3.1,4.8,5.6,6.9,7.3,8.1,9.5], y=[2.1,3.4,5.2,4.6,6.8,7.1,8.4,9.0,9.8]",
    };
    build("scatter", &v, args)
}

pub fn demo_bubble(input: &str) -> String {
    let v = parse_variant(input);
    build("bubble", &v, "x=[65000,12500,48000,33000,42000,28000], y=[78.5,77.1,81.3,80.2,79.4,76.8], sizes=[33,141,8,12,67,45], labels=[\"USA\",\"China\",\"Germany\",\"France\",\"Japan\",\"Brazil\"]")
}

pub fn demo_histogram(input: &str) -> String {
    let v = parse_variant(input);
    build("histogram", &v, "values=[1.2,2.4,2.7,3.1,3.5,3.6,4.0,4.2,4.5,4.8,5.0,5.2,5.5,5.7,6.1,6.4,6.8,7.2,7.5,8.0,3.2,4.1,5.3,4.6,3.9,4.7,5.1,4.3,3.8,5.6]")
}

pub fn demo_grouped_bar(input: &str) -> String {
    let v = parse_variant(input);
    build("grouped_bar", &v, "labels=[\"Q1\",\"Q2\",\"Q3\",\"Q4\"], values=[24,38,17,42,18,29,33,21,12,15,28,30], series_names=[\"Product A\",\"Product B\",\"Product C\"]")
}

pub fn demo_stacked_bar(input: &str) -> String {
    let v = parse_variant(input);
    build("stacked_bar", &v, "labels=[\"Q1\",\"Q2\",\"Q3\",\"Q4\"], values=[24,38,17,42,18,29,33,21,12,15,28,30], series_names=[\"A\",\"B\",\"C\"]")
}

pub fn demo_heatmap(input: &str) -> String {
    let v = parse_variant(input);
    build("heatmap", &v, "labels=[\"R1\",\"R2\",\"R3\",\"R4\"], col_labels=[\"C1\",\"C2\",\"C3\",\"C4\"], values=[1,5,3,7,2,8,4,6,9,3,5,2,4,6,7,1]")
}

pub fn demo_pie(input: &str) -> String {
    let v = parse_variant(input);
    let args = match v.as_str() {
        "subplots" | "grid" | "facet" | "multi" =>
            "labels=[\"A\",\"B\",\"C\",\"D\"], series=[[42,28,17,13],[35,30,20,15],[40,25,20,15]]",
        "nested" | "concentric" | "rings" | "double_ring" | "multi_ring" =>
            "labels=[\"Tech\",\"Finance\",\"Health\"], values=[45,30,25], secondary_labels=[\"Web\",\"Mobile\",\"Banks\",\"Pharma\",\"Devices\"], secondary_values=[20,25,18,12,15]",
        _ => "labels=[\"Apple\",\"Samsung\",\"Xiaomi\",\"Other\"], values=[42,28,17,13]",
    };
    build("pie", &v, args)
}

pub fn demo_donut(input: &str) -> String {
    let v = parse_variant(input);
    build("donut", &v, "labels=[\"Apple\",\"Samsung\",\"Xiaomi\",\"Other\"], values=[42,28,17,13]")
}

pub fn demo_boxplot(input: &str) -> String {
    let v = parse_variant(input);
    build("boxplot", &v, "labels=[\"Group A\",\"Group B\",\"Group C\"], series=[[1.2,2.4,2.7,3.1,3.5,3.8,4.2,5.1,6.0],[2.0,2.8,3.2,3.6,4.1,4.5,5.0,5.7,6.5],[1.8,2.2,2.6,3.0,3.4,3.9,4.3,4.9,5.5]]")
}

pub fn demo_violin(input: &str) -> String {
    let v = parse_variant(input);
    build("violin", &v, "labels=[\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\"], values=[1.2,2.4,2.7,3.1,3.5,3.8,4.2,5.1,6.0,2.0,2.8,3.2,3.6,4.1,4.5,5.0,5.7,6.5,1.8,2.2,2.6,3.0,3.4,3.9,4.3,4.9,5.5]")
}

pub fn demo_slope(input: &str) -> String {
    let v = parse_variant(input);
    build("slope", &v, "labels=[\"Alpha\",\"Beta\",\"Gamma\",\"Delta\"], left=[10,22,15,8], right=[18,16,25,14]")
}

pub fn demo_sunburst(input: &str) -> String {
    let v = parse_variant(input);
    build("sunburst", &v, "labels=[\"Tech\",\"Finance\",\"Health\",\"Web\",\"Mobile\",\"Banks\",\"Pharma\"], parents=[\"\",\"\",\"\",\"Tech\",\"Tech\",\"Finance\",\"Health\"], values=[0,0,0,30,40,50,35]")
}

pub fn demo_funnel(input: &str) -> String {
    let v = parse_variant(input);
    build("funnel", &v, "labels=[\"Visits\",\"Leads\",\"Trials\",\"Customers\"], values=[1000,520,210,85]")
}

pub fn demo_treemap(input: &str) -> String {
    let v = parse_variant(input);
    build("treemap", &v, "labels=[\"Apple\",\"Samsung\",\"Xiaomi\",\"Huawei\",\"Sony\",\"Other\"], values=[450,320,210,180,90,150]")
}

pub fn demo_waterfall(input: &str) -> String {
    let v = parse_variant(input);
    build("waterfall", &v, "labels=[\"Start\",\"Sales\",\"Returns\",\"Costs\",\"Tax\",\"Net\"], values=[1000,500,-150,-300,-100,950]")
}

pub fn demo_bullet(input: &str) -> String {
    let v = parse_variant(input);
    let args = match v.as_str() {
        "compare" | "vs" | "ghost" | "prior" =>
            "labels=[\"Sales\",\"Profit\",\"Growth\",\"NPS\"], values=[78,62,45,82], targets=[80,65,50,75], comparisons=[70,55,40,70], max_vals=[100,100,100,100]",
        "stacked" | "stacked_ranges" | "zones" | "qualitative" | "segmented" | "traffic" | "rag" | "zones_color" =>
            "labels=[\"Sales\",\"Profit\",\"Growth\",\"NPS\"], values=[78,62,45,82], targets=[80,65,50,75], ranges=[40,70,100,40,70,100,40,70,100,40,70,100], max_vals=[100,100,100,100]",
        _ => "labels=[\"Sales\",\"Profit\",\"Growth\",\"NPS\"], values=[78,62,45,82], targets=[80,65,50,75], max_vals=[100,100,100,100]",
    };
    build("bullet", &v, args)
}

pub fn demo_radar(input: &str) -> String {
    let v = parse_variant(input);
    build("radar", &v, "axes=[\"Speed\",\"Power\",\"Range\",\"Comfort\",\"Price\",\"Style\"], series=[[80,72,65,90,55,85],[65,85,80,70,75,60]], series_names=[\"Model A\",\"Model B\"]")
}

pub fn demo_lollipop(input: &str) -> String {
    let v = parse_variant(input);
    build("lollipop", &v, "labels=[\"Alpha\",\"Beta\",\"Gamma\",\"Delta\",\"Epsilon\"], values=[24,38,17,42,29]")
}

pub fn demo_kde(input: &str) -> String {
    let v = parse_variant(input);
    let args = match v.as_str() {
        "outline" | "line" | "stroke" | "compare" | "no_fill" =>
            "values=[1.2,2.1,2.4,2.7,3.0,3.1,3.3,3.5,3.7,3.8,4.0,4.2,4.4,4.5,4.7,4.8,5.0,5.2,5.4,5.7,6.0,6.3,6.7,7.1,2.5,3.2,3.8,4.1,4.6,5.1,5.5,6.0], categories=[\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\"]",
        _ => "values=[1.2,2.1,2.4,2.7,3.0,3.1,3.3,3.5,3.7,3.8,4.0,4.2,4.4,4.5,4.7,4.8,5.0,5.2,5.4,5.7,6.0,6.3,6.7,7.1]",
    };
    build("kde", &v, args)
}

pub fn demo_ridgeline(input: &str) -> String {
    let v = parse_variant(input);
    build("ridgeline", &v, "categories=[\"Mon\",\"Mon\",\"Mon\",\"Mon\",\"Mon\",\"Mon\",\"Mon\",\"Tue\",\"Tue\",\"Tue\",\"Tue\",\"Tue\",\"Tue\",\"Tue\",\"Wed\",\"Wed\",\"Wed\",\"Wed\",\"Wed\",\"Wed\",\"Wed\",\"Thu\",\"Thu\",\"Thu\",\"Thu\",\"Thu\",\"Thu\",\"Thu\",\"Fri\",\"Fri\",\"Fri\",\"Fri\",\"Fri\",\"Fri\",\"Fri\"], values=[1.2,2.4,2.7,3.1,3.5,3.8,4.2,2.0,2.8,3.2,3.6,4.1,4.5,5.0,1.8,2.2,2.6,3.0,3.4,3.9,4.3,2.5,3.1,3.5,4.0,4.5,5.0,5.5,1.5,2.0,2.5,3.0,3.5,4.0,4.5]")
}

pub fn demo_candlestick(input: &str) -> String {
    let v = parse_variant(input);
    build("candlestick", &v, "labels=[\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\"], open=[100,108,112,109,118], high=[112,118,120,121,125], low=[98,105,108,107,115], close=[108,112,109,118,122]")
}

pub fn demo_dumbbell(input: &str) -> String {
    let v = parse_variant(input);
    build("dumbbell", &v, "labels=[\"Alpha\",\"Beta\",\"Gamma\",\"Delta\",\"Epsilon\"], start=[10,22,15,8,18], end=[18,16,25,14,30]")
}

pub fn demo_gauge(input: &str) -> String {
    let v = parse_variant(input);
    build("gauge", &v, "value=72, min_val=0, max_val=100, label=\"KPI\"")
}

pub fn demo_parallel(input: &str) -> String {
    let v = parse_variant(input);
    build("parallel", &v, "axes=[\"Speed\",\"Power\",\"Range\",\"Comfort\",\"Price\"], series=[[80,72,65,90,55],[65,85,80,70,75],[70,60,75,80,65]], series_names=[\"A\",\"B\",\"C\"]")
}

pub fn demo_wordcloud(input: &str) -> String {
    let v = parse_variant(input);
    build("wordcloud", &v, "words=[\"data\",\"science\",\"machine\",\"learning\",\"python\",\"rust\",\"chart\",\"plot\",\"visual\",\"analytics\",\"insight\",\"model\",\"neural\",\"network\",\"deep\"], frequencies=[60,55,50,48,45,42,40,38,35,33,30,28,25,22,20]")
}

pub fn demo_bubble_map(input: &str) -> String {
    let v = parse_variant(input);
    build("bubble_map", &v, "labels=[\"France\",\"USA\",\"India\",\"Brazil\",\"China\"], values=[68,331,1380,213,1411]")
}

pub fn demo_choropleth(input: &str) -> String {
    let v = parse_variant(input);
    build("choropleth", &v, "labels=[\"France\",\"USA\",\"India\",\"Brazil\",\"China\"], values=[68,331,1380,213,1411]")
}

pub fn demo_scatter3d(input: &str) -> String {
    let v = parse_variant(input);
    build("scatter3d", &v, "x=[1.2,2.4,3.1,4.8,5.6,6.9], y=[2.1,3.4,5.2,4.6,6.8,7.1], z=[0.5,1.2,2.0,3.1,4.0,5.2]")
}

pub fn demo_bar3d(input: &str) -> String {
    let v = parse_variant(input);
    build("bar3d", &v, "x=[0,1,2,0,1,2,0,1,2], y=[0,0,0,1,1,1,2,2,2], z=[1,5,3,2,8,4,9,3,5]")
}

pub fn demo_line3d(input: &str) -> String {
    let v = parse_variant(input);
    build("line3d", &v, "x=[0,1,2,3,4,5,6,7], y=[0,1,4,9,16,25,36,49], z=[0,2,4,6,8,10,12,14]")
}

pub fn demo_radar3d(input: &str) -> String {
    let v = parse_variant(input);
    build("radar3d", &v, "axes=[\"Speed\",\"Power\",\"Range\",\"Comfort\",\"Price\"], series=[[80,72,65,90,55],[65,85,80,70,75]], series_names=[\"A\",\"B\"]")
}

pub fn demo_lollipop3d(input: &str) -> String {
    let v = parse_variant(input);
    build("lollipop3d", &v, "x=[0,1,2,3,4], y=[0,1,2,3,4], z=[10,22,15,30,18]")
}

pub fn demo_kde3d(input: &str) -> String {
    let v = parse_variant(input);
    build("kde3d", &v, "values=[1.2,2.1,2.4,2.7,3.0,3.1,3.3,3.5,3.7,3.8,4.0,4.2,4.4,4.5,4.7,4.8,5.0,5.2,5.4,5.7,6.0,6.3,6.7,7.1]")
}

pub fn demo_ridgeline3d(input: &str) -> String {
    let v = parse_variant(input);
    build("ridgeline3d", &v, "categories=[\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"D\",\"D\",\"D\",\"D\",\"D\",\"D\"], values=[1.2,2.4,2.7,3.1,3.5,3.8,2.0,2.8,3.2,3.6,4.1,4.5,1.8,2.2,2.6,3.0,3.4,3.9,2.5,3.1,3.5,4.0,4.5,5.0]")
}

pub fn demo_bubble3d(input: &str) -> String {
    let v = parse_variant(input);
    build("bubble3d", &v, "x=[1,5,10,3,7], y=[2,3,1,5,4], z=[0,1,2,3,4], sizes=[5,12,30,18,22]")
}

pub fn demo_pie3d(input: &str) -> String {
    let v = parse_variant(input);
    build("pie3d", &v, "labels=[\"Apple\",\"Samsung\",\"Xiaomi\",\"Other\"], values=[42,28,17,13]")
}

pub fn demo_violin3d(input: &str) -> String {
    let v = parse_variant(input);
    build("violin3d", &v, "categories=[\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"A\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"B\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\",\"C\"], values=[1.2,2.4,2.7,3.1,3.5,3.8,4.2,2.0,2.8,3.2,3.6,4.1,4.5,5.0,1.8,2.2,2.6,3.0,3.4,3.9,4.3]")
}

pub fn demo_heatmap3d(input: &str) -> String {
    let v = parse_variant(input);
    build("heatmap3d", &v, "x_labels=[\"C1\",\"C2\",\"C3\",\"C4\"], categories=[\"R1\",\"R2\",\"R3\",\"R4\"], matrix=[[1,5,3,7],[2,8,4,6],[9,3,5,2],[4,6,7,1]]")
}

pub fn demo_candlestick3d(input: &str) -> String {
    let v = parse_variant(input);
    build("candlestick3d", &v, "labels=[\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\"], open=[100,108,112,109,118], high=[112,118,120,121,125], low=[98,105,108,107,115], close=[108,112,109,118,122]")
}

pub fn demo_dumbbell3d(input: &str) -> String {
    let v = parse_variant(input);
    build("dumbbell3d", &v, "labels=[\"A\",\"B\",\"C\",\"D\"], start=[10,22,15,8], end=[18,16,25,14]")
}

pub fn demo_funnel3d(input: &str) -> String {
    let v = parse_variant(input);
    build("funnel3d", &v, "labels=[\"Visits\",\"Leads\",\"Trials\",\"Customers\"], values=[1000,520,210,85]")
}

pub fn demo_sunburst3d(input: &str) -> String {
    let v = parse_variant(input);
    build("sunburst3d", &v, "labels=[\"Tech\",\"Finance\",\"Web\",\"Mobile\",\"Banks\"], parents=[\"\",\"\",\"Tech\",\"Tech\",\"Finance\"], values=[0,0,30,40,50]")
}

pub fn demo_stacked_bar3d(input: &str) -> String {
    let v = parse_variant(input);
    build("stacked_bar3d", &v, "labels=[\"Q1\",\"Q2\",\"Q3\",\"Q4\"], series=[[24,38,17,42],[18,29,33,21],[12,15,28,30]], series_names=[\"A\",\"B\",\"C\"]")
}

pub fn demo_globe3d(input: &str) -> String {
    let v = parse_variant(input);
    build("globe3d", &v, "labels=[\"Paris\",\"NYC\",\"Tokyo\",\"Sydney\"], lats=[48.85,40.71,35.68,-33.87], lons=[2.35,-74.01,139.69,151.21], values=[100,80,60,40]")
}
