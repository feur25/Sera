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

pub fn dispatch_demo(name: &str, variant: &str) -> Option<String> {
    let input = format!("{{\"variant\":\"{}\"}}", variant);
    match name {
        "bar"          | "demo_bar"          => Some(demo_bar(&input)),
        "hbar"         | "demo_hbar"         => Some(demo_hbar(&input)),
        "line"         | "demo_line"         => Some(demo_line(&input)),
        "multiline"    | "demo_multiline"    => Some(demo_multiline(&input)),
        "area"         | "demo_area"         => Some(demo_area(&input)),
        "scatter"      | "demo_scatter"      => Some(demo_scatter(&input)),
        "bubble"       | "demo_bubble"       => Some(demo_bubble(&input)),
        "histogram"    | "demo_histogram"    => Some(demo_histogram(&input)),
        "grouped_bar"  | "demo_grouped_bar"  => Some(demo_grouped_bar(&input)),
        "stacked_bar"  | "demo_stacked_bar"  => Some(demo_stacked_bar(&input)),
        "heatmap"      | "demo_heatmap"      => Some(demo_heatmap(&input)),
        "pie"          | "demo_pie"          => Some(demo_pie(&input)),
        "donut"        | "demo_donut"        => Some(demo_donut(&input)),
        "boxplot"      | "demo_boxplot"      => Some(demo_boxplot(&input)),
        "violin"       | "demo_violin"       => Some(demo_violin(&input)),
        "slope"        | "demo_slope"        => Some(demo_slope(&input)),
        "sunburst"     | "demo_sunburst"     => Some(demo_sunburst(&input)),
        "funnel"       | "demo_funnel"       => Some(demo_funnel(&input)),
        "treemap"      | "demo_treemap"      => Some(demo_treemap(&input)),
        "waterfall"    | "demo_waterfall"    => Some(demo_waterfall(&input)),
        "bullet"       | "demo_bullet"       => Some(demo_bullet(&input)),
        "radar"        | "demo_radar"        => Some(demo_radar(&input)),
        "lollipop"     | "demo_lollipop"     => Some(demo_lollipop(&input)),
        "kde"          | "demo_kde"          => Some(demo_kde(&input)),
        "ridgeline"    | "demo_ridgeline"    => Some(demo_ridgeline(&input)),
        "candlestick"  | "demo_candlestick"  => Some(demo_candlestick(&input)),
        "dumbbell"     | "demo_dumbbell"     => Some(demo_dumbbell(&input)),
        "gauge"        | "demo_gauge"        => Some(demo_gauge(&input)),
        "parallel"     | "demo_parallel"     => Some(demo_parallel(&input)),
        "wordcloud"    | "demo_wordcloud"    => Some(demo_wordcloud(&input)),
        "bubble_map"   | "demo_bubble_map"   => Some(demo_bubble_map(&input)),
        "choropleth"   | "demo_choropleth"   => Some(demo_choropleth(&input)),
        "scatter3d"    | "demo_scatter3d"    => Some(demo_scatter3d(&input)),
        "bar3d"        | "demo_bar3d"        => Some(demo_bar3d(&input)),
        "line3d"       | "demo_line3d"       => Some(demo_line3d(&input)),
        "radar3d"      | "demo_radar3d"      => Some(demo_radar3d(&input)),
        "lollipop3d"   | "demo_lollipop3d"   => Some(demo_lollipop3d(&input)),
        "kde3d"        | "demo_kde3d"        => Some(demo_kde3d(&input)),
        "ridgeline3d"  | "demo_ridgeline3d"  => Some(demo_ridgeline3d(&input)),
        "bubble3d"     | "demo_bubble3d"     => Some(demo_bubble3d(&input)),
        "pie3d"        | "demo_pie3d"        => Some(demo_pie3d(&input)),
        "violin3d"     | "demo_violin3d"     => Some(demo_violin3d(&input)),
        "heatmap3d"    | "demo_heatmap3d"    => Some(demo_heatmap3d(&input)),
        "candlestick3d"| "demo_candlestick3d"=> Some(demo_candlestick3d(&input)),
        "dumbbell3d"   | "demo_dumbbell3d"   => Some(demo_dumbbell3d(&input)),
        "funnel3d"     | "demo_funnel3d"     => Some(demo_funnel3d(&input)),
        "sunburst3d"   | "demo_sunburst3d"   => Some(demo_sunburst3d(&input)),
        "stacked_bar3d"| "demo_stacked_bar3d"=> Some(demo_stacked_bar3d(&input)),
        "globe3d"      | "demo_globe3d"      => Some(demo_globe3d(&input)),
        _ => None,
    }
}

pub fn list_demos() -> Vec<&'static str> {
    vec![
        "bar", "hbar", "line", "multiline", "area", "scatter", "bubble",
        "histogram", "grouped_bar", "stacked_bar", "heatmap", "pie", "donut",
        "boxplot", "violin", "slope", "sunburst", "funnel", "treemap",
        "waterfall", "bullet", "radar", "lollipop", "kde", "ridgeline",
        "candlestick", "dumbbell", "gauge", "parallel", "wordcloud",
        "bubble_map", "choropleth", "scatter3d", "bar3d", "line3d",
        "radar3d", "lollipop3d", "kde3d", "ridgeline3d", "bubble3d",
        "pie3d", "violin3d", "heatmap3d", "candlestick3d", "dumbbell3d",
        "funnel3d", "sunburst3d", "stacked_bar3d", "globe3d",
    ]
}

