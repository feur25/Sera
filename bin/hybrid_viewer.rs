use std::path::Path;
use std::time::Instant;

fn main() {
    let labels: Vec<String> = vec![
        "Butter", "Oil", "Cream", "Chocolate", "Nuts", 
        "Cheese", "Bacon", "Eggs", "Salmon", "Beef",
        "Chicken", "Pork", "Turkey", "Lamb", "Duck",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect();

    let values: Vec<f64> = vec![
        720.0, 884.0, 340.0, 537.0, 567.0, 
        402.0, 541.0, 155.0, 208.0, 250.0,
        165.0, 242.0, 135.0, 294.0, 404.0,
    ];

    println!("SeraPlot Hybrid Viewer");
    println!("======================\n");

    println!("[Init] Creating hybrid viewer...");
    let start = Instant::now();
    
    let config = seraplot::bindings::FastChartConfig::bar_horizontal();
    let renderer = seraplot::bindings::FastChartRenderer::new(config, "Energy Products")
        .with_data(labels.clone(), values.clone());
    
    let svg = renderer.render_svg();
    let elapsed = start.elapsed();

    println!("  SVG generated in {:.4}s ({:.2}ms)", 
        elapsed.as_secs_f64(), 
        elapsed.as_secs_f64() * 1000.0
    );
    println!("  Size: {} bytes", svg.len());

    let output = "hybrid_output.svg";
    std::fs::write(output, &svg).expect("Failed to write SVG");
    println!("  Saved to: {}\n", output);

    println!("[GUI] Would launch interactive viewer");
    println!("  Use toggle to switch between:");
    println!("  • 📊 Fast SVG (instant)");
    println!("  • 🎨 Interactive (with hover/zoom)");
}
