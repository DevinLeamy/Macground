// std
use std::default::Default;
use std::path::{Path, PathBuf};
use std::process::Command;
// third party
use glyph_brush_layout::{ab_glyph::*, *};
use image::{ImageBuffer, Rgba};
use imageproc::drawing::text_size;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use rusttype::{Font, Scale};

mod text;
use text::{draw_text, generate_glyphs, Bounds, TextConfig};

mod background;
use background::BackgroundConfig;

fn main() {
    let message = "-TextBreak-".to_string();

    let config = BackgroundConfig {
        message,
        ..Default::default()
    };
    let image_path = create_new_background(config);
    println!("Created new image");

    display_image_as_background(image_path);
}

fn create_new_background(config: BackgroundConfig) -> PathBuf {
    let mut image: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_pixel(config.width, config.height, config.color);

    let text_config = TextConfig {
        text: config.message,
        text_scale: 100.0,
        context_bounds: Bounds {
            width: config.width as f32,
            height: config.height as f32,
        },
        ..Default::default()
    };

    draw_text(&mut image, text_config);
    save_image(image)
}

fn load_font(_font: &'static str) -> Font<'static> {
    let font_data = text::FontLoader::FONTS[0];
    Font::try_from_bytes(font_data).unwrap()
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

    println!("Updated background image");
}