pub fn demo_bar(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::BarVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = BarVariant::from_str(key);
    let args = crate::plot::statistical::bar::demo_kwargs(bv);
    build("bar", &v, args)
}

pub fn demo_hbar(input: &str) -> String {
    let v = parse_variant(input);
    build("hbar", &v, "labels=[\"Apples\",\"Bananas\",\"Cherries\",\"Dates\"], values=[42,28,33,17]")
}

pub fn demo_line(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::LineVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = LineVariant::from_str(key);
    let args = crate::plot::statistical::line::demo_kwargs(bv);
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
    use crate::plot::statistical::ScatterVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = ScatterVariant::from_str(key);
    let args = crate::plot::statistical::scatter::demo_kwargs(bv);
    build("scatter", &v, args)
}

pub fn demo_bubble(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::BubbleVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = BubbleVariant::from_str(key);
    let args = crate::plot::statistical::bubble::demo_kwargs(bv);
    build("bubble", &v, args)
}

pub fn demo_histogram(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::HistogramVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = HistogramVariant::from_str(key);
    let args = crate::plot::statistical::histogram::demo_kwargs(bv);
    build("histogram", &v, args)
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
    use crate::plot::statistical::HeatmapVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = HeatmapVariant::from_str(key);
    let args = crate::plot::statistical::heatmap::demo_kwargs(bv);
    build("heatmap", &v, args)
}

pub fn demo_pie(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::PieVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = PieVariant::from_str(key);
    let args = crate::plot::statistical::pie::demo_kwargs(bv);
    build("pie", &v, args)
}

pub fn demo_donut(input: &str) -> String {
    let v = parse_variant(input);
    build("donut", &v, "labels=[\"Apple\",\"Samsung\",\"Xiaomi\",\"Other\"], values=[42,28,17,13]")
}

pub fn demo_boxplot(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::BoxplotVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = BoxplotVariant::from_str(key);
    let args = crate::plot::statistical::boxplot::demo_kwargs(bv);
    build("boxplot", &v, args)
}

pub fn demo_violin(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::ViolinVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = ViolinVariant::from_str(key);
    let args = crate::plot::statistical::violin::demo_kwargs(bv);
    build("violin", &v, args)
}

pub fn demo_slope(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::SlopeVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = SlopeVariant::from_str(key);
    let args = crate::plot::statistical::slope::demo_kwargs(bv);
    build("slope", &v, args)
}

pub fn demo_sunburst(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::SunburstVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = SunburstVariant::from_str(key);
    let args = crate::plot::statistical::sunburst::demo_kwargs(bv);
    build("sunburst", &v, args)
}

pub fn demo_funnel(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::FunnelVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = FunnelVariant::from_str(key);
    let args = crate::plot::statistical::funnel::demo_kwargs(bv);
    build("funnel", &v, args)
}

pub fn demo_treemap(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::TreemapVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = TreemapVariant::from_str(key);
    let args = crate::plot::statistical::treemap::demo_kwargs(bv);
    build("treemap", &v, args)
}

pub fn demo_waterfall(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::WaterfallVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = WaterfallVariant::from_str(key);
    let args = crate::plot::statistical::waterfall::demo_kwargs(bv);
    build("waterfall", &v, args)
}

pub fn demo_bullet(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::BulletVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = BulletVariant::from_str(key);
    let args = crate::plot::statistical::bullet::demo_kwargs(bv);
    build("bullet", &v, args)
}

pub fn demo_radar(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::RadarVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = RadarVariant::from_str(key);
    let args = crate::plot::statistical::radar::demo_kwargs(bv);
    build("radar", &v, args)
}

pub fn demo_lollipop(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::LollipopVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = LollipopVariant::from_str(key);
    let args = crate::plot::statistical::lollipop::demo_kwargs(bv);
    build("lollipop", &v, args)
}

pub fn demo_kde(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::KdeVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = KdeVariant::from_str(key);
    let args = crate::plot::statistical::kde::demo_kwargs(bv);
    build("kde", &v, args)
}

pub fn demo_ridgeline(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::RidgelineVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = RidgelineVariant::from_str(key);
    let args = crate::plot::statistical::ridgeline::demo_kwargs(bv);
    build("ridgeline", &v, args)
}

pub fn demo_candlestick(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::CandlestickVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = CandlestickVariant::from_str(key);
    let args = crate::plot::statistical::candlestick::demo_kwargs(bv);
    build("candlestick", &v, args)
}

pub fn demo_dumbbell(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::DumbbellVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = DumbbellVariant::from_str(key);
    let args = crate::plot::statistical::dumbbell::demo_kwargs(bv);
    build("dumbbell", &v, args)
}

pub fn demo_gauge(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::GaugeVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = GaugeVariant::from_str(key);
    let args = crate::plot::statistical::gauge::demo_kwargs(bv);
    build("gauge", &v, args)
}

pub fn demo_parallel(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::ParallelVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = ParallelVariant::from_str(key);
    let args = crate::plot::statistical::parallel::demo_kwargs(bv);
    build("parallel", &v, args)
}

pub fn demo_wordcloud(input: &str) -> String {
    let v = parse_variant(input);
    use crate::plot::statistical::WordCloudVariant;
    let key = if v == "default" || v.is_empty() { "basic" } else { v.as_str() };
    let bv = WordCloudVariant::from_str(key);
    let args = crate::plot::statistical::wordcloud::demo_kwargs(bv);
    build("wordcloud", &v, args)
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