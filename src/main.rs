// std
use clap::Parser;
use std::collections::HashMap;
use std::default::Default;
use std::error::Error;
use std::path::PathBuf;
// third party
use image::{ImageBuffer, ImageResult, Pixel, Rgba};
mod text;
use display_info::DisplayInfo;
use dotenv::dotenv;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use source::{ImageSource, QuoteSource, RandomWordSource, TextSource};
use text::TextConfig;

mod args;
mod source;
mod utils;

use crate::args::{BackgroundOptions, RawOptions, TextOptions};
use crate::source::{ColorSource, Source};
use crate::text::{draw_textbox, TextBox, TextSize, FONT_LOADER};
use crate::utils::application_data_path;
use args::Options;

const WW: u32 = 3840;
const WH: u32 = 2160;

fn main() {
    let options = Options::from(RawOptions::parse());

    #[allow(unused)]
    let pretty_images: Vec<&str> = vec![
        "https://images.pexels.com/photos/589840/pexels-photo-589840.jpeg?cs=srgb&dl=pexels-valiphotos-589840.jpg&fm=jpg",
        "https://hips.hearstapps.com/hmg-prod.s3.amazonaws.com/images/cute-cat-photos-1593441022.jpg?crop=0.670xw:1.00xh;0.167xw,0&resize=640:*"
    ];

    let (width, height) = match get_display_resolution() {
        Some(dimensions) => dimensions,
        None => (WW, WH),
    };

    println!("Width: {width}, Height: {height}");

    // Create a background
    let mut background = match options.background {
        BackgroundOptions::Color(color) => {
            let color_source = if &color == "random" {
                ColorSource::random(width, height)
            } else {
                let parsed_color = parse_color(&color);
                if parsed_color.is_none() {
                    panic!("Invalid color {}", color)
                }
                ColorSource::new(width, height, Rgba(parsed_color.unwrap()))
            };
            color_source.get_background()
        }
        BackgroundOptions::RandomImage => {
            let random_image_url = get_random_image();
            let image_source = ImageSource::new(width, height, random_image_url);
            image_source.get_background()
        }
        BackgroundOptions::Url(url) => {
            let image_source = ImageSource::new(width, height, url);
            image_source.get_background()
        }
    };

    // Create a message
    let text = match options.text {
        TextOptions::Message(message) => vec![message],
        TextOptions::RandomQuote => {
            let random_quote_source = QuoteSource::default();
            random_quote_source.source_text()
        }
        TextOptions::RandomWord => {
            let random_word_source = RandomWordSource::default();
            random_word_source.source_text()
        }
    };

    // Load the required fonts
    let font_path = options.font.font_path.clone();
    if let Some(font_path) = font_path {
        (*FONT_LOADER).load_font("first".to_string(), PathBuf::from(font_path).as_path());
    }

    let text_config = TextConfig {
        size: match options.font.font_size {
            Some(size) => TextSize::PxScale(size as f32),
            None => TextSize::FillParent,
        },
        font: "default".to_string(),
        color: Rgba(parse_color(&options.font.color).unwrap()),
        ..Default::default()
    };

    let textbox = TextBox {
        text: text[0].to_owned(),
        width: width / 2,
        height: height / 5,
        style: text_config,
    };

    draw_textbox(&mut background, textbox, width / 2, height / 2);
    let mut output_path = application_data_path();
    output_path.push("backgrounds");
    std::fs::create_dir_all(&output_path).unwrap();
    output_path.push(generate_file_name());
    // let output_path = PathBuf::from(&format!(
    //     "{}/assets/backgrounds/{}",
    //     env!("CARGO_MANIFEST_DIR"),
    //     generate_file_name()
    // ));
    BackgroundImage::save(background, &output_path).expect("Failed to save background image.");

    match display_image_as_background(&output_path) {
        Ok(()) => println!("Updated wallpaper with image [{:?}]", output_path),
        Err(e) => println!("Failed to set wallpaper. {e}"),
    };
}

fn generate_file_name() -> String {
    let mut rng = thread_rng();
    let id = rng.sample(Uniform::new(1000, 9999));

    format!("background_{id}.png")
}

/// Sets the background image on all active desktops.
///
/// Note: Setting the wallpaper of individual desktop is currently not supported by
///       wallpaper. It can be done easily on MacOS to be consitent between platforms
///       all desktops are set. This should eventually become a configuration options.
fn display_image_as_background(image_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    wallpaper::set_from_path(image_path.to_str().unwrap())
}

/// Computes the dimensions of the primary display.
fn get_display_resolution() -> Option<(u32, u32)> {
    let displays = DisplayInfo::all().unwrap();
    for display in &displays {
        dbg!(display);
    }
    if let Some(primary) = displays.iter().filter(|display| display.is_primary).next() {
        return Some((primary.width as u32, primary.height as u32));
    }

    None
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
            // println!("Out of bounds ({x}, {y})");
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

fn get_random_image() -> String {
    // Loads the environment variables from .env
    dotenv().ok();

    let api_key = std::env::var("UNSPLASH_API_KEY").unwrap();
    let endpoint = format!("https://api.unsplash.com/photos/random?client_id={api_key}");
    let response = reqwest::blocking::get(&endpoint)
        .unwrap()
        .json::<UnsplashResponse>()
        .unwrap();

    response.urls.get(&"full".to_string()).unwrap().to_owned()
}

fn parse_color(raw_color: &str) -> Option<[u8; 4]> {
    let parsed_color = csscolorparser::parse(raw_color);
    match parsed_color {
        Ok(color) => Some(color.to_rgba8()),
        _ => None,
    }
}
