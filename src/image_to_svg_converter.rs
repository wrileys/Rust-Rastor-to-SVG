// src/image_to_svg_converter.rs
use image::{DynamicImage, GenericImageView, Pixel};
use svg::{
    node::element::{path::Data, Path},
    Document,
};
use std::collections::VecDeque;
use log::info;

pub struct ImageToSVGConverter {}

impl ImageToSVGConverter {
    pub fn new() -> Self {
        ImageToSVGConverter {}
    }

    pub fn convert(&self, image: &DynamicImage) -> svg::Document {
    let (width, height) = image.dimensions();
    let mut processed_rows = vec![false; height as usize];
    let mut visited = vec![vec![false; width as usize]; height as usize];

    let mut paths = Vec::new();

    for y in 0..height {
        if !processed_rows[y as usize] {
            let mut stack = VecDeque::new();
            stack.push_back((0, y));

            let mut points = Vec::new();
            while let Some((x, y)) = stack.pop_back() {
                if x < width && y < height && !visited[y as usize][x as usize] {
                    visited[y as usize][x as usize] = true;
                    
                    let pixel = image.get_pixel(x, y);
                    let channels = pixel.channels();
                    if channels[0] != 0 || channels[1] != 0 || channels[2] != 0 {
                        processed_rows[y as usize] = true;
                        points.push((x as f64, y as f64));
                        
                        // Properly manage boundaries when updating the stack
                        if y + 1 < height {
                            stack.push_back((x, y + 1));
                        }
                        if x + 1 < width {
                            stack.push_back((x + 1, y));
                        }
                        if y >= 1 {
                            stack.push_back((x, y - 1));
                        }
                        if x >= 1 {
                            stack.push_back((x - 1, y));
                        }
                    }
                }
            }

            if points.len() > 1 {
                let simplified_points = rdp(&points, 1.0);
                let path_data = self.path_points_to_svg_path_data(&simplified_points);
                let path = Path::new().set("d", path_data);
                paths.push(path);
            }
        }
        // Log the progress
        info!("Processing row: {} of {}", y, height);
    }

    let mut document = Document::new().set("viewBox", (0, 0, width, height));
    for path in paths {
        document = document.add(path);
    }
    document
}


    fn path_points_to_svg_path_data(&self, points: &[(f64, f64)]) -> Data {
        let mut path_data = Data::new();
        if let Some((x, y)) = points.first() {
            path_data = path_data.move_to((*x, *y));
        }
        for &(x, y) in points.iter().skip(1) {
            path_data = path_data.line_to((x, y));
        }
        path_data
    }
}

fn rdp(points: &[(f64, f64)], epsilon: f64) -> Vec<(f64, f64)> {
    let mut simplified_points = Vec::new();
    rdp_recursive(points, &mut simplified_points, epsilon);
    simplified_points.push(*points.last().unwrap());
    simplified_points
}

fn perpendicular_distance(p: &(f64, f64), p1: &(f64, f64), p2: &(f64, f64)) -> f64 {
    let (x, y) = *p;
    let (x1, y1) = *p1;
    let (x2, y2) = *p2;
    let num = ((y2 - y1) * x - (x2 - x1) * y + x2 * y1 - y2 * x1).abs();
    let denom = ((y2 - y1).powi(2) + (x2 - x1).powi(2)).sqrt();
    num / denom
}


fn rdp_recursive(points: &[(f64, f64)], simplified_points: &mut Vec<(f64, f64)>, epsilon: f64) {
    if points.len() < 2 {
        return;
    }

    let mut index = 0;
    let mut max_distance = 0.0;

    for (i, point) in points.iter().enumerate().skip(1).take(points.len() - 2) {
        let distance = perpendicular_distance(&point, &points[0], &points[points.len() - 1]);

        if distance > max_distance {
            index = i;
            max_distance = distance;
        }
    }

    if max_distance > epsilon {
        rdp_recursive(&points[..=index], simplified_points, epsilon);
        simplified_points.pop(); // remove the last point to avoid duplicates
        rdp_recursive(&points[index..], simplified_points, epsilon);
    } else {
        simplified_points.push(points[0]);
        simplified_points.push(points[points.len() - 1]);
    }
}


