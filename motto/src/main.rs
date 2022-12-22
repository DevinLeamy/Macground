// std
use std::default::Default;
use std::path::{Path, PathBuf};
use std::process::Command;
// third party
use image::{ImageBuffer, ImageResult, Pixel, Rgba};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

mod text;
use text::{draw_text, Bounds, TextConfig};

mod background;
use background::BackgroundConfig;

use crate::text::font_path;

fn main() {
    let message = "\"You, my friend, are a piece of shit\" - D.L.".to_string();

    let config = BackgroundConfig {
        message,
        color: random_color(),
        font_path: font_path("font1.otf"),
        ..Default::default()
    };
    let image_path = create_new_background(config);
    println!("Created new image");

    display_image_as_background(image_path);
}

/// Creates a new background image and returns a path to the
/// created image.
fn create_new_background(config: BackgroundConfig) -> PathBuf {
    let mut background = BackgroundImage::new(config.width, config.height, &config.color);

    // (*crate::text::FONT_LOADER).load_font(config.font_path.as_path());
    let text_config = TextConfig {
        text: config.message,
        text_scale: 100.0,
        font_path: config.font_path,
        context_bounds: Bounds {
            width: config.width as f32,
            height: config.height as f32,
        },
        ..Default::default()
    };

    draw_text(&mut background, text_config);
    save_image(background)
}

fn generate_file_name() -> String {
    let mut rng = thread_rng();
    let id = rng.sample(Uniform::new(1000, 9999));

    format!("background_{id}.png")
}

fn save_image(image: BackgroundImage) -> PathBuf {
    let output_path = PathBuf::from(&format!(
        "{}/assets/backgrounds/{}",
        env!("CARGO_MANIFEST_DIR"),
        generate_file_name()
    ));

    BackgroundImage::save(image, &output_path).expect("Failed to save background image.");

    output_path
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
