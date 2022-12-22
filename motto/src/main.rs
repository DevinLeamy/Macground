// std
use std::default::Default;
use std::path::{Path, PathBuf};
use std::process::Command;
// third party
use image::{ImageBuffer, Rgba};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

mod text;
use text::{draw_text, Bounds, TextConfig};

mod background;
use background::BackgroundConfig;

use crate::text::font_path;

fn main() {
    let message = "\".-_Youmyfr".to_string();

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
    let mut image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(config.width, config.height, config.color);

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

    draw_text(&mut image, text_config);
    save_image(image)
}

fn generate_file_name() -> String {
    let mut rng = thread_rng();
    let id = rng.sample(Uniform::new(1000, 9999));

    format!("background_{id}.png")
}

fn save_image(image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> PathBuf {
    let output_path = PathBuf::from(&format!(
        "{}/assets/backgrounds/{}",
        env!("CARGO_MANIFEST_DIR"),
        generate_file_name()
    ));

    image
        .save(&output_path)
        .expect("Error: failed to save image");

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
