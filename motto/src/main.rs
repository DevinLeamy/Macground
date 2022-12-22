use std::collections::HashMap;
// std
use std::default::Default;
use std::path::PathBuf;
use std::process::{Command, Stdio};
// third party
use image::{ImageBuffer, ImageResult, Pixel, Rgba};
mod text;
use dotenv::dotenv;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use source::ImageSource;
use text::{draw_text, Bounds, TextConfig};

mod source;

use crate::source::{ColorSource, Source};
use crate::text::font_path;

const WW: u32 = 3840;
const WH: u32 = 2160;
/*
Configurable things / CLI args:
- Font path
- Text
- Background color
- Text color
*/

fn main() {
    let pretty_images: Vec<&str> = vec![
        "https://images.pexels.com/photos/589840/pexels-photo-589840.jpeg?cs=srgb&dl=pexels-valiphotos-589840.jpg&fm=jpg",
    ];
    // let image_urls = get_random_image_urls();
    let (width, height) = match get_display_resolution() {
        Some(dimensions) => dimensions,
        None => (WW, WH),
    };

    // let image_source = ImageSource::new(width, height, image_urls[0].clone());
    let image_source = ImageSource::new(width, height, pretty_images[0].to_string());
    let color_source = ColorSource::random(width, height);
    let message = "\"You, my friend, are a piece of shit\" - D.L.".to_string();
    let mut background = image_source.get_background();
    // let mut background = color_source.get_background();

    let text_config = TextConfig {
        text: message,
        text_scale: 100.0,
        font_path: font_path("font1.otf"),
        context_bounds: Bounds {
            width: background.width() as f32,
            height: background.height() as f32,
        },
        ..Default::default()
    };

    draw_text(&mut background, text_config);
    let output_path = PathBuf::from(&format!(
        "{}/assets/backgrounds/{}",
        env!("CARGO_MANIFEST_DIR"),
        generate_file_name()
    ));
    BackgroundImage::save(background, &output_path).expect("Failed to save background image.");

    display_image_as_background(output_path);
}

fn generate_file_name() -> String {
    let mut rng = thread_rng();
    let id = rng.sample(Uniform::new(1000, 9999));

    format!("background_{id}.png")
}

/**
 * Use apple script to set the background image
 * to the image found at the provided `image_path`
 *
 * Note: This only sets the background image on one desktop
 * TODO: Allow the user to configure the desktop that gets modified
 */
fn display_image_as_background(image_path: PathBuf) -> () {
    Command::new("osascript")
        .args([
            "-e",
            format!(
                "tell application \"Finder\" to set desktop picture to POSIX file \"{}\"",
                image_path.to_str().unwrap()
            )
            .as_str(),
        ])
        .spawn()
        .expect("failed to set background image");
}

/// Computes the dimensions of your display. If there are multiple displays, it
/// computes the dimensions of the first one it finds.
fn get_display_resolution() -> Option<(u32, u32)> {
    let process_one = Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = Command::new("awk")
        .arg("/Resolution/{print $2, $3, $4}")
        .stdin(Stdio::from(process_one.stdout.unwrap()))
        .output()
        .unwrap();

    let result = std::str::from_utf8(&output.stdout).unwrap();
    // Some: "[width0, x, height0, x, width1, x, height1]"
    let split = result.split_whitespace();

    let mut dimensions: Vec<u32> = vec![];
    for str in split {
        if str == "x" {
            continue;
        }
        dimensions.push(str.parse::<u32>().unwrap());
    }

    if dimensions.is_empty() {
        None
    } else {
        Some((dimensions[0], dimensions[1]))
    }
}

#[derive(Debug)]
pub struct BackgroundImage {
    width: u32,
    height: u32,
    buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl BackgroundImage {
    pub fn new(width: u32, height: u32, background_color: &Rgba<u8>) -> Self {
        Self {
            buffer: ImageBuffer::from_pixel(width, height, *background_color),
            width,
            height,
        }
    }

    pub fn from(buffer: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Self {
        Self {
            width: buffer.width(),
            height: buffer.height(),
            buffer,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Sets the color of a given pixel
    pub fn set_pixel(&mut self, x: u32, y: u32, color: &Rgba<u8>) {
        if x >= self.width() || y >= self.height() {
            println!("Out of bounds ({x}, {y})");
            return;
        }
        self.buffer.get_pixel_mut(x, y).blend(&color);
    }

    pub fn save(image: BackgroundImage, path: &PathBuf) -> ImageResult<()> {
        image.buffer.save(path)
    }
}

#[derive(Deserialize)]
struct UnsplashResponse {
    urls: HashMap<String, String>,
}

fn get_random_image_urls() -> Vec<String> {
    // Loads the environment variables from .env
    dotenv().ok();

    let api_key = std::env::var("UNSPLASH_API_KEY").unwrap();
    let endpoint = format!("https://api.unsplash.com/photos/random?client_id={api_key}");
    let response = reqwest::blocking::get(&endpoint)
        .unwrap()
        .json::<UnsplashResponse>()
        .unwrap();

    vec![response.urls.get(&"full".to_string()).unwrap().to_owned()]
}
