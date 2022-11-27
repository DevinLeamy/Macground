// std
use std::process::Command;
use std::default::Default;
// third party
use image::{ImageBuffer, Rgba};
use imageproc::drawing::text_size;
use rand::{thread_rng, Rng};
use rand::distributions::Uniform;
use rusttype::{Scale, Font};
use glyph_brush_layout::{*, ab_glyph::*};

mod text;
use text::{TextConfig, draw_text, generate_glyphs, Bounds};

mod background;
use background::{BackgroundConfig};

const OUTPUT_PATH: &str = "/Users/Devin/Desktop/background/motto/assets/backgrounds";
const FONT_PATH: &str = "/Users/Devin/Desktop/background/motto/assets/fonts";


fn create_new_background(config: BackgroundConfig) -> String {
    let mut image: ImageBuffer<Rgba<u8>, Vec<u8>>  = ImageBuffer::from_pixel(config.width, config.height, config.color);
    let font = load_font(config.font_name);

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

fn main() {
    let display_message = "-TextBreak-".to_string();
    
    let config = BackgroundConfig {
        message: display_message,
        ..Default::default()
    };
    let background_image_path = create_new_background(config);

    display_image_as_background(background_image_path);
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

fn save_image(image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> String {
    let image_path = format!("{OUTPUT_PATH}/{}", generate_file_name());
    image.save(image_path.clone()).expect("Error: failed to save image");

    println!("Saved image to: {image_path}");

    image_path
}

/**
 * Use apple script to set the background image
 * to the image found at the provided `image_path`
 *
 * Note: This only sets the background image on one desktop
 * TODO: Allow the user to configure the desktop that gets modified
 */
fn display_image_as_background(image_path: String) -> () {
    Command::new("osascript")
        .args(["-e", format!("tell application \"Finder\" to set desktop picture to POSIX file \"{image_path}\"").as_str()])
        .spawn()
        .expect("failed to set background image");

    println!("Updated background image");
}


