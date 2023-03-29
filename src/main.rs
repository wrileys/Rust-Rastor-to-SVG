// src/main.rs
use std::env;
use std::path::Path;
use image::io::Reader as ImageReader;
use crate::image_to_svg_converter::ImageToSVGConverter;
use chrono::Utc;
use std::io::Write;
mod image_to_svg_converter;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .format(|buf, record| {
            writeln!(
                buf,
                "{{ \"timestamp\": \"{}\", \"level\": \"{}\", \"message\": \"{}\" }}",
                Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                record.level(),
                record.args()
            )
        })
        .init();
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: image_to_svg <input_image> <output_svg>");
        return;
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let input_image = match ImageReader::open(&Path::new(input_file)) {
        Ok(reader) => match reader.decode() {
            Ok(image) => image,
            Err(error) => {
                println!("Error decoding image '{}': {}", input_file, error);
                return;
            }
        },
        Err(error) => {
            println!("Error opening image '{}': {}", input_file, error);
            return;
        }
    };

    let converter = ImageToSVGConverter::new();
    let document = converter.convert(&input_image);

    match std::fs::write(output_file, document.to_string()) {
        Ok(_) => println!("SVG saved to '{}'", output_file),
        Err(error) => println!("Error saving SVG to '{}': {}", output_file, error),
    };  
}
