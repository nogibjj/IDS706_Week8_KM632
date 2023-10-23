use image::imageops::FilterType;
use std::path::Path;
use std::time::Instant;
use std::process::Command;

fn main() {
    

    let start_time = Instant::now();

    // IO operation
    let input_image_path = Path::new("input").join("input.jpeg");
    let output_image_path = "output.jpeg";
    let image: image::DynamicImage = image::open(input_image_path).expect("Failed to open image");

    // Edit the image
    let new_size = (300, 200);
    let resized_image = image.resize_exact(new_size.0, new_size.1, FilterType::Gaussian);
    resized_image
        .save(output_image_path)
        .expect("Failed to save image");

    let end_time = Instant::now();
    let execution_time_ns = end_time.duration_since(start_time).as_nanos();
    println!("Execution Time: {} nanoseconds", execution_time_ns);
     // CPU calculation
    let output = Command::new("ps")
        .arg("-o")
        .arg("%cpu")
        .arg("-p")
        .arg(format!("{}", std::process::id()))
        .output()
        .expect("Failed to execute ps command");

    // Convert the output to a string
    let usage = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = usage.split('\n').collect();

    // Parse the CPU usage from the output
    if lines.len() >= 2 {
        let usage_str = lines[1].trim();
        let usage_float: Result<f32, _> = usage_str.parse();
        match usage_float {
            Ok(usage) => println!("CPU Usage: {:.2}%", usage),
            Err(_) => println!("Failed to parse CPU usage"),
        }
    } else {
        println!("Failed to get CPU usage");
    }


}



