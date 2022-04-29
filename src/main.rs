use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use glob::glob;
use image::{ColorType, GenericImageView, imageops, ImageOutputFormat, RgbaImage};

fn main() -> Result<(), Box<dyn Error>> {
    let mut images = Vec::new();
    //let mut collector = |path: PathBuf| if !path.file_name().unwrap().to_str().unwrap().contains("result") { images.push(image::open(path).unwrap()) };
    let mut collector = |path: PathBuf| if !path.file_name().unwrap().to_str().unwrap().contains("result") { images.push(path) };

    glob("**/*.png")?.flatten().for_each(&mut collector);
    glob("**/*.jpg")?.flatten().for_each(&mut collector);
    glob("**/*.jpeg")?.flatten().for_each(&mut collector);

    println!("{} images found", images.len());

    if images.len() == 0 {
        return Err("No images found!".into())
    }

    //let (big_x, big_y) = images.iter().map(|it| it.dimensions()).max().ok_or("No max dim found!")?;
    let (big_x, big_y) = (1000, 1000);
    let (grid_width, grid_height) = largest_two_factors(images.len()).ok_or("No factors found!")?;
    let extra_space = 15;

    let (image_width, image_height) = ((big_x + extra_space) * grid_width as u32 + extra_space, (big_y + extra_space) * grid_height as u32 + extra_space);

    let mut output_image = RgbaImage::new(image_width, image_height);

    let mut iter = images.iter();

    for x in 0..grid_width {
        for y in 0..grid_height {
            if let Some(image) = iter.next() {
                let image = image::open(image)?;
                let (width, height) = image.dimensions();
                let (center_x, center_y) = (width as u32 / 2, height as u32 / 2);
                let (actual_center_x, actual_center_y) = ((big_x + extra_space) * x as u32 + extra_space + big_x / 2, (big_y + extra_space) * y as u32 + extra_space + big_y / 2);
                let (pos_x, pos_y) = (actual_center_x - center_x, actual_center_y - center_y);

                imageops::overlay(&mut output_image, &image, pos_x as i64, pos_y as i64)
            }
        }
    }

    image::write_buffer_with_format(&mut File::create("result.png")?, &output_image, image_width, image_height, ColorType::Rgba8, ImageOutputFormat::Png)?;

    Ok(())
}

fn largest_two_factors(num: usize) -> Option<(usize, usize)> {
    // inefficient but computer go fast
    let max_possibility = (num as f64).sqrt();

    for possible_factor in (1..=max_possibility as usize).rev() {
        if num % possible_factor == 0 {
            let a = num / possible_factor;
            let b = possible_factor;

            return Some((a, b));
        }
    }

    None
}
