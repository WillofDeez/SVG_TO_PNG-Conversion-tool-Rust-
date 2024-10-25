use png::Encoder;
use resvg::render;
use resvg::usvg::{Options, Tree};
use std::fs::File;
use std::io::{self, BufWriter};
use tiny_skia::Pixmap;

fn convert_svg_to_png(
    input_file: &str,
    output_file: &str,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Load SVG Data
    let svg_data = std::fs::read(input_file)?;
    let opt = Options::default();
    let tree = Tree::from_data(&svg_data, &opt)?;

    // Create Pixmap to render the SVG
    let mut pixmap: Pixmap = Pixmap::new(width, height).ok_or("Failed to create Pixmap")?;

    // Scaling
    let scale_x = width as f32 / tree.size().width();
    let scale_y = height as f32 / tree.size().height();
    let transform = tiny_skia::Transform::from_scale(scale_x, scale_y);

    // Render the SVG onto the Pixmap
    render(&tree, transform, &mut pixmap.as_mut());

    // Save the Pixmap as a PNG file
    let file = File::create(output_file)?;
    let writer = BufWriter::new(file);

    // Encoder creation
    let mut encoder = Encoder::new(writer, width, height);

    // Encoder configuration
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    // Encoder initialization
    let mut png_writer = encoder.write_header()?;

    // Writing pixel data from the pixmap to the PNG
    png_writer.write_image_data(pixmap.data())?;

    // Return Ok result
    Ok(())
}

fn main() {
    let input_file = "C:/Users/HP/Desktop/svg_to_png_converter/new.svg"; // Full path to SVG file
    let output_file = "output.png"; // Desired output PNG file path

    // User input handling
    let (width, height) = loop {
        let mut width_input = String::new();
        let mut height_input = String::new();

        println!("Enter the width for the PNG output:");
        io::stdin()
            .read_line(&mut width_input)
            .expect("Failed to read line");
        let width: u32 = match width_input.trim().parse() {
            Ok(w) => w,
            Err(_) => {
                println!("Please enter a valid width number.");
                continue; // Retry input
            }
        };

        println!("Enter the height for the PNG output:");
        io::stdin()
            .read_line(&mut height_input)
            .expect("Failed to read line");
        let height: u32 = match height_input.trim().parse() {
            Ok(h) => h,
            Err(_) => {
                println!("Please enter a valid height number.");
                continue; // Retry input
            }
        };

        break (width, height); // Exit the loop with the valid width and height
    };

    // Convert SVG to PNG
    match convert_svg_to_png(input_file, output_file, width, height) {
        Ok(_) => println!("SVG Successfully Converted To PNG"),
        Err(e) => println!("Failed To Convert SVG: {}", e),
    }
}
