// std
use std::default::Default;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
// third party
use image::{ImageBuffer, ImageResult, Pixel, Rgba};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

mod text;
use text::{draw_text, Bounds, TextConfig};

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
    let message = "\"You, my friend, are a piece of shit\" - D.L.".to_string();
    let (width, height) = match get_display_resolution() {
        Some(dimensions) => dimensions,
        None => (WW, WH),
    };

    let mut background = BackgroundImage::new(width, height, &random_color());
    get_display_resolution();

    let text_config = TextConfig {
        text: message,
        text_scale: 100.0,
        font_path: font_path("font1.otf"),
        context_bounds: Bounds {
            width: width as f32,
            height: height as f32,
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
    println!("Created new image");

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
        println!("STR: {str}");
        dimensions.push(str.parse::<u32>().unwrap());
    }

    if dimensions.is_empty() {
        None
    } else {
        Some((dimensions[0], dimensions[1]))
    }
}

pub fn random_color() -> Rgba<u8> {
    let mut rng = thread_rng();
    let r = rng.sample(Uniform::new(0, 255));
    let g = rng.sample(Uniform::new(0, 255));
    let b = rng.sample(Uniform::new(0, 255));

    [r, g, b, 255].into()
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

    /// Sets the color of a given pixel
    pub fn set_pixel(&mut self, x: u32, y: u32, color: &Rgba<u8>) {
        self.buffer.get_pixel_mut(x, y).blend(&color);
    }

    pub fn save(image: BackgroundImage, path: &PathBuf) -> ImageResult<()> {
        image.buffer.save(path)
    }
}

struct TextBox {}

// pub fn
