use std::path::PathBuf;
use std::time::Instant;

fn main() {
    println!("SeraPlot Fast Renderer Benchmark");
    println!("================================\n");

    let labels: Vec<String> = vec![
        "Butter".to_string(),
        "Oil".to_string(),
        "Cream".to_string(),
        "Chocolate".to_string(),
        "Nuts".to_string(),
        "Cheese".to_string(),
        "Bacon".to_string(),
        "Eggs".to_string(),
        "Salmon".to_string(),
        "Beef".to_string(),
        "Chicken".to_string(),
        "Pork".to_string(),
        "Turkey".to_string(),
        "Lamb".to_string(),
        "Duck".to_string(),
        "Goose".to_string(),
        "Venison".to_string(),
        "Rabbit".to_string(),
        "Pheasant".to_string(),
        "Quail".to_string(),
    ];

    let values: Vec<f64> = vec![
        720.0, 884.0, 340.0, 537.0, 567.0, 
        402.0, 541.0, 155.0, 208.0, 250.0,
        165.0, 242.0, 135.0, 294.0, 404.0,
        371.0, 163.0, 102.0, 238.0, 158.0,
    ];

    println!("Test 1: Bar Chart (Horizontal)\n");

    let start = Instant::now();
    
    let config = seraplot::bindings::FastChartConfig::bar_horizontal();
    let renderer = seraplot::bindings::FastChartRenderer::new(config, "Top Energy Products")
        .with_data(labels.clone(), values.clone());
    
    let svg = renderer.render_svg();

    let elapsed = start.elapsed();

    println!("  SVG size: {} bytes", svg.len());
    println!("  Render time: {:.4}s ({:.2}ms)", elapsed.as_secs_f64(), elapsed.as_secs_f64() * 1000.0);
    println!("  Data points: {}", values.len());
    println!("  Performance: {:.2} points/ms\n", values.len() as f64 / elapsed.as_secs_f64() / 1000.0);

    let output_path = PathBuf::from("./fast_render_output.svg");
    std::fs::write(&output_path, &svg).expect("Failed to write SVG");
    println!("  Saved to: {}\n", output_path.display());

    println!("Test 2: Line Chart\n");

    let start = Instant::now();
    
    let config = seraplot::bindings::FastChartConfig::line();
    let renderer = seraplot::bindings::FastChartRenderer::new(config, "Energy Progression")
        .with_data(labels.clone(), values.clone());
    
    let svg = renderer.render_svg();

    let elapsed = start.elapsed();

    println!("  SVG size: {} bytes", svg.len());
    println!("  Render time: {:.4}s ({:.2}ms)\n", elapsed.as_secs_f64(), elapsed.as_secs_f64() * 1000.0);

    println!("Test 3: Scatter Chart\n");

    let start = Instant::now();
    
    let config = seraplot::bindings::FastChartConfig::scatter();
    let renderer = seraplot::bindings::FastChartRenderer::new(config, "Scatter Distribution")
        .with_data(labels.clone(), values.clone());
    
    let svg = renderer.render_svg();

    let elapsed = start.elapsed();

    println!("  SVG size: {} bytes", svg.len());
    println!("  Render time: {:.4}s ({:.2}ms)\n", elapsed.as_secs_f64(), elapsed.as_secs_f64() * 1000.0);

    println!("\n================================");
    println!("Benchmark Complete");
    println!("================================");
}
