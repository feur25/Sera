use std::time::Instant;

fn main() {
    println!("SeraPlot ChartBuilder Test");
    println!("=========================\n");

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

    println!("Test 1: ChartBuilder Bar Horizontal\n");

    let start = Instant::now();

    let builder = seraplot::bindings::ChartBuilder::new(1)
        .with_title("Top Energy Products (kcal/100g)")
        .with_data(labels.clone(), values.clone());

    let svg = builder.render();

    let elapsed = start.elapsed();

    println!("  SVG size: {} bytes", svg.len());
    println!("  Build + Render time: {:.4}s ({:.2}ms)", 
        elapsed.as_secs_f64(), 
        elapsed.as_secs_f64() * 1000.0
    );
    println!("  Data points: {}\n", values.len());

    let output_file = "chart_builder_output.svg";
    std::fs::write(output_file, &svg).expect("Failed to write SVG");
    println!("  Saved to: {}\n", output_file);

    println!("Test 2: ChartBuilder Bar Vertical\n");

    let start = Instant::now();

    let builder = seraplot::bindings::ChartBuilder::new(0)
        .with_title("Energy Distribution")
        .with_data(labels.clone(), values.clone());

    let svg = builder.render();

    let elapsed = start.elapsed();

    println!("  SVG size: {} bytes", svg.len());
    println!("  Build + Render time: {:.4}s ({:.2}ms)\n", 
        elapsed.as_secs_f64(), 
        elapsed.as_secs_f64() * 1000.0
    );

    println!("Test 3: ChartBuilder Line\n");

    let start = Instant::now();

    let builder = seraplot::bindings::ChartBuilder::new(2)
        .with_title("Energy Progression")
        .with_data(labels.clone(), values.clone());

    let svg = builder.render();

    let elapsed = start.elapsed();

    println!("  SVG size: {} bytes", svg.len());
    println!("  Build + Render time: {:.4}s ({:.2}ms)\n", 
        elapsed.as_secs_f64(), 
        elapsed.as_secs_f64() * 1000.0
    );

    println!("Test 4: ChartBuilder Scatter\n");

    let start = Instant::now();

    let builder = seraplot::bindings::ChartBuilder::new(3)
        .with_title("Scatter Distribution")
        .with_data(labels.clone(), values.clone());

    let svg = builder.render();

    let elapsed = start.elapsed();

    println!("  SVG size: {} bytes", svg.len());
    println!("  Build + Render time: {:.4}s ({:.2}ms)\n", 
        elapsed.as_secs_f64(), 
        elapsed.as_secs_f64() * 1000.0
    );

    println!("\n=========================");
    println!("ChartBuilder Test Complete");
    println!("=========================");
}
